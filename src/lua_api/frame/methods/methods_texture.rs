//! Texture-related methods: SetTexture, SetAtlas, SetTexCoord, etc.

use super::super::handle::FrameRef;
use super::methods_helpers::resolve_file_data_id_or_path;
use crate::lua_api::frame::handle::{extract_frame_id, frame_ref, get_sim_state};
use crate::widget::{Frame, WidgetType};
use mlua::Value;

/// Add texture-related methods to the shared methods table.
pub fn add_texture_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_texture_path_methods(methods);
    add_tiling_methods(methods);
    add_blend_and_desaturation_methods(methods);
    add_atlas_methods(methods);
    add_pixel_grid_methods(methods);
    add_nine_slice_methods(methods);
    add_vertex_color_methods(methods);
    add_tex_coord_methods(methods);
    add_mask_methods(methods);
    add_rotation_methods(methods);
    add_draw_layer_methods(methods);
    add_visual_methods(methods);
}

/// SetTexture, GetTexture, SetColorTexture.
fn add_texture_path_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_set_texture(methods);
    add_get_texture(methods);
    add_set_color_texture(methods);
}

/// Extract numeric file data ID from a Lua value (for GetTexture round-trip).
fn extract_file_data_id(value: &Value) -> Option<i64> {
    match value {
        Value::Integer(n) => Some(*n),
        Value::Number(n) => Some(*n as i64),
        Value::String(s) => s.to_str().ok().and_then(|s| s.parse::<i64>().ok()),
        _ => None,
    }
}

fn add_set_texture<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetTexture", |lua, this, args: mlua::MultiValue| {
        let id = this.0;
        let args_vec: Vec<Value> = args.into_iter().collect();
        let file_data_id = args_vec.first().and_then(extract_file_data_id);
        let path = args_vec.first().map(resolve_file_data_id_or_path).unwrap_or(None);
        let horiz_tile = args_vec.get(1).and_then(|v| if let Value::Boolean(b) = v { Some(*b) } else { None });
        let vert_tile = args_vec.get(2).and_then(|v| if let Value::Boolean(b) = v { Some(*b) } else { None });
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(id) {
            frame.texture = path;
            frame.texture_file_data_id = file_data_id;
            if let Some(h) = horiz_tile { frame.horiz_tile = h; }
            if let Some(v) = vert_tile { frame.vert_tile = v; }
        }
        Ok(())
    });
}

fn add_get_texture<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetTexture", |lua, this, ()| {
        let id = this.0;
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let frame = state.widgets.get(id);
        if let Some(fid) = frame.and_then(|f| f.texture_file_data_id) {
            return Ok(Value::Integer(fid));
        }
        let texture = frame.and_then(|f| f.texture.clone());
        match texture {
            Some(ref s) if s.parse::<i64>().is_ok() => Ok(Value::Integer(s.parse::<i64>().unwrap())),
            Some(s) => Ok(Value::String(lua.create_string(&s)?)),
            None => Ok(Value::Nil),
        }
    });
}

fn add_set_color_texture<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetColorTexture", |lua, this, (r, g, b, a): (f32, f32, f32, Option<f32>)| {
        let id = this.0;
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(id) {
            frame.color_texture = Some(crate::widget::Color::new(r, g, b, a.unwrap_or(1.0)));
            frame.texture = None;
            frame.texture_file_data_id = None;
        }
        Ok(())
    });
}

/// SetHorizTile, GetHorizTile, SetVertTile, GetVertTile.
fn add_tiling_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetHorizTile", |lua, this, tile: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.horiz_tile = tile;
        }
        Ok(())
    });
    methods.add_method("GetHorizTile", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state.widgets.get(this.0).map(|f| f.horiz_tile).unwrap_or(false))
    });
    methods.add_method("SetVertTile", |lua, this, tile: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.vert_tile = tile;
        }
        Ok(())
    });
    methods.add_method("GetVertTile", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state.widgets.get(this.0).map(|f| f.vert_tile).unwrap_or(false))
    });
}

/// SetBlendMode, GetBlendMode, SetDesaturated, IsDesaturated.
fn add_blend_and_desaturation_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_blend_mode_methods(methods);
    add_desaturation_methods(methods);
}

fn add_blend_mode_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetBlendMode", |lua, this, mode: Option<String>| {
        let blend = match mode.as_deref() {
            Some("ADD") => crate::render::BlendMode::Additive,
            _ => crate::render::BlendMode::Alpha,
        };
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(f) = state.widgets.get_mut_visual(this.0) {
            f.blend_mode = blend;
        }
        Ok(())
    });
    methods.add_method("GetBlendMode", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(match state.widgets.get(this.0).map(|f| f.blend_mode) {
            Some(crate::render::BlendMode::Additive) => "ADD",
            _ => "BLEND",
        })
    });
}

fn add_desaturation_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetDesaturated", |lua, this, desaturated: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(f) = state.widgets.get_mut_visual(this.0) {
            f.desaturated = desaturated;
        }
        Ok(())
    });
    methods.add_method("IsDesaturated", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state.widgets.get(this.0).map(|f| f.desaturated).unwrap_or(false))
    });
    methods.add_method("GetDesaturation", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(if state.widgets.get(this.0).map(|f| f.desaturated).unwrap_or(false) { 1.0_f64 } else { 0.0 })
    });
    methods.add_method("SetDesaturation", |lua, this, desat: f64| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(f) = state.widgets.get_mut_visual(this.0) {
            f.desaturated = desat > 0.0;
        }
        Ok(())
    });
}

/// Resolve atlas name from a Lua value (string or numeric element ID).
fn resolve_atlas_name(value: &Value) -> Option<String> {
    match value {
        Value::String(s) => Some(s.to_string_lossy().to_string()),
        Value::Integer(id) => {
            crate::atlas::get_atlas_name_by_element_id(*id as u32)
                .map(|s| s.to_string())
        }
        _ => None,
    }
}

/// SetAtlas, GetAtlas.
fn add_atlas_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetAtlas", |lua, this, args: mlua::MultiValue| {
        let id = this.0;
        let args_vec: Vec<Value> = args.into_iter().collect();
        let atlas_name = args_vec.first().and_then(resolve_atlas_name);
        let use_atlas_size = args_vec
            .get(1)
            .map(|v| matches!(v, Value::Boolean(true)))
            .unwrap_or(false);
        if let Some(name) = atlas_name {
            apply_set_atlas(lua, id, &name, use_atlas_size)?;
        }
        Ok(())
    });
    methods.add_method("GetAtlas", |lua, this, ()| {
        let id = this.0;
        if let Some(result) = call_lua_override(lua, id, "GetAtlas")? {
            return Ok(result);
        }
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let atlas = state.widgets.get(id).and_then(|f| f.atlas.clone());
        match atlas {
            Some(name) => Ok(Value::String(lua.create_string(&name)?)),
            None => Ok(Value::Nil),
        }
    });
}

/// Apply SetAtlas logic: look up atlas info, apply nine-slice or regular atlas.
fn apply_set_atlas(lua: &mlua::Lua, id: u64, name: &str, use_atlas_size: bool) -> mlua::Result<()> {
    let lookup = crate::atlas::get_atlas_info(name);
    let prefer_nine_slice = lookup.as_ref().is_some_and(|l| l.is_2x_fallback);
    let ns_info = if lookup.is_none() || prefer_nine_slice {
        crate::atlas::get_nine_slice_atlas_info(name)
    } else {
        None
    };

    let state_rc = get_sim_state(lua);
    if let Some(ns_info) = ns_info {
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(id) {
            frame.nine_slice_atlas = Some(ns_info);
            frame.atlas = Some(name.to_string());
            frame.texture = None;
            frame.tex_coords = None;
            frame.tex_coords_quad = None;
        }
    } else if let Some(lookup) = lookup {
        let atlas_info = lookup.info;
        let mut state = state_rc.borrow_mut();
        let parent_info = find_parent_key(&state.widgets, id);
        apply_atlas_to_frame(&mut state.widgets, id, atlas_info, name, &lookup, use_atlas_size);
        propagate_atlas_to_button(&mut state.widgets, parent_info, atlas_info);
        if use_atlas_size {
            state.invalidate_layout_with_dependents(id);
        }
    } else {
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(id) {
            frame.atlas = Some(name.to_string());
        }
    }
    Ok(())
}

/// Check __frame_fields for a Lua override of a method and call it if present.
fn call_lua_override(lua: &mlua::Lua, id: u64, method_name: &str) -> mlua::Result<Option<Value>> {
    if let Some(fields_table) = crate::lua_api::script_helpers::get_frame_fields_table(lua)
        && let Ok(frame_fields) = fields_table.get::<mlua::Table>(id)
            && let Ok(Value::Function(f)) = frame_fields.get::<Value>(method_name) {
                return Ok(Some(f.call::<Value>(frame_ref(lua, id)?)?));
            }
    Ok(None)
}

/// Find which parent_key this frame is registered as in its parent's children_keys.
fn find_parent_key(
    widgets: &crate::widget::WidgetRegistry,
    frame_id: u64,
) -> Option<(u64, Option<String>)> {
    widgets.get(frame_id).and_then(|f| {
        f.parent_id.and_then(|pid| {
            widgets.get(pid).map(|parent| {
                let key = parent
                    .children_keys
                    .iter()
                    .find(|(_, child_id)| **child_id == frame_id)
                    .map(|(k, _)| k.clone());
                (pid, key)
            })
        })
    })
}

/// Apply atlas info to a frame: set texture, UVs, tiling, atlas name, and optionally size.
fn apply_atlas_to_frame(
    widgets: &mut crate::widget::WidgetRegistry,
    frame_id: u64,
    atlas_info: &crate::atlas::AtlasInfo,
    atlas_name: &str,
    lookup: &crate::atlas::AtlasLookup,
    use_atlas_size: bool,
) {
    if let Some(frame) = widgets.get_mut_visual(frame_id) {
        frame.texture = Some(atlas_info.file.to_string());
        let atlas_uvs = (
            atlas_info.left_tex_coord,
            atlas_info.right_tex_coord,
            atlas_info.top_tex_coord,
            atlas_info.bottom_tex_coord,
        );
        frame.atlas_tex_coords = Some(atlas_uvs);
        frame.tex_coords = Some(atlas_uvs);
        frame.horiz_tile = atlas_info.tiles_horizontally;
        frame.vert_tile = atlas_info.tiles_vertically;
        frame.atlas = Some(atlas_name.to_string());
        frame.three_slice_h = three_slice_caps_for_atlas(atlas_name, atlas_info.width);
        if use_atlas_size {
            frame.width = lookup.width() as f32;
            frame.height = lookup.height() as f32;
        }
    }
}

/// Return horizontal three-slice cap info for known atlas entries.
fn three_slice_caps_for_atlas(atlas_name: &str, atlas_width: u32) -> Option<(f32, f32, f32)> {
    let w = atlas_width as f32;
    match atlas_name {
        "common-dropdown-textholder" => Some((12.0, 12.0, w)),
        _ => None,
    }
}

/// Propagate atlas texture path and UV coords to the parent button if appropriate.
fn propagate_atlas_to_button(
    widgets: &mut crate::widget::WidgetRegistry,
    parent_info: Option<(u64, Option<String>)>,
    atlas_info: &crate::atlas::AtlasInfo,
) {
    let Some((parent_id, Some(parent_key))) = parent_info else { return };
    let Some(parent) = widgets.get_mut_visual(parent_id) else { return };
    if !matches!(parent.widget_type, WidgetType::Button | WidgetType::CheckButton) {
        return;
    }
    let texture_path = atlas_info.file.to_string();
    let tex_coords = (
        atlas_info.left_tex_coord,
        atlas_info.right_tex_coord,
        atlas_info.top_tex_coord,
        atlas_info.bottom_tex_coord,
    );
    set_button_texture_field(parent, &parent_key, texture_path, tex_coords);
}

/// Set the appropriate texture field on a button based on the parent key name.
fn set_button_texture_field(
    parent: &mut Frame,
    parent_key: &str,
    texture_path: String,
    tex_coords: (f32, f32, f32, f32),
) {
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
        "CheckedTexture" => {
            parent.checked_texture = Some(texture_path);
            parent.checked_tex_coords = Some(tex_coords);
        }
        "DisabledCheckedTexture" => {
            parent.disabled_checked_texture = Some(texture_path);
            parent.disabled_checked_tex_coords = Some(tex_coords);
        }
        _ => {}
    }
}

/// SetSnapToPixelGrid, IsSnappingToPixelGrid, SetTexelSnappingBias, GetTexelSnappingBias.
fn add_pixel_grid_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetSnapToPixelGrid", |_, _this, _snap: bool| Ok(()));
    methods.add_method("IsSnappingToPixelGrid", |_, _this, ()| Ok(false));
    methods.add_method("SetTexelSnappingBias", |_, _this, _bias: f32| Ok(()));
    methods.add_method("GetTexelSnappingBias", |_, _this, ()| Ok(0.0_f32));
}

/// SetTextureSliceMargins etc.
fn add_nine_slice_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetTextureSliceMargins", |_, _this, (_l, _r, _t, _b): (f32, f32, f32, f32)| Ok(()));
    methods.add_method("GetTextureSliceMargins", |_, _this, ()| {
        Ok((0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32))
    });
    methods.add_method("SetTextureSliceMode", |_, _this, _mode: i32| Ok(()));
    methods.add_method("GetTextureSliceMode", |_, _this, ()| Ok(0i32));
    methods.add_method("ClearTextureSlice", |_, _this, ()| Ok(()));
}

/// SetVertexColor, GetVertexColor, SetCenterColor.
fn add_vertex_color_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetVertexColor", |lua, this, (r, g, b, a): (Option<f32>, Option<f32>, Option<f32>, Option<f32>)| {
        let (Some(r), Some(g), Some(b)) = (r, g, b) else { return Ok(()) };
        let id = this.0;
        let new_color = crate::widget::Color::new(r, g, b, a.unwrap_or(1.0));
        let state_rc = get_sim_state(lua);
        let already_set = state_rc.borrow().widgets.get(id)
            .and_then(|f| f.vertex_color.as_ref())
            .map(|c| c.r == new_color.r && c.g == new_color.g && c.b == new_color.b && c.a == new_color.a)
            .unwrap_or(false);
        if !already_set {
            let mut state = state_rc.borrow_mut();
            if let Some(frame) = state.widgets.get_mut_visual(id) {
                frame.vertex_color = Some(new_color);
                frame.alpha = new_color.a;
            }
        }
        Ok(())
    });
    methods.add_method("GetVertexColor", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        if let Some(frame) = state.widgets.get(this.0)
            && let Some(color) = &frame.vertex_color {
                return Ok((color.r, color.g, color.b, color.a));
            }
        Ok((1.0f32, 1.0f32, 1.0f32, 1.0f32))
    });
    methods.add_method("SetCenterColor", |_, _this, _args: mlua::MultiValue| Ok(()));
}

/// GetTexCoord, SetTexCoord.
fn add_tex_coord_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetTexCoord", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        if let Some(frame) = state.widgets.get(this.0)
            && let Some((left, right, top, bottom)) = frame.tex_coords {
                return Ok((left, top, left, bottom, right, top, right, bottom));
            }
        Ok((0.0_f32, 0.0_f32, 0.0_f32, 1.0_f32, 1.0_f32, 0.0_f32, 1.0_f32, 1.0_f32))
    });
    methods.add_method("SetTexCoord", |lua, this, args: mlua::MultiValue| {
        let id = this.0;
        let args_vec: Vec<Value> = args.into_iter().collect();
        let (raw_quad, left, right, top, bottom) = parse_tex_coord_args(&args_vec);
        let (Some(left), Some(right), Some(top), Some(bottom)) = (left, right, top, bottom) else {
            return Ok(());
        };
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(id) {
            frame.tex_coords =
                Some(remap_tex_coords(frame.atlas_tex_coords, left, right, top, bottom));
            frame.tex_coords_quad = raw_quad;
        }
        Ok(())
    });
}

/// Parse SetTexCoord arguments into raw quad and (left, right, top, bottom).
fn parse_tex_coord_args(args_vec: &[Value]) -> (Option<[f32; 8]>, Option<f32>, Option<f32>, Option<f32>, Option<f32>) {
    if args_vec.len() >= 8 {
        parse_tex_coord_8_args(args_vec)
    } else if args_vec.len() >= 4 {
        let coords = (
            value_to_f32(&args_vec[0], 0.0),
            value_to_f32(&args_vec[1], 1.0),
            value_to_f32(&args_vec[2], 0.0),
            value_to_f32(&args_vec[3], 1.0),
        );
        (None, Some(coords.0), Some(coords.1), Some(coords.2), Some(coords.3))
    } else {
        (None, None, None, None, None)
    }
}

/// Parse 8-arg form: ULx, ULy, LLx, LLy, URx, URy, LRx, LRy.
fn parse_tex_coord_8_args(args_vec: &[Value]) -> (Option<[f32; 8]>, Option<f32>, Option<f32>, Option<f32>, Option<f32>) {
    let ul_x = value_to_f32(&args_vec[0], 0.0);
    let ul_y = value_to_f32(&args_vec[1], 0.0);
    let ll_x = value_to_f32(&args_vec[2], 0.0);
    let ll_y = value_to_f32(&args_vec[3], 1.0);
    let ur_x = value_to_f32(&args_vec[4], 1.0);
    let ur_y = value_to_f32(&args_vec[5], 0.0);
    let lr_x = value_to_f32(&args_vec[6], 1.0);
    let lr_y = value_to_f32(&args_vec[7], 1.0);
    let raw_quad = Some([ul_x, ul_y, ll_x, ll_y, ur_x, ur_y, lr_x, lr_y]);
    let left = ul_x.min(ll_x).min(ur_x).min(lr_x);
    let right = ul_x.max(ll_x).max(ur_x).max(lr_x);
    let top = ul_y.min(ll_y).min(ur_y).min(lr_y);
    let bottom = ul_y.max(ll_y).max(ur_y).max(lr_y);
    (raw_quad, Some(left), Some(right), Some(top), Some(bottom))
}

fn value_to_f32(value: &Value, default: f32) -> f32 {
    match value {
        Value::Number(n) => *n as f32,
        Value::Integer(n) => *n as f32,
        _ => default,
    }
}

fn remap_tex_coords(
    atlas_tex_coords: Option<(f32, f32, f32, f32)>,
    left: f32, right: f32, top: f32, bottom: f32,
) -> (f32, f32, f32, f32) {
    if let Some((al, ar, at, ab)) = atlas_tex_coords {
        let aw = ar - al;
        let ah = ab - at;
        (al + left * aw, al + right * aw, at + top * ah, at + bottom * ah)
    } else {
        (left, right, top, bottom)
    }
}

/// AddMaskTexture, RemoveMaskTexture, GetNumMaskTextures, GetMaskTexture.
fn add_mask_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("AddMaskTexture", |lua, this, mask: Value| {
        let mask_id = extract_frame_id(&mask);
        if let Some(mask_id) = mask_id {
            let state_rc = get_sim_state(lua);
            let mut state = state_rc.borrow_mut();
            if let Some(frame) = state.widgets.get_mut_visual(this.0) {
                if !frame.mask_textures.contains(&mask_id) {
                    frame.mask_textures.push(mask_id);
                }
            }
        }
        Ok(())
    });
    methods.add_method("RemoveMaskTexture", |lua, this, mask: Value| {
        let mask_id = extract_frame_id(&mask);
        if let Some(mask_id) = mask_id {
            let state_rc = get_sim_state(lua);
            let mut state = state_rc.borrow_mut();
            if let Some(frame) = state.widgets.get_mut_visual(this.0) {
                frame.mask_textures.retain(|&mid| mid != mask_id);
            }
        }
        Ok(())
    });
    methods.add_method("GetNumMaskTextures", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state.widgets.get(this.0).map_or(0, |f| f.mask_textures.len()))
    });
    methods.add_method("GetMaskTexture", |_, _this, _index: i32| Ok(Value::Nil));
}

/// SetRotation, GetRotation.
fn add_rotation_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetRotation", |lua, this, radians: f64| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.rotation = radians as f32;
        }
        Ok(())
    });
    methods.add_method("GetRotation", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state.widgets.get(this.0).map(|f| f.rotation as f64).unwrap_or(0.0))
    });
}

/// SetGradient, SetDrawLayer, GetDrawLayer.
fn add_draw_layer_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetGradient", |lua, this, _args: mlua::MultiValue| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.vertex_color = None;
            frame.alpha = 1.0;
        }
        Ok(())
    });
    methods.add_method("SetDrawLayer", |lua, this, args: mlua::MultiValue| {
        use crate::widget::DrawLayer;
        let id = this.0;
        let args_vec: Vec<Value> = args.into_iter().collect();
        if let Some(Value::String(s)) = args_vec.first() {
            let layer_str = s.to_string_lossy();
            if let Some(layer) = DrawLayer::from_str(&layer_str) {
                let state_rc = get_sim_state(lua);
                let mut state = state_rc.borrow_mut();
                if let Some(frame) = state.widgets.get_mut_visual(id) {
                    frame.draw_layer = layer;
                    frame.draw_sub_layer = match args_vec.get(1) {
                        Some(Value::Integer(n)) => *n as i32,
                        Some(Value::Number(n)) => *n as i32,
                        _ => 0,
                    };
                }
            }
        }
        Ok(())
    });
    methods.add_method("GetDrawLayer", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        if let Some(f) = state.widgets.get(this.0) {
            Ok((f.draw_layer.as_str().to_string(), f.draw_sub_layer))
        } else {
            Ok(("ARTWORK".to_string(), 0i32))
        }
    });
}

/// SetVisuals - used by StatusBar spark textures.
fn add_visual_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetVisuals", |_, _this, _info: Value| Ok(()));
}
