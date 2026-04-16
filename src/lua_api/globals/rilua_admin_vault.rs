//! Rilua A_Admin handlers — Vault.
//!
//! Extracted from rilua_admin_world.rs per the 750-line file cap and to keep
//! each sub-module focused on a single concern. The parent entry
//! point in rilua_admin.rs imports these as pub(super) and weaves
//! them into the A_Admin TableBuilder chain.

use crate::lua_api::rilua_methods::borrow_state_mut;
use crate::lua_bridge::FromStack;
use rilua::LuaResult;
use rilua::vm::state::LuaState;

// ── Vault ─────────────────────────────────────────────────────────────────────

pub(super) fn set_vault_activity(state: &mut LuaState) -> LuaResult<u32> {
    use crate::lua_api::state::GreatVaultActivity;
    let atype = i32::from_stack(state, 1)?;
    let index = i32::from_stack(state, 2)?;
    let threshold = i32::from_stack(state, 3)?;
    let progress = i32::from_stack(state, 4)?;
    let level = i32::from_stack(state, 5)?;
    let activity = GreatVaultActivity {
        activity_type: atype,
        index,
        threshold,
        progress,
        level,
    };
    let mut st = borrow_state_mut(state)?;
    if let Some(existing) = st
        .world
        .great_vault_activities
        .iter_mut()
        .find(|a| a.activity_type == atype && a.index == index)
    {
        *existing = activity;
    } else {
        st.world.great_vault_activities.push(activity);
    }
    Ok(0)
}

pub(super) fn set_vault_rewards(state: &mut LuaState) -> LuaResult<u32> {
    let has = bool::from_stack(state, 1)?;
    let can_claim = Option::<bool>::from_stack(state, 2)?;
    let mut st = borrow_state_mut(state)?;
    st.world.great_vault_has_rewards = has;
    st.world.great_vault_can_claim = can_claim.unwrap_or(has);
    Ok(0)
}

pub(super) fn clear_vault(state: &mut LuaState) -> LuaResult<u32> {
    let mut st = borrow_state_mut(state)?;
    st.world.great_vault_activities.clear();
    st.world.great_vault_has_rewards = false;
    st.world.great_vault_can_claim = false;
    Ok(0)
}
