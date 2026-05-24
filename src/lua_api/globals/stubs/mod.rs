//! rilua RustFn stubs for global and C_* namespace functions.
//!
//! Provides trivial constant-return stubs for the majority of WoW API
//! functions that return `nil`, `false`, `0`, or an empty table.
//!
//! # Design
//!
//! Four shared stub functions cover almost every case:
//!   - `stub_nil`         → returns nothing (Lua `nil`)
//!   - `stub_false`       → returns `false`
//!   - `stub_zero`        → returns `0`
//!   - `stub_empty_table` → returns a fresh empty table `{}`
//!
//! `register_all` maps function names to the appropriate stub via static
//! slice tables, then uses helper macros to avoid per-call boilerplate.

use rilua::vm::closure::{Closure, RustClosure, RustFn};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

mod global_stubs;
mod namespace_stubs;

#[cfg(test)]
mod tests;

use global_stubs::register_global_stubs;
use namespace_stubs::register_namespace_stubs;

// ── Shared stub implementations ──────────────────────────────────────────────

/// Returns nothing — Lua sees `nil`.
pub fn stub_nil(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

/// Returns `false`.
pub fn stub_false(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

/// Returns `true`.
pub fn stub_true(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    Ok(1)
}

/// Returns `0`.
pub fn stub_zero(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

/// Returns `(0, false)` for merchant repair cost checks.
pub fn stub_repair_all_cost(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    state.push(Val::Bool(false));
    Ok(2)
}

/// Returns a fresh empty table `{}`.
pub fn stub_empty_table(state: &mut LuaState) -> LuaResult<u32> {
    let table_ref = state.gc.alloc_table(Table::new());
    state.push(Val::Table(table_ref));
    Ok(1)
}

// ── Internal registration helpers ─────────────────────────────────────────────

/// Set a `RustFn` as a global in the rilua state.
fn set_global_fn(state: &mut LuaState, name: &'static str, func: RustFn) {
    let key = state.gc.intern_string(name.as_bytes());
    let closure = Closure::Rust(RustClosure::new(func, name));
    let closure_ref = state.gc.alloc_closure(closure);
    let global = state.global;
    if let Some(g) = state.gc.tables.get_mut(global) {
        let _ = g.raw_set(
            Val::Str(key),
            Val::Function(closure_ref),
            &state.gc.string_arena,
        );
    }
    state.gc.barrier_back(global);
}

/// Resolve an existing namespace table, or create and register a new one.
fn ensure_namespace_table(state: &mut LuaState, namespace: &'static str) -> GcRef<Table> {
    let ns_key = state.gc.intern_string(namespace.as_bytes());
    let global = state.global;
    let existing = state
        .gc
        .tables
        .get(global)
        .map(|g| g.get_str(ns_key, &state.gc.string_arena))
        .unwrap_or(Val::Nil);
    match existing {
        Val::Table(t) => t,
        _ => {
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
    }
}

/// Get or create a C_* namespace table in globals, then set a `RustFn` on it.
fn set_namespace_fn(
    state: &mut LuaState,
    namespace: &'static str,
    method: &'static str,
    func: RustFn,
) {
    let ns_ref = ensure_namespace_table(state, namespace);
    let m_key = state.gc.intern_string(method.as_bytes());
    let closure = Closure::Rust(RustClosure::new(func, method));
    let closure_ref = state.gc.alloc_closure(closure);
    if let Some(ns) = state.gc.tables.get_mut(ns_ref) {
        let _ = ns.raw_set(
            Val::Str(m_key),
            Val::Function(closure_ref),
            &state.gc.string_arena,
        );
    }
    state.gc.barrier_back(ns_ref);
}

// ── Registration entry point ──────────────────────────────────────────────────

/// Register all rilua stub globals and C_* namespace stubs.
///
/// Only registers each name if the global slot is currently `nil`, so
/// hand-written implementations registered earlier always take priority.
pub fn register_all(state: &mut LuaState) {
    register_global_stubs(state);
    register_namespace_stubs(state);
}

/// Returns true if the global `name` is currently `nil`.
fn is_nil_global(state: &mut LuaState, name: &str) -> bool {
    let key = state.gc.intern_string(name.as_bytes());
    let global = state.global;
    state
        .gc
        .tables
        .get(global)
        .map(|g| g.get_str(key, &state.gc.string_arena) == Val::Nil)
        .unwrap_or(true)
}

/// Returns true if `namespace.method` is currently `nil`.
fn is_nil_namespace(state: &mut LuaState, namespace: &str, method: &str) -> bool {
    let ns_key = state.gc.intern_string(namespace.as_bytes());
    let m_key = state.gc.intern_string(method.as_bytes());
    let global = state.global;
    let ns_val = state
        .gc
        .tables
        .get(global)
        .map(|g| g.get_str(ns_key, &state.gc.string_arena))
        .unwrap_or(Val::Nil);
    match ns_val {
        Val::Table(t) => state
            .gc
            .tables
            .get(t)
            .map(|tbl| tbl.get_str(m_key, &state.gc.string_arena) == Val::Nil)
            .unwrap_or(true),
        Val::Nil => true,
        _ => false,
    }
}
