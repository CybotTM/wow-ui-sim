//! Draw layer and shadow methods.

use super::super::shared::{opt_bool, opt_f32, opt_string};
use crate::lua_api::methods::{borrow_state, borrow_state_mut, create_string, frame_id_from_stack};
use crate::lua_bridge::{IntoStack, stack_val};
use crate::widget::Color;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn set_security_disable_set_text(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let disabled = opt_bool(state, 2).unwrap_or(false);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.editbox_security_disable_set_text = disabled;
    }
    Ok(0)
}

pub(super) fn set_draw_layer(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let Some(layer_name) = opt_string(state, 2) else {
        return Ok(0);
    };
    let Some(draw_layer) = crate::widget::DrawLayer::from_str(&layer_name) else {
        return Ok(0);
    };
    let sub_level = match stack_val(state, 3) {
        Val::Num(value) => Some(value as i32),
        _ => None,
    };
    let mut sim = borrow_state_mut(state)?;
    let mut changed = false;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        let layer_changed = frame.draw_layer != draw_layer;
        let sub_level_changed = sub_level.is_some_and(|value| frame.draw_sub_layer != value);
        if layer_changed {
            frame.draw_layer = draw_layer;
        }
        if let Some(sub_level) = sub_level
            && sub_level_changed
        {
            frame.draw_sub_layer = sub_level;
        }
        changed = layer_changed || sub_level_changed;
    }
    if changed {
        sim.invalidate_strata_buckets();
    }
    Ok(0)
}

pub(super) fn get_draw_layer(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let (layer_name, sub_level) = sim
        .widgets
        .get(id)
        .map(|f| (f.draw_layer.as_str(), f.draw_sub_layer))
        .unwrap_or(("ARTWORK", 0));
    drop(sim);
    let s = create_string(state, layer_name);
    state.push(s);
    state.push(Val::Num(sub_level as f64));
    Ok(2)
}

pub(super) fn set_shadow_offset(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let x = opt_f32(state, 2).unwrap_or(0.0);
    let y = opt_f32(state, 3).unwrap_or(0.0);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.shadow_offset = (x, y);
    }
    Ok(0)
}

pub(super) fn get_shadow_offset(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let (x, y) = sim
        .widgets
        .get(id)
        .map(|frame| frame.shadow_offset)
        .unwrap_or((0.0, 0.0));
    drop(sim);
    (x as f64, y as f64).into_stack(state)
}

pub(super) fn set_shadow_color(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let r = opt_f32(state, 2).unwrap_or(0.0);
    let g = opt_f32(state, 3).unwrap_or(0.0);
    let b = opt_f32(state, 4).unwrap_or(0.0);
    let a = opt_f32(state, 5).unwrap_or(1.0);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.shadow_color = Color::new(r, g, b, a);
    }
    Ok(0)
}

pub(super) fn get_shadow_color(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let color = sim
        .widgets
        .get(id)
        .map(|frame| frame.shadow_color)
        .unwrap_or_else(|| Color::new(0.0, 0.0, 0.0, 1.0));
    drop(sim);
    (
        color.r as f64,
        color.g as f64,
        color.b as f64,
        color.a as f64,
    )
        .into_stack(state)
}
