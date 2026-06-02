//! Temporary legacy talent, PvP talent, arena, and skill-window probe shims.
//!
//! These globals are kept as explicit temporary shims because the simulator does
//! not yet model pre-MoP talent trees, the retail PvP talent selection model,
//! arena rosters, or the removed legacy skill window.

use crate::lua_api::methods::{create_table, table_set};
use crate::lua_bridge::stack_val;
use rilua::{LuaApiMut, LuaResult, Val};

/// `GetNumTalentTabs()` — modern retail dropped pre-MoP talent trees.
fn get_num_talent_tabs(state: &mut rilua::vm::state::LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

/// `GetTalentInfo(tabIndex, talentIndex)` — nil because the sim
/// doesn't model the pre-MoP tree.
fn get_talent_info(state: &mut rilua::vm::state::LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    Ok(1)
}

/// `GetTalentInfoBySpecialization(...)` — nil because the sim
/// doesn't model the obsolete talent-row grid.
fn get_talent_info_by_specialization(state: &mut rilua::vm::state::LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    Ok(1)
}

/// `GetPvpTalentSlotInfo(slotIndex)` — placeholder for the unmodeled
/// PvP talent selection model.
fn get_pvp_talent_slot_info(state: &mut rilua::vm::state::LuaState) -> LuaResult<u32> {
    let slot_index = match stack_val(state, 1) {
        Val::Num(index) => index as i32,
        _ => 0,
    };
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

/// `GetArenaOpponentSpec(opponentIndex)` — no arena roster is modeled yet.
fn get_arena_opponent_spec(state: &mut rilua::vm::state::LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

/// `GetNumSkillLines()` — no removed legacy skill window is modeled yet.
fn get_num_skill_lines(state: &mut rilua::vm::state::LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

/// `GetSkillLineInfo(index)` — matches `GetNumSkillLines`: nil for every index.
fn get_skill_line_info(state: &mut rilua::vm::state::LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    Ok(1)
}

/// `GetSelectedSkill()` — no legacy skill selection state is modeled yet.
fn get_selected_skill(state: &mut rilua::vm::state::LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

pub fn register_all(lua: &mut rilua::Lua) -> LuaResult<()> {
    LuaApiMut::register_function(lua, "GetNumTalentTabs", get_num_talent_tabs)?;
    LuaApiMut::register_function(lua, "GetTalentInfo", get_talent_info)?;
    LuaApiMut::register_function(
        lua,
        "GetTalentInfoBySpecialization",
        get_talent_info_by_specialization,
    )?;
    LuaApiMut::register_function(lua, "GetPvpTalentSlotInfo", get_pvp_talent_slot_info)?;
    LuaApiMut::register_function(lua, "GetArenaOpponentSpec", get_arena_opponent_spec)?;
    LuaApiMut::register_function(lua, "GetNumSkillLines", get_num_skill_lines)?;
    LuaApiMut::register_function(lua, "GetSkillLineInfo", get_skill_line_info)?;
    LuaApiMut::register_function(lua, "GetSelectedSkill", get_selected_skill)?;
    Ok(())
}
