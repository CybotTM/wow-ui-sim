//! Generator for data/encounter_journal.rs from WoW Journal* CSV exports.
//!
//! Reads from ~/Projects/wow/data/:
//!   - JournalTier.csv
//!   - JournalTierXInstance.csv
//!   - JournalInstance.csv
//!   - JournalEncounter.csv
//!   - JournalEncounterCreature.csv
//!   - JournalEncounterSection.csv
//!   - JournalEncounterItem.csv
//!   - Map.csv  (only for instance_type lookup on instance map IDs)
//!
//! Generates: data/encounter_journal.rs

use super::csv_util::{escape_str, parse_csv_line, read_csv_records, wow_data_dir};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let wow_data = wow_data_dir();

    let map_types = load_map_instance_types(&wow_data)?;
    let tiers = load_tiers(&wow_data)?;
    let mut instances = load_instances(&wow_data, &map_types)?;
    let mut encounters = load_encounters(&wow_data)?;
    let mut creatures = load_creatures(&wow_data)?;
    let mut sections = load_sections(&wow_data)?;
    let mut loot = load_loot(&wow_data)?;
    let mut tier_instances = load_tier_instances(&wow_data, &tiers)?;

    instances.sort_by_key(|i| i.id);
    encounters.sort_by_key(|e| (e.instance_id, e.order_index, e.id));
    creatures.sort_by_key(|c| (c.encounter_id, c.order_index, c.id));
    sections.sort_by_key(|s| s.id);
    loot.sort_by_key(|l| (l.encounter_id, l.id));
    tier_instances.sort_by_key(|t| (t.tier_order, t.order_index, t.instance_id));

    std::fs::create_dir_all("data")?;
    let output_path = Path::new("data/encounter_journal.rs");
    let mut out = File::create(output_path)?;
    write_module(
        &mut out,
        &tiers,
        &instances,
        &encounters,
        &creatures,
        &sections,
        &loot,
        &tier_instances,
    )?;

    println!("Generated data/encounter_journal.rs:");
    println!("  tiers:          {}", tiers.len());
    println!("  instances:      {}", instances.len());
    println!("  encounters:     {}", encounters.len());
    println!("  creatures:      {}", creatures.len());
    println!("  sections:       {}", sections.len());
    println!("  loot:           {}", loot.len());
    println!("  tier_instances: {}", tier_instances.len());
    Ok(())
}

// ---------------------------------------------------------------------------
// Row types (mirror the generated Rust structs)
// ---------------------------------------------------------------------------

struct Tier {
    id: u32,
    name: String,
    expansion: u32,
    order: u32,
}

struct Instance {
    id: u32,
    name: String,
    description: String,
    map_id: u32,
    bg_file_id: u32,
    button_file_id: u32,
    button_small_file_id: u32,
    lore_file_id: u32,
    flags: u32,
    area_id: u32,
    is_raid: bool,
}

struct Encounter {
    id: u32,
    name: String,
    description: String,
    instance_id: u32,
    dungeon_encounter_id: u32,
    order_index: u32,
    first_section_id: u32,
    map_x: f32,
    map_y: f32,
    ui_map_id: u32,
    flags: u32,
    difficulty_mask: i64,
}

struct Creature {
    id: u32,
    encounter_id: u32,
    name: String,
    description: String,
    display_id: u32,
    icon_file_id: u32,
    order_index: u32,
    model_scene_id: u32,
}

struct Section {
    id: u32,
    encounter_id: u32,
    title: String,
    body: String,
    order_index: u32,
    parent_id: u32,
    first_child_id: u32,
    next_sibling_id: u32,
    kind: u8,
    icon_creature_display_id: u32,
    model_scene_id: u32,
    spell_id: u32,
    icon_file_id: u32,
    flags: u32,
    icon_flags: u32,
    difficulty_mask: i64,
}

struct Loot {
    id: u32,
    encounter_id: u32,
    item_id: u32,
    faction_mask: i32,
    flags: u32,
    difficulty_mask: u32,
}

struct TierInstance {
    tier_id: u32,
    tier_order: u32,
    instance_id: u32,
    order_index: u32,
}

// ---------------------------------------------------------------------------
// Loaders
// ---------------------------------------------------------------------------

fn open_records(path: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(read_csv_records(reader)?)
}

fn header_index(header: &str) -> HashMap<String, usize> {
    parse_csv_line(header)
        .into_iter()
        .enumerate()
        .map(|(i, name)| (name, i))
        .collect()
}

fn field<'a>(fields: &'a [String], idx: &HashMap<String, usize>, key: &str) -> &'a str {
    idx.get(key)
        .and_then(|i| fields.get(*i))
        .map(String::as_str)
        .unwrap_or("")
}

fn parse_u32(s: &str) -> u32 {
    s.parse().unwrap_or(0)
}

fn parse_i32(s: &str) -> i32 {
    s.parse().unwrap_or(0)
}

fn parse_i64(s: &str) -> i64 {
    s.parse().unwrap_or(0)
}

fn parse_u8(s: &str) -> u8 {
    s.parse().unwrap_or(0)
}

fn parse_f32(s: &str) -> f32 {
    s.parse().unwrap_or(0.0)
}

fn load_map_instance_types(
    wow_data: &Path,
) -> Result<HashMap<u32, u8>, Box<dyn std::error::Error>> {
    let records = open_records(&wow_data.join("Map.csv"))?;
    let mut iter = records.iter();
    let header = iter.next().ok_or("empty Map.csv")?;
    let idx = header_index(header);
    let mut map = HashMap::new();
    for record in iter {
        let fields = parse_csv_line(record);
        let id = parse_u32(field(&fields, &idx, "ID"));
        let instance_type = parse_u8(field(&fields, &idx, "InstanceType"));
        map.insert(id, instance_type);
    }
    Ok(map)
}

fn load_tiers(wow_data: &Path) -> Result<Vec<Tier>, Box<dyn std::error::Error>> {
    let records = open_records(&wow_data.join("JournalTier.csv"))?;
    let mut iter = records.iter();
    let header = iter.next().ok_or("empty JournalTier.csv")?;
    let idx = header_index(header);
    let mut rows: Vec<Tier> = iter
        .map(|record| {
            let fields = parse_csv_line(record);
            Tier {
                id: parse_u32(field(&fields, &idx, "ID")),
                name: field(&fields, &idx, "Name_lang").to_string(),
                expansion: parse_u32(field(&fields, &idx, "Expansion")),
                order: 0,
            }
        })
        .collect();
    rows.sort_by_key(|t| t.expansion);
    for (i, tier) in rows.iter_mut().enumerate() {
        tier.order = (i + 1) as u32;
    }
    Ok(rows)
}

fn load_instances(
    wow_data: &Path,
    map_types: &HashMap<u32, u8>,
) -> Result<Vec<Instance>, Box<dyn std::error::Error>> {
    let records = open_records(&wow_data.join("JournalInstance.csv"))?;
    let mut iter = records.iter();
    let header = iter.next().ok_or("empty JournalInstance.csv")?;
    let idx = header_index(header);
    Ok(iter
        .map(|record| {
            let fields = parse_csv_line(record);
            let map_id = parse_u32(field(&fields, &idx, "MapID"));
            let instance_type = map_types.get(&map_id).copied().unwrap_or(0);
            Instance {
                id: parse_u32(field(&fields, &idx, "ID")),
                name: field(&fields, &idx, "Name_lang").to_string(),
                description: field(&fields, &idx, "Description_lang").to_string(),
                map_id,
                bg_file_id: parse_u32(field(&fields, &idx, "BackgroundFileDataID")),
                button_file_id: parse_u32(field(&fields, &idx, "ButtonFileDataID")),
                button_small_file_id: parse_u32(field(&fields, &idx, "ButtonSmallFileDataID")),
                lore_file_id: parse_u32(field(&fields, &idx, "LoreFileDataID")),
                flags: parse_u32(field(&fields, &idx, "Flags")),
                area_id: parse_u32(field(&fields, &idx, "AreaID")),
                is_raid: instance_type == 2,
            }
        })
        .collect())
}

fn load_encounters(wow_data: &Path) -> Result<Vec<Encounter>, Box<dyn std::error::Error>> {
    let records = open_records(&wow_data.join("JournalEncounter.csv"))?;
    let mut iter = records.iter();
    let header = iter.next().ok_or("empty JournalEncounter.csv")?;
    let idx = header_index(header);
    Ok(iter
        .map(|record| {
            let fields = parse_csv_line(record);
            Encounter {
                id: parse_u32(field(&fields, &idx, "ID")),
                name: field(&fields, &idx, "Name_lang").to_string(),
                description: field(&fields, &idx, "Description_lang").to_string(),
                instance_id: parse_u32(field(&fields, &idx, "JournalInstanceID")),
                dungeon_encounter_id: parse_u32(field(&fields, &idx, "DungeonEncounterID")),
                order_index: parse_u32(field(&fields, &idx, "OrderIndex")),
                first_section_id: parse_u32(field(&fields, &idx, "FirstSectionID")),
                map_x: parse_f32(field(&fields, &idx, "Map_0")),
                map_y: parse_f32(field(&fields, &idx, "Map_1")),
                ui_map_id: parse_u32(field(&fields, &idx, "UiMapID")),
                flags: parse_u32(field(&fields, &idx, "Flags")),
                difficulty_mask: parse_i64(field(&fields, &idx, "DifficultyMask")),
            }
        })
        .collect())
}

fn load_creatures(wow_data: &Path) -> Result<Vec<Creature>, Box<dyn std::error::Error>> {
    let records = open_records(&wow_data.join("JournalEncounterCreature.csv"))?;
    let mut iter = records.iter();
    let header = iter.next().ok_or("empty JournalEncounterCreature.csv")?;
    let idx = header_index(header);
    Ok(iter
        .map(|record| {
            let fields = parse_csv_line(record);
            Creature {
                id: parse_u32(field(&fields, &idx, "ID")),
                encounter_id: parse_u32(field(&fields, &idx, "JournalEncounterID")),
                name: field(&fields, &idx, "Name_lang").to_string(),
                description: field(&fields, &idx, "Description_lang").to_string(),
                display_id: parse_u32(field(&fields, &idx, "CreatureDisplayInfoID")),
                icon_file_id: parse_u32(field(&fields, &idx, "FileDataID")),
                order_index: parse_u32(field(&fields, &idx, "OrderIndex")),
                model_scene_id: parse_u32(field(&fields, &idx, "UiModelSceneID")),
            }
        })
        .collect())
}

fn load_sections(wow_data: &Path) -> Result<Vec<Section>, Box<dyn std::error::Error>> {
    let records = open_records(&wow_data.join("JournalEncounterSection.csv"))?;
    let mut iter = records.iter();
    let header = iter.next().ok_or("empty JournalEncounterSection.csv")?;
    let idx = header_index(header);
    Ok(iter
        .map(|record| {
            let fields = parse_csv_line(record);
            Section {
                id: parse_u32(field(&fields, &idx, "ID")),
                encounter_id: parse_u32(field(&fields, &idx, "JournalEncounterID")),
                title: field(&fields, &idx, "Title_lang").to_string(),
                body: field(&fields, &idx, "BodyText_lang").to_string(),
                order_index: parse_u32(field(&fields, &idx, "OrderIndex")),
                parent_id: parse_u32(field(&fields, &idx, "ParentSectionID")),
                first_child_id: parse_u32(field(&fields, &idx, "FirstChildSectionID")),
                next_sibling_id: parse_u32(field(&fields, &idx, "NextSiblingSectionID")),
                kind: parse_u8(field(&fields, &idx, "Type")),
                icon_creature_display_id: parse_u32(field(
                    &fields,
                    &idx,
                    "IconCreatureDisplayInfoID",
                )),
                model_scene_id: parse_u32(field(&fields, &idx, "UiModelSceneID")),
                spell_id: parse_u32(field(&fields, &idx, "SpellID")),
                icon_file_id: parse_u32(field(&fields, &idx, "IconFileDataID")),
                flags: parse_u32(field(&fields, &idx, "Flags")),
                icon_flags: parse_u32(field(&fields, &idx, "IconFlags")),
                difficulty_mask: parse_i64(field(&fields, &idx, "DifficultyMask")),
            }
        })
        .collect())
}

fn load_loot(wow_data: &Path) -> Result<Vec<Loot>, Box<dyn std::error::Error>> {
    let records = open_records(&wow_data.join("JournalEncounterItem.csv"))?;
    let mut iter = records.iter();
    let header = iter.next().ok_or("empty JournalEncounterItem.csv")?;
    let idx = header_index(header);
    Ok(iter
        .map(|record| {
            let fields = parse_csv_line(record);
            Loot {
                id: parse_u32(field(&fields, &idx, "ID")),
                encounter_id: parse_u32(field(&fields, &idx, "JournalEncounterID")),
                item_id: parse_u32(field(&fields, &idx, "ItemID")),
                faction_mask: parse_i32(field(&fields, &idx, "FactionMask")),
                flags: parse_u32(field(&fields, &idx, "Flags")),
                difficulty_mask: parse_u32(field(&fields, &idx, "DifficultyMask")),
            }
        })
        .collect())
}

fn load_tier_instances(
    wow_data: &Path,
    tiers: &[Tier],
) -> Result<Vec<TierInstance>, Box<dyn std::error::Error>> {
    let tier_order: HashMap<u32, u32> = tiers.iter().map(|t| (t.id, t.order)).collect();
    let records = open_records(&wow_data.join("JournalTierXInstance.csv"))?;
    let mut iter = records.iter();
    let header = iter.next().ok_or("empty JournalTierXInstance.csv")?;
    let idx = header_index(header);
    Ok(iter
        .map(|record| {
            let fields = parse_csv_line(record);
            let tier_id = parse_u32(field(&fields, &idx, "JournalTierID"));
            TierInstance {
                tier_id,
                tier_order: tier_order.get(&tier_id).copied().unwrap_or(0),
                instance_id: parse_u32(field(&fields, &idx, "JournalInstanceID")),
                order_index: parse_u32(field(&fields, &idx, "OrderIndex")),
            }
        })
        .collect())
}

// ---------------------------------------------------------------------------
// Output writer
// ---------------------------------------------------------------------------

fn write_module(
    out: &mut File,
    tiers: &[Tier],
    instances: &[Instance],
    encounters: &[Encounter],
    creatures: &[Creature],
    sections: &[Section],
    loot: &[Loot],
    tier_instances: &[TierInstance],
) -> std::io::Result<()> {
    writeln!(
        out,
        "//! Auto-generated EncounterJournal data from WoW CSV exports."
    )?;
    writeln!(
        out,
        "//! Do not edit manually - regenerate with: wow-cli generate encounter-journal"
    )?;
    writeln!(out, "#![allow(clippy::all)]")?;
    writeln!(out)?;
    write_struct_defs(out)?;

    writeln!(out, "pub static TIERS: &[Tier] = &[")?;
    for tier in tiers {
        writeln!(
            out,
            "    Tier {{ id: {}, name: \"{}\", expansion: {}, order: {} }},",
            tier.id,
            escape_str(&tier.name),
            tier.expansion,
            tier.order
        )?;
    }
    writeln!(out, "];")?;
    writeln!(out)?;

    writeln!(out, "pub static INSTANCES: &[Instance] = &[")?;
    for instance in instances {
        writeln!(
            out,
            "    Instance {{ id: {}, name: \"{}\", description: \"{}\", map_id: {}, \
             bg_file_id: {}, button_file_id: {}, button_small_file_id: {}, lore_file_id: {}, \
             flags: {}, area_id: {}, is_raid: {} }},",
            instance.id,
            escape_str(&instance.name),
            escape_str(&instance.description),
            instance.map_id,
            instance.bg_file_id,
            instance.button_file_id,
            instance.button_small_file_id,
            instance.lore_file_id,
            instance.flags,
            instance.area_id,
            instance.is_raid
        )?;
    }
    writeln!(out, "];")?;
    writeln!(out)?;

    writeln!(out, "pub static ENCOUNTERS: &[Encounter] = &[")?;
    for encounter in encounters {
        writeln!(
            out,
            "    Encounter {{ id: {}, name: \"{}\", description: \"{}\", instance_id: {}, \
             dungeon_encounter_id: {}, order_index: {}, first_section_id: {}, \
             map_x: {:.6}, map_y: {:.6}, ui_map_id: {}, flags: {}, difficulty_mask: {} }},",
            encounter.id,
            escape_str(&encounter.name),
            escape_str(&encounter.description),
            encounter.instance_id,
            encounter.dungeon_encounter_id,
            encounter.order_index,
            encounter.first_section_id,
            encounter.map_x,
            encounter.map_y,
            encounter.ui_map_id,
            encounter.flags,
            encounter.difficulty_mask
        )?;
    }
    writeln!(out, "];")?;
    writeln!(out)?;

    writeln!(out, "pub static CREATURES: &[Creature] = &[")?;
    for creature in creatures {
        writeln!(
            out,
            "    Creature {{ id: {}, encounter_id: {}, name: \"{}\", description: \"{}\", \
             display_id: {}, icon_file_id: {}, order_index: {}, model_scene_id: {} }},",
            creature.id,
            creature.encounter_id,
            escape_str(&creature.name),
            escape_str(&creature.description),
            creature.display_id,
            creature.icon_file_id,
            creature.order_index,
            creature.model_scene_id
        )?;
    }
    writeln!(out, "];")?;
    writeln!(out)?;

    writeln!(out, "pub static SECTIONS: &[Section] = &[")?;
    for section in sections {
        writeln!(
            out,
            "    Section {{ id: {}, encounter_id: {}, title: \"{}\", body: \"{}\", \
             order_index: {}, parent_id: {}, first_child_id: {}, next_sibling_id: {}, \
             kind: {}, icon_creature_display_id: {}, model_scene_id: {}, spell_id: {}, \
             icon_file_id: {}, flags: {}, icon_flags: {}, difficulty_mask: {} }},",
            section.id,
            section.encounter_id,
            escape_str(&section.title),
            escape_str(&section.body),
            section.order_index,
            section.parent_id,
            section.first_child_id,
            section.next_sibling_id,
            section.kind,
            section.icon_creature_display_id,
            section.model_scene_id,
            section.spell_id,
            section.icon_file_id,
            section.flags,
            section.icon_flags,
            section.difficulty_mask
        )?;
    }
    writeln!(out, "];")?;
    writeln!(out)?;

    writeln!(out, "pub static LOOT: &[Loot] = &[")?;
    for row in loot {
        writeln!(
            out,
            "    Loot {{ id: {}, encounter_id: {}, item_id: {}, faction_mask: {}, \
             flags: {}, difficulty_mask: {} }},",
            row.id, row.encounter_id, row.item_id, row.faction_mask, row.flags, row.difficulty_mask
        )?;
    }
    writeln!(out, "];")?;
    writeln!(out)?;

    writeln!(out, "pub static TIER_INSTANCES: &[TierInstance] = &[")?;
    for ti in tier_instances {
        writeln!(
            out,
            "    TierInstance {{ tier_id: {}, tier_order: {}, instance_id: {}, order_index: {} }},",
            ti.tier_id, ti.tier_order, ti.instance_id, ti.order_index
        )?;
    }
    writeln!(out, "];")?;
    writeln!(out)?;

    write_lookup_fns(out)?;
    Ok(())
}

fn write_struct_defs(out: &mut File) -> std::io::Result<()> {
    writeln!(
        out,
        "#[derive(Debug, Clone, Copy)]\npub struct Tier {{ \
         pub id: u32, pub name: &'static str, pub expansion: u32, pub order: u32 }}\n"
    )?;
    writeln!(
        out,
        "#[derive(Debug, Clone, Copy)]\npub struct Instance {{ \
         pub id: u32, pub name: &'static str, pub description: &'static str, \
         pub map_id: u32, pub bg_file_id: u32, pub button_file_id: u32, \
         pub button_small_file_id: u32, pub lore_file_id: u32, pub flags: u32, \
         pub area_id: u32, pub is_raid: bool }}\n"
    )?;
    writeln!(
        out,
        "#[derive(Debug, Clone, Copy)]\npub struct Encounter {{ \
         pub id: u32, pub name: &'static str, pub description: &'static str, \
         pub instance_id: u32, pub dungeon_encounter_id: u32, pub order_index: u32, \
         pub first_section_id: u32, pub map_x: f32, pub map_y: f32, pub ui_map_id: u32, \
         pub flags: u32, pub difficulty_mask: i64 }}\n"
    )?;
    writeln!(
        out,
        "#[derive(Debug, Clone, Copy)]\npub struct Creature {{ \
         pub id: u32, pub encounter_id: u32, pub name: &'static str, \
         pub description: &'static str, pub display_id: u32, pub icon_file_id: u32, \
         pub order_index: u32, pub model_scene_id: u32 }}\n"
    )?;
    writeln!(
        out,
        "#[derive(Debug, Clone, Copy)]\npub struct Section {{ \
         pub id: u32, pub encounter_id: u32, pub title: &'static str, \
         pub body: &'static str, pub order_index: u32, pub parent_id: u32, \
         pub first_child_id: u32, pub next_sibling_id: u32, pub kind: u8, \
         pub icon_creature_display_id: u32, pub model_scene_id: u32, pub spell_id: u32, \
         pub icon_file_id: u32, pub flags: u32, pub icon_flags: u32, \
         pub difficulty_mask: i64 }}\n"
    )?;
    writeln!(
        out,
        "#[derive(Debug, Clone, Copy)]\npub struct Loot {{ \
         pub id: u32, pub encounter_id: u32, pub item_id: u32, pub faction_mask: i32, \
         pub flags: u32, pub difficulty_mask: u32 }}\n"
    )?;
    writeln!(
        out,
        "#[derive(Debug, Clone, Copy)]\npub struct TierInstance {{ \
         pub tier_id: u32, pub tier_order: u32, pub instance_id: u32, pub order_index: u32 }}\n"
    )?;
    Ok(())
}

fn write_lookup_fns(out: &mut File) -> std::io::Result<()> {
    writeln!(
        out,
        "pub fn tier_by_order(order: u32) -> Option<&'static Tier> {{
    TIERS.iter().find(|t| t.order == order)
}}

pub fn instance_by_id(id: u32) -> Option<&'static Instance> {{
    INSTANCES.binary_search_by_key(&id, |i| i.id).ok().map(|i| &INSTANCES[i])
}}

pub fn encounter_by_id(id: u32) -> Option<&'static Encounter> {{
    ENCOUNTERS.iter().find(|e| e.id == id)
}}

pub fn section_by_id(id: u32) -> Option<&'static Section> {{
    SECTIONS.binary_search_by_key(&id, |s| s.id).ok().map(|i| &SECTIONS[i])
}}

pub fn instances_for_tier(tier_order: u32, is_raid: bool) -> Vec<&'static Instance> {{
    let mut rows: Vec<(&'static TierInstance, &'static Instance)> = TIER_INSTANCES
        .iter()
        .filter(|t| t.tier_order == tier_order)
        .filter_map(|t| instance_by_id(t.instance_id).map(|i| (t, i)))
        .filter(|(_, i)| i.is_raid == is_raid)
        .collect();
    rows.sort_by_key(|(t, _)| t.order_index);
    rows.into_iter().map(|(_, i)| i).collect()
}}

pub fn encounters_for_instance(instance_id: u32) -> Vec<&'static Encounter> {{
    let mut rows: Vec<&'static Encounter> = ENCOUNTERS
        .iter()
        .filter(|e| e.instance_id == instance_id)
        .collect();
    rows.sort_by_key(|e| e.order_index);
    rows
}}

pub fn creatures_for_encounter(encounter_id: u32) -> Vec<&'static Creature> {{
    let mut rows: Vec<&'static Creature> = CREATURES
        .iter()
        .filter(|c| c.encounter_id == encounter_id)
        .collect();
    rows.sort_by_key(|c| c.order_index);
    rows
}}

pub fn loot_for_encounter(encounter_id: u32) -> Vec<&'static Loot> {{
    LOOT.iter()
        .filter(|l| l.encounter_id == encounter_id)
        .collect()
}}

pub fn loot_item_ids() -> impl Iterator<Item = u32> {{
    LOOT.iter().map(|l| l.item_id)
}}
"
    )?;
    Ok(())
}
