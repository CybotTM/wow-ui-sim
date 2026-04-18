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
    let use_atlas_size = opt_bool(state, 3).unwrap_or(false);
    let mut sim = borrow_state_mut(state)?;
    apply_atlas(&mut sim.widgets, id, &atlas_name, lookup.info, use_atlas_size);
    Ok(0)
}

/// Write the atlas onto the child frame, then mirror the slot into the
/// parent button's matching texture slot when applicable.
fn apply_atlas(
    widgets: &mut crate::widget::WidgetRegistry,
    id: u64,
    atlas_name: &str,
    info: &crate::atlas::AtlasInfo,
    use_atlas_size: bool,
) {
    let tex_coords = atlas_slot_tex_coords(info);
    let parent_info = collect_parent_slot(widgets, id);
    apply_atlas_to_frame(widgets, id, atlas_name, info, tex_coords, use_atlas_size);
    if let Some((parent_id, parent_key)) = parent_info {
        propagate_atlas_to_button_slot(
            widgets,
            parent_id,
            &parent_key,
            info.file.to_string(),
            tex_coords,
        );
    }
}

/// Atlas slot UV box `(left, right, top, bottom)` for the matched atlas entry.
fn atlas_slot_tex_coords(info: &crate::atlas::AtlasInfo) -> (f32, f32, f32, f32) {
    (
        info.left_tex_coord,
        info.right_tex_coord,
        info.top_tex_coord,
        info.bottom_tex_coord,
    )
}

/// Parent id + parentKey when both are set. Captured before the child borrow
/// so the propagation step can run after the child mutation without
/// re-borrowing state.
fn collect_parent_slot(
    widgets: &crate::widget::WidgetRegistry,
    id: u64,
) -> Option<(u64, String)> {
    let frame = widgets.get(id)?;
    let parent_id = frame.parent_id?;
    let parent_key = frame.parent_key.clone()?;
    Some((parent_id, parent_key))
}

/// Write atlas name, source texture, and atlas UVs into the child frame.
/// When `use_atlas_size` is true, also resize the frame to the slot dimensions.
fn apply_atlas_to_frame(
    widgets: &mut crate::widget::WidgetRegistry,
    id: u64,
    atlas_name: &str,
    info: &crate::atlas::AtlasInfo,
    tex_coords: (f32, f32, f32, f32),
    use_atlas_size: bool,
) {
    let Some(frame) = widgets.get_mut_visual(id) else {
        return;
    };
    frame.atlas = Some(atlas_name.to_string());
    frame.texture = Some(info.file.to_string());
    frame.tex_coords = Some(tex_coords);
    frame.atlas_tex_coords = Some(tex_coords);
    if use_atlas_size {
        frame.set_size(info.width as f32, info.height as f32);
    }
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
