# Redumb

**Rust port of OKC’s “DumbPreprocessor”**  
A chunked, streaming tokenizer/preprocessor with on-the-fly dictionary building.

---

## Key Highlights

- **Super-fast tokenization**: ~1 minute to tokenize **enwik9** (≈1 GB) into ~42 M tokens  
- **Memory-efficient**: Fixed-size chunks (100 MB) keep RAM usage bounded  
- **Self-contained CLI**: `encode`, `restore`, `merge-dicts` commands  

---

## 📊 Benchmark (enwik9)

|                        Metric | Value                        |
| ----------------------------: | :--------------------------- |
|                    **Chunks** | 10                           |
|     **Compressed dicts (7z)** | 15.6 MB (15 609 856 bytes)   |
|         **Raw chunked dicts** | 10 × \~3.82 MB = 38.2 MB     |
|         **Encodings on disk** | 10 × \~140 MB = 1.4 GB       |
| **Compressed encodings (7z)** | 216.8 MB (216 829 952 bytes) |
|           **Processing time** | \~30 minutes                 |
| **Total footprint (7zipped)** | \~230 MB                     |


---

## Merged Dictionary

After concatenating and deduplicating all chunked dicts:

- **Total entries:** 4 331 474  
- **Unique tokens:** 2 223 154  
- **Merged dict size:** 20.8 MB (20 803 584 bytes)  
- **7z compressed merged dict:** 9.2 MB (9 162 752 bytes)  

> **Note:** To actually use the merged dictionary for encoding you’ll need to re-map all chunked indices to the new global indices—a non-trivial re-indexing pass. This logic can be parallelized (potentially halving runtime), but it adds complexity and is deferred for now.

---

## Next Steps

1. **Merge chunked dictionaries**  
   ```bash
   redumb merge-dicts <chunked_dict_dir> <output_dict_file>
	```

→ Produces final.dict (requires re-index logic to be decoding-usable).

🔮 Next Steps

    Re-index merged encodings
    Map all chunk-local indexes into the final global dictionary space.

    Full parallelism
    Encode & restore chunks concurrently with Rayon for ½ wall-time or better.

    Alternative token mappings
    Experiment with fixed-width (u16) or varint codes for sub-byte savings.

    Test enwik8 & enwik7
    Verify scaling and wall-time improvements on smaller Wikipedia slices.

Redumb is your blazing-fast Rust preprocessor for modern text compression pipelines.


CLI Usage

# Chunked encoding
redumb encode <input_file> <dict_dir> <sdict_dir> <enc_dir>

# Chunked restoration
redumb restore <dict_dir> <enc_dir> <output_file>

# Merge dictionaries (produces final.dict; re-indexing pass required to use)
redumb merge-dicts <chunked_dict_dir> <output_dict_file>

This project demonstrates a high-performance, chunked preprocessor pipeline in Rust—ideal as a frontend for more sophisticated text compressors.
