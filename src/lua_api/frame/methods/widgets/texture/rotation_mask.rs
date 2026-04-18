//! Rotation, mask, gradient, visuals, and sprite sheet methods.

use super::super::shared::{opt_f32, opt_string};
use super::color::color_from_table;
use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, call_function_state, create_table, frame_id_from_stack,
    get_or_create_frame_fields, table_get, table_set,
};
use crate::lua_bridge::stack_val;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn set_rotation(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let radians = opt_f32(state, 2).unwrap_or(0.0);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.rotation = radians;
    }
    Ok(0)
}

pub(super) fn get_rotation(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let radians = borrow_state(state)?
        .widgets
        .get(id)
        .map(|frame| frame.rotation as f64)
        .unwrap_or(0.0);
    state.push(Val::Num(radians));
    Ok(1)
}

// ---------------------------------------------------------------------------
// SetMask — no-op stub (not implemented on master either)
// ---------------------------------------------------------------------------

pub(super) fn set_mask(state: &mut LuaState) -> LuaResult<u32> {
    let _ = frame_id_from_stack(state, 1);
    Ok(0)
}

// ---------------------------------------------------------------------------
// SetGradient
// ---------------------------------------------------------------------------

pub(super) fn set_gradient(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let orientation = opt_string(state, 2).unwrap_or_else(|| "VERTICAL".to_string());
    let vertical = orientation.to_ascii_uppercase() != "HORIZONTAL";
    let min_val = stack_val(state, 3);
    let max_val = stack_val(state, 4);
    let min_color = color_from_table(state, min_val);
    let max_color = color_from_table(state, max_val);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.gradient = Some(crate::widget::Gradient {
            vertical,
            min_color,
            max_color,
        });
    }
    Ok(0)
}

// ---------------------------------------------------------------------------
// SetVisuals — no-op (matches master)
// ---------------------------------------------------------------------------

pub(super) fn set_visuals(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let fields = get_or_create_frame_fields(state, id);
    let override_fn = table_get(state, fields, "SetVisuals");
    if matches!(override_fn, Val::Function(_)) {
        let arg_count = state.top.saturating_sub(state.base) as i32;
        let args: Vec<Val> = (1..=arg_count)
            .map(|index| stack_val(state, index))
            .collect();
        let _ = call_function_state(state, override_fn, &args)?;
        return Ok(0);
    }
    let visual_args = create_table(state);
    if let Val::Table(table_ref) = visual_args {
        let arg_count = state.top.saturating_sub(state.base) as i32;
        let values: Vec<Val> = (2..=arg_count)
            .map(|index| stack_val(state, index))
            .collect();
        if let Some(table) = state.gc.tables.get_mut(table_ref) {
            for (offset, value) in values.into_iter().enumerate() {
                let key = (offset + 1) as f64;
                let _ = table.raw_set(Val::Num(key), value, &state.gc.string_arena);
            }
        }
        state.gc.barrier_back(table_ref);
    }
    table_set(state, fields, "visualArgs", visual_args);
    Ok(0)
}

// ---------------------------------------------------------------------------
// SetSpriteSheetCell — no-op stub (not implemented on master)
// ---------------------------------------------------------------------------

pub(super) fn set_sprite_sheet_cell(state: &mut LuaState) -> LuaResult<u32> {
    let _ = frame_id_from_stack(state, 1);
    Ok(0)
}
