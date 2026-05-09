//! Helpers shared across all widget method submodules.

use crate::lua_api::methods::{table_get, val_to_string};
use crate::lua_bridge::stack_val;
use rilua::Val;
use rilua::vm::state::LuaState;

pub(super) fn val_to_f64(val: Val) -> f64 {
    match val {
        Val::Num(n) => n,
        _ => 0.0,
    }
}

pub(super) fn val_to_bool(val: Val) -> bool {
    matches!(val, Val::Bool(true))
}

pub(super) fn opt_string(state: &LuaState, index: i32) -> Option<String> {
    val_to_string(state, stack_val(state, index))
}

pub(super) fn opt_bool(state: &LuaState, index: i32) -> Option<bool> {
    match stack_val(state, index) {
        Val::Bool(value) => Some(value),
        _ => None,
    }
}

pub(super) fn opt_f32(state: &LuaState, index: i32) -> Option<f32> {
    match stack_val(state, index) {
        Val::Num(value) => Some(value as f32),
        _ => None,
    }
}

pub(super) fn rgba_from_stack(state: &mut LuaState, start: i32) -> Option<crate::widget::Color> {
    if let Val::Table(_) = stack_val(state, start) {
        let color = stack_val(state, start);
        let r = f32_from_table_field(state, color, "r")?;
        let g = f32_from_table_field(state, color, "g")?;
        let b = f32_from_table_field(state, color, "b")?;
        let a = opt_f32(state, start + 1)
            .or_else(|| f32_from_table_field(state, color, "a"))
            .unwrap_or(1.0);
        return Some(crate::widget::Color::new(r, g, b, a));
    }

    let r = opt_f32(state, start)?;
    let g = opt_f32(state, start + 1)?;
    let b = opt_f32(state, start + 2)?;
    let a = opt_f32(state, start + 3).unwrap_or(1.0);
    Some(crate::widget::Color::new(r, g, b, a))
}

fn f32_from_table_field(state: &mut LuaState, table: Val, key: &str) -> Option<f32> {
    match table_get(state, table, key) {
        Val::Num(value) => Some(value as f32),
        _ => None,
    }
}

pub(super) fn animation_group_id_for_frame(
    sim: &crate::lua_api::SimState,
    frame_id: u64,
) -> Option<u64> {
    sim.anim_frame_to_group.get(&frame_id).copied().or_else(|| {
        sim.anim_frame_to_anim
            .get(&frame_id)
            .map(|(group_id, _)| *group_id)
    })
}
