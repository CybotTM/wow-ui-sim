//! Protected call overrides (pcall, xpcall).
//!
//! - `pcall()` - Strips mlua's stack traceback from error messages
//! - `xpcall()` - Varargs support (Lua 5.2+ feature needed by WoW addons)

use mlua::{Lua, MultiValue, Result, Value};

/// Register pcall and xpcall overrides.
pub fn register_protected_calls(lua: &Lua) -> Result<()> {
    register_pcall(lua)?;
    register_xpcall(lua)?;
    Ok(())
}

/// Override `pcall()` to strip mlua's traceback from error messages.
/// mlua wraps Rust callback errors in CallbackError which appends a stack traceback.
/// WoW's pcall returns just the error message string.
fn register_pcall(lua: &Lua) -> Result<()> {
    let orig_pcall: mlua::Function = lua.globals().get("pcall")?;
    let tostring: mlua::Function = lua.globals().get("tostring")?;
    let pcall_fn = lua.create_function(move |lua, args: MultiValue| {
        let result = orig_pcall.call::<MultiValue>(args)?;
        let mut result_vec: Vec<Value> = result.into_iter().collect();
        if let Some(Value::Boolean(false)) = result_vec.first() {
            // Nil error objects pass through unchanged (error() with no args).
            // All other error values are converted to string and traceback-stripped.
            if result_vec.len() > 1 && !matches!(result_vec[1], Value::Nil) {
                let msg = error_to_string(lua, &tostring, &result_vec[1])?;
                let clean = strip_error_wrapper(&msg);
                // Do NOT call collect_lua_error here — pcall is intended to catch errors
                // silently. Reporting them would flood the error log with intentionally
                // caught errors (e.g. nil index checks in anchors, optional method calls).
                result_vec[1] = Value::String(lua.create_string(clean)?);
            }
        }
        Ok(MultiValue::from_iter(result_vec))
    })?;
    lua.globals().set("pcall", pcall_fn)?;
    Ok(())
}

/// Convert an error value to a string, handling both String and Error types.
fn error_to_string(_lua: &Lua, tostring: &mlua::Function, val: &Value) -> Result<String> {
    match val {
        Value::String(s) => Ok(s.to_string_lossy().to_string()),
        _ => {
            let s: mlua::String = tostring.call(val.clone())?;
            Ok(s.to_string_lossy().to_string())
        }
    }
}

/// Strip mlua's "runtime error: " prefix, stack traceback suffix, and trailing newlines.
fn strip_error_wrapper(msg: &str) -> &str {
    let msg = msg.strip_prefix("runtime error: ").unwrap_or(msg);
    let msg = match msg.find("\nstack traceback:") {
        Some(pos) => &msg[..pos],
        None => msg,
    };
    msg.trim_end_matches('\n')
}

/// Override `xpcall()` with varargs support (Lua 5.2+ feature needed by WoW addons).
fn register_xpcall(lua: &Lua) -> Result<()> {
    let xpcall_fn = lua.create_function(|lua, args: MultiValue| {
        let mut args_vec: Vec<Value> = args.into_iter().collect();
        if args_vec.len() < 2 {
            return Err(mlua::Error::RuntimeError(
                "xpcall requires at least 2 arguments".to_string(),
            ));
        }

        let func = match args_vec.remove(0) {
            Value::Function(f) => f,
            _ => {
                return Err(mlua::Error::RuntimeError(
                    "bad argument #1 to 'xpcall' (function expected)".to_string(),
                ));
            }
        };

        let error_handler = match args_vec.remove(0) {
            Value::Function(f) => f,
            _ => {
                return Err(mlua::Error::RuntimeError(
                    "bad argument #2 to 'xpcall' (function expected)".to_string(),
                ));
            }
        };

        let call_args: MultiValue = args_vec.into_iter().collect();
        call_with_handler(lua, &func, &error_handler, call_args)
    })?;
    lua.globals().set("xpcall", xpcall_fn)?;
    Ok(())
}

/// Execute a function with an error handler, returning (true, results...) or (false, handler_result).
fn call_with_handler(
    lua: &Lua,
    func: &mlua::Function,
    error_handler: &mlua::Function,
    call_args: MultiValue,
) -> Result<MultiValue> {
    match func.call::<MultiValue>(call_args) {
        Ok(results) => {
            let mut ret = MultiValue::new();
            ret.push_back(Value::Boolean(true));
            for v in results {
                ret.push_back(v);
            }
            Ok(ret)
        }
        Err(e) => {
            let raw = e.to_string();
            let clean = strip_error_wrapper(&raw);
            crate::lua_api::script_helpers::collect_lua_error(lua, clean);
            let error_msg = lua.create_string(clean)?;
            let handler_result = error_handler.call::<Value>(Value::String(error_msg));
            let mut ret = MultiValue::new();
            ret.push_back(Value::Boolean(false));
            match handler_result {
                Ok(v) => ret.push_back(v),
                Err(he) => ret.push_back(Value::String(lua.create_string(he.to_string())?)),
            }
            Ok(ret)
        }
    }
}
