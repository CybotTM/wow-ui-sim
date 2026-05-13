//! Button texture setter/getter methods, atlas setters, and clear methods.

use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, extract_frame_id, frame_id_from_stack, frame_ref,
    sync_child_to_rilua, val_to_string,
};
use crate::lua_bridge::stack_val;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

use super::shared::opt_string;

type TextureCoords = (f32, f32, f32, f32);

struct ResolvedTexturePath {
    path: Option<String>,
    tex_coords: Option<TextureCoords>,
    file_data_id: Option<i64>,
}

// ── Visibility logic ──────────────────────────────────────────────────────────

pub(super) fn button_texture_should_show(
    sim: &crate::lua_api::SimState,
    button_id: u64,
    parent_key: &str,
) -> bool {
    let (enabled, checked, button_state) = sim
        .widgets
        .get(button_id)
        .map(|frame| {
            let enabled = frame
                .attributes
                .get("__enabled")
                .and_then(|v| match v {
                    crate::widget::AttributeValue::Boolean(b) => Some(*b),
                    _ => None,
                })
                .unwrap_or(true);
            let checked = frame
                .attributes
                .get("__checked")
                .and_then(|v| match v {
                    crate::widget::AttributeValue::Boolean(b) => Some(*b),
                    _ => None,
                })
                .unwrap_or(false);
            (enabled, checked, frame.button_state)
        })
        .unwrap_or((true, false, 0));
    match parent_key {
        "NormalTexture" => enabled && button_state == 0,
        "PushedTexture" => enabled && button_state == 1,
        "DisabledTexture" => !enabled,
        "HighlightTexture" => false,
        "CheckedTexture" => enabled && checked,
        "DisabledCheckedTexture" => !enabled && checked,
        _ => true,
    }
}

// ── Texture getters ───────────────────────────────────────────────────────────

fn push_button_texture_child(state: &mut LuaState, id: u64, parent_key: &str) -> LuaResult<u32> {
    let tex_id = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .and_then(|f| f.children_keys.get(parent_key).copied())
    };
    match tex_id {
        Some(tid) => {
            let val = frame_ref(state, tid)?;
            state.push(val);
            Ok(1)
        }
        None => {
            state.push(Val::Nil);
            Ok(1)
        }
    }
}

pub(super) fn get_normal_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    push_button_texture_child(state, id, "NormalTexture")
}

pub(super) fn get_highlight_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    push_button_texture_child(state, id, "HighlightTexture")
}

pub(super) fn get_pushed_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    push_button_texture_child(state, id, "PushedTexture")
}

pub(super) fn get_disabled_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    push_button_texture_child(state, id, "DisabledTexture")
}

pub(super) fn get_checked_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    push_button_texture_child(state, id, "CheckedTexture")
}

pub(super) fn get_disabled_checked_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    push_button_texture_child(state, id, "DisabledCheckedTexture")
}

// ── apply_texture_path_to_button ──────────────────────────────────────────────

/// Apply a texture path/atlas/fileDataID to a button slot and its child texture.
pub(super) fn apply_texture_path_to_button(
    state: &mut LuaState,
    button_id: u64,
    parent_key: &str,
    texture_val: Val,
    set_button_field: fn(&mut crate::widget::Frame, Option<String>, Option<(f32, f32, f32, f32)>),
) -> LuaResult<()> {
    if let Some(tex_id) = extract_frame_id(state, texture_val) {
        return apply_texture_userdata(state, button_id, parent_key, tex_id);
    }
    apply_texture_path(state, button_id, parent_key, texture_val, set_button_field)
}

fn apply_texture_userdata(
    state: &mut LuaState,
    button_id: u64,
    parent_key: &str,
    tex_id: u64,
) -> LuaResult<()> {
    let mut sim = borrow_state_mut(state)?;
    let current_parent = sim.widgets.get(tex_id).and_then(|f| f.parent_id);
    let needs_default_anchors = sim
        .widgets
        .get(tex_id)
        .map(|t| t.anchors.is_empty())
        .unwrap_or(false);
    if current_parent != Some(button_id) {
        super::super::methods_hierarchy::reparent_widget(&mut sim.widgets, tex_id, Some(button_id));
    }
    if let Some(tex) = sim.widgets.get_mut_visual(tex_id) {
        if needs_default_anchors {
            super::super::methods_helpers::set_all_points_anchors_pub(tex, button_id);
        }
        tex.parent_key = Some(parent_key.to_string());
    }
    if let Some(btn) = sim.widgets.get_mut_visual(button_id) {
        btn.children_keys.insert(parent_key.to_string(), tex_id);
    }
    if parent_key == "HighlightTexture" {
        if let Some(tex) = sim.widgets.get_mut_visual(tex_id) {
            tex.draw_layer = crate::widget::DrawLayer::Highlight;
            tex.alpha_mode = Some("ADD".to_string());
            tex.blend_mode = crate::render::BlendMode::Additive;
        }
    }
    let should_show = button_texture_should_show(&sim, button_id, parent_key);
    sim.widgets.set_visible(tex_id, should_show);
    drop(sim);
    let _ = sync_child_to_rilua(state, button_id, parent_key, tex_id);
    Ok(())
}

fn resolve_atlas_path(path: &str) -> (Option<String>, Option<(f32, f32, f32, f32)>) {
    if let Some(lookup) = crate::atlas::get_render_atlas_info(path) {
        let info = lookup.info;
        let coords = (
            info.left_tex_coord,
            info.right_tex_coord,
            info.top_tex_coord,
            info.bottom_tex_coord,
        );
        (Some(info.file.to_string()), Some(coords))
    } else {
        (Some(path.to_string()), None)
    }
}

fn button_texture_field_matches(
    button: &crate::widget::Frame,
    parent_key: &str,
    resolved_path: Option<&str>,
    tex_coords: Option<(f32, f32, f32, f32)>,
) -> bool {
    match parent_key {
        "NormalTexture" => {
            button.normal_texture.as_deref() == resolved_path
                && button.normal_tex_coords == tex_coords
        }
        "HighlightTexture" => {
            button.highlight_texture.as_deref() == resolved_path
                && button.highlight_tex_coords == tex_coords
        }
        "PushedTexture" => {
            button.pushed_texture.as_deref() == resolved_path
                && button.pushed_tex_coords == tex_coords
        }
        "DisabledTexture" => {
            button.disabled_texture.as_deref() == resolved_path
                && button.disabled_tex_coords == tex_coords
        }
        "CheckedTexture" => button.checked_texture.as_deref() == resolved_path,
        "DisabledCheckedTexture" => button.disabled_checked_texture.as_deref() == resolved_path,
        _ => false,
    }
}

fn texture_child_matches(
    sim: &crate::lua_api::SimState,
    button_id: u64,
    parent_key: &str,
    resolved_path: Option<&str>,
    tex_coords: Option<(f32, f32, f32, f32)>,
    file_data_id: Option<i64>,
) -> bool {
    let Some(button) = sim.widgets.get(button_id) else {
        return false;
    };
    if !button_texture_field_matches(button, parent_key, resolved_path, tex_coords) {
        return false;
    }
    let Some(tex_id) = button.children_keys.get(parent_key).copied() else {
        return false;
    };
    let Some(texture) = sim.widgets.get(tex_id) else {
        return false;
    };
    texture.parent_key.as_deref() == Some(parent_key)
        && texture.texture.as_deref() == resolved_path
        && texture.tex_coords == tex_coords
        && texture.atlas_tex_coords == tex_coords
        && texture.texture_file_data_id == file_data_id
}

fn apply_texture_path(
    state: &mut LuaState,
    button_id: u64,
    parent_key: &str,
    texture_val: Val,
    set_button_field: fn(&mut crate::widget::Frame, Option<String>, Option<(f32, f32, f32, f32)>),
) -> LuaResult<()> {
    let texture = resolve_texture_path_value(state, texture_val);
    let mut sim = borrow_state_mut(state)?;

    if texture_path_already_applied(&sim, button_id, parent_key, &texture) {
        return Ok(());
    }

    let tex_id =
        apply_texture_path_to_widgets(&mut sim, button_id, parent_key, texture, set_button_field);
    drop(sim);
    let _ = sync_child_to_rilua(state, button_id, parent_key, tex_id);
    Ok(())
}

fn resolve_texture_path_value(state: &mut LuaState, texture_val: Val) -> ResolvedTexturePath {
    let path = match texture_val {
        Val::Str(_) => val_to_string(state, texture_val),
        _ => None,
    };
    let file_data_id = match texture_val {
        Val::Num(n) => Some(n as i64),
        _ => None,
    };
    let (path, tex_coords) = match path {
        Some(path) => {
            let (resolved_path, tex_coords) = resolve_atlas_path(&path);
            (resolved_path, tex_coords)
        }
        None => (None, None),
    };

    ResolvedTexturePath {
        path,
        tex_coords,
        file_data_id,
    }
}

fn texture_path_already_applied(
    sim: &crate::lua_api::SimState,
    button_id: u64,
    parent_key: &str,
    texture: &ResolvedTexturePath,
) -> bool {
    texture_child_matches(
        sim,
        button_id,
        parent_key,
        texture.path.as_deref(),
        texture.tex_coords,
        texture.file_data_id,
    )
}

fn apply_texture_path_to_widgets(
    sim: &mut crate::lua_api::SimState,
    button_id: u64,
    parent_key: &str,
    texture: ResolvedTexturePath,
    set_button_field: fn(&mut crate::widget::Frame, Option<String>, Option<(f32, f32, f32, f32)>),
) -> u64 {
    if let Some(frame) = sim.widgets.get_mut_visual(button_id) {
        set_button_field(frame, texture.path.clone(), texture.tex_coords);
    }

    let tex_id =
        super::super::methods_helpers::get_or_create_button_texture(sim, button_id, parent_key);
    if let Some(tex) = sim.widgets.get_mut_visual(tex_id) {
        tex.texture = texture.path;
        tex.tex_coords = texture.tex_coords;
        tex.atlas_tex_coords = texture.tex_coords;
        tex.texture_file_data_id = texture.file_data_id;
    }
    let should_show = button_texture_should_show(&sim, button_id, parent_key);
    sim.widgets.set_visible(tex_id, should_show);
    tex_id
}

// ── Texture setters ───────────────────────────────────────────────────────────

pub(super) fn set_normal_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let texture = stack_val(state, 2);
    apply_texture_path_to_button(state, id, "NormalTexture", texture, |f, path, coords| {
        f.normal_texture = path;
        f.normal_tex_coords = coords;
    })?;
    Ok(0)
}

pub(super) fn set_highlight_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let texture = stack_val(state, 2);
    apply_texture_path_to_button(state, id, "HighlightTexture", texture, |f, path, coords| {
        f.highlight_texture = path;
        f.highlight_tex_coords = coords;
    })?;
    Ok(0)
}

pub(super) fn set_pushed_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let texture = stack_val(state, 2);
    apply_texture_path_to_button(state, id, "PushedTexture", texture, |f, path, coords| {
        f.pushed_texture = path;
        f.pushed_tex_coords = coords;
    })?;
    Ok(0)
}

pub(super) fn set_disabled_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let texture = stack_val(state, 2);
    apply_texture_path_to_button(state, id, "DisabledTexture", texture, |f, path, coords| {
        f.disabled_texture = path;
        f.disabled_tex_coords = coords;
    })?;
    Ok(0)
}

pub(super) fn set_checked_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let texture = stack_val(state, 2);
    apply_texture_path_to_button(state, id, "CheckedTexture", texture, |f, path, _coords| {
        f.checked_texture = path;
    })?;
    Ok(0)
}

pub(super) fn set_disabled_checked_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let texture = stack_val(state, 2);
    apply_texture_path_to_button(
        state,
        id,
        "DisabledCheckedTexture",
        texture,
        |f, path, _coords| {
            f.disabled_checked_texture = path;
        },
    )?;
    Ok(0)
}

// ── Atlas setters ─────────────────────────────────────────────────────────────

fn ensure_button_texture_child(
    state: &mut LuaState,
    button_id: u64,
    parent_key: &str,
) -> LuaResult<Option<u64>> {
    let mut sim = borrow_state_mut(state)?;
    let child_id = super::super::methods_helpers::get_or_create_button_texture(
        &mut sim, button_id, parent_key,
    );
    Ok(Some(child_id))
}

fn apply_atlas_setter(
    state: &mut LuaState,
    button_id: u64,
    parent_key: &str,
    atlas_name: &str,
    set_button_field: fn(&mut crate::widget::Frame, String, (f32, f32, f32, f32)),
) -> LuaResult<()> {
    let Some(lookup) = crate::atlas::get_render_atlas_info(atlas_name) else {
        return Ok(());
    };
    let tex_coords = (
        lookup.info.left_tex_coord,
        lookup.info.right_tex_coord,
        lookup.info.top_tex_coord,
        lookup.info.bottom_tex_coord,
    );
    let file = lookup.info.file.to_string();
    let tex_id = ensure_button_texture_child(state, button_id, parent_key)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(tid) = tex_id {
        if let Some(tex) = sim.widgets.get_mut_visual(tid) {
            tex.atlas = Some(atlas_name.to_string());
            tex.texture = Some(file.clone());
            tex.tex_coords = Some(tex_coords);
            tex.atlas_tex_coords = Some(tex_coords);
        }
    }
    if let Some(frame) = sim.widgets.get_mut_visual(button_id) {
        set_button_field(frame, file, tex_coords);
    }
    let should_show = button_texture_should_show(&sim, button_id, parent_key);
    if let Some(tid) = tex_id {
        sim.widgets.set_visible(tid, should_show);
    }
    drop(sim);
    if let Some(tid) = tex_id {
        let _ = sync_child_to_rilua(state, button_id, parent_key, tid);
    }
    Ok(())
}

pub(super) fn set_normal_atlas(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    if let Some(name) = opt_string(state, 2) {
        apply_atlas_setter(state, id, "NormalTexture", &name, |f, file, coords| {
            f.normal_texture = Some(file);
            f.normal_tex_coords = Some(coords);
        })?;
    }
    Ok(0)
}

pub(super) fn set_pushed_atlas(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    if let Some(name) = opt_string(state, 2) {
        apply_atlas_setter(state, id, "PushedTexture", &name, |f, file, coords| {
            f.pushed_texture = Some(file);
            f.pushed_tex_coords = Some(coords);
        })?;
    }
    Ok(0)
}

pub(super) fn set_disabled_atlas(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    if let Some(name) = opt_string(state, 2) {
        apply_atlas_setter(state, id, "DisabledTexture", &name, |f, file, coords| {
            f.disabled_texture = Some(file);
            f.disabled_tex_coords = Some(coords);
        })?;
    }
    Ok(0)
}

pub(super) fn set_highlight_atlas(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    if let Some(name) = opt_string(state, 2) {
        apply_atlas_setter(state, id, "HighlightTexture", &name, |f, file, coords| {
            f.highlight_texture = Some(file);
            f.highlight_tex_coords = Some(coords);
        })?;
    }
    Ok(0)
}

// ── Clear methods ─────────────────────────────────────────────────────────────

fn clear_button_field(button: &mut crate::widget::Frame, parent_key: &str) {
    match parent_key {
        "NormalTexture" => {
            button.normal_texture = None;
            button.normal_tex_coords = None;
        }
        "HighlightTexture" => {
            button.highlight_texture = None;
            button.highlight_tex_coords = None;
        }
        "PushedTexture" => {
            button.pushed_texture = None;
            button.pushed_tex_coords = None;
        }
        "DisabledTexture" => {
            button.disabled_texture = None;
            button.disabled_tex_coords = None;
        }
        _ => {}
    }
}

fn clear_child_texture(child: &mut crate::widget::Frame) {
    child.texture = None;
    child.texture_file_data_id = None;
    child.tex_coords = None;
    child.tex_coords_quad = None;
    child.atlas_tex_coords = None;
    child.atlas = None;
    child.three_slice_h = None;
}

/// Clear the button field and child texture for a given parent_key.
fn clear_button_texture_impl(
    state: &mut LuaState,
    button_id: u64,
    parent_key: &str,
) -> LuaResult<()> {
    let mut sim = borrow_state_mut(state)?;
    if let Some(button) = sim.widgets.get_mut_visual(button_id) {
        clear_button_field(button, parent_key);
    }
    let child_id = sim
        .widgets
        .get(button_id)
        .and_then(|b| b.children_keys.get(parent_key).copied());
    if let Some(cid) = child_id {
        if let Some(child) = sim.widgets.get_mut_visual(cid) {
            clear_child_texture(child);
        }
    }
    sim.widgets.mark_rect_dirty(button_id);
    Ok(())
}

pub(super) fn clear_normal_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    clear_button_texture_impl(state, id, "NormalTexture")?;
    Ok(0)
}

pub(super) fn clear_highlight_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    clear_button_texture_impl(state, id, "HighlightTexture")?;
    Ok(0)
}

pub(super) fn clear_pushed_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    clear_button_texture_impl(state, id, "PushedTexture")?;
    Ok(0)
}

pub(super) fn clear_disabled_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    clear_button_texture_impl(state, id, "DisabledTexture")?;
    Ok(0)
}

// ── Three-slice texture methods ───────────────────────────────────────────────

pub(super) fn set_left_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let path = opt_string(state, 2);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.left_texture = path;
    }
    Ok(0)
}

pub(super) fn set_middle_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let path = opt_string(state, 2);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.middle_texture = path;
    }
    Ok(0)
}

pub(super) fn set_right_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let path = opt_string(state, 2);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.right_texture = path;
    }
    Ok(0)
}
