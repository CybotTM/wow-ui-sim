//! Rilua A_Admin handlers — Zone, Economy.
//!
//! Extracted from rilua_admin_extras.rs per the 750-line file cap and to keep
//! each sub-module focused on a single concern. The parent entry
//! point in rilua_admin.rs imports these as pub(super) and weaves
//! them into the A_Admin TableBuilder chain.

use crate::lua_api::rilua_methods::borrow_state_mut;
use crate::lua_bridge::FromStack;
use rilua::LuaResult;
use rilua::vm::state::LuaState;

// ── Zone ──────────────────────────────────────────────────────────────────────

pub(super) fn set_zone(state: &mut LuaState) -> LuaResult<u32> {
    let name = String::from_stack(state, 1)?;
    let id = i32::from_stack(state, 2)?;
    let mut st = borrow_state_mut(state)?;
    st.world.zone_name = name;
    st.world.zone_id = id;
    Ok(0)
}

pub(super) fn set_sub_zone(state: &mut LuaState) -> LuaResult<u32> {
    let name = String::from_stack(state, 1)?;
    borrow_state_mut(state)?.world.sub_zone_name = name;
    Ok(0)
}

pub(super) fn set_instance_info(state: &mut LuaState) -> LuaResult<u32> {
    let name = String::from_stack(state, 1)?;
    let inst_type = String::from_stack(state, 2)?;
    let difficulty = i32::from_stack(state, 3)?;
    let max_players = i32::from_stack(state, 4)?;
    let mut st = borrow_state_mut(state)?;
    st.world.instance_name = name;
    st.world.instance_type = inst_type;
    st.world.instance_difficulty = difficulty;
    st.world.instance_max_players = max_players;
    st.world.in_instance = true;
    Ok(0)
}

pub(super) fn set_in_instance(state: &mut LuaState) -> LuaResult<u32> {
    let v = bool::from_stack(state, 1)?;
    borrow_state_mut(state)?.world.in_instance = v;
    Ok(0)
}

// ── Economy ───────────────────────────────────────────────────────────────────

pub(super) fn set_money(state: &mut LuaState) -> LuaResult<u32> {
    let copper = i64::from_stack(state, 1)?;
    borrow_state_mut(state)?.player.money = copper;
    Ok(0)
}

pub(super) fn set_item_level(state: &mut LuaState) -> LuaResult<u32> {
    let ilvl = f64::from_stack(state, 1)?;
    borrow_state_mut(state)?.player.item_level = ilvl as f32;
    Ok(0)
}

// ── Network stats (for GetNetStats) ───────────────────────────────────────────

/// `A_Admin.SetNetStats(bandwidthIn, bandwidthOut, latencyHome, latencyWorld)`.
/// All four arguments are optional; missing values default to 0. Drives the
/// values returned by `GetNetStats` (registered in `rilua_net_stats.rs`).
pub(super) fn set_net_stats(state: &mut LuaState) -> LuaResult<u32> {
    let bandwidth_in = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0);
    let bandwidth_out = Option::<f64>::from_stack(state, 2)?.unwrap_or(0.0);
    let latency_home = Option::<f64>::from_stack(state, 3)?.unwrap_or(0.0);
    let latency_world = Option::<f64>::from_stack(state, 4)?.unwrap_or(0.0);
    let mut st = borrow_state_mut(state)?;
    st.net_stats.bandwidth_in_kbps = bandwidth_in;
    st.net_stats.bandwidth_out_kbps = bandwidth_out;
    st.net_stats.latency_home_ms = latency_home;
    st.net_stats.latency_world_ms = latency_world;
    Ok(0)
}

/// `A_Admin.SetStoreFrameShown(shown)`. Missing arg defaults to `true` so
/// `A_Admin.SetStoreFrameShown()` opens the store. Drives `StoreFrame_IsShown`
/// (registered in `rilua_store_frame.rs`).
pub(super) fn set_store_frame_shown(state: &mut LuaState) -> LuaResult<u32> {
    let shown = Option::<bool>::from_stack(state, 1)?.unwrap_or(true);
    borrow_state_mut(state)?.store_frame_shown = shown;
    Ok(0)
}

/// `A_Admin.SetTimerunningSeasonID(id)` — pass `nil` or `0` to clear
/// (no active seasonal mode), or a positive id to enable. Drives
/// `PlayerIsTimerunning()` (returns whether id is non-zero) and
/// `PlayerGetTimerunningSeasonID()` (returns the id, or 0 when none).
pub(super) fn set_timerunning_season_id(state: &mut LuaState) -> LuaResult<u32> {
    let id = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i64;
    let season = if id > 0 { Some(id as u32) } else { None };
    borrow_state_mut(state)?.timerunning_season_id = season;
    Ok(0)
}
