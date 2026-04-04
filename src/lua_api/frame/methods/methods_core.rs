//! Core frame methods: GetName, SetSize, Show/Hide, strata/level, mouse, scale, rect.

use super::super::handle::FrameRef;
use super::combat_lockdown;
use super::methods_core_identity;
use super::methods_core_state;
use super::methods_helpers::{calculate_frame_height, calculate_frame_width};
use crate::lua_api::SimState;
use crate::lua_api::frame::handle::get_sim_state;

pub(crate) use methods_core_identity::is_anim_type;

/// Read screen dimensions from SimState.
pub(crate) fn screen_dims(state: &SimState) -> (f32, f32) {
    (state.screen_width, state.screen_height)
}

/// Check combat lockdown for `id` and fire ADDON_ACTION_BLOCKED if blocked.
/// Returns `true` when the caller should return early (call was blocked).
pub(super) fn lockdown_blocked(lua: &mlua::Lua, id: u64, method_name: &str) -> bool {
    let state_rc = get_sim_state(lua);
    combat_lockdown::check_and_fire(lua, &state_rc, id, method_name)
}

/// Add core frame methods to the shared methods table.
pub fn add_core_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods_core_identity::add_identity_methods(methods);
    add_size_methods(methods);
    super::methods_rect::add_rect_methods(methods);
    methods_core_state::add_core_state_methods(methods);
}

fn add_size_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_size_getters(methods);
    add_size_setters(methods);
}

fn add_size_getters<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_get_width(methods);
    add_get_height(methods);
    add_get_size(methods);
}

fn add_get_width<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetWidth", |lua, this, ignore: Option<bool>| {
        size_value_or_raw(
            lua,
            this.0,
            ignore,
            |f| f.width,
            |widgets, id| calculate_frame_width(widgets, id),
        )
    });
}

fn add_get_height<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetHeight", |lua, this, ignore: Option<bool>| {
        size_value_or_raw(
            lua,
            this.0,
            ignore,
            |f| f.height,
            |widgets, id| calculate_frame_height(widgets, id),
        )
    });
}

fn add_get_size<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetSize", |lua, this, ignore: Option<bool>| {
        if ignore == Some(true) {
            return Ok(raw_frame_size(lua, this.0));
        }
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        state.resolve_rect_if_dirty(this.0);
        Ok((
            calculate_frame_width(&state.widgets, this.0),
            calculate_frame_height(&state.widgets, this.0),
        ))
    });
}

fn size_value_or_raw<Raw, Resolved>(
    lua: &mlua::Lua,
    id: u64,
    ignore: Option<bool>,
    raw: Raw,
    resolved: Resolved,
) -> mlua::Result<f32>
where
    Raw: FnOnce(&crate::widget::Frame) -> f32,
    Resolved: FnOnce(&crate::widget::WidgetRegistry, u64) -> f32,
{
    if ignore == Some(true) {
        return Ok(raw_size_value(lua, id, raw));
    }
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    state.resolve_rect_if_dirty(id);
    Ok(resolved(&state.widgets, id))
}

fn raw_size_value<F>(lua: &mlua::Lua, id: u64, raw: F) -> f32
where
    F: FnOnce(&crate::widget::Frame) -> f32,
{
    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    state.widgets.get(id).map(raw).unwrap_or(0.0)
}

fn raw_frame_size(lua: &mlua::Lua, id: u64) -> (f32, f32) {
    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    state
        .widgets
        .get(id)
        .map(|f| (f.width, f.height))
        .unwrap_or((0.0, 0.0))
}

fn add_size_setters<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_set_size(methods);
    add_set_width(methods);
    add_set_height(methods);
}

fn add_set_size<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetSize", |lua, this, (width, height): (f32, f32)| {
        let id = this.0;
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        let changed = state
            .widgets
            .get(id)
            .map(|f| f.width != width || f.height != height)
            .unwrap_or(false);
        if let Some(frame) = state.widgets.get_mut_visual(id) {
            frame.set_size(width, height);
            frame.width_is_text_auto = false;
        }
        if changed {
            state.widgets.mark_rect_dirty(id);
        }
        Ok(())
    });
}

fn add_set_width<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetWidth", |lua, this, width: f32| {
        let id = this.0;
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        let changed = state
            .widgets
            .get(id)
            .map(|f| f.width != width)
            .unwrap_or(false);
        if let Some(frame) = state.widgets.get_mut_visual(id) {
            frame.width = width;
            frame.width_is_text_auto = false;
        }
        if changed {
            state.widgets.mark_rect_dirty(id);
        }
        Ok(())
    });
}

fn add_set_height<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetHeight", |lua, this, height: f32| {
        let id = this.0;
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        let changed = state
            .widgets
            .get(id)
            .map(|f| f.height != height)
            .unwrap_or(false);
        if let Some(frame) = state.widgets.get_mut_visual(id) {
            frame.height = height;
        }
        if changed {
            state.widgets.mark_rect_dirty(id);
        }
        Ok(())
    });
}
