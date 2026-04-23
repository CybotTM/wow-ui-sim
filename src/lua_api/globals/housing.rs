//! `C_Housing` namespace — currently just `IsHousingServiceEnabled`.
//!
//! MainMenuBarMicroButtons gates the Housing micro-button on this probe.
//! Sim has no housing service, default is `false`. Admin
//! `A_Admin.SetHousingServiceEnabled(b?)` flips the flag for tests that want
//! to render the Housing button.
//!
//! `C_Housing` namespace table is provided by the Lua bootstrap
//! `__wow_merge_namespace` so other unimplemented members still fall through
//! to the no-op metamethod.

use crate::lua_api::methods::{borrow_state, borrow_state_mut, create_table};
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

pub fn is_housing_service_enabled(state: &mut LuaState) -> LuaResult<u32> {
    let enabled = borrow_state(state)?.housing_service_enabled;
    state.push(Val::Bool(enabled));
    Ok(1)
}

/// `C_Housing.HasHousingExpansionAccess` — gates the Midnight housing
/// dashboard. Blizzard_HousingDashboard/Blizzard_HousingDashboardHouseInfoContent
/// blocks dashboard access when this returns false; the sim grants access so
/// the dashboard renders populated.
pub fn has_housing_expansion_access(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    Ok(1)
}

fn ensure_c_housing_table(state: &mut LuaState) -> GcRef<Table> {
    let key = state.gc.intern_string_static(b"C_Housing");
    let global = state.global;
    let existing = state
        .gc
        .tables
        .get(global)
        .map(|t| t.get_str(key, &state.gc.string_arena));
    if let Some(Val::Table(r)) = existing {
        return r;
    }
    let new_val = create_table(state);
    let Val::Table(new_ref) = new_val else {
        unreachable!("create_table must return a table");
    };
    if let Some(global_table) = state.gc.tables.get_mut(global) {
        let _ = global_table.raw_set(Val::Str(key), new_val, &state.gc.string_arena);
    }
    state.gc.barrier_back(global);
    new_ref
}

pub fn register_all(lua: &mut rilua::Lua) -> LuaResult<()> {
    use rilua::LuaApiMut;
    let state = lua.state_mut();
    let table_ref = ensure_c_housing_table(state);
    table_set_rust_fn_static(
        state,
        table_ref,
        "IsHousingServiceEnabled",
        is_housing_service_enabled,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "HasHousingExpansionAccess",
        has_housing_expansion_access,
    )?;
    Ok(())
}

/// `A_Admin.SetHousingServiceEnabled(enabled?)` — missing arg defaults to
/// `true` so `A_Admin.SetHousingServiceEnabled()` opens housing.
pub fn admin_set_housing_service_enabled(state: &mut LuaState) -> LuaResult<u32> {
    let enabled = Option::<bool>::from_stack(state, 1)?.unwrap_or(true);
    borrow_state_mut(state)?.housing_service_enabled = enabled;
    Ok(0)
}
