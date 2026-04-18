//! Draw layer and shadow methods.

use super::super::shared::{opt_bool, opt_string};
use crate::lua_api::methods::{borrow_state, borrow_state_mut, create_string, frame_id_from_stack};
use crate::lua_bridge::{IntoStack, stack_val};
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
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.draw_layer = draw_layer;
        if let Some(sub_level) = sub_level {
            frame.draw_sub_layer = sub_level;
        }
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
    let _ = frame_id_from_stack(state, 1);
    Ok(0)
}

pub(super) fn get_shadow_offset(state: &mut LuaState) -> LuaResult<u32> {
    let _ = frame_id_from_stack(state, 1);
    (0.0_f64, 0.0_f64).into_stack(state)
}

pub(super) fn set_shadow_color(state: &mut LuaState) -> LuaResult<u32> {
    let _ = frame_id_from_stack(state, 1);
    Ok(0)
}

pub(super) fn get_shadow_color(state: &mut LuaState) -> LuaResult<u32> {
    let _ = frame_id_from_stack(state, 1);
    (0.0_f64, 0.0_f64, 0.0_f64, 1.0_f64).into_stack(state)
}
