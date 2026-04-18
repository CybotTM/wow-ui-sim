//! Atlas and texture-resolve methods.

use super::super::shared::opt_bool;
use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, create_string, frame_id_from_stack, val_to_string,
};
use crate::lua_bridge::stack_val;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn set_atlas(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let Some(atlas_name) = super::super::shared::opt_string(state, 2) else {
        return Ok(0);
    };
    let Some(lookup) = crate::atlas::get_atlas_info(&atlas_name) else {
        return Ok(0);
    };
    let tex_coords = (
        lookup.info.left_tex_coord,
        lookup.info.right_tex_coord,
        lookup.info.top_tex_coord,
        lookup.info.bottom_tex_coord,
    );
    let use_atlas_size = opt_bool(state, 3).unwrap_or(false);
    let mut sim = borrow_state_mut(state)?;

    // Collect parent info before mutably borrowing the child.
    let parent_info: Option<(u64, String)> = sim.widgets.get(id).and_then(|f| {
        let pid = f.parent_id?;
        let key = f.parent_key.clone()?;
        Some((pid, key))
    });

    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.atlas = Some(atlas_name.clone());
        frame.texture = Some(lookup.info.file.to_string());
        frame.tex_coords = Some(tex_coords);
        frame.atlas_tex_coords = Some(tex_coords);
        if use_atlas_size {
            frame.set_size(lookup.info.width as f32, lookup.info.height as f32);
        }
    }

    // Propagate to the parent button's texture slot when parentKey matches a
    // standard slot name and the parent is a Button or CheckButton.
    if let Some((parent_id, ref parent_key)) = parent_info {
        propagate_atlas_to_button_slot(
            &mut sim.widgets,
            parent_id,
            parent_key,
            lookup.info.file.to_string(),
            tex_coords,
        );
    }

    Ok(0)
}

/// Copy atlas texture/UV data from a child texture onto the parent Button's
/// corresponding slot field when `parent_key` is one of the four standard names.
fn propagate_atlas_to_button_slot(
    widgets: &mut crate::widget::WidgetRegistry,
    parent_id: u64,
    parent_key: &str,
    texture_path: String,
    tex_coords: (f32, f32, f32, f32),
) {
    let Some(parent) = widgets.get_mut_visual(parent_id) else {
        return;
    };
    if !matches!(
        parent.widget_type,
        crate::widget::WidgetType::Button | crate::widget::WidgetType::CheckButton
    ) {
        return;
    }
    match parent_key {
        "NormalTexture" => {
            parent.normal_texture = Some(texture_path);
            parent.normal_tex_coords = Some(tex_coords);
        }
        "PushedTexture" => {
            parent.pushed_texture = Some(texture_path);
            parent.pushed_tex_coords = Some(tex_coords);
        }
        "HighlightTexture" => {
            parent.highlight_texture = Some(texture_path);
            parent.highlight_tex_coords = Some(tex_coords);
        }
        "DisabledTexture" => {
            parent.disabled_texture = Some(texture_path);
            parent.disabled_tex_coords = Some(tex_coords);
        }
        _ => {}
    }
}

pub(super) fn set_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let texture_val = stack_val(state, 2);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        let Some((path, file_data_id)) = resolve_texture_value(state, texture_val) else {
            return Ok(0);
        };
        frame.texture = path;
        frame.texture_file_data_id = file_data_id;
        frame.color_texture = None;
        frame.atlas = None;
        frame.atlas_tex_coords = None;
    }
    Ok(0)
}

fn resolve_texture_value(state: &LuaState, value: Val) -> Option<(Option<String>, Option<i64>)> {
    match value {
        Val::Str(_) => Some(resolve_texture_string(state, value)),
        Val::Num(number) if number == 0.0 => Some((None, None)),
        Val::Num(number) => {
            let file_data_id = number as u32;
            Some((
                Some(resolve_file_data_id_path(file_data_id)),
                Some(file_data_id as i64),
            ))
        }
        Val::Nil => Some((None, None)),
        _ => None,
    }
}

fn resolve_texture_string(state: &LuaState, value: Val) -> (Option<String>, Option<i64>) {
    let Some(raw) = val_to_string(state, value) else {
        return (None, None);
    };
    let Ok(file_data_id) = raw.parse::<u32>() else {
        return (Some(raw), None);
    };
    (
        Some(resolve_file_data_id_path(file_data_id)),
        Some(file_data_id as i64),
    )
}

fn resolve_file_data_id_path(file_data_id: u32) -> String {
    crate::manifest_interface_data::get_texture_path(file_data_id)
        .map(|path| format!("Interface\\{}", path.replace('/', "\\")))
        .unwrap_or_else(|| file_data_id.to_string())
}

pub(super) fn get_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let (file_id, path) = {
        let sim = borrow_state(state)?;
        let frame = sim.widgets.get(id);
        (
            frame.and_then(|frame| frame.texture_file_data_id),
            frame.and_then(|frame| frame.texture.clone()),
        )
    };
    let value = if let Some(file_id) = file_id {
        Val::Num(file_id as f64)
    } else if let Some(path) = path {
        create_string(state, &path)
    } else {
        Val::Nil
    };
    state.push(value);
    Ok(1)
}

pub(super) fn get_texture_file_id(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let file_id = borrow_state(state)?
        .widgets
        .get(id)
        .and_then(|frame| frame.texture_file_data_id);
    match file_id {
        Some(file_id) => state.push(Val::Num(file_id as f64)),
        None => state.push(Val::Nil),
    }
    Ok(1)
}

pub(super) fn get_texture_file_path(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let path = borrow_state(state)?
        .widgets
        .get(id)
        .and_then(|frame| frame.texture.clone());
    match path {
        Some(path) => {
            let path_val = create_string(state, &path);
            state.push(path_val);
        }
        None => state.push(Val::Nil),
    }
    Ok(1)
}

pub(super) fn get_atlas(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let atlas = borrow_state(state)?
        .widgets
        .get(id)
        .and_then(|frame| frame.atlas.clone());
    match atlas {
        Some(atlas) => {
            let atlas_val = create_string(state, &atlas);
            state.push(atlas_val);
        }
        None => state.push(Val::Nil),
    }
    Ok(1)
}
