//! Shared helpers for table-backed proxy objects.
//!
//! Used by AbbreviateConfig, FunctionContainer, LuaDurationObject,
//! UnitHealPrediction, CurveUtil, and animation proxies.

use mlua::{AnyUserData, Lua, Result, Value};

/// Extract the hidden userdata from a proxy table or raw userdata value.
pub fn proxy_userdata(value: &Value) -> Option<AnyUserData> {
    match value {
        Value::UserData(userdata) => Some(userdata.clone()),
        Value::Table(table) => match table.raw_get::<Value>("__lud") {
            Ok(Value::UserData(userdata)) => Some(userdata),
            _ => None,
        },
        _ => None,
    }
}

/// Look up a registered method on the userdata's metatable.
pub fn lookup_registered_method(userdata: &AnyUserData, key: &Value) -> Result<Value> {
    let Value::String(name) = key else {
        return Ok(Value::Nil);
    };
    let index_value: Value = userdata.metatable()?.get("__index")?;
    match index_value {
        Value::Function(function) => function.call((userdata.clone(), name.clone())),
        Value::Table(table) => table.raw_get(name.clone()),
        _ => Ok(Value::Nil),
    }
}

/// Wrap a method function so its first argument is bound to the given userdata.
pub fn wrap_fn_with_userdata(
    lua: &Lua,
    function: mlua::Function,
    userdata: AnyUserData,
    bind_method_key: &str,
) -> Result<mlua::Function> {
    let bind_fn: mlua::Function = lua.named_registry_value(bind_method_key)?;
    bind_fn.call((function, Value::UserData(userdata)))
}
