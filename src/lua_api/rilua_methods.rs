//! rilua-side frame method helpers for Phase 3 migration.
//!
//! Production frame methods need `SimState` access (behind `Rc<RefCell<>>`),
//! which doesn't fit the `define_methods!` macro's `FrameArena` pattern.
//! Instead, methods are raw `RustFn`s that use these helpers to extract
//! the frame ID from a rilua-backed table and borrow `SimState`.

use super::SimState;
use super::env::WowLuaAppData;
use crate::lua_bridge::create_frame_table;
use crate::lua_bridge::stack_val;
use rilua::vm::callinfo::LUA_MULTRET;
use rilua::vm::execute::{CallResult, execute};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val, runtime_error};
use std::cell::{Ref, RefMut};
use std::rc::Rc;

/// Extract the frame ID (u64) from a Lua argument (a backed table).
///
/// The table's backing `(index, generation)` encodes the widget ID as
/// `(id as u32, (id >> 32) as u32)`.
pub fn frame_id_from_stack(state: &LuaState, index: i32) -> LuaResult<u64> {
    let val = stack_val(state, index);
    let Val::Table(table_ref) = val else {
        return Err(runtime_error("expected frame table as self argument"));
    };
    let backing = state
        .gc
        .tables
        .get(table_ref)
        .and_then(|t| t.backing())
        .ok_or_else(|| runtime_error("expected frame-backed table"))?;
    Ok((backing.0 as u64) | ((backing.1 as u64) << 32))
}

/// Borrow `SimState` immutably from rilua's app_data.
pub fn borrow_state(state: &LuaState) -> LuaResult<Ref<'_, SimState>> {
    let app = state
        .app_data::<WowLuaAppData>()
        .ok_or_else(|| runtime_error("missing WowLuaAppData"))?;
    Ok(app.sim_state.borrow())
}

/// Borrow `SimState` mutably from rilua's app_data.
pub fn borrow_state_mut(state: &LuaState) -> LuaResult<RefMut<'_, SimState>> {
    let app = state
        .app_data::<WowLuaAppData>()
        .ok_or_else(|| runtime_error("missing WowLuaAppData"))?;
    Ok(app.sim_state.borrow_mut())
}

/// Clone the owning SimState handle from app_data.
pub fn state_handle(state: &LuaState) -> LuaResult<Rc<std::cell::RefCell<SimState>>> {
    let app = state
        .app_data::<WowLuaAppData>()
        .ok_or_else(|| runtime_error("missing WowLuaAppData"))?;
    Ok(Rc::clone(&app.sim_state))
}

/// Clone the owning rilua cell from app_data for runtime loader helpers.
pub fn borrow_lua(state: &LuaState) -> LuaResult<Rc<std::cell::RefCell<rilua::Lua>>> {
    let app = state
        .app_data::<WowLuaAppData>()
        .ok_or_else(|| runtime_error("missing WowLuaAppData"))?;
    app.lua
        .as_ref()
        .cloned()
        .ok_or_else(|| runtime_error("missing WowLuaEnv lua handle"))
}

// ── Frame ref creation and caching ───────────────────────────────────

const FRAME_REFS_KEY: &str = "__rilua_frame_refs";

/// Get or create a rilua frame-backed table for the given widget ID.
///
/// Returns a cached `Val::Table` if one exists for this ID, otherwise creates
/// a new backed table with the shared frame metatable and caches it.
///
/// The metatable must be pre-registered in the registry as `__rilua_frame_mt`
/// before calling this function. If absent, the table has no metatable.
pub fn frame_ref(state: &mut LuaState, id: u64) -> LuaResult<Val> {
    let cache = frame_ref_cache(state);
    let cached = table_get_num(state, cache, id as f64);
    if cached != Val::Nil {
        return Ok(cached);
    }
    let (lo, hi) = unpack_id(id);
    let table_ref = create_frame_table(state, lo, hi);
    attach_frame_metatable(state, table_ref);
    let val = Val::Table(table_ref);
    table_set_num(state, cache, id as f64, val);
    let _ = get_or_create_frame_fields(state, id);
    Ok(val)
}

/// Extract frame ID from a `Val` (must be a backed table).
pub fn extract_frame_id(state: &LuaState, val: Val) -> Option<u64> {
    let Val::Table(table_ref) = val else {
        return None;
    };
    let (lo, hi) = state.gc.tables.get(table_ref)?.backing()?;
    Some(pack_id(lo, hi))
}

/// Get or create the per-frame fields table in the `__rilua_frame_fields`
/// registry entry and bind it to the frame env slot that `debug.getfenv`
/// callers read via `[1]`.
pub fn get_or_create_frame_fields(state: &mut LuaState, frame_id: u64) -> Val {
    let fields_registry = registry_table_or_create(state, "__rilua_frame_fields");
    let Val::Table(fields_reg_ref) = fields_registry else {
        return Val::Nil;
    };

    let existing = state
        .gc
        .tables
        .get(fields_reg_ref)
        .map(|t| {
            let int_val = t.get_int(frame_id as i64);
            if int_val != Val::Nil {
                int_val
            } else {
                t.get(Val::Num(frame_id as f64), &state.gc.string_arena)
            }
        })
        .unwrap_or(Val::Nil);
    let fields = if let Val::Table(_) = existing {
        existing
    } else {
        let created = create_table(state);
        if let Some(reg) = state.gc.tables.get_mut(fields_reg_ref) {
            let _ = reg.raw_set(Val::Num(frame_id as f64), created, &state.gc.string_arena);
        }
        created
    };

    bind_frame_fields_env_slot(state, frame_id, fields);
    fields
}

/// Sync a child frame ref into a parent frame's table.
///
/// Sets `parent_table[key] = child_frame_ref` so that Lua-side `parent.key`
/// resolves to the child frame.
pub fn sync_child_to_rilua(
    state: &mut LuaState,
    parent_id: u64,
    key: &str,
    child_id: u64,
) -> LuaResult<()> {
    let parent_val = frame_ref(state, parent_id)?;
    let child_val = frame_ref(state, child_id)?;
    let Val::Table(parent_ref) = parent_val else {
        return Ok(());
    };
    let key_ref = state.gc.intern_string(key.as_bytes());
    if let Some(t) = state.gc.tables.get_mut(parent_ref) {
        let _ = t.raw_set(Val::Str(key_ref), child_val, &state.gc.string_arena);
    }
    Ok(())
}

/// Get or create the frame ref cache table in the registry.
/// Pre-sized for ~3000 frames (typical Blizzard UI load).
fn frame_ref_cache(state: &mut LuaState) -> GcRef<Table> {
    let key_ref = state.gc.intern_string(FRAME_REFS_KEY.as_bytes());
    let registry = state.gc.tables.get(state.registry);
    if let Some(reg) = registry {
        if let Val::Table(cache) = reg.get_str(key_ref, &state.gc.string_arena) {
            return cache;
        }
    }
    // Pre-allocate for typical frame count to avoid rehashing
    let cache = state.gc.alloc_table(Table::with_sizes(4096, 0));
    if let Some(reg) = state.gc.tables.get_mut(state.registry) {
        let _ = reg.raw_set(Val::Str(key_ref), Val::Table(cache), &state.gc.string_arena);
    }
    cache
}

/// Attach the shared frame metatable to a table (if registered).
///
/// Methods are accessed via `__index` in the metatable, not copied directly.
/// This avoids ~636 raw_set calls per frame and reduces memory/GC pressure.
fn attach_frame_metatable(state: &mut LuaState, table_ref: GcRef<Table>) {
    let mt_key = state.gc.intern_string(b"__rilua_frame_mt");
    let mt_val = state
        .gc
        .tables
        .get(state.registry)
        .map(|reg| reg.get_str(mt_key, &state.gc.string_arena))
        .unwrap_or(Val::Nil);
    if let Val::Table(mt_ref) = mt_val {
        if let Some(t) = state.gc.tables.get_mut(table_ref) {
            t.set_metatable(Some(mt_ref));
        }
        // Methods accessed via __index, no need to copy
    }
}

fn copy_frame_methods_from_metatable(
    state: &mut LuaState,
    table_ref: GcRef<Table>,
    mt_ref: GcRef<Table>,
) {
    let entries = state
        .gc
        .tables
        .get(mt_ref)
        .map(|table| table.hash_entries())
        .unwrap_or_default();

    for (key, value) in entries {
        let Val::Str(str_ref) = key else {
            continue;
        };

        let Some(name) = state.gc.string_arena.get(str_ref) else {
            continue;
        };
        if name.data().starts_with(b"__") {
            continue;
        }

        if let Some(table) = state.gc.tables.get_mut(table_ref) {
            let _ = table.raw_set(key, value, &state.gc.string_arena);
        }
    }
}

/// Get a numeric-keyed value from a table.
fn table_get_num(state: &LuaState, table: GcRef<Table>, key: f64) -> Val {
    state
        .gc
        .tables
        .get(table)
        .map(|t| {
            let int_key = key as i64;
            if int_key > 0 && int_key as f64 == key {
                t.get_int(int_key)
            } else {
                t.get(Val::Num(key), &state.gc.string_arena)
            }
        })
        .unwrap_or(Val::Nil)
}

/// Set a numeric-keyed value in a table.
fn table_set_num(state: &mut LuaState, table: GcRef<Table>, key: f64, value: Val) {
    if let Some(t) = state.gc.tables.get_mut(table) {
        let int_key = key as i64;
        if int_key > 0 && int_key as f64 == key {
            let _ = t.raw_set(Val::Num(int_key as f64), value, &state.gc.string_arena);
        } else {
            let _ = t.raw_set(Val::Num(key), value, &state.gc.string_arena);
        }
    }
}

fn bind_frame_fields_env_slot(state: &mut LuaState, frame_id: u64, fields: Val) {
    let cache = frame_ref_cache(state);
    let frame_val = table_get_num(state, cache, frame_id as f64);
    let Val::Table(frame_ref) = frame_val else {
        return;
    };
    if let Some(table) = state.gc.tables.get_mut(frame_ref) {
        let _ = table.raw_set(Val::Num(1.0), fields, &state.gc.string_arena);
    }
}

// ── ID encoding ─────────────────────────────────────────────────────

/// Pack a u64 widget ID into the (u32, u32) backing slot format.
pub fn pack_id(lo: u32, hi: u32) -> u64 {
    (lo as u64) | ((hi as u64) << 32)
}

/// Unpack a u64 widget ID into (lo, hi) for table backing.
pub fn unpack_id(id: u64) -> (u32, u32) {
    (id as u32, (id >> 32) as u32)
}

// ── String creation ─────────────────────────────────────────────────

/// Create a Lua string Val from a Rust &str.
pub fn create_string(state: &mut LuaState, s: &str) -> Val {
    Val::Str(state.gc.intern_string(s.as_bytes()))
}

/// Create a Lua string Val from raw bytes.
pub fn create_string_bytes(state: &mut LuaState, bytes: &[u8]) -> Val {
    Val::Str(state.gc.intern_string(bytes))
}

/// Extract a Rust String from a Lua string Val.
pub fn val_to_string(state: &LuaState, val: Val) -> Option<String> {
    let Val::Str(str_ref) = val else { return None };
    let lua_str = state.gc.string_arena.get(str_ref)?;
    String::from_utf8(lua_str.data().to_vec()).ok()
}

// ── Function calling ────────────────────────────────────────────────

/// Call a Lua function stored as a Val with the given arguments.
///
/// Wraps `rilua::Lua::call_function` for use in frame method dispatch.
/// Returns the first return value, or Val::Nil if no returns.
pub fn call_function(lua: &mut rilua::Lua, func: Val, args: &[Val]) -> LuaResult<Val> {
    let Val::Function(func_ref) = func else {
        return Err(runtime_error("expected function"));
    };
    let func_handle = rilua::Function::from_gc_ref(func_ref);
    let results = lua.call_function(&func_handle, args)?;
    Ok(results.into_iter().next().unwrap_or(Val::Nil))
}

pub fn call_function_state(state: &mut LuaState, func: Val, args: &[Val]) -> LuaResult<Val> {
    let Val::Function(_) = func else {
        return Err(runtime_error("expected function"));
    };
    let func_idx = state.top;
    state.ensure_stack(func_idx + 1 + args.len());
    state.stack_set(func_idx, func);
    state.top = func_idx + 1;

    for arg in args {
        let top = state.top;
        state.stack_set(top, *arg);
        state.top = top + 1;
    }

    let save_base = state.base;
    state.base = func_idx + 1;

    let result = match state.precall(func_idx, LUA_MULTRET)? {
        CallResult::Lua => execute(state),
        CallResult::Rust => Ok(()),
    };

    let first = if result.is_ok() && state.top > func_idx {
        state.stack_get(func_idx)
    } else {
        Val::Nil
    };

    state.top = func_idx;
    state.base = save_base;
    result?;
    Ok(first)
}

/// Call a Lua function with error handling (pcall semantics).
///
/// Returns the results on success, or logs the error and returns an empty vec on failure.
pub fn pcall_function(lua: &mut rilua::Lua, func: Val, args: &[Val]) -> Vec<Val> {
    let Val::Function(func_ref) = func else {
        return vec![];
    };
    let func_handle = rilua::Function::from_gc_ref(func_ref);
    match lua.call_function(&func_handle, args) {
        Ok(results) => results,
        Err(e) => {
            super::rilua_script_helpers::call_error_handler(lua, &e.to_string());
            vec![]
        }
    }
}

// ── Registry value storage ──────────────────────────────────────────

/// Get a named value from rilua's registry table.
///
/// Equivalent to mlua's `lua.named_registry_value(key)`.
pub fn registry_get(state: &mut LuaState, key: &str) -> Val {
    let key_ref = state.gc.intern_string(key.as_bytes());
    state
        .gc
        .tables
        .get(state.registry)
        .map(|reg| reg.get_str(key_ref, &state.gc.string_arena))
        .unwrap_or(Val::Nil)
}

/// Set a named value in rilua's registry table.
///
/// Equivalent to mlua's `lua.set_named_registry_value(key, value)`.
pub fn registry_set(state: &mut LuaState, key: &str, value: Val) {
    let key_ref = state.gc.intern_string(key.as_bytes());
    if let Some(reg) = state.gc.tables.get_mut(state.registry) {
        let _ = reg.raw_set(Val::Str(key_ref), value, &state.gc.string_arena);
    }
}

/// Get or create a named table in rilua's registry.
///
/// Equivalent to mlua's pattern: `lua.named_registry_value(key).unwrap_or_else(|| { create + set })`.
pub fn registry_table_or_create(state: &mut LuaState, key: &str) -> Val {
    let existing = registry_get(state, key);
    if let Val::Table(_) = existing {
        return existing;
    }
    let table = Val::Table(state.gc.alloc_table(Table::new()));
    registry_set(state, key, table);
    table
}

#[cfg(test)]
mod tests {
    use super::{borrow_state, frame_ref};
    use crate::lua_api::WowLuaEnv;
    use rilua::LuaApiMut;

    #[test]
    fn frame_ref_returns_same_table_for_same_widget_id() {
        let env = WowLuaEnv::new().expect("env");
        let mut lua = env.rilua_mut();
        let state = lua.state_mut();
        let ui_parent_id = borrow_state(state)
            .expect("borrow state")
            .widgets
            .get_id_by_name("UIParent")
            .expect("UIParent");

        let first = frame_ref(state, ui_parent_id).expect("first frame ref");
        let second = frame_ref(state, ui_parent_id).expect("second frame ref");

        assert_eq!(first, second, "frame_ref should reuse cached table refs");
    }
}

// ── Table creation ──────────────────────────────────────────────────

/// Create a new empty table in rilua's GC heap.
pub fn create_table(state: &mut LuaState) -> Val {
    Val::Table(state.gc.alloc_table(Table::new()))
}

/// Create a new table with pre-allocated hash capacity.
/// `hash_capacity` is rounded up to the next power of 2.
pub fn create_table_with_capacity(state: &mut LuaState, hash_capacity: usize) -> Val {
    Val::Table(state.gc.alloc_table(Table::with_sizes(0, hash_capacity)))
}

/// Create a new table and set string-keyed fields on it.
/// Pre-sizes the hash part to avoid rehashing.
pub fn create_table_with_fields(state: &mut LuaState, fields: &[(&str, Val)]) -> Val {
    let table_ref = state.gc.alloc_table(Table::with_sizes(0, fields.len()));
    for &(key, value) in fields {
        let key_ref = state.gc.intern_string(key.as_bytes());
        if let Some(t) = state.gc.tables.get_mut(table_ref) {
            let _ = t.raw_set(Val::Str(key_ref), value, &state.gc.string_arena);
        }
    }
    Val::Table(table_ref)
}

/// Set a string key on an existing table Val.
pub fn table_set(state: &mut LuaState, table: Val, key: &str, value: Val) {
    let Val::Table(table_ref) = table else { return };
    let key_ref = state.gc.intern_string(key.as_bytes());
    if let Some(t) = state.gc.tables.get_mut(table_ref) {
        let _ = t.raw_set(Val::Str(key_ref), value, &state.gc.string_arena);
    }
}

/// Get a string key from a table Val.
pub fn table_get(state: &mut LuaState, table: Val, key: &str) -> Val {
    let Val::Table(table_ref) = table else {
        return Val::Nil;
    };
    let key_ref = state.gc.intern_string(key.as_bytes());
    state
        .gc
        .tables
        .get(table_ref)
        .map(|t| t.get_str(key_ref, &state.gc.string_arena))
        .unwrap_or(Val::Nil)
}
