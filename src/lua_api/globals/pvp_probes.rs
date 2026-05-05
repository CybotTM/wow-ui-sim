//! PvP probe globals reading from `WorldState.pvp_*` plus seeded PvP
//! battleground info and `SimState.pvp_last_honor_gain` /
//! `player.pvp_enabled`.
//!
//! Migrates 4 entries off `GLOBAL_FALSE_STUBS`:
//!
//! - `IsInActiveWorldPVP()`     — true when `world.pvp_type` is one of the
//!                                active-combat tokens (`"combat"`,
//!                                `"hostile"`, `"arena"`).
//! - `GetPVPDesired()`          — `player.pvp_enabled`.
//! - `GetPVPLastHonorGain()`    — `pvp_last_honor_gain` (i32).
//! - `IsSubZonePVP()`           — `world.is_sub_zone_pvp`.
//! - `GetWorldPVPAreaInfo(idx)`  — seeded battleground info table.
//! - `GetHolidayBGInfo()`       — seeded random BG info table.
//! - `GetLocklistMap(idx)`      — mutable locklist slot lookup.
//! - `GetLocklistMapName(idx)`  — locklist slot name lookup.
//! - `SetLocklistMap(mapID)`    — append if absent.
//! - `ClearLocklistMap(mapID)`  — remove matching entries.

use crate::lua_api::methods::borrow_state;
use crate::lua_api::methods::{borrow_state_mut, create_string, create_table, table_set};
use crate::lua_bridge::stack_val;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};
use std::collections::HashSet;

fn stack_i32(state: &LuaState, index: i32) -> Option<i32> {
    match stack_val(state, index) {
        Val::Num(n) => Some(n as i32),
        _ => None,
    }
}

pub(super) fn is_in_active_world_pvp(state: &mut LuaState) -> LuaResult<u32> {
    let active = {
        let st = borrow_state(state)?;
        matches!(st.world.pvp_type.as_str(), "combat" | "hostile" | "arena")
    };
    state.push(Val::Bool(active));
    Ok(1)
}

pub(super) fn get_pvp_desired(state: &mut LuaState) -> LuaResult<u32> {
    let desired = borrow_state(state)?.player.pvp_enabled;
    state.push(Val::Bool(desired));
    Ok(1)
}

pub(super) fn get_pvp_last_honor_gain(state: &mut LuaState) -> LuaResult<u32> {
    let honor = borrow_state(state)?.pvp_last_honor_gain;
    state.push(Val::Num(honor as f64));
    Ok(1)
}

pub(super) fn is_sub_zone_pvp(state: &mut LuaState) -> LuaResult<u32> {
    let flag = borrow_state(state)?.world.is_sub_zone_pvp;
    state.push(Val::Bool(flag));
    Ok(1)
}

pub(super) fn get_world_pvp_area_info(state: &mut LuaState) -> LuaResult<u32> {
    let index = stack_i32(state, 1).unwrap_or(0);
    let Some(area) = borrow_state(state)?
        .world
        .world_pvp_areas
        .get(index.saturating_sub(1) as usize)
        .cloned()
    else {
        return Ok(0);
    };
    let table = create_table(state);
    table_set(state, table, "bgID", Val::Num(area.bg_id as f64));
    table_set(state, table, "canEnter", Val::Bool(area.can_enter));
    table_set(state, table, "canQueue", Val::Bool(area.can_queue));
    table_set(state, table, "isActive", Val::Bool(area.is_active));
    table_set(state, table, "maxLevel", Val::Num(area.max_level as f64));
    table_set(state, table, "minLevel", Val::Num(area.min_level as f64));
    let name = create_string(state, &area.name);
    table_set(state, table, "name", name);
    table_set(state, table, "startTime", Val::Num(area.start_time as f64));
    state.push(table);
    Ok(1)
}

pub(super) fn get_holiday_bg_info(state: &mut LuaState) -> LuaResult<u32> {
    let Some(info) = borrow_state(state)?.world.holiday_bg_info.clone() else {
        return Ok(0);
    };
    let table = create_table(state);
    table_set(state, table, "bgID", Val::Num(info.bg_id as f64));
    table_set(state, table, "bgIndex", Val::Num(info.bg_index as f64));
    table_set(state, table, "canQueue", Val::Bool(info.can_queue));
    table_set(
        state,
        table,
        "hasRandomWinToday",
        Val::Bool(info.has_random_win_today),
    );
    table_set(state, table, "maxLevel", Val::Num(info.max_level as f64));
    table_set(state, table, "minLevel", Val::Num(info.min_level as f64));
    let name = create_string(state, &info.name);
    table_set(state, table, "name", name);
    state.push(table);
    Ok(1)
}

pub(super) fn get_locklist_map(state: &mut LuaState) -> LuaResult<u32> {
    let index = stack_i32(state, 1).unwrap_or(0);
    let map_id = borrow_state(state)?
        .world
        .locklist_maps
        .get(index.saturating_sub(1) as usize)
        .copied()
        .unwrap_or(0);
    state.push(Val::Num(map_id as f64));
    Ok(1)
}

pub(super) fn get_locklist_map_name(state: &mut LuaState) -> LuaResult<u32> {
    let index = stack_i32(state, 1).unwrap_or(0);
    let map_id = borrow_state(state)?
        .world
        .locklist_maps
        .get(index.saturating_sub(1) as usize)
        .copied()
        .unwrap_or(0);
    if let Some(name) = locklist_map_name(map_id) {
        let name = create_string(state, name);
        state.push(name);
    } else {
        state.push(Val::Nil);
    }
    Ok(1)
}

pub(super) fn set_locklist_map(state: &mut LuaState) -> LuaResult<u32> {
    let map_id = stack_i32(state, 1).unwrap_or(0);
    let mut sim = borrow_state_mut(state)?;
    append_locklist_map(&mut sim.world.locklist_maps, map_id);
    Ok(0)
}

fn append_locklist_map(locklist_maps: &mut Vec<u32>, map_id: i32) {
    if map_id <= 0 {
        return;
    }
    let map_id = map_id as u32;
    let mut known_maps = locklist_maps.iter().copied().collect::<HashSet<_>>();
    if known_maps.insert(map_id) {
        locklist_maps.push(map_id);
    }
}

pub(super) fn clear_locklist_map(state: &mut LuaState) -> LuaResult<u32> {
    let map_id = stack_i32(state, 1).unwrap_or(0) as u32;
    let mut sim = borrow_state_mut(state)?;
    sim.world.locklist_maps.retain(|current| *current != map_id);
    Ok(0)
}

fn locklist_map_name(map_id: u32) -> Option<&'static str> {
    match map_id {
        566 => Some("Eye of the Storm"),
        727 => Some("Silvershard Mines"),
        _ => None,
    }
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "IsInActiveWorldPVP", is_in_active_world_pvp)?;
    LuaApiMut::register_function(lua, "GetPVPDesired", get_pvp_desired)?;
    LuaApiMut::register_function(lua, "GetPVPLastHonorGain", get_pvp_last_honor_gain)?;
    LuaApiMut::register_function(lua, "IsSubZonePVP", is_sub_zone_pvp)?;
    LuaApiMut::register_function(lua, "GetWorldPVPAreaInfo", get_world_pvp_area_info)?;
    LuaApiMut::register_function(lua, "GetHolidayBGInfo", get_holiday_bg_info)?;
    LuaApiMut::register_function(lua, "GetLocklistMap", get_locklist_map)?;
    LuaApiMut::register_function(lua, "GetLocklistMapName", get_locklist_map_name)?;
    LuaApiMut::register_function(lua, "SetLocklistMap", set_locklist_map)?;
    LuaApiMut::register_function(lua, "ClearLocklistMap", clear_locklist_map)?;
    Ok(())
}
