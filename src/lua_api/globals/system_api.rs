//! System utility functions.
//!
//! This module contains WoW's core system functions including:
//! - `type()` - Type introspection with Frame UserData (FrameRef) support
//! - `rawget()` - Raw table access with userdata compatibility
//! - `xpcall()` - Protected call with error handler and varargs (Lua 5.2+ feature)
//! - `SlashCmdList` - Slash command registry table
//! - `FireEvent()` - Simulator utility to fire events for testing
//! - `ReloadUI()` - Reload the interface (fires startup events again)
//! - `GetTime()` - Returns seconds since UI load
//! - Build type checks: `IsPublicTestClient()`, `IsBetaBuild()`, `IsPublicBuild()`
//! - Battle.net stubs: `BNFeaturesEnabled()`, `BNConnected()`, etc.
//! - Streaming stubs: `GetFileStreamingStatus()`, `GetBackgroundLoadingStatus()`

use crate::lua_api::SimState;
use crate::lua_api::frame::{FrameRef, frame_ref};
use mlua::{Lua, Result, Value};
use std::cell::RefCell;
use std::rc::Rc;

/// Register system utility functions in the Lua global namespace.
pub fn register_system_api(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    register_type_overrides(lua)?;
    super::protected_call::register_protected_calls(lua)?;
    register_slash_cmd_list(lua)?;
    register_fire_event(lua, Rc::clone(&state))?;
    register_reload_ui(lua, Rc::clone(&state))?;
    register_build_type_checks(lua)?;
    register_battlenet_stubs(lua)?;
    register_secure_stubs(lua)?;
    super::system_api_runtime::register_runtime_system_api(lua, Rc::clone(&state))?;
    register_lua_stdlib_extensions(lua)?;
    Ok(())
}

/// Override `type()` and `rawget()` to handle frame UserData (FrameRef) as "table".
fn register_type_overrides(lua: &Lua) -> Result<()> {
    register_type_override(lua)?;
    register_rawget_override(lua)
}

/// Override `type()` to report frames (FrameRef UserData) as "table".
///
/// Blizzard's Dump.lua does `type(v) == "table"` checks and we want frames to pass.
fn register_type_override(lua: &Lua) -> Result<()> {
    let type_fn = lua.create_function(|_lua, value: Value| {
        let type_str = match &value {
            Value::Nil => "nil",
            Value::Boolean(_) => "boolean",
            Value::Integer(_) | Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Table(_) => "table",
            Value::Function(_) => "function",
            Value::Thread(_) => "thread",
            Value::UserData(ud) => {
                if ud.borrow::<FrameRef>().is_ok() {
                    return Ok("table");
                }
                "userdata"
            }
            Value::LightUserData(_) | Value::Error(_) | Value::Other(_) => "userdata",
        };
        Ok(type_str)
    })?;
    lua.globals().set("type", type_fn)
}

/// Override `rawget()` to handle userdata gracefully.
///
/// Blizzard's Dump.lua does `rawget(v, 0)` on things that pass `type(v) == "table"`.
fn register_rawget_override(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    let rawget_fn = lua.create_function(|lua, (table, key): (Value, Value)| match table {
        Value::Table(t) => t.raw_get(key),
        Value::UserData(_) => Ok(Value::Nil),
        _ => {
            let original: mlua::Function = lua.named_registry_value("__original_rawget")?;
            original.call((table, key))
        }
    })?;
    let original_rawget: mlua::Function = globals.raw_get("rawget")?;
    lua.set_named_registry_value("__original_rawget", original_rawget)?;
    globals.set("rawget", rawget_fn)
}

/// Register the `SlashCmdList` table.
fn register_slash_cmd_list(lua: &Lua) -> Result<()> {
    lua.globals().set("SlashCmdList", lua.create_table()?)?;
    Ok(())
}

/// Register `FireEvent()` - simulator utility to fire events for testing.
fn register_fire_event(lua: &Lua, _state: Rc<RefCell<SimState>>) -> Result<()> {
    let fire_event = lua.create_function(move |lua, args: mlua::Variadic<Value>| {
        let mut args_iter = args.into_iter();
        let event_name: String = match args_iter.next() {
            Some(Value::String(s)) => s.to_str()?.to_string(),
            _ => {
                return Err(mlua::Error::runtime(
                    "FireEvent requires event name as first argument",
                ));
            }
        };

        let event_args: Vec<Value> = args_iter.collect();

        let listeners =
            crate::lua_api::script_helpers::get_event_listeners_lua_order(lua, &event_name)?;

        for widget_id in listeners {
            if let Some(handler) =
                crate::lua_api::script_helpers::get_script(lua, widget_id, "OnEvent")
            {
                let frame = frame_ref(lua, widget_id)?;

                let mut call_args = vec![frame, Value::String(lua.create_string(&event_name)?)];
                call_args.extend(event_args.iter().cloned());

                if let Err(e) = handler.call::<()>(mlua::MultiValue::from_vec(call_args)) {
                    crate::lua_api::script_helpers::call_error_handler(lua, &e.to_string());
                }
            }
        }

        Ok(())
    })?;
    lua.globals().set("FireEvent", fire_event)?;
    Ok(())
}

/// Register `ReloadUI()` - reload the interface by firing startup events again.
fn register_reload_ui(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let reload_ui = lua.create_function(move |lua, ()| {
        fire_reload_startup_events(lua, &state)?;
        state
            .borrow_mut()
            .console_output
            .push("UI Reloaded".to_string());
        Ok(())
    })?;
    lua.globals().set("ReloadUI", reload_ui)?;
    Ok(())
}

/// Fire the sequence of startup events that ReloadUI replays.
fn fire_reload_startup_events(lua: &Lua, state: &Rc<RefCell<SimState>>) -> Result<()> {
    fire_event_to_listeners(lua, state, "ADDON_LOADED", |lua| {
        let event_str = lua.create_string("ADDON_LOADED")?;
        let addon_name = lua.create_string("WoWUISim")?;
        Ok(vec![Value::String(event_str), Value::String(addon_name)])
    })?;
    fire_event_to_listeners(lua, state, "VARIABLES_LOADED", |lua| {
        Ok(vec![Value::String(lua.create_string("VARIABLES_LOADED")?)])
    })?;
    fire_event_to_listeners(lua, state, "PLAYER_ENTERING_WORLD", |lua| {
        Ok(vec![
            Value::String(lua.create_string("PLAYER_ENTERING_WORLD")?),
            Value::Boolean(false),
            Value::Boolean(true),
        ])
    })?;
    fire_event_to_listeners(lua, state, "UPDATE_BINDINGS", |lua| {
        Ok(vec![Value::String(lua.create_string("UPDATE_BINDINGS")?)])
    })?;
    fire_event_to_listeners(lua, state, "DISPLAY_SIZE_CHANGED", |lua| {
        Ok(vec![Value::String(
            lua.create_string("DISPLAY_SIZE_CHANGED")?,
        )])
    })?;
    fire_event_to_listeners(lua, state, "UI_SCALE_CHANGED", |lua| {
        Ok(vec![Value::String(lua.create_string("UI_SCALE_CHANGED")?)])
    })
}

/// Fire an event to all registered listeners, building extra args via a closure.
fn fire_event_to_listeners<F>(
    lua: &Lua,
    _state: &Rc<RefCell<SimState>>,
    event_name: &str,
    build_extra_args: F,
) -> Result<()>
where
    F: Fn(&Lua) -> Result<Vec<Value>>,
{
    let listeners = crate::lua_api::script_helpers::get_event_listeners_lua_order(lua, event_name)?;
    for widget_id in listeners {
        if let Some(handler) = crate::lua_api::script_helpers::get_script(lua, widget_id, "OnEvent")
            && let Some(frame) = crate::lua_api::script_helpers::get_frame_ref(lua, widget_id)
        {
            let mut call_args = vec![frame];
            call_args.extend(build_extra_args(lua)?);
            if let Err(e) = handler.call::<()>(mlua::MultiValue::from_vec(call_args)) {
                crate::lua_api::script_helpers::call_error_handler(lua, &e.to_string());
            }
        }
    }
    Ok(())
}

/// Register build type check functions.
fn register_build_type_checks(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    globals.set(
        "IsPublicTestClient",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    globals.set("IsBetaBuild", lua.create_function(|_, ()| Ok(false))?)?;
    globals.set("IsPublicBuild", lua.create_function(|_, ()| Ok(true))?)?;
    Ok(())
}

/// Register Battle.net stub functions.
fn register_battlenet_stubs(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    globals.set("BNFeaturesEnabled", lua.create_function(|_, ()| Ok(false))?)?;
    globals.set(
        "BNFeaturesEnabledAndConnected",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    globals.set("BNConnected", lua.create_function(|_, ()| Ok(true))?)?;
    globals.set(
        "BNGetFriendInfo",
        lua.create_function(|_, _index: i32| Ok(Value::Nil))?,
    )?;
    globals.set(
        "BNGetNumFriends",
        lua.create_function(|_, ()| Ok((0, 0, 0, 0)))?,
    )?; // total, online, favorites, favoritesOnline
    globals.set(
        "BNGetInfo",
        lua.create_function(|lua, ()| {
            Ok((
                Value::Integer(0),
                Value::String(lua.create_string("SimPlayer#0000")?),
                Value::Nil,
                Value::String(lua.create_string("")?),
                Value::Boolean(false),
                Value::Boolean(false),
                Value::Boolean(false),
            ))
        })?,
    )?;
    Ok(())
}

/// Register secure environment stubs.
fn register_secure_stubs(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    register_swap_to_global_environment(lua, &globals)?;
    globals.set("IsGMClient", lua.create_function(|_, ()| Ok(false))?)?;
    globals.set(
        "RegisterStaticConstants",
        lua.create_function(|_, _: Value| Ok(()))?,
    )?;
    register_metatable_getters(lua, &globals)?;
    globals.set("C_GamePad", register_c_gamepad(lua)?)?;
    globals.set("C_AssistedCombat", register_c_assisted_combat(lua)?)?;
    globals.set("C_Widget", register_c_widget(lua)?)?;
    Ok(())
}

/// SwapToGlobalEnvironment: sets the caller's environment back to _G.
/// Implemented as a Rust closure so it's a C function (passes impltype test).
fn register_swap_to_global_environment(lua: &Lua, globals: &mlua::Table) -> Result<()> {
    globals.set(
        "SwapToGlobalEnvironment",
        lua.create_function(|lua, ()| {
            let is_secure: bool = lua.globals().get::<mlua::Function>("issecure")?.call(())?;
            if !is_secure {
                return Err(mlua::Error::RuntimeError(
                    "cannot modify function environment from a tainted context".to_string(),
                ));
            }
            let setfenv: mlua::Function = lua.globals().get("setfenv")?;
            setfenv.call::<()>((2, lua.globals()))?;
            Ok(())
        })?,
    )
}

/// GetFrameMetatable/GetButtonMetatable — return metatables with __index forwarding.
fn register_metatable_getters(lua: &Lua, globals: &mlua::Table) -> Result<()> {
    for name in &["GetFrameMetatable", "GetButtonMetatable"] {
        globals.set(
            *name,
            lua.create_function(|lua, ()| {
                let mt = lua.create_table()?;
                let index = create_frame_method_forwarders(lua)?;
                mt.set("__index", index)?;
                Ok(Value::Table(mt))
            })?,
        )?;
    }
    Ok(())
}

/// Build a table of forwarding functions for Frame methods.
///
/// SecureTemplates.lua does `LOCAL_CHECK_Frame = CopyTable(GetFrameMetatable().__index)`
/// then calls `LOCAL_CHECK_Frame.GetAttribute(frame, ...)` — i.e. methods as plain
/// functions with explicit self. We create Lua closures that forward these calls.
fn create_frame_method_forwarders(lua: &Lua) -> Result<mlua::Table> {
    let index = lua.create_table()?;
    let methods = &[
        "GetAttribute",
        "SetAttribute",
        "GetParent",
        "GetName",
        "GetObjectType",
        "IsObjectType",
        "GetFrameStrata",
        "GetFrameLevel",
        "IsShown",
        "IsVisible",
        "GetWidth",
        "GetHeight",
        "GetSize",
        "GetScale",
        "GetAlpha",
    ];
    for method in methods {
        let forwarder = lua
            .load(format!(
                "return function(self, ...) return self:{method}(...) end"
            ))
            .eval::<mlua::Function>()?;
        index.set(*method, forwarder)?;
    }
    Ok(index)
}

/// C_GamePad namespace stubs.
fn register_c_gamepad(lua: &Lua) -> Result<mlua::Table> {
    let t = lua.create_table()?;
    t.set("IsEnabled", lua.create_function(|_, ()| Ok(false))?)?;
    t.set("GetActiveDeviceID", lua.create_function(|_, ()| Ok(0i32))?)?;
    t.set(
        "GetDeviceMappedState",
        lua.create_function(|_, _id: Option<i32>| Ok(Value::Nil))?,
    )?;
    t.set(
        "SetLedColor",
        lua.create_function(|_, _args: mlua::MultiValue| Ok(()))?,
    )?;
    t.set(
        "GetConfig",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    t.set(
        "GetCombinedDeviceID",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    t.set(
        "GetPowerLevel",
        lua.create_function(|_, _id: Option<i32>| Ok(Value::Nil))?,
    )?;
    Ok(t)
}

/// C_AssistedCombat namespace stubs.
fn register_c_assisted_combat(lua: &Lua) -> Result<mlua::Table> {
    let t = lua.create_table()?;
    t.set(
        "GetActionSpell",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetNextCastSpell",
        lua.create_function(|_, _check: Option<bool>| Ok(Value::Nil))?,
    )?;
    t.set(
        "GetRotationSpells",
        lua.create_function(|lua, ()| lua.create_table())?,
    )?;
    t.set(
        "IsAvailable",
        lua.create_function(|lua, ()| {
            Ok((false, Value::String(lua.create_string("Not available")?)))
        })?,
    )?;
    Ok(t)
}

/// C_Widget namespace - widget type checking.
fn register_c_widget(lua: &Lua) -> Result<mlua::Table> {
    let t = lua.create_table()?;
    t.set("IsFrameWidget", lua.create_function(is_frame_widget)?)?;
    t.set("IsRenderableWidget", lua.create_function(is_frame_widget)?)?;
    t.set("IsWidget", lua.create_function(is_frame_widget)?)?;
    Ok(t)
}

/// Check if a Lua value is a FrameRef UserData (i.e. a WoW widget).
fn is_frame_widget(_: &Lua, widget: Value) -> Result<bool> {
    match &widget {
        Value::UserData(ud) => Ok(ud.borrow::<FrameRef>().is_ok()),
        _ => Ok(false),
    }
}

/// Register WoW extensions to Lua stdlib tables (coroutine, math) and global `clock`.
///
/// WoW ships Lua 5.1 with additional C functions patched into the standard tables:
/// - `coroutine.bind`, `coroutine.call`, `coroutine.mainthread`
/// - `math.securerandom`
/// - global `clock` (wall-clock seconds, similar to os.clock)
fn register_lua_stdlib_extensions(lua: &Lua) -> Result<()> {
    register_coroutine_extensions(lua)?;
    register_math_extensions(lua)?;
    register_clock_global(lua)?;
    Ok(())
}

/// Add WoW-specific coroutine functions: bind, call, mainthread.
///
/// - `coroutine.bind(f)` → returns a function that resumes a new coroutine running f
/// - `coroutine.call(f, ...)` → creates and resumes a coroutine immediately
/// - `coroutine.mainthread()` → returns the main thread
fn register_coroutine_extensions(lua: &Lua) -> Result<()> {
    let co_tbl: mlua::Table = lua.globals().get("coroutine")?;
    co_tbl.set(
        "bind",
        lua.create_function(|lua, f: mlua::Function| {
            let co = lua.create_thread(f)?;
            lua.create_function(move |_, args: mlua::MultiValue| {
                co.resume::<mlua::MultiValue>(args)
            })
        })?,
    )?;
    co_tbl.set(
        "call",
        lua.create_function(|lua, (f, args): (mlua::Function, mlua::MultiValue)| {
            let co = lua.create_thread(f)?;
            co.resume::<mlua::MultiValue>(args)
        })?,
    )?;
    co_tbl.set(
        "mainthread",
        lua.create_function(|lua, ()| Ok(lua.current_thread()))?,
    )?;
    Ok(())
}

/// Add `math.securerandom([m [, n]])` → cryptographically secure random number.
///
/// Stubbed to delegate to the standard `math.random` (no security requirements in sim).
fn register_math_extensions(lua: &Lua) -> Result<()> {
    let math_tbl: mlua::Table = lua.globals().get("math")?;
    let math_random: mlua::Function = math_tbl.get("random")?;
    math_tbl.set("securerandom", math_random)?;
    Ok(())
}

/// Add global `clock()` → elapsed time in seconds (approximated via GetTime).
///
/// WoW's `clock()` is a C function returning a float representing wall-clock time.
fn register_clock_global(lua: &Lua) -> Result<()> {
    lua.globals().set(
        "clock",
        lua.create_function(|lua, ()| {
            let get_time: mlua::Function = lua.globals().get("GetTime")?;
            get_time.call::<f64>(())
        })?,
    )?;
    Ok(())
}
