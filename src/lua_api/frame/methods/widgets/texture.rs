//! Texture / draw-layer widget methods.

use super::shared::{opt_bool, opt_f32, opt_string, rgba_from_stack, val_to_bool, val_to_f64};
use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, create_string, frame_id_from_stack, val_to_string,
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
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.atlas = Some(atlas_name);
        frame.texture = Some(lookup.info.file.to_string());
        frame.tex_coords = Some(tex_coords);
        frame.atlas_tex_coords = Some(tex_coords);
        if use_atlas_size {
            frame.set_size(lookup.info.width as f32, lookup.info.height as f32);
        }
    }
    Ok(0)
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
            frame.tex_coords = Some((coords[0], coords[1], coords[2], coords[3]));
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
            frame.tex_coords = Some((left, right, top, bottom));
            frame.tex_coords_quad = Some(quad);
        }
    }
    Ok(0)
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
    ("GetTextureSliceMargins", get_texture_slice_margins),
    ("GetTextureSliceMode", get_texture_slice_mode),
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
    // Tex coords + thickness
    ("SetTexCoord", set_tex_coord),
    ("GetTexCoord", get_tex_coord),
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
    ("SetSecurityDisableSetText", set_security_disable_set_text),
];

pub(super) fn register_texture(state: &mut LuaState, metatable: GcRef<Table>) -> LuaResult<()> {
    for (name, func) in TEXTURE_METHODS {
        table_set_rust_fn(state, metatable, name, *func)?;
    }
    Ok(())
}
