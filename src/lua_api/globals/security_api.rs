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

    // Taint-aware functions using Elune's debug.getvaluetaint
    lua.load(
        r##"
        function securecallmethod(obj, name, ...)
            if not obj then return end
            local method = obj[name]
            if method then
                return securecall(method, obj, ...)
            end
        end

        function issecretvalue(val)
            return debug.getvaluetaint(val) ~= nil
        end

        function canaccessvalue(val)
            return debug.getvaluetaint(val) == nil
        end

        function canaccessallvalues(...)
            for i = 1, select("#", ...) do
                if debug.getvaluetaint(select(i, ...)) ~= nil then
                    return false
                end
            end
            return true
        end

        function canaccesstable(tbl)
            return debug.getvaluetaint(tbl) == nil
        end
        "##,
    )
    .exec()?;

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
