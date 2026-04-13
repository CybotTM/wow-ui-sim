//! Helper functions for script table access and error handling.
//!
//! All internal tables (__scripts, __script_hooks, __frame_fields) are stored
//! in the Lua registry, invisible to addon Lua code.

use crate::lua_api::SimState;
use mlua::{Lua, Value};
use std::cell::RefCell;
use std::rc::Rc;

// ── __scripts table ──────────────────────────────────────────────────

const SCRIPTS_KEY: &str = "__scripts";
const FRAME_FIELDS_KEY: &str = "__frame_fields";
const FRAME_UNIT_EVENT_CALLBACKS_KEY: &str = "__frame_unit_event_callbacks";
const ON_UPDATE_SCRIPTS_KEY: &str = "__on_update_scripts";
const ON_POST_UPDATE_SCRIPTS_KEY: &str = "__on_post_update_scripts";

/// Get the __scripts table from the Lua registry. Returns None if not yet created.
pub fn get_scripts_table(lua: &Lua) -> Option<mlua::Table> {
    lua.named_registry_value(SCRIPTS_KEY).ok()
}

/// Get or create the __scripts table in the Lua registry.
pub fn get_or_create_scripts_table(lua: &Lua) -> mlua::Table {
    lua.named_registry_value(SCRIPTS_KEY).unwrap_or_else(|_| {
        let t = lua.create_table().unwrap();
        lua.set_named_registry_value(SCRIPTS_KEY, t.clone())
            .unwrap();
        t
    })
}

/// Get the script handler for a given frame + handler name.
pub fn get_script(lua: &Lua, widget_id: u64, handler_name: &str) -> Option<mlua::Function> {
    let table = get_scripts_table(lua)?;
    let key = format!("{}_{}", widget_id, handler_name);
    table.get(key.as_str()).ok()
}

/// Set a script handler for a given frame + handler name.
pub fn set_script(lua: &Lua, widget_id: u64, handler_name: &str, func: mlua::Function) {
    let table = get_or_create_scripts_table(lua);
    let key = format!("{}_{}", widget_id, handler_name);
    table.set(key.as_str(), func.clone()).ok();
    sync_on_update_script_cache(lua, widget_id, handler_name, Value::Function(func));
}

/// Remove a script handler for a given frame + handler name.
pub fn remove_script(lua: &Lua, widget_id: u64, handler_name: &str) {
    if let Some(table) = get_scripts_table(lua) {
        let key = format!("{}_{}", widget_id, handler_name);
        table.set(key.as_str(), Value::Nil).ok();
    }
    sync_on_update_script_cache(lua, widget_id, handler_name, Value::Nil);
}

pub fn clear_on_update_script_caches(lua: &Lua, widget_id: u64) {
    clear_registry_hot_script(lua, ON_UPDATE_SCRIPTS_KEY, widget_id);
    clear_registry_hot_script(lua, ON_POST_UPDATE_SCRIPTS_KEY, widget_id);
}

fn sync_on_update_script_cache(lua: &Lua, widget_id: u64, handler_name: &str, value: Value) {
    let Some(cache_key) = on_update_script_cache_key(handler_name) else {
        return;
    };
    let table = get_or_create_registry_table(lua, cache_key);
    table.raw_set(widget_id as i64, value).ok();
}

fn clear_registry_hot_script(lua: &Lua, key: &str, widget_id: u64) {
    if let Ok(table) = lua.named_registry_value::<mlua::Table>(key) {
        table.raw_set(widget_id as i64, Value::Nil).ok();
    }
}

fn on_update_script_cache_key(handler_name: &str) -> Option<&'static str> {
    match handler_name {
        "OnUpdate" => Some(ON_UPDATE_SCRIPTS_KEY),
        "OnPostUpdate" => Some(ON_POST_UPDATE_SCRIPTS_KEY),
        _ => None,
    }
}

fn get_or_create_registry_table(lua: &Lua, key: &str) -> mlua::Table {
    lua.named_registry_value(key).unwrap_or_else(|_| {
        let table = lua.create_table().unwrap();
        lua.set_named_registry_value(key, table.clone()).unwrap();
        table
    })
}

// ── __frame_fields table ─────────────────────────────────────────────

/// Get the __frame_fields table from the Lua registry. Returns None if not yet created.
pub fn get_frame_fields_table(lua: &Lua) -> Option<mlua::Table> {
    lua.named_registry_value(FRAME_FIELDS_KEY).ok()
}

/// Get or create the __frame_fields table in the Lua registry.
pub fn get_or_create_frame_fields_table(lua: &Lua) -> mlua::Table {
    lua.named_registry_value(FRAME_FIELDS_KEY)
        .unwrap_or_else(|_| {
            let t = lua.create_table().unwrap();
            lua.set_named_registry_value(FRAME_FIELDS_KEY, t.clone())
                .unwrap();
            t
        })
}

/// Get or create a per-frame fields sub-table within __frame_fields.
pub fn get_or_create_frame_fields(lua: &Lua, frame_id: u64) -> mlua::Table {
    let fields_table = get_or_create_frame_fields_table(lua);
    fields_table
        .get::<mlua::Table>(frame_id)
        .unwrap_or_else(|_| {
            let t = lua.create_table().unwrap();
            fields_table.set(frame_id, t.clone()).unwrap();
            t
        })
}

fn get_or_create_unit_event_callback_table(lua: &Lua) -> mlua::Table {
    lua.named_registry_value(FRAME_UNIT_EVENT_CALLBACKS_KEY)
        .unwrap_or_else(|_| {
            let t = lua.create_table().unwrap();
            lua.set_named_registry_value(FRAME_UNIT_EVENT_CALLBACKS_KEY, t.clone())
                .unwrap();
            t
        })
}

pub fn add_frame_unit_event_callback(
    lua: &Lua,
    frame_id: u64,
    event: &str,
    callback: mlua::Function,
    units: &[String],
) -> mlua::Result<()> {
    let callback_table = get_or_create_unit_event_callback_table(lua);
    let frame_callbacks = callback_table
        .get::<mlua::Table>(frame_id)
        .unwrap_or_else(|_| {
            let t = lua.create_table().unwrap();
            callback_table.set(frame_id, t.clone()).unwrap();
            t
        });
    let event_callbacks = frame_callbacks
        .get::<mlua::Table>(event)
        .unwrap_or_else(|_| {
            let t = lua.create_table().unwrap();
            frame_callbacks.set(event, t.clone()).unwrap();
            t
        });

    let entry = lua.create_table()?;
    entry.set("callback", callback)?;
    entry.set("units", unit_event_callback_units(lua, units)?)?;

    let next_index = event_callbacks.raw_len() + 1;
    event_callbacks.raw_set(next_index, entry)
}

pub fn dispatch_frame_unit_event_callbacks(
    lua: &Lua,
    frame_id: u64,
    owner: Value,
    event_args: &[Value],
    event_name: &str,
) -> mlua::Result<()> {
    let callback_table =
        match lua.named_registry_value::<mlua::Table>(FRAME_UNIT_EVENT_CALLBACKS_KEY) {
            Ok(table) => table,
            Err(_) => return Ok(()),
        };
    let frame_callbacks = match callback_table.get::<mlua::Table>(frame_id) {
        Ok(table) => table,
        Err(_) => return Ok(()),
    };
    let event_callbacks = match frame_callbacks.get::<mlua::Table>(event_name) {
        Ok(table) => table,
        Err(_) => return Ok(()),
    };

    let n = event_callbacks.raw_len();
    for i in 1..=n {
        let entry = match event_callbacks.raw_get::<mlua::Table>(i as i64) {
            Ok(table) => table,
            Err(_) => continue,
        };
        if !unit_event_callback_matches(&entry, event_args) {
            continue;
        }

        let callback: mlua::Function = match entry.get("callback") {
            Ok(callback) => callback,
            Err(_) => continue,
        };
        let mut call_args = vec![owner.clone()];
        call_args.extend(event_args.iter().cloned());
        if let Err(e) = callback.call::<()>(mlua::MultiValue::from_vec(call_args)) {
            call_error_handler(lua, &e.to_string());
        }
    }

    Ok(())
}

fn unit_event_callback_units(lua: &Lua, units: &[String]) -> mlua::Result<Value> {
    if units.is_empty() {
        return Ok(Value::Nil);
    }

    let unit_table = lua.create_table()?;
    for (index, unit) in units.iter().enumerate() {
        unit_table.raw_set((index + 1) as i64, unit.as_str())?;
    }
    Ok(Value::Table(unit_table))
}

fn unit_event_callback_matches(entry: &mlua::Table, event_args: &[Value]) -> bool {
    let units = match entry.get::<Value>("units") {
        Ok(Value::Table(units)) => units,
        Ok(Value::Nil) | Err(_) => return true,
        _ => return false,
    };

    let Some(Value::String(unit_value)) = event_args.first() else {
        return false;
    };
    let Ok(event_unit) = unit_value.to_str() else {
        return false;
    };

    let n = units.raw_len();
    for index in 1..=n {
        let Ok(registered_unit) = units.raw_get::<String>(index as i64) else {
            continue;
        };
        if event_unit == registered_unit {
            return true;
        }
    }
    false
}

// ── Frame reference ──────────────────────────────────────────────────

/// Get the UserData Value for a given widget ID (cached FrameRef).
pub fn get_frame_ref(lua: &Lua, widget_id: u64) -> Option<Value> {
    super::frame::frame_ref(lua, widget_id).ok()
}

// ── Error handler ────────────────────────────────────────────────────

/// Call the WoW error handler (set via `seterrorhandler`) and always log to stderr.
///
/// Uses Elune's `geterrorhandler()` which reads `LUA_ERRORHANDLERINDEX` (-9999),
/// the same slot that `securecall`'s `lua_pcall` references.
pub fn call_error_handler(lua: &Lua, error_msg: &str) {
    if collect_lua_error(lua, error_msg) {
        eprintln!("Lua error: {error_msg}");
    }
    let result: mlua::Result<()> = lua
        .load(
            r#"
        local handler = geterrorhandler()
        if handler then handler((...)) end
    "#,
        )
        .call(error_msg.to_string());
    if let Err(e) = result {
        eprintln!("Error in error handler: {e}");
    }
}

/// Get the calling addon name by walking the Lua stack from Rust.
///
/// Walks the stack looking for a source path containing "AddOns/<name>".
/// Falls back to `debug.getstacktaint()` if no addon source is found.
/// Uses mlua's `inspect_stack` (pure Rust) for speed — avoids compiling
/// and running a Lua chunk on each call.
pub fn get_stack_taint(lua: &Lua) -> Option<String> {
    // Fast path: walk the stack from Rust
    for level in 2..30usize {
        let debug = match lua.inspect_stack(level) {
            Some(d) => d,
            None => break,
        };
        let src = debug.source();
        if let Some(source_str) = src.source {
            let s = source_str.as_ref();
            if let Some(pos) = s.find("AddOns/") {
                let rest = &s[pos + 7..];
                if let Some(end) = rest.find('/') {
                    return Some(rest[..end].to_string());
                }
            }
        }
    }
    // Fallback to Elune's taint tracking
    let fallback: mlua::Function = lua
        .named_registry_value("__get_stack_taint_fallback")
        .ok()?;
    fallback.call(()).ok()
}

fn current_error_addon_name(lua: &Lua) -> Option<String> {
    let from_state = lua
        .app_data_ref::<Rc<RefCell<SimState>>>()
        .and_then(|state_rc| {
            state_rc.try_borrow().ok().and_then(|state| {
                state
                    .executing_addon_index
                    .or(state.loading_addon_index)
                    .and_then(|idx| state.addons.get(idx as usize))
                    .map(|addon| addon.folder_name.clone())
            })
        });

    from_state.or_else(|| get_stack_taint(lua))
}

/// Push an error message into SimState.lua_errors for later retrieval.
pub fn collect_lua_error(lua: &Lua, msg: &str) -> bool {
    let addon_name = current_error_addon_name(lua);
    if let Some(state_rc) = lua.app_data_ref::<Rc<RefCell<SimState>>>() {
        if let Ok(mut state) = state_rc.try_borrow_mut() {
            state.lua_errors.push(msg.to_string());
            state
                .lua_error_records
                .push(crate::lua_api::state::LuaErrorRecord {
                    message: msg.to_string(),
                    addon_name,
                });
            let normalized = crate::lua_errors::extract_error_message(msg);
            let entry = state.lua_error_counts.entry(normalized).or_insert(0);
            let is_first = *entry == 0;
            *entry += 1;
            return is_first;
        }
    }
    false
}

// ── Event dispatch ordering ───────────────────────────────────────────

/// Get event listeners in Lua hash table order (matching WoW's dispatch behaviour).
///
/// Returns event listeners in WoW dispatch order: individual-event registrations
/// first, then all-events registrations. Both use hlist ordering (insertion order
/// with swap-remove on unregister). Duplicates are skipped.
pub fn get_event_listeners_lua_order(lua: &Lua, event: &str) -> mlua::Result<Vec<u64>> {
    let mut result = Vec::new();
    let mut individual_ids = std::collections::HashSet::new();

    let individual: mlua::Table = lua.named_registry_value("__event_individual")?;
    if let Ok(event_tbl) = individual.get::<mlua::Table>(event) {
        let n = event_tbl.raw_len();
        for i in 1..=n {
            if let Ok(id) = event_tbl.raw_get::<u64>(i as i64) {
                result.push(id);
                individual_ids.insert(id);
            }
        }
    }

    let all_events: mlua::Table = lua.named_registry_value("__event_all")?;
    let n = all_events.raw_len();
    for i in 1..=n {
        if let Ok(id) = all_events.raw_get::<u64>(i as i64) {
            if !individual_ids.contains(&id) {
                result.push(id);
            }
        }
    }

    Ok(result)
}

/// A Lua-style error with no prefix. When caught by pcall, shows just the message.
/// Uses ExternalError so Display outputs the raw message without "runtime error: " prefix.
#[derive(Debug)]
pub struct LuaApiError(pub String);
impl std::fmt::Display for LuaApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::error::Error for LuaApiError {}

/// Create a Lua API error that pcall catches as just the message string.
pub fn lua_error(_lua: &Lua, msg: impl Into<String>) -> mlua::Error {
    lua_error_val(msg)
}

/// Same as `lua_error` but without requiring a Lua reference.
pub fn lua_error_val(msg: impl Into<String>) -> mlua::Error {
    mlua::Error::external(LuaApiError(msg.into()))
}
