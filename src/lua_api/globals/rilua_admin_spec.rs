//! Rilua A_Admin handlers — Spec & talents.
//!
//! Extracted from rilua_admin_extras.rs per the 750-line file cap and to keep
//! each sub-module focused on a single concern. The parent entry
//! point in rilua_admin.rs imports these as pub(super) and weaves
//! them into the A_Admin TableBuilder chain.

use crate::lua_api::rilua_methods::borrow_state_mut;
use crate::lua_bridge::FromStack;
use rilua::LuaResult;
use rilua::vm::state::LuaState;

// ── Spec & talents ────────────────────────────────────────────────────────────

pub(super) fn set_spec(state: &mut LuaState) -> LuaResult<u32> {
    let spec_index = i32::from_stack(state, 1)?;
    borrow_state_mut(state)?.player.active_spec_index = spec_index;
    Ok(0)
}

pub(super) fn set_talent_rank(state: &mut LuaState) -> LuaResult<u32> {
    let node_id = u32::from_stack(state, 1)?;
    let rank = u32::from_stack(state, 2)?;
    borrow_state_mut(state)?
        .talents
        .set_node_rank(node_id, rank);
    Ok(0)
}

pub(super) fn set_talent_selection(state: &mut LuaState) -> LuaResult<u32> {
    let node_id = u32::from_stack(state, 1)?;
    let entry_id = u32::from_stack(state, 2)?;
    borrow_state_mut(state)?
        .talents
        .set_node_selection(node_id, Some(entry_id));
    Ok(0)
}

pub(super) fn reset_talents(state: &mut LuaState) -> LuaResult<u32> {
    let mut st = borrow_state_mut(state)?;
    st.talents.clear_ranks();
    st.talents.node_selections.clear();
    st.talents.active_hero_subtree_id = None;
    Ok(0)
}
