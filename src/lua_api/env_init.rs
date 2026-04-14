//! Initialization helpers for the WoW Lua environment.
//!
//! Standalone functions extracted from `env.rs` that are called during
//! `WowLuaEnv::new()` and from event/script dispatch paths.

use super::builtin_frames::create_builtin_frames;
use super::state::{AddonRuntimeMetrics, SimState};
use mlua::{Lua, MultiValue, Value};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

/// Increment threshold counters for a frame's addon time.
pub(super) fn update_threshold_counters(rt: &mut AddonRuntimeMetrics, ms: f64) {
    if ms > 1.0 {
        rt.count_over_1ms += 1;
    }
    if ms > 5.0 {
        rt.count_over_5ms += 1;
    }
    if ms > 10.0 {
        rt.count_over_10ms += 1;
    }
    if ms > 50.0 {
        rt.count_over_50ms += 1;
    }
    if ms > 100.0 {
        rt.count_over_100ms += 1;
    }
    if ms > 500.0 {
        rt.count_over_500ms += 1;
    }
    if ms > 1000.0 {
        rt.count_over_1000ms += 1;
    }
}

/// Stamp addon taint on a handler and call it. The VM applies fixedtaint on entry.
/// For Blizzard addons (is_blizzard=true), clear the handler's taint so issecure()
/// returns true during execution, matching real WoW behavior.
pub(super) fn call_with_taint(
    lua: &Lua,
    handler: mlua::Function,
    taint: Option<String>,
    is_blizzard: bool,
    args: Vec<Value>,
) -> mlua::Result<()> {
    if let Ok(sot) = lua.named_registry_value::<mlua::Function>("__setobjecttaint") {
        if is_blizzard {
            // Clear taint on Blizzard handlers so issecure() returns true.
            sot.call::<()>((handler.clone(), Value::Nil))?;
        } else if let Some(ref name) = taint {
            sot.call::<()>((handler.clone(), name.as_str()))?;
        }
    }
    handler.call(MultiValue::from_vec(args))
}

/// Look up the addon folder name for a given owner_addon index.
pub(super) fn addon_taint_name(state: &Rc<RefCell<SimState>>, idx: Option<u16>) -> Option<String> {
    idx.and_then(|i| {
        state
            .borrow()
            .addons
            .get(i as usize)
            .map(|a| a.folder_name.clone())
    })
}

/// Check whether an addon index refers to a Blizzard addon (runs secure).
pub(super) fn is_blizzard_addon(state: &Rc<RefCell<SimState>>, idx: Option<u16>) -> bool {
    idx.map(|i| {
        state
            .borrow()
            .addons
            .get(i as usize)
            .is_some_and(|a| a.folder_name.starts_with("Blizzard_"))
    })
    .unwrap_or(true)
}

/// Record per-addon timing from an Instant.
pub(super) fn record_addon_time(state: &Rc<RefCell<SimState>>, idx: Option<u16>, start: &Instant) {
    if let Some(i) = idx {
        let ms = start.elapsed().as_secs_f64() * 1000.0;
        if let Some(addon) = state.borrow_mut().addons.get_mut(i as usize) {
            addon.runtime.current_frame_ms += ms;
        }
    }
}

/// Create built-in frames in the widget registry before Lua loads.
/// Registers a `__BuiltIn` pseudo-addon as their owner.
pub(super) fn init_builtin_frames(state: &Rc<RefCell<SimState>>) {
    let mut s = state.borrow_mut();
    let owner = s.addons.len() as u16;
    s.addons.push(super::AddonInfo {
        folder_name: "__BuiltIn".to_string(),
        title: "Built-in Frames".to_string(),
        enabled: true,
        loaded: true,
        ..Default::default()
    });
    let (w, h) = (s.screen_width, s.screen_height);
    create_builtin_frames(&mut s.widgets, w, h, owner);
}

/// Initialize the Lua state: register globals, patch stdlib, run keybindings.
pub(super) fn init_lua_state(lua: &Lua, state: Rc<RefCell<SimState>>) -> crate::Result<()> {
    register_pure_lua_taint_stubs(lua)?;
    init_registry_tables(lua, &state)?;
    super::globals::register_globals(lua, Rc::clone(&state))?;
    super::secure_env::create_secure_environment(lua)?;
    enable_taint_and_wrap_loadstring(lua)?;
    super::keybindings::init_keybindings(lua)?;
    crate::loader::precompiled::init(lua)?;
    remove_sandbox_globals(lua)?;
    Ok(())
}

/// Register pure-Lua taint stubs replacing Elune's C security library.
///
/// Provides the same API surface as Elune's `luaopen_security` and
/// `luaopen_securecalls` without the C library dependency. Taint tracking
/// is permissive — `issecure()` always returns true, `issecurevariable()`
/// always returns true, `securecall` just calls the function directly.
fn register_pure_lua_taint_stubs(lua: &Lua) -> crate::Result<()> {
    lua.load(include_str!("../../data/lua/taint_stubs.lua"))
        .exec()?;
    Ok(())
}

/// Set up registry tables for event dispatch and taint fallback.
fn init_registry_tables(lua: &Lua, state: &Rc<RefCell<SimState>>) -> mlua::Result<()> {
    lua.set_named_registry_value("__event_individual", lua.create_table()?)?;
    lua.set_named_registry_value("__event_all", lua.create_table()?)?;
    // Persistent tables for OnUpdate profiler attribution.
    lua.set_named_registry_value("__frame_owners", lua.create_table()?)?;
    lua.set_named_registry_value("__frame_refs", lua.create_table()?)?;
    lua.set_named_registry_value("__addon_timing", lua.create_table()?)?;
    lua.set_named_registry_value("__addon_names", lua.create_table()?)?;
    lua.set_named_registry_value("__on_update_scripts", lua.create_table()?)?;
    lua.set_named_registry_value("__on_post_update_scripts", lua.create_table()?)?;
    let tainted_loadstring_functions = lua.create_table()?;
    let weak_meta = lua.create_table()?;
    weak_meta.set("__mode", "k")?;
    tainted_loadstring_functions.set_metatable(Some(weak_meta));
    lua.set_named_registry_value(
        "__tainted_loadstring_functions",
        tainted_loadstring_functions,
    )?;
    let taint_fallback: mlua::Function =
        lua.load("return debug.getstacktaint()").into_function()?;
    lua.set_named_registry_value("__get_stack_taint_fallback", taint_fallback)?;
    super::on_update::register(lua, state)
}

/// Enable Elune taint tracking and wrap loadstring as secure.
fn enable_taint_and_wrap_loadstring(lua: &Lua) -> mlua::Result<()> {
    lua.load("seterrorhandler(function() end); debug.settaintmode('rw')")
        .exec()?;
    // Cache setobjecttaint in registry for Rust-side and Lua-side use.
    let sot: mlua::Function = lua.load("return debug.setobjecttaint").eval()?;
    lua.set_named_registry_value("__setobjecttaint", sot)?;
    let sst: mlua::Function = lua.load("return debug.setstacktaint").eval()?;
    lua.set_named_registry_value("__setstacktaint", sst)?;
    lua.load(
        r#"
        local original_ls = loadstring
        local sst = debug.setstacktaint
        local sot = debug.setobjecttaint
        local tainted = debug.getregistry().__tainted_loadstring_functions
        loadstring = debug.newsecurefunction(function(code, name)
            sst("*** ForceTaint_Strong ***")
            local loaded, err = original_ls(code, name)
            if type(loaded) == "function" then
                sot(loaded, "*** ForceTaint_Strong ***")
                tainted[loaded] = true
            end
            return loaded, err
        end)
    "#,
    )
    .exec()?;
    Ok(())
}

/// Remove globals that WoW's sandbox doesn't expose and internal helpers
/// now stored in the Lua registry.
fn remove_sandbox_globals(lua: &Lua) -> mlua::Result<()> {
    let g = lua.globals();
    for name in &[
        "dofile",
        "load",
        "loadfile",
        "module",
        "require",
        "__original_ipairs",
        "__original_rawget",
        "__real_getmetatable",
        "__real_setmetatable",
        "__SetMixinOverride",
        "__report_script_error",
    ] {
        g.set(*name, Value::Nil)?;
    }
    lua.globals()
        .get::<mlua::Table>("string")?
        .set("dump", Value::Nil)?;
    lua.globals()
        .get::<mlua::Table>("math")?
        .set("randomseed", Value::Nil)?;
    Ok(())
}
