//! Slider, CheckButton, shared value (Slider+StatusBar), and ScrollFrame methods.

use super::shared::{opt_string, val_to_bool, val_to_f64};
use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, extract_frame_id, frame_id_from_stack, frame_ref,
    get_or_create_frame_fields, sync_child_to_rilua, table_get, table_set,
};
use crate::lua_bridge::{IntoStack, stack_val, table_set_rust_fn, table_set_rust_fn_static};
use crate::widget::{Frame, WidgetType};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

// ---------------------------------------------------------------------------
// Slider
// ---------------------------------------------------------------------------

pub(super) fn set_value_step(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let step = val_to_f64(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.slider_step = step;
    }
    Ok(0)
}

pub(super) fn get_value_step(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim.widgets.get(id).map(|f| f.slider_step).unwrap_or(1.0);
    drop(sim);
    v.into_stack(state)
}

pub(super) fn set_orientation(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let orientation = opt_string(state, 2)
        .map(|s| s.to_uppercase())
        .unwrap_or_else(|| "HORIZONTAL".to_string());
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.slider_orientation = orientation;
    }
    Ok(0)
}

pub(super) fn get_orientation(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let orientation = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|f| f.slider_orientation.clone())
            .unwrap_or_else(|| "HORIZONTAL".to_string())
    };
    let val = crate::lua_api::methods::create_string(state, &orientation);
    val.into_stack(state)
}

pub(super) fn set_obey_step_on_drag(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let obey = val_to_bool(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.slider_obey_step_on_drag = obey;
    }
    Ok(0)
}

pub(super) fn get_obey_step_on_drag(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .map(|f| f.slider_obey_step_on_drag)
        .unwrap_or(false);
    drop(sim);
    v.into_stack(state)
}

pub(super) fn set_steps_per_page(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let steps = val_to_f64(stack_val(state, 2)) as i32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.slider_steps_per_page = steps;
    }
    Ok(0)
}

pub(super) fn get_steps_per_page(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .map(|f| f.slider_steps_per_page)
        .unwrap_or(1);
    drop(sim);
    (v as f64).into_stack(state)
}

pub(super) fn is_dragging_thumb(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim.active_slider_thumb_drag_frame == Some(id);
    drop(sim);
    v.into_stack(state)
}

fn get_named_child_texture_id(state: &LuaState, id: u64, key: &str) -> Option<u64> {
    borrow_state(state)
        .ok()?
        .widgets
        .get(id)
        .and_then(|frame| frame.children_keys.get(key).copied())
}

fn ensure_named_child_texture(state: &mut LuaState, id: u64, key: &str) -> LuaResult<u64> {
    if let Some(child_id) = get_named_child_texture_id(state, id, key) {
        return Ok(child_id);
    }

    let child = Frame::new(WidgetType::Texture, None, Some(id));
    let child_id = child.id;
    let mut sim = borrow_state_mut(state)?;
    sim.widgets.register(child);
    sim.widgets.add_child(id, child_id);
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.children_keys.insert(key.to_string(), child_id);
    }
    Ok(child_id)
}

fn assign_texture_payload(
    state: &mut LuaState,
    child_id: u64,
    texture_value: Val,
) -> LuaResult<()> {
    let mut sim = borrow_state_mut(state)?;
    let Some(child) = sim.widgets.get_mut_visual(child_id) else {
        return Ok(());
    };
    match texture_value {
        Val::Num(value) => {
            child.texture_file_data_id = Some(value as i64);
            child.texture = None;
        }
        Val::Str(value) => {
            let Some(raw) = state.gc.string_arena.get(value) else {
                return Ok(());
            };
            child.texture = Some(String::from_utf8_lossy(raw.data()).to_string());
            child.texture_file_data_id = None;
        }
        _ => {}
    }
    Ok(())
}

pub(super) fn set_thumb_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let texture_value = stack_val(state, 2);
    if let Some(child_id) = extract_frame_id(state, texture_value) {
        let mut sim = borrow_state_mut(state)?;
        if let Some(frame) = sim.widgets.get_mut_visual(id) {
            frame
                .children_keys
                .insert("ThumbTexture".to_string(), child_id);
        }
        return Ok(0);
    }

    let child_id = ensure_named_child_texture(state, id, "ThumbTexture")?;
    assign_texture_payload(state, child_id, texture_value)?;
    Ok(0)
}

pub(super) fn get_thumb_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    match get_named_child_texture_id(state, id, "ThumbTexture") {
        Some(child_id) => {
            let thumb = frame_ref(state, child_id)?;
            state.push(thumb);
        }
        None => state.push(Val::Nil),
    }
    Ok(1)
}

// ---------------------------------------------------------------------------
// Shared value (Slider + StatusBar)
// ---------------------------------------------------------------------------

/// Clamp and apply `value` to a Slider, returning the clamped value.
/// Returns `None` if the value was unchanged (no update needed).
fn apply_slider_value(sim: &mut crate::lua_api::SimState, id: u64, value: f64) -> Option<f64> {
    let f = sim.widgets.get(id)?;
    let clamped = value.clamp(f.slider_min, f.slider_max);
    if clamped == f.slider_value {
        return None;
    }
    sim.widgets.get_mut_visual(id).unwrap().slider_value = clamped;
    Some(clamped)
}

/// Apply `value` to a StatusBar, clamping and updating interpolation state.
fn apply_statusbar_value(sim: &mut crate::lua_api::SimState, id: u64, value: f64) {
    let Some(f) = sim.widgets.get(id) else {
        return;
    };
    let clamped = value.clamp(f.statusbar_min, f.statusbar_max);
    if clamped == f.statusbar_value {
        return;
    }
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.statusbar_value = clamped;
        f.statusbar_interpolated_value = clamped;
        f.statusbar_interpolation_target = None;
    }
}

pub(super) fn shared_set_value(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = val_to_f64(stack_val(state, 2));
    let interpolation_mode = opt_string(state, 3);
    let wtype = borrow_state(state)?.widgets.get(id).map(|f| f.widget_type);
    match wtype {
        Some(WidgetType::Slider) => {
            let clamped = {
                let mut sim = borrow_state_mut(state)?;
                apply_slider_value(&mut sim, id, value)
            };
            // TODO: fire OnValueChanged (needs rilua script dispatch)
            let _ = clamped;
        }
        Some(WidgetType::StatusBar) => {
            let mut sim = borrow_state_mut(state)?;
            if interpolation_mode.is_some() {
                if let Some(f) = sim.widgets.get(id) {
                    let clamped = value.clamp(f.statusbar_min, f.statusbar_max);
                    if let Some(f) = sim.widgets.get_mut_visual(id) {
                        f.statusbar_value = clamped;
                        f.statusbar_interpolation_target = Some(clamped);
                    }
                }
            } else {
                apply_statusbar_value(&mut sim, id, value);
            }
        }
        _ => {}
    }
    Ok(0)
}

pub(super) fn shared_get_value(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .map(|f| match f.widget_type {
            WidgetType::Slider => f.slider_value,
            WidgetType::StatusBar => f.statusbar_value,
            _ => 0.0,
        })
        .unwrap_or(0.0);
    drop(sim);
    v.into_stack(state)
}

pub(super) fn shared_set_min_max_values(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let min = val_to_f64(stack_val(state, 2));
    let max = val_to_f64(stack_val(state, 3));
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        match f.widget_type {
            WidgetType::Slider => {
                f.slider_min = min;
                f.slider_max = max;
                f.slider_value = f.slider_value.clamp(min, max);
            }
            WidgetType::StatusBar => {
                f.statusbar_min = min;
                f.statusbar_max = max;
                f.statusbar_value = f.statusbar_value.clamp(min, max);
                f.statusbar_interpolated_value = f.statusbar_interpolated_value.clamp(min, max);
                f.statusbar_interpolation_target = f
                    .statusbar_interpolation_target
                    .map(|t| t.clamp(min, max))
                    .filter(|&t| t != f.statusbar_interpolated_value);
            }
            _ => {}
        }
    }
    Ok(0)
}

pub(super) fn shared_get_min_max_values(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let (min, max) = sim
        .widgets
        .get(id)
        .map(|f| match f.widget_type {
            WidgetType::Slider => (f.slider_min, f.slider_max),
            WidgetType::StatusBar => (f.statusbar_min, f.statusbar_max),
            _ => (0.0, 1.0),
        })
        .unwrap_or((0.0, 1.0));
    drop(sim);
    (min, max).into_stack(state)
}

// ---------------------------------------------------------------------------
// CheckButton
// ---------------------------------------------------------------------------

pub(super) fn checkbutton_set_checked(state: &mut LuaState) -> LuaResult<u32> {
    use crate::widget::AttributeValue;
    let id = frame_id_from_stack(state, 1)?;
    let checked = val_to_bool(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    let already = sim
        .widgets
        .get(id)
        .and_then(|f| f.attributes.get("__checked"))
        .map(|v| matches!(v, AttributeValue::Boolean(b) if *b == checked))
        .unwrap_or(false);
    if already {
        return Ok(0);
    }
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.attributes
            .insert("__checked".to_string(), AttributeValue::Boolean(checked));
    }
    for key in ["CheckedTexture", "DisabledCheckedTexture"] {
        let visible = sim
            .widgets
            .get(id)
            .map(|frame| {
                let enabled = frame
                    .attributes
                    .get("__enabled")
                    .and_then(|value| match value {
                        AttributeValue::Boolean(flag) => Some(*flag),
                        _ => None,
                    })
                    .unwrap_or(true);
                match key {
                    "CheckedTexture" => enabled && checked,
                    "DisabledCheckedTexture" => !enabled && checked,
                    _ => false,
                }
            })
            .unwrap_or(false);
        if let Some(tex_id) = sim
            .widgets
            .get(id)
            .and_then(|frame| frame.children_keys.get(key).copied())
        {
            sim.set_frame_visible(tex_id, visible);
        }
    }
    Ok(0)
}

pub(super) fn checkbutton_get_checked(state: &mut LuaState) -> LuaResult<u32> {
    use crate::widget::AttributeValue;
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .and_then(|f| f.attributes.get("__checked"))
        .map(|v| matches!(v, AttributeValue::Boolean(true)))
        .unwrap_or(false);
    drop(sim);
    v.into_stack(state)
}

// ---------------------------------------------------------------------------
// ScrollFrame
// ---------------------------------------------------------------------------

fn scroll_child_extent(frame: &crate::widget::Frame, axis: char) -> f64 {
    match (frame.scroll_child_rect_size, axis) {
        (Some((width, _)), 'h') => width as f64,
        (Some((_, height)), 'v') => height as f64,
        _ => 0.0,
    }
}

fn scroll_range(frame: &crate::widget::Frame, axis: char) -> f64 {
    let child_extent = scroll_child_extent(frame, axis);
    let own_extent = match axis {
        'h' => frame.width as f64,
        'v' => frame.height as f64,
        _ => 0.0,
    };
    (child_extent - own_extent).max(0.0)
}

fn scroll_child_subtree_bounds(
    registry: &crate::widget::WidgetRegistry,
    root_id: u64,
    screen_width: f32,
    screen_height: f32,
) -> Option<(f32, f32)> {
    let mut cache = crate::layout::LayoutCache::default();
    let mut stack = vec![root_id];
    let mut bounds: Option<(f32, f32, f32, f32)> = None;

    while let Some(id) = stack.pop() {
        let rect = crate::layout::compute_frame_rect_cached(
            registry,
            id,
            screen_width,
            screen_height,
            &mut cache,
        )
        .rect;
        bounds = Some(match bounds {
            Some((min_x, min_y, max_x, max_y)) => (
                min_x.min(rect.x),
                min_y.min(rect.y),
                max_x.max(rect.x + rect.width),
                max_y.max(rect.y + rect.height),
            ),
            None => (rect.x, rect.y, rect.x + rect.width, rect.y + rect.height),
        });
        if let Some(frame) = registry.get(id) {
            stack.extend(frame.children.iter().copied());
        }
    }

    bounds.map(|(min_x, min_y, max_x, max_y)| (max_x - min_x, max_y - min_y))
}

pub(super) fn get_horizontal_scroll(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let offset = borrow_state(state)?
        .widgets
        .get(id)
        .map(|frame| frame.scroll_horizontal)
        .unwrap_or(0.0);
    state.push(Val::Num(offset));
    Ok(1)
}

pub(super) fn set_horizontal_scroll(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let offset = val_to_f64(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    let range = sim
        .widgets
        .get(id)
        .map(|frame| scroll_range(frame, 'h'))
        .unwrap_or(0.0);
    let new_offset = offset.clamp(0.0, range);
    if sim
        .widgets
        .get(id)
        .is_some_and(|frame| frame.scroll_horizontal == new_offset)
    {
        return Ok(0);
    }
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.scroll_horizontal = new_offset;
    }
    Ok(0)
}

pub(super) fn get_horizontal_scroll_range(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let range = borrow_state(state)?
        .widgets
        .get(id)
        .map(|frame| scroll_range(frame, 'h'))
        .unwrap_or(0.0);
    state.push(Val::Num(range));
    Ok(1)
}

pub(super) fn get_vertical_scroll(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let offset = borrow_state(state)?
        .widgets
        .get(id)
        .map(|frame| frame.scroll_vertical)
        .unwrap_or(0.0);
    state.push(Val::Num(offset));
    Ok(1)
}

pub(super) fn set_vertical_scroll(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let offset = val_to_f64(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    let range = sim
        .widgets
        .get(id)
        .map(|frame| scroll_range(frame, 'v'))
        .unwrap_or(0.0);
    let new_offset = offset.clamp(0.0, range);
    if sim
        .widgets
        .get(id)
        .is_some_and(|frame| frame.scroll_vertical == new_offset)
    {
        return Ok(0);
    }
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.scroll_vertical = new_offset;
    }
    Ok(0)
}

pub(super) fn get_vertical_scroll_range(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let range = borrow_state(state)?
        .widgets
        .get(id)
        .map(|frame| scroll_range(frame, 'v'))
        .unwrap_or(0.0);
    state.push(Val::Num(range));
    Ok(1)
}

pub(super) fn get_scroll_child(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let child_id = {
        let sim = borrow_state(state)?;
        sim.widgets.get(id).and_then(|frame| frame.scroll_child_id)
    };
    match child_id {
        Some(child_id) => {
            let child = frame_ref(state, child_id)?;
            state.push(child);
        }
        None => state.push(Val::Nil),
    }
    Ok(1)
}

pub(super) fn set_scroll_child(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let child = stack_val(state, 2);
    let Some(child_id) = extract_frame_id(state, child) else {
        let mut sim = borrow_state_mut(state)?;
        if let Some(frame) = sim.widgets.get_mut_visual(id) {
            frame.scroll_child_id = None;
            frame.scroll_child_rect_size = None;
        }
        return Ok(0);
    };
    {
        let mut sim = borrow_state_mut(state)?;
        crate::lua_api::frame::methods::widget_scroll::assign_scroll_child(
            &mut sim, id, child_id, true,
        );
    }
    let _ = sync_child_to_rilua(state, id, "ScrollChild", child_id);
    Ok(0)
}

pub(super) fn update_scroll_child_rect(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let scroll_child_rect_size = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .and_then(|frame| frame.scroll_child_id)
            .and_then(|child_id| {
                scroll_child_subtree_bounds(
                    &sim.widgets,
                    child_id,
                    sim.screen_width,
                    sim.screen_height,
                )
            })
    };
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.scroll_child_rect_size = scroll_child_rect_size;
    }
    Ok(0)
}

// ---------------------------------------------------------------------------
// register_slider / register_shared_value / register_scrollframe / register_checkbutton
// ---------------------------------------------------------------------------

pub(super) fn register_slider(state: &mut LuaState, metatable: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(state, metatable, "SetValueStep", set_value_step)?;
    table_set_rust_fn_static(state, metatable, "GetValueStep", get_value_step)?;
    table_set_rust_fn_static(state, metatable, "SetOrientation", set_orientation)?;
    table_set_rust_fn_static(state, metatable, "GetOrientation", get_orientation)?;
    table_set_rust_fn_static(state, metatable, "SetObeyStepOnDrag", set_obey_step_on_drag)?;
    table_set_rust_fn_static(state, metatable, "GetObeyStepOnDrag", get_obey_step_on_drag)?;
    table_set_rust_fn_static(state, metatable, "SetStepsPerPage", set_steps_per_page)?;
    table_set_rust_fn_static(state, metatable, "GetStepsPerPage", get_steps_per_page)?;
    table_set_rust_fn_static(state, metatable, "IsDraggingThumb", is_dragging_thumb)?;
    table_set_rust_fn_static(state, metatable, "SetThumbTexture", set_thumb_texture)?;
    table_set_rust_fn_static(state, metatable, "GetThumbTexture", get_thumb_texture)?;
    Ok(())
}

fn read_color_component(state: &mut LuaState, id: u64, key: &str, default: f64) -> f64 {
    let fields = get_or_create_frame_fields(state, id);
    match table_get(state, fields, key) {
        Val::Num(value) => value,
        _ => default,
    }
}

fn write_color_components(state: &mut LuaState, id: u64, rgba: (f64, f64, f64, f64)) {
    let fields = get_or_create_frame_fields(state, id);
    table_set(state, fields, "__color_r", Val::Num(rgba.0));
    table_set(state, fields, "__color_g", Val::Num(rgba.1));
    table_set(state, fields, "__color_b", Val::Num(rgba.2));
    table_set(state, fields, "__color_a", Val::Num(rgba.3));
}

fn rgb_to_hsv(r: f64, g: f64, b: f64) -> (f64, f64, f64) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;
    let hue = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * ((g - b) / delta).rem_euclid(6.0)
    } else if max == g {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };
    let sat = if max == 0.0 { 0.0 } else { delta / max };
    (hue, sat, max)
}

fn hsv_to_rgb(h: f64, s: f64, v: f64) -> (f64, f64, f64) {
    let c = v * s;
    let x = c * (1.0 - (((h / 60.0).rem_euclid(2.0)) - 1.0).abs());
    let m = v - c;
    let (r1, g1, b1) = match h.rem_euclid(360.0) {
        h if h < 60.0 => (c, x, 0.0),
        h if h < 120.0 => (x, c, 0.0),
        h if h < 180.0 => (0.0, c, x),
        h if h < 240.0 => (0.0, x, c),
        h if h < 300.0 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    (r1 + m, g1 + m, b1 + m)
}

fn get_colorselect_texture(state: &mut LuaState, id: u64, key: &str) -> LuaResult<u32> {
    match get_named_child_texture_id(state, id, key) {
        Some(child_id) => {
            let texture = frame_ref(state, child_id)?;
            state.push(texture);
        }
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn set_colorselect_texture(state: &mut LuaState, id: u64, key: &str, value: Val) -> LuaResult<u32> {
    if matches!(value, Val::Nil) {
        let mut sim = borrow_state_mut(state)?;
        if let Some(frame) = sim.widgets.get_mut_visual(id) {
            frame.children_keys.remove(key);
        }
        return Ok(0);
    }
    if let Some(child_id) = extract_frame_id(state, value) {
        let mut sim = borrow_state_mut(state)?;
        if let Some(frame) = sim.widgets.get_mut_visual(id) {
            frame.children_keys.insert(key.to_string(), child_id);
        }
        return Ok(0);
    }
    let child_id = ensure_named_child_texture(state, id, key)?;
    assign_texture_payload(state, child_id, value)?;
    Ok(0)
}

pub(super) fn colorselect_set_color_rgb(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let alpha = read_color_component(state, id, "__color_a", 1.0);
    write_color_components(
        state,
        id,
        (
            val_to_f64(stack_val(state, 2)),
            val_to_f64(stack_val(state, 3)),
            val_to_f64(stack_val(state, 4)),
            alpha,
        ),
    );
    Ok(0)
}

pub(super) fn colorselect_get_color_rgb(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let r = read_color_component(state, id, "__color_r", 1.0);
    let g = read_color_component(state, id, "__color_g", 1.0);
    let b = read_color_component(state, id, "__color_b", 1.0);
    state.push(Val::Num(r));
    state.push(Val::Num(g));
    state.push(Val::Num(b));
    Ok(3)
}

pub(super) fn colorselect_set_color_hsv(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let alpha = read_color_component(state, id, "__color_a", 1.0);
    let h = val_to_f64(stack_val(state, 2));
    let s = val_to_f64(stack_val(state, 3));
    let v = val_to_f64(stack_val(state, 4));
    let (r, g, b) = hsv_to_rgb(h, s, v);
    write_color_components(state, id, (r, g, b, alpha));
    Ok(0)
}

pub(super) fn colorselect_get_color_hsv(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let r = read_color_component(state, id, "__color_r", 1.0);
    let g = read_color_component(state, id, "__color_g", 1.0);
    let b = read_color_component(state, id, "__color_b", 1.0);
    let (h, s, v) = rgb_to_hsv(r, g, b);
    state.push(Val::Num(h));
    state.push(Val::Num(s));
    state.push(Val::Num(v));
    Ok(3)
}

pub(super) fn colorselect_set_color_alpha(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let r = read_color_component(state, id, "__color_r", 1.0);
    let g = read_color_component(state, id, "__color_g", 1.0);
    let b = read_color_component(state, id, "__color_b", 1.0);
    let a = val_to_f64(stack_val(state, 2));
    write_color_components(state, id, (r, g, b, a));
    Ok(0)
}

pub(super) fn colorselect_get_color_alpha(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let alpha = read_color_component(state, id, "__color_a", 1.0);
    state.push(Val::Num(alpha));
    Ok(1)
}

macro_rules! colorselect_texture_methods {
    ($set_fn:ident, $get_fn:ident, $key:literal) => {
        pub(super) fn $set_fn(state: &mut LuaState) -> LuaResult<u32> {
            let id = frame_id_from_stack(state, 1)?;
            let value = stack_val(state, 2);
            set_colorselect_texture(state, id, $key, value)
        }
        pub(super) fn $get_fn(state: &mut LuaState) -> LuaResult<u32> {
            let id = frame_id_from_stack(state, 1)?;
            get_colorselect_texture(state, id, $key)
        }
    };
}

colorselect_texture_methods!(
    colorselect_set_color_alpha_texture,
    colorselect_get_color_alpha_texture,
    "ColorAlphaTexture"
);
colorselect_texture_methods!(
    colorselect_set_color_alpha_thumb_texture,
    colorselect_get_color_alpha_thumb_texture,
    "ColorAlphaThumbTexture"
);
colorselect_texture_methods!(
    colorselect_set_color_value_texture,
    colorselect_get_color_value_texture,
    "ColorValueTexture"
);
colorselect_texture_methods!(
    colorselect_set_color_value_thumb_texture,
    colorselect_get_color_value_thumb_texture,
    "ColorValueThumbTexture"
);
colorselect_texture_methods!(
    colorselect_set_color_wheel_texture,
    colorselect_get_color_wheel_texture,
    "ColorWheelTexture"
);
colorselect_texture_methods!(
    colorselect_set_color_wheel_thumb_texture,
    colorselect_get_color_wheel_thumb_texture,
    "ColorWheelThumbTexture"
);

pub(super) fn colorselect_clear_color_wheel_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    set_colorselect_texture(state, id, "ColorWheelTexture", Val::Nil)
}

pub(super) fn register_shared_value(
    state: &mut LuaState,
    metatable: GcRef<Table>,
) -> LuaResult<()> {
    table_set_rust_fn_static(state, metatable, "SetValue", shared_set_value)?;
    table_set_rust_fn_static(state, metatable, "GetValue", shared_get_value)?;
    table_set_rust_fn_static(
        state,
        metatable,
        "SetMinMaxValues",
        shared_set_min_max_values,
    )?;
    table_set_rust_fn_static(
        state,
        metatable,
        "GetMinMaxValues",
        shared_get_min_max_values,
    )?;
    Ok(())
}

pub(super) fn register_checkbutton(state: &mut LuaState, metatable: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(state, metatable, "SetChecked", checkbutton_set_checked)?;
    table_set_rust_fn_static(state, metatable, "GetChecked", checkbutton_get_checked)?;
    Ok(())
}

pub(super) fn register_colorselect(state: &mut LuaState, metatable: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn_static(state, metatable, "SetColorRGB", colorselect_set_color_rgb)?;
    table_set_rust_fn_static(state, metatable, "GetColorRGB", colorselect_get_color_rgb)?;
    table_set_rust_fn_static(state, metatable, "SetColorHSV", colorselect_set_color_hsv)?;
    table_set_rust_fn_static(state, metatable, "GetColorHSV", colorselect_get_color_hsv)?;
    table_set_rust_fn_static(
        state,
        metatable,
        "SetColorAlpha",
        colorselect_set_color_alpha,
    )?;
    table_set_rust_fn_static(
        state,
        metatable,
        "GetColorAlpha",
        colorselect_get_color_alpha,
    )?;
    table_set_rust_fn_static(
        state,
        metatable,
        "SetColorAlphaTexture",
        colorselect_set_color_alpha_texture,
    )?;
    table_set_rust_fn_static(
        state,
        metatable,
        "GetColorAlphaTexture",
        colorselect_get_color_alpha_texture,
    )?;
    table_set_rust_fn_static(
        state,
        metatable,
        "SetColorAlphaThumbTexture",
        colorselect_set_color_alpha_thumb_texture,
    )?;
    table_set_rust_fn_static(
        state,
        metatable,
        "GetColorAlphaThumbTexture",
        colorselect_get_color_alpha_thumb_texture,
    )?;
    table_set_rust_fn_static(
        state,
        metatable,
        "SetColorValueTexture",
        colorselect_set_color_value_texture,
    )?;
    table_set_rust_fn_static(
        state,
        metatable,
        "GetColorValueTexture",
        colorselect_get_color_value_texture,
    )?;
    table_set_rust_fn_static(
        state,
        metatable,
        "SetColorValueThumbTexture",
        colorselect_set_color_value_thumb_texture,
    )?;
    table_set_rust_fn_static(
        state,
        metatable,
        "GetColorValueThumbTexture",
        colorselect_get_color_value_thumb_texture,
    )?;
    table_set_rust_fn_static(
        state,
        metatable,
        "SetColorWheelTexture",
        colorselect_set_color_wheel_texture,
    )?;
    table_set_rust_fn_static(
        state,
        metatable,
        "GetColorWheelTexture",
        colorselect_get_color_wheel_texture,
    )?;
    table_set_rust_fn_static(
        state,
        metatable,
        "SetColorWheelThumbTexture",
        colorselect_set_color_wheel_thumb_texture,
    )?;
    table_set_rust_fn_static(
        state,
        metatable,
        "GetColorWheelThumbTexture",
        colorselect_get_color_wheel_thumb_texture,
    )?;
    table_set_rust_fn_static(
        state,
        metatable,
        "ClearColorWheelTexture",
        colorselect_clear_color_wheel_texture,
    )?;
    Ok(())
}

const SCROLLFRAME_METHODS: &[(&'static str, rilua::vm::closure::RustFn)] = &[
    ("GetHorizontalScroll", get_horizontal_scroll),
    ("SetHorizontalScroll", set_horizontal_scroll),
    ("GetHorizontalScrollRange", get_horizontal_scroll_range),
    ("GetVerticalScroll", get_vertical_scroll),
    ("SetVerticalScroll", set_vertical_scroll),
    ("GetVerticalScrollRange", get_vertical_scroll_range),
    ("GetScrollChild", get_scroll_child),
    ("SetScrollChild", set_scroll_child),
    ("UpdateScrollChildRect", update_scroll_child_rect),
];

pub(super) fn register_scrollframe(state: &mut LuaState, metatable: GcRef<Table>) -> LuaResult<()> {
    for (name, func) in SCROLLFRAME_METHODS {
        table_set_rust_fn(state, metatable, name, *func)?;
    }
    Ok(())
}
