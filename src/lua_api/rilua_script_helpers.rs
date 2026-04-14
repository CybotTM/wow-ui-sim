//! rilua-side script dispatch helpers for Phase 3 migration.
//!
//! Mirrors `script_helpers.rs` but operates on `rilua::Lua` / `LuaState`
//! instead of `mlua::Lua`. Script handlers are stored in registry tables
//! using the same key format (`{widget_id}_{handler_name}`).

use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaApiMut, Val};

// ── Registry helpers ────────────────────────────────────────────────

const SCRIPTS_KEY: &str = "__scripts";
const ON_UPDATE_SCRIPTS_KEY: &str = "__on_update_scripts";
const ON_POST_UPDATE_SCRIPTS_KEY: &str = "__on_post_update_scripts";

/// Get a named table from rilua's registry, returning None if absent.
fn registry_table(state: &mut LuaState, key: &str) -> Option<GcRef<Table>> {
    let key_ref = state.gc.intern_string(key.as_bytes());
    let registry = state.gc.tables.get(state.registry)?;
    match registry.get_str(key_ref, &state.gc.string_arena) {
        Val::Table(t) => Some(t),
        _ => None,
    }
}

/// Get or create a named table in rilua's registry.
fn registry_table_or_create(state: &mut LuaState, key: &str) -> GcRef<Table> {
    if let Some(existing) = registry_table(state, key) {
        return existing;
    }
    let new_table = state.gc.alloc_table(Table::new());
    let key_ref = state.gc.intern_string(key.as_bytes());
    if let Some(reg) = state.gc.tables.get_mut(state.registry) {
        let _ = reg.raw_set(Val::Str(key_ref), Val::Table(new_table), &state.gc.string_arena);
    }
    new_table
}

/// Set a string-keyed value in a table.
fn table_set_str(state: &mut LuaState, table: GcRef<Table>, key: &str, value: Val) {
    let key_ref = state.gc.intern_string(key.as_bytes());
    if let Some(t) = state.gc.tables.get_mut(table) {
        let _ = t.raw_set(Val::Str(key_ref), value, &state.gc.string_arena);
    }
}

/// Get a string-keyed value from a table.
fn table_get_str(state: &mut LuaState, table: GcRef<Table>, key: &str) -> Val {
    let key_ref = state.gc.intern_string(key.as_bytes());
    state
        .gc
        .tables
        .get(table)
        .map(|t| t.get_str(key_ref, &state.gc.string_arena))
        .unwrap_or(Val::Nil)
}

// ── Script storage ──────────────────────────────────────────────────

/// Get a script handler for a given frame + handler name.
pub fn get_script(state: &mut LuaState, widget_id: u64, handler_name: &str) -> Option<Val> {
    let scripts = registry_table(state, SCRIPTS_KEY)?;
    let key = format!("{}_{}", widget_id, handler_name);
    match table_get_str(state, scripts, &key) {
        Val::Nil => None,
        val => Some(val),
    }
}

/// Set a script handler for a given frame + handler name.
pub fn set_script(state: &mut LuaState, widget_id: u64, handler_name: &str, func: Val) {
    let scripts = registry_table_or_create(state, SCRIPTS_KEY);
    let key = format!("{}_{}", widget_id, handler_name);
    table_set_str(state, scripts, &key, func);
    sync_on_update_cache(state, widget_id, handler_name, func);
}

/// Remove a script handler.
pub fn remove_script(state: &mut LuaState, widget_id: u64, handler_name: &str) {
    if let Some(scripts) = registry_table(state, SCRIPTS_KEY) {
        let key = format!("{}_{}", widget_id, handler_name);
        table_set_str(state, scripts, &key, Val::Nil);
    }
    sync_on_update_cache(state, widget_id, handler_name, Val::Nil);
}

fn sync_on_update_cache(state: &mut LuaState, widget_id: u64, handler_name: &str, value: Val) {
    let cache_key = match handler_name {
        "OnUpdate" => ON_UPDATE_SCRIPTS_KEY,
        "OnPostUpdate" => ON_POST_UPDATE_SCRIPTS_KEY,
        _ => return,
    };
    let table_ref = registry_table_or_create(state, cache_key);
    if let Some(table) = state.gc.tables.get_mut(table_ref) {
        let _ = table.raw_set(
            Val::Num(widget_id as f64),
            value,
            &state.gc.string_arena,
        );
    }
}

// ── Error handler ───────────────────────────────────────────────────

/// Call the WoW error handler and log to stderr.
pub fn call_error_handler(lua: &mut rilua::Lua, error_msg: &str) {
    eprintln!("Lua error: {error_msg}");
    let handler_code = r#"
        local handler = geterrorhandler()
        if handler then handler((...)) end
    "#;
    let Ok(func) = lua.load(handler_code) else {
        return;
    };
    let msg_ref = lua.state_mut().gc.intern_string(error_msg.as_bytes());
    let _ = lua.call_function(&func, &[Val::Str(msg_ref)]);
}

/// Collect a Lua error into SimState for later retrieval.
pub fn collect_lua_error(state: &LuaState, msg: &str) -> bool {
    use super::env::WowLuaAppData;
    let Some(app) = state.app_data::<WowLuaAppData>() else {
        return false;
    };
    let Ok(mut sim) = app.sim_state.try_borrow_mut() else {
        return false;
    };
    sim.lua_errors.push(msg.to_string());
    sim.lua_error_records
        .push(crate::lua_api::state::LuaErrorRecord {
            message: msg.to_string(),
            addon_name: None,
        });
    let normalized = crate::lua_errors::extract_error_message(msg);
    let entry = sim.lua_error_counts.entry(normalized).or_insert(0);
    let is_first = *entry == 0;
    *entry += 1;
    is_first
}

// ── Event dispatch ordering ─────────────────────────────────────────

/// Get event listeners in registration order from rilua's registry.
pub fn get_event_listeners(state: &mut LuaState, event: &str) -> Vec<u64> {
    let mut result = Vec::new();
    let mut seen = std::collections::HashSet::new();
    collect_individual_listeners(state, event, &mut result, &mut seen);
    collect_all_event_listeners(state, &mut result, &seen);
    result
}

fn collect_individual_listeners(
    state: &mut LuaState,
    event: &str,
    result: &mut Vec<u64>,
    seen: &mut std::collections::HashSet<u64>,
) {
    let event_tbl = resolve_event_subtable(state, "__event_individual", event);
    let Some(tbl) = event_tbl.and_then(|r| state.gc.tables.get(r)) else {
        return;
    };
    let slice = tbl.array_slice();
    for val in slice {
        if let Val::Num(id) = val {
            let id = *id as u64;
            result.push(id);
            seen.insert(id);
        }
    }
}

fn collect_all_event_listeners(
    state: &mut LuaState,
    result: &mut Vec<u64>,
    seen: &std::collections::HashSet<u64>,
) {
    let Some(all_ref) = registry_table(state, "__event_all") else {
        return;
    };
    let Some(all) = state.gc.tables.get(all_ref) else {
        return;
    };
    let slice = all.array_slice();
    for val in slice {
        if let Val::Num(id) = val {
            let id = *id as u64;
            if !seen.contains(&id) {
                result.push(id);
            }
        }
    }
}

fn resolve_event_subtable(
    state: &mut LuaState,
    registry_key: &str,
    event: &str,
) -> Option<GcRef<Table>> {
    let container_ref = registry_table(state, registry_key)?;
    match table_get_str(state, container_ref, event) {
        Val::Table(t) => Some(t),
        _ => None,
    }
}

// ── Script dispatch ─────────────────────────────────────────────────

/// Dispatch a script handler for a frame via rilua.
///
/// Looks up the script handler in the rilua registry and calls it with the
/// frame value as the first argument, followed by any additional args.
///
/// This is the rilua equivalent of calling `get_script` + `handler.call(frame_val)`.
pub fn dispatch_script(
    lua: &mut rilua::Lua,
    widget_id: u64,
    handler_name: &str,
    extra_args: &[Val],
) -> rilua::LuaResult<()> {
    let handler = {
        let state = lua.state_mut();
        get_script(state, widget_id, handler_name)
    };
    let Some(handler_val) = handler else {
        return Ok(());
    };

    // Build args: frame_ref as first arg, then extra_args
    let frame_val = {
        let state = lua.state_mut();
        crate::lua_api::rilua_methods::frame_ref(state, widget_id)?
    };
    let mut args = vec![frame_val];
    args.extend_from_slice(extra_args);

    let Val::Function(func_ref) = handler_val else {
        return Ok(());
    };
    let func = rilua::Function::from_gc_ref(func_ref);
    match lua.call_function(&func, &args) {
        Ok(_) => Ok(()),
        Err(e) => {
            call_error_handler(lua, &e.to_string());
            Ok(())
        }
    }
}

// ── OnUpdate dispatch ───────────────────────────────────────────────

/// Dispatch OnUpdate handlers for visible frames via rilua.
///
/// This is the rilua equivalent of `on_update::fire`. It iterates the
/// `__on_update_scripts` registry table and calls each handler with
/// `(frame, elapsed)` arguments.
///
/// Callers should pause GC before calling this and step GC after.
pub fn dispatch_on_update(
    lua: &mut rilua::Lua,
    frame_ids: &[u64],
    elapsed: f64,
) -> rilua::LuaResult<()> {
    let elapsed_val = Val::Num(elapsed);
    for &frame_id in frame_ids {
        let handler = {
            let state = lua.state_mut();
            get_script(state, frame_id, "OnUpdate")
        };
        let Some(handler_val) = handler else {
            continue;
        };
        let frame_val = {
            let state = lua.state_mut();
            crate::lua_api::rilua_methods::frame_ref(state, frame_id)?
        };
        let Val::Function(func_ref) = handler_val else {
            continue;
        };
        let func = rilua::Function::from_gc_ref(func_ref);
        if let Err(e) = lua.call_function(&func, &[frame_val, elapsed_val]) {
            call_error_handler(lua, &e.to_string());
        }
    }
    Ok(())
}
