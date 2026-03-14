//! C_Timer namespace for WoW timer functionality.
//!
//! Provides timer creation and management functions used by addons.

use super::super::{PendingTimer, SimState, next_timer_id};
use super::function_container::FunctionContainer;
use mlua::{Lua, Result, Value};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

/// Maximum allowed timer duration in seconds ((2^32 - 1) / 1000).
const MAX_TIMER_SECS: f64 = (u32::MAX as f64) / 1000.0;

/// Register C_Timer namespace and timer-related functions.
pub fn register_timer_api(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let c_timer = lua.create_table()?;

    c_timer.set("After", create_timer_after(lua, Rc::clone(&state))?)?;
    c_timer.set("NewTicker", create_new_ticker(lua, Rc::clone(&state))?)?;
    c_timer.set("NewTimer", create_new_timer(lua, Rc::clone(&state))?)?;

    lua.globals().set("C_Timer", c_timer)?;
    Ok(())
}

/// Extract the Lua function from a callback value.
///
/// Accepts either a plain Lua function or a FunctionContainer UserData.
/// Returns an error if the value is a C function or invalid type.
fn extract_callback_function(lua: &Lua, callback: &Value) -> Result<mlua::Function> {
    match callback {
        Value::Function(f) => {
            // Check it's a Lua function (not C)
            if !is_lua_function(lua, f)? {
                return Err(mlua::Error::RuntimeError(
                    "bad argument #2 (Lua function expected)".to_string(),
                ));
            }
            Ok(f.clone())
        }
        Value::UserData(ud) => {
            // Accept FunctionContainer - extract its inner callback
            let fc = ud.borrow::<FunctionContainer>()?;
            lua.registry_value::<mlua::Function>(&fc.callback)
        }
        _ => Err(mlua::Error::RuntimeError(
            "bad argument #2 (function expected)".to_string(),
        )),
    }
}

/// Check if a function is a Lua function (not a C function).
fn is_lua_function(lua: &Lua, func: &mlua::Function) -> Result<bool> {
    let debug: mlua::Table = lua.globals().get("debug")?;
    let iscfunction: mlua::Function = debug.get("iscfunction")?;
    let is_c: bool = iscfunction.call(func.clone())?;
    Ok(!is_c)
}

/// Validate timer seconds: must be >= 0 and <= MAX_TIMER_SECS.
fn validate_seconds(seconds: f64) -> Result<f64> {
    if seconds < 0.0 || seconds > MAX_TIMER_SECS || seconds.is_nan() || seconds.is_infinite() {
        return Err(mlua::Error::RuntimeError(
            "bad argument #1 (invalid duration)".to_string(),
        ));
    }
    Ok(seconds)
}

/// C_Timer.After(seconds, callback) - one-shot timer, no handle returned.
fn create_timer_after(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<mlua::Function> {
    lua.create_function(move |lua, (seconds, callback): (f64, mlua::Function)| {
        let id = next_timer_id();
        let callback_key = lua.create_registry_value(callback)?;
        let secs = seconds.max(0.0);
        let fire_at = Instant::now() + Duration::from_secs_f64(secs);
        let owner_addon = {
            let s = state.borrow();
            s.loading_addon_index.or(s.executing_addon_index)
        };

        let timer = PendingTimer {
            id,
            fire_at,
            callback_key,
            interval: None,
            remaining: None,
            cancelled: false,
            handle_key: None,
            owner_addon,
        };

        state.borrow_mut().timers.push_back(timer);
        Ok(())
    })
}

/// C_Timer.NewTicker(seconds, callback, iterations) - repeating timer with handle.
fn create_new_ticker(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<mlua::Function> {
    lua.create_function(
        move |lua, (seconds, callback, iterations): (f64, Value, Option<i32>)| {
            let secs = validate_seconds(seconds)?;
            let func = extract_callback_function(lua, &callback)?;

            let id = next_timer_id();
            let callback_key = lua.create_registry_value(func)?;
            let fire_at = Instant::now() + Duration::from_secs_f64(secs);
            let interval = Duration::from_secs_f64(secs);
            let owner_addon = {
                let s = state.borrow();
                s.loading_addon_index.or(s.executing_addon_index)
            };

            let handle = FunctionContainer::new_timer(
                lua,
                lua.registry_value::<mlua::Function>(&callback_key)?,
                Rc::clone(&state),
                id,
            )?;
            let handle_ud = lua.create_userdata(handle)?;
            let handle_key = lua.create_registry_value(handle_ud.clone())?;

            let timer = PendingTimer {
                id,
                fire_at,
                callback_key,
                interval: Some(interval),
                remaining: iterations,
                cancelled: false,
                handle_key: Some(handle_key),
                owner_addon,
            };

            state.borrow_mut().timers.push_back(timer);
            Ok(handle_ud)
        },
    )
}

/// C_Timer.NewTimer(seconds, callback) - one-shot timer with handle.
fn create_new_timer(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<mlua::Function> {
    lua.create_function(move |lua, (seconds, callback): (f64, Value)| {
        let secs = validate_seconds(seconds)?;
        let func = extract_callback_function(lua, &callback)?;

        let id = next_timer_id();
        let callback_key = lua.create_registry_value(func)?;
        let fire_at = Instant::now() + Duration::from_secs_f64(secs);
        let owner_addon = {
            let s = state.borrow();
            s.loading_addon_index.or(s.executing_addon_index)
        };

        let handle = FunctionContainer::new_timer(
            lua,
            lua.registry_value::<mlua::Function>(&callback_key)?,
            Rc::clone(&state),
            id,
        )?;
        let handle_ud = lua.create_userdata(handle)?;
        let handle_key = lua.create_registry_value(handle_ud.clone())?;

        let timer = PendingTimer {
            id,
            fire_at,
            callback_key,
            interval: None,
            remaining: None,
            cancelled: false,
            handle_key: Some(handle_key),
            owner_addon,
        };

        state.borrow_mut().timers.push_back(timer);
        Ok(handle_ud)
    })
}

/// Create a proxy FunctionContainer for passing to a timer callback.
///
/// The proxy shares the same inner state as the original (cancelled, fields)
/// but is a distinct Lua UserData object. `proxy == original` via `__eq`
/// because they share the same `Rc<FcInner>`.
pub fn create_fc_proxy(lua: &Lua, handle_ud: &mlua::AnyUserData) -> Result<mlua::AnyUserData> {
    let fc = handle_ud.borrow::<FunctionContainer>()?;
    let proxy = FunctionContainer::new_proxy(lua, &fc)?;
    drop(fc);
    lua.create_userdata(proxy)
}
