//! Shared CSV utilities for code generators.

use std::io::{BufRead, BufReader, Read};

/// Read a CSV file as logical records, joining physical lines that contain
/// unbalanced quotes (multi-line fields). Returns rows with the header row
/// at index 0.
pub fn read_csv_records<R: Read>(reader: BufReader<R>) -> std::io::Result<Vec<String>> {
    let mut records = Vec::new();
    let mut buffer = String::new();
    let mut quote_count = 0usize;
    for line in reader.lines() {
        let line = line?;
        if buffer.is_empty() {
            quote_count = line.bytes().filter(|b| *b == b'"').count();
            if quote_count.is_multiple_of(2) {
                records.push(line);
            } else {
                buffer.push_str(&line);
            }
        } else {
            buffer.push('\n');
            buffer.push_str(&line);
            quote_count += line.bytes().filter(|b| *b == b'"').count();
            if quote_count.is_multiple_of(2) {
                records.push(std::mem::take(&mut buffer));
                quote_count = 0;
            }
        }
    }
    if !buffer.is_empty() {
        records.push(buffer);
    }
    Ok(records)
}

/// Parse a CSV line, handling quoted fields properly.
pub fn parse_csv_line(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '"' if !in_quotes => {
                in_quotes = true;
            }
            '"' if in_quotes => {
                if chars.peek() == Some(&'"') {
                    current.push('"');
                    chars.next();
                } else {
                    in_quotes = false;
                }
            }
            ',' if !in_quotes => {
                fields.push(current.clone());
                current = String::new();
            }
            _ => {
                current.push(c);
            }
        }
    }
    fields.push(current);
    fields
}

/// Escape a string for use inside a Rust string literal.
pub fn escape_str(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

/// Return the default WoW data directory path.
pub fn wow_data_dir() -> std::path::PathBuf {
    dirs::home_dir()
        .expect("No home dir")
        .join("Projects/wow/data")
}
