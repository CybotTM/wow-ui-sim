use super::super::super::text_attribute_event::callbacks;
use super::super::shared::val_to_f64;
use crate::lua_api::methods::{borrow_state, borrow_state_mut, frame_id_from_stack};
use crate::lua_bridge::stack_val;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn set_scroll_percentage(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let percentage = val_to_f64(stack_val(state, 2)).clamp(0.0, 1.0);
    set_scroll_percentage_value(state, id, percentage)
}

fn set_scroll_percentage_value(state: &mut LuaState, id: u64, percentage: f64) -> LuaResult<u32> {
    let changed = {
        let mut sim = borrow_state_mut(state)?;
        let Some(frame) = sim.widgets.get_mut_visual(id) else {
            return Ok(0);
        };
        if frame.slider_scroll_percentage == percentage {
            false
        } else {
            frame.slider_scroll_percentage = percentage;
            true
        }
    };
    if changed {
        callbacks::trigger_callback_event_for_frame(state, id, "OnScroll", &[Val::Num(percentage)]);
    }
    Ok(0)
}

pub(super) fn get_scroll_percentage(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let percentage = borrow_state(state)?
        .widgets
        .get(id)
        .map(|frame| frame.slider_scroll_percentage)
        .unwrap_or(0.0);
    state.push(Val::Num(percentage));
    Ok(1)
}

pub(super) fn set_visible_extent_percentage(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let percentage = val_to_f64(stack_val(state, 2)).clamp(0.0, 1.0);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.slider_visible_extent_percentage = percentage;
    }
    Ok(0)
}

pub(super) fn get_visible_extent_percentage(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let percentage = borrow_state(state)?
        .widgets
        .get(id)
        .map(|frame| frame.slider_visible_extent_percentage)
        .unwrap_or(0.0);
    state.push(Val::Num(percentage));
    Ok(1)
}

pub(super) fn set_pan_extent_percentage(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let percentage = val_to_f64(stack_val(state, 2)).clamp(0.0, 1.0);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.slider_pan_extent_percentage = percentage;
    }
    Ok(0)
}

pub(super) fn get_pan_extent_percentage(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let percentage = borrow_state(state)?
        .widgets
        .get(id)
        .map(|frame| frame.slider_pan_extent_percentage)
        .unwrap_or(0.0);
    state.push(Val::Num(percentage));
    Ok(1)
}

pub(super) fn scroll_step_in_direction(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let direction = val_to_f64(stack_val(state, 2)).signum();
    let (current, step) = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|frame| {
                (
                    frame.slider_scroll_percentage,
                    frame.slider_pan_extent_percentage.max(0.05),
                )
            })
            .unwrap_or((0.0, 0.05))
    };
    let target = (current + (direction * step)).clamp(0.0, 1.0);
    set_scroll_percentage_value(state, id, target)
}
