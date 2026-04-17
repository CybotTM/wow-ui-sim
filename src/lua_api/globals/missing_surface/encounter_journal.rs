//! `C_EncounterJournal` static probe surface.
//!
//! Migrates 2 entries off `NAMESPACE_NIL_STUBS`:
//!
//! - `C_EncounterJournal.GetEncounterInfo(encounterID)` — returns the
//!   8-value retail tuple `(name, description, encounterID, rootSectionID,
//!   linkSection, journalInstanceID, dungeonEncounterID, instanceID)` for
//!   a seeded encounter, or nothing for an unknown id.
//! - `C_EncounterJournal.GetInstanceInfo(instanceID)` — returns the
//!   9-value retail tuple `(name, description, bgImage, buttonImage1,
//!   loreImage, buttonImage2, dungeonAreaMapID, linkRaidID, linkDungeonID)`
//!   for a seeded instance, or nothing for an unknown id.
//!
//! Data covers Dragonflight + War Within raids (Vault of the Incarnates,
//! Aberrus, Amirdrassil, Nerub-ar Palace) with real retail encounter and
//! instance IDs from Wowpedia / wago.tools.

use super::ensure_namespace;
use crate::lua_api::methods::create_string;
use crate::lua_bridge::{FromStack, table_set_rust_fn};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

// ---------------------------------------------------------------------------
// Seed data
// ---------------------------------------------------------------------------

struct EncounterRow {
    encounter_id: u32,
    name: &'static str,
    description: &'static str,
    root_section_id: u32,
    link_section: &'static str,
    journal_instance_id: u32,
    dungeon_encounter_id: u32,
    instance_id: u32,
}

struct InstanceRow {
    instance_id: u32,
    name: &'static str,
    description: &'static str,
    bg_image: &'static str,
    button_image1: &'static str,
    lore_image: &'static str,
    button_image2: &'static str,
    dungeon_area_map_id: u32,
    link_raid_id: u32,
    link_dungeon_id: u32,
}

// Encounter IDs, names and instance IDs from wowpedia / wago.tools.
// journalInstanceID is the EJ instance id (distinct from the LFG instance id).
static ENCOUNTERS: &[EncounterRow] = &[
    // --- Vault of the Incarnates (instance 1200, journalInstanceID 1193) ---
    EncounterRow {
        encounter_id: 2587,
        name: "Eranog",
        description: "Commander of the Primalist armies.",
        root_section_id: 14981,
        link_section: "",
        journal_instance_id: 1193,
        dungeon_encounter_id: 2587,
        instance_id: 1200,
    },
    EncounterRow {
        encounter_id: 2639,
        name: "The Primal Council",
        description: "Four Primalist commanders united in purpose.",
        root_section_id: 15052,
        link_section: "",
        journal_instance_id: 1193,
        dungeon_encounter_id: 2639,
        instance_id: 1200,
    },
    EncounterRow {
        encounter_id: 2590,
        name: "Sennarth, The Cold Breath",
        description: "Ancient spider corrupted by the Primalists.",
        root_section_id: 14984,
        link_section: "",
        journal_instance_id: 1193,
        dungeon_encounter_id: 2590,
        instance_id: 1200,
    },
    EncounterRow {
        encounter_id: 2592,
        name: "Raszageth the Storm-Eater",
        description: "Final boss of the Vault of the Incarnates.",
        root_section_id: 14986,
        link_section: "",
        journal_instance_id: 1193,
        dungeon_encounter_id: 2592,
        instance_id: 1200,
    },
    // --- Aberrus, the Shadowed Crucible (instance 1208, journalInstanceID 1208) ---
    EncounterRow {
        encounter_id: 2688,
        name: "Kazzara, the Hellforged",
        description: "A dracthyr twisted by Neltharion's experiments.",
        root_section_id: 15738,
        link_section: "",
        journal_instance_id: 1208,
        dungeon_encounter_id: 2688,
        instance_id: 1208,
    },
    EncounterRow {
        encounter_id: 2693,
        name: "Rashok, the Elder",
        description: "An elder nathrezim empowered by the Shadowflame.",
        root_section_id: 15743,
        link_section: "",
        journal_instance_id: 1208,
        dungeon_encounter_id: 2693,
        instance_id: 1208,
    },
    // --- Amirdrassil, the Dream's Hope (instance 2549, journalInstanceID 1207) ---
    EncounterRow {
        encounter_id: 2728,
        name: "Gnarlroot",
        description: "A corrupted treant guardian of Amirdrassil.",
        root_section_id: 16026,
        link_section: "",
        journal_instance_id: 1207,
        dungeon_encounter_id: 2728,
        instance_id: 2549,
    },
    EncounterRow {
        encounter_id: 2731,
        name: "Tindral Sageswift, Seer of the Flame",
        description: "A night elf druid who bargained with the Emerald Dream's corruption.",
        root_section_id: 16029,
        link_section: "",
        journal_instance_id: 1207,
        dungeon_encounter_id: 2731,
        instance_id: 2549,
    },
    EncounterRow {
        encounter_id: 2737,
        name: "Fyrakk the Blazing",
        description: "Incarnate of fire, final boss of Amirdrassil.",
        root_section_id: 16035,
        link_section: "",
        journal_instance_id: 1207,
        dungeon_encounter_id: 2737,
        instance_id: 2549,
    },
    // --- Nerub-ar Palace (instance 2657, journalInstanceID 1273) ---
    EncounterRow {
        encounter_id: 2902,
        name: "Ulgrax the Devourer",
        description: "A massive nerubian beast lurking beneath the palace.",
        root_section_id: 17392,
        link_section: "",
        journal_instance_id: 1273,
        dungeon_encounter_id: 2902,
        instance_id: 2657,
    },
    EncounterRow {
        encounter_id: 2917,
        name: "The Bloodbound Horror",
        description: "A monstrosity bound from the blood of fallen heroes.",
        root_section_id: 17407,
        link_section: "",
        journal_instance_id: 1273,
        dungeon_encounter_id: 2917,
        instance_id: 2657,
    },
    EncounterRow {
        encounter_id: 2922,
        name: "Queen Ansurek",
        description: "Queen of the nerubians and final boss of Nerub-ar Palace.",
        root_section_id: 17412,
        link_section: "",
        journal_instance_id: 1273,
        dungeon_encounter_id: 2922,
        instance_id: 2657,
    },
];

static INSTANCES: &[InstanceRow] = &[
    InstanceRow {
        instance_id: 1200,
        name: "Vault of the Incarnates",
        description: "The Primalists have broken through to Thaldraszus.",
        bg_image: "Interface\\EncounterJournal\\UI-EJ-BG-VaultOfTheIncarnates",
        button_image1: "",
        lore_image: "",
        button_image2: "",
        dungeon_area_map_id: 0,
        link_raid_id: 1200,
        link_dungeon_id: 0,
    },
    InstanceRow {
        instance_id: 1208,
        name: "Aberrus, the Shadowed Crucible",
        description: "Neltharion's secret laboratory hidden beneath Zaralek Cavern.",
        bg_image: "Interface\\EncounterJournal\\UI-EJ-BG-Aberrus",
        button_image1: "",
        lore_image: "",
        button_image2: "",
        dungeon_area_map_id: 0,
        link_raid_id: 1208,
        link_dungeon_id: 0,
    },
    InstanceRow {
        instance_id: 2549,
        name: "Amirdrassil, the Dream's Hope",
        description: "The new world tree grows in the Emerald Dream.",
        bg_image: "Interface\\EncounterJournal\\UI-EJ-BG-Amirdrassil",
        button_image1: "",
        lore_image: "",
        button_image2: "",
        dungeon_area_map_id: 0,
        link_raid_id: 2549,
        link_dungeon_id: 0,
    },
    InstanceRow {
        instance_id: 2657,
        name: "Nerub-ar Palace",
        description: "Queen Ansurek's ancient nerubian stronghold beneath Azj-Kahet.",
        bg_image: "Interface\\EncounterJournal\\UI-EJ-BG-NerubarPalace",
        button_image1: "",
        lore_image: "",
        button_image2: "",
        dungeon_area_map_id: 0,
        link_raid_id: 2657,
        link_dungeon_id: 0,
    },
    InstanceRow {
        instance_id: 2522,
        name: "Vault of the Incarnates",
        description: "The Primalists have broken through to Thaldraszus.",
        bg_image: "Interface\\EncounterJournal\\UI-EJ-BG-VaultOfTheIncarnates",
        button_image1: "",
        lore_image: "",
        button_image2: "",
        dungeon_area_map_id: 0,
        link_raid_id: 2522,
        link_dungeon_id: 0,
    },
    InstanceRow {
        instance_id: 2569,
        name: "Aberrus, the Shadowed Crucible",
        description: "Neltharion's secret laboratory hidden beneath Zaralek Cavern.",
        bg_image: "Interface\\EncounterJournal\\UI-EJ-BG-Aberrus",
        button_image1: "",
        lore_image: "",
        button_image2: "",
        dungeon_area_map_id: 0,
        link_raid_id: 2569,
        link_dungeon_id: 0,
    },
];

// ---------------------------------------------------------------------------
// Registration
// ---------------------------------------------------------------------------

pub(super) fn register_encounter_journal_surface(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_EncounterJournal")?;
    table_set_rust_fn(state, table_ref, "GetEncounterInfo", get_encounter_info)?;
    table_set_rust_fn(state, table_ref, "GetInstanceInfo", get_instance_info)?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

fn get_encounter_info(state: &mut LuaState) -> LuaResult<u32> {
    let encounter_id = u32::from_stack(state, 1)?;
    let Some(row) = ENCOUNTERS.iter().find(|r| r.encounter_id == encounter_id) else {
        return Ok(0);
    };
    let name = create_string(state, row.name);
    let description = create_string(state, row.description);
    let link_section = create_string(state, row.link_section);
    state.push(name);
    state.push(description);
    state.push(Val::Num(row.encounter_id as f64));
    state.push(Val::Num(row.root_section_id as f64));
    state.push(link_section);
    state.push(Val::Num(row.journal_instance_id as f64));
    state.push(Val::Num(row.dungeon_encounter_id as f64));
    state.push(Val::Num(row.instance_id as f64));
    Ok(8)
}

fn get_instance_info(state: &mut LuaState) -> LuaResult<u32> {
    let instance_id = u32::from_stack(state, 1)?;
    let Some(row) = INSTANCES.iter().find(|r| r.instance_id == instance_id) else {
        return Ok(0);
    };
    let name = create_string(state, row.name);
    let description = create_string(state, row.description);
    let bg_image = create_string(state, row.bg_image);
    let button_image1 = create_string(state, row.button_image1);
    let lore_image = create_string(state, row.lore_image);
    let button_image2 = create_string(state, row.button_image2);
    state.push(name);
    state.push(description);
    state.push(bg_image);
    state.push(button_image1);
    state.push(lore_image);
    state.push(button_image2);
    state.push(Val::Num(row.dungeon_area_map_id as f64));
    state.push(Val::Num(row.link_raid_id as f64));
    state.push(Val::Num(row.link_dungeon_id as f64));
    Ok(9)
}
