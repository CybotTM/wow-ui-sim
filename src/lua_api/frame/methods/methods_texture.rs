//! Texture-related methods: SetTexture, SetAtlas, SetTexCoord, etc.

use super::super::handle::FrameRef;
use super::methods_helpers::resolve_file_data_id_or_path;
use super::methods_texture_visual;
use crate::lua_api::frame::handle::{frame_ref, get_sim_state};
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
    methods_texture_visual::add_texture_visual_methods(methods);
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
        let texture_args = parse_set_texture_args(args);
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(id) {
            apply_set_texture_args(frame, texture_args);
        }
        Ok(())
    });
}

fn parse_set_texture_args(args: mlua::MultiValue) -> SetTextureArgs {
    let args_vec: Vec<Value> = args.into_iter().collect();
    let file_data_id = args_vec.first().and_then(extract_file_data_id);
    let path = args_vec
        .first()
        .map(resolve_file_data_id_or_path)
        .unwrap_or(None);
    SetTextureArgs {
        path,
        file_data_id,
        horiz_tile: bool_arg(&args_vec, 1),
        vert_tile: bool_arg(&args_vec, 2),
    }
}

fn bool_arg(args: &[Value], index: usize) -> Option<bool> {
    match args.get(index) {
        Some(Value::Boolean(value)) => Some(*value),
        _ => None,
    }
}

fn apply_set_texture_args(frame: &mut Frame, texture_args: SetTextureArgs) {
    frame.texture = texture_args.path;
    frame.texture_file_data_id = texture_args.file_data_id;
    if let Some(horiz_tile) = texture_args.horiz_tile {
        frame.horiz_tile = horiz_tile;
    }
    if let Some(vert_tile) = texture_args.vert_tile {
        frame.vert_tile = vert_tile;
    }
}

struct SetTextureArgs {
    path: Option<String>,
    file_data_id: Option<i64>,
    horiz_tile: Option<bool>,
    vert_tile: Option<bool>,
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
            Some(ref s) if s.parse::<i64>().is_ok() => {
                Ok(Value::Integer(s.parse::<i64>().unwrap()))
            }
            Some(s) => Ok(Value::String(lua.create_string(&s)?)),
            None => Ok(Value::Nil),
        }
    });
}

fn add_set_color_texture<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method(
        "SetColorTexture",
        |lua, this, (r, g, b, a): (f32, f32, f32, Option<f32>)| {
            let id = this.0;
            let state_rc = get_sim_state(lua);
            let mut state = state_rc.borrow_mut();
            if let Some(frame) = state.widgets.get_mut_visual(id) {
                frame.color_texture = Some(crate::widget::Color::new(r, g, b, a.unwrap_or(1.0)));
                frame.texture = None;
                frame.texture_file_data_id = None;
            }
            Ok(())
        },
    );
}

/// SetHorizTile, GetHorizTile, SetVertTile, GetVertTile.
fn add_tiling_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_tile_setter(methods, "SetHorizTile", |frame, tile| {
        frame.horiz_tile = tile
    });
    add_tile_getter(methods, "GetHorizTile", |frame| frame.horiz_tile);
    add_tile_setter(methods, "SetVertTile", |frame, tile| frame.vert_tile = tile);
    add_tile_getter(methods, "GetVertTile", |frame| frame.vert_tile);
}

fn add_tile_setter<M, F>(methods: &mut M, name: &str, update: F)
where
    M: mlua::UserDataMethods<FrameRef>,
    F: Fn(&mut Frame, bool) + Copy + 'static,
{
    methods.add_method(name, move |lua, this, tile: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            update(frame, tile);
        }
        Ok(())
    });
}

fn add_tile_getter<M, F>(methods: &mut M, name: &str, read: F)
where
    M: mlua::UserDataMethods<FrameRef>,
    F: Fn(&Frame) -> bool + Copy + 'static,
{
    methods.add_method(name, move |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state.widgets.get(this.0).map(read).unwrap_or(false))
    });
}

/// SetBlendMode, GetBlendMode, SetDesaturated, IsDesaturated.
fn add_blend_and_desaturation_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_blend_mode_methods(methods);
    add_desaturation_methods(methods);
}

fn add_blend_mode_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetBlendMode", |lua, this, mode: Option<String>| {
        let raw_mode = normalize_blend_mode(mode);
        let blend = parse_blend_mode(raw_mode.as_deref());
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(f) = state.widgets.get_mut_visual(this.0) {
            f.alpha_mode = raw_mode;
            f.blend_mode = blend;
        }
        Ok(())
    });
    methods.add_method("GetBlendMode", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let mode = state
            .widgets
            .get(this.0)
            .and_then(|f| f.alpha_mode.clone())
            .unwrap_or_else(|| {
                match state.widgets.get(this.0).map(|f| f.blend_mode) {
                    Some(crate::render::BlendMode::Additive) => "ADD",
                    _ => "BLEND",
                }
                .to_string()
            });
        Ok(mode)
    });
}

fn normalize_blend_mode(mode: Option<String>) -> Option<String> {
    mode.map(|mode| mode.trim().to_ascii_uppercase())
}

fn parse_blend_mode(mode: Option<&str>) -> crate::render::BlendMode {
    match mode {
        Some("ADD") => crate::render::BlendMode::Additive,
        _ => crate::render::BlendMode::Alpha,
    }
}

fn add_desaturation_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_desaturated_bool_setter(methods);
    add_desaturated_bool_getter(methods);
    add_desaturation_value_getter(methods);
    add_desaturation_value_setter(methods);
}

fn add_desaturated_bool_setter<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetDesaturated", |lua, this, desaturated: bool| {
        set_desaturated(get_sim_state(lua), this.0, desaturated);
        Ok(())
    });
}

fn add_desaturated_bool_getter<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("IsDesaturated", |lua, this, ()| {
        Ok(read_desaturated(get_sim_state(lua), this.0))
    });
}

fn add_desaturation_value_getter<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetDesaturation", |lua, this, ()| {
        Ok(if read_desaturated(get_sim_state(lua), this.0) {
            1.0_f64
        } else {
            0.0
        })
    });
}

fn add_desaturation_value_setter<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetDesaturation", |lua, this, desat: f64| {
        set_desaturated(get_sim_state(lua), this.0, desat > 0.0);
        Ok(())
    });
}

fn set_desaturated(
    state_rc: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
    id: u64,
    value: bool,
) {
    let mut state = state_rc.borrow_mut();
    if let Some(f) = state.widgets.get_mut_visual(id) {
        f.desaturated = value;
    }
}

fn read_desaturated(
    state_rc: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
    id: u64,
) -> bool {
    let state = state_rc.borrow();
    state
        .widgets
        .get(id)
        .map(|f| f.desaturated)
        .unwrap_or(false)
}

/// Resolve atlas name from a Lua value (string or numeric element ID).
fn resolve_atlas_name(value: &Value) -> Option<String> {
    match value {
        Value::String(s) => Some(s.to_string_lossy().to_string()),
        Value::Integer(id) => {
            crate::atlas::get_atlas_name_by_element_id(*id as u32).map(|s| s.to_string())
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

/// Returns true if the atlas is already set to the given name and no resize is needed.
fn atlas_unchanged(lua: &mlua::Lua, id: u64, name: &str, use_atlas_size: bool) -> bool {
    if use_atlas_size {
        return false;
    }
    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    state
        .widgets
        .get(id)
        .is_some_and(|f| f.atlas.as_deref() == Some(name))
}

/// Apply a nine-slice atlas to a frame.
fn apply_nine_slice(
    lua: &mlua::Lua,
    id: u64,
    name: &str,
    ns_info: crate::atlas::NineSliceAtlasInfo,
) {
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    if let Some(frame) = state.widgets.get_mut_visual(id) {
        frame.nine_slice_atlas = Some(ns_info);
        frame.atlas = Some(name.to_string());
        frame.texture = None;
        frame.tex_coords = None;
        frame.tex_coords_quad = None;
    }
}

/// Apply a regular atlas lookup to a frame, propagating to parent button if applicable.
fn apply_regular_atlas(
    lua: &mlua::Lua,
    id: u64,
    name: &str,
    lookup: &crate::atlas::AtlasLookup,
    use_atlas_size: bool,
) {
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    let parent_info = find_parent_key(&state.widgets, id);
    apply_atlas_to_frame(
        &mut state.widgets,
        id,
        lookup.info,
        name,
        lookup,
        use_atlas_size,
    );
    propagate_atlas_to_button(&mut state.widgets, parent_info, lookup.info);
    if use_atlas_size {
        state.invalidate_layout_with_dependents(id);
    }
}

/// Apply SetAtlas logic: look up atlas info, apply nine-slice or regular atlas.
fn apply_set_atlas(lua: &mlua::Lua, id: u64, name: &str, use_atlas_size: bool) -> mlua::Result<()> {
    if atlas_unchanged(lua, id, name, use_atlas_size) {
        return Ok(());
    }
    let lookup = crate::atlas::get_render_atlas_info(name);
    let prefer_nine_slice = lookup.as_ref().is_some_and(|l| l.is_2x_fallback);
    let ns_info = if lookup.is_none() || prefer_nine_slice {
        crate::atlas::get_nine_slice_atlas_info(name)
    } else {
        None
    };
    if let Some(ns_info) = ns_info {
        apply_nine_slice(lua, id, name, ns_info);
    } else if let Some(lookup) = lookup {
        apply_regular_atlas(lua, id, name, &lookup, use_atlas_size);
    } else {
        let state_rc = get_sim_state(lua);
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
        && let Ok(Value::Function(f)) = frame_fields.get::<Value>(method_name)
    {
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
pub(crate) fn apply_atlas_to_frame(
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
    let Some((parent_id, Some(parent_key))) = parent_info else {
        return;
    };
    let Some(parent) = widgets.get_mut_visual(parent_id) else {
        return;
    };
    if !matches!(
        parent.widget_type,
        WidgetType::Button | WidgetType::CheckButton
    ) {
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
pub(crate) fn set_button_texture_field(
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
    methods.add_method("SetSnapToPixelGrid", |lua, this, snap: bool| {
        set_snap_to_pixel_grid(get_sim_state(lua), this.0, snap);
        Ok(())
    });
    methods.add_method("IsSnappingToPixelGrid", |lua, this, ()| {
        Ok(read_snap_to_pixel_grid(get_sim_state(lua), this.0))
    });
    methods.add_method("SetTexelSnappingBias", |lua, this, bias: f32| {
        set_texel_snapping_bias(get_sim_state(lua), this.0, bias);
        Ok(())
    });
    methods.add_method("GetTexelSnappingBias", |lua, this, ()| {
        Ok(read_texel_snapping_bias(get_sim_state(lua), this.0))
    });
}

fn set_snap_to_pixel_grid(
    state_rc: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
    id: u64,
    snap: bool,
) {
    let mut state = state_rc.borrow_mut();
    if let Some(frame) = state.widgets.get_mut_visual(id) {
        frame.snap_to_pixel_grid = snap;
    }
}

fn read_snap_to_pixel_grid(
    state_rc: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
    id: u64,
) -> bool {
    let state = state_rc.borrow();
    state
        .widgets
        .get(id)
        .map(|frame| frame.snap_to_pixel_grid)
        .unwrap_or(false)
}

fn set_texel_snapping_bias(
    state_rc: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
    id: u64,
    bias: f32,
) {
    let mut state = state_rc.borrow_mut();
    if let Some(frame) = state.widgets.get_mut_visual(id) {
        frame.texel_snapping_bias = bias;
    }
}

fn read_texel_snapping_bias(
    state_rc: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
    id: u64,
) -> f32 {
    let state = state_rc.borrow();
    state
        .widgets
        .get(id)
        .map(|frame| frame.texel_snapping_bias)
        .unwrap_or(0.0)
}

/// SetTextureSliceMargins etc.
fn add_nine_slice_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method(
        "SetTextureSliceMargins",
        |lua, this, (left, right, top, bottom): (f32, f32, f32, f32)| {
            set_texture_slice_margins(get_sim_state(lua), this.0, (left, right, top, bottom));
            Ok(())
        },
    );
    methods.add_method("GetTextureSliceMargins", |lua, this, ()| {
        Ok(read_texture_slice_margins(get_sim_state(lua), this.0))
    });
    methods.add_method("SetTextureSliceMode", |lua, this, mode: i32| {
        set_texture_slice_mode(get_sim_state(lua), this.0, mode);
        Ok(())
    });
    methods.add_method("GetTextureSliceMode", |lua, this, ()| {
        Ok(read_texture_slice_mode(get_sim_state(lua), this.0))
    });
    methods.add_method("ClearTextureSlice", |lua, this, ()| {
        clear_texture_slice(get_sim_state(lua), this.0);
        Ok(())
    });
}

fn set_texture_slice_margins(
    state_rc: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
    id: u64,
    margins: (f32, f32, f32, f32),
) {
    let mut state = state_rc.borrow_mut();
    if let Some(frame) = state.widgets.get_mut_visual(id) {
        frame.texture_slice_margins = margins;
    }
}

fn read_texture_slice_margins(
    state_rc: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
    id: u64,
) -> (f32, f32, f32, f32) {
    let state = state_rc.borrow();
    state
        .widgets
        .get(id)
        .map(|frame| frame.texture_slice_margins)
        .unwrap_or((0.0, 0.0, 0.0, 0.0))
}

fn set_texture_slice_mode(
    state_rc: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
    id: u64,
    mode: i32,
) {
    let mut state = state_rc.borrow_mut();
    if let Some(frame) = state.widgets.get_mut_visual(id) {
        frame.texture_slice_mode = mode;
    }
}

fn read_texture_slice_mode(
    state_rc: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
    id: u64,
) -> i32 {
    let state = state_rc.borrow();
    state
        .widgets
        .get(id)
        .map(|frame| frame.texture_slice_mode)
        .unwrap_or(0)
}

fn clear_texture_slice(
    state_rc: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
    id: u64,
) {
    let mut state = state_rc.borrow_mut();
    if let Some(frame) = state.widgets.get_mut_visual(id) {
        frame.texture_slice_margins = (0.0, 0.0, 0.0, 0.0);
        frame.texture_slice_mode = 0;
    }
}
