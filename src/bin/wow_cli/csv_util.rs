//! Shared CSV utilities for code generators.

use std::io::{BufRead, BufReader, Read};

/// Read a CSV file as logical records, joining physical lines that contain
/// unbalanced quotes (multi-line fields). Returns rows with the header row
/// at index 0.
pub fn read_csv_records<R: Read>(mut reader: BufReader<R>) -> std::io::Result<Vec<String>> {
    let mut records = Vec::new();
    let mut buffer = String::new();
    let mut quote_count = 0usize;
    let mut line = String::new();
    while reader.read_line(&mut line)? != 0 {
        quote_count += line.bytes().filter(|b| *b == b'"').count();
        buffer.push_str(&line);
        line.clear();
        if quote_count.is_multiple_of(2) {
            // Strip only the record terminator; preserve quoted field CRLF bytes.
            if buffer.ends_with('\n') {
                buffer.pop();
                if buffer.ends_with('\r') {
                    buffer.pop();
                }
            }
            records.push(std::mem::take(&mut buffer));
            quote_count = 0;
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

#[cfg(test)]
mod tests {
    use super::{parse_csv_line, read_csv_records};
    use std::io::{BufReader, Cursor};

    #[test]
    fn csv_records_preserve_embedded_crlf_and_following_row() {
        let input = b"ID,Text\r\n7,\"First\r\n\r\nLast, \"\"quoted\"\"\"\r\n8,After";
        let records = read_csv_records(BufReader::new(Cursor::new(input))).unwrap();
        assert_eq!(records.len(), 3);
        assert_eq!(records[0], "ID,Text");
        assert_eq!(
            parse_csv_line(&records[1]),
            ["7", "First\r\n\r\nLast, \"quoted\""]
        );
        assert_eq!(parse_csv_line(&records[2]), ["8", "After"]);
    }
}
