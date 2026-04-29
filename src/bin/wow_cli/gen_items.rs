//! Generator for items.rs from WoW CSV exports.
//!
//! Reads from ~/Projects/wow/data/:
//!   - ItemSparse.csv
//!   - ItemModifiedAppearance.csv
//!   - ItemAppearance.csv
//!
//! Generates: data/items.rs

use super::csv_util::{escape_str, parse_csv_line, read_csv_records, wow_data_dir};
use std::collections::{BTreeSet, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let wow_data = wow_data_dir();
    let csv_path = wow_data.join("ItemSparse.csv");
    println!("Loading ItemSparse from {}...", csv_path.display());

    let required_ids = collect_required_item_ids();
    println!("Required item IDs: {} (deduplicated)", required_ids.len());

    let file = File::open(&csv_path)?;
    let reader = BufReader::new(file);

    let icon_map = build_icon_map(&wow_data, &required_ids)?;

    std::fs::create_dir_all("data")?;
    let output_path = Path::new("data/items.rs");
    let mut out = File::create(output_path)?;

    write_header(&mut out)?;

    let (count, skipped) = build_item_map(&mut out, reader, &icon_map, &required_ids)?;

    write_lookup_fn(&mut out)?;
    write_tests(&mut out)?;

    println!("Generated {} item entries ({} skipped)", count, skipped);
    println!("Output: {}", output_path.display());
    Ok(())
}

/// Collect item IDs referenced by the simulator from source files.
fn collect_required_item_ids() -> BTreeSet<u32> {
    let mut ids = BTreeSet::new();

    // Equipped items seeded in state defaults: e(211993), e(211995), etc.
    if let Ok(src) = std::fs::read_to_string("src/lua_api/state_defaults.rs") {
        collect_number_literals_after(&src, "e(", &mut ids);
    }

    // Legacy fallback for older layouts.
    if let Ok(src) = std::fs::read_to_string("src/lua_api/state_types.rs") {
        collect_number_literals_after(&src, "e(", &mut ids);
    }

    // Profession data: item_id, output_item_id, reagent item_ids
    if let Ok(src) = std::fs::read_to_string("src/lua_api/globals/profession_data.rs") {
        collect_number_literals_after(&src, "item_id: ", &mut ids);
        collect_number_literals_after(&src, "output_item_id: ", &mut ids);
    }

    // Legacy fallback for older container stubs.
    if let Ok(src) = std::fs::read_to_string("src/lua_api/globals/c_container_api.rs") {
        collect_number_literals_after(&src, "item_id: ", &mut ids);
    }

    // Store/collection items
    if let Ok(src) = std::fs::read_to_string("src/lua_api/globals/c_stubs_api_store.rs") {
        collect_number_literals_after(&src, "item_id: ", &mut ids);
        collect_number_literals_after(&src, "itemID = ", &mut ids);
    }

    // Adventure Guide loot rows.
    let journal_loot = wow_data_dir().join("JournalEncounterItem.csv");
    if let Ok(file) = std::fs::File::open(&journal_loot) {
        if let Ok(records) = read_csv_records(BufReader::new(file)) {
            collect_item_ids_from_journal_loot(&records, &mut ids);
        }
    }

    // Bag items from state.rs default backpack
    if let Ok(src) = std::fs::read_to_string("src/lua_api/state.rs") {
        collect_number_literals_after(&src, "item_id: ", &mut ids);
    }

    // Baseline items always needed
    ids.insert(6948); // Hearthstone (test)

    ids.remove(&0);
    ids
}

fn collect_number_literals_after(src: &str, marker: &str, out: &mut BTreeSet<u32>) {
    let mut rest = src;
    while let Some(idx) = rest.find(marker) {
        rest = &rest[idx + marker.len()..];
        let digits_len = rest.bytes().take_while(|b| b.is_ascii_digit()).count();
        if digits_len == 0 {
            continue;
        }
        if let Ok(id) = rest[..digits_len].parse::<u32>() {
            if id != 0 {
                out.insert(id);
            }
        }
        rest = &rest[digits_len..];
    }
}

fn collect_item_ids_from_journal_loot(records: &[String], out: &mut BTreeSet<u32>) {
    let mut iter = records.iter();
    let Some(header) = iter.next() else {
        return;
    };
    let header_fields = parse_csv_line(header);
    let item_idx = header_fields
        .iter()
        .position(|field| field == "ItemID")
        .unwrap_or(0);
    for record in iter {
        let fields = parse_csv_line(record);
        if let Some(item_id) = fields
            .get(item_idx)
            .and_then(|field| field.parse::<u32>().ok())
        {
            if item_id != 0 {
                out.insert(item_id);
            }
        }
    }
}

/// Build a HashMap<item_id, icon_file_data_id> from ItemModifiedAppearance + ItemAppearance CSVs.
/// Only loads appearances for items in `required_ids`.
fn build_icon_map(
    wow_data: &Path,
    required_ids: &BTreeSet<u32>,
) -> Result<HashMap<u32, u32>, Box<dyn std::error::Error>> {
    let appearance_map = parse_appearance_icons(wow_data)?;
    let mut icon_map = resolve_item_icons(wow_data, required_ids, &appearance_map)?;

    // Non-equippable items (consumables, utility items) can be missing from
    // ItemModifiedAppearance. Seed explicit icon fileDataIDs for baseline items
    // the simulator always places in the backpack.
    for (item_id, icon_file_data_id) in required_item_icon_overrides() {
        if required_ids.contains(item_id) {
            icon_map.entry(*item_id).or_insert(*icon_file_data_id);
        }
    }

    Ok(icon_map)
}

fn required_item_icon_overrides() -> &'static [(u32, u32)] {
    &[
        // Hearthstone
        (6948, 134414), // ICONS/INV_MISC_RUNE_01
        // Refreshing Spring Water
        (159, 132788), // ICONS/INV_Drink_01
        // Tough Hunk of Bread
        (4540, 133964), // ICONS/INV_MISC_FOOD_11
    ]
}

/// Parse ItemAppearance.csv: appearance_id → icon fileDataID.
fn parse_appearance_icons(
    wow_data: &Path,
) -> Result<HashMap<u32, u32>, Box<dyn std::error::Error>> {
    let file = File::open(wow_data.join("ItemAppearance.csv"))?;
    let mut map: HashMap<u32, u32> = HashMap::new();
    for (i, line) in BufReader::new(file).lines().enumerate() {
        let line = line?;
        if i == 0 {
            continue;
        }
        let fields = parse_csv_line(&line);
        if fields.len() < 4 {
            continue;
        }
        let Ok(appearance_id) = fields[0].parse::<u32>() else {
            continue;
        };
        let icon: u32 = fields[3].parse().unwrap_or(0);
        map.insert(appearance_id, icon);
    }
    Ok(map)
}

/// Parse ItemModifiedAppearance.csv: item_id → icon fileDataID (first match per item).
fn resolve_item_icons(
    wow_data: &Path,
    required_ids: &BTreeSet<u32>,
    appearance_map: &HashMap<u32, u32>,
) -> Result<HashMap<u32, u32>, Box<dyn std::error::Error>> {
    let file = File::open(wow_data.join("ItemModifiedAppearance.csv"))?;
    let mut icon_map: HashMap<u32, u32> = HashMap::new();
    for (i, line) in BufReader::new(file).lines().enumerate() {
        let line = line?;
        if i == 0 {
            continue;
        }
        let fields = parse_csv_line(&line);
        if fields.len() < 4 {
            continue;
        }
        let Ok(item_id) = fields[1].parse::<u32>() else {
            continue;
        };
        if !required_ids.contains(&item_id) || icon_map.contains_key(&item_id) {
            continue;
        }
        let appearance_id: u32 = fields[3].parse().unwrap_or(0);
        let icon = appearance_map.get(&appearance_id).copied().unwrap_or(0);
        if icon != 0 {
            icon_map.insert(item_id, icon);
        }
    }
    Ok(icon_map)
}

fn build_item_map(
    out: &mut File,
    reader: BufReader<File>,
    icon_map: &HashMap<u32, u32>,
    required_ids: &BTreeSet<u32>,
) -> Result<(u32, u32), Box<dyn std::error::Error>> {
    let mut builder = phf_codegen::Map::new();
    let mut count = 0u32;
    let mut skipped = 0u32;

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        if i == 0 {
            continue;
        }
        match parse_item_row(&line, icon_map) {
            Some((id, value)) if required_ids.contains(&id) => {
                builder.entry(id, &value);
                count += 1;
            }
            _ => {
                skipped += 1;
            }
        }
    }

    writeln!(
        out,
        "pub static ITEM_DB: phf::Map<u32, ItemInfo> = {};",
        builder.build()
    )?;
    writeln!(out)?;
    Ok((count, skipped))
}

fn parse_item_row(line: &str, icon_map: &HashMap<u32, u32>) -> Option<(u32, String)> {
    let fields = parse_csv_line(line);
    if fields.len() < 102 {
        return None;
    }
    let id: u32 = fields[0].parse().ok()?;
    let name = &fields[6];
    if name.is_empty() {
        return None;
    }
    let icon_file_data_id = icon_map.get(&id).copied().unwrap_or(0);
    Some((id, format_item_info(&fields, name, icon_file_data_id)))
}

fn format_item_info(fields: &[String], name: &str, icon_file_data_id: u32) -> String {
    let escaped_name = escape_str(name);
    let expansion_id: u8 = fields[7].parse().unwrap_or(0);
    let stackable: u32 = fields[46].parse().unwrap_or(1);
    let sell_price: u32 = fields[50].parse().unwrap_or(0);
    let item_level: u16 = fields[83].parse().unwrap_or(0);
    let bonding: u8 = fields[94].parse().unwrap_or(0);
    let required_level: u16 = fields[99].parse().unwrap_or(0);
    let inventory_type: u8 = fields[100].parse().unwrap_or(0);
    let quality: u8 = fields[101].parse().unwrap_or(0);
    let stat_percent_editor = parse_stat_percent_editor(fields);
    let stat_modifier_bonus_stat = parse_stat_modifier_bonus_stat(fields);

    let stat_percent_editor = format_u16_array(&stat_percent_editor);
    let stat_modifier_bonus_stat = format_i16_array(&stat_modifier_bonus_stat);
    format!(
        "ItemInfo {{ name: \"{escaped_name}\", quality: {quality}, item_level: {item_level}, \
         required_level: {required_level}, inventory_type: {inventory_type}, \
         sell_price: {sell_price}, stackable: {stackable}, bonding: {bonding}, \
         expansion_id: {expansion_id}, icon_file_data_id: {icon_file_data_id}, \
         stat_percent_editor: {stat_percent_editor}, \
         stat_modifier_bonus_stat: {stat_modifier_bonus_stat} }}"
    )
}

fn parse_stat_percent_editor(fields: &[String]) -> [u16; 10] {
    std::array::from_fn(|index| fields[26 + index].parse::<u16>().unwrap_or(0))
}

fn parse_stat_modifier_bonus_stat(fields: &[String]) -> [i16; 10] {
    std::array::from_fn(|index| fields[36 + index].parse::<i16>().unwrap_or(-1))
}

fn format_array<T: std::fmt::Display>(values: &[T]) -> String {
    let mut out = String::from("[");
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            out.push_str(", ");
        }
        out.push_str(&value.to_string());
    }
    out.push(']');
    out
}

fn format_u16_array(values: &[u16; 10]) -> String {
    format_array(values)
}

fn format_i16_array(values: &[i16; 10]) -> String {
    format_array(values)
}

fn write_header(out: &mut File) -> std::io::Result<()> {
    writeln!(out, "//! Auto-generated item data from WoW CSV exports.")?;
    writeln!(
        out,
        "//! Do not edit manually - regenerate with: wow-cli generate items"
    )?;
    writeln!(out)?;
    writeln!(out, "#[derive(Debug, Clone)]")?;
    writeln!(out, "pub struct ItemInfo {{")?;
    writeln!(out, "    pub name: &'static str,")?;
    writeln!(out, "    pub quality: u8,")?;
    writeln!(out, "    pub item_level: u16,")?;
    writeln!(out, "    pub required_level: u16,")?;
    writeln!(out, "    pub inventory_type: u8,")?;
    writeln!(out, "    pub sell_price: u32,")?;
    writeln!(out, "    pub stackable: u32,")?;
    writeln!(out, "    pub bonding: u8,")?;
    writeln!(out, "    pub expansion_id: u8,")?;
    writeln!(out, "    pub icon_file_data_id: u32,")?;
    writeln!(out, "    pub stat_percent_editor: [u16; 10],")?;
    writeln!(out, "    pub stat_modifier_bonus_stat: [i16; 10],")?;
    writeln!(out, "}}")?;
    writeln!(out)?;
    Ok(())
}

fn write_lookup_fn(out: &mut File) -> std::io::Result<()> {
    writeln!(
        out,
        "pub fn get_item(id: u32) -> Option<&'static ItemInfo> {{"
    )?;
    writeln!(out, "    ITEM_DB.get(&id)")?;
    writeln!(out, "}}")?;
    Ok(())
}

fn write_tests(out: &mut File) -> std::io::Result<()> {
    writeln!(out)?;
    writeln!(out, "#[cfg(test)]")?;
    writeln!(out, "mod tests {{")?;
    writeln!(out, "    use super::*;")?;
    writeln!(out)?;
    write_test_item_count(out)?;
    write_test_hearthstone(out)?;
    write_test_default_backpack_consumable_icons(out)?;
    write_test_nonexistent_item(out)?;
    writeln!(out, "}}")?;
    Ok(())
}

fn write_test_item_count(out: &mut File) -> std::io::Result<()> {
    writeln!(out, "    #[test]")?;
    writeln!(out, "    fn test_item_count() {{")?;
    writeln!(out, "        assert!(ITEM_DB.len() > 10);")?;
    writeln!(out, "    }}")?;
    writeln!(out)?;
    Ok(())
}

fn write_test_hearthstone(out: &mut File) -> std::io::Result<()> {
    writeln!(out, "    #[test]")?;
    writeln!(out, "    fn test_hearthstone() {{")?;
    writeln!(
        out,
        "        let item = get_item(6948).expect(\"Hearthstone (6948) should exist\");"
    )?;
    writeln!(out, "        assert_eq!(item.name, \"Hearthstone\");")?;
    writeln!(out, "        assert_eq!(item.quality, 1);")?;
    writeln!(out, "        assert_eq!(item.icon_file_data_id, 134414);")?;
    writeln!(out, "    }}")?;
    writeln!(out)?;
    Ok(())
}

fn write_test_default_backpack_consumable_icons(out: &mut File) -> std::io::Result<()> {
    writeln!(out, "    #[test]")?;
    writeln!(out, "    fn test_default_backpack_consumable_icons() {{")?;
    writeln!(
        out,
        "        let water = get_item(159).expect(\"Refreshing Spring Water (159) should exist\");"
    )?;
    writeln!(
        out,
        "        let bread = get_item(4540).expect(\"Tough Hunk of Bread (4540) should exist\");"
    )?;
    writeln!(out, "        assert_eq!(water.icon_file_data_id, 132788);")?;
    writeln!(out, "        assert_eq!(bread.icon_file_data_id, 133964);")?;
    writeln!(out, "    }}")?;
    writeln!(out)?;
    Ok(())
}

fn write_test_nonexistent_item(out: &mut File) -> std::io::Result<()> {
    writeln!(out, "    #[test]")?;
    writeln!(out, "    fn test_nonexistent_item() {{")?;
    writeln!(out, "        assert!(get_item(999_999_999).is_none());")?;
    writeln!(out, "    }}")?;
    Ok(())
}
