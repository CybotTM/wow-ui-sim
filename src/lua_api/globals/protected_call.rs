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
            // Convert error value to string and strip traceback
            if let Some(err_val) = result_vec.get(1) {
                let msg = coerce_to_string(lua, &tostring, err_val)?;
                let clean = strip_traceback(&msg);
                result_vec[1] = Value::String(lua.create_string(clean)?);
            }
        }
        Ok(MultiValue::from_iter(result_vec))
    })?;
    lua.globals().set("pcall", pcall_fn)?;
    Ok(())
}

/// Convert any Lua value to a string via tostring().
fn coerce_to_string(_lua: &Lua, tostring: &mlua::Function, val: &Value) -> Result<String> {
    match val {
        Value::String(s) => Ok(s.to_string_lossy().to_string()),
        _ => {
            let s: mlua::String = tostring.call(val.clone())?;
            Ok(s.to_string_lossy().to_string())
        }
    }
}

/// Strip mlua's stack traceback suffix and trailing newlines from error messages.
fn strip_traceback(msg: &str) -> &str {
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
                ))
            }
        };

        let error_handler = match args_vec.remove(0) {
            Value::Function(f) => f,
            _ => {
                return Err(mlua::Error::RuntimeError(
                    "bad argument #2 to 'xpcall' (function expected)".to_string(),
                ))
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
            let error_msg = lua.create_string(e.to_string())?;
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
