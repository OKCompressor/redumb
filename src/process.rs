// src/process.rs

use anyhow::Result;
use crate::replace::{replace_special_chars, restore_special_chars};
use crate::dictionary::Dictionary;
use regex::Regex;
use std::fs::{File, create_dir_all, metadata};
use std::io::{Read, Write, BufWriter};
use std::path::Path;
use indicatif::{ProgressBar, ProgressStyle};

/// Maximum bytes per chunk (e.g. 100 MB)
pub const CHUNK_SIZE: usize = 100 * 1024 * 1024;

/// Splits an input file into fixed‐size byte chunks, builds a separate dictionary
/// per chunk, and writes three files per chunk under the given directories:
///   - `{dict_dir}/chunk_{:03}.dict`   (one token per line)
///   - `{sdict_dir}/chunk_{:03}.sdict` (sorted tokens)
///   - `{encoded_dir}/chunk_{:03}.enc` (space‐separated indices)
pub fn transform_chunked(
    input_file: &str,
    dict_dir: &str,
    sdict_dir: &str,
    encoded_dir: &str,
) -> Result<()> {
    // Ensure output dirs
    create_dir_all(dict_dir)?;
    create_dir_all(sdict_dir)?;
    create_dir_all(encoded_dir)?;

    // Prepare a progress bar over file bytes
    let total_bytes = metadata(input_file)?.len();
    let pb = ProgressBar::new(total_bytes);
    pb.set_style(ProgressStyle::with_template(
        "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})"
    )?
    .progress_chars("#>-"));

    let token_re = Regex::new(
        r"\d+|\w+|&[a-z]+;|[^\w\s]|\u{0004}[A-Za-z0-9+/=]+\u{0005}|\u{0001}|\u{0002}\d+\u{0002}|\u{0003}| "
    )?;

    let mut f = File::open(input_file)?;
    let mut leftover = Vec::new();
    let mut chunk_idx = 0;

    loop {
        // Read up to CHUNK_SIZE bytes
        let mut buf = vec![0u8; CHUNK_SIZE];
        let n = f.read(&mut buf)?;
        if n == 0 && leftover.is_empty() {
            break;
        }
        buf.truncate(n);
        if !leftover.is_empty() {
            let mut tmp = leftover.clone();
            tmp.extend_from_slice(&buf);
            buf = tmp;
            leftover.clear();
        }

        // Truncate at valid UTF-8 boundary
        let valid_up_to = match std::str::from_utf8(&buf) {
            Ok(_) => buf.len(),
            Err(e) => e.valid_up_to(),
        };
        let (chunk_bytes, rest) = buf.split_at(valid_up_to);
        leftover = rest.to_vec();

        // Advance our progress bar by the bytes we've consumed
        pb.inc(valid_up_to as u64);

        // Process this chunk
        let chunk_str = std::str::from_utf8(chunk_bytes)?;
        let processed = replace_special_chars(chunk_str);

        // Build per-chunk dictionary & encode
        let mut dict = Dictionary::new(&[]);
        dict.token_to_idx.clear();
        dict.idx_to_token.clear();

        let enc_path = Path::new(encoded_dir).join(format!("chunk_{:03}.enc", chunk_idx));
        let mut enc_w = BufWriter::new(File::create(&enc_path)?);
        for mat in token_re.find_iter(&processed) {
            let tok = mat.as_str();
            let idx = dict
                .token_to_idx
                .get(tok)
                .cloned()
                .unwrap_or_else(|| {
                    let i = dict.token_to_idx.len();
                    dict.token_to_idx.insert(tok.to_string(), i);
                    dict.idx_to_token.push(tok.to_string());
                    i
                });
            write!(enc_w, "{} ", idx)?;
        }
        enc_w.flush()?;

        // Save unsorted dict
        let dict_path = Path::new(dict_dir).join(format!("chunk_{:03}.dict", chunk_idx));
        let mut d_w = BufWriter::new(File::create(&dict_path)?);
        for tok in &dict.idx_to_token {
            writeln!(d_w, "{}", tok)?;
        }
        d_w.flush()?;

        // Save sorted dict
        let sdict_path = Path::new(sdict_dir).join(format!("chunk_{:03}.sdict", chunk_idx));
        let mut sorted = dict.idx_to_token.clone();
        sorted.sort();
        let mut sd_w = BufWriter::new(File::create(&sdict_path)?);
        for tok in sorted {
            writeln!(sd_w, "{}", tok)?;
        }
        sd_w.flush()?;

        chunk_idx += 1;
        if n == 0 {
            break;
        }
    }

    pb.finish_and_clear();
    Ok(())
}

/// Reads each chunk’s `.dict` and `.enc` in index order, stitches the tokens back
/// together, reverses placeholders, and writes the full reconstructed text.
pub fn restore_chunked(
    dict_dir: &str,
    encoded_dir: &str,
    output_file: &str,
) -> Result<()> {
    // Gather and sort chunk indices
    let mut chunks: Vec<usize> = std::fs::read_dir(encoded_dir)?
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let name = e.file_name().into_string().ok()?;
            if name.starts_with("chunk_") && name.ends_with(".enc") {
                name["chunk_".len()..name.len() - 4]
                    .parse::<usize>()
                    .ok()
            } else {
                None
            }
        })
        .collect();
    chunks.sort();

    // Prepare a progress bar over chunks
    let pb = ProgressBar::new(chunks.len() as u64);
    pb.set_style(ProgressStyle::with_template(
        "{spinner:.green} [{elapsed_precise}] [{bar:40.magenta/blue}] {pos}/{len} chunks"
    )?
    .progress_chars("=>-"));

    let mut full = String::new();
    for idx in chunks {
        // Load this chunk’s dictionary
        let dict_path = Path::new(dict_dir).join(format!("chunk_{:03}.dict", idx));
        let dict_txt = std::fs::read_to_string(dict_path)?;
        let toks: Vec<String> = dict_txt.lines().map(String::from).collect();
        let dict = Dictionary::new(&toks);

        // Read encoded indices
        let enc_path = Path::new(encoded_dir).join(format!("chunk_{:03}.enc", idx));
        let enc_txt = std::fs::read_to_string(enc_path)?;
        for id_str in enc_txt.split_whitespace() {
            if let Ok(i) = id_str.parse::<usize>() {
                if let Some(tok) = dict.idx_to_token.get(i) {
                    full.push_str(tok);
                }
            }
        }

        pb.inc(1);
    }

    pb.finish_and_clear();

    // Undo placeholders and write out
    let original = restore_special_chars(&full);
    std::fs::write(output_file, original)?;
    Ok(())
}

