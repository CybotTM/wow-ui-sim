//! C_Timer namespace for WoW timer functionality.
//!
//! Provides timer creation and management functions used by addons.

use super::super::{next_timer_id, PendingTimer, SimState};
use mlua::{Lua, Result};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

/// Register C_Timer namespace and timer-related functions.
pub fn register_timer_api(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let c_timer = lua.create_table()?;

    c_timer.set("After", create_timer_after(lua, Rc::clone(&state))?)?;
    c_timer.set("NewTicker", create_new_ticker(lua, Rc::clone(&state))?)?;
    c_timer.set("NewTimer", create_new_timer(lua, Rc::clone(&state))?)?;

    lua.globals().set("C_Timer", c_timer)?;
    Ok(())
}

/// C_Timer.After(seconds, callback) - one-shot timer, no handle returned.
fn create_timer_after(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<mlua::Function> {
    lua.create_function(move |lua, (seconds, callback): (f64, mlua::Function)| {
        let id = next_timer_id();
        let callback_key = lua.create_registry_value(callback)?;
        let secs = seconds.max(0.0);
        let fire_at = Instant::now() + Duration::from_secs_f64(secs);
        let owner_addon = state.borrow().loading_addon_index;

        let timer = PendingTimer {
            id, fire_at, callback_key,
            interval: None, remaining: None, cancelled: false,
            handle_key: None, owner_addon,
        };

        state.borrow_mut().timers.push_back(timer);
        Ok(())
    })
}

/// C_Timer.NewTicker(seconds, callback, iterations) - repeating timer with handle.
fn create_new_ticker(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<mlua::Function> {
    lua.create_function(
        move |lua, (seconds, callback, iterations): (f64, mlua::Function, Option<i32>)| {
            let id = next_timer_id();
            let callback_key = lua.create_registry_value(callback)?;
            let secs = seconds.max(0.0);
            let fire_at = Instant::now() + Duration::from_secs_f64(secs);
            let interval = Duration::from_secs_f64(secs);
            let owner_addon = state.borrow().loading_addon_index;

            let ticker = create_timer_handle(lua, id, &state)?;
            let handle_key = lua.create_registry_value(ticker.clone())?;

            let timer = PendingTimer {
                id, fire_at, callback_key,
                interval: Some(interval), remaining: iterations, cancelled: false,
                handle_key: Some(handle_key), owner_addon,
            };

            state.borrow_mut().timers.push_back(timer);
            Ok(ticker)
        },
    )
}

/// C_Timer.NewTimer(seconds, callback) - one-shot timer with handle.
fn create_new_timer(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<mlua::Function> {
    lua.create_function(move |lua, (seconds, callback): (f64, mlua::Function)| {
        let id = next_timer_id();
        let callback_key = lua.create_registry_value(callback)?;
        let secs = seconds.max(0.0);
        let fire_at = Instant::now() + Duration::from_secs_f64(secs);
        let owner_addon = state.borrow().loading_addon_index;

        let timer_handle = create_timer_handle(lua, id, &state)?;
        let handle_key = lua.create_registry_value(timer_handle.clone())?;

        let timer = PendingTimer {
            id, fire_at, callback_key,
            interval: None, remaining: None, cancelled: false,
            handle_key: Some(handle_key), owner_addon,
        };

        state.borrow_mut().timers.push_back(timer);
        Ok(timer_handle)
    })
}

/// Get or create the shared timer handle metatable.
///
/// The metatable provides:
/// - `__eq`: compares `_id` fields so proxies compare equal to their originals
/// - `__index`: delegates to `_proxy_target` if present (for proxies)
/// - `__newindex`: delegates to `_proxy_target` if present, otherwise rawset
/// - `__tostring`: returns `"TimerHandle: <id>"` for originals; delegates to `tostring(target)` for proxies
fn get_timer_handle_metatable(lua: &Lua) -> Result<mlua::Table> {
    if let Ok(mt) = lua.named_registry_value::<mlua::Table>("wow_timer_handle_mt") {
        return Ok(mt);
    }
    let mt = lua.create_table()?;
    mt.raw_set(
        "__eq",
        lua.create_function(|_, (a, b): (mlua::Table, mlua::Table)| {
            let id_a: u64 = a.raw_get("_id")?;
            let id_b: u64 = b.raw_get("_id")?;
            Ok(id_a == id_b)
        })?,
    )?;
    mt.raw_set(
        "__index",
        lua.create_function(|_, (this, key): (mlua::Table, mlua::Value)| {
            let target: Option<mlua::Table> = this.raw_get("_proxy_target")?;
            match target {
                Some(t) => t.get::<mlua::Value>(key),
                None => Ok(mlua::Value::Nil),
            }
        })?,
    )?;
    mt.raw_set(
        "__newindex",
        lua.create_function(
            |_, (this, key, value): (mlua::Table, mlua::Value, mlua::Value)| {
                let target: Option<mlua::Table> = this.raw_get("_proxy_target")?;
                match target {
                    Some(t) => t.raw_set(key, value),
                    None => this.raw_set(key, value),
                }
            },
        )?,
    )?;
    mt.raw_set(
        "__tostring",
        lua.create_function(|lua, this: mlua::Table| {
            let target: Option<mlua::Table> = this.raw_get("_proxy_target")?;
            match target {
                Some(t) => {
                    let tostring: mlua::Function = lua.globals().get("tostring")?;
                    tostring.call::<mlua::String>(t)
                }
                None => {
                    let id: u64 = this.raw_get("_id")?;
                    lua.create_string(format!("TimerHandle: {}", id))
                }
            }
        })?,
    )?;
    lua.set_named_registry_value("wow_timer_handle_mt", mt.clone())?;
    Ok(mt)
}

/// Create a proxy table that delegates to the original handle.
///
/// The proxy has the same `_id` and shared metatable as the original,
/// so `proxy == original` is true via `__eq`, but they are different
/// Lua table objects (raw table key lookup distinguishes them).
pub fn create_timer_proxy(lua: &Lua, handle: &mlua::Table) -> Result<mlua::Table> {
    let proxy = lua.create_table()?;
    let id: u64 = handle.raw_get("_id")?;
    proxy.raw_set("_id", id)?;
    proxy.raw_set("_proxy_target", handle.clone())?;
    let mt = get_timer_handle_metatable(lua)?;
    proxy.set_metatable(Some(mt));
    Ok(proxy)
}

/// Create a timer handle table with Cancel and IsCancelled methods.
fn create_timer_handle(
    lua: &Lua,
    id: u64,
    state: &Rc<RefCell<SimState>>,
) -> Result<mlua::Table> {
    let handle = lua.create_table()?;
    handle.set("_id", id)?;
    handle.set("_cancelled", false)?;

    let state_cancel = Rc::clone(state);
    let handle_clone = handle.clone();
    let cancel = lua.create_function(move |_, ()| {
        handle_clone.set("_cancelled", true)?;
        let mut state = state_cancel.borrow_mut();
        for timer in state.timers.iter_mut() {
            if timer.id == id {
                timer.cancelled = true;
                break;
            }
        }
        Ok(())
    })?;
    handle.set("Cancel", cancel)?;

    let handle_for_check = handle.clone();
    let is_cancelled = lua.create_function(move |_, ()| {
        let cancelled: bool = handle_for_check.get("_cancelled").unwrap_or(false);
        Ok(cancelled)
    })?;
    handle.set("IsCancelled", is_cancelled)?;

    let mt = get_timer_handle_metatable(lua)?;
    handle.set_metatable(Some(mt));

    Ok(handle)
}
