//! Slider, CheckButton, shared value (Slider+StatusBar), and ScrollFrame methods.

use super::shared::{opt_bool, opt_string, val_to_bool, val_to_f64};
use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, extract_frame_id, frame_id_from_stack, frame_ref,
    sync_child_to_rilua,
};
use crate::lua_bridge::{IntoStack, stack_val, table_set_rust_fn};
use crate::widget::WidgetType;
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
            apply_statusbar_value(&mut sim, id, value);
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
    let checked_tex_id = sim
        .widgets
        .get(id)
        .and_then(|f| f.children_keys.get("CheckedTexture").copied());
    if let Some(tex_id) = checked_tex_id {
        sim.set_frame_visible(tex_id, checked);
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
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.scroll_horizontal = offset.clamp(0.0, range);
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
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.scroll_vertical = offset.clamp(0.0, range);
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
    let mut sim = borrow_state_mut(state)?;
    let scroll_child_rect_size = sim
        .widgets
        .get(id)
        .and_then(|frame| frame.scroll_child_id)
        .and_then(|child_id| {
            sim.widgets
                .get(child_id)
                .map(|child| (child.width, child.height))
        });
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.scroll_child_rect_size = scroll_child_rect_size;
    }
    Ok(0)
}

// ---------------------------------------------------------------------------
// register_slider / register_shared_value / register_scrollframe / register_checkbutton
// ---------------------------------------------------------------------------

pub(super) fn register_slider(state: &mut LuaState, metatable: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn(state, metatable, "SetValueStep", set_value_step)?;
    table_set_rust_fn(state, metatable, "GetValueStep", get_value_step)?;
    table_set_rust_fn(state, metatable, "SetOrientation", set_orientation)?;
    table_set_rust_fn(state, metatable, "GetOrientation", get_orientation)?;
    table_set_rust_fn(state, metatable, "SetObeyStepOnDrag", set_obey_step_on_drag)?;
    table_set_rust_fn(state, metatable, "GetObeyStepOnDrag", get_obey_step_on_drag)?;
    table_set_rust_fn(state, metatable, "SetStepsPerPage", set_steps_per_page)?;
    table_set_rust_fn(state, metatable, "GetStepsPerPage", get_steps_per_page)?;
    table_set_rust_fn(state, metatable, "IsDraggingThumb", is_dragging_thumb)?;
    Ok(())
}

pub(super) fn register_shared_value(
    state: &mut LuaState,
    metatable: GcRef<Table>,
) -> LuaResult<()> {
    table_set_rust_fn(state, metatable, "SetValue", shared_set_value)?;
    table_set_rust_fn(state, metatable, "GetValue", shared_get_value)?;
    table_set_rust_fn(
        state,
        metatable,
        "SetMinMaxValues",
        shared_set_min_max_values,
    )?;
    table_set_rust_fn(
        state,
        metatable,
        "GetMinMaxValues",
        shared_get_min_max_values,
    )?;
    Ok(())
}

pub(super) fn register_checkbutton(state: &mut LuaState, metatable: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn(state, metatable, "SetChecked", checkbutton_set_checked)?;
    table_set_rust_fn(state, metatable, "GetChecked", checkbutton_get_checked)?;
    Ok(())
}

const SCROLLFRAME_METHODS: &[(&str, rilua::vm::closure::RustFn)] = &[
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
