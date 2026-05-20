//! C_ConfigurationWarnings temporary shim — warning state is not modeled.
//!
//! The shared bootstrap asks this namespace for visible warnings during
//! startup. Until the simulator has a real configuration-warning source,
//! expose an empty list plus a local "seen" cache so callers can exercise the
//! API shape without inventing warning data.

use crate::c_api::ensure_namespace;
use crate::lua_api::methods::{create_table, table_get, table_set, val_to_string};
use crate::lua_bridge::{stack_val, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

const SEEN_WARNINGS_KEY: &str = "__wow_seen_warnings";

pub fn register_c_configuration_warnings(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_ConfigurationWarnings")?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetConfigurationWarningSeen",
        c_config_warning_seen,
    )?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetConfigurationWarningString",
        c_config_warning_string,
    )?;
    table_set_rust_fn_static(state, ns, "GetConfigurationWarnings", c_config_warnings)?;
    table_set_rust_fn_static(
        state,
        ns,
        "SetConfigurationWarningSeen",
        c_set_config_warning_seen,
    )?;
    Ok(())
}

fn c_config_warning_seen(state: &mut LuaState) -> LuaResult<u32> {
    let warning = stack_val(state, 1);
    let seen = warning_seen(state, warning);
    state.push(Val::Bool(seen));
    Ok(1)
}

fn c_config_warning_string(state: &mut LuaState) -> LuaResult<u32> {
    let _warning = stack_val(state, 1);
    state.push(Val::Nil);
    Ok(1)
}

fn c_config_warnings(state: &mut LuaState) -> LuaResult<u32> {
    let warnings = create_table(state);
    state.push(warnings);
    Ok(1)
}

fn c_set_config_warning_seen(state: &mut LuaState) -> LuaResult<u32> {
    let warning = stack_val(state, 1);
    if matches!(warning, Val::Nil) {
        return Ok(0);
    }

    let key = val_to_string(state, warning).unwrap_or_default();
    let seen_table = get_or_create_seen_table(state);
    table_set(state, seen_table, &key, Val::Bool(true));
    Ok(0)
}

fn warning_seen(state: &mut LuaState, warning: Val) -> bool {
    let key = val_to_string(state, warning).unwrap_or_default();
    let seen_table = get_or_create_seen_table(state);
    matches!(table_get(state, seen_table, &key), Val::Bool(true))
}

fn get_or_create_seen_table(state: &mut LuaState) -> Val {
    let namespace = ensure_namespace(state, "C_ConfigurationWarnings")
        .expect("C_ConfigurationWarnings namespace should be available");
    let namespace_val = Val::Table(namespace);
    let existing = table_get(state, namespace_val, SEEN_WARNINGS_KEY);
    if matches!(existing, Val::Table(_)) {
        return existing;
    }

    let created = create_table(state);
    table_set(state, namespace_val, SEEN_WARNINGS_KEY, created);
    created
}
