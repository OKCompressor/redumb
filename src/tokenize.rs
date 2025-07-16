// src/tokenize.rs
use regex::Regex;

/// Tokenize text into tokens
pub fn tokenize_text(text: &str) -> Vec<String> {
    // Example pattern: digits, words, entities, base64 blocks, tokens, whitespace
    let re = Regex::new(r"\d+|\w+|&[a-z]+;|[^\w\s]|\x04[A-Za-z0-9+/=]+\x05|\x01|\x02\d+\x02|\x03| ").unwrap();
    re.find_iter(text)
        .map(|m| m.as_str().to_string())
        .collect()
}