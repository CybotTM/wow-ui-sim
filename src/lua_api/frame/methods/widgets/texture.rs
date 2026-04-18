//! Texture / draw-layer widget methods.

use super::shared::{opt_bool, opt_f32, opt_string, rgba_from_stack, val_to_bool, val_to_f64};
use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, create_string, frame_id_from_stack, table_get, val_to_string,
};
use crate::lua_bridge::{IntoStack, stack_val, table_set_rust_fn};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
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

pub(super) fn set_atlas(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let Some(atlas_name) = opt_string(state, 2) else {
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

pub(super) fn set_color_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let Some(color) = rgba_from_stack(state, 2) else {
        return Ok(0);
    };
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.color_texture = Some(color);
        frame.texture = None;
        frame.texture_file_data_id = None;
        frame.atlas = None;
        frame.atlas_tex_coords = None;
    }
    Ok(0)
}

pub(super) fn set_vertex_color(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let Some(color) = rgba_from_stack(state, 2) else {
        return Ok(0);
    };
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.vertex_color = Some(color);
    }
    Ok(0)
}

pub(super) fn get_vertex_color(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let (r, g, b, a) = borrow_state(state)?
        .widgets
        .get(id)
        .and_then(|frame| frame.vertex_color)
        .map(|c| (c.r as f64, c.g as f64, c.b as f64, c.a as f64))
        .unwrap_or((1.0, 1.0, 1.0, 1.0));
    (r, g, b, a).into_stack(state)
}

pub(super) fn set_desaturated(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let desaturated = val_to_bool(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.desaturated = desaturated;
    }
    Ok(0)
}

pub(super) fn is_desaturated(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let desaturated = borrow_state(state)?
        .widgets
        .get(id)
        .map(|frame| frame.desaturated)
        .unwrap_or(false);
    state.push(Val::Bool(desaturated));
    Ok(1)
}

pub(super) fn set_desaturation(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let desaturated = val_to_f64(stack_val(state, 2)) > 0.0;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.desaturated = desaturated;
    }
    Ok(0)
}

pub(super) fn get_desaturation(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let desaturation = if borrow_state(state)?
        .widgets
        .get(id)
        .map(|frame| frame.desaturated)
        .unwrap_or(false)
    {
        1.0
    } else {
        0.0
    };
    state.push(Val::Num(desaturation));
    Ok(1)
}

pub(super) fn set_blend_mode(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let Some(mode_name) = opt_string(state, 2) else {
        return Ok(0);
    };
    let normalized = mode_name.to_ascii_uppercase();
    let blend_mode = match normalized.as_str() {
        "ADD" => crate::BlendMode::Additive,
        _ => crate::BlendMode::Alpha,
    };
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.alpha_mode = Some(normalized);
        frame.blend_mode = blend_mode;
    }
    Ok(0)
}

pub(super) fn get_blend_mode(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mode = borrow_state(state)?
        .widgets
        .get(id)
        .and_then(|frame| frame.alpha_mode.clone())
        .unwrap_or_else(|| "BLEND".to_string());
    let mode_val = create_string(state, &mode);
    state.push(mode_val);
    Ok(1)
}

pub(super) fn set_tex_coord(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let coords: Vec<f32> = (2..=9).filter_map(|index| opt_f32(state, index)).collect();
    if coords.len() != 4 && coords.len() != 8 {
        return Ok(0);
    }
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        if coords.len() == 4 {
            let remapped = remap_tex_coords(
                frame.atlas_tex_coords,
                coords[0],
                coords[1],
                coords[2],
                coords[3],
            );
            frame.tex_coords = Some(remapped);
            frame.tex_coords_quad = None;
        } else {
            let quad = [
                coords[0], coords[1], coords[2], coords[3], coords[4], coords[5], coords[6],
                coords[7],
            ];
            let left = quad[0].min(quad[2]).min(quad[4]).min(quad[6]);
            let right = quad[0].max(quad[2]).max(quad[4]).max(quad[6]);
            let top = quad[1].min(quad[3]).min(quad[5]).min(quad[7]);
            let bottom = quad[1].max(quad[3]).max(quad[5]).max(quad[7]);
            frame.tex_coords = Some(remap_tex_coords(
                frame.atlas_tex_coords,
                left,
                right,
                top,
                bottom,
            ));
            frame.tex_coords_quad = Some(quad);
        }
    }
    Ok(0)
}

/// Remap UV coordinates into an atlas sub-region when one is active.
///
/// When an atlas is set, the caller's [0,1] UV space maps onto the atlas slot.
/// Without an atlas the coords pass through unchanged.
fn remap_tex_coords(
    atlas_tex_coords: Option<(f32, f32, f32, f32)>,
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
) -> (f32, f32, f32, f32) {
    if let Some((al, ar, at, ab)) = atlas_tex_coords {
        let aw = ar - al;
        let ah = ab - at;
        (al + left * aw, al + right * aw, at + top * ah, at + bottom * ah)
    } else {
        (left, right, top, bottom)
    }
}

pub(super) fn get_tex_coord(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    if let Some(frame) = sim.widgets.get(id) {
        if let Some(quad) = frame.tex_coords_quad {
            drop(sim);
            for value in quad {
                state.push(Val::Num(value as f64));
            }
            return Ok(8);
        }
        if let Some((left, right, top, bottom)) = frame.tex_coords {
            drop(sim);
            return (left as f64, right as f64, top as f64, bottom as f64).into_stack(state);
        }
    }
    drop(sim);
    (0.0, 1.0, 0.0, 1.0).into_stack(state)
}

pub(super) fn set_thickness(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let thickness = val_to_f64(stack_val(state, 2)) as f32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.line_thickness = thickness;
    }
    Ok(0)
}

pub(super) fn get_thickness(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let thickness = borrow_state(state)?
        .widgets
        .get(id)
        .map(|frame| frame.line_thickness as f64)
        .unwrap_or(1.0);
    state.push(Val::Num(thickness));
    Ok(1)
}

pub(super) fn set_horiz_tile(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let enabled = opt_bool(state, 2).unwrap_or(false);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.horiz_tile = enabled;
    }
    Ok(0)
}

pub(super) fn get_horiz_tile(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let enabled = borrow_state(state)?
        .widgets
        .get(id)
        .map(|frame| frame.horiz_tile)
        .unwrap_or(false);
    state.push(Val::Bool(enabled));
    Ok(1)
}

pub(super) fn set_vert_tile(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let enabled = opt_bool(state, 2).unwrap_or(false);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.vert_tile = enabled;
    }
    Ok(0)
}

pub(super) fn get_vert_tile(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let enabled = borrow_state(state)?
        .widgets
        .get(id)
        .map(|frame| frame.vert_tile)
        .unwrap_or(false);
    state.push(Val::Bool(enabled));
    Ok(1)
}

pub(super) fn set_texel_snapping_bias(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let bias = opt_f32(state, 2).unwrap_or(0.0);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.texel_snapping_bias = bias;
    }
    Ok(0)
}

pub(super) fn get_texel_snapping_bias(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let bias = borrow_state(state)?
        .widgets
        .get(id)
        .map(|frame| frame.texel_snapping_bias)
        .unwrap_or_default();
    state.push(Val::Num(bias as f64));
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

pub(super) fn get_texture_slice_margins(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let (left, right, top, bottom) = borrow_state(state)?
        .widgets
        .get(id)
        .map(|frame| frame.texture_slice_margins)
        .unwrap_or((0.0, 0.0, 0.0, 0.0));
    (left as f64, right as f64, top as f64, bottom as f64).into_stack(state)
}

pub(super) fn get_texture_slice_mode(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mode = borrow_state(state)?
        .widgets
        .get(id)
        .map(|frame| frame.texture_slice_mode)
        .unwrap_or_default();
    state.push(Val::Num(mode as f64));
    Ok(1)
}

pub(super) fn set_snap_to_pixel_grid(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let enabled = opt_bool(state, 2).unwrap_or(false);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.snap_to_pixel_grid = enabled;
    }
    Ok(0)
}

pub(super) fn is_snapping_to_pixel_grid(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let snapping = borrow_state(state)?
        .widgets
        .get(id)
        .map(|frame| frame.snap_to_pixel_grid)
        .unwrap_or(false);
    state.push(Val::Bool(snapping));
    Ok(1)
}

// ---------------------------------------------------------------------------
// Nine-slice setters (counterparts to existing getters)
// ---------------------------------------------------------------------------

pub(super) fn set_texture_slice_margins(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let left = opt_f32(state, 2).unwrap_or(0.0);
    let right = opt_f32(state, 3).unwrap_or(0.0);
    let top = opt_f32(state, 4).unwrap_or(0.0);
    let bottom = opt_f32(state, 5).unwrap_or(0.0);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.texture_slice_margins = (left, right, top, bottom);
    }
    Ok(0)
}

pub(super) fn set_texture_slice_mode(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mode = match stack_val(state, 2) {
        Val::Num(n) => n as i32,
        _ => 0,
    };
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.texture_slice_mode = mode;
    }
    Ok(0)
}

pub(super) fn clear_texture_slice(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.texture_slice_margins = (0.0, 0.0, 0.0, 0.0);
        frame.texture_slice_mode = 0;
    }
    Ok(0)
}

// ---------------------------------------------------------------------------
// Rotation
// ---------------------------------------------------------------------------

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

fn color_from_table(state: &mut LuaState, val: Val) -> crate::widget::Color {
    let r = f32_from_table_field(state, val, "r");
    let g = f32_from_table_field(state, val, "g");
    let b = f32_from_table_field(state, val, "b");
    let a = f32_from_table_field_or(state, val, "a", 1.0);
    crate::widget::Color::new(r, g, b, a)
}

fn f32_from_table_field(state: &mut LuaState, table: Val, key: &str) -> f32 {
    match table_get(state, table, key) {
        Val::Num(n) => n as f32,
        _ => 0.0,
    }
}

fn f32_from_table_field_or(state: &mut LuaState, table: Val, key: &str, default: f32) -> f32 {
    match table_get(state, table, key) {
        Val::Num(n) => n as f32,
        _ => default,
    }
}

// ---------------------------------------------------------------------------
// SetCenterColor — no-op (matches master)
// ---------------------------------------------------------------------------

pub(super) fn set_center_color(state: &mut LuaState) -> LuaResult<u32> {
    let _ = frame_id_from_stack(state, 1);
    Ok(0)
}

// ---------------------------------------------------------------------------
// SetVisuals — no-op (matches master)
// ---------------------------------------------------------------------------

pub(super) fn set_visuals(state: &mut LuaState) -> LuaResult<u32> {
    let _ = frame_id_from_stack(state, 1);
    Ok(0)
}

// ---------------------------------------------------------------------------
// SetSpriteSheetCell — no-op stub (not implemented on master)
// ---------------------------------------------------------------------------

pub(super) fn set_sprite_sheet_cell(state: &mut LuaState) -> LuaResult<u32> {
    let _ = frame_id_from_stack(state, 1);
    Ok(0)
}

// ---------------------------------------------------------------------------
// Vertex offsets
// ---------------------------------------------------------------------------

pub(super) fn set_vertex_offset(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let index = match stack_val(state, 2) {
        Val::Num(n) => n as usize,
        _ => return Ok(0),
    };
    if index == 0 || index > 4 {
        return Ok(0);
    }
    let x = opt_f32(state, 3).unwrap_or(0.0);
    let y = opt_f32(state, 4).unwrap_or(0.0);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        let offsets = frame.vertex_offsets.get_or_insert([(0.0, 0.0); 4]);
        offsets[index - 1] = (x, y);
    }
    Ok(0)
}

pub(super) fn get_vertex_offset(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let index = match stack_val(state, 2) {
        Val::Num(n) => n as usize,
        _ => {
            state.push(Val::Num(0.0));
            state.push(Val::Num(0.0));
            return Ok(2);
        }
    };
    if index == 0 || index > 4 {
        state.push(Val::Num(0.0));
        state.push(Val::Num(0.0));
        return Ok(2);
    }
    let (x, y) = borrow_state(state)?
        .widgets
        .get(id)
        .and_then(|frame| frame.vertex_offsets)
        .map(|offsets| offsets[index - 1])
        .unwrap_or((0.0, 0.0));
    state.push(Val::Num(x as f64));
    state.push(Val::Num(y as f64));
    Ok(2)
}

pub(super) fn clear_vertex_offsets(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.vertex_offsets = None;
    }
    Ok(0)
}

// ---------------------------------------------------------------------------
// ResetTexCoord
// ---------------------------------------------------------------------------

pub(super) fn reset_tex_coord(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.tex_coords = frame.atlas_tex_coords;
        frame.tex_coords_quad = None;
    }
    Ok(0)
}

// ---------------------------------------------------------------------------
// Blocking loads
// ---------------------------------------------------------------------------

pub(super) fn set_blocking_loads_requested(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let blocking = val_to_bool(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.blocking_loads_requested = blocking;
    }
    Ok(0)
}

pub(super) fn is_blocking_load_requested(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let blocking = borrow_state(state)?
        .widgets
        .get(id)
        .map(|frame| frame.blocking_loads_requested)
        .unwrap_or(false);
    state.push(Val::Bool(blocking));
    Ok(1)
}

// ---------------------------------------------------------------------------
// register_texture
// ---------------------------------------------------------------------------

const TEXTURE_METHODS: &[(&str, rilua::vm::closure::RustFn)] = &[
    // Draw layer + shadow
    ("SetDrawLayer", set_draw_layer),
    ("GetDrawLayer", get_draw_layer),
    ("SetShadowOffset", set_shadow_offset),
    ("GetShadowOffset", get_shadow_offset),
    ("SetShadowColor", set_shadow_color),
    ("GetShadowColor", get_shadow_color),
    // Nine-slice
    ("SetTextureSliceMargins", set_texture_slice_margins),
    ("GetTextureSliceMargins", get_texture_slice_margins),
    ("SetTextureSliceMode", set_texture_slice_mode),
    ("GetTextureSliceMode", get_texture_slice_mode),
    ("ClearTextureSlice", clear_texture_slice),
    // Atlas / texture source
    ("SetAtlas", set_atlas),
    ("GetAtlas", get_atlas),
    ("SetTexture", set_texture),
    ("GetTexture", get_texture),
    ("GetTextureFileID", get_texture_file_id),
    ("GetTextureFilePath", get_texture_file_path),
    // Desaturation
    ("SetDesaturated", set_desaturated),
    ("IsDesaturated", is_desaturated),
    ("SetDesaturation", set_desaturation),
    ("GetDesaturation", get_desaturation),
    // Color + blend
    ("SetColorTexture", set_color_texture),
    ("SetVertexColor", set_vertex_color),
    ("GetVertexColor", get_vertex_color),
    ("SetBlendMode", set_blend_mode),
    ("GetBlendMode", get_blend_mode),
    // Gradient + center color
    ("SetGradient", set_gradient),
    ("SetCenterColor", set_center_color),
    // Rotation
    ("SetRotation", set_rotation),
    ("GetRotation", get_rotation),
    // Mask
    ("SetMask", set_mask),
    // Tex coords + thickness
    ("SetTexCoord", set_tex_coord),
    ("GetTexCoord", get_tex_coord),
    ("ResetTexCoord", reset_tex_coord),
    ("SetThickness", set_thickness),
    ("GetThickness", get_thickness),
    // Tiling
    ("SetHorizTile", set_horiz_tile),
    ("GetHorizTile", get_horiz_tile),
    ("SetVertTile", set_vert_tile),
    ("GetVertTile", get_vert_tile),
    // Pixel snapping + security
    ("SetTexelSnappingBias", set_texel_snapping_bias),
    ("GetTexelSnappingBias", get_texel_snapping_bias),
    ("SetSnapToPixelGrid", set_snap_to_pixel_grid),
    ("IsSnappingToPixelGrid", is_snapping_to_pixel_grid),
    ("SetSecurityDisableSetText", set_security_disable_set_text),
    // Visuals
    ("SetVisuals", set_visuals),
    // Sprite sheet
    ("SetSpriteSheetCell", set_sprite_sheet_cell),
    // Vertex offsets
    ("SetVertexOffset", set_vertex_offset),
    ("GetVertexOffset", get_vertex_offset),
    ("ClearVertexOffsets", clear_vertex_offsets),
    // Blocking loads
    ("SetBlockingLoadsRequested", set_blocking_loads_requested),
    ("IsBlockingLoadRequested", is_blocking_load_requested),
];

pub(super) fn register_texture(state: &mut LuaState, metatable: GcRef<Table>) -> LuaResult<()> {
    for (name, func) in TEXTURE_METHODS {
        table_set_rust_fn(state, metatable, name, *func)?;
    }
    Ok(())
}
