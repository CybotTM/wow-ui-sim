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
const SCRIPT_HOOKS_KEY: &str = "__script_hooks";
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

// ── __script_hooks table ─────────────────────────────────────────────

/// Get or create the __script_hooks table in the Lua registry.
pub fn get_or_create_hooks_table(lua: &Lua) -> mlua::Table {
    lua.named_registry_value(SCRIPT_HOOKS_KEY)
        .unwrap_or_else(|_| {
            let t = lua.create_table().unwrap();
            lua.set_named_registry_value(SCRIPT_HOOKS_KEY, t.clone())
                .unwrap();
            t
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

