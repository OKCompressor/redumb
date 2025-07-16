// src/process.rs
use crate::io::{read_input_file, save_output_file};
use crate::replace::replace_special_chars;
use crate::tokenize::tokenize_text;
use crate::dictionary::Dictionary;

/// High-level transform pipeline
pub fn transform(
    input_path: &str,
    dict_path: &str,
    output_path: &str,
) -> anyhow::Result<()> {
    let text = read_input_file(input_path)?;
    let replaced = replace_special_chars(&text);
    let tokens = tokenize_text(&replaced);
    let dict = Dictionary::new(&tokens);

    // Map tokens to indices
    let indices: Vec<String> = tokens
        .iter()
        .map(|t| dict.token_to_idx[t].to_string())
        .collect();

    // Save
    save_output_file(output_path, &indices.join(" "))?;
    // TODO: save dict to dict_path
    Ok(())
}