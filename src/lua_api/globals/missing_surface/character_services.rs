//! `C_CharacterServices` probe surface backed by
//! `SimState.character_services`.
//!
//! Migrates 3 entries off the namespace stub tables:
//!
//! - `C_CharacterServices.GetActiveCharacterUpgradeBoostType()` — returns nil
//!   when no upgrade boost is pending, or the boost-type integer when one is
//!   active.
//! - `C_CharacterServices.GetActiveClassTrialBoostType()` — returns nil when
//!   no class trial is running, or the trial-type integer when one is active.

use super::ensure_namespace;
use crate::lua_api::methods::borrow_state;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn register_character_services_surface(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_CharacterServices")?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetActiveCharacterUpgradeBoostType",
        get_active_character_upgrade_boost_type,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "GetActiveClassTrialBoostType",
        get_active_class_trial_boost_type,
    )?;
    Ok(())
}

fn get_active_character_upgrade_boost_type(state: &mut LuaState) -> LuaResult<u32> {
    let boost_type = borrow_state(state)?
        .character_services
        .active_upgrade_boost_type;
    match boost_type {
        Some(t) => state.push(Val::Num(t as f64)),
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn get_active_class_trial_boost_type(state: &mut LuaState) -> LuaResult<u32> {
    let trial_type = borrow_state(state)?
        .character_services
        .active_class_trial_boost_type;
    match trial_type {
        Some(t) => state.push(Val::Num(t as f64)),
        None => state.push(Val::Nil),
    }
    Ok(1)
}
