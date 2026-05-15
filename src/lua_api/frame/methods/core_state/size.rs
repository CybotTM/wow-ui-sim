//! Frame size methods: GetWidth, GetHeight, GetSize, SetSize, SetWidth, SetHeight.

use super::helpers::{
    apply_explicit_height, apply_explicit_size, apply_explicit_width, clear_auto_height_flag,
    clear_auto_width_flag, current_explicit_size_state, frame_id, frame_size, opt_f32,
};
use crate::lua_api::frame::methods::methods_helpers::{
    can_change_protected_state_for, emit_addon_action_blocked,
};
use crate::lua_api::frame::methods::text_attribute_event::refresh_auto_text_height_after_width_change;
use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, call_function_state, frame_ref, table_get_static,
};
use crate::lua_bridge::FromStack;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub fn get_width(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let ignore = bool::from_stack(state, 2).ok().unwrap_or(false);
    let (width, _) = frame_size(state, id, ignore)?;
    state.push(Val::Num(width as f64));
    Ok(1)
}

pub fn get_height(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let ignore = bool::from_stack(state, 2).ok().unwrap_or(false);
    let (_, height) = frame_size(state, id, ignore)?;
    state.push(Val::Num(height as f64));
    Ok(1)
}

pub fn get_size(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let ignore = bool::from_stack(state, 2).ok().unwrap_or(false);
    let (width, height) = frame_size(state, id, ignore)?;
    state.push(Val::Num(width as f64));
    state.push(Val::Num(height as f64));
    Ok(2)
}

pub fn set_size(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let width = opt_f32(state, 2);
    let height = opt_f32(state, 3);
    if !can_change_protected_state_for(state, id) {
        emit_addon_action_blocked(state, id, "SetSize");
        return Ok(0);
    }
    let mut sim = borrow_state_mut(state)?;
    let Some(current) = current_explicit_size_state(&sim, id) else {
        return Ok(0);
    };

    let size_changed = current.width != width || current.height != height;
    if !size_changed {
        if current.width_is_text_auto {
            clear_auto_width_flag(&mut sim, id);
        }
        if current.height_is_text_auto {
            clear_auto_height_flag(&mut sim, id);
        }
        return Ok(0);
    }

    apply_explicit_size(&mut sim, id, width, height);
    drop(sim);
    mark_nearest_layout_parent_dirty(state, id);
    super::super::widgets::refresh_scroll_frames_for_resized_frame(state, id)?;
    Ok(0)
}

pub fn set_fixed_size(state: &mut LuaState) -> LuaResult<u32> {
    set_size(state)
}

pub fn set_width(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let width = opt_f32(state, 2);
    if !can_change_protected_state_for(state, id) {
        emit_addon_action_blocked(state, id, "SetWidth");
        return Ok(0);
    }
    let mut sim = borrow_state_mut(state)?;
    let Some(current) = current_explicit_size_state(&sim, id) else {
        return Ok(0);
    };

    if current.width == width {
        if current.width_is_text_auto {
            clear_auto_width_flag(&mut sim, id);
        }
        return Ok(0);
    }

    apply_explicit_width(&mut sim, id, width);
    drop(sim);
    mark_nearest_layout_parent_dirty(state, id);
    refresh_auto_text_height_after_width_change(state, id);
    super::super::widgets::refresh_scroll_frames_for_resized_frame(state, id)?;
    Ok(0)
}

pub fn set_height(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let height = opt_f32(state, 2);
    if !can_change_protected_state_for(state, id) {
        emit_addon_action_blocked(state, id, "SetHeight");
        return Ok(0);
    }
    let mut sim = borrow_state_mut(state)?;
    let Some(current) = current_explicit_size_state(&sim, id) else {
        return Ok(0);
    };

    if current.height == height {
        if current.height_is_text_auto {
            clear_auto_height_flag(&mut sim, id);
        }
        return Ok(0);
    }

    apply_explicit_height(&mut sim, id, height);
    drop(sim);
    mark_nearest_layout_parent_dirty(state, id);
    super::super::widgets::refresh_scroll_frames_for_resized_frame(state, id)?;
    Ok(0)
}

pub(crate) fn mark_nearest_layout_parent_dirty(state: &mut LuaState, id: u64) {
    if is_loading_addon(state) {
        return;
    }

    let Some(helper) = layout_dirty_helper(state) else {
        return;
    };

    let ancestors = {
        let Ok(sim) = borrow_state(state) else {
            return;
        };
        let mut ancestors = Vec::new();
        let mut current = sim.widgets.get(id).and_then(|frame| frame.parent_id);
        while let Some(parent_id) = current {
            ancestors.push(parent_id);
            current = sim.widgets.get(parent_id).and_then(|frame| frame.parent_id);
        }
        ancestors
    };

    for ancestor_id in ancestors {
        let Ok(frame) = frame_ref(state, ancestor_id) else {
            continue;
        };
        if let Ok(Val::Bool(true)) = call_function_state(state, helper, &[frame]) {
            break;
        }
    }
}

pub(crate) fn mark_visible_layout_subtree_dirty(state: &mut LuaState, id: u64) {
    if is_loading_addon(state) {
        return;
    }

    let Some(helper) = layout_dirty_helper(state) else {
        return;
    };

    let descendants = {
        let Ok(sim) = borrow_state(state) else {
            return;
        };
        let mut descendants = vec![id];
        let mut index = 0;
        while let Some(current_id) = descendants.get(index).copied() {
            index += 1;
            if let Some(frame) = sim.widgets.get(current_id) {
                descendants.extend(frame.children.iter().copied());
            }
        }
        descendants
            .into_iter()
            .filter(|&descendant_id| sim.widgets.is_ancestor_visible(descendant_id))
            .collect::<Vec<_>>()
    };

    for descendant_id in descendants {
        let Ok(frame) = frame_ref(state, descendant_id) else {
            continue;
        };
        let _ = call_function_state(state, helper, &[frame]);
    }
}

fn layout_dirty_helper(state: &mut LuaState) -> Option<Val> {
    let helper = table_get_static(
        state,
        Val::Table(state.global),
        "__wow_mark_layout_frame_dirty",
    );
    matches!(helper, Val::Function(_)).then_some(helper)
}

fn is_loading_addon(state: &mut LuaState) -> bool {
    borrow_state(state)
        .map(|sim| sim.loading_addon_index.is_some())
        .unwrap_or(false)
}
