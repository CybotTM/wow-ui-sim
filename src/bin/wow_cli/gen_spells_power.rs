use super::csv_util::parse_csv_line;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

const POWER_TYPE_NAMES: &[(&'static str, &str)] = &[
    ("-2", "HEALTH"),
    ("0", "MANA"),
    ("1", "RAGE"),
    ("2", "FOCUS"),
    ("3", "ENERGY"),
    ("4", "COMBO_POINTS"),
    ("5", "RUNES"),
    ("6", "RUNIC_POWER"),
    ("7", "SOUL_SHARDS"),
    ("8", "LUNAR_POWER"),
    ("9", "HOLY_POWER"),
    ("10", "ALTERNATE_POWER"),
    ("11", "MAELSTROM"),
    ("12", "CHI"),
    ("13", "INSANITY"),
    ("17", "FURY"),
    ("19", "ESSENCE"),
];

const SPELL_POWER_TESTS: &[&str] = &[
    "",
    "#[cfg(test)]",
    "mod tests {",
    "    use super::*;",
    "",
    "    #[test]",
    "    fn test_flash_of_light_power_cost() {",
    "        let costs = get_spell_power(19750).expect(\"Flash of Light should have power cost\");",
    "        assert!(!costs.is_empty());",
    "        assert_eq!(costs[0].power_type, 0); // MANA",
    "    }",
    "",
    "    #[test]",
    "    fn test_spell_power_count() {",
    "        assert!(SPELL_POWER_DB.len() > 100);",
    "    }",
    "",
    "    #[test]",
    "    fn test_no_power_for_unknown() {",
    "        assert!(get_spell_power(999_999_999).is_none());",
    "    }",
    "}",
];

const POWER_TYPE_NAME_FN_HEADER: &str =
    "pub fn power_type_name(power_type: i8) -> &'static str {\n    match power_type {\n";
const POWER_TYPE_NAME_FN_FOOTER: &str = "        _ => \"MANA\",\n    }\n}";

/// Parsed SpellPower row.
#[derive(Clone)]
pub(super) struct SpellPowerRow {
    power_type: i8,
    mana_cost: i32,
    cost_pct: f32,
    cost_max_pct: f32,
    cost_per_sec: f32,
    required_aura_id: u32,
    optional_cost: i32,
    order_index: u32,
}

/// Load SpellPower.csv grouped by SpellID, sorted by OrderIndex.
///
/// Columns: ID(0), OrderIndex(1), ManaCost(2), ManaCostPerLevel(3),
/// ManaPerSecond(4), PowerDisplayID(5), AltPowerBarID(6), PowerCostPct(7),
/// PowerCostMaxPct(8), OptionalCostPct(9), PowerPctPerSecond(10),
/// PowerType(11), RequiredAuraSpellID(12), OptionalCost(13), SpellID(14)
pub(super) fn load_spell_power(
    path: &Path,
) -> Result<HashMap<u32, Vec<SpellPowerRow>>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut map: HashMap<u32, Vec<SpellPowerRow>> = HashMap::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        if i == 0 {
            continue;
        }
        let f = parse_csv_line(&line);
        if f.len() < 15 {
            continue;
        }
        let spell_id: u32 = match f[14].parse() {
            Ok(id) => id,
            Err(_) => continue,
        };
        let row = SpellPowerRow {
            order_index: f[1].parse().unwrap_or(0),
            mana_cost: f[2].parse().unwrap_or(0),
            cost_per_sec: f[4].parse().unwrap_or(0.0),
            cost_pct: f[7].parse().unwrap_or(0.0),
            cost_max_pct: f[8].parse().unwrap_or(0.0),
            power_type: f[11].parse().unwrap_or(0),
            required_aura_id: f[12].parse().unwrap_or(0),
            optional_cost: f[13].parse().unwrap_or(0),
        };
        map.entry(spell_id).or_default().push(row);
    }

    for entries in map.values_mut() {
        entries.sort_by_key(|e| e.order_index);
    }
    Ok(map)
}

/// Generate data/spell_power.rs with static arrays + phf map.
pub(super) fn write_spell_power(
    out: &mut File,
    spell_power: &HashMap<u32, Vec<SpellPowerRow>>,
) -> Result<u32, Box<dyn std::error::Error>> {
    let mut spell_ids: Vec<u32> = spell_power.keys().copied().collect();
    spell_ids.sort();

    write_spell_power_header(out)?;
    write_spell_power_arrays(out, &spell_ids, spell_power)?;
    write_spell_power_phf_map(out, &spell_ids)?;
    write_spell_power_lookup_fns(out)?;
    write_spell_power_tests(out)?;

    Ok(spell_ids.len() as u32)
}

fn write_spell_power_header(out: &mut File) -> std::io::Result<()> {
    writeln!(
        out,
        "//! Auto-generated spell power cost data from WoW SpellPower.csv."
    )?;
    writeln!(
        out,
        "//! Do not edit manually - regenerate with: wow-cli generate spells"
    )?;
    writeln!(out)?;
    writeln!(out, "#[derive(Debug, Clone, Copy)]")?;
    writeln!(out, "pub struct SpellPowerCost {{")?;
    writeln!(out, "    pub power_type: i8,")?;
    writeln!(out, "    pub mana_cost: i32,")?;
    writeln!(out, "    pub cost_pct: f32,")?;
    writeln!(out, "    pub cost_max_pct: f32,")?;
    writeln!(out, "    pub cost_per_sec: f32,")?;
    writeln!(out, "    pub required_aura_id: u32,")?;
    writeln!(out, "    pub optional_cost: i32,")?;
    writeln!(out, "}}")?;
    writeln!(out)
}

fn write_spell_power_arrays(
    out: &mut File,
    spell_ids: &[u32],
    spell_power: &HashMap<u32, Vec<SpellPowerRow>>,
) -> std::io::Result<()> {
    for &spell_id in spell_ids {
        write_spell_power_array(out, spell_id, &spell_power[&spell_id])?;
    }
    writeln!(out)
}

fn write_spell_power_array(
    out: &mut File,
    spell_id: u32,
    entries: &[SpellPowerRow],
) -> std::io::Result<()> {
    writeln!(
        out,
        "static SPELL_POWER_{spell_id}: [SpellPowerCost; {}] = [",
        entries.len()
    )?;
    write_spell_power_entries(out, entries)?;
    writeln!(out, "];")?;
    Ok(())
}

fn write_spell_power_entries(out: &mut File, entries: &[SpellPowerRow]) -> std::io::Result<()> {
    for entry in entries {
        write_spell_power_entry(out, entry)?;
    }
    Ok(())
}

fn write_spell_power_entry(out: &mut File, entry: &SpellPowerRow) -> std::io::Result<()> {
    writeln!(
        out,
        "    SpellPowerCost {{ power_type: {}, mana_cost: {}, cost_pct: {:?}_f32, \
         cost_max_pct: {:?}_f32, cost_per_sec: {:?}_f32, required_aura_id: {}, \
         optional_cost: {} }},",
        entry.power_type,
        entry.mana_cost,
        entry.cost_pct,
        entry.cost_max_pct,
        entry.cost_per_sec,
        entry.required_aura_id,
        entry.optional_cost,
    )
}

fn write_spell_power_phf_map(
    out: &mut File,
    spell_ids: &[u32],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = phf_codegen::Map::new();
    for &spell_id in spell_ids {
        builder.entry(spell_id, &format!("&SPELL_POWER_{spell_id}"));
    }
    writeln!(
        out,
        "pub static SPELL_POWER_DB: phf::Map<u32, &'static [SpellPowerCost]> = {};",
        builder.build()
    )?;
    writeln!(out)?;
    Ok(())
}

fn write_spell_power_lookup_fns(out: &mut File) -> std::io::Result<()> {
    write_get_spell_power_fn(out)?;
    write_power_type_name_fn(out)?;
    Ok(())
}

fn write_get_spell_power_fn(out: &mut File) -> std::io::Result<()> {
    writeln!(
        out,
        "pub fn get_spell_power(id: u32) -> Option<&'static [SpellPowerCost]> {{"
    )?;
    writeln!(out, "    SPELL_POWER_DB.get(&id).copied()")?;
    writeln!(out, "}}")?;
    writeln!(out)?;
    Ok(())
}

fn write_power_type_name_fn(out: &mut File) -> std::io::Result<()> {
    write!(out, "{POWER_TYPE_NAME_FN_HEADER}")?;
    write_power_type_name_arms(out)?;
    write!(out, "{POWER_TYPE_NAME_FN_FOOTER}")
}

fn write_power_type_name_arms(out: &mut File) -> std::io::Result<()> {
    for (val, name) in POWER_TYPE_NAMES {
        writeln!(out, "        {val} => \"{name}\",")?;
    }
    Ok(())
}

fn write_spell_power_tests(out: &mut File) -> std::io::Result<()> {
    write_literal_lines(out, SPELL_POWER_TESTS)
}

fn write_literal_lines(out: &mut File, lines: &[&str]) -> std::io::Result<()> {
    for line in lines {
        writeln!(out, "{line}")?;
    }
    Ok(())
}
