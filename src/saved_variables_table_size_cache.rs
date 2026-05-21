use std::fs;
use std::path::{Path, PathBuf};

use super::saved_variables_parse::{SavedVariablesTableSize, SavedVariablesTableSizeCache};

const TABLE_SIZE_CACHE_FILE: &str = ".saved-variable-table-sizes.tsv";

pub(super) fn load_table_size_cache(
    storage_dir: &Path,
    source_path: &Path,
) -> SavedVariablesTableSizeCache {
    let source_key = table_size_cache_source_key(source_path);
    let Ok(contents) = fs::read_to_string(table_size_cache_path(storage_dir)) else {
        return SavedVariablesTableSizeCache::default();
    };

    let mut cache = SavedVariablesTableSizeCache::default();
    for line in contents.lines() {
        let Some((cached_source, table_path, size)) = parse_table_size_cache_line(line) else {
            continue;
        };
        if cached_source == source_key {
            cache.insert(table_path, size);
        }
    }
    cache
}

pub(super) fn save_table_size_cache(
    storage_dir: &Path,
    source_path: &Path,
    cache: &SavedVariablesTableSizeCache,
) -> crate::Result<()> {
    fs::create_dir_all(storage_dir).map_err(|error| crate::Error::Other(error.to_string()))?;

    let source_key = table_size_cache_source_key(source_path);
    let cache_path = table_size_cache_path(storage_dir);
    let mut lines = existing_cache_lines_for_other_sources(&cache_path, &source_key);

    for (table_path, size) in cache.iter() {
        lines.push(format!(
            "{}\t{}\t{}\t{}",
            escape_table_size_cache_field(&source_key),
            escape_table_size_cache_field(table_path),
            size.array_count,
            size.hash_count
        ));
    }

    let mut contents = lines.join("\n");
    if !contents.is_empty() {
        contents.push('\n');
    }
    fs::write(cache_path, contents).map_err(|error| crate::Error::Other(error.to_string()))
}

fn existing_cache_lines_for_other_sources(cache_path: &Path, source_key: &str) -> Vec<String> {
    let Ok(contents) = fs::read_to_string(cache_path) else {
        return Vec::new();
    };
    contents
        .lines()
        .filter_map(|line| {
            let (cached_source, _, _) = parse_table_size_cache_line(line)?;
            (cached_source != source_key).then(|| line.to_string())
        })
        .collect()
}

fn table_size_cache_path(storage_dir: &Path) -> PathBuf {
    storage_dir.join(TABLE_SIZE_CACHE_FILE)
}

fn table_size_cache_source_key(path: &Path) -> String {
    path.canonicalize()
        .unwrap_or_else(|_| path.to_path_buf())
        .to_string_lossy()
        .into_owned()
}

fn parse_table_size_cache_line(line: &str) -> Option<(String, String, SavedVariablesTableSize)> {
    let mut fields = line.split('\t');
    let source = unescape_table_size_cache_field(fields.next()?)?;
    let table_path = unescape_table_size_cache_field(fields.next()?)?;
    let array_count = fields.next()?.parse().ok()?;
    let hash_count = fields.next()?.parse().ok()?;
    if fields.next().is_some() {
        return None;
    }
    Some((
        source,
        table_path,
        SavedVariablesTableSize {
            array_count,
            hash_count,
        },
    ))
}

fn escape_table_size_cache_field(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for byte in value.bytes() {
        match byte {
            b'%' => escaped.push_str("%25"),
            b'\t' => escaped.push_str("%09"),
            b'\n' => escaped.push_str("%0A"),
            b'\r' => escaped.push_str("%0D"),
            _ => escaped.push(byte as char),
        }
    }
    escaped
}

fn unescape_table_size_cache_field(value: &str) -> Option<String> {
    let bytes = value.as_bytes();
    let mut output = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] != b'%' {
            output.push(bytes[index]);
            index += 1;
            continue;
        }

        let hex = bytes.get(index + 1..index + 3)?;
        let text = std::str::from_utf8(hex).ok()?;
        let byte = u8::from_str_radix(text, 16).ok()?;
        output.push(byte);
        index += 3;
    }
    String::from_utf8(output).ok()
}
