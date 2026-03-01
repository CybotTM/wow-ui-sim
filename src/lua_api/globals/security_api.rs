//! Security-related WoW API functions.
//!
//! Contains securecallmethod (not in Elune), SecureHandler stubs,
//! state/attribute driver stubs, and SecureCmdOptionParse.
//!
//! Functions provided by Elune's baselib_shared (issecure, issecurevariable,
//! securecall, securecallfunction, forceinsecure, hooksecurefunc,
//! secureexecuterange) are NOT registered here — they come from the C runtime.

use mlua::{Function, Lua, MultiValue, Result, Value};

/// Register all security-related API functions.
pub fn register_security_functions(lua: &Lua) -> Result<()> {
    register_taint_functions(lua)?;
    register_secure_handler_stubs(lua)?;
    register_state_driver_stubs(lua)?;
    register_secure_cmd_option_parse(lua)?;
    // Override Elune's issecure() to always return true.
    // We stamp per-addon taint for tracking but don't enforce restrictions,
    // so all code should be considered secure (e.g. SetAttribute in ActionBar).
    lua.globals().set("issecure", lua.create_function(|_, ()| Ok(true))?)?;
    Ok(())
}

/// Register taint-aware functions (no taint in simulator — all return constants).
fn register_taint_functions(lua: &Lua) -> Result<()> {
    let globals = lua.globals();

    globals.set("securecallmethod", make_securecallmethod(lua)?)?;

    // issecretvalue: no taint in simulator, always false
    globals.set(
        "issecretvalue",
        lua.create_function(|_, _: MultiValue| Ok(false))?,
    )?;

    // canaccessvalue / canaccessallvalues / canaccesstable: no taint restrictions
    globals.set(
        "canaccessvalue",
        lua.create_function(|_, _: MultiValue| Ok(true))?,
    )?;
    globals.set(
        "canaccessallvalues",
        lua.create_function(|_, _: MultiValue| Ok(true))?,
    )?;
    globals.set(
        "canaccesstable",
        lua.create_function(|_, _: MultiValue| Ok(true))?,
    )?;

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
        let method_name = extract_method_name(iter.next())?;
        let obj_table = extract_table(&obj)?;
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

fn extract_method_name(arg: Option<Value>) -> Result<String> {
    match arg {
        Some(Value::String(s)) => Ok(s.to_str()?.to_string()),
        _ => Err(mlua::Error::RuntimeError(
            "Usage: securecallmethod(table, name, ...)".into(),
        )),
    }
}

fn extract_table(val: &Value) -> Result<mlua::Table> {
    match val {
        Value::Table(t) => Ok(t.clone()),
        _ => Err(mlua::Error::RuntimeError(
            "Usage: securecallmethod(table, name, ...)".into(),
        )),
    }
}

/// SecureHandler execution stubs (SecureHandlerSetFrameRef, Execute, WrapScript).
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

    Ok(())
}

/// State/attribute driver stubs (Register/UnregisterStateDriver, Register/UnregisterAttributeDriver).
fn register_state_driver_stubs(lua: &Lua) -> Result<()> {
    let globals = lua.globals();

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
