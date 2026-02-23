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
    table.set(key.as_str(), func).ok();
}

/// Remove a script handler for a given frame + handler name.
pub fn remove_script(lua: &Lua, widget_id: u64, handler_name: &str) {
    if let Some(table) = get_scripts_table(lua) {
        let key = format!("{}_{}", widget_id, handler_name);
        table.set(key.as_str(), Value::Nil).ok();
    }
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

// ── Frame reference ──────────────────────────────────────────────────

/// Get the LightUserData value for a given widget ID.
///
/// With LightUserData, this is a trivial pointer construction — no global
/// lookup, no allocation. Always returns Some.
pub fn get_frame_ref(_lua: &Lua, widget_id: u64) -> Option<Value> {
    Some(super::frame::frame_lud(widget_id))
}

// ── Error handler ────────────────────────────────────────────────────

/// Call the WoW error handler (set via `seterrorhandler`) and always log to stderr.
///
/// Uses Elune's `geterrorhandler()` which reads `LUA_ERRORHANDLERINDEX` (-9999),
/// the same slot that `securecall`'s `lua_pcall` references.
pub fn call_error_handler(lua: &Lua, error_msg: &str) {
    eprintln!("Lua error: {error_msg}");
    collect_lua_error(lua, error_msg);
    let result: mlua::Result<()> = lua.load(r#"
        local handler = geterrorhandler()
        if handler then handler((...)) end
    "#).call(error_msg.to_string());
    if let Err(e) = result {
        eprintln!("Error in error handler: {e}");
    }
}

/// Push an error message into SimState.lua_errors for later retrieval.
pub fn collect_lua_error(lua: &Lua, msg: &str) {
    if let Some(state_rc) = lua.app_data_ref::<Rc<RefCell<SimState>>>() {
        if let Ok(mut state) = state_rc.try_borrow_mut() {
            state.lua_errors.push(msg.to_string());
        }
    }
}

// ── Event dispatch ordering ───────────────────────────────────────────

/// Get event listeners in Lua hash table order (matching WoW's dispatch behaviour).
///
/// WoW stores registrations in Lua tables and iterates with `pairs()`, which
/// follows Lua 5.1 hash table order. This replicates that by reading the same
/// tables we maintain alongside the Rust-side `registered_events` HashSet.
///
/// Returns individual-event registrations first (in Lua hash order), then
/// all-events registrations (in Lua hash order), with duplicates skipped.
pub fn get_event_listeners_lua_order(lua: &Lua, event: &str) -> mlua::Result<Vec<u64>> {
    use mlua::{LightUserData, Value};
    let mut result = Vec::new();
    let mut individual_ids = std::collections::HashSet::new();

    let individual: mlua::Table = lua.named_registry_value("__event_individual")?;
    if let Ok(event_tbl) = individual.get::<mlua::Table>(event) {
        for pair in event_tbl.pairs::<LightUserData, Value>() {
            if let Ok((lud, _)) = pair {
                let id = lud.0 as u64;
                result.push(id);
                individual_ids.insert(id);
            }
        }
    }

    let all_events: mlua::Table = lua.named_registry_value("__event_all")?;
    for pair in all_events.pairs::<LightUserData, Value>() {
        if let Ok((lud, _)) = pair {
            let id = lud.0 as u64;
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
    mlua::Error::external(LuaApiError(msg.into()))
}

