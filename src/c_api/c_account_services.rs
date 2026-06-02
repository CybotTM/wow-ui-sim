//! `C_AccountServices` probe surface backed by `SimState` account-save flags.
//!
//! The Account Save UI only needs four simple probes. The simulator keeps the
//! mutable state in `SimState` so tests can flip the flags directly and verify
//! the UI response without bootstrap glue.

use crate::c_api::helpers::ensure_namespace;
use crate::lua_api::methods::{borrow_state, borrow_state_mut};
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

const ACCOUNT_EXPORT_SUCCESS: f64 = 0.0;
const ACCOUNT_EXPORT_ALREADY_IN_PROGRESS: f64 = 11.0;
const ACCOUNT_EXPORT_UNAVAILABLE: f64 = 10.0;

pub(crate) fn register_c_account_services_surface(state: &mut LuaState) -> LuaResult<()> {
    let account_services = ensure_namespace(state, "C_AccountServices")?;
    table_set_rust_fn_static(
        state,
        account_services,
        "IsAccountSaveEnabled",
        is_account_save_enabled,
    )?;
    table_set_rust_fn_static(
        state,
        account_services,
        "IsAccountSaveInProgress",
        is_account_save_in_progress,
    )?;
    table_set_rust_fn_static(
        state,
        account_services,
        "IsAccountLockedPostSave",
        is_account_locked_post_save,
    )?;
    table_set_rust_fn_static(
        state,
        account_services,
        "SaveAccountData",
        save_account_data,
    )
}

fn is_account_save_enabled(state: &mut LuaState) -> LuaResult<u32> {
    let enabled = borrow_state(state)?.account_save_enabled;
    state.push(Val::Bool(enabled));
    Ok(1)
}

fn is_account_save_in_progress(state: &mut LuaState) -> LuaResult<u32> {
    let in_progress = borrow_state(state)?.account_save_in_progress;
    state.push(Val::Bool(in_progress));
    Ok(1)
}

fn is_account_locked_post_save(state: &mut LuaState) -> LuaResult<u32> {
    let locked = borrow_state(state)?.account_locked_post_save;
    state.push(Val::Bool(locked));
    Ok(1)
}

fn save_account_data(state: &mut LuaState) -> LuaResult<u32> {
    let (started, result_code, lock_account) = {
        let mut sim = borrow_state_mut(state)?;
        if !sim.account_save_enabled {
            (false, ACCOUNT_EXPORT_UNAVAILABLE, false)
        } else if sim.account_save_in_progress {
            (false, ACCOUNT_EXPORT_ALREADY_IN_PROGRESS, false)
        } else {
            sim.account_save_in_progress = false;
            (true, ACCOUNT_EXPORT_SUCCESS, true)
        }
    };

    if lock_account {
        borrow_state_mut(state)?.account_locked_post_save = true;
    }

    state.push(Val::Bool(started));
    state.push(Val::Num(result_code));
    Ok(2)
}
