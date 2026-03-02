//! Combat lockdown enforcement for protected frames.
//!
//! WoW blocks certain method calls on protected frames when both:
//! - the player is in combat (`InCombatLockdown()` is true), and
//! - the caller is insecure (addon code, not Blizzard code).
//!
//! When blocked, the call silently no-ops and fires `ADDON_ACTION_BLOCKED`.

use crate::lua_api::SimState;
use mlua::Lua;
use std::cell::RefCell;
use std::rc::Rc;

/// Check if action is blocked and fire `ADDON_ACTION_BLOCKED` if so.
///
/// Returns `true` if the caller should early-return (action blocked).
/// Borrows state, extracts needed data, drops the borrow, then fires
/// the event to avoid RefCell conflicts with Lua re-entry.
pub fn check_and_fire(
    lua: &Lua,
    state_rc: &Rc<RefCell<SimState>>,
    frame_id: u64,
    method_name: &str,
) -> bool {
    let blocked_info = {
        let state = state_rc.borrow();
        let Some(frame) = state.widgets.get(frame_id) else {
            return false;
        };
        if !frame.is_protected || !state.player.in_combat {
            return false;
        }
        if !is_caller_insecure(lua) {
            return false;
        }
        let frame_name = frame.name.clone().unwrap_or_default();
        let addon_name = state
            .executing_addon_index
            .and_then(|idx| state.addons.get(idx as usize))
            .map(|a| a.folder_name.clone())
            .unwrap_or_default();
        Some((frame_name, addon_name))
        // state borrow dropped here
    };
    if let Some((frame_name, addon_name)) = blocked_info {
        let func_str = format!("{}:{}()", frame_name, method_name);
        fire_event(lua, &addon_name, &func_str);
        return true;
    }
    false
}

/// Returns `true` if the current Lua call stack is insecure (addon code).
fn is_caller_insecure(lua: &Lua) -> bool {
    lua.globals()
        .get::<mlua::Function>("issecure")
        .and_then(|f| f.call::<bool>(()))
        .map(|secure| !secure)
        .unwrap_or(true)
}

/// Fire the `ADDON_ACTION_BLOCKED` event with pre-extracted strings.
fn fire_event(lua: &Lua, addon_name: &str, function_name: &str) {
    let Ok(fire) = lua.globals().get::<mlua::Function>("FireEvent") else {
        return;
    };
    let Ok(event_str) = lua.create_string("ADDON_ACTION_BLOCKED") else {
        return;
    };
    let Ok(addon_str) = lua.create_string(addon_name) else {
        return;
    };
    let Ok(func_str) = lua.create_string(function_name) else {
        return;
    };
    let _ = fire.call::<()>((event_str, addon_str, func_str));
}
