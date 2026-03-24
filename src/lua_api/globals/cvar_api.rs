//! CVar WoW API functions.
//!
//! Provides access to configuration variables (CVars).

use super::super::SimState;
use mlua::{Function, Lua, Result, Table, Value};
use std::cell::RefCell;
use std::rc::Rc;

fn parse_bitfield_mask(value: Option<String>) -> u128 {
    value
        .and_then(|v| v.trim().parse::<u128>().ok())
        .unwrap_or(0)
}

fn bitfield_bit(index: Option<i32>) -> Option<u128> {
    let index = u32::try_from(index?).ok()?;
    (index < 128).then(|| 1u128 << index)
}

fn get_cvar_value(lua: &Lua, state: &SimState, cvar: &str) -> Result<Value> {
    match state.cvars.get(cvar) {
        Some(value) => Ok(Value::String(lua.create_string(&value)?)),
        None => Ok(Value::Nil),
    }
}

fn get_cvar_default_value(lua: &Lua, state: &SimState, cvar: &str) -> Result<Value> {
    match state.cvars.get_default(cvar) {
        Some(value) => Ok(Value::String(lua.create_string(&value)?)),
        None => Ok(Value::Nil),
    }
}

fn get_cvar_bitfield(state: &SimState, name: &str, index: Option<i32>) -> bool {
    let Some(bit) = bitfield_bit(index) else {
        return false;
    };
    (parse_bitfield_mask(state.cvars.get(name)) & bit) != 0
}

fn set_cvar_bitfield(state: &SimState, name: &str, index: i32, value: bool) -> bool {
    let Some(bit) = bitfield_bit(Some(index)) else {
        return false;
    };
    let current = parse_bitfield_mask(state.cvars.get(name));
    let updated = if value { current | bit } else { current & !bit };
    state.cvars.set(name, &updated.to_string())
}

fn create_get_cvar(lua: &Lua, state: &Rc<RefCell<SimState>>) -> Result<Function> {
    let state = Rc::clone(state);
    lua.create_function(move |lua, cvar: String| {
        let state = state.borrow();
        get_cvar_value(lua, &state, &cvar)
    })
}

fn create_set_cvar(lua: &Lua, state: &Rc<RefCell<SimState>>) -> Result<Function> {
    let state = Rc::clone(state);
    lua.create_function(move |_, (cvar, value): (String, String)| {
        let state = state.borrow();
        Ok(state.cvars.set(&cvar, &value))
    })
}

fn create_get_cvar_bool(lua: &Lua, state: &Rc<RefCell<SimState>>) -> Result<Function> {
    let state = Rc::clone(state);
    lua.create_function(move |_, cvar: String| {
        let state = state.borrow();
        Ok(state.cvars.get_bool(&cvar))
    })
}

fn create_get_cvar_default(lua: &Lua, state: &Rc<RefCell<SimState>>) -> Result<Function> {
    let state = Rc::clone(state);
    lua.create_function(move |lua, cvar: String| {
        let state = state.borrow();
        get_cvar_default_value(lua, &state, &cvar)
    })
}

fn create_get_cvar_bitfield(lua: &Lua, state: &Rc<RefCell<SimState>>) -> Result<Function> {
    let state = Rc::clone(state);
    lua.create_function(move |_, (name, index): (String, Option<i32>)| {
        let state = state.borrow();
        Ok(get_cvar_bitfield(&state, &name, index))
    })
}

fn create_set_cvar_bitfield(lua: &Lua, state: &Rc<RefCell<SimState>>) -> Result<Function> {
    let state = Rc::clone(state);
    lua.create_function(
        move |_, (name, index, value, _script): (String, i32, bool, Option<String>)| {
            let state = state.borrow();
            Ok(set_cvar_bitfield(&state, &name, index, value))
        },
    )
}

fn register_bitfield_functions(
    lua: &Lua,
    table: &Table,
    state: &Rc<RefCell<SimState>>,
) -> Result<()> {
    table.set("GetCVarBitfield", create_get_cvar_bitfield(lua, state)?)?;
    table.set("SetCVarBitfield", create_set_cvar_bitfield(lua, state)?)?;
    Ok(())
}

fn register_c_cvar_namespace(lua: &Lua, state: &Rc<RefCell<SimState>>) -> Result<()> {
    let table = lua.create_table()?;
    table.set("GetCVar", create_get_cvar(lua, state)?)?;
    table.set("SetCVar", create_optional_set_cvar(lua, state)?)?;
    table.set("GetCVarBool", create_get_cvar_bool(lua, state)?)?;
    table.set("GetCVarDefault", create_get_cvar_default(lua, state)?)?;
    register_bitfield_functions(lua, &table, state)?;
    register_c_cvar_helpers(lua, &table)?;
    lua.globals().set("C_CVar", table)?;
    Ok(())
}

fn create_optional_set_cvar(lua: &Lua, state: &Rc<RefCell<SimState>>) -> Result<Function> {
    let state = Rc::clone(state);
    lua.create_function(move |_, (cvar, value): (String, Option<String>)| {
        let state = state.borrow();
        state.cvars.set(&cvar, value.as_deref().unwrap_or(""));
        Ok(true)
    })
}

fn register_c_cvar_helpers(lua: &Lua, table: &Table) -> Result<()> {
    table.set(
        "RegisterCVar",
        lua.create_function(|_, (_name, _value): (String, Option<String>)| Ok(()))?,
    )?;
    table.set("ResetTestCVars", lua.create_function(|_, ()| Ok(()))?)?;
    Ok(())
}

fn register_cvar_functions(lua: &Lua, state: &Rc<RefCell<SimState>>) -> Result<()> {
    let globals = lua.globals();
    globals.set("GetCVar", create_get_cvar(lua, state)?)?;
    globals.set("SetCVar", create_set_cvar(lua, state)?)?;
    register_bitfield_functions(lua, &globals, state)?;
    globals.set(
        "ConsoleGetAllCommands",
        create_console_get_all_commands(lua, state)?,
    )?;
    Ok(())
}

fn create_console_get_all_commands(lua: &Lua, state: &Rc<RefCell<SimState>>) -> Result<Function> {
    let state = Rc::clone(state);
    lua.create_function(move |lua, ()| {
        let state = state.borrow();
        let keys = state.cvars.all_keys();
        let result = lua.create_table_with_capacity(keys.len(), 0)?;
        for (i, key) in keys.iter().enumerate() {
            let entry = lua.create_table_with_capacity(0, 1)?;
            entry.set("command", key.as_str())?;
            result.set(i + 1, entry)?;
        }
        Ok(result)
    })
}

/// Register CVar global functions.
pub fn register_cvar_api(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    register_cvar_functions(lua, &state)?;
    register_c_cvar_namespace(lua, &state)?;
    Ok(())
}
