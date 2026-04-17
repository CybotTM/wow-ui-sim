//! Rilua A_Admin handlers — Buffs.
//!
//! Extracted from rilua_admin_extras.rs per the 750-line file cap and to keep
//! each sub-module focused on a single concern. The parent entry
//! point in admin.rs imports these as pub(super) and weaves
//! them into the A_Admin TableBuilder chain.

use super::admin::build_admin_buff;
use crate::lua_api::methods::borrow_state_mut;
use crate::lua_bridge::FromStack;
use rilua::LuaResult;
use rilua::vm::state::LuaState;

// ── Buffs ─────────────────────────────────────────────────────────────────────

pub(super) fn add_buff(state: &mut LuaState) -> LuaResult<u32> {
    let spell_id = i32::from_stack(state, 1)?;
    let name = String::from_stack(state, 2)?;
    let icon = String::from_stack(state, 3)?;
    let duration = f64::from_stack(state, 4)?;
    let stacks = i32::from_stack(state, 5)?;
    let mut st = borrow_state_mut(state)?;
    let buff = build_admin_buff(&st, spell_id, name, icon, duration, stacks);
    st.player.buffs.push(buff);
    Ok(0)
}

pub(super) fn remove_buff(state: &mut LuaState) -> LuaResult<u32> {
    let spell_id = i32::from_stack(state, 1)?;
    borrow_state_mut(state)?
        .player
        .buffs
        .retain(|a| a.spell_id != spell_id);
    Ok(0)
}

pub(super) fn clear_buffs(state: &mut LuaState) -> LuaResult<u32> {
    borrow_state_mut(state)?.player.buffs.clear();
    Ok(0)
}
