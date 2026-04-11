//! Extra C_* namespace stubs and global tables split from c_stubs_api.rs.

mod account;
mod constants;
mod globals;
mod namespaces;
mod tables;
mod utilities;

use crate::lua_api::SimState;
use mlua::{Lua, Result, Value};
use std::cell::RefCell;
use std::rc::Rc;

/// Convert a Lua value to f64 for abbreviation functions, returning None for non-numeric.
fn to_abbrev_number(value: &Value) -> Option<f64> {
    match value {
        Value::Nil => None,
        Value::Number(n) => Some(*n),
        Value::Integer(n) => Some(*n as f64),
        Value::String(s) => s.to_str().ok()?.parse::<f64>().ok(),
        _ => None,
    }
}

/// Format a number with B/M/K suffixes. threshold_k controls K cutoff (10000 or 1000).
fn format_abbreviated(n: f64, threshold_k: f64) -> String {
    if n >= 1_000_000_000.0 {
        format!("{:.1}B", n / 1_000_000_000.0)
    } else if n >= 1_000_000.0 {
        format!("{:.1}M", n / 1_000_000.0)
    } else if n >= threshold_k {
        format!("{:.1}K", n / 1_000.0)
    } else {
        format!("{}", n.floor() as i64)
    }
}

/// Register all extra stubs (called from c_stubs_api::register_c_stubs_api).
pub fn register_extra_stubs(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let g = lua.globals();
    utilities::register_missing_c_namespaces(lua, &g)?;
    globals::register_missing_global_functions(lua, &g)?;
    constants::register_missing_constants(lua, &g)?;
    tables::register_missing_global_tables(lua, &g)?;
    super::c_stubs_achievement::register_simulate_ping(lua)?;
    super::c_stubs_api_combat::fixup_combat_log_aliases(lua, &g)?;
    register_secure_namespaces(lua, &g)?;
    let _ = state;
    Ok(())
}

pub(crate) fn register_diff_missing_namespaces(
    lua: &Lua,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    let g = lua.globals();
    account::register_account_encounter_proto_namespaces(lua, &g, state)?;
    utilities::register_reincarnation_table_util(lua, &g)
}

/// Secure/premium/niche C_* namespaces referenced during addon loading.
fn register_secure_namespaces(lua: &Lua, g: &mlua::Table) -> Result<()> {
    super::c_stubs_api_secure::register_auth_ping_store(lua, g)?;
    super::c_stubs_api_social::register_social_feature_stubs(lua, g)?;
    Ok(())
}
