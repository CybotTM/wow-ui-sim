//! `C_EncounterJournal` namespace and `EJ_*` legacy globals — the data
//! surface behind the Adventure Guide. Backed by static tables in
//! `data/encounter_journal.rs` (regenerated via `wow-cli generate
//! encounter-journal`) and per-session state in `SimState.encounter_journal`.
//!
//! Schema parity:
//!
//! - `C_EncounterJournal.GetEncounterInfo` — 8-tuple matching the official
//!   retail return: `(name, description, encounterID, rootSectionID,
//!   linkSection, journalInstanceID, dungeonEncounterID, instanceID)`.
//! - `C_EncounterJournal.GetInstanceInfo` — 9-tuple: `(name, description,
//!   bgImage, buttonImage1, loreImage, buttonImage2, dungeonAreaMapID,
//!   linkRaidID, linkDungeonID)`. Image fields are returned as fileDataIDs
//!   (numbers) per the real DBC schema.
//! - `C_EncounterJournal.GetSectionInfo` — table per
//!   `vendor/wow-ui-source/.../EncounterJournalDocumentation` with at least
//!   `spellID`, `headerType`, `description`, `title`, `siblingSectionID`,
//!   `firstChildSectionID`.
//! - `C_EncounterJournal.GetLootInfo` / `GetLootInfoByIndex` — table with
//!   `name`, `icon`, `itemQuality`, `itemID`, `link`, `slot`, `armorType`,
//!   `encounterID`, `displayAsPerPlayerLoot` (always false here).
//!
//! `EJ_*` legacy globals route through the same data, with state held
//! in `SimState.encounter_journal` rather than Lua globals.

use super::ensure_namespace;
use crate::encounter_journal_data as data;
use crate::items;
use crate::lua_api::globals::strings::string_data::game_enums::ITEM_QUALITY_COLORS_DATA;
use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, create_string, create_table, table_set, table_set_num,
};
use crate::lua_bridge::{FromStack, stack_val, table_set_rust_fn_static};
use crate::spells;
use rilua::vm::closure::RustFn;
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaApiMut, LuaResult, Val};

const ENCOUNTER_JOURNAL_ICON_FLAG_COUNT: u32 = 14;
const FALLBACK_ABILITY_ICON: u32 = 136243;
const ADVENTURE_VISIBLE_SUGGESTIONS: usize = 3;
const ADVENTURE_SUGGESTION_CLEAR_LIMIT: usize = 16;

struct AdventureSuggestion {
    title: &'static str,
    description: &'static str,
    button_text: &'static str,
    icon_path: &'static str,
}

const ADVENTURE_SUGGESTIONS: &[AdventureSuggestion] = &[
    AdventureSuggestion {
        title: "Dungeons",
        description: "Queue for current dungeons.",
        button_text: "View",
        icon_path: "Interface\\Icons\\Achievement_Dungeon_UlduarRaid_Titan_01",
    },
    AdventureSuggestion {
        title: "Nerub-ar Palace",
        description: "Study bosses and loot.",
        button_text: "Open",
        icon_path: "Interface\\Icons\\INV_Nerubian_Ring_01_Color5",
    },
    AdventureSuggestion {
        title: "Delves",
        description: "Explore short adventures.",
        button_text: "Start",
        icon_path: "Interface\\Icons\\INV_Misc_ScrollUnrolled03",
    },
    AdventureSuggestion {
        title: "Legacy Raids",
        description: "Browse raid appearances.",
        button_text: "Browse",
        icon_path: "Interface\\Icons\\Achievement_Raid_DragonSoul",
    },
    AdventureSuggestion {
        title: "Collections",
        description: "Track mounts and outfits.",
        button_text: "Open",
        icon_path: "Interface\\Icons\\Achievement_General_StayClassy",
    },
];

const ADVENTURE_JOURNAL_FUNCTIONS: &[(&str, RustFn)] = &[
    ("CanBeShown", adventure_can_be_shown),
    ("UpdateSuggestions", adventure_update_suggestions),
    ("GetPrimaryOffset", adventure_get_primary_offset),
    ("SetPrimaryOffset", adventure_set_primary_offset),
    (
        "GetNumAvailableSuggestions",
        adventure_get_num_available_suggestions,
    ),
    ("GetSuggestions", adventure_get_suggestions),
    ("GetReward", adventure_get_reward),
    ("ActivateEntry", adventure_activate_entry),
];

const ENCOUNTER_JOURNAL_FUNCTIONS: &[(&str, RustFn)] = &[
    ("GetEncounterInfo", get_encounter_info),
    ("GetInstanceInfo", get_instance_info),
    ("GetSectionInfo", get_section_info),
    ("GetLootInfo", get_loot_info),
    ("GetLootInfoByIndex", get_loot_info_by_index),
    ("GetSectionIconFlags", get_section_icon_flags),
    ("InstanceHasLoot", instance_has_loot),
    ("GetSlotFilter", get_slot_filter),
    ("SetSlotFilter", set_slot_filter),
    ("ResetSlotFilter", reset_slot_filter),
    ("SetTab", set_tab),
    ("OnOpen", noop),
    ("OnClose", noop),
    ("InitalizeSelectedTier", initialize_selected_tier),
    ("StartArathiRPE", noop),
];

const EJ_GLOBAL_FUNCTIONS: &[(&str, RustFn)] = &[
    ("EJ_GetNumTiers", ej_get_num_tiers),
    ("EJ_GetTierInfo", ej_get_tier_info),
    ("EJ_GetCurrentTier", ej_get_current_tier),
    ("EJ_SelectTier", ej_select_tier),
    ("EJ_GetInstanceInfo", ej_get_instance_info),
    ("EJ_GetInstanceByIndex", ej_get_instance_by_index),
    ("EJ_GetEncounterInfo", ej_get_encounter_info),
    ("EJ_GetEncounterInfoByIndex", ej_get_encounter_info_by_index),
    ("EJ_GetCreatureInfo", ej_get_creature_info),
    ("EJ_SelectInstance", ej_select_instance),
    ("EJ_SelectEncounter", ej_select_encounter),
    ("EJ_GetSelectedInstance", ej_get_selected_instance),
    ("EJ_GetSelectedEncounter", ej_get_selected_encounter),
    ("EJ_GetDifficulty", ej_get_difficulty),
    ("EJ_SetDifficulty", ej_set_difficulty),
    ("EJ_InstanceIsRaid", ej_instance_is_raid),
    (
        "EJ_IsValidInstanceDifficulty",
        ej_is_valid_instance_difficulty,
    ),
    ("EJ_GetNumLoot", ej_get_num_loot),
    ("EJ_GetLootInfoByIndex", ej_get_loot_info_by_index_global),
    ("EJ_GetLootFilter", ej_get_loot_filter),
    ("EJ_SetLootFilter", ej_set_loot_filter),
    ("EJ_ResetLootFilter", ej_reset_loot_filter),
    ("EJ_GetInvTypeSortOrder", ej_get_inv_type_sort_order),
    (
        "EJ_GetNumEncountersForLootByIndex",
        ej_get_num_encounters_for_loot_by_index,
    ),
    ("EJ_IsLootListOutOfDate", ej_is_loot_list_out_of_date),
    ("EJ_GetSectionPath", ej_get_section_path),
    ("EJ_GetContentTuningID", ej_get_content_tuning_id),
    ("EJ_SetSearch", ej_set_search),
    ("EJ_ClearSearch", ej_clear_search),
    ("EJ_EndSearch", ej_end_search),
    ("EJ_GetSearchSize", ej_get_search_size),
    ("EJ_GetSearchProgress", ej_get_search_progress),
    ("EJ_GetNumSearchResults", ej_get_num_search_results),
    ("EJ_GetSearchResult", ej_get_search_result),
    ("EJ_IsSearchFinished", ej_is_search_finished),
    ("EJ_HandleLinkPath", ej_handle_link_path),
    ("EJ_HideLootJournalPanel", noop_global),
    ("EJ_HideNonInstancePanels", noop_global),
    ("EJ_HideSuggestPanel", noop_global),
    ("EJ_HideTutorialsPanel", noop_global),
];

// ---------------------------------------------------------------------------
// Registration
// ---------------------------------------------------------------------------

pub(super) fn register_encounter_journal_surface(state: &mut LuaState) -> LuaResult<()> {
    register_encounter_journal_functions(state)?;
    register_adventure_journal_surface(state)?;
    Ok(())
}

fn register_encounter_journal_functions(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_EncounterJournal")?;
    for &(name, handler) in ENCOUNTER_JOURNAL_FUNCTIONS {
        table_set_rust_fn_static(state, table_ref, name, handler)?;
    }
    Ok(())
}

fn register_adventure_journal_surface(state: &mut LuaState) -> LuaResult<()> {
    let adventure_table_ref = ensure_namespace(state, "C_AdventureJournal")?;
    for &(name, handler) in ADVENTURE_JOURNAL_FUNCTIONS {
        table_set_rust_fn_static(state, adventure_table_ref, name, handler)?;
    }
    Ok(())
}

pub(crate) fn register_ej_globals(lua: &mut rilua::Lua) -> LuaResult<()> {
    for &(name, handler) in EJ_GLOBAL_FUNCTIONS {
        LuaApiMut::register_function(lua, name, handler)?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// C_EncounterJournal handlers
// ---------------------------------------------------------------------------

fn get_encounter_info(state: &mut LuaState) -> LuaResult<u32> {
    let encounter_id = u32::from_stack(state, 1)?;
    let Some(row) = data::encounter_by_id(encounter_id) else {
        return Ok(0);
    };
    push_encounter_tuple(state, row);
    Ok(8)
}

fn get_instance_info(state: &mut LuaState) -> LuaResult<u32> {
    let instance_id = u32::from_stack(state, 1)?;
    let Some(row) = data::instance_by_id(instance_id) else {
        return Ok(0);
    };
    push_instance_tuple(state, row);
    Ok(9)
}

fn get_section_info(state: &mut LuaState) -> LuaResult<u32> {
    let section_id = u32::from_stack(state, 1)?;
    let Some(row) = data::section_by_id(section_id) else {
        return Ok(0);
    };
    let table = build_section_info_table(state, row);
    state.push(table);
    Ok(1)
}

fn build_section_info_table(state: &mut LuaState, row: &data::Section) -> Val {
    let table = create_table(state);
    set_section_text_fields(state, table, row);
    set_section_relationship_fields(state, table, row);
    set_section_visual_fields(state, table, row);
    table_set(state, table, "filteredByDifficulty", Val::Bool(false));
    table_set(state, table, "startsOpen", Val::Bool(false));
    table
}

fn set_section_text_fields(state: &mut LuaState, table: Val, row: &data::Section) {
    let title = create_string(state, row.title);
    let body = create_string(state, row.body);
    let empty_link = create_string(state, "");

    table_set(state, table, "spellID", Val::Num(row.spell_id as f64));
    table_set(state, table, "headerType", Val::Num(row.kind as f64));
    table_set(state, table, "title", title);
    table_set(state, table, "description", body);
    table_set(state, table, "link", empty_link);
}

fn set_section_relationship_fields(state: &mut LuaState, table: Val, row: &data::Section) {
    table_set(
        state,
        table,
        "siblingSectionID",
        Val::Num(row.next_sibling_id as f64),
    );
    table_set(
        state,
        table,
        "firstChildSectionID",
        Val::Num(row.first_child_id as f64),
    );
    table_set(
        state,
        table,
        "parentSectionID",
        Val::Num(row.parent_id as f64),
    );
}

fn set_section_visual_fields(state: &mut LuaState, table: Val, row: &data::Section) {
    table_set(
        state,
        table,
        "creatureDisplayID",
        Val::Num(row.icon_creature_display_id as f64),
    );
    table_set(
        state,
        table,
        "uiModelSceneID",
        Val::Num(row.model_scene_id as f64),
    );
    if let Some(ability_icon) = ability_icon_for_section(row) {
        table_set(state, table, "abilityIcon", Val::Num(ability_icon as f64));
    }
}

fn ability_icon_for_section(row: &data::Section) -> Option<u32> {
    if row.icon_file_id != 0 {
        return Some(row.icon_file_id);
    }

    if row.spell_id == 0 {
        return None;
    }

    Some(
        spells::get_spell(row.spell_id)
            .map(|spell| spell.icon_file_data_id)
            .unwrap_or(FALLBACK_ABILITY_ICON),
    )
}

fn get_loot_info(state: &mut LuaState) -> LuaResult<u32> {
    let item_id = u32::from_stack(state, 1)?;
    let table = build_loot_table(state, item_id, 0);
    state.push(table);
    Ok(1)
}

fn get_loot_info_by_index(state: &mut LuaState) -> LuaResult<u32> {
    let index = u32::from_stack(state, 1)? as usize;
    let (current_encounter, current_instance, difficulty) = {
        let sim = borrow_state(state)?;
        (
            sim.encounter_journal.current_encounter,
            sim.encounter_journal.current_instance,
            sim.encounter_journal.difficulty,
        )
    };
    let loot = filtered_loot(current_encounter, current_instance, difficulty);
    if index == 0 || index > loot.len() {
        return Ok(0);
    }
    let row = loot[index - 1];
    let item_id = row.item_id;
    let encounter_id = row.encounter_id;
    let table = build_loot_table(state, item_id, encounter_id);
    state.push(table);
    Ok(1)
}

fn get_section_icon_flags(state: &mut LuaState) -> LuaResult<u32> {
    let section_id = u32::from_stack(state, 1)?;
    let Some(section) = data::section_by_id(section_id) else {
        return Ok(0);
    };
    let Some(icon_indices) = section_icon_indices_table(state, section.icon_flags) else {
        return Ok(0);
    };
    state.push(icon_indices);
    Ok(1)
}

fn section_icon_indices_table(state: &mut LuaState, icon_flags: u32) -> Option<Val> {
    if icon_flags == 0 {
        return None;
    }

    let table = create_table(state);
    let Val::Table(table_ref) = table else {
        return None;
    };

    let mut lua_index = 1.0;
    for icon_index in 0..ENCOUNTER_JOURNAL_ICON_FLAG_COUNT {
        let flag = 1_u32 << icon_index;
        if icon_flags & flag != 0 {
            table_set_num(state, table_ref, lua_index, Val::Num(icon_index as f64));
            lua_index += 1.0;
        }
    }

    Some(table)
}

fn instance_has_loot(state: &mut LuaState) -> LuaResult<u32> {
    let instance_id = borrow_state(state)?.encounter_journal.current_instance;
    let has_loot = data::encounters_for_instance(instance_id)
        .iter()
        .any(|e| !data::loot_for_encounter(e.id).is_empty());
    state.push(Val::Bool(has_loot));
    Ok(1)
}

fn get_slot_filter(state: &mut LuaState) -> LuaResult<u32> {
    let slot = borrow_state(state)?.encounter_journal.slot_filter;
    state.push(Val::Num(slot as f64));
    Ok(1)
}

fn set_slot_filter(state: &mut LuaState) -> LuaResult<u32> {
    let slot = i32::from_stack(state, 1).unwrap_or(-1);
    borrow_state_mut(state)?.encounter_journal.slot_filter = slot;
    Ok(0)
}

fn reset_slot_filter(state: &mut LuaState) -> LuaResult<u32> {
    borrow_state_mut(state)?.encounter_journal.slot_filter = -1;
    Ok(0)
}

fn adventure_can_be_shown(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    Ok(1)
}

fn adventure_update_suggestions(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn adventure_get_primary_offset(state: &mut LuaState) -> LuaResult<u32> {
    let offset = borrow_state(state)?
        .encounter_journal
        .adventure_primary_offset;
    state.push(Val::Num(offset as f64));
    Ok(1)
}

fn adventure_set_primary_offset(state: &mut LuaState) -> LuaResult<u32> {
    let offset = u32::from_stack(state, 1).unwrap_or(0);
    let max_offset = adventure_max_primary_offset();
    borrow_state_mut(state)?
        .encounter_journal
        .adventure_primary_offset = offset.min(max_offset);
    Ok(0)
}

fn adventure_get_num_available_suggestions(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(ADVENTURE_SUGGESTIONS.len() as f64));
    Ok(1)
}

fn adventure_get_suggestions(state: &mut LuaState) -> LuaResult<u32> {
    let Val::Table(output_ref) = stack_val(state, 1) else {
        return Ok(0);
    };
    clear_adventure_suggestions(state, output_ref);

    let offset = borrow_state(state)?
        .encounter_journal
        .adventure_primary_offset
        .min(adventure_max_primary_offset()) as usize;

    for visible_index in 0..ADVENTURE_VISIBLE_SUGGESTIONS {
        let suggestion_index = (offset + visible_index) % ADVENTURE_SUGGESTIONS.len();
        let suggestion = build_adventure_suggestion_table(state, suggestion_index);
        table_set_num(state, output_ref, (visible_index + 1) as f64, suggestion);
    }

    Ok(0)
}

fn adventure_get_reward(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    Ok(1)
}

fn adventure_activate_entry(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn adventure_max_primary_offset() -> u32 {
    ADVENTURE_SUGGESTIONS.len().saturating_sub(1) as u32
}

fn clear_adventure_suggestions(state: &mut LuaState, output_ref: GcRef<Table>) {
    for index in 1..=ADVENTURE_SUGGESTION_CLEAR_LIMIT {
        table_set_num(state, output_ref, index as f64, Val::Nil);
    }
}

fn build_adventure_suggestion_table(state: &mut LuaState, suggestion_index: usize) -> Val {
    let suggestion = &ADVENTURE_SUGGESTIONS[suggestion_index];
    let table = create_table(state);
    let title = create_string(state, suggestion.title);
    let description = create_string(state, suggestion.description);
    let button_text = create_string(state, suggestion.button_text);
    let icon_path = create_string(state, suggestion.icon_path);

    table_set(
        state,
        table,
        "index",
        Val::Num((suggestion_index + 1) as f64),
    );
    table_set(state, table, "title", title);
    table_set(state, table, "description", description);
    table_set(state, table, "buttonText", button_text);
    table_set(state, table, "iconPath", icon_path);
    table
}

fn set_tab(state: &mut LuaState) -> LuaResult<u32> {
    let tab = u32::from_stack(state, 1).unwrap_or(0);
    borrow_state_mut(state)?.encounter_journal.current_tab = tab;
    Ok(0)
}

fn initialize_selected_tier(state: &mut LuaState) -> LuaResult<u32> {
    let mut sim = borrow_state_mut(state)?;
    // InitalizeSelectedTier resets to the Dragonflight tier -- the first expansion
    // with Journeys content (EJ_JOURNEYS_MIN_TIER in the Lua layer).  Tier 10 is
    // Dragonflight (expansion 1000).  If absent, fall back to the smallest tier.
    let df_order = data::TIERS
        .iter()
        .find(|t| t.expansion == 1000)
        .map(|t| t.order)
        .unwrap_or_else(|| data::TIERS.iter().map(|t| t.order).min().unwrap_or(1));
    sim.encounter_journal.current_tier = df_order;
    Ok(0)
}

fn noop(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn noop_global(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

// ---------------------------------------------------------------------------
// EJ_* handlers
// ---------------------------------------------------------------------------

fn ej_get_num_tiers(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(data::TIERS.len() as f64));
    Ok(1)
}

fn ej_get_tier_info(state: &mut LuaState) -> LuaResult<u32> {
    let tier_order = u32::from_stack(state, 1).unwrap_or(0);
    let Some(tier) = data::tier_by_order(tier_order) else {
        let empty = create_string(state, "");
        state.push(empty);
        return Ok(1);
    };
    let name = create_string(state, tier.name);
    state.push(name);
    Ok(1)
}

fn ej_get_current_tier(state: &mut LuaState) -> LuaResult<u32> {
    let tier = borrow_state(state)?.encounter_journal.current_tier;
    state.push(Val::Num(tier as f64));
    Ok(1)
}

fn ej_select_tier(state: &mut LuaState) -> LuaResult<u32> {
    let tier = u32::from_stack(state, 1).unwrap_or(0);
    if data::tier_by_order(tier).is_some() {
        borrow_state_mut(state)?.encounter_journal.current_tier = tier;
    }
    Ok(0)
}

fn ej_get_instance_info(state: &mut LuaState) -> LuaResult<u32> {
    let arg_present = !matches!(stack_val(state, 1), Val::Nil);
    let instance_id = if arg_present {
        u32::from_stack(state, 1).unwrap_or(0)
    } else {
        borrow_state(state)?.encounter_journal.current_instance
    };
    let Some(row) = data::instance_by_id(instance_id) else {
        return Ok(0);
    };
    push_instance_legacy_tuple(state, row);
    Ok(12)
}

fn ej_get_instance_by_index(state: &mut LuaState) -> LuaResult<u32> {
    let index = u32::from_stack(state, 1).unwrap_or(0) as usize;
    let is_raid = bool::from_stack(state, 2).unwrap_or(true);
    let tier_order = borrow_state(state)?.encounter_journal.current_tier;
    let instances = data::instances_for_tier(tier_order, is_raid);
    if index == 0 || index > instances.len() {
        return Ok(0);
    }
    let inst = instances[index - 1];
    state.push(Val::Num(inst.id as f64));
    push_instance_legacy_tuple(state, inst);
    Ok(13)
}

fn ej_get_encounter_info(state: &mut LuaState) -> LuaResult<u32> {
    let encounter_id = u32::from_stack(state, 1)?;
    let Some(row) = data::encounter_by_id(encounter_id) else {
        return Ok(0);
    };
    push_encounter_tuple(state, row);
    Ok(8)
}

fn ej_get_encounter_info_by_index(state: &mut LuaState) -> LuaResult<u32> {
    let index = u32::from_stack(state, 1).unwrap_or(0) as usize;
    let arg2_present = !matches!(stack_val(state, 2), Val::Nil);
    let instance_id = if arg2_present {
        u32::from_stack(state, 2).unwrap_or(0)
    } else {
        borrow_state(state)?.encounter_journal.current_instance
    };
    let encounters = data::encounters_for_instance(instance_id);
    if index == 0 || index > encounters.len() {
        return Ok(0);
    }
    push_encounter_tuple(state, encounters[index - 1]);
    Ok(8)
}

fn ej_get_creature_info(state: &mut LuaState) -> LuaResult<u32> {
    let index = u32::from_stack(state, 1).unwrap_or(0) as usize;
    let arg2_present = !matches!(stack_val(state, 2), Val::Nil);
    let encounter_id = if arg2_present {
        u32::from_stack(state, 2).unwrap_or(0)
    } else {
        borrow_state(state)?.encounter_journal.current_encounter
    };
    let creatures = data::creatures_for_encounter(encounter_id);
    if index == 0 || index > creatures.len() {
        return Ok(0);
    }
    let c = creatures[index - 1];
    let name = create_string(state, c.name);
    let description = create_string(state, c.description);
    state.push(Val::Num(c.id as f64));
    state.push(name);
    state.push(description);
    state.push(Val::Num(c.display_id as f64));
    state.push(optional_file_data_id(c.icon_file_id));
    state.push(Val::Num(c.model_scene_id as f64));
    Ok(6)
}

fn optional_file_data_id(file_data_id: u32) -> Val {
    if file_data_id == 0 {
        Val::Nil
    } else {
        Val::Num(file_data_id as f64)
    }
}

fn ej_select_instance(state: &mut LuaState) -> LuaResult<u32> {
    let instance_id = u32::from_stack(state, 1).unwrap_or(0);
    let mut sim = borrow_state_mut(state)?;
    sim.encounter_journal.current_instance = instance_id;
    sim.encounter_journal.current_encounter = 0;
    if let Some(inst) = data::instance_by_id(instance_id) {
        sim.encounter_journal.is_raid = inst.is_raid;
    }
    Ok(0)
}

fn ej_select_encounter(state: &mut LuaState) -> LuaResult<u32> {
    let encounter_id = u32::from_stack(state, 1).unwrap_or(0);
    borrow_state_mut(state)?.encounter_journal.current_encounter = encounter_id;
    Ok(0)
}

fn ej_get_selected_instance(state: &mut LuaState) -> LuaResult<u32> {
    let id = borrow_state(state)?.encounter_journal.current_instance;
    if id == 0 {
        return Ok(0);
    }
    state.push(Val::Num(id as f64));
    Ok(1)
}

fn ej_get_selected_encounter(state: &mut LuaState) -> LuaResult<u32> {
    let id = borrow_state(state)?.encounter_journal.current_encounter;
    if id == 0 {
        return Ok(0);
    }
    state.push(Val::Num(id as f64));
    Ok(1)
}

fn ej_get_difficulty(state: &mut LuaState) -> LuaResult<u32> {
    let difficulty = borrow_state(state)?.encounter_journal.difficulty;
    state.push(Val::Num(difficulty as f64));
    Ok(1)
}

fn ej_set_difficulty(state: &mut LuaState) -> LuaResult<u32> {
    let difficulty = u32::from_stack(state, 1).unwrap_or(0);
    borrow_state_mut(state)?.encounter_journal.difficulty = difficulty;
    Ok(0)
}

fn ej_instance_is_raid(state: &mut LuaState) -> LuaResult<u32> {
    let is_raid = {
        let sim = borrow_state(state)?;
        let id = sim.encounter_journal.current_instance;
        data::instance_by_id(id)
            .map(|i| i.is_raid)
            .unwrap_or(sim.encounter_journal.is_raid)
    };
    state.push(Val::Bool(is_raid));
    Ok(1)
}

fn ej_is_valid_instance_difficulty(state: &mut LuaState) -> LuaResult<u32> {
    let difficulty = u32::from_stack(state, 1).unwrap_or(0);
    state.push(Val::Bool(difficulty > 0));
    Ok(1)
}

fn ej_get_num_loot(state: &mut LuaState) -> LuaResult<u32> {
    let (encounter, instance, difficulty) = {
        let sim = borrow_state(state)?;
        (
            sim.encounter_journal.current_encounter,
            sim.encounter_journal.current_instance,
            sim.encounter_journal.difficulty,
        )
    };
    let count = filtered_loot(encounter, instance, difficulty).len();
    state.push(Val::Num(count as f64));
    Ok(1)
}

fn ej_get_loot_info_by_index_global(state: &mut LuaState) -> LuaResult<u32> {
    get_loot_info_by_index(state)
}

fn ej_get_loot_filter(state: &mut LuaState) -> LuaResult<u32> {
    let (class_filter, spec_filter) = {
        let sim = borrow_state(state)?;
        (
            sim.encounter_journal.class_filter,
            sim.encounter_journal.spec_filter,
        )
    };
    state.push(Val::Num(class_filter as f64));
    state.push(Val::Num(spec_filter as f64));
    Ok(2)
}

fn ej_set_loot_filter(state: &mut LuaState) -> LuaResult<u32> {
    let class_filter = u32::from_stack(state, 1).unwrap_or(0);
    let spec_filter = u32::from_stack(state, 2).unwrap_or(0);
    let mut sim = borrow_state_mut(state)?;
    sim.encounter_journal.class_filter = class_filter;
    sim.encounter_journal.spec_filter = spec_filter;
    Ok(0)
}

fn ej_reset_loot_filter(state: &mut LuaState) -> LuaResult<u32> {
    let mut sim = borrow_state_mut(state)?;
    sim.encounter_journal.class_filter = 0;
    sim.encounter_journal.spec_filter = 0;
    Ok(0)
}

fn ej_get_inv_type_sort_order(state: &mut LuaState) -> LuaResult<u32> {
    let inv_type = u32::from_stack(state, 1).unwrap_or(0);
    let order = match inv_type {
        1 => 1,                       // INVTYPE_HEAD
        3 => 2,                       // INVTYPE_SHOULDER
        16 => 3,                      // INVTYPE_CLOAK
        5 | 20 => 4,                  // INVTYPE_CHEST / INVTYPE_ROBE
        4 => 5,                       // INVTYPE_BODY (shirt)
        19 => 6,                      // INVTYPE_TABARD
        9 => 7,                       // INVTYPE_WRIST
        10 => 8,                      // INVTYPE_HAND
        6 => 9,                       // INVTYPE_WAIST
        7 => 10,                      // INVTYPE_LEGS
        8 => 11,                      // INVTYPE_FEET
        2 => 12,                      // INVTYPE_NECK
        11 => 13,                     // INVTYPE_FINGER
        12 => 14,                     // INVTYPE_TRINKET
        13 | 17 | 21 | 22 | 26 => 15, // weapons
        14 => 16,                     // INVTYPE_SHIELD
        15 | 23 | 25 | 28 => 17,      // ranged/holdable/relic
        _ => 99,
    };
    state.push(Val::Num(order as f64));
    Ok(1)
}

fn ej_get_num_encounters_for_loot_by_index(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(1.0));
    Ok(1)
}

fn ej_is_loot_list_out_of_date(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn ej_get_section_path(state: &mut LuaState) -> LuaResult<u32> {
    let section_id = u32::from_stack(state, 1).unwrap_or(0);
    let mut path: Vec<&'static str> = Vec::new();
    let mut cursor = section_id;
    while cursor != 0 {
        let Some(row) = data::section_by_id(cursor) else {
            break;
        };
        path.push(row.title);
        cursor = row.parent_id;
        if path.len() >= 16 {
            break;
        }
    }
    path.reverse();
    let joined = path.join(" > ");
    let s = create_string(state, &joined);
    state.push(s);
    Ok(1)
}

fn ej_get_content_tuning_id(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn ej_set_search(state: &mut LuaState) -> LuaResult<u32> {
    let text = String::from_stack(state, 1).unwrap_or_default();
    let results = compute_search_results(&text);
    let mut sim = borrow_state_mut(state)?;
    sim.encounter_journal.search_text = text;
    sim.encounter_journal.search_finished = true;
    sim.encounter_journal.search_results = results;
    Ok(0)
}

fn ej_clear_search(state: &mut LuaState) -> LuaResult<u32> {
    let mut sim = borrow_state_mut(state)?;
    sim.encounter_journal.search_text.clear();
    sim.encounter_journal.search_results.clear();
    sim.encounter_journal.search_finished = true;
    Ok(0)
}

fn ej_end_search(state: &mut LuaState) -> LuaResult<u32> {
    borrow_state_mut(state)?.encounter_journal.search_finished = true;
    Ok(0)
}

fn ej_get_search_size(state: &mut LuaState) -> LuaResult<u32> {
    let n = borrow_state(state)?.encounter_journal.search_results.len();
    state.push(Val::Num(n as f64));
    Ok(1)
}

fn ej_get_search_progress(state: &mut LuaState) -> LuaResult<u32> {
    let n = borrow_state(state)?.encounter_journal.search_results.len();
    state.push(Val::Num(n as f64));
    Ok(1)
}

fn ej_get_num_search_results(state: &mut LuaState) -> LuaResult<u32> {
    let n = borrow_state(state)?.encounter_journal.search_results.len();
    state.push(Val::Num(n as f64));
    Ok(1)
}

fn ej_get_search_result(state: &mut LuaState) -> LuaResult<u32> {
    let index = u32::from_stack(state, 1).unwrap_or(0) as usize;
    let result = {
        let sim = borrow_state(state)?;
        if index == 0 || index > sim.encounter_journal.search_results.len() {
            return Ok(0);
        }
        sim.encounter_journal.search_results[index - 1].clone()
    };
    let link = create_string(state, &result.item_link);
    state.push(Val::Num(result.id as f64));
    state.push(Val::Num(result.kind as f64));
    state.push(Val::Num(result.difficulty_id as f64));
    state.push(Val::Num(result.instance_id as f64));
    state.push(Val::Num(result.encounter_id as f64));
    state.push(link);
    state.push(Val::Num(result.icon as f64));
    Ok(7)
}

fn ej_is_search_finished(state: &mut LuaState) -> LuaResult<u32> {
    let finished = borrow_state(state)?.encounter_journal.search_finished;
    state.push(Val::Bool(finished));
    Ok(1)
}

fn ej_handle_link_path(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn push_encounter_tuple(state: &mut LuaState, row: &data::Encounter) {
    let name = create_string(state, row.name);
    let description = create_string(state, row.description);
    let link = create_string(state, "");
    state.push(name);
    state.push(description);
    state.push(Val::Num(row.id as f64));
    state.push(Val::Num(row.first_section_id as f64));
    state.push(link);
    state.push(Val::Num(row.instance_id as f64));
    state.push(Val::Num(row.dungeon_encounter_id as f64));
    state.push(Val::Num(row.ui_map_id as f64));
}

fn push_instance_tuple(state: &mut LuaState, row: &data::Instance) {
    push_instance_base_tuple(state, row);
}

fn push_instance_legacy_tuple(state: &mut LuaState, row: &data::Instance) {
    let name = create_string(state, row.name);
    let description = create_string(state, row.description);
    state.push(name);
    state.push(description);
    state.push(Val::Num(row.bg_file_id as f64));
    state.push(Val::Num(row.button_file_id as f64));
    state.push(Val::Num(row.lore_file_id as f64));
    state.push(Val::Num(row.button_small_file_id as f64));
    state.push(Val::Num(row.area_id as f64));
    state.push(Val::Num(if row.is_raid { row.id as f64 } else { 0.0 }));
    state.push(Val::Bool(row.is_raid));
    state.push(Val::Num(row.map_id as f64));
    state.push(Val::Num(0.0));
    state.push(Val::Bool(row.is_raid));
}

fn push_instance_base_tuple(state: &mut LuaState, row: &data::Instance) {
    let name = create_string(state, row.name);
    let description = create_string(state, row.description);
    let link_dungeon_id = linked_lfd_dungeon_id(row);
    state.push(name);
    state.push(description);
    state.push(Val::Num(row.bg_file_id as f64));
    state.push(Val::Num(row.button_file_id as f64));
    state.push(Val::Num(row.lore_file_id as f64));
    state.push(Val::Num(row.button_small_file_id as f64));
    state.push(Val::Num(row.area_id as f64));
    state.push(Val::Num(if row.is_raid { row.id as f64 } else { 0.0 }));
    state.push(Val::Num(link_dungeon_id as f64));
}

fn linked_lfd_dungeon_id(row: &data::Instance) -> u32 {
    if row.is_raid {
        return 0;
    }
    match row.name {
        "Ara-Kara, City of Echoes" => 1201,
        "City of Threads" => 1202,
        "Mists of Tirna Scithe" => 1203,
        "The Stonevault" => 1204,
        "Grim Batol" => 1205,
        "The Dawnbreaker" => 1206,
        "Darkflame Cleft" => 1207,
        "The Rookery" => 1208,
        _ => row.id,
    }
}

fn build_loot_table(state: &mut LuaState, item_id: u32, encounter_id: u32) -> Val {
    let table = create_table(state);
    let item = items::get_item(item_id);
    let name_str = item.map(|i| i.name).unwrap_or("");
    let icon_id = loot_icon_file_data_id(item) as f64;
    let quality_color = loot_quality_color_code(item);
    let inv_type_num = item.map(|i| i.inventory_type as f64).unwrap_or(0.0);
    let link_str = item_link(item_id);
    let slot_str = inv_type_slot(item.map(|i| i.inventory_type).unwrap_or(0));

    let name = create_string(state, name_str);
    let item_quality = create_string(state, quality_color);
    let link = create_string(state, &link_str);
    let slot = create_string(state, slot_str);
    let armor_type = create_string(state, "");

    table_set(state, table, "itemID", Val::Num(item_id as f64));
    table_set(state, table, "name", name);
    table_set(state, table, "icon", Val::Num(icon_id));
    table_set(state, table, "itemQuality", item_quality);
    table_set(state, table, "inventoryType", Val::Num(inv_type_num));
    table_set(state, table, "link", link);
    table_set(state, table, "slot", slot);
    table_set(state, table, "armorType", armor_type);
    table_set(state, table, "encounterID", Val::Num(encounter_id as f64));
    table_set(state, table, "displayAsPerPlayerLoot", Val::Bool(false));
    table_set(state, table, "displayAsExtremelyRare", Val::Bool(false));
    table_set(state, table, "displayAsVeryRare", Val::Bool(false));
    table_set(state, table, "handError", Val::Bool(false));
    table_set(state, table, "weaponTypeError", Val::Bool(false));
    table_set(state, table, "displaySeasonID", Val::Nil);
    table_set(state, table, "filterType", Val::Num(0.0));
    table
}

fn loot_icon_file_data_id(item: Option<&items::ItemInfo>) -> u32 {
    item.and_then(|i| (i.icon_file_data_id != 0).then_some(i.icon_file_data_id))
        .unwrap_or(134400)
}

fn loot_quality_color_code(item: Option<&items::ItemInfo>) -> &'static str {
    let quality = item.map(|i| i.quality).unwrap_or(1);
    item_quality_color_code(quality)
}

fn item_quality_color_code(quality: u8) -> &'static str {
    ITEM_QUALITY_COLORS_DATA
        .iter()
        .find_map(|(index, _, _, _, hex)| (*index == quality as i32).then_some(*hex))
        .unwrap_or("ffffffff")
}

fn item_link(item_id: u32) -> String {
    if item_id == 0 {
        return String::new();
    }
    let name = items::get_item(item_id)
        .map(|i| i.name)
        .unwrap_or("Unknown");
    format!("|cffffffff|Hitem:{item_id}|h[{name}]|h|r")
}

fn inv_type_slot(inv_type: u8) -> &'static str {
    match inv_type {
        1 => "INVTYPE_HEAD",
        2 => "INVTYPE_NECK",
        3 => "INVTYPE_SHOULDER",
        4 => "INVTYPE_BODY",
        5 => "INVTYPE_CHEST",
        6 => "INVTYPE_WAIST",
        7 => "INVTYPE_LEGS",
        8 => "INVTYPE_FEET",
        9 => "INVTYPE_WRIST",
        10 => "INVTYPE_HAND",
        11 => "INVTYPE_FINGER",
        12 => "INVTYPE_TRINKET",
        13 => "INVTYPE_WEAPON",
        14 => "INVTYPE_SHIELD",
        15 => "INVTYPE_RANGED",
        16 => "INVTYPE_CLOAK",
        17 => "INVTYPE_2HWEAPON",
        20 => "INVTYPE_ROBE",
        21 => "INVTYPE_WEAPONMAINHAND",
        22 => "INVTYPE_WEAPONOFFHAND",
        23 => "INVTYPE_HOLDABLE",
        25 => "INVTYPE_THROWN",
        26 => "INVTYPE_RANGEDRIGHT",
        28 => "INVTYPE_RELIC",
        _ => "",
    }
}

fn filtered_loot(encounter_id: u32, instance_id: u32, difficulty: u32) -> Vec<&'static data::Loot> {
    if encounter_id != 0 {
        return data::loot_for_encounter(encounter_id)
            .iter()
            .copied()
            .filter(|l| {
                difficulty == 0
                    || l.difficulty_mask == 0
                    || (l.difficulty_mask & (1 << difficulty.saturating_sub(1)) != 0)
            })
            .collect();
    }
    if instance_id != 0 {
        let mut all = Vec::new();
        for encounter in data::encounters_for_instance(instance_id) {
            all.extend(data::loot_for_encounter(encounter.id).iter().copied());
        }
        return all;
    }
    Vec::new()
}

fn compute_search_results(query: &str) -> Vec<crate::lua_api::state::EncounterJournalSearchResult> {
    use crate::lua_api::state::EncounterJournalSearchResult;
    let needle = query.trim().to_ascii_lowercase();
    if needle.is_empty() {
        return Vec::new();
    }
    let mut results = Vec::new();
    for instance in data::INSTANCES.iter() {
        if instance.name.to_ascii_lowercase().contains(&needle) {
            results.push(EncounterJournalSearchResult {
                id: instance.id,
                kind: 1,
                instance_id: instance.id,
                ..Default::default()
            });
            if results.len() >= 200 {
                return results;
            }
        }
    }
    for encounter in data::ENCOUNTERS.iter() {
        if encounter.name.to_ascii_lowercase().contains(&needle) {
            results.push(EncounterJournalSearchResult {
                id: encounter.id,
                kind: 2,
                instance_id: encounter.instance_id,
                encounter_id: encounter.id,
                ..Default::default()
            });
            if results.len() >= 200 {
                return results;
            }
        }
    }
    for section in data::SECTIONS.iter().take(5000) {
        if section.title.to_ascii_lowercase().contains(&needle) {
            results.push(EncounterJournalSearchResult {
                id: section.id,
                kind: 3,
                instance_id: 0,
                encounter_id: section.encounter_id,
                ..Default::default()
            });
            if results.len() >= 200 {
                return results;
            }
        }
    }
    results
}
