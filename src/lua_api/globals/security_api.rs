//! Security-related WoW API functions.
//!
//! Contains securecallmethod, SecureHandler stubs, state/attribute driver stubs,
//! and SecureCmdOptionParse. Taint functions (issecure, issecurevariable, etc.)
//! are provided by data/lua/taint_stubs.lua loaded at startup.

use mlua::{Function, Lua, MultiValue, Result, Value};

/// Register all security-related API functions.
pub fn register_security_functions(lua: &Lua) -> Result<()> {
    register_securecallmethod(lua)?;
    register_secure_handler_stubs(lua)?;
    register_state_driver_stubs(lua)?;
    register_secure_cmd_option_parse(lua)?;
    Ok(())
}

fn register_securecallmethod(lua: &Lua) -> Result<()> {
    lua.globals()
        .set("securecallmethod", make_securecallmethod(lua)?)?;
    Ok(())
}

/// Build the securecallmethod closure: calls obj[name](obj, ...) via securecall.
fn make_securecallmethod(lua: &Lua) -> Result<Function> {
    lua.create_function(|lua, args: MultiValue| {
        let mut iter = args.into_iter();
        let obj = iter.next().unwrap_or(Value::Nil);
        if matches!(obj, Value::Nil) {
            return Ok(MultiValue::new());
        }
        let method_name = match iter.next() {
            Some(Value::String(s)) => s.to_str()?.to_string(),
            _ => {
                return Err(mlua::Error::RuntimeError(
                    "Usage: securecallmethod(table, name, ...)".into(),
                ))
            }
        };
        let obj_table = match &obj {
            Value::Table(t) => t.clone(),
            _ => {
                return Err(mlua::Error::RuntimeError(
                    "Usage: securecallmethod(table, name, ...)".into(),
                ))
            }
        };
        let method: Option<Function> = obj_table.get(method_name)?;
        let Some(method) = method else {
            return Ok(MultiValue::new());
        };
        let securecall: Function = lua.globals().get("securecall")?;
        let mut call_args = vec![Value::Function(method), obj];
        call_args.extend(iter);
        securecall.call::<MultiValue>(MultiValue::from_iter(call_args))
    })
}

/// SecureHandler execution stubs (no-ops).
fn register_secure_handler_stubs(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    g.set(
        "SecureHandlerSetFrameRef",
        lua.create_function(|_, (_frame, _name, _target): (Value, String, Value)| Ok(()))?,
    )?;
    g.set(
        "SecureHandlerExecute",
        lua.create_function(|_, (_frame, _body, _args): (Value, String, MultiValue)| Ok(()))?,
    )?;
    g.set(
        "SecureHandlerWrapScript",
        lua.create_function(|_, (_frame, _script, _body): (Value, String, String)| Ok(()))?,
    )?;
    Ok(())
}

/// State/attribute driver stubs (no-ops).
fn register_state_driver_stubs(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    g.set(
        "RegisterStateDriver",
        lua.create_function(|_, (_frame, _attr, _driver): (Value, String, String)| Ok(()))?,
    )?;
    g.set(
        "UnregisterStateDriver",
        lua.create_function(|_, (_frame, _attr): (Value, String)| Ok(()))?,
    )?;
    g.set(
        "RegisterAttributeDriver",
        lua.create_function(|_, (_frame, _attr, _driver): (Value, String, String)| Ok(()))?,
    )?;
    g.set(
        "UnregisterAttributeDriver",
        lua.create_function(|_, (_frame, _attr): (Value, String)| Ok(()))?,
    )?;
    Ok(())
}

/// SecureCmdOptionParse: returns the default (last semicolon-delimited) option.
fn register_secure_cmd_option_parse(lua: &Lua) -> Result<()> {
    lua.globals().set(
        "SecureCmdOptionParse",
        lua.create_function(|lua, options: String| {
            if let Some(last) = options.split(';').next_back() {
                Ok(Value::String(lua.create_string(last.trim())?))
            } else {
                Ok(Value::Nil)
            }
        })?,
    )
}
