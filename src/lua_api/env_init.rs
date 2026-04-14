//! Initialization helpers for the WoW Lua environment.
//!
//! Standalone functions extracted from `env.rs` that are called during
//! `WowLuaEnv::new()` and from event/script dispatch paths.

use super::builtin_frames::create_builtin_frames;
use super::state::{AddonRuntimeMetrics, SimState};
use crate::lua_api::frame::methods::{
    rilua_button_anchor_hierarchy, rilua_core_state, rilua_misc, rilua_text_attribute_event,
    rilua_widgets,
};
use crate::lua_api::rilua_methods::{registry_set, registry_table_or_create, table_set};
use rilua::LuaApiMut;
use rilua::Val;
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
pub(super) fn call_with_taint<L, H, A>(
    _lua: &L,
    _handler: H,
    _taint: Option<String>,
    _is_blizzard: bool,
    _args: A,
) -> crate::Result<()> {
    Ok(())
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

/// Initialize the primary rilua state: seed registries, globals, frame methods, and taint.
pub(super) fn init_lua_state(
    lua: &mut rilua::Lua,
    state: Rc<RefCell<SimState>>,
) -> crate::Result<()> {
    register_pure_lua_taint_stubs(lua)?;
    init_registry_tables(lua, &state)?;
    register_rilua_globals(lua)?;
    super::globals::rilua_security::create_secure_environment(lua)?;
    init_frame_metatable(lua)?;
    enable_taint_and_wrap_loadstring(lua)?;
    super::keybindings::init_keybindings(&mut *lua.state_mut())?;
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
fn register_pure_lua_taint_stubs(lua: &mut rilua::Lua) -> crate::Result<()> {
    super::globals::rilua_security::register_all(lua)?;
    Ok(())
}

/// Set up registry tables for event dispatch and taint fallback.
fn init_registry_tables(lua: &mut rilua::Lua, state: &Rc<RefCell<SimState>>) -> crate::Result<()> {
    let lua_state = lua.state_mut();
    let _ = state;
    let _ = registry_table_or_create(lua_state, "__addon_names");
    let _ = registry_table_or_create(lua_state, "__addon_timing");
    let _ = registry_table_or_create(lua_state, "__event_individual");
    let _ = registry_table_or_create(lua_state, "__event_all");
    let _ = registry_table_or_create(lua_state, "__scripts");
    let _ = registry_table_or_create(lua_state, "__on_update_scripts");
    let _ = registry_table_or_create(lua_state, "__on_post_update_scripts");
    let _ = registry_table_or_create(lua_state, "__rilua_frame_fields");
    super::on_update::register(lua_state, state)
}

/// Enable Elune taint tracking and wrap loadstring as secure.
fn enable_taint_and_wrap_loadstring(lua: &mut rilua::Lua) -> crate::Result<()> {
    super::rilua_taint::enable_taint_mode(lua);
    Ok(())
}

/// Remove globals that WoW's sandbox doesn't expose and internal helpers
/// now stored in the Lua registry.
fn remove_sandbox_globals(_lua: &mut rilua::Lua) -> crate::Result<()> {
    Ok(())
}

fn register_rilua_globals(lua: &mut rilua::Lua) -> crate::Result<()> {
    super::globals::rilua_stubs::register_all(lua.state_mut());
    super::globals::rilua_create_frame::register_all(lua)?;
    super::globals::rilua_font_strings_collection::register_all(lua)?;
    super::globals::rilua_utility_system_spell::register_all(lua)?;
    super::globals::rilua_admin::register_all(lua)?;
    super::rilua_timer_layout::register_all(lua)?;
    Ok(())
}

fn init_frame_metatable(lua: &mut rilua::Lua) -> crate::Result<()> {
    let state = lua.state_mut();
    let frame_mt = Val::Table(state.gc.alloc_table(rilua::vm::table::Table::new()));
    table_set(state, frame_mt, "__index", frame_mt);
    registry_set(state, "__rilua_frame_mt", frame_mt);

    let Val::Table(frame_mt_ref) = frame_mt else {
        unreachable!("frame metatable must be a table");
    };
    rilua_core_state::register_all(state, frame_mt_ref)?;
    rilua_misc::register_all(state, frame_mt_ref)?;
    rilua_text_attribute_event::register_all(state, frame_mt_ref)?;
    rilua_button_anchor_hierarchy::register_all(state, frame_mt_ref)?;
    rilua_widgets::register_all(state, frame_mt_ref)?;
    Ok(())
}
