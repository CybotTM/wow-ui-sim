//! Generator for currencies.rs from WoW CurrencyTypes CSV export.
//!
//! Reads from ~/Projects/wow/data/:
//!   - CurrencyTypes.csv
//!
//! Generates: data/currencies.rs

use super::csv_util::{escape_str, parse_csv_line, read_csv_records, wow_data_dir};
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let wow_data = wow_data_dir();
    let csv_path = wow_data.join("CurrencyTypes.csv");
    println!("Loading CurrencyTypes from {}...", csv_path.display());

    let file = File::open(&csv_path)?;
    let records = read_csv_records(BufReader::new(file))?;

    std::fs::create_dir_all("data")?;
    let output_path = Path::new("data/currencies.rs");
    let mut out = File::create(output_path)?;

    write_header(&mut out)?;
    let (count, skipped) = write_currency_entries(&mut out, records)?;
    writeln!(out, "}};")?;
    write_lookup_fn(&mut out)?;
    write_tests(&mut out)?;

    println!("Generated {count} currency entries ({skipped} skipped)");
    println!("Output: {}", output_path.display());
    Ok(())
}

fn write_header(out: &mut File) -> std::io::Result<()> {
    write_literal_lines(out, CURRENCY_HEADER)?;
    write_currency_type_struct(out)?;
    write_map_start(out)
}

const CURRENCY_HEADER: &[&str] = &[
    "//! Auto-generated currency data from WoW CurrencyTypes CSV.",
    "//! Source: https://wago.tools/db2/CurrencyTypes/csv",
    "//! Do not edit manually - regenerate with: wow-cli generate currencies",
    "",
    "use phf::phf_map;",
    "",
];

fn write_literal_lines(out: &mut File, lines: &[&str]) -> std::io::Result<()> {
    for line in lines {
        writeln!(out, "{line}")?;
    }
    Ok(())
}

fn write_currency_type_struct(out: &mut File) -> std::io::Result<()> {
    write_literal_lines(
        out,
        &[
            "#[derive(Debug, Clone)]",
            "pub struct CurrencyTypeInfo {",
            "    pub name: &'static str,",
            "    pub description: &'static str,",
            "    pub icon_file_id: u32,",
            "    pub max_quantity: i32,",
            "    pub max_weekly_quantity: i32,",
            "    pub quality: i32,",
            "    pub transfer_percentage: Option<f64>,",
            "}",
            "",
        ],
    )
}

fn write_map_start(out: &mut File) -> std::io::Result<()> {
    out.write_all(b"pub static CURRENCY_TYPES: phf::Map<i32, CurrencyTypeInfo> = phf_map! ")?;
    out.write_all(&[123, b'\n'])
}

fn write_currency_entries(
    out: &mut File,
    records: Vec<String>,
) -> Result<(u32, u32), Box<dyn std::error::Error>> {
    let mut count = 0;
    let mut skipped = 0;

    for (index, line) in records.iter().enumerate() {
        if index == 0 {
            continue;
        }

        match parse_currency_row(line) {
            Some((currency_id, value)) => {
                writeln!(out, "    {currency_id}i32 => {value},")?;
                count += 1;
            }
            None => skipped += 1,
        }
    }

    Ok((count, skipped))
}

fn parse_currency_row(line: &str) -> Option<(i32, String)> {
    let fields = parse_csv_line(line);
    if fields.len() < 18 {
        return None;
    }

    let currency_id: i32 = fields[0].parse().ok()?;
    let name = fields[1].trim();
    if name.is_empty() {
        return None;
    }

    let description = fields[2].trim();
    let icon_file_id = parse_u32_field(&fields[4]);
    let max_quantity = parse_i32_field(&fields[7]);
    let max_weekly_quantity = parse_i32_field(&fields[8]);
    let quality = parse_i32_field(&fields[9]);
    let transfer_percentage = parse_transfer_percentage(&fields[17]);

    let value = format!(
        "CurrencyTypeInfo {{ name: \"{}\", description: \"{}\", icon_file_id: {}, max_quantity: {}, max_weekly_quantity: {}, quality: {}, transfer_percentage: {} }}",
        escape_str(name),
        escape_str(description),
        icon_file_id,
        max_quantity,
        max_weekly_quantity,
        quality,
        transfer_percentage
    );

    Some((currency_id, value))
}

fn parse_i32_field(value: &str) -> i32 {
    value.parse().unwrap_or(0)
}

fn parse_u32_field(value: &str) -> u32 {
    value.parse().unwrap_or(0)
}

fn parse_transfer_percentage(value: &str) -> String {
    match value.parse::<f64>() {
        Ok(percentage) if percentage > 0.0 => format!("Some({percentage:?})"),
        _ => "None".to_string(),
    }
}

fn write_lookup_fn(out: &mut File) -> std::io::Result<()> {
    writeln!(out)?;
    writeln!(
        out,
        "pub fn get_currency_type(id: i32) -> Option<&'static CurrencyTypeInfo> {{"
    )?;
    writeln!(out, "    CURRENCY_TYPES.get(&id)")?;
    writeln!(out, "}}")?;
    Ok(())
}

fn write_tests(out: &mut File) -> std::io::Result<()> {
    writeln!(out)?;
    writeln!(out, "#[cfg(test)]")?;
    writeln!(out, "mod tests {{")?;
    writeln!(out, "    use super::*;")?;
    writeln!(out)?;
    write_test_currency_count(out)?;
    write_test_epicureans_award(out)?;
    write_test_unknown_currency(out)?;
    writeln!(out, "}}")?;
    Ok(())
}

fn write_test_currency_count(out: &mut File) -> std::io::Result<()> {
    writeln!(out, "    #[test]")?;
    writeln!(out, "    fn test_currency_count() {{")?;
    writeln!(out, "        assert!(CURRENCY_TYPES.len() > 500);")?;
    writeln!(out, "    }}")?;
    writeln!(out)?;
    Ok(())
}

fn write_test_epicureans_award(out: &mut File) -> std::io::Result<()> {
    writeln!(out, "    #[test]")?;
    writeln!(out, "    fn test_epicureans_award() {{")?;
    writeln!(
        out,
        "        let currency = get_currency_type(81).expect(\"Epicurean's Award should exist\");"
    )?;
    writeln!(
        out,
        "        assert_eq!(currency.name, \"Epicurean's Award\");"
    )?;
    writeln!(out, "        assert!(currency.icon_file_id > 0);")?;
    writeln!(out, "    }}")?;
    writeln!(out)?;
    Ok(())
}

fn write_test_unknown_currency(out: &mut File) -> std::io::Result<()> {
    writeln!(out, "    #[test]")?;
    writeln!(out, "    fn test_unknown_currency() {{")?;
    writeln!(
        out,
        "        assert!(get_currency_type(999_999).is_none());"
    )?;
    writeln!(out, "    }}")?;
    Ok(())
}
