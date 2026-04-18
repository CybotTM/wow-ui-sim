//! Namespace-level runtime patches ported from master's
//! `src/lua_api/globals/system_api_runtime.rs`.
//!
//! The stubs layer (`stubs::namespace_stubs`) registers these methods at
//! startup, but `patch_namespace_stubs` is kept as a named entry point so
//! tests can invoke it explicitly and future callers can run it late
//! (after the stub pass) to guarantee the correct values are in place.
//!
//! # Ported surface
//!
//! - `C_UIWidgetManager.GetPowerBarWidgetSetID` → returns `0`
//! - `C_PlayerInfo.IsPlayerInRPE`               → now in `missing_surface/player_info.rs`
//! - `C_PlayerInfo.GetAlternateFormInfo`         → now in `missing_surface/player_info.rs`
//!
//! `UpdateUIParentPosition` is NOT here — it is already registered in
//! `register.rs` by a parallel agent.
//!
//! # Wiring
//!
//! Called from `register_bootstrap_globals` in
//! `src/lua_api/globals/register.rs`, right after
//! `stubs::register_all(lua.state_mut())`. Covered by
//! `tests/namespace_stubs_patched.rs`.

use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

// ── Stub implementations ──────────────────────────────────────────────────────

fn stub_zero(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn stub_false(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn stub_false_false(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    state.push(Val::Bool(false));
    Ok(2)
}

// ── Namespace table helper ────────────────────────────────────────────────────

/// Get or create a C_* namespace table in globals and return its ref.
fn ensure_namespace(state: &mut LuaState, namespace: &str) -> GcRef<Table> {
    let ns_key = state.gc.intern_string(namespace.as_bytes());
    let global = state.global;
    let existing = state
        .gc
        .tables
        .get(global)
        .map(|t| t.get_str(ns_key, &state.gc.string_arena));
    if let Some(Val::Table(t)) = existing {
        return t;
    }
    let new_table = state.gc.alloc_table(Table::new());
    if let Some(g) = state.gc.tables.get_mut(global) {
        let _ = g.raw_set(
            Val::Str(ns_key),
            Val::Table(new_table),
            &state.gc.string_arena,
        );
    }
    state.gc.barrier_back(global);
    new_table
}

// ── Public entry point ────────────────────────────────────────────────────────

/// Patch `C_UIWidgetManager` and `C_PlayerInfo` with missing methods.
///
/// Idempotent — overwrites existing entries with the same constant values.
/// Safe to call after `stubs::register_all` to guarantee master-era behaviour.
pub fn patch_namespace_stubs(state: &mut LuaState) {
    let widget_mgr = ensure_namespace(state, "C_UIWidgetManager");
    let _ = table_set_rust_fn_static(state, widget_mgr, "GetPowerBarWidgetSetID", stub_zero);

    let player_info = ensure_namespace(state, "C_PlayerInfo");
    let _ = table_set_rust_fn_static(state, player_info, "IsPlayerInRPE", stub_false);
    let _ = table_set_rust_fn_static(state, player_info, "GetAlternateFormInfo", stub_false_false);
}
