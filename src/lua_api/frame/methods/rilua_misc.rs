//! rilua RustFn equivalents of the miscellaneous frame methods in `methods_misc.rs`.
//!
//! Each function signature is `pub fn name(state: &mut LuaState) -> LuaResult<u32>`
//! where the return value is the number of results pushed onto the stack.
//!
//! Methods that require mlua table/function support (frame_fields, resolve_and_extract,
//! issecure() call, SetToDefaults) are stubbed with a `// TODO` comment.

use crate::lua_api::rilua_methods::{
    borrow_state, borrow_state_mut, call_function_state, create_string, create_table,
    extract_frame_id, frame_id_from_stack, get_or_create_frame_fields, table_get, table_set,
    val_to_string,
};
use crate::lua_bridge::{FromStack, IntoStack, stack_val, table_set_rust_fn};
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
    table_set_rust_fn(state, mt, "RegisterForDrag", register_for_drag)?;
    table_set_rust_fn(state, mt, "SetMovable", set_movable)?;
    table_set_rust_fn(state, mt, "IsMovable", is_movable)?;
    table_set_rust_fn(state, mt, "StartMoving", start_moving)?;
    table_set_rust_fn(state, mt, "StopMovingOrSizing", stop_moving_or_sizing)?;
    table_set_rust_fn(state, mt, "SetUserPlaced", set_user_placed)?;
    table_set_rust_fn(state, mt, "IsUserPlaced", is_user_placed)?;
    table_set_rust_fn(state, mt, "SetClampedToScreen", set_clamped_to_screen)?;
    table_set_rust_fn(state, mt, "IsClampedToScreen", is_clamped_to_screen)?;

    // Propagation
    table_set_rust_fn(
        state,
        mt,
        "CanPropagateMouseClicks",
        can_propagate_mouse_clicks,
    )?;
    table_set_rust_fn(
        state,
        mt,
        "CanPropagateMouseMotion",
        can_propagate_mouse_motion,
    )?;
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
    table_set_rust_fn(
        state,
        mt,
        "SetPropagateMouseClicks",
        set_propagate_mouse_clicks,
    )?;
    table_set_rust_fn(
        state,
        mt,
        "SetPropagateMouseMotion",
        set_propagate_mouse_motion,
    )?;

    // Gamepad
    table_set_rust_fn(state, mt, "EnableGamePadButton", enable_game_pad_button)?;
    table_set_rust_fn(state, mt, "EnableGamePadStick", enable_game_pad_stick)?;
    table_set_rust_fn(
        state,
        mt,
        "IsGamePadButtonEnabled",
        is_game_pad_button_enabled,
    )?;
    table_set_rust_fn(
        state,
        mt,
        "IsGamePadStickEnabled",
        is_game_pad_stick_enabled,
    )?;
    table_set_rust_fn(
        state,
        mt,
        "ShouldButtonPassThrough",
        should_button_pass_through,
    )?;

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
    table_set_rust_fn(state, mt, "GetResizeBounds", get_resize_bounds)?;
    table_set_rust_fn(state, mt, "SetClampRectInsets", set_clamp_rect_insets)?;
    table_set_rust_fn(state, mt, "SetMinResize", set_min_resize)?;
    table_set_rust_fn(state, mt, "SetMaxResize", set_max_resize)?;
    table_set_rust_fn(state, mt, "SetResizeBounds", set_resize_bounds)?;
    table_set_rust_fn(state, mt, "SetPointsOffset", set_points_offset)?;
    table_set_rust_fn(state, mt, "UpdateHeight", update_height)?;

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
    table_set_rust_fn(
        state,
        mt,
        "IsPreventingSecretValues",
        is_preventing_secret_values,
    )?;
    table_set_rust_fn(state, mt, "IsProtected", is_protected)?;
    table_set_rust_fn(state, mt, "Protect", protect)?;
    table_set_rust_fn(
        state,
        mt,
        "SetPreventSecretValues",
        set_prevent_secret_values,
    )?;

    // Flatten render layers
    table_set_rust_fn(
        state,
        mt,
        "GetEffectivelyFlattensRenderLayers",
        get_effectively_flattens_render_layers,
    )?;
    table_set_rust_fn(
        state,
        mt,
        "GetFlattensRenderLayers",
        get_flattens_render_layers,
    )?;

    // Window / display
    table_set_rust_fn(state, mt, "GetDontSavePosition", get_dont_save_position)?;
    table_set_rust_fn(state, mt, "GetWindow", get_window)?;
    table_set_rust_fn(state, mt, "SetWindow", set_window)?;

    // Misc
    table_set_rust_fn(state, mt, "DesaturateHierarchy", desaturate_hierarchy)?;
    table_set_rust_fn(state, mt, "IsHighlightLocked", is_highlight_locked)?;
    table_set_rust_fn(state, mt, "LockHighlight", lock_highlight)?;
    table_set_rust_fn(
        state,
        mt,
        "IsIgnoringChildrenForBounds",
        is_ignoring_children_for_bounds,
    )?;
    table_set_rust_fn(state, mt, "SetHighlightLocked", set_highlight_locked)?;
    table_set_rust_fn(
        state,
        mt,
        "SetIgnoringChildrenForBounds",
        set_ignoring_children_for_bounds,
    )?;
    table_set_rust_fn(state, mt, "GetOrCreateGroup", get_or_create_group)?;
    table_set_rust_fn(state, mt, "ForceUpdateTimers", force_update_timers)?;
    table_set_rust_fn(state, mt, "RegisterFontStrings", register_font_strings)?;
    table_set_rust_fn(
        state,
        mt,
        "RegisterBackgroundTexture",
        register_background_texture,
    )?;
    table_set_rust_fn(state, mt, "RegisterFrames", register_frames)?;
    table_set_rust_fn(state, mt, "SetBorderAlpha", set_border_alpha)?;
    table_set_rust_fn(state, mt, "SetBorderScalar", set_border_scalar)?;
    table_set_rust_fn(state, mt, "SetBorderTexture", set_border_texture)?;
    table_set_rust_fn(state, mt, "SetFillAlpha", set_fill_alpha)?;
    table_set_rust_fn(state, mt, "SetOwningDialog", set_owning_dialog)?;
    table_set_rust_fn(state, mt, "SetFillTexture", set_fill_texture)?;
    table_set_rust_fn(state, mt, "SetToDefaults", set_to_defaults)?;
    table_set_rust_fn(state, mt, "UnlockHighlight", unlock_highlight)?;

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
    let is_dragging = {
        let sim = borrow_state(state)?;
        sim.active_drag_frame == Some(id)
    };
    state.push(Val::Bool(is_dragging));
    Ok(1)
}

pub fn register_for_drag(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mut buttons = Vec::new();
    let mut index = 2;
    loop {
        let value = stack_val(state, index);
        if value == Val::Nil {
            break;
        }
        if let Some(button) = val_to_string(state, value) {
            buttons.push(button);
        }
        index += 1;
    }
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.registered_drag_buttons.clear();
        frame.registered_drag_buttons.extend(buttons);
    }
    Ok(0)
}

pub fn set_movable(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let movable = bool::from_stack(state, 2)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.movable = movable;
    }
    Ok(0)
}

pub fn is_movable(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let movable = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|frame| frame.movable)
            .unwrap_or(false)
    };
    state.push(Val::Bool(movable));
    Ok(1)
}

pub fn start_moving(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id)
        && frame.movable
    {
        frame.is_moving = true;
    }
    Ok(0)
}

pub fn stop_moving_or_sizing(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        if frame.is_moving {
            frame.user_placed = true;
        }
        frame.is_moving = false;
    }
    Ok(0)
}

pub fn set_user_placed(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let user_placed = bool::from_stack(state, 2)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.user_placed = user_placed;
    }
    Ok(0)
}

pub fn is_user_placed(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let user_placed = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|frame| frame.user_placed)
            .unwrap_or(false)
    };
    state.push(Val::Bool(user_placed));
    Ok(1)
}

pub fn set_clamped_to_screen(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let clamped = bool::from_stack(state, 2)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.clamped_to_screen = clamped;
    }
    sim.widgets.mark_rect_dirty(id);
    Ok(0)
}

pub fn is_clamped_to_screen(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let clamped = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|frame| frame.clamped_to_screen)
            .unwrap_or(false)
    };
    state.push(Val::Bool(clamped));
    Ok(1)
}

// ── Propagation ───────────────────────────────────────────────────────────────

pub fn can_propagate_mouse_clicks(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let val = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|f| f.propagate_mouse_clicks)
            .unwrap_or(false)
    };
    state.push(Val::Bool(val));
    Ok(1)
}

pub fn can_propagate_mouse_motion(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let val = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|f| f.propagate_mouse_motion)
            .unwrap_or(false)
    };
    state.push(Val::Bool(val));
    Ok(1)
}

pub fn does_hyperlink_propagate_to_parent(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let val = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|f| f.propagate_hyperlinks_to_parent)
            .unwrap_or(false)
    };
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
    let val = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|f| f.gamepad_button_enabled)
            .unwrap_or(false)
    };
    state.push(Val::Bool(val));
    Ok(1)
}

pub fn is_game_pad_stick_enabled(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let val = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|f| f.gamepad_stick_enabled)
            .unwrap_or(false)
    };
    state.push(Val::Bool(val));
    Ok(1)
}

pub fn should_button_pass_through(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let button = String::from_stack(state, 2)?;
    let normalized = button.to_ascii_lowercase();
    let val = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|f| f.pass_through_buttons.contains(&normalized))
            .unwrap_or(false)
    };
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
    let val = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|f| !f.alpha_gradients.is_empty())
            .unwrap_or(false)
    };
    state.push(Val::Bool(val));
    Ok(1)
}

pub fn set_alpha_gradient(_state: &mut LuaState) -> LuaResult<u32> {
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
    let val = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|f| f.is_frame_buffer)
            .unwrap_or(false)
    };
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

fn rotate_descendant_textures_fn(sim: &mut crate::lua_api::SimState, frame_id: u64, radians: f32) {
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
    let id = frame_id_from_stack(state, 1)?;
    {
        let needs_resolve = borrow_state(state)?.widgets.is_rect_dirty(id);
        if needs_resolve {
            borrow_state_mut(state)?.resolve_rect_if_dirty(id);
        }
    }
    let (left, bottom, width, height) = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .and_then(|frame| {
                frame
                    .layout_rect
                    .map(|rect| (rect, frame.effective_scale.max(1e-6)))
            })
            .map(|(rect, eff_scale)| {
                (
                    (rect.x / eff_scale) as f64,
                    ((sim.screen_height - rect.y - rect.height) / eff_scale) as f64,
                    (rect.width / eff_scale) as f64,
                    (rect.height / eff_scale) as f64,
                )
            })
            .unwrap_or((0.0, 0.0, 0.0, 0.0))
    };
    (left, bottom, width, height).into_stack(state)
}

pub fn get_clamp_rect_insets(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let (left, right, top, bottom) = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|f| f.clamp_rect_insets)
            .unwrap_or((0.0, 0.0, 0.0, 0.0))
    };
    state.push(Val::Num(left as f64));
    state.push(Val::Num(right as f64));
    state.push(Val::Num(top as f64));
    state.push(Val::Num(bottom as f64));
    Ok(4)
}

pub fn set_clamp_rect_insets(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let left = f64::from_stack(state, 2).unwrap_or(0.0) as f32;
    let right = f64::from_stack(state, 3).unwrap_or(0.0) as f32;
    let top = f64::from_stack(state, 4).unwrap_or(0.0) as f32;
    let bottom = f64::from_stack(state, 5).unwrap_or(0.0) as f32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.clamp_rect_insets = (left, right, top, bottom);
    }
    Ok(0)
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

pub fn get_resize_bounds(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let (min_width, min_height, max_width, max_height) = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|frame| {
                let (min_width, min_height) = frame.resize_bounds_min;
                let (max_width, max_height) = frame
                    .resize_bounds_max
                    .map(|(w, h)| (Val::Num(w as f64), Val::Num(h as f64)))
                    .unwrap_or((Val::Nil, Val::Nil));
                (min_width, min_height, max_width, max_height)
            })
            .unwrap_or((0.0, 0.0, Val::Nil, Val::Nil))
    };
    state.push(Val::Num(min_width as f64));
    state.push(Val::Num(min_height as f64));
    state.push(max_width);
    state.push(max_height);
    Ok(4)
}

pub fn set_min_resize(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let min_width = match stack_val(state, 2) {
        Val::Num(value) => value.max(0.0) as f32,
        _ => 0.0,
    };
    let min_height = match stack_val(state, 3) {
        Val::Num(value) => value.max(0.0) as f32,
        _ => 0.0,
    };
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.resize_bounds_min = (min_width, min_height);
    }
    Ok(0)
}

pub fn set_max_resize(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let max_width = match stack_val(state, 2) {
        Val::Num(value) => Some(value.max(0.0) as f32),
        Val::Nil => None,
        _ => None,
    };
    let max_height = match stack_val(state, 3) {
        Val::Num(value) => Some(value.max(0.0) as f32),
        Val::Nil => None,
        _ => None,
    };
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.resize_bounds_max = max_width.zip(max_height);
    }
    Ok(0)
}

pub fn set_resize_bounds(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let min_width = match stack_val(state, 2) {
        Val::Num(value) => value.max(0.0) as f32,
        _ => 0.0,
    };
    let min_height = match stack_val(state, 3) {
        Val::Num(value) => value.max(0.0) as f32,
        _ => 0.0,
    };
    let max_width = match stack_val(state, 4) {
        Val::Num(value) => Some(value.max(0.0) as f32),
        Val::Nil => None,
        _ => None,
    };
    let max_height = match stack_val(state, 5) {
        Val::Num(value) => Some(value.max(0.0) as f32),
        Val::Nil => None,
        _ => None,
    };
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.resize_bounds_min = (min_width, min_height);
        frame.resize_bounds_max = max_width.zip(max_height);
    }
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
    let level = {
        let sim = borrow_state(state)?;
        highest_frame_level(&sim.widgets, id, iterate_all)
    };
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
    let level = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|f| f.frame_level + f.raise_order)
            .unwrap_or(0)
    };
    state.push(Val::Num(level as f64));
    Ok(1)
}

pub fn is_using_parent_level(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let val = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|f| !f.has_fixed_frame_level)
            .unwrap_or(false)
    };
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
    let val = {
        let sim = borrow_state(state)?;
        frame_has_secret_values_fn(&sim.widgets, id)
            || frame_is_anchoring_restricted_fn(&sim.widgets, id)
    };
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
    let has_any = {
        let sim = borrow_state(state)?;
        frame_has_secret_values_fn(&sim.widgets, id)
            || frame_is_anchoring_restricted_fn(&sim.widgets, id)
    };
    let result = aspect_num.is_some_and(|v| v == 1 && has_any);
    state.push(Val::Bool(result));
    Ok(1)
}

pub fn has_secret_values(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let has_secret_values = {
        let sim = borrow_state(state)?;
        frame_has_secret_values_fn(&sim.widgets, id)
    };
    state.push(Val::Bool(has_secret_values));
    Ok(1)
}

pub fn is_anchoring_restricted(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let is_restricted = {
        let sim = borrow_state(state)?;
        frame_is_anchoring_restricted_fn(&sim.widgets, id)
    };
    state.push(Val::Bool(is_restricted));
    Ok(1)
}

pub fn is_anchoring_secret(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let is_secret = {
        let sim = borrow_state(state)?;
        frame_has_secret_values_fn(&sim.widgets, id)
    };
    state.push(Val::Bool(is_secret));
    Ok(1)
}

pub fn is_preventing_secret_values(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let val = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|f| f.prevent_secret_values)
            .unwrap_or(false)
    };
    state.push(Val::Bool(val));
    Ok(1)
}

pub fn is_protected(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let protected = {
        let sim = borrow_state(state)?;
        sim.widgets.get(id).map(|f| f.is_protected).unwrap_or(false)
    };
    state.push(Val::Bool(protected));
    state.push(Val::Bool(protected));
    Ok(2)
}

pub fn protect(_state: &mut LuaState) -> LuaResult<u32> {
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
    let val = {
        let sim = borrow_state(state)?;
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
    let val = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|f| f.flattens_render_layers)
            .unwrap_or(false)
    };
    state.push(Val::Bool(val));
    Ok(1)
}

// ── Window / Display ──────────────────────────────────────────────────────────

pub fn get_dont_save_position(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let val = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|f| f.dont_save_position)
            .unwrap_or(false)
    };
    state.push(Val::Bool(val));
    Ok(1)
}

pub fn get_window(_state: &mut LuaState) -> LuaResult<u32> {
    // TODO: needs frame_fields table access — requires mlua/Lua table support
    Ok(0)
}

pub fn set_window(_state: &mut LuaState) -> LuaResult<u32> {
    // TODO: needs frame_fields table access — requires mlua/Lua table support
    Ok(0)
}

// ── Misc ──────────────────────────────────────────────────────────────────────

fn frame_fields_table(state: &mut LuaState) -> LuaResult<Val> {
    let frame = Val::from_stack(state, 1)?;
    let Some(id) = extract_frame_id(state, frame) else {
        return Ok(Val::Nil);
    };
    Ok(get_or_create_frame_fields(state, id))
}

fn table_set_array_value(state: &mut LuaState, table: Val, index: i64, value: Val) {
    let Val::Table(table_ref) = table else {
        return;
    };
    if let Some(table) = state.gc.tables.get_mut(table_ref) {
        let _ = table.raw_set(Val::Num(index as f64), value, &state.gc.string_arena);
    }
}

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
    let val = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|f| f.highlight_locked)
            .unwrap_or(false)
    };
    state.push(Val::Bool(val));
    Ok(1)
}

pub fn lock_highlight(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.highlight_locked = true;
    }
    if let Some(highlight_id) = sim
        .widgets
        .get(id)
        .and_then(|frame| frame.children_keys.get("HighlightTexture").copied())
    {
        sim.widgets.set_visible(highlight_id, true);
    }
    Ok(0)
}

pub fn is_ignoring_children_for_bounds(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let val = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|f| f.ignoring_children_for_bounds)
            .unwrap_or(false)
    };
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

pub fn unlock_highlight(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.highlight_locked = false;
    }
    if let Some(highlight_id) = sim
        .widgets
        .get(id)
        .and_then(|frame| frame.children_keys.get("HighlightTexture").copied())
    {
        sim.widgets.set_visible(highlight_id, false);
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

pub fn get_or_create_group(state: &mut LuaState) -> LuaResult<u32> {
    let self_table = Val::from_stack(state, 1)?;
    let group_text = String::from_stack(state, 2)?;
    let order = match Val::from_stack(state, 3)? {
        Val::Num(value) => value,
        _ => 10.0,
    };

    let groups = match table_get(state, self_table, "groups") {
        table @ Val::Table(_) => table,
        _ => {
            let table = create_table(state);
            table_set(state, self_table, "groups", table);
            table
        }
    };

    if let Val::Table(groups_ref) = groups {
        let existing = state
            .gc
            .tables
            .get(groups_ref)
            .map(|table| table.array_slice().to_vec())
            .unwrap_or_default();

        for entry in &existing {
            let group_name = table_get(state, *entry, "groupText");
            let existing_text = val_to_string(state, group_name);
            if existing_text.as_deref() == Some(group_text.as_str()) {
                state.push(*entry);
                return Ok(1);
            }
        }

        let group = create_table(state);
        let group_name = create_string(state, &group_text);
        let categories = create_table(state);
        table_set(state, group, "groupText", group_name);
        table_set(state, group, "order", Val::Num(order));
        table_set(state, group, "categories", categories);

        if let Some(table) = state.gc.tables.get_mut(groups_ref) {
            let _ = table.raw_set(
                Val::Num((existing.len() + 1) as f64),
                group,
                &state.gc.string_arena,
            );
        }
        state.push(group);
        return Ok(1);
    }

    state.push(Val::Nil);
    Ok(1)
}

fn update_height(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

pub fn register_font_strings(state: &mut LuaState) -> LuaResult<u32> {
    let fields = frame_fields_table(state)?;
    let font_strings = create_table(state);
    let mut out_index = 1;
    let mut input_index = 2;
    loop {
        let value = stack_val(state, input_index);
        if value == Val::Nil {
            break;
        }
        table_set_array_value(state, font_strings, out_index, value);
        out_index += 1;
        input_index += 1;
    }
    table_set(state, fields, "fontStrings", font_strings);
    table_set(state, fields, "__registeredFontStrings", font_strings);
    Ok(0)
}

pub fn register_background_texture(state: &mut LuaState) -> LuaResult<u32> {
    let fields = frame_fields_table(state)?;
    let background = Val::from_stack(state, 2)?;
    let texture_kit = Val::from_stack(state, 3)?;
    table_set(state, fields, "backgroundTexture", background);
    table_set(state, fields, "textureKit", texture_kit);
    Ok(0)
}

pub fn register_frames(state: &mut LuaState) -> LuaResult<u32> {
    let fields = frame_fields_table(state)?;
    let frames = create_table(state);
    let mut out_index = 1;
    let mut input_index = 2;
    loop {
        let value = stack_val(state, input_index);
        if value == Val::Nil {
            break;
        }
        table_set_array_value(state, frames, out_index, value);
        out_index += 1;
        input_index += 1;
    }
    table_set(state, fields, "frames", frames);
    Ok(0)
}

pub fn set_owning_dialog(state: &mut LuaState) -> LuaResult<u32> {
    let fields = frame_fields_table(state)?;
    let dialog = Val::from_stack(state, 2)?;
    table_set(state, fields, "owningDialog", dialog);
    table_set(state, fields, "OwningDialog", dialog);
    Ok(0)
}

pub fn force_update_timers(state: &mut LuaState) -> LuaResult<u32> {
    let self_table = Val::from_stack(state, 1)?;
    let active_timers = table_get(state, self_table, "activeTimers");
    let Val::Table(active_timers_ref) = active_timers else {
        return Ok(0);
    };

    let timers = state
        .gc
        .tables
        .get(active_timers_ref)
        .map(|table| table.hash_entries())
        .unwrap_or_default();
    for (_, timer) in timers {
        let update_fn = table_get(state, timer, "OnUpdate");
        if matches!(update_fn, Val::Function(_)) {
            let _ = call_function_state(state, update_fn, &[timer]);
        }
    }
    Ok(0)
}

pub fn set_fill_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let texture = val_to_string(state, stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    let blob = sim.quest_blobs.entry(id).or_default();
    blob.fill_texture = texture;
    Ok(0)
}

pub fn set_fill_alpha(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let alpha = match stack_val(state, 2) {
        Val::Num(value) => Some(value),
        _ => None,
    };
    let mut sim = borrow_state_mut(state)?;
    let blob = sim.quest_blobs.entry(id).or_default();
    blob.fill_alpha = alpha;
    Ok(0)
}

pub fn set_border_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let texture = val_to_string(state, stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    let blob = sim.quest_blobs.entry(id).or_default();
    blob.border_texture = texture;
    Ok(0)
}

pub fn set_border_alpha(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let alpha = match stack_val(state, 2) {
        Val::Num(value) => Some(value),
        _ => None,
    };
    let mut sim = borrow_state_mut(state)?;
    let blob = sim.quest_blobs.entry(id).or_default();
    blob.border_alpha = alpha;
    Ok(0)
}

pub fn set_border_scalar(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let scalar = match stack_val(state, 2) {
        Val::Num(value) => Some(value),
        _ => None,
    };
    let mut sim = borrow_state_mut(state)?;
    let blob = sim.quest_blobs.entry(id).or_default();
    blob.border_scalar = scalar;
    Ok(0)
}

pub fn set_to_defaults(_state: &mut LuaState) -> LuaResult<u32> {
    // TODO: needs reset_frame_to_defaults from methods_misc_minimap — requires mlua context
    Ok(0)
}
