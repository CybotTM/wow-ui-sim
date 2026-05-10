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
//! - Classic honor stat globals — read from `PvpHonorState`.

use crate::lua_api::methods::{borrow_state, val_to_string};
use crate::lua_api::methods::{borrow_state_mut, create_string, create_table, table_set};
use crate::lua_api::state_types::PvpHonorState;
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

fn get_personal_rated_info(state: &mut LuaState) -> LuaResult<u32> {
    let _bracket_index = stack_i32(state, 1).unwrap_or(0);
    state.push(Val::Num(0.0)); // rating
    state.push(Val::Num(0.0)); // seasonBest
    state.push(Val::Num(0.0)); // weeklyBest
    state.push(Val::Num(0.0)); // seasonPlayed
    state.push(Val::Num(0.0)); // seasonWon
    state.push(Val::Num(0.0)); // weeklyPlayed
    state.push(Val::Num(0.0)); // weeklyWon
    state.push(Val::Num(0.0)); // lastWeeksBest
    state.push(Val::Bool(false)); // hasWon
    state.push(Val::Nil); // pvpTier
    state.push(Val::Num(0.0)); // ranking
    state.push(Val::Num(0.0)); // roundsSeasonPlayed
    state.push(Val::Num(0.0)); // roundsSeasonWon
    state.push(Val::Num(0.0)); // roundsWeeklyPlayed
    state.push(Val::Num(0.0)); // roundsWeeklyWon
    Ok(15)
}

fn honor_system_enabled(state: &mut LuaState) -> LuaResult<u32> {
    let enabled = borrow_state(state)?.pvp_honor.classic_honor_system_enabled;
    state.push(Val::Bool(enabled));
    Ok(1)
}

fn get_pvp_yesterday_stats(state: &mut LuaState) -> LuaResult<u32> {
    push_pvp_pair(state, |xp| {
        (
            xp.yesterday_honorable_kills,
            xp.yesterday_dishonorable_kills,
        )
    })
}

fn get_pvp_this_week_stats(state: &mut LuaState) -> LuaResult<u32> {
    push_pvp_pair(state, |xp| {
        (xp.this_week_honorable_kills, xp.this_week_contribution)
    })
}

fn get_pvp_last_week_stats(state: &mut LuaState) -> LuaResult<u32> {
    let (honorable_kills, dishonorable_kills, contribution, rank) = {
        let state_ref = borrow_state(state)?;
        let xp = &state_ref.pvp_honor;
        (
            xp.last_week_honorable_kills,
            xp.last_week_dishonorable_kills,
            xp.last_week_contribution,
            xp.last_week_rank,
        )
    };
    state.push(Val::Num(honorable_kills as f64));
    state.push(Val::Num(dishonorable_kills as f64));
    state.push(Val::Num(contribution as f64));
    state.push(Val::Num(rank as f64));
    Ok(4)
}

fn get_pvp_session_stats(state: &mut LuaState) -> LuaResult<u32> {
    push_pvp_pair(state, |xp| {
        (xp.session_honorable_kills, xp.session_dishonorable_kills)
    })
}

fn get_pvp_lifetime_stats(state: &mut LuaState) -> LuaResult<u32> {
    let (honorable_kills, dishonorable_kills, highest_rank) = {
        let state_ref = borrow_state(state)?;
        let xp = &state_ref.pvp_honor;
        (
            xp.lifetime_honorable_kills,
            xp.lifetime_dishonorable_kills,
            xp.lifetime_highest_rank,
        )
    };
    state.push(Val::Num(honorable_kills as f64));
    state.push(Val::Num(dishonorable_kills as f64));
    state.push(Val::Num(highest_rank as f64));
    Ok(3)
}

fn get_pvp_rank_info(state: &mut LuaState) -> LuaResult<u32> {
    let rank = stack_i32(state, 1).unwrap_or(0);
    let rank_name = if rank > 0 { "Rank" } else { "None" };
    let rank_name = create_string(state, rank_name);
    state.push(rank_name);
    state.push(Val::Num(rank.max(0) as f64));
    Ok(2)
}

fn unit_pvp_rank(state: &mut LuaState) -> LuaResult<u32> {
    let rank = if stack_arg_is_player(state, 1) {
        borrow_state(state)?.player.honor_level
    } else {
        0
    };
    state.push(Val::Num(rank as f64));
    Ok(1)
}

fn get_pvp_rank_progress(state: &mut LuaState) -> LuaResult<u32> {
    let progress = borrow_state(state)?.pvp_honor.rank_progress;
    state.push(Val::Num(progress));
    Ok(1)
}

fn push_pvp_pair(
    state: &mut LuaState,
    read: impl FnOnce(&PvpHonorState) -> (i32, i32),
) -> LuaResult<u32> {
    let (first, second) = read(&borrow_state(state)?.pvp_honor);
    state.push(Val::Num(first as f64));
    state.push(Val::Num(second as f64));
    Ok(2)
}

fn stack_arg_is_player(state: &LuaState, index: i32) -> bool {
    val_to_string(state, stack_val(state, index)).as_deref() == Some("player")
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
    LuaApiMut::register_function(lua, "GetPersonalRatedInfo", get_personal_rated_info)?;
    LuaApiMut::register_function(lua, "IsSubZonePVP", is_sub_zone_pvp)?;
    LuaApiMut::register_function(lua, "HonorSystemEnabled", honor_system_enabled)?;
    LuaApiMut::register_function(lua, "GetPVPYesterdayStats", get_pvp_yesterday_stats)?;
    LuaApiMut::register_function(lua, "GetPVPThisWeekStats", get_pvp_this_week_stats)?;
    LuaApiMut::register_function(lua, "GetPVPLastWeekStats", get_pvp_last_week_stats)?;
    LuaApiMut::register_function(lua, "GetPVPSessionStats", get_pvp_session_stats)?;
    LuaApiMut::register_function(lua, "GetPVPLifetimeStats", get_pvp_lifetime_stats)?;
    LuaApiMut::register_function(lua, "GetPVPRankInfo", get_pvp_rank_info)?;
    LuaApiMut::register_function(lua, "UnitPVPRank", unit_pvp_rank)?;
    LuaApiMut::register_function(lua, "GetPVPRankProgress", get_pvp_rank_progress)?;
    LuaApiMut::register_function(lua, "GetWorldPVPAreaInfo", get_world_pvp_area_info)?;
    LuaApiMut::register_function(lua, "GetHolidayBGInfo", get_holiday_bg_info)?;
    LuaApiMut::register_function(lua, "GetLocklistMap", get_locklist_map)?;
    LuaApiMut::register_function(lua, "GetLocklistMapName", get_locklist_map_name)?;
    LuaApiMut::register_function(lua, "SetLocklistMap", set_locklist_map)?;
    LuaApiMut::register_function(lua, "ClearLocklistMap", clear_locklist_map)?;
    Ok(())
}
