//! Visibility methods (Show, Hide, SetShown) and OnShow/OnHide recursive firing.
//!
//! WoW fires Show/Hide handlers iteratively, not recursively. When a handler
//! calls Show/Hide on the same frame, the state changes immediately but the
//! handler is deferred. After the current handler returns, the loop detects
//! the state change and fires the next handler. This limits mutual recursion
//! to 12 handler invocations (6 cycles of OnHide→OnShow).

use super::super::handle::{frame_ref, FrameRef};
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
    methods.add_method("Show", |lua, this, ()| show_or_hide(lua, this.0, true));
    methods.add_method("Hide", |lua, this, ()| show_or_hide(lua, this.0, false));
    methods.add_method("SetShown", |lua, this, shown: bool| show_or_hide(lua, this.0, shown));
}

/// Unified Show/Hide implementation with iterative handler loop.
///
/// When called from inside a handler (re-entrant), just changes visible
/// state and returns. The outer loop detects the change after the handler
/// returns and fires the next handler.
fn show_or_hide(lua: &Lua, id: u64, show: bool) -> mlua::Result<()> {
    let state_rc = get_sim_state(lua);
    let (needs_change, in_handler) = {
        let state = state_rc.borrow();
        let f = state.widgets.get(id);
        (
            f.map(|f| f.visible != show).unwrap_or(false),
            f.map(|f| f.show_hide_depth > 0).unwrap_or(false),
        )
    };
    if !needs_change {
        return Ok(());
    }
    state_rc.borrow_mut().set_frame_visible(id, show);
    if in_handler {
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
        let visible_after = state_rc.borrow().widgets.get(id)
            .map(|f| f.visible).unwrap_or(false);
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
        st.widgets.get(id)
            .map(|f| {
                f.children.iter()
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
