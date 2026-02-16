//! Security-related WoW API functions.
//!
//! Contains securecallmethod (not in Elune), SecureHandler stubs,
//! state/attribute driver stubs, and SecureCmdOptionParse.
//!
//! Functions provided by Elune's baselib_shared (issecure, issecurevariable,
//! securecall, securecallfunction, forceinsecure, hooksecurefunc,
//! secureexecuterange) are NOT registered here — they come from the C runtime.

use mlua::{Lua, Result, Value};

/// Register all security-related API functions.
pub fn register_security_functions(lua: &Lua) -> Result<()> {
    let globals = lua.globals();

    // Elune provides: issecure, issecurevariable, securecall,
    // securecallfunction, forceinsecure, hooksecurefunc, secureexecuterange

    globals.set("securecallmethod", lua.create_function(securecallmethod_impl)?)?;

    globals.set("issecretvalue", lua.create_function(|_, _val: Value| Ok(false))?)?;
    globals.set("canaccessvalue", lua.create_function(|_, _val: Value| Ok(true))?)?;
    globals.set(
        "canaccessallvalues",
        lua.create_function(|_, _vals: mlua::MultiValue| Ok(true))?,
    )?;
    globals.set("canaccesstable", lua.create_function(|_, _val: Value| Ok(true))?)?;

    register_secure_handler_stubs(lua)?;

    // SecureCmdOptionParse - returns the default (last) option
    globals.set(
        "SecureCmdOptionParse",
        lua.create_function(|lua, options: String| {
            if let Some(last) = options.split(';').next_back() {
                Ok(Value::String(lua.create_string(last.trim())?))
            } else {
                Ok(Value::Nil)
            }
        })?,
    )?;

    Ok(())
}

/// securecallmethod(object, methodName, ...) → object:methodName(...)
fn securecallmethod_impl(_lua: &Lua, args: mlua::MultiValue) -> Result<mlua::MultiValue> {
    let mut it = args.into_iter();
    let obj = match it.next() {
        Some(Value::Table(t)) => t,
        _ => return Ok(mlua::MultiValue::new()),
    };
    let method_name = match it.next() {
        Some(Value::String(s)) => s,
        _ => return Ok(mlua::MultiValue::new()),
    };
    let remaining: Vec<Value> = it.collect();
    match obj.get::<Value>(method_name)? {
        Value::Function(f) => {
            let mut call_args = vec![Value::Table(obj)];
            call_args.extend(remaining);
            f.call::<mlua::MultiValue>(mlua::MultiValue::from_iter(call_args))
        }
        _ => Ok(mlua::MultiValue::new()),
    }
}

/// SecureHandler stubs and state/attribute driver stubs.
fn register_secure_handler_stubs(lua: &Lua) -> Result<()> {
    let globals = lua.globals();

    globals.set(
        "SecureHandlerSetFrameRef",
        lua.create_function(|_, (_frame, _name, _target): (Value, String, Value)| Ok(()))?,
    )?;
    globals.set(
        "SecureHandlerExecute",
        lua.create_function(|_, (_frame, _body, _args): (Value, String, mlua::MultiValue)| {
            Ok(())
        })?,
    )?;
    globals.set(
        "SecureHandlerWrapScript",
        lua.create_function(|_, (_frame, _script, _body): (Value, String, String)| Ok(()))?,
    )?;

    globals.set(
        "RegisterStateDriver",
        lua.create_function(|_, (_frame, _attr, _driver): (Value, String, String)| Ok(()))?,
    )?;
    globals.set(
        "UnregisterStateDriver",
        lua.create_function(|_, (_frame, _attr): (Value, String)| Ok(()))?,
    )?;
    globals.set(
        "RegisterAttributeDriver",
        lua.create_function(|_, (_frame, _attr, _driver): (Value, String, String)| Ok(()))?,
    )?;
    globals.set(
        "UnregisterAttributeDriver",
        lua.create_function(|_, (_frame, _attr): (Value, String)| Ok(()))?,
    )?;

    Ok(())
}
