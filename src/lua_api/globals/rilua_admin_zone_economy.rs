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
