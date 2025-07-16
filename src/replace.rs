// src/replace.rs

use regex::{Regex, Captures};
use base64::{engine::general_purpose, Engine as _};

const TAB_TOKEN: &str = "\u{0001}";
const SPACE_TOKEN: &str = "\u{0002}";
const NEWLINE_PLACEHOLDER: &str = "\u{0003}";
const B64_PREFIX: &str = "\u{0004}";
const B64_SUFFIX: &str = "\u{0005}";
const CHAR_THRESHOLD: u32 = 0x7F;

/// Replace non-ASCII, newlines, tabs, and runs of spaces in the input text
/// with placeholder tokens.
pub fn replace_special_chars(input: &str) -> String {
    // 1) Normalize all line endings to '\n'
    let normalized = input.replace("\r\n", "\n").replace('\r', "\n");

    // 2) Replace non-ASCII chars with base64 markers
    let mut interim = String::with_capacity(normalized.len());
    for ch in normalized.chars() {
        if (ch as u32) > CHAR_THRESHOLD {
            // Encode the UTF-8 bytes of the single char
            let mut buf = [0u8; 4];
            let slice = ch.encode_utf8(&mut buf).as_bytes();
            let encoded = general_purpose::STANDARD.encode(slice);
            interim.push_str(B64_PREFIX);
            interim.push_str(&encoded);
            interim.push_str(B64_SUFFIX);
        } else {
            interim.push(ch);
        }
    }

    // 3) Placeholderize newlines and tabs
    let mut s = interim.replace('\n', NEWLINE_PLACEHOLDER);
    s = s.replace('\t', TAB_TOKEN);

    // 4) Collapse runs of ≥2 spaces into SPACE_TOKEN<length>SPACE_TOKEN,
    //    unless the run is immediately followed by a digit.
    let mut output = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == ' ' {
            // Count consecutive spaces
            let mut count = 1;
            while let Some(&' ') = chars.peek() {
                chars.next();
                count += 1;
            }
            if count >= 2 {
                // Peek next character to avoid digit collisions
                if let Some(&next_ch) = chars.peek() {
                    if next_ch.is_ascii_digit() {
                        // Leave as literal spaces
                        for _ in 0..count { output.push(' '); }
                    } else {
                        // Placeholder
                        output.push_str(SPACE_TOKEN);
                        output.push_str(&count.to_string());
                        output.push_str(SPACE_TOKEN);
                    }
                } else {
                    // End of text, always placeholder
                    output.push_str(SPACE_TOKEN);
                    output.push_str(&count.to_string());
                    output.push_str(SPACE_TOKEN);
                }
            } else {
                // Single space
                output.push(' ');
            }
        } else {
            output.push(ch);
        }
    }

    output
}

/// Restore placeholders back into their original characters.
pub fn restore_special_chars(input: &str) -> String {
    // 1) Restore base64-encoded non-ASCII chars
    let b64_re = Regex::new(
        &format!(
            "{}([A-Za-z0-9+/=]+){}",
            regex::escape(B64_PREFIX),
            regex::escape(B64_SUFFIX)
        )
    ).unwrap();
    let stage1 = b64_re.replace_all(input, |caps: &Captures| {
        let enc = &caps[1];
        // Add padding if missing
        let padded = format!("{}{}", enc, "=".repeat((4 - enc.len() % 4) % 4));
        match general_purpose::STANDARD.decode(padded) {
            Ok(bytes) => String::from_utf8(bytes).unwrap_or("\u{FFFD}".to_string()),
            Err(_) => "\u{FFFD}".to_string(),
        }
    });

    // 2) Restore space placeholders
    let space_re = Regex::new(
        &format!(
            "{}(\\d+){}",
            regex::escape(SPACE_TOKEN),
            regex::escape(SPACE_TOKEN)
        )
    ).unwrap();
    let stage2 = space_re.replace_all(&stage1, |caps: &Captures| {
        let n: usize = caps[1].parse().unwrap_or(0);
        " ".repeat(n)
    });

    // 3) Restore newlines and tabs
    let s3 = stage2.replace(NEWLINE_PLACEHOLDER, "\n");
    let s4 = s3.replace(TAB_TOKEN, "\t");

    s4
}
