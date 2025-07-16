// src/replace.rs
use regex::Regex;
use base64::{engine::general_purpose, Engine as _};

const CHAR_THRESHOLD: u32 = 0x7F;
pub const TAB_TOKEN: &str = "\x01";
pub const SPACE_TOKEN: &str = "\x02";
pub const NEWLINE_PLACEHOLDER: &str = "\x03";
pub const B64_PREFIX: &str = "\x04";
pub const B64_SUFFIX: &str = "\x05";

/// Replace non-ASCII and special chars
pub fn replace_special_chars(text: &str) -> String {
    let mut out = String::new();
    for ch in text.chars() {
        if ch as u32 > CHAR_THRESHOLD {
            let encoded = general_purpose::STANDARD.encode(ch.to_string());
            out.push_str(B64_PREFIX);
            out.push_str(&encoded);
            out.push_str(B64_SUFFIX);
        } else {
            out.push(ch);
        }
    }
    out.replace("\n", NEWLINE_PLACEHOLDER)
       .replace("\t", TAB_TOKEN)
    // TODO: implement multi-space replacement
}