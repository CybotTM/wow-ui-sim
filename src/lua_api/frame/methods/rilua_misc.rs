//! rilua RustFn equivalents of the miscellaneous frame methods in `methods_misc.rs`.
//!
//! Each function signature is `pub fn name(state: &mut LuaState) -> LuaResult<u32>`
//! where the return value is the number of results pushed onto the stack.
//!
//! Methods that require mlua table/function support (frame_fields, resolve_and_extract,
//! issecure() call, SetToDefaults) are stubbed with a `// TODO` comment.

use crate::lua_api::rilua_methods::{
    borrow_state, borrow_state_mut, extract_frame_id, frame_id_from_stack,
};
use crate::lua_bridge::from_stack::FromStack;
use crate::lua_bridge::table_builder::table_set_rust_fn;
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

// ── Register all ─────────────────────────────────────────────────────────────

/// Register all miscellaneous frame methods onto the given metatable.
pub fn register_all(state: &mut LuaState, mt: GcRef<Table>) -> LuaResult<()> {
    // Drag / Input
    table_set_rust_fn(state, mt, "AbortDrag", abort_drag)?;
    table_set_rust_fn(state, mt, "InterceptStartDrag", intercept_start_drag)?;
    table_set_rust_fn(state, mt, "IsDragging", is_dragging)?;

    // Propagation
    table_set_rust_fn(state, mt, "CanPropagateMouseClicks", can_propagate_mouse_clicks)?;
    table_set_rust_fn(state, mt, "CanPropagateMouseMotion", can_propagate_mouse_motion)?;
    table_set_rust_fn(
        state,
        mt,
        "DoesHyperlinkPropagateToParent",
        does_hyperlink_propagate_to_parent,
    )?;
    table_set_rust_fn(
        state,
        mt,
        "SetHyperlinkPropagateToParent",
        set_hyperlink_propagate_to_parent,
    )?;
    table_set_rust_fn(state, mt, "SetPropagateMouseClicks", set_propagate_mouse_clicks)?;
    table_set_rust_fn(state, mt, "SetPropagateMouseMotion", set_propagate_mouse_motion)?;

    // Gamepad
    table_set_rust_fn(state, mt, "EnableGamePadButton", enable_game_pad_button)?;
    table_set_rust_fn(state, mt, "EnableGamePadStick", enable_game_pad_stick)?;
    table_set_rust_fn(state, mt, "IsGamePadButtonEnabled", is_game_pad_button_enabled)?;
    table_set_rust_fn(state, mt, "IsGamePadStickEnabled", is_game_pad_stick_enabled)?;
    table_set_rust_fn(state, mt, "ShouldButtonPassThrough", should_button_pass_through)?;

    // Alpha Gradient
    table_set_rust_fn(state, mt, "ClearAlphaGradient", clear_alpha_gradient)?;
    table_set_rust_fn(state, mt, "HasAlphaGradient", has_alpha_gradient)?;
    table_set_rust_fn(state, mt, "SetAlphaGradient", set_alpha_gradient)?;

    // Draw Layer
    table_set_rust_fn(state, mt, "DisableDrawLayer", disable_draw_layer)?;
    table_set_rust_fn(state, mt, "EnableDrawLayer", enable_draw_layer)?;

    // Frame Buffer
    table_set_rust_fn(state, mt, "IsFrameBuffer", is_frame_buffer)?;
    table_set_rust_fn(state, mt, "RotateTextures", rotate_textures)?;
    table_set_rust_fn(state, mt, "SetIsFrameBuffer", set_is_frame_buffer)?;

    // Bounds / Position
    table_set_rust_fn(state, mt, "GetBoundsRect", get_bounds_rect)?;
    table_set_rust_fn(state, mt, "GetClampRectInsets", get_clamp_rect_insets)?;
    table_set_rust_fn(state, mt, "SetPointsOffset", set_points_offset)?;

    // Attribute stubs
    table_set_rust_fn(state, mt, "CanChangeAttribute", can_change_attribute)?;
    table_set_rust_fn(state, mt, "ClearAttribute", clear_attribute)?;
    table_set_rust_fn(state, mt, "ClearParentKey", clear_parent_key)?;

    // Frame Level / Hierarchy
    table_set_rust_fn(state, mt, "Lower", lower)?;
    table_set_rust_fn(state, mt, "Raise", raise)?;
    table_set_rust_fn(state, mt, "GetHighestFrameLevel", get_highest_frame_level)?;
    table_set_rust_fn(state, mt, "GetRaisedFrameLevel", get_raised_frame_level)?;
    table_set_rust_fn(state, mt, "IsUsingParentLevel", is_using_parent_level)?;
    table_set_rust_fn(state, mt, "SetUsingParentLevel", set_using_parent_level)?;

    // Secret / Protected
    table_set_rust_fn(state, mt, "HasAnySecretAspect", has_any_secret_aspect)?;
    table_set_rust_fn(state, mt, "HasSecretAspect", has_secret_aspect)?;
    table_set_rust_fn(state, mt, "HasSecretValues", has_secret_values)?;
    table_set_rust_fn(state, mt, "IsAnchoringRestricted", is_anchoring_restricted)?;
    table_set_rust_fn(state, mt, "IsAnchoringSecret", is_anchoring_secret)?;
    table_set_rust_fn(state, mt, "IsPreventingSecretValues", is_preventing_secret_values)?;
    table_set_rust_fn(state, mt, "IsProtected", is_protected)?;
    table_set_rust_fn(state, mt, "Protect", protect)?;
    table_set_rust_fn(state, mt, "SetPreventSecretValues", set_prevent_secret_values)?;

    // Flatten render layers
    table_set_rust_fn(
        state,
        mt,
        "GetEffectivelyFlattensRenderLayers",
        get_effectively_flattens_render_layers,
    )?;
    table_set_rust_fn(state, mt, "GetFlattensRenderLayers", get_flattens_render_layers)?;

    // Window / display
    table_set_rust_fn(state, mt, "GetDontSavePosition", get_dont_save_position)?;
    table_set_rust_fn(state, mt, "GetWindow", get_window)?;
    table_set_rust_fn(state, mt, "SetWindow", set_window)?;

    // Misc
    table_set_rust_fn(state, mt, "DesaturateHierarchy", desaturate_hierarchy)?;
    table_set_rust_fn(state, mt, "IsHighlightLocked", is_highlight_locked)?;
    table_set_rust_fn(state, mt, "IsIgnoringChildrenForBounds", is_ignoring_children_for_bounds)?;
    table_set_rust_fn(state, mt, "SetHighlightLocked", set_highlight_locked)?;
    table_set_rust_fn(
        state,
        mt,
        "SetIgnoringChildrenForBounds",
        set_ignoring_children_for_bounds,
    )?;
    table_set_rust_fn(state, mt, "SetToDefaults", set_to_defaults)?;

    Ok(())
}

// ── Drag / Input ──────────────────────────────────────────────────────────────

pub fn abort_drag(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if sim.active_drag_frame == Some(id) {
        sim.set_active_drag_frame(None);
    }
    Ok(0)
}

pub fn intercept_start_drag(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let delegate_val = Val::from_stack(state, 2)?;
    let delegate_id = extract_frame_id(state, delegate_val);

    let result = 'check: {
        let Some(delegate_id) = delegate_id else {
            break 'check false;
        };
        let mut sim = borrow_state_mut(state)?;
        if sim.active_drag_frame != Some(id) {
            break 'check false;
        }
        if sim.widgets.get(delegate_id).is_none() {
            break 'check false;
        }
        sim.set_active_drag_frame(Some(delegate_id));
        true
    };
    state.push(Val::Bool(result));
    Ok(1)
}

pub fn is_dragging(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    state.push(Val::Bool(sim.active_drag_frame == Some(id)));
    Ok(1)
}

// ── Propagation ───────────────────────────────────────────────────────────────

pub fn can_propagate_mouse_clicks(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let val = sim
        .widgets
        .get(id)
        .map(|f| f.propagate_mouse_clicks)
        .unwrap_or(false);
    state.push(Val::Bool(val));
    Ok(1)
}

pub fn can_propagate_mouse_motion(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let val = sim
        .widgets
        .get(id)
        .map(|f| f.propagate_mouse_motion)
        .unwrap_or(false);
    state.push(Val::Bool(val));
    Ok(1)
}

pub fn does_hyperlink_propagate_to_parent(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let val = sim
        .widgets
        .get(id)
        .map(|f| f.propagate_hyperlinks_to_parent)
        .unwrap_or(false);
    state.push(Val::Bool(val));
    Ok(1)
}

pub fn set_hyperlink_propagate_to_parent(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = bool::from_stack(state, 2)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut(id) {
        frame.propagate_hyperlinks_to_parent = value;
    }
    Ok(0)
}

pub fn set_propagate_mouse_clicks(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = bool::from_stack(state, 2)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut(id) {
        frame.propagate_mouse_clicks = value;
    }
    Ok(0)
}

pub fn set_propagate_mouse_motion(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = bool::from_stack(state, 2)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut(id) {
        frame.propagate_mouse_motion = value;
    }
    Ok(0)
}

// ── Gamepad ───────────────────────────────────────────────────────────────────

pub fn enable_game_pad_button(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let enabled = bool::from_stack(state, 2)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut(id) {
        frame.gamepad_button_enabled = enabled;
    }
    Ok(0)
}

pub fn enable_game_pad_stick(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let enabled = bool::from_stack(state, 2)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut(id) {
        frame.gamepad_stick_enabled = enabled;
    }
    Ok(0)
}

pub fn is_game_pad_button_enabled(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let val = sim
        .widgets
        .get(id)
        .map(|f| f.gamepad_button_enabled)
        .unwrap_or(false);
    state.push(Val::Bool(val));
    Ok(1)
}

pub fn is_game_pad_stick_enabled(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let val = sim
        .widgets
        .get(id)
        .map(|f| f.gamepad_stick_enabled)
        .unwrap_or(false);
    state.push(Val::Bool(val));
    Ok(1)
}

pub fn should_button_pass_through(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let button = String::from_stack(state, 2)?;
    let normalized = button.to_ascii_lowercase();
    let sim = borrow_state(state)?;
    let val = sim
        .widgets
        .get(id)
        .map(|f| f.pass_through_buttons.contains(&normalized))
        .unwrap_or(false);
    state.push(Val::Bool(val));
    Ok(1)
}

// ── Alpha Gradient ────────────────────────────────────────────────────────────

pub fn clear_alpha_gradient(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut(id) {
        frame.alpha_gradients.clear();
    }
    Ok(0)
}

pub fn has_alpha_gradient(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let val = sim
        .widgets
        .get(id)
        .map(|f| !f.alpha_gradients.is_empty())
        .unwrap_or(false);
    state.push(Val::Bool(val));
    Ok(1)
}

pub fn set_alpha_gradient(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: needs table support to read the gradient table argument
    Ok(0)
}

// ── Draw Layer ────────────────────────────────────────────────────────────────

pub fn disable_draw_layer(state: &mut LuaState) -> LuaResult<u32> {
    set_draw_layer_enabled_fn(state, false)
}

pub fn enable_draw_layer(state: &mut LuaState) -> LuaResult<u32> {
    set_draw_layer_enabled_fn(state, true)
}

fn set_draw_layer_enabled_fn(state: &mut LuaState, enabled: bool) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let layer_name = String::from_stack(state, 2)?;
    let Some(layer) = crate::widget::DrawLayer::from_str(&layer_name) else {
        return Ok(0);
    };
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.set_draw_layer_enabled(layer, enabled);
    }
    Ok(0)
}

// ── Frame Buffer ──────────────────────────────────────────────────────────────

pub fn is_frame_buffer(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let val = sim
        .widgets
        .get(id)
        .map(|f| f.is_frame_buffer)
        .unwrap_or(false);
    state.push(Val::Bool(val));
    Ok(1)
}

pub fn rotate_textures(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let radians = match Val::from_stack(state, 2)? {
        Val::Num(n) => n as f32,
        _ => 0.0,
    };
    let mut sim = borrow_state_mut(state)?;
    rotate_descendant_textures_fn(&mut sim, id, radians);
    Ok(0)
}

fn rotate_descendant_textures_fn(
    sim: &mut crate::lua_api::SimState,
    frame_id: u64,
    radians: f32,
) {
    let mut pending = vec![frame_id];
    while let Some(current_id) = pending.pop() {
        let Some(frame) = sim.widgets.get(current_id) else {
            continue;
        };
        let child_ids = frame.children.clone();
        pending.extend(child_ids.iter().copied());
        for child_id in child_ids {
            if let Some(child) = sim.widgets.get_mut_visual(child_id) {
                if child.widget_type == crate::widget::WidgetType::Texture {
                    child.rotation = radians;
                }
            }
        }
    }
}

pub fn set_is_frame_buffer(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let enabled = bool::from_stack(state, 2)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut(id) {
        frame.is_frame_buffer = enabled;
    }
    Ok(0)
}

// ── Bounds / Position ─────────────────────────────────────────────────────────

pub fn get_bounds_rect(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: needs resolve_and_extract which requires mlua/SimState layout resolution
    Ok(0)
}

pub fn get_clamp_rect_insets(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let (left, right, top, bottom) = sim
        .widgets
        .get(id)
        .map(|f| f.clamp_rect_insets)
        .unwrap_or((0.0, 0.0, 0.0, 0.0));
    state.push(Val::Num(left as f64));
    state.push(Val::Num(right as f64));
    state.push(Val::Num(top as f64));
    state.push(Val::Num(bottom as f64));
    Ok(4)
}

pub fn set_points_offset(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let x = f64::from_stack(state, 2)?;
    let y = f64::from_stack(state, 3)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        for anchor in &mut frame.anchors {
            anchor.x_offset = x as f32;
            anchor.y_offset = y as f32;
        }
    }
    sim.widgets.mark_rect_dirty(id);
    Ok(0)
}

// ── Attribute stubs ───────────────────────────────────────────────────────────

pub fn can_change_attribute(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    Ok(1)
}

pub fn clear_attribute(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

pub fn clear_parent_key(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

// ── Frame Level / Hierarchy ───────────────────────────────────────────────────

pub fn lower(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    borrow_state_mut(state)?.lower_frame(id);
    Ok(0)
}

pub fn raise(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    borrow_state_mut(state)?.raise_frame(id);
    Ok(0)
}

pub fn get_highest_frame_level(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let iterate_all = Option::<bool>::from_stack(state, 2)?.unwrap_or(false);
    let sim = borrow_state(state)?;
    let level = highest_frame_level(&sim.widgets, id, iterate_all);
    state.push(Val::Num(level as f64));
    Ok(1)
}

fn highest_frame_level(
    widgets: &crate::widget::WidgetRegistry,
    root_id: u64,
    iterate_all_children: bool,
) -> i32 {
    let Some(root) = widgets.get(root_id) else {
        return 0;
    };
    if !iterate_all_children {
        return root.frame_level;
    }
    let mut highest = root.frame_level;
    let mut queue = root.children.clone();
    while let Some(child_id) = queue.pop() {
        let Some(child) = widgets.get(child_id) else {
            continue;
        };
        highest = highest.max(child.frame_level);
        queue.extend(child.children.iter().copied());
    }
    highest
}

pub fn get_raised_frame_level(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let level = sim
        .widgets
        .get(id)
        .map(|f| f.frame_level + f.raise_order)
        .unwrap_or(0);
    state.push(Val::Num(level as f64));
    Ok(1)
}

pub fn is_using_parent_level(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let val = sim
        .widgets
        .get(id)
        .map(|f| !f.has_fixed_frame_level)
        .unwrap_or(false);
    state.push(Val::Bool(val));
    Ok(1)
}

pub fn set_using_parent_level(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let using_parent_level = bool::from_stack(state, 2)?;
    // NOTE: lockdown check omitted — requires mlua context for combat_lockdown::check_and_fire
    let mut sim = borrow_state_mut(state)?;
    let inherited_level = inherited_parent_level_fn(&sim.widgets, id);
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.has_fixed_frame_level = !using_parent_level;
        if let Some(level) = inherited_level.filter(|_| using_parent_level) {
            frame.frame_level = level;
        }
    }
    super::methods_hierarchy::propagate_strata_level_pub(&mut sim.widgets, id);
    Ok(0)
}

fn inherited_parent_level_fn(widgets: &crate::widget::WidgetRegistry, id: u64) -> Option<i32> {
    let frame = widgets.get(id)?;
    let parent_level = widgets.get(frame.parent_id?)?.frame_level;
    Some(parent_level + frame.frame_level_offset.unwrap_or(1))
}

// ── Secret / Protected ────────────────────────────────────────────────────────

pub fn has_any_secret_aspect(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let val = frame_has_secret_values_fn(&sim.widgets, id)
        || frame_is_anchoring_restricted_fn(&sim.widgets, id);
    state.push(Val::Bool(val));
    Ok(1)
}

pub fn has_secret_aspect(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let aspect_val = Val::from_stack(state, 2)?;
    let aspect_num: Option<i64> = match aspect_val {
        Val::Num(n) => Some(n as i64),
        _ => None,
    };
    let sim = borrow_state(state)?;
    let has_any = frame_has_secret_values_fn(&sim.widgets, id)
        || frame_is_anchoring_restricted_fn(&sim.widgets, id);
    let result = aspect_num.is_some_and(|v| v == 1 && has_any);
    state.push(Val::Bool(result));
    Ok(1)
}

pub fn has_secret_values(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    state.push(Val::Bool(frame_has_secret_values_fn(&sim.widgets, id)));
    Ok(1)
}

pub fn is_anchoring_restricted(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    state.push(Val::Bool(frame_is_anchoring_restricted_fn(&sim.widgets, id)));
    Ok(1)
}

pub fn is_anchoring_secret(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    state.push(Val::Bool(frame_has_secret_values_fn(&sim.widgets, id)));
    Ok(1)
}

pub fn is_preventing_secret_values(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let val = sim
        .widgets
        .get(id)
        .map(|f| f.prevent_secret_values)
        .unwrap_or(false);
    state.push(Val::Bool(val));
    Ok(1)
}

pub fn is_protected(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let protected = sim
        .widgets
        .get(id)
        .map(|f| f.is_protected)
        .unwrap_or(false);
    state.push(Val::Bool(protected));
    state.push(Val::Bool(protected));
    Ok(2)
}

pub fn protect(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: needs issecure() call via Lua globals — requires mlua/function support
    Ok(0)
}

pub fn set_prevent_secret_values(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let prevent = bool::from_stack(state, 2)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut(id) {
        frame.prevent_secret_values = prevent;
    }
    Ok(0)
}

fn frame_has_secret_values_fn(widgets: &crate::widget::WidgetRegistry, id: u64) -> bool {
    widgets
        .get(id)
        .map(|f| f.prevent_secret_values)
        .unwrap_or(false)
}

fn frame_is_anchoring_restricted_fn(widgets: &crate::widget::WidgetRegistry, id: u64) -> bool {
    widgets
        .get(id)
        .map(|f| f.forbidden || f.is_protected)
        .unwrap_or(false)
}

// ── Flatten Render Layers ─────────────────────────────────────────────────────

pub fn get_effectively_flattens_render_layers(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let val = {
        let mut current = Some(id);
        let mut found = false;
        while let Some(fid) = current {
            if let Some(f) = sim.widgets.get(fid) {
                if f.flattens_render_layers {
                    found = true;
                    break;
                }
                current = f.parent_id;
            } else {
                break;
            }
        }
        found
    };
    state.push(Val::Bool(val));
    Ok(1)
}

pub fn get_flattens_render_layers(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let val = sim
        .widgets
        .get(id)
        .map(|f| f.flattens_render_layers)
        .unwrap_or(false);
    state.push(Val::Bool(val));
    Ok(1)
}

// ── Window / Display ──────────────────────────────────────────────────────────

pub fn get_dont_save_position(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let val = sim
        .widgets
        .get(id)
        .map(|f| f.dont_save_position)
        .unwrap_or(false);
    state.push(Val::Bool(val));
    Ok(1)
}

pub fn get_window(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: needs frame_fields table access — requires mlua/Lua table support
    Ok(0)
}

pub fn set_window(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: needs frame_fields table access — requires mlua/Lua table support
    Ok(0)
}

// ── Misc ──────────────────────────────────────────────────────────────────────

pub fn desaturate_hierarchy(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let desaturation = f64::from_stack(state, 2)?;
    let exclude_root = Option::<bool>::from_stack(state, 3)?.unwrap_or(false);
    let mut sim = borrow_state_mut(state)?;
    let ids = collect_frame_and_descendant_ids_fn(&sim.widgets, id, exclude_root);
    let desaturated = desaturation > 0.0;
    for fid in ids {
        if let Some(f) = sim.widgets.get_mut_visual(fid) {
            f.desaturated = desaturated;
        }
    }
    Ok(0)
}

fn collect_frame_and_descendant_ids_fn(
    widgets: &crate::widget::WidgetRegistry,
    root_id: u64,
    exclude_root: bool,
) -> Vec<u64> {
    let mut ids = Vec::new();
    let mut stack = vec![root_id];
    while let Some(fid) = stack.pop() {
        if !(exclude_root && fid == root_id) {
            ids.push(fid);
        }
        if let Some(f) = widgets.get(fid) {
            stack.extend(f.children.iter().rev().copied());
        }
    }
    ids
}

pub fn is_highlight_locked(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let val = sim
        .widgets
        .get(id)
        .map(|f| f.highlight_locked)
        .unwrap_or(false);
    state.push(Val::Bool(val));
    Ok(1)
}

pub fn is_ignoring_children_for_bounds(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let val = sim
        .widgets
        .get(id)
        .map(|f| f.ignoring_children_for_bounds)
        .unwrap_or(false);
    state.push(Val::Bool(val));
    Ok(1)
}

pub fn set_highlight_locked(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let locked = bool::from_stack(state, 2)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.highlight_locked = locked;
    }
    Ok(0)
}

pub fn set_ignoring_children_for_bounds(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let ignore = bool::from_stack(state, 2)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut(id) {
        frame.ignoring_children_for_bounds = ignore;
    }
    Ok(0)
}

pub fn set_to_defaults(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: needs reset_frame_to_defaults from methods_misc_minimap — requires mlua context
    Ok(0)
}
