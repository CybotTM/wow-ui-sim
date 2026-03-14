//! Visibility methods (Show, Hide, SetShown) and OnShow/OnHide recursive firing.
//!
//! WoW fires Show/Hide handlers iteratively, not recursively. When a handler
//! calls Show/Hide on the same frame, the state changes immediately but the
//! handler is deferred. After the current handler returns, the loop detects
//! the state change and fires the next handler. This limits mutual recursion
//! to 12 handler invocations (6 cycles of OnHide→OnShow).

use super::super::handle::{FrameRef, frame_ref};
use super::combat_lockdown;
use crate::lua_api::frame::handle::get_sim_state;
use mlua::Lua;

/// Maximum handler invocations per Show/Hide call (6 cycles × 2 handlers).
const SHOW_HIDE_HANDLER_LIMIT: usize = 12;

/// Fire OnShow on a frame and recursively on its visible children.
pub(crate) fn fire_on_show_recursive(lua: &Lua, id: u64) -> mlua::Result<()> {
    fire_script_recursive(lua, id, "OnShow")
}

/// Register Show, Hide, SetShown methods.
pub(super) fn add_show_hide_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("Show", |lua, this, ()| {
        show_or_hide(lua, this.0, true, "Show")
    });
    methods.add_method("Hide", |lua, this, ()| {
        show_or_hide(lua, this.0, false, "Hide")
    });
    methods.add_method("SetShown", |lua, this, shown: bool| {
        show_or_hide(lua, this.0, shown, "SetShown")
    });
}

/// Unified Show/Hide implementation with iterative handler loop.
///
/// When called from inside a handler (re-entrant), just changes visible
/// state and returns. The outer loop detects the change after the handler
/// returns and fires the next handler.
fn show_or_hide(lua: &Lua, id: u64, show: bool, method_name: &str) -> mlua::Result<()> {
    if blocked_by_combat_lockdown(lua, id, method_name) {
        return Ok(());
    }
    let state_rc = get_sim_state(lua);
    let (needs_change, in_handler, parent_visible) =
        read_show_hide_state(&state_rc.borrow(), id, show);
    if !needs_change {
        return Ok(());
    }
    state_rc.borrow_mut().set_frame_visible(id, show);
    if in_handler || !parent_visible {
        return Ok(());
    }
    drain_visibility_handlers(lua, id, show)?;
    // Restore to outermost caller's intended state
    state_rc.borrow_mut().set_frame_visible(id, show);
    if let Some(f) = state_rc.borrow_mut().widgets.get_mut(id) {
        f.show_hide_depth = 0;
    }
    Ok(())
}

/// Check combat lockdown and fire the blocked event if needed. Returns true if blocked.
fn blocked_by_combat_lockdown(lua: &Lua, id: u64, method_name: &str) -> bool {
    let state_rc = get_sim_state(lua);
    combat_lockdown::check_and_fire(lua, &state_rc, id, method_name)
}

/// Extract (needs_change, in_handler, parent_visible) from state for show_or_hide.
fn read_show_hide_state(
    state: &crate::lua_api::SimState,
    id: u64,
    show: bool,
) -> (bool, bool, bool) {
    let f = state.widgets.get(id);
    (
        f.map(|f| f.visible != show).unwrap_or(false),
        f.map(|f| f.show_hide_depth > 0).unwrap_or(false),
        // WoW only fires OnShow/OnHide when the frame is effectively visible,
        // which requires all ancestors to be visible too. If parent is hidden,
        // just change the flag silently.
        f.and_then(|f| f.parent_id)
            .map(|pid| state.widgets.is_ancestor_visible(pid))
            .unwrap_or(true), // root frames have no parent, always "parent visible"
    )
}

/// Iteratively fire OnShow/OnHide handlers until no more state changes
/// occur or the handler limit is reached.
fn drain_visibility_handlers(lua: &Lua, id: u64, initial_target: bool) -> mlua::Result<()> {
    let state_rc = get_sim_state(lua);
    if let Some(f) = state_rc.borrow_mut().widgets.get_mut(id) {
        f.show_hide_depth = 1;
    }
    let mut target = initial_target;
    for _ in 0..SHOW_HIDE_HANDLER_LIMIT {
        let handler = if target { "OnShow" } else { "OnHide" };
        fire_script_recursive(lua, id, handler)?;
        let visible_after = state_rc
            .borrow()
            .widgets
            .get(id)
            .map(|f| f.visible)
            .unwrap_or(false);
        state_rc.borrow_mut().set_frame_visible(id, target);
        if visible_after == target {
            break;
        }
        target = visible_after;
        state_rc.borrow_mut().set_frame_visible(id, target);
    }
    Ok(())
}

/// Fire a script handler depth-first: recurse into visible children first,
/// then fire the handler on this frame. WoW fires OnShow/OnHide on children
/// before parents so all frames see correct visibility when their handler runs.
fn fire_script_recursive(lua: &Lua, id: u64, handler_name: &str) -> mlua::Result<()> {
    let children: Vec<u64> = {
        let state_rc = get_sim_state(lua);
        let st = state_rc.borrow();
        st.widgets
            .get(id)
            .map(|f| {
                f.children
                    .iter()
                    .filter(|&&cid| st.widgets.get(cid).map(|c| c.visible).unwrap_or(false))
                    .copied()
                    .collect()
            })
            .unwrap_or_default()
    };

    for child_id in children {
        fire_script_recursive(lua, child_id, handler_name)?;
    }

    if let Some(handler) = crate::lua_api::script_helpers::get_script(lua, id, handler_name) {
        let frame_val = frame_ref(lua, id)?;
        if let Err(e) = handler.call::<()>(frame_val) {
            crate::lua_api::script_helpers::call_error_handler(lua, &e.to_string());
        }
    }

    Ok(())
}
