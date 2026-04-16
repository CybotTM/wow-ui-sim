//! rilua-side script dispatch helpers for Phase 3 migration.
//!
//! Mirrors `script_helpers.rs` but operates on `rilua::Lua` / `LuaState`
//! instead of `mlua::Lua`. Script handlers are stored in registry tables
//! using the same key format (`{widget_id}_{handler_name}`).

use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaApi, LuaApiMut, Val};
use std::time::Instant;

use crate::lua_api::rilua_methods::{call_function_state, val_to_string};

// ── Registry helpers ────────────────────────────────────────────────

const SCRIPTS_KEY: &str = "__scripts";
const ON_UPDATE_SCRIPTS_KEY: &str = "__on_update_scripts";
const ON_POST_UPDATE_SCRIPTS_KEY: &str = "__on_post_update_scripts";
const ERROR_HANDLER_KEY: &str = "__error_handler";
const PROTECTED_LUA_PCALL_WRAPPER_FACTORY_KEY: &str = "__protected_lua_pcall_wrapper_factory";
const LUA_MULTRET: i32 = -1;

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
        let _ = reg.raw_set(
            Val::Str(key_ref),
            Val::Table(new_table),
            &state.gc.string_arena,
        );
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
        let _ = table.raw_set(Val::Num(widget_id as f64), value, &state.gc.string_arena);
    }
    sync_on_update_runtime_cache(state, widget_id);
}

// ── Error handler ───────────────────────────────────────────────────

/// Call the WoW error handler and log to stderr.
pub fn call_error_handler(lua: &mut rilua::Lua, error_msg: &str) {
    call_error_handler_state(lua.state_mut(), error_msg);
}

/// State-only variant for RustFn call sites that only hold `&mut LuaState`.
pub fn call_error_handler_state(state: &mut LuaState, error_msg: &str) {
    if collect_lua_error(state, error_msg) {
        eprintln!("Lua error: {error_msg}");
    }
    let Ok(handler) = ensure_error_handler(state) else {
        return;
    };
    let Val::Function(_) = handler else {
        return;
    };
    let msg_ref = state.gc.intern_string(error_msg.as_bytes());
    let _ = protected_call_state(state, handler, &[Val::Str(msg_ref)]);
}

pub fn protected_call_state(
    state: &mut LuaState,
    func: Val,
    args: &[Val],
) -> Result<Vec<Val>, Val> {
    let saved_top = state.top;
    let call_base = saved_top;
    let saved_ci = state.ci;
    let saved_n_ccalls = state.n_ccalls;
    let saved_call_depth = state.call_depth;
    state.error_object = None;

    state.ensure_stack(call_base + 1 + args.len());
    state.stack_set(call_base, func);
    for (index, arg) in args.iter().copied().enumerate() {
        state.stack_set(call_base + 1 + index, arg);
    }
    state.top = call_base + 1 + args.len();

    let call_result = state.call_function(call_base, LUA_MULTRET);
    match call_result {
        Ok(()) => {
            let results = (call_base..state.top)
                .map(|idx| state.stack_get(idx))
                .collect();
            state.top = saved_top;
            Ok(results)
        }
        Err(err) => {
            state.ci = saved_ci;
            state.base = state.call_stack[state.ci].base;
            state.n_ccalls = saved_n_ccalls;
            state.call_depth = saved_call_depth;
            if state.ci < rilua::vm::state::MAXCALLS {
                state.ci_overflow = false;
            }
            state.close_upvalues(call_base);
            let error_val = state.error_object.take().unwrap_or_else(|| {
                let r = state.gc.intern_string(err.to_string().as_bytes());
                Val::Str(r)
            });
            state.top = saved_top;
            Err(error_val)
        }
    }
}

/// Call a function through Lua's own `pcall` path.
///
/// Some handlers created from Blizzard XML work when called by Lua but trip
/// rilua's direct Rust-side call path with "expected Lua closure in execute".
/// Wrapping the call in a tiny Lua closure keeps dispatch on the VM's normal
/// path and returns either the function results or a formatted error string.
pub fn protected_lua_pcall_state(
    state: &mut LuaState,
    func: Val,
    args: &[Val],
) -> Result<Vec<Val>, String> {
    let wrapper_factory =
        ensure_protected_lua_pcall_wrapper_factory(state).map_err(|error| error.to_string())?;
    let wrapper = call_function_state(state, Val::Function(wrapper_factory.gc_ref()), &[func])
        .map_err(|error| error.to_string())?;
    let results = protected_call_state(state, wrapper, args).map_err(|error| {
        val_to_string(state, error)
            .unwrap_or_else(|| format!("script error ({})", error.type_name()))
    })?;
    match results.first().copied() {
        Some(Val::Bool(true)) => Ok(results.into_iter().skip(1).collect()),
        Some(Val::Bool(false)) => {
            let error = results
                .get(1)
                .copied()
                .and_then(|value| val_to_string(state, value))
                .unwrap_or_else(|| "script error".to_string());
            Err(error)
        }
        _ => Ok(results),
    }
}

fn ensure_protected_lua_pcall_wrapper_factory(state: &mut LuaState) -> rilua::LuaResult<rilua::Function> {
    let existing = registry_value(state, PROTECTED_LUA_PCALL_WRAPPER_FACTORY_KEY);
    if let Val::Function(func_ref) = existing {
        return Ok(rilua::Function::from_gc_ref(func_ref));
    }

    let factory = state.load(
        r#"
        local func = ...
        return function(...)
            return pcall(func, ...)
        end
    "#,
    )?;
    set_registry_value(
        state,
        PROTECTED_LUA_PCALL_WRAPPER_FACTORY_KEY,
        Val::Function(factory.gc_ref()),
    );
    Ok(factory)
}

fn sync_on_update_runtime_cache(state: &mut LuaState, widget_id: u64) {
    use super::env::WowLuaAppData;

    let has_on_update = cached_handler_present(state, ON_UPDATE_SCRIPTS_KEY, widget_id);
    let has_on_post_update = cached_handler_present(state, ON_POST_UPDATE_SCRIPTS_KEY, widget_id);
    let should_track = has_on_update || has_on_post_update;

    let Some(app) = state.app_data::<WowLuaAppData>() else {
        return;
    };
    let Ok(mut sim) = app.sim_state.try_borrow_mut() else {
        return;
    };

    if should_track {
        sim.on_update_frames.insert(widget_id);
    } else {
        sim.on_update_frames.remove(&widget_id);
    }
    sim.visible_on_update_cache = None;
}

fn cached_handler_present(state: &mut LuaState, cache_key: &str, widget_id: u64) -> bool {
    let Some(table_ref) = registry_table(state, cache_key) else {
        return false;
    };
    state
        .gc
        .tables
        .get(table_ref)
        .map(|table| !matches!(table.get_int(widget_id as i64), Val::Nil))
        .unwrap_or(false)
}

fn ensure_error_handler(state: &mut LuaState) -> rilua::LuaResult<Val> {
    let existing = registry_value(state, ERROR_HANDLER_KEY);
    if existing != Val::Nil {
        return Ok(existing);
    }

    let func = state.load("return function(_msg) end")?;
    let call_base = state.top;
    state.ensure_stack(call_base + 2);
    state.stack_set(call_base, Val::Function(func.gc_ref()));
    state.top = call_base + 1;
    state.call_function(call_base, 1)?;
    let handler = state.stack_get(call_base);
    state.top = call_base;
    set_registry_value(state, ERROR_HANDLER_KEY, handler);
    Ok(handler)
}

fn registry_value(state: &mut LuaState, key: &str) -> Val {
    let key_ref = state.gc.intern_string(key.as_bytes());
    state
        .gc
        .tables
        .get(state.registry)
        .map(|table| table.get_str(key_ref, &state.gc.string_arena))
        .unwrap_or(Val::Nil)
}

fn set_registry_value(state: &mut LuaState, key: &str, value: Val) {
    let key_ref = state.gc.intern_string(key.as_bytes());
    if let Some(table) = state.gc.tables.get_mut(state.registry) {
        let _ = table.raw_set(Val::Str(key_ref), value, &state.gc.string_arena);
    }
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
    let addon_name = sim
        .executing_addon_index
        .or(sim.loading_addon_index)
        .and_then(|idx| sim.addons.get(idx as usize))
        .map(|addon| addon.folder_name.clone());
    sim.lua_error_records
        .push(crate::lua_api::state::LuaErrorRecord {
            message: msg.to_string(),
            addon_name,
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
        let owner_addon = frame_owner_addon(lua.state(), frame_id);
        let func = rilua::Function::from_gc_ref(func_ref);
        let start = Instant::now();
        if let Err(e) = lua.call_function(&func, &[frame_val, elapsed_val]) {
            call_error_handler(lua, &e.to_string());
        }
        record_frame_timing(lua.state(), owner_addon, &start);
    }
    Ok(())
}

fn frame_owner_addon(state: &LuaState, frame_id: u64) -> Option<u16> {
    use super::env::WowLuaAppData;

    let app = state.app_data::<WowLuaAppData>()?;
    let sim = app.sim_state.try_borrow().ok()?;
    sim.widgets
        .get(frame_id)
        .and_then(|frame| frame.owner_addon)
}

fn record_frame_timing(state: &LuaState, owner_addon: Option<u16>, start: &Instant) {
    use super::env::WowLuaAppData;

    let Some(addon_idx) = owner_addon else {
        return;
    };
    let Some(app) = state.app_data::<WowLuaAppData>() else {
        return;
    };
    let Ok(mut sim) = app.sim_state.try_borrow_mut() else {
        return;
    };
    let Some(addon) = sim.addons.get_mut(addon_idx as usize) else {
        return;
    };
    addon.runtime.current_frame_ms += start.elapsed().as_secs_f64() * 1000.0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn protected_lua_pcall_state_caches_wrapper_factory() {
        let mut lua = rilua::Lua::new().expect("lua should initialize");
        let direct = lua
            .state_mut()
            .load("return function(value) return value end")
            .expect("wrapper source should compile");
        let direct_func = call_function_state(
            lua.state_mut(),
            Val::Function(direct.gc_ref()),
            &[],
        )
        .expect("direct function should build");

        let first_results = protected_lua_pcall_state(
            lua.state_mut(),
            direct_func,
            &[Val::Num(7.0)],
        )
        .expect("first protected call should succeed");
        assert_eq!(first_results, vec![Val::Num(7.0)]);

        let first_factory = registry_value(lua.state_mut(), "__protected_lua_pcall_wrapper_factory");
        assert!(
            !matches!(first_factory, Val::Nil),
            "wrapper factory should be cached in the registry"
        );

        let second_results = protected_lua_pcall_state(
            lua.state_mut(),
            direct_func,
            &[Val::Num(9.0)],
        )
        .expect("second protected call should succeed");
        assert_eq!(second_results, vec![Val::Num(9.0)]);

        let second_factory = registry_value(lua.state_mut(), "__protected_lua_pcall_wrapper_factory");
        assert_eq!(first_factory, second_factory, "cached wrapper factory should be reused");
    }
}
