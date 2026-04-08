//! Security-related WoW API functions.
//!
//! Contains securecallmethod (not in Elune), fallback taint helpers,
//! SecureHandler stubs, state/attribute driver stubs, and SecureCmdOptionParse.
//!
//! Functions provided by Elune's baselib_shared (issecure, issecurevariable,
//! securecall, securecallfunction, forceinsecure, hooksecurefunc,
//! secureexecuterange) are NOT registered here — they come from the C runtime.
//! When Elune also provides secret/access helpers, we preserve those definitions
//! instead of overriding them with simulator fallbacks.

use mlua::{Function, Lua, MultiValue, Result, Value};

/// Register all security-related API functions.
pub fn register_security_functions(lua: &Lua) -> Result<()> {
    register_taint_functions(lua)?;
    register_secure_handler_stubs(lua)?;
    register_state_driver_stubs(lua)?;
    register_secure_cmd_option_parse(lua)?;
    Ok(())
}

/// Register taint helpers.
///
/// Elune provides real taint-aware implementations for these helpers in normal
/// simulator startup. We only install permissive fallbacks when they are absent
/// so stripped-down environments still boot.
fn register_taint_functions(lua: &Lua) -> Result<()> {
    let globals = lua.globals();

    globals.set("securecallmethod", make_securecallmethod(lua)?)?;

    set_if_missing(
        &globals,
        "issecretvalue",
        lua.create_function(|lua, args: MultiValue| {
            Ok(matches!(args.front(), Some(value) if is_secret_value(lua, value)?))
        })?,
    )?;

    set_if_missing(
        &globals,
        "canaccessvalue",
        lua.create_function(|lua, args: MultiValue| {
            Ok(matches!(args.front(), Some(value) if !is_secret_value(lua, value)?))
        })?,
    )?;
    set_if_missing(
        &globals,
        "canaccessallvalues",
        lua.create_function(|lua, args: MultiValue| {
            for value in &args {
                if is_secret_value(lua, value)? {
                    return Ok(false);
                }
            }
            Ok(true)
        })?,
    )?;
    set_if_missing(
        &globals,
        "canaccesstable",
        lua.create_function(|lua, args: MultiValue| match args.front() {
            Some(Value::Table(table)) => table_is_accessible(lua, table),
            Some(value) => Ok(!is_secret_value(lua, value)?),
            None => Ok(false),
        })?,
    )?;

    set_if_missing(
        &globals,
        "scrub",
        lua.create_function(|_, args: MultiValue| Ok(args))?,
    )?;

    set_if_missing(
        &globals,
        "scrubsecretvalues",
        lua.create_function(|_, args: MultiValue| Ok(args))?,
    )?;

    Ok(())
}

fn set_if_missing(globals: &mlua::Table, name: &str, value: Function) -> Result<()> {
    if matches!(globals.get::<Value>(name)?, Value::Nil) {
        globals.set(name, value)?;
    }
    Ok(())
}

fn is_secret_value(lua: &Lua, value: &Value) -> Result<bool> {
    match value {
        Value::Function(function) => is_loadstring_tainted_function(lua, function),
        _ => Ok(false),
    }
}

fn is_loadstring_tainted_function(lua: &Lua, function: &Function) -> Result<bool> {
    let table = match lua.named_registry_value::<mlua::Table>("__tainted_loadstring_functions") {
        Ok(table) => table,
        Err(_) => return Ok(false),
    };
    Ok(table.get::<bool>(function.clone()).unwrap_or(false))
}

fn table_is_accessible(lua: &Lua, table: &mlua::Table) -> Result<bool> {
    Ok(!is_secret_value(lua, &Value::Table(table.clone()))?
        && !table_contains_secret_values(lua, table)?)
}

fn table_contains_secret_values(lua: &Lua, table: &mlua::Table) -> Result<bool> {
    for pair in table.pairs::<Value, Value>() {
        let (key, value) = pair?;
        if is_secret_value(lua, &key)? || is_secret_value(lua, &value)? {
            return Ok(true);
        }
    }
    Ok(false)
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

/// SecureHandler execution stubs.
///
/// These APIs require Blizzard's restricted execution environment and protected
/// frame semantics. The simulator does not model that yet, so these remain
/// inert no-ops instead of pretending to enforce partial security rules.
fn register_secure_handler_stubs(lua: &Lua) -> Result<()> {
    let globals = lua.globals();

    globals.set(
        "SecureHandlerSetFrameRef",
        lua.create_function(|_, (_frame, _name, _target): (Value, String, Value)| Ok(()))?,
    )?;
    globals.set(
        "SecureHandlerExecute",
        lua.create_function(|_, (_frame, _body, _args): (Value, String, mlua::MultiValue)| Ok(()))?,
    )?;
    globals.set(
        "SecureHandlerWrapScript",
        lua.create_function(|_, (_frame, _script, _body): (Value, String, String)| Ok(()))?,
    )?;

    Ok(())
}

/// State/attribute driver stubs.
///
/// Real drivers depend on SecureStateDriverManager and protected attribute
/// propagation. We leave them inert for now rather than simulating a misleading
/// subset of protected-frame behavior.
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
