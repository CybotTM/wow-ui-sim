//! Legacy talent / spellbook-tab / skill-line probe globals.
//!
//! Migrates 9 entries off `GLOBAL_ZERO_STUBS`:
//!
//! - `GetNumTalentTabs()`         → 0 (pre-MoP talent trees were dropped;
//!   modern addons that still probe this expect a falsy / zero result).
//! - `GetTalentInfo(tab, index)`  → nil — no pre-MoP tree to query.
//! - `GetTalentInfoBySpecialization(...)` → nil — legacy talent-row helper,
//!   same unsupported grid as `GetTalentInfo`.
//! - `GetNumSpellTabs()`          → 1 (the class spellbook tab; specs /
//!   pet tabs aren't modelled here).
//! - `GetSpellTabInfo(tab)`       → `(name, icon, offset, numSpells,
//!   isGuild, specID)` derived from `PlayerState.class_index` +
//!   `active_spec_index`.
//! - `GetPvpTalentSlotInfo(slot)` → minimal table (3 slots, all
//!   unlocked but unselected).
//! - `GetArenaOpponentSpec(idx)`  → 0 — the sim has no arena roster.
//! - `GetNumSkillLines()`         → 0 — the legacy skill window was
//!   removed from retail.
//! - `GetSkillLineInfo(idx)`      → nil — matches `GetNumSkillLines`.
//! - `GetSelectedSkill()`         → 0 — same rationale.

use crate::lua_api::game_data::CLASS_LABELS;
use crate::lua_api::methods::{borrow_state, create_string, create_table, table_set};
use crate::lua_bridge::stack_val;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

fn stack_i32(state: &LuaState, index: i32) -> Option<i32> {
    match stack_val(state, index) {
        Val::Num(n) => Some(n as i32),
        _ => None,
    }
}

/// `GetNumTalentTabs()` — modern retail dropped pre-MoP talent trees.
fn get_num_talent_tabs(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

/// `GetTalentInfo(tabIndex, talentIndex)` — nil because the sim
/// doesn't model the pre-MoP tree.
fn get_talent_info(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    Ok(1)
}

/// `GetTalentInfoBySpecialization(...)` — nil because the sim
/// doesn't model the obsolete talent-row grid.
fn get_talent_info_by_specialization(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    Ok(1)
}

/// `GetNumSpellTabs()` — the sim collapses the spellbook into a
/// single class tab.
fn get_num_spell_tabs(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(1.0));
    Ok(1)
}

/// `GetSpellTabInfo(tabIndex)` — retail:
/// `(name, icon, offset, numSpells, isGuild, specID)`. The sim only
/// exposes a single tab, so indexes outside `[1, 1]` return nil to
/// match retail's "tab doesn't exist" shape.
fn get_spell_tab_info(state: &mut LuaState) -> LuaResult<u32> {
    let tab_index = stack_i32(state, 1).unwrap_or(0);
    if tab_index != 1 {
        state.push(Val::Nil);
        return Ok(1);
    }
    let (class_label, spec_id) = {
        let sim = borrow_state(state)?;
        let class_idx = sim.player.class_index.max(1).min(CLASS_LABELS.len() as i32) as usize - 1;
        (CLASS_LABELS[class_idx], sim.player.active_spec_index)
    };
    let name = create_string(state, class_label);
    let icon = create_string(state, "Interface\\Icons\\Spell_Holy_PowerWordShield");
    state.push(name);
    state.push(icon);
    state.push(Val::Num(0.0)); // offset (first spell index in tab)
    state.push(Val::Num(0.0)); // numSpells (tab-local spell count)
    state.push(Val::Bool(false)); // isGuild
    state.push(Val::Num(spec_id as f64));
    Ok(6)
}

/// `GetPvpTalentSlotInfo(slotIndex)` — retail returns a table with
/// `enabled`, `locked`, `selectedTalentID`, `slotIndex`. The sim
/// exposes three slots (the retail PvP talent count) that are all
/// enabled / unlocked / unselected.
fn get_pvp_talent_slot_info(state: &mut LuaState) -> LuaResult<u32> {
    let slot_index = stack_i32(state, 1).unwrap_or(0);
    if !is_pvp_talent_slot_index(slot_index) {
        state.push(Val::Nil);
        return Ok(1);
    }
    let info = create_table(state);
    table_set(state, info, "enabled", Val::Bool(true));
    table_set(state, info, "locked", Val::Bool(false));
    table_set(state, info, "selectedTalentID", Val::Num(0.0));
    table_set(state, info, "slotIndex", Val::Num(slot_index as f64));
    state.push(info);
    Ok(1)
}

fn is_pvp_talent_slot_index(slot_index: i32) -> bool {
    matches!(slot_index, 1..=3)
}

/// `GetArenaOpponentSpec(opponentIndex)` — the sim has no arena
/// roster, so every index reports 0.
fn get_arena_opponent_spec(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

/// `GetNumSkillLines()` — the legacy skill window was removed from
/// retail. Reports 0.
fn get_num_skill_lines(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

/// `GetSkillLineInfo(index)` — matches `GetNumSkillLines`: nil for
/// every index.
fn get_skill_line_info(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    Ok(1)
}

/// `GetSelectedSkill()` — always 0; no skill window to track selections for.
fn get_selected_skill(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "GetNumTalentTabs", get_num_talent_tabs)?;
    LuaApiMut::register_function(lua, "GetTalentInfo", get_talent_info)?;
    LuaApiMut::register_function(
        lua,
        "GetTalentInfoBySpecialization",
        get_talent_info_by_specialization,
    )?;
    LuaApiMut::register_function(lua, "GetNumSpellTabs", get_num_spell_tabs)?;
    LuaApiMut::register_function(lua, "GetSpellTabInfo", get_spell_tab_info)?;
    LuaApiMut::register_function(lua, "GetPvpTalentSlotInfo", get_pvp_talent_slot_info)?;
    LuaApiMut::register_function(lua, "GetArenaOpponentSpec", get_arena_opponent_spec)?;
    LuaApiMut::register_function(lua, "GetNumSkillLines", get_num_skill_lines)?;
    LuaApiMut::register_function(lua, "GetSkillLineInfo", get_skill_line_info)?;
    LuaApiMut::register_function(lua, "GetSelectedSkill", get_selected_skill)?;
    Ok(())
}
