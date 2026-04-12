//! Generator for quest_ui_map.rs from WoW QuestPOIBlob CSV export.
//!
//! Reads from data/db2/:
//!   - QuestPOIBlob.csv
//!
//! Only includes quest IDs referenced by the simulator's quest data
//! (same sampling pattern as gen_spells.rs).
//!
//! Generates: data/quest_ui_map.rs

use super::csv_util::parse_csv_line;
use std::collections::{BTreeSet, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let csv_path = Path::new("data/db2/QuestPOIBlob.csv");
    println!("Loading QuestPOIBlob from {}...", csv_path.display());

    let required_ids = collect_required_quest_ids()?;
    println!("Required quest IDs: {}", required_ids.len());

    let file = File::open(csv_path)?;
    let reader = BufReader::new(file);

    std::fs::create_dir_all("data")?;
    let output_path = Path::new("data/quest_ui_map.rs");
    let mut out = File::create(output_path)?;

    let quest_map = collect_quest_map(reader, &required_ids)?;
    let count = quest_map.len();
    write_file(&mut out, quest_map)?;

    println!("Generated {} quest UI map entries", count);
    println!("Output: {}", output_path.display());
    Ok(())
}

/// Collect quest IDs the simulator actually references.
///
/// Sources:
/// - `src/lua_api/globals/c_quest_api.rs`: quest_id fields in QUEST_LOG
/// - `data/quest_poi_blobs.rs`: quest_id fields in POI blob data
fn collect_required_quest_ids() -> Result<BTreeSet<u32>, Box<dyn std::error::Error>> {
    let mut ids = BTreeSet::new();

    for path in &[
        "src/lua_api/globals/c_quest_api.rs",
        "data/quest_poi_blobs.rs",
    ] {
        let src = std::fs::read_to_string(path)?;
        for line in src.lines() {
            let trimmed = line.trim();
            if let Some(rest) = trimmed.strip_prefix("quest_id:") {
                if let Some(id) = rest.trim().trim_end_matches(',').parse::<u32>().ok() {
                    ids.insert(id);
                }
            }
        }
    }

    Ok(ids)
}

/// Collect quest→UiMapID mapping, filtered to required IDs only.
///
/// Priority per quest:
/// 1. First row with ObjectiveIndex == -1 (quest turnin location)
/// 2. First row's UiMapID if no ObjectiveIndex -1 row exists
/// 3. Skip rows where UiMapID == 0
fn collect_quest_map(
    reader: BufReader<File>,
    required_ids: &BTreeSet<u32>,
) -> Result<HashMap<u32, u32>, Box<dyn std::error::Error>> {
    // Per quest: (turnin_ui_map_id, first_ui_map_id)
    let mut entries: HashMap<u32, (Option<u32>, Option<u32>)> = HashMap::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        if i == 0 {
            continue; // skip header
        }
        if let Some((quest_id, ui_map_id, objective_index)) = parse_row(&line) {
            if !required_ids.contains(&quest_id) {
                continue;
            }
            let entry = entries.entry(quest_id).or_insert((None, None));
            if objective_index == -1 && entry.0.is_none() {
                entry.0 = Some(ui_map_id);
            }
            if entry.1.is_none() {
                entry.1 = Some(ui_map_id);
            }
        }
    }

    let result = entries
        .into_iter()
        .filter_map(|(quest_id, (turnin, first))| {
            let ui_map_id = turnin.or(first)?;
            Some((quest_id, ui_map_id))
        })
        .collect();

    Ok(result)
}

fn parse_row(line: &str) -> Option<(u32, u32, i32)> {
    // Columns: ID,MapID,UiMapID,Flags,NumPoints,QuestID,ObjectiveIndex,...
    let fields = parse_csv_line(line);
    if fields.len() < 7 {
        return None;
    }
    let ui_map_id: u32 = fields[2].parse().ok()?;
    if ui_map_id == 0 {
        return None;
    }
    let quest_id: u32 = fields[5].parse().ok()?;
    let objective_index: i32 = fields[6].parse().ok()?;
    Some((quest_id, ui_map_id, objective_index))
}

fn write_file(out: &mut File, quest_map: HashMap<u32, u32>) -> std::io::Result<()> {
    write_header(out)?;
    if quest_map.is_empty() {
        // No matching quests — write a simple empty lookup
        writeln!(out, "pub fn get_quest_ui_map_id(_quest_id: u32) -> u32 {{")?;
        writeln!(out, "    0")?;
        writeln!(out, "}}")?;
    } else {
        build_phf_map(out, &quest_map)?;
        write_lookup_fn(out)?;
    }
    write_tests(out, &quest_map)?;
    Ok(())
}

fn write_header(out: &mut File) -> std::io::Result<()> {
    writeln!(
        out,
        "//! Auto-generated quest UI map data from WoW QuestPOIBlob CSV."
    )?;
    writeln!(
        out,
        "//! Do not edit manually - regenerate with: wow-cli generate quest-poi"
    )?;
    writeln!(out)?;
    Ok(())
}

fn build_phf_map(out: &mut File, quest_map: &HashMap<u32, u32>) -> std::io::Result<()> {
    let mut builder = phf_codegen::Map::new();
    let mut entries: Vec<(u32, u32)> = quest_map.iter().map(|(&k, &v)| (k, v)).collect();
    entries.sort_by_key(|(quest_id, _)| *quest_id);
    for (quest_id, ui_map_id) in entries {
        builder.entry(quest_id, &ui_map_id.to_string());
    }
    writeln!(
        out,
        "static QUEST_UI_MAP: phf::Map<u32, u32> = {};",
        builder.build()
    )?;
    writeln!(out)?;
    Ok(())
}

fn write_lookup_fn(out: &mut File) -> std::io::Result<()> {
    writeln!(out, "pub fn get_quest_ui_map_id(quest_id: u32) -> u32 {{")?;
    writeln!(out, "    QUEST_UI_MAP.get(&quest_id).copied().unwrap_or(0)")?;
    writeln!(out, "}}")?;
    Ok(())
}

fn write_tests(out: &mut File, quest_map: &HashMap<u32, u32>) -> std::io::Result<()> {
    writeln!(out)?;
    writeln!(out, "#[cfg(test)]")?;
    writeln!(out, "mod tests {{")?;
    writeln!(out, "    use super::*;")?;
    writeln!(out)?;
    writeln!(out, "    #[test]")?;
    writeln!(out, "    fn test_unknown_quest_returns_zero() {{")?;
    writeln!(
        out,
        "        assert_eq!(get_quest_ui_map_id(999_999_999), 0);"
    )?;
    writeln!(out, "    }}")?;

    // Generate a test for the first known quest if any exist
    if let Some((&quest_id, &ui_map_id)) = quest_map.iter().next() {
        writeln!(out)?;
        writeln!(out, "    #[test]")?;
        writeln!(out, "    fn test_known_quest() {{")?;
        writeln!(
            out,
            "        assert_eq!(get_quest_ui_map_id({}), {});",
            quest_id, ui_map_id
        )?;
        writeln!(out, "    }}")?;
    }

    writeln!(out, "}}")?;
    Ok(())
}
