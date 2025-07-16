// src/io.rs
use std::fs;
use std::io::{self, Write};

/// Read entire file to a String
pub fn read_input_file(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

/// Save a String to a file
pub fn save_output_file(path: &str, content: &str) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    file.write_all(content.as_bytes())
}