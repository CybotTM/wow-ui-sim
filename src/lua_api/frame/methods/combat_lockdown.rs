//! Combat lockdown enforcement for protected frames.
//!
//! WoW blocks certain method calls on protected frames when both:
//! - the player is in combat (`InCombatLockdown()` is true), and
//! - the caller is insecure (addon code, not Blizzard code).
//!
//! When blocked, the call silently no-ops and fires `ADDON_ACTION_BLOCKED`.

use crate::lua_api::SimState;
use crate::widget::WidgetRegistry;
use mlua::Lua;
use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
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
    if let Some((frame_name, addon_name)) = blocked_call_context(lua, state_rc, frame_id) {
        let func_str = format!("{}:{}()", frame_name, method_name);
        fire_event(lua, &addon_name, &func_str);
        return true;
    }
    false
}

fn blocked_call_context(
    lua: &Lua,
    state_rc: &Rc<RefCell<SimState>>,
    frame_id: u64,
) -> Option<(String, String)> {
    if !is_caller_insecure(lua) {
        return None;
    }
    let state = state_rc.borrow();
    let frame = state.widgets.get(frame_id)?;
    if !state.player.in_combat || !is_protected_action_blocked(&state.widgets, frame_id) {
        return None;
    }
    let frame_name = frame.name.clone().unwrap_or_default();
    let addon_name = state
        .executing_addon_index
        .and_then(|idx| state.addons.get(idx as usize))
        .map(|addon| addon.folder_name.clone())
        .unwrap_or_default();
    Some((frame_name, addon_name))
}

fn is_protected_action_blocked(widgets: &WidgetRegistry, frame_id: u64) -> bool {
    has_protected_self_or_ancestor(widgets, frame_id)
        || has_protected_descendant(widgets, frame_id)
        || is_anchored_to_protected_relation(widgets, frame_id)
}

fn has_protected_self_or_ancestor(widgets: &WidgetRegistry, frame_id: u64) -> bool {
    let mut current = Some(frame_id);
    while let Some(id) = current {
        let Some(frame) = widgets.get(id) else {
            return false;
        };
        if frame.is_protected {
            return true;
        }
        current = frame.parent_id;
    }
    false
}

fn has_protected_descendant(widgets: &WidgetRegistry, frame_id: u64) -> bool {
    let mut queue: VecDeque<u64> = VecDeque::from([frame_id]);
    while let Some(id) = queue.pop_front() {
        let Some(frame) = widgets.get(id) else {
            continue;
        };
        if id != frame_id && frame.is_protected {
            return true;
        }
        queue.extend(frame.children.iter().copied());
    }
    false
}

fn is_anchored_to_protected_relation(widgets: &WidgetRegistry, frame_id: u64) -> bool {
    let mut seen = HashSet::new();
    let mut queue: VecDeque<u64> = anchor_targets(widgets, frame_id).into();
    while let Some(target_id) = queue.pop_front() {
        if !seen.insert(target_id) {
            continue;
        }
        if has_protected_self_or_ancestor(widgets, target_id)
            || has_protected_descendant(widgets, target_id)
        {
            return true;
        }
        queue.extend(anchor_targets(widgets, target_id));
    }
    false
}

fn anchor_targets(widgets: &WidgetRegistry, frame_id: u64) -> Vec<u64> {
    widgets
        .get(frame_id)
        .map(|frame| {
            frame
                .anchors
                .iter()
                .filter_map(|anchor| anchor.relative_to_id.map(|id| id as u64))
                .collect()
        })
        .unwrap_or_default()
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
