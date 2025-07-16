## 1. Treat all tokens (ASCII & non-ASCII) uniformly

Instead of Base64 markers, just let your tokenizer emit each non-ASCII code-point (or grapheme) as its own multi‐byte `String` token.  They’ll get a unique index, too, and round-trip perfectly.

```rust
// in your replace.rs, drop Base64 entirely:
for ch in normalized.chars() {
    if ch.is_ascii() {
        interim.push(ch);
    } else {
        // emit the raw UTF-8 char as a token
        interim.push_str(&ch.to_string());
    }
}
```

Now every character or word‐fragment becomes exactly one entry in your `Dictionary`.

---

## 2. Switch from ASCII decimals → binary indexes

Writing `"12345 "`  for each token index is cripplingly slow on decode.  Instead:

1. **Choose a fixed‐width type** (e.g. `u32`), or
2. **Varint‐encode** with LEB128 (to save space on small indices).

**Example: fixed‐width `u32`:**

```rust
// encode: write 4 bytes per token, little‐endian
use byteorder::{LittleEndian, WriteBytesExt};

for idx in &indices {
    enc_w.write_u32::<LittleEndian>(*idx as u32)?;
}
```

```rust
// decode: read 4 bytes at a time in a tight loop
use byteorder::{LittleEndian, ReadBytesExt};

let mut buf = [0u8; 4];
while enc_f.read_exact(&mut buf).is_ok() {
    let idx = (&buf[..]).read_u32::<LittleEndian>()? as usize;
    output.push_str(&dict[idx]);
}
```

That drops both file size (binary is denser) *and* massively speeds up the decode loop (no parsing, no branches).

---

## 3. Optimize the restore path for speed

– **Memory-map** your `.dict` file (or read it once into a `Vec<String>`).
– **Pre-allocate** a big `String` or `Vec<u8>` for the entire output (if you know its rough size).
– **Decode chunks in parallel** (e.g. Rayon `par_iter` over your chunk files).

```rust
let dict: Vec<String> = merged_dict
    .lines()
    .map(String::from)
    .collect();

// parallel decode
rayon::scope(|s| {
  for part in &chunk_parts {
    s.spawn(move |_| {
      let mut local_buf = String::new();
      let mut f = File::open(part).unwrap();
      let mut idx_bytes = [0u8; 4];
      while f.read_exact(&mut idx_bytes).is_ok() {
        let idx = u32::from_le_bytes(idx_bytes) as usize;
        local_buf.push_str(&dict[idx]);
      }
      // write local_buf into the correct offset of the final output
      // …
    });
  }
});
```

On a multicore machine you can decode 10 chunks in parallel and nearly quarter the wall-time.

---

## 4. Keep entropy coding out of the critical path

As you said, ANS or arithmetic coding is *lower* priority.  Once you have a compact binary index stream, you can pipe it into any compressor off‐the‐shelf (zstd, zpaq, cmix), or drop a custom ANS backend on it as a filter—completely separately from your fast encode/decode core.

---

### TL;DR

1. **Index non-ASCII** as raw tokens, not Base64.
2. **Emit binary indexes** (u32 or varint), not ASCII numbers.
3. **Parallelize & memory‐map** the restore pass.
4. Layer your entropy coder *after* you’ve built that super-fast token stream.

This will give you:

* **<½ the file size** (binary vs text),
* **10×–50× faster decode** (no string parsing),
* **Near-linear speed-up** on multicore, and
* A clean separation so you can drop in any ANS/PAQ/CMIX front-end later.

That combination is exactly what real Hutter-Prize preprocessors do.

