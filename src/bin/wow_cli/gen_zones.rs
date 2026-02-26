//! Generator for zones.rs from WoW AreaTable CSV export.
//!
//! Reads from ~/Projects/wow/data/:
//!   - AreaTable.csv
//!
//! Generates: data/zones.rs

use super::csv_util::{escape_str, parse_csv_line, wow_data_dir};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let wow_data = wow_data_dir();
    let csv_path = wow_data.join("AreaTable.csv");
    println!("Loading AreaTable from {}...", csv_path.display());

    let file = File::open(&csv_path)?;
    let reader = BufReader::new(file);

    std::fs::create_dir_all("data")?;
    let output_path = Path::new("data/zones.rs");
    let mut out = File::create(output_path)?;

    write_header(&mut out)?;
    let (count, skipped) = build_area_map(&mut out, reader)?;
    write_lookup_fn(&mut out)?;
    write_tests(&mut out)?;

    println!("Generated {} area entries ({} skipped)", count, skipped);
    println!("Output: {}", output_path.display());
    Ok(())
}

fn build_area_map(
    out: &mut File,
    reader: BufReader<File>,
) -> Result<(u32, u32), Box<dyn std::error::Error>> {
    let mut builder = phf_codegen::Map::new();
    let mut count = 0u32;
    let mut skipped = 0u32;

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        if i == 0 {
            continue;
        }
        match parse_area_row(&line) {
            Some((id, value)) => {
                builder.entry(id, &value);
                count += 1;
            }
            None => {
                skipped += 1;
            }
        }
    }

    writeln!(
        out,
        "pub static AREA_DB: phf::Map<u32, AreaInfo> = {};",
        builder.build()
    )?;
    writeln!(out)?;
    Ok((count, skipped))
}

fn parse_area_row(line: &str) -> Option<(u32, String)> {
    let fields = parse_csv_line(line);
    if fields.len() < 5 {
        return None;
    }
    let id: u32 = fields[0].parse().ok()?;
    let name = &fields[2];
    if name.is_empty() {
        return None;
    }

    let escaped_name = escape_str(name);
    let parent_area_id: u32 = fields[4].parse().unwrap_or(0);

    let value = format!(
        "AreaInfo {{ name: \"{}\", parent_area_id: {} }}",
        escaped_name, parent_area_id
    );
    Some((id, value))
}

fn write_header(out: &mut File) -> std::io::Result<()> {
    writeln!(out, "//! Auto-generated area/zone data from WoW AreaTable CSV.")?;
    writeln!(
        out,
        "//! Do not edit manually - regenerate with: wow-cli generate zones"
    )?;
    writeln!(out)?;
    writeln!(out, "#[derive(Debug, Clone)]")?;
    writeln!(out, "pub struct AreaInfo {{")?;
    writeln!(out, "    pub name: &'static str,")?;
    writeln!(out, "    pub parent_area_id: u32,")?;
    writeln!(out, "}}")?;
    writeln!(out)?;
    Ok(())
}

fn write_lookup_fn(out: &mut File) -> std::io::Result<()> {
    writeln!(out, "pub fn get_area(id: u32) -> Option<&'static AreaInfo> {{")?;
    writeln!(out, "    AREA_DB.get(&id)")?;
    writeln!(out, "}}")?;
    Ok(())
}

fn write_tests(out: &mut File) -> std::io::Result<()> {
    writeln!(out)?;
    writeln!(out, "#[cfg(test)]")?;
    writeln!(out, "mod tests {{")?;
    writeln!(out, "    use super::*;")?;
    write_test_area_count(out)?;
    write_test_stormwind(out)?;
    write_test_trade_district(out)?;
    write_test_nonexistent(out)?;
    writeln!(out, "}}")?;
    Ok(())
}

fn write_test_area_count(out: &mut File) -> std::io::Result<()> {
    writeln!(out)?;
    writeln!(out, "    #[test]")?;
    writeln!(out, "    fn test_area_count() {{")?;
    writeln!(out, "        assert!(AREA_DB.len() > 5000);")?;
    writeln!(out, "    }}")?;
    Ok(())
}

fn write_test_stormwind(out: &mut File) -> std::io::Result<()> {
    writeln!(out)?;
    writeln!(out, "    #[test]")?;
    writeln!(out, "    fn test_stormwind_city() {{")?;
    writeln!(
        out,
        "        let area = get_area(1519).expect(\"Stormwind City (1519) should exist\");"
    )?;
    writeln!(out, "        assert_eq!(area.name, \"Stormwind City\");")?;
    writeln!(out, "        assert_eq!(area.parent_area_id, 0);")?;
    writeln!(out, "    }}")?;
    Ok(())
}

fn write_test_trade_district(out: &mut File) -> std::io::Result<()> {
    writeln!(out)?;
    writeln!(out, "    #[test]")?;
    writeln!(out, "    fn test_trade_district() {{")?;
    writeln!(
        out,
        "        let area = get_area(5148).expect(\"Trade District (5148) should exist\");"
    )?;
    writeln!(out, "        assert_eq!(area.name, \"Trade District\");")?;
    writeln!(out, "        assert_eq!(area.parent_area_id, 1519);")?;
    writeln!(out, "    }}")?;
    Ok(())
}

fn write_test_nonexistent(out: &mut File) -> std::io::Result<()> {
    writeln!(out)?;
    writeln!(out, "    #[test]")?;
    writeln!(out, "    fn test_nonexistent_area() {{")?;
    writeln!(out, "        assert!(get_area(999_999_999).is_none());")?;
    writeln!(out, "    }}")?;
    Ok(())
}
