//! Compatibility storage for 12.1 TextureBase radial progress APIs.

use crate::lua_api::methods::{
    create_string, frame_id_from_stack, get_or_create_frame_fields, table_get, table_set,
};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

fn fields(state: &mut LuaState) -> LuaResult<Val> {
    let id = frame_id_from_stack(state, 1)?;
    Ok(get_or_create_frame_fields(state, id))
}

fn stack_number(state: &mut LuaState, index: i32) -> f64 {
    match crate::lua_bridge::stack_val(state, index) {
        Val::Num(value) => value,
        _ => 0.0,
    }
}

fn set_number(state: &mut LuaState, key: &'static str) -> LuaResult<u32> {
    let fields = fields(state)?;
    let value = stack_number(state, 2);
    table_set(state, fields, key, Val::Num(value));
    Ok(0)
}

fn get_number(state: &mut LuaState, key: &'static str, default: f64) -> LuaResult<u32> {
    let fields = fields(state)?;
    match table_get(state, fields, key) {
        Val::Num(value) => state.push(Val::Num(value)),
        _ => state.push(Val::Num(default)),
    }
    Ok(1)
}

pub(super) fn clear_radial_progress_bar(state: &mut LuaState) -> LuaResult<u32> {
    let fields = fields(state)?;
    for key in [
        "__radialProgressPercent",
        "__radialProgressStartOffset",
        "__radialProgressEndOffset",
        "__radialProgressFeather",
        "__radialProgressReverse",
    ] {
        table_set(state, fields, key, Val::Nil);
    }
    Ok(0)
}

pub(super) fn set_radial_progress_bar_percent(state: &mut LuaState) -> LuaResult<u32> {
    set_number(state, "__radialProgressPercent")
}

pub(super) fn get_radial_progress_bar_percent(state: &mut LuaState) -> LuaResult<u32> {
    get_number(state, "__radialProgressPercent", 0.0)
}

pub(super) fn set_radial_progress_bar_start_offset(state: &mut LuaState) -> LuaResult<u32> {
    set_number(state, "__radialProgressStartOffset")
}

pub(super) fn get_radial_progress_bar_start_offset(state: &mut LuaState) -> LuaResult<u32> {
    get_number(state, "__radialProgressStartOffset", 0.0)
}

pub(super) fn set_radial_progress_bar_end_offset(state: &mut LuaState) -> LuaResult<u32> {
    set_number(state, "__radialProgressEndOffset")
}

pub(super) fn get_radial_progress_bar_end_offset(state: &mut LuaState) -> LuaResult<u32> {
    get_number(state, "__radialProgressEndOffset", 1.0)
}

pub(super) fn set_radial_progress_bar_feather(state: &mut LuaState) -> LuaResult<u32> {
    set_number(state, "__radialProgressFeather")
}

pub(super) fn get_radial_progress_bar_feather(state: &mut LuaState) -> LuaResult<u32> {
    get_number(state, "__radialProgressFeather", 0.0)
}

pub(super) fn set_radial_progress_bar_reverse(state: &mut LuaState) -> LuaResult<u32> {
    let fields = fields(state)?;
    table_set(
        state,
        fields,
        "__radialProgressReverse",
        Val::Bool(matches!(
            crate::lua_bridge::stack_val(state, 2),
            Val::Bool(true)
        )),
    );
    Ok(0)
}

pub(super) fn get_radial_progress_bar_reverse(state: &mut LuaState) -> LuaResult<u32> {
    let fields = fields(state)?;
    match table_get(state, fields, "__radialProgressReverse") {
        Val::Bool(value) => state.push(Val::Bool(value)),
        _ => state.push(Val::Bool(false)),
    }
    Ok(1)
}

pub(super) fn set_visual_radial_progress_bar_mode(state: &mut LuaState) -> LuaResult<u32> {
    let fields = fields(state)?;
    let mode = create_string(state, "radialProgressBar");
    table_set(state, fields, "__visualMode", mode);
    Ok(0)
}

pub(super) fn clear_svg(state: &mut LuaState) -> LuaResult<u32> {
    let fields = fields(state)?;
    table_set(state, fields, "__svgFileID", Val::Nil);
    Ok(0)
}

pub(super) fn set_svg(state: &mut LuaState) -> LuaResult<u32> {
    let fields = fields(state)?;
    let file_id = match crate::lua_bridge::stack_val(state, 2) {
        Val::Num(value) => Val::Num(value),
        Val::Str(value) => Val::Str(value),
        _ => Val::Nil,
    };
    table_set(state, fields, "__svgFileID", file_id);
    Ok(0)
}

pub(super) fn get_svg_file_id(state: &mut LuaState) -> LuaResult<u32> {
    let fields = fields(state)?;
    let file_id = table_get(state, fields, "__svgFileID");
    state.push(file_id);
    Ok(1)
}

pub(super) fn has_svg(state: &mut LuaState) -> LuaResult<u32> {
    let fields = fields(state)?;
    let file_id = table_get(state, fields, "__svgFileID");
    state.push(Val::Bool(!matches!(file_id, Val::Nil)));
    Ok(1)
}
