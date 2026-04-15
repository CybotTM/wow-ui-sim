//! rilua RustFn equivalents of button, anchor, hierarchy, and create methods.
//!
//! Each function follows the pattern:
//! - `frame_id_from_stack(state, 1)` for self
//! - `FromStack::from_stack(state, N)` for typed args
//! - `borrow_state` / `borrow_state_mut` for SimState
//! - `state.push(...)` + `Ok(count)` for returns
//!
//! Complex mlua operations (table creation, script calls, Lua value passing) are
//! stubbed with TODO comments where a direct translation is not yet possible.

use crate::lua_api::rilua_methods::{
    borrow_state, borrow_state_mut, create_string, extract_frame_id, frame_id_from_stack,
    frame_ref, registry_table_or_create, sync_child_to_rilua, val_to_string,
};
use crate::lua_bridge::{FromStack, IntoStack, stack_val, table_set_rust_fn};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val, runtime_error};

// ── Helpers ──────────────────────────────────────────────────────────────────

/// Extract an optional f32 number from the stack (accepts Num).
fn opt_f32(state: &LuaState, index: i32) -> Option<f32> {
    match Val::from_stack(state, index) {
        Ok(Val::Num(n)) => Some(n as f32),
        _ => None,
    }
}

/// Extract an optional String from the stack, returns None for non-string.
fn opt_string(state: &LuaState, index: i32) -> Option<String> {
    match Val::from_stack(state, index) {
        Ok(Val::Str(_)) => String::from_stack(state, index).ok(),
        _ => None,
    }
}

fn bind_named_child_global(state: &mut LuaState, name: &str, child_id: u64) -> LuaResult<()> {
    let child_ref = frame_ref(state, child_id)?;
    let key = state.gc.intern_string(name.as_bytes());
    if let Some(globals) = state.gc.tables.get_mut(state.global) {
        let _ = globals.raw_set(Val::Str(key), child_ref, &state.gc.string_arena);
    }
    Ok(())
}

// ── Button font object methods ────────────────────────────────────────────────

/// GetOrCreate the `__button_font_objects` registry table.
fn get_or_create_button_font_store(state: &mut LuaState) -> Val {
    registry_table_or_create(state, "__button_font_objects")
}

/// SetNormalFontObject(fontObject)
fn set_normal_font_object(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let font_object = Val::from_stack(state, 2)?;
    let store = get_or_create_button_font_store(state);
    let key = create_string(state, &format!("{}:normal", id));
    // TODO: store font_object into store table under key
    let _ = (store, key, font_object);
    Ok(0)
}

/// GetNormalFontObject() -> fontObject
fn get_normal_font_object(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let store = get_or_create_button_font_store(state);
    // TODO: look up store["id:normal"] and push result
    let _ = (store, id);
    state.push(Val::Nil);
    Ok(1)
}

/// SetHighlightFontObject(fontObject)
fn set_highlight_font_object(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let font_object = Val::from_stack(state, 2)?;
    let store = get_or_create_button_font_store(state);
    let key = create_string(state, &format!("{}:highlight", id));
    // TODO: store font_object into store table under key
    let _ = (store, key, font_object);
    Ok(0)
}

/// GetHighlightFontObject() -> fontObject
fn get_highlight_font_object(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let store = get_or_create_button_font_store(state);
    // TODO: look up store["id:highlight"] and push result
    let _ = (store, id);
    state.push(Val::Nil);
    Ok(1)
}

/// SetDisabledFontObject(fontObject)
fn set_disabled_font_object(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let font_object = Val::from_stack(state, 2)?;
    let store = get_or_create_button_font_store(state);
    let key = create_string(state, &format!("{}:disabled", id));
    // TODO: store font_object into store table under key
    let _ = (store, key, font_object);
    Ok(0)
}

/// GetDisabledFontObject() -> fontObject
fn get_disabled_font_object(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let store = get_or_create_button_font_store(state);
    // TODO: look up store["id:disabled"] and push result
    let _ = (store, id);
    state.push(Val::Nil);
    Ok(1)
}

// ── Pushed text offset ────────────────────────────────────────────────────────

/// SetPushedTextOffset(x, y)
fn set_pushed_text_offset(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let x = f64::from_stack(state, 2)? as f32;
    let y = f64::from_stack(state, 3)? as f32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.pushed_text_offset = (x, y);
    }
    Ok(0)
}

/// GetPushedTextOffset() -> x, y
fn get_pushed_text_offset(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let (x, y) = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|f| f.pushed_text_offset)
            .unwrap_or((0.0, 0.0))
    };
    (x as f64, y as f64).into_stack(state)
}

// ── Texture getter methods ────────────────────────────────────────────────────

/// Get an existing button texture child by parent_key, or push nil.
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

/// GetNormalTexture() -> texture
fn get_normal_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    push_button_texture_child(state, id, "NormalTexture")
}

/// GetHighlightTexture() -> texture
fn get_highlight_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    push_button_texture_child(state, id, "HighlightTexture")
}

/// GetPushedTexture() -> texture
fn get_pushed_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    push_button_texture_child(state, id, "PushedTexture")
}

/// GetDisabledTexture() -> texture
fn get_disabled_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    push_button_texture_child(state, id, "DisabledTexture")
}

// ── Texture setter helpers ────────────────────────────────────────────────────

/// Determine visibility for a button texture child based on button state.
fn button_texture_should_show(
    sim: &crate::lua_api::SimState,
    button_id: u64,
    parent_key: &str,
) -> bool {
    let (enabled, button_state) = sim
        .widgets
        .get(button_id)
        .map(|frame| {
            let enabled = frame
                .attributes
                .get("__enabled")
                .and_then(|value| match value {
                    crate::widget::AttributeValue::Boolean(value) => Some(*value),
                    _ => None,
                })
                .unwrap_or(true);
            (enabled, frame.button_state)
        })
        .unwrap_or((true, 0));
    match parent_key {
        "NormalTexture" => enabled && button_state == 0,
        "PushedTexture" => enabled && button_state == 1,
        "DisabledTexture" => !enabled,
        _ => true,
    }
}

/// Apply a texture path/atlas/fileDataID to a button slot and its child texture.
fn apply_texture_path_to_button(
    state: &mut LuaState,
    button_id: u64,
    parent_key: &str,
    texture_val: Val,
    set_button_field: fn(&mut crate::widget::Frame, Option<String>, Option<(f32, f32, f32, f32)>),
) -> LuaResult<()> {
    // Check if val is a frame reference
    let maybe_tex_id = extract_frame_id(state, texture_val);
    if let Some(tex_id) = maybe_tex_id {
        // userdata path: reparent and assign
        let mut sim = borrow_state_mut(state)?;
        let current_parent = sim.widgets.get(tex_id).and_then(|f| f.parent_id);
        let needs_default_anchors = sim
            .widgets
            .get(tex_id)
            .map(|t| t.anchors.is_empty())
            .unwrap_or(false);
        if current_parent != Some(button_id) {
            super::methods_hierarchy::reparent_widget(&mut sim.widgets, tex_id, Some(button_id));
        }
        if let Some(tex) = sim.widgets.get_mut_visual(tex_id) {
            if needs_default_anchors {
                super::methods_helpers::set_all_points_anchors_pub(tex, button_id);
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
        // TODO: sync_child_to_rilua(state, button_id, parent_key, tex_id)?;
        let _ = sync_child_to_rilua(state, button_id, parent_key, tex_id);
        return Ok(());
    }

    // Non-userdata path: extract string/integer texture reference
    let path: Option<String> = match texture_val {
        Val::Str(_) => val_to_string(state, texture_val),
        _ => None,
    };
    let file_data_id: Option<i64> = match texture_val {
        Val::Num(n) => Some(n as i64),
        _ => None,
    };

    // Resolve atlas or plain path
    let resolved_path: Option<String>;
    let tex_coords: Option<(f32, f32, f32, f32)>;
    if let Some(ref p) = path {
        if let Some(lookup) = crate::atlas::get_atlas_info(p) {
            let info = lookup.info;
            tex_coords = Some((
                info.left_tex_coord,
                info.right_tex_coord,
                info.top_tex_coord,
                info.bottom_tex_coord,
            ));
            resolved_path = Some(info.file.to_string());
        } else {
            resolved_path = Some(p.clone());
            tex_coords = None;
        }
    } else {
        resolved_path = None;
        tex_coords = None;
    }

    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(button_id) {
        set_button_field(frame, resolved_path.clone(), tex_coords);
    }

    // Get or create the child texture widget
    // TODO: get_or_create_button_texture requires mlua::Lua — use rilua equivalent
    // For now: find existing child or skip creation
    let tex_id_opt = sim
        .widgets
        .get(button_id)
        .and_then(|f| f.children_keys.get(parent_key).copied());
    if let Some(tex_id) = tex_id_opt {
        if let Some(tex) = sim.widgets.get_mut_visual(tex_id) {
            tex.texture = resolved_path;
            tex.tex_coords = tex_coords;
            tex.atlas_tex_coords = tex_coords;
            tex.texture_file_data_id = file_data_id;
        }
        let should_show = button_texture_should_show(&sim, button_id, parent_key);
        sim.widgets.set_visible(tex_id, should_show);
    }
    Ok(())
}

/// SetNormalTexture(texture)
fn set_normal_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let texture = Val::from_stack(state, 2)?;
    apply_texture_path_to_button(state, id, "NormalTexture", texture, |f, path, coords| {
        f.normal_texture = path;
        f.normal_tex_coords = coords;
    })?;
    Ok(0)
}

/// SetHighlightTexture(texture)
fn set_highlight_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let texture = Val::from_stack(state, 2)?;
    apply_texture_path_to_button(state, id, "HighlightTexture", texture, |f, path, coords| {
        f.highlight_texture = path;
        f.highlight_tex_coords = coords;
    })?;
    Ok(0)
}

/// SetPushedTexture(texture)
fn set_pushed_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let texture = Val::from_stack(state, 2)?;
    apply_texture_path_to_button(state, id, "PushedTexture", texture, |f, path, coords| {
        f.pushed_texture = path;
        f.pushed_tex_coords = coords;
    })?;
    Ok(0)
}

/// SetDisabledTexture(texture)
fn set_disabled_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let texture = Val::from_stack(state, 2)?;
    apply_texture_path_to_button(state, id, "DisabledTexture", texture, |f, path, coords| {
        f.disabled_texture = path;
        f.disabled_tex_coords = coords;
    })?;
    Ok(0)
}

// ── Atlas setter methods ──────────────────────────────────────────────────────

/// Apply atlas info to both the child texture widget and the parent button field.
fn apply_atlas_setter(
    state: &mut LuaState,
    button_id: u64,
    parent_key: &str,
    atlas_name: &str,
    set_button_field: fn(&mut crate::widget::Frame, String, (f32, f32, f32, f32)),
) -> LuaResult<()> {
    let Some(lookup) = crate::atlas::get_atlas_info(atlas_name) else {
        return Ok(());
    };
    let tex_coords = (
        lookup.info.left_tex_coord,
        lookup.info.right_tex_coord,
        lookup.info.top_tex_coord,
        lookup.info.bottom_tex_coord,
    );
    let file = lookup.info.file.to_string();
    let mut sim = borrow_state_mut(state)?;
    // Find existing child texture (TODO: create if missing, requires rilua CreateFrame)
    let tex_id_opt = sim
        .widgets
        .get(button_id)
        .and_then(|f| f.children_keys.get(parent_key).copied());
    if let Some(tex_id) = tex_id_opt {
        let already_set = sim
            .widgets
            .get(tex_id)
            .map(|t| t.atlas.as_deref() == Some(atlas_name))
            .unwrap_or(false);
        if already_set {
            return Ok(());
        }
        if let Some(tex) = sim.widgets.get_mut_visual(tex_id) {
            tex.atlas = Some(atlas_name.to_string());
            tex.texture = Some(file.clone());
            tex.tex_coords = Some(tex_coords);
        }
    }
    if let Some(frame) = sim.widgets.get_mut_visual(button_id) {
        set_button_field(frame, file, tex_coords);
    }
    Ok(())
}

/// SetNormalAtlas(atlasName)
fn set_normal_atlas(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    if let Some(name) = opt_string(state, 2) {
        apply_atlas_setter(state, id, "NormalTexture", &name, |f, file, coords| {
            f.normal_texture = Some(file);
            f.normal_tex_coords = Some(coords);
        })?;
    }
    Ok(0)
}

/// SetPushedAtlas(atlasName)
fn set_pushed_atlas(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    if let Some(name) = opt_string(state, 2) {
        apply_atlas_setter(state, id, "PushedTexture", &name, |f, file, coords| {
            f.pushed_texture = Some(file);
            f.pushed_tex_coords = Some(coords);
        })?;
    }
    Ok(0)
}

/// SetDisabledAtlas(atlasName)
fn set_disabled_atlas(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    if let Some(name) = opt_string(state, 2) {
        apply_atlas_setter(state, id, "DisabledTexture", &name, |f, file, coords| {
            f.disabled_texture = Some(file);
            f.disabled_tex_coords = Some(coords);
        })?;
    }
    Ok(0)
}

/// SetHighlightAtlas(atlasName)
fn set_highlight_atlas(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    if let Some(name) = opt_string(state, 2) {
        apply_atlas_setter(state, id, "HighlightTexture", &name, |f, file, coords| {
            f.highlight_texture = Some(file);
            f.highlight_tex_coords = Some(coords);
        })?;
    }
    Ok(0)
}

// ── Checked texture methods ───────────────────────────────────────────────────

/// SetCheckedTexture(texture)
fn set_checked_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let texture = Val::from_stack(state, 2)?;
    let is_userdata = extract_frame_id(state, texture).is_some();
    let path: Option<String> = if !is_userdata {
        match texture {
            Val::Str(_) => val_to_string(state, texture),
            _ => None,
        }
    } else {
        None
    };
    let mut sim = borrow_state_mut(state)?;
    if !is_userdata {
        if let Some(frame) = sim.widgets.get_mut_visual(id) {
            frame.checked_texture = path.clone();
        }
    }
    // TODO: get_or_create_button_texture — find/create "CheckedTexture" child
    let tex_id_opt = sim
        .widgets
        .get(id)
        .and_then(|f| f.children_keys.get("CheckedTexture").copied());
    if let Some(tex_id) = tex_id_opt {
        if let Some(tex) = sim.widgets.get_mut_visual(tex_id) {
            if !is_userdata {
                tex.texture = path;
            }
            tex.visible = false;
        }
    }
    Ok(0)
}

/// SetDisabledCheckedTexture(texture)
fn set_disabled_checked_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let texture = Val::from_stack(state, 2)?;
    let is_userdata = extract_frame_id(state, texture).is_some();
    let path: Option<String> = if !is_userdata {
        match texture {
            Val::Str(_) => val_to_string(state, texture),
            _ => None,
        }
    } else {
        None
    };
    let mut sim = borrow_state_mut(state)?;
    if !is_userdata {
        if let Some(frame) = sim.widgets.get_mut_visual(id) {
            frame.disabled_checked_texture = path.clone();
        }
    }
    let tex_id_opt = sim
        .widgets
        .get(id)
        .and_then(|f| f.children_keys.get("DisabledCheckedTexture").copied());
    if let Some(tex_id) = tex_id_opt {
        if let Some(tex) = sim.widgets.get_mut_visual(tex_id) {
            if !is_userdata {
                tex.texture = path;
            }
            tex.visible = false;
        }
    }
    Ok(0)
}

/// GetDisabledCheckedTexture() -> texture
fn get_disabled_checked_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    push_button_texture_child(state, id, "DisabledCheckedTexture")
}

// ── Clear texture methods ─────────────────────────────────────────────────────

/// Clear the button field and child texture for a given parent_key.
fn clear_button_texture_impl(
    state: &mut LuaState,
    button_id: u64,
    parent_key: &str,
) -> LuaResult<()> {
    let mut sim = borrow_state_mut(state)?;
    // Clear the button's own field
    if let Some(button) = sim.widgets.get_mut_visual(button_id) {
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
    // Clear the child texture widget
    let child_id = sim
        .widgets
        .get(button_id)
        .and_then(|b| b.children_keys.get(parent_key).copied());
    if let Some(cid) = child_id {
        if let Some(child) = sim.widgets.get_mut_visual(cid) {
            child.texture = None;
            child.texture_file_data_id = None;
            child.tex_coords = None;
            child.tex_coords_quad = None;
            child.atlas_tex_coords = None;
            child.atlas = None;
            child.three_slice_h = None;
        }
    }
    sim.widgets.mark_rect_dirty(button_id);
    Ok(())
}

fn clear_normal_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    clear_button_texture_impl(state, id, "NormalTexture")?;
    Ok(0)
}

fn clear_highlight_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    clear_button_texture_impl(state, id, "HighlightTexture")?;
    Ok(0)
}

fn clear_pushed_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    clear_button_texture_impl(state, id, "PushedTexture")?;
    Ok(0)
}

fn clear_disabled_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    clear_button_texture_impl(state, id, "DisabledTexture")?;
    Ok(0)
}

// ── Three-slice methods ───────────────────────────────────────────────────────

/// SetLeftTexture(texture)
fn set_left_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let path = opt_string(state, 2);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.left_texture = path;
    }
    Ok(0)
}

/// SetMiddleTexture(texture)
fn set_middle_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let path = opt_string(state, 2);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.middle_texture = path;
    }
    Ok(0)
}

/// SetRightTexture(texture)
fn set_right_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let path = opt_string(state, 2);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.right_texture = path;
    }
    Ok(0)
}

// ── FontString methods ────────────────────────────────────────────────────────

/// GetFontString() -> fontstring
fn get_font_string(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let text_id = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .and_then(|f| f.children_keys.get("Text").copied())
    };
    match text_id {
        Some(tid) => {
            let val = frame_ref(state, tid)?;
            state.push(val);
            Ok(1)
        }
        None => {
            // TODO: check mixin override for GetFontString
            state.push(Val::Nil);
            Ok(1)
        }
    }
}

/// SetFontString(fontstring)
fn set_font_string(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let fontstring_val = Val::from_stack(state, 2)?;
    let fs_id_opt = extract_frame_id(state, fontstring_val);
    if let Some(fs_id) = fs_id_opt {
        let mut sim = borrow_state_mut(state)?;
        super::methods_hierarchy::reparent_widget(&mut sim.widgets, fs_id, Some(id));
        if let Some(fs) = sim.widgets.get_mut_visual(fs_id) {
            fs.anchors.clear();
            super::methods_helpers::set_all_points_anchors_pub(fs, id);
        }
        if let Some(btn) = sim.widgets.get_mut_visual(id) {
            btn.children_keys.insert("Text".to_string(), fs_id);
        }
        if let Some(fs) = sim.widgets.get_mut_visual(fs_id) {
            fs.parent_key = Some("Text".to_string());
        }
        drop(sim);
        let _ = sync_child_to_rilua(state, id, "Text", fs_id);
    } else {
        let mut sim = borrow_state_mut(state)?;
        if let Some(btn) = sim.widgets.get_mut_visual(id) {
            btn.children_keys.remove("Text");
        }
    }
    Ok(0)
}

// ── Anchor methods ────────────────────────────────────────────────────────────

/// ClearAllPoints()
fn clear_all_points(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    // TODO: combat lockdown check (requires Lua call)
    let already_empty = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|f| f.anchors.is_empty())
            .unwrap_or(true)
    };
    if !already_empty {
        let mut sim = borrow_state_mut(state)?;
        sim.widgets.remove_all_anchor_dependents_for(id);
        if let Some(frame) = sim.widgets.get_mut_visual(id) {
            frame.clear_all_points();
        }
        sim.widgets.mark_rect_dirty(id);
    }
    Ok(0)
}

/// ClearPoint(pointName)
fn clear_point(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let point_name = String::from_stack(state, 2)?;
    let Some(point) = crate::widget::AnchorPoint::from_str(&point_name) else {
        return Ok(0);
    };
    let mut sim = borrow_state_mut(state)?;
    let target_id = sim
        .widgets
        .get(id)
        .and_then(|f| f.anchors.iter().find(|a| a.point == point))
        .and_then(|a| a.relative_to_id);
    if let Some(target) = target_id {
        sim.widgets.remove_anchor_dependent(target as u64, id);
    }
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.anchors.retain(|a| a.point != point);
    }
    sim.widgets.mark_rect_dirty(id);
    Ok(0)
}

/// ClearPointsOffset() — no-op stub
fn clear_points_offset(state: &mut LuaState) -> LuaResult<u32> {
    let _id = frame_id_from_stack(state, 1)?;
    Ok(0)
}

/// AdjustPointsOffset(x, y)
fn adjust_points_offset(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let x_offset = f64::from_stack(state, 2)? as f32;
    let y_offset = f64::from_stack(state, 3)? as f32;
    // TODO: combat lockdown check (requires Lua call)
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        for anchor in &mut frame.anchors {
            anchor.x_offset += x_offset;
            anchor.y_offset += y_offset;
        }
    }
    sim.widgets.mark_rect_dirty(id);
    Ok(0)
}

/// GetNumPoints() -> count
fn get_num_points(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let count = {
        let sim = borrow_state(state)?;
        sim.widgets.get(id).map(|f| f.anchors.len()).unwrap_or(0) as i32
    };
    count.into_stack(state)
}

/// GetPoint([index]) -> point, relativeTo, relativePoint, xOfs, yOfs
fn get_point(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let index = opt_f32(state, 2).map(|n| n as i32).unwrap_or(1);
    let idx = (index - 1).max(0) as usize;
    let anchor_data = {
        let sim = borrow_state(state)?;
        let Some(frame) = sim.widgets.get(id) else {
            return Ok(0);
        };
        let mut sorted: Vec<_> = frame.anchors.iter().collect();
        sorted.sort_by_key(|a| a.point.sort_key());
        let Some(anchor) = sorted.get(idx) else {
            return Ok(0);
        };
        (
            anchor.point,
            anchor.relative_to_id,
            anchor.relative_point,
            anchor.x_offset,
            anchor.y_offset,
        )
    };
    let (point, relative_to_id, relative_point, x_offset, y_offset) = anchor_data;
    let point_str = create_string(state, point.as_str());
    state.push(point_str);
    match relative_to_id {
        Some(rid) => {
            let rel_val = frame_ref(state, rid as u64)?;
            state.push(rel_val);
        }
        None => state.push(Val::Nil),
    }
    let rel_point_str = create_string(state, relative_point.as_str());
    state.push(rel_point_str);
    state.push(Val::Num(x_offset as f64));
    state.push(Val::Num(y_offset as f64));
    Ok(5)
}

/// GetPointByName(pointName) -> point, relativeTo, relativePoint, xOfs, yOfs
fn get_point_by_name(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let point_name = String::from_stack(state, 2)?;
    let point_upper = point_name.to_uppercase();
    let anchor_data = {
        let sim = borrow_state(state)?;
        let Some(frame) = sim.widgets.get(id) else {
            return Ok(0);
        };
        frame
            .anchors
            .iter()
            .find(|a| a.point.as_str().to_uppercase() == point_upper)
            .map(|a| {
                (
                    a.point,
                    a.relative_to_id,
                    a.relative_point,
                    a.x_offset,
                    a.y_offset,
                )
            })
    };
    let Some((point, relative_to_id, relative_point, x_offset, y_offset)) = anchor_data else {
        return Ok(0);
    };
    let point_str = create_string(state, point.as_str());
    state.push(point_str);
    match relative_to_id {
        Some(rid) => {
            let rel_val = frame_ref(state, rid as u64)?;
            state.push(rel_val);
        }
        None => state.push(Val::Nil),
    }
    let rel_point_str = create_string(state, relative_point.as_str());
    state.push(rel_point_str);
    state.push(Val::Num(x_offset as f64));
    state.push(Val::Num(y_offset as f64));
    Ok(5)
}

/// SetPoint(point [, relativeTo [, relativePoint]] [, xOfs, yOfs])
///
/// TODO: This requires variadic argument parsing and combat lockdown checks
/// that depend on Lua execution. The full implementation is deferred.
fn set_point(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let point_name = String::from_stack(state, 2)?;
    let Some(point) = crate::widget::AnchorPoint::from_str(&point_name) else {
        return Err(runtime_error(format!(
            "Frame:SetPoint(): Invalid region point {point_name}"
        )));
    };
    // TODO: full variadic arg parsing (relativeTo as frame/string/nil, relativePoint, x, y)
    // TODO: combat lockdown check
    // TODO: cycle detection
    // Minimal implementation: apply with parent as relativeTo and zero offsets
    let relative_to = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .and_then(|f| f.parent_id)
            .map(|pid| pid as usize)
    };
    let mut sim = borrow_state_mut(state)?;
    if let Some(old) = sim.widgets.get(id).and_then(|f| {
        f.anchors
            .iter()
            .find(|a| a.point == point)
            .map(|a| a.relative_to_id)
    }) {
        if let Some(old_target) = old {
            sim.widgets.remove_anchor_dependent(old_target as u64, id);
        }
    }
    if let Some(rel_id) = relative_to {
        sim.widgets.add_anchor_dependent(rel_id as u64, id);
    }
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.set_point(point, relative_to, point, 0.0, 0.0);
    }
    sim.widgets.mark_rect_dirty(id);
    Ok(0)
}

/// SetAllPoints([relativeTo])
///
/// TODO: Full arg parsing (bool, frame, nil, string).
fn set_all_points(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let arg = Val::from_stack(state, 2)?;
    let relative_to_id: Option<usize> = match arg {
        Val::Bool(false) => return Ok(0),
        _ if extract_frame_id(state, arg).is_some() => {
            extract_frame_id(state, arg).map(|rid| rid as usize)
        }
        _ => {
            let sim = borrow_state(state)?;
            sim.widgets
                .get(id)
                .and_then(|f| f.parent_id)
                .map(|p| p as usize)
        }
    };
    let mut sim = borrow_state_mut(state)?;
    sim.widgets.remove_all_anchor_dependents_for(id);
    if let Some(rel_id) = relative_to_id {
        sim.widgets.add_anchor_dependent(rel_id as u64, id);
    }
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.clear_all_points();
        frame.set_point(
            crate::widget::AnchorPoint::TopLeft,
            relative_to_id,
            crate::widget::AnchorPoint::TopLeft,
            0.0,
            0.0,
        );
        frame.set_point(
            crate::widget::AnchorPoint::BottomRight,
            relative_to_id,
            crate::widget::AnchorPoint::BottomRight,
            0.0,
            0.0,
        );
    }
    sim.widgets.mark_rect_dirty(id);
    Ok(0)
}

// ── Hierarchy methods ─────────────────────────────────────────────────────────

/// GetParent() -> parent
fn get_parent(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let parent_id = {
        let sim = borrow_state(state)?;
        sim.widgets.get(id).and_then(|f| f.parent_id)
    };
    match parent_id {
        Some(pid) => {
            let val = frame_ref(state, pid)?;
            state.push(val);
            Ok(1)
        }
        None => {
            state.push(Val::Nil);
            Ok(1)
        }
    }
}

/// SetParent(parent)
fn set_parent(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    // TODO: combat lockdown check
    let parent_val = Val::from_stack(state, 2)?;
    let new_parent_id = extract_frame_id(state, parent_val);
    let mut sim = borrow_state_mut(state)?;
    super::methods_hierarchy::reparent_widget(&mut sim.widgets, id, new_parent_id);
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.default_parent = false;
    }
    sim.visible_on_update_cache = None;
    sim.widgets.mark_rect_dirty(id);
    Ok(0)
}

/// GetNumChildren() -> count
fn get_num_children(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let count = {
        let sim = borrow_state(state)?;
        sim.widgets.get(id).map(|f| f.children.len()).unwrap_or(0) as i32
    };
    count.into_stack(state)
}

/// GetChildren() -> child1, child2, ...
fn get_children(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let children = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|f| f.children.clone())
            .unwrap_or_default()
    };
    let count = children.len() as u32;
    for child_id in children {
        let val = frame_ref(state, child_id)?;
        state.push(val);
    }
    Ok(count)
}

/// GetNumRegions() -> count
fn get_num_regions(state: &mut LuaState) -> LuaResult<u32> {
    use crate::widget::WidgetType;
    let id = frame_id_from_stack(state, 1)?;
    let count = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|f| {
                f.children
                    .iter()
                    .filter(|&&cid| {
                        sim.widgets
                            .get(cid)
                            .map(|c| {
                                matches!(
                                    c.widget_type,
                                    WidgetType::Texture | WidgetType::FontString | WidgetType::Line
                                )
                            })
                            .unwrap_or(false)
                    })
                    .count()
            })
            .unwrap_or(0) as i32
    };
    count.into_stack(state)
}

/// GetRegions() -> region1, region2, ...
fn get_regions(state: &mut LuaState) -> LuaResult<u32> {
    use crate::widget::WidgetType;
    let id = frame_id_from_stack(state, 1)?;
    let children = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|f| f.children.clone())
            .unwrap_or_default()
    };
    let mut count = 0u32;
    for child_id in children {
        let is_region = {
            let sim = borrow_state(state)?;
            sim.widgets
                .get(child_id)
                .map(|c| {
                    matches!(
                        c.widget_type,
                        WidgetType::Texture | WidgetType::FontString | WidgetType::Line
                    )
                })
                .unwrap_or(false)
        };
        if is_region {
            let val = frame_ref(state, child_id)?;
            state.push(val);
            count += 1;
        }
    }
    Ok(count)
}

/// GetAdditionalRegions() -> (none)
fn get_additional_regions(state: &mut LuaState) -> LuaResult<u32> {
    let _id = frame_id_from_stack(state, 1)?;
    Ok(0)
}

/// GetParentKey() -> key
fn get_parent_key(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let key = {
        let sim = borrow_state(state)?;
        sim.widgets.get(id).and_then(|f| f.parent_key.clone())
    };
    match key {
        Some(k) => {
            let val = create_string(state, &k);
            state.push(val);
            Ok(1)
        }
        None => {
            state.push(Val::Nil);
            Ok(1)
        }
    }
}

/// SetParentKey(key [, removeOld])
fn set_parent_key(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let key = String::from_stack(state, 2)?;
    let _remove_old = bool::from_stack(state, 3)?; // optional, defaults false
    let parent_id = {
        let sim = borrow_state(state)?;
        sim.widgets.get(id).and_then(|f| f.parent_id)
    };
    let Some(pid) = parent_id else {
        return Ok(0);
    };
    // TODO: remove old parent keys if remove_old is true (requires Lua table operations)
    let child_val = frame_ref(state, id)?;
    let parent_val = frame_ref(state, pid)?;
    // TODO: set parent_table[key] = child_val (requires Table rawset on rilua table)
    let _ = (child_val, parent_val, key);
    Ok(0)
}

// ── Create methods ────────────────────────────────────────────────────────────

/// CreateTexture([name [, layer [, inherits [, subLevel]]]]) -> texture
fn create_texture(state: &mut LuaState) -> LuaResult<u32> {
    use crate::widget::{DrawLayer, Frame, WidgetType};
    let parent_id = frame_id_from_stack(state, 1)?;
    let name_raw: Option<String> = Option::<String>::from_stack(state, 2)?;
    let layer = opt_string(state, 3);
    let _inherits = opt_string(state, 4);
    let sub_level = opt_f32(state, 5).map(|n| n as i32);

    let name = name_raw.map(|n| {
        let sim = borrow_state(state).ok();
        if let Some(sim) = sim {
            crate::lua_api::globals::create_frame::apply_parent_sub(&n, Some(parent_id), &sim)
        } else {
            n
        }
    });

    let mut texture = Frame::new(WidgetType::Texture, name.clone(), Some(parent_id));
    if let Some(layer_str) = layer {
        if let Some(draw_layer) = DrawLayer::from_str(&layer_str) {
            texture.draw_layer = draw_layer;
        }
    }
    if let Some(sub_level) = sub_level {
        texture.draw_sub_layer = sub_level;
    }

    let child_id = texture.id;
    {
        let mut sim = borrow_state_mut(state)?;
        sim.widgets.register(texture);
        sim.widgets.add_child(parent_id, child_id);
        sim.invalidate_strata_buckets();
        let parent_props = sim
            .widgets
            .get(parent_id)
            .map(|p| (p.frame_strata, p.frame_level));
        if let Some((parent_strata, parent_level)) = parent_props {
            if let Some(f) = sim.widgets.get_mut_visual(child_id) {
                f.frame_strata = parent_strata;
                f.frame_level = parent_level + 1;
            }
        }
    }

    if let Some(ref n) = name {
        bind_named_child_global(state, n, child_id)?;
    }

    let val = frame_ref(state, child_id)?;
    state.push(val);
    Ok(1)
}

/// CreateMaskTexture([name]) -> masktexture
fn create_mask_texture(state: &mut LuaState) -> LuaResult<u32> {
    use crate::widget::{Frame, WidgetType};
    let parent_id = frame_id_from_stack(state, 1)?;
    let name_raw: Option<String> = Option::<String>::from_stack(state, 2)?;
    let name = name_raw.map(|n| {
        let sim = borrow_state(state).ok();
        if let Some(sim) = sim {
            crate::lua_api::globals::create_frame::apply_parent_sub(&n, Some(parent_id), &sim)
        } else {
            n
        }
    });
    let mut texture = Frame::new(WidgetType::Texture, name.clone(), Some(parent_id));
    texture.is_mask = true;
    texture.object_type_name = Some("MaskTexture".to_string());
    let child_id = texture.id;
    {
        let mut sim = borrow_state_mut(state)?;
        sim.widgets.register(texture);
        sim.widgets.add_child(parent_id, child_id);
        sim.invalidate_strata_buckets();
    }
    let val = frame_ref(state, child_id)?;
    state.push(val);
    Ok(1)
}

/// AddMaskTexture(maskTexture)
fn add_mask_texture(state: &mut LuaState) -> LuaResult<u32> {
    let texture_id = frame_id_from_stack(state, 1)?;
    let mask_id = extract_frame_id(state, Val::from_stack(state, 2)?)
        .ok_or_else(|| runtime_error("expected mask texture"))?;

    let mut sim = borrow_state_mut(state)?;
    let is_mask = sim.widgets.get(mask_id).map(|f| f.is_mask).unwrap_or(false);
    if !is_mask {
        return Err(runtime_error("expected mask texture"));
    }

    if let Some(texture) = sim.widgets.get_mut_visual(texture_id)
        && !texture.mask_textures.contains(&mask_id)
    {
        texture.mask_textures.push(mask_id);
    }

    Ok(0)
}

/// RemoveMaskTexture(maskTexture)
fn remove_mask_texture(state: &mut LuaState) -> LuaResult<u32> {
    let texture_id = frame_id_from_stack(state, 1)?;
    let Some(mask_id) = extract_frame_id(state, Val::from_stack(state, 2)?) else {
        return Ok(0);
    };

    let mut sim = borrow_state_mut(state)?;
    if let Some(texture) = sim.widgets.get_mut_visual(texture_id) {
        texture.mask_textures.retain(|id| *id != mask_id);
    }

    Ok(0)
}

/// GetNumMaskTextures() -> count
fn get_num_mask_textures(state: &mut LuaState) -> LuaResult<u32> {
    let texture_id = frame_id_from_stack(state, 1)?;
    let count = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(texture_id)
            .map(|f| f.mask_textures.len())
            .unwrap_or(0)
    };
    state.push(Val::Num(count as f64));
    Ok(1)
}

/// GetMaskTexture(index) -> maskTexture|nil
fn get_mask_texture(state: &mut LuaState) -> LuaResult<u32> {
    let texture_id = frame_id_from_stack(state, 1)?;
    let index = i64::from_stack(state, 2).unwrap_or(1);
    let mask_id = {
        let sim = borrow_state(state)?;
        if index <= 0 {
            None
        } else {
            sim.widgets
                .get(texture_id)
                .and_then(|f| f.mask_textures.get((index - 1) as usize).copied())
        }
    };

    if let Some(mask_id) = mask_id {
        let mask_ref = frame_ref(state, mask_id)?;
        state.push(mask_ref);
    } else {
        state.push(Val::Nil);
    }
    Ok(1)
}

/// CreateLine([name [, layer [, inherits]]]) -> line
fn create_line(state: &mut LuaState) -> LuaResult<u32> {
    use crate::widget::{DrawLayer, Frame, WidgetType};
    let parent_id = frame_id_from_stack(state, 1)?;
    let name_raw: Option<String> = Option::<String>::from_stack(state, 2)?;
    let layer = opt_string(state, 3);
    let _inherits = opt_string(state, 4);
    let name = name_raw.map(|n| {
        let sim = borrow_state(state).ok();
        if let Some(sim) = sim {
            crate::lua_api::globals::create_frame::apply_parent_sub(&n, Some(parent_id), &sim)
        } else {
            n
        }
    });
    let mut line = Frame::new(WidgetType::Line, name.clone(), Some(parent_id));
    if let Some(layer_str) = layer {
        if let Some(draw_layer) = DrawLayer::from_str(&layer_str) {
            line.draw_layer = draw_layer;
        }
    }
    let child_id = line.id;
    {
        let mut sim = borrow_state_mut(state)?;
        sim.widgets.register(line);
        sim.widgets.add_child(parent_id, child_id);
        sim.invalidate_strata_buckets();
    }
    // TODO: apply templates from registry if inherits is set
    let val = frame_ref(state, child_id)?;
    state.push(val);
    Ok(1)
}

/// CreateFontString([name [, layer [, inherits]]]) -> fontstring
fn create_font_string(state: &mut LuaState) -> LuaResult<u32> {
    use crate::widget::{DrawLayer, Frame, WidgetType};
    let parent_id = frame_id_from_stack(state, 1)?;
    let name_raw: Option<String> = Option::<String>::from_stack(state, 2)?;
    let layer = opt_string(state, 3);
    let inherits = opt_string(state, 4);
    let name = name_raw.map(|n| {
        let sim = borrow_state(state).ok();
        if let Some(sim) = sim {
            crate::lua_api::globals::create_frame::apply_parent_sub(&n, Some(parent_id), &sim)
        } else {
            n
        }
    });
    let mut fontstring = Frame::new(WidgetType::FontString, name.clone(), Some(parent_id));
    if let Some(layer_str) = layer {
        if let Some(draw_layer) = DrawLayer::from_str(&layer_str) {
            fontstring.draw_layer = draw_layer;
        }
    }
    // TODO: apply_font_inherit — requires mlua globals lookup for font object
    let _ = inherits;
    let child_id = fontstring.id;
    {
        let mut sim = borrow_state_mut(state)?;
        sim.widgets.register(fontstring);
        sim.widgets.add_child(parent_id, child_id);
        sim.invalidate_strata_buckets();
    }
    let val = frame_ref(state, child_id)?;
    state.push(val);
    Ok(1)
}

/// AttachTexture() -> texture
fn attach_texture(state: &mut LuaState) -> LuaResult<u32> {
    use crate::widget::{Frame, WidgetType};
    let parent_id = frame_id_from_stack(state, 1)?;
    let texture = Frame::new(WidgetType::Texture, None, Some(parent_id));
    let child_id = texture.id;
    {
        let mut sim = borrow_state_mut(state)?;
        sim.widgets.register(texture);
        sim.widgets.add_child(parent_id, child_id);
        sim.invalidate_strata_buckets();
    }
    let val = frame_ref(state, child_id)?;
    state.push(val);
    Ok(1)
}

/// AttachFontString() -> fontstring
fn attach_font_string(state: &mut LuaState) -> LuaResult<u32> {
    use crate::widget::{Frame, WidgetType};
    let parent_id = frame_id_from_stack(state, 1)?;
    let fontstring = Frame::new(WidgetType::FontString, None, Some(parent_id));
    let child_id = fontstring.id;
    {
        let mut sim = borrow_state_mut(state)?;
        sim.widgets.register(fontstring);
        sim.widgets.add_child(parent_id, child_id);
        sim.invalidate_strata_buckets();
    }
    let val = frame_ref(state, child_id)?;
    state.push(val);
    Ok(1)
}

/// GetAnimationGroups() -> group1, group2, ...
fn get_animation_groups(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let ag_frame_ids: Vec<u64> = {
        let sim = borrow_state(state)?;
        sim.anim_frame_to_group
            .iter()
            .filter(|&(_, &gid)| {
                sim.animation_groups
                    .get(&gid)
                    .is_some_and(|g| g.owner_frame_id == id)
            })
            .map(|(&fid, _)| fid)
            .collect()
    };
    let count = ag_frame_ids.len() as u32;
    for fid in ag_frame_ids {
        let val = frame_ref(state, fid)?;
        state.push(val);
    }
    Ok(count)
}

/// CreateAnimationGroup([name [, inherits]]) -> animationGroup
fn create_animation_group(state: &mut LuaState) -> LuaResult<u32> {
    use crate::lua_api::animation::AnimGroupState;
    use crate::widget::{Frame, WidgetType};
    let parent_id = frame_id_from_stack(state, 1)?;
    let name_raw: Option<String> = Option::<String>::from_stack(state, 2)?;
    let _inherits: Option<String> = Option::<String>::from_stack(state, 3)?;
    let name = name_raw.map(|n| {
        let sim = borrow_state(state).ok();
        if let Some(sim) = sim {
            crate::lua_api::globals::create_frame::apply_parent_sub(&n, Some(parent_id), &sim)
        } else {
            n
        }
    });
    let mut child = Frame::new(WidgetType::Frame, name.clone(), Some(parent_id));
    child.object_type_name = Some("AnimationGroup".to_string());
    let child_id = child.id;
    {
        let mut sim = borrow_state_mut(state)?;
        let gid = sim.next_anim_group_id;
        sim.next_anim_group_id += 1;
        let mut group = AnimGroupState::new(parent_id);
        group.name = name.clone();
        group.frame_id = Some(child_id);
        sim.animation_groups.insert(gid, group);
        sim.anim_frame_to_group.insert(child_id, gid);
        sim.widgets.register(child);
        sim.widgets.add_child(parent_id, child_id);
        sim.invalidate_strata_buckets();
    }
    let val = frame_ref(state, child_id)?;
    state.push(val);
    Ok(1)
}

/// CreateAnimation([type [, name]]) -> animation
fn create_animation(state: &mut LuaState) -> LuaResult<u32> {
    use crate::lua_api::animation::{AnimState, AnimationType};
    use crate::widget::{Frame, WidgetType};
    let group_frame_id = frame_id_from_stack(state, 1)?;
    let anim_type_str = opt_string(state, 2);
    let anim_name_raw: Option<String> = Option::<String>::from_stack(state, 3)?;

    let group_id = {
        let sim = borrow_state(state)?;
        sim.anim_frame_to_group
            .get(&group_frame_id)
            .copied()
            .ok_or_else(|| runtime_error("CreateAnimation called on non-AnimationGroup"))?
    };
    let anim_type = AnimationType::from_str(anim_type_str.as_deref().unwrap_or("Animation"));
    let type_name = anim_type.as_str().to_string();
    let name = anim_name_raw.map(|n| {
        let sim = borrow_state(state).ok();
        if let Some(sim) = sim {
            crate::lua_api::globals::create_frame::apply_parent_sub(&n, Some(group_frame_id), &sim)
        } else {
            n
        }
    });
    let mut child = Frame::new(WidgetType::Frame, name.clone(), Some(group_frame_id));
    child.object_type_name = Some(type_name);
    let child_id = child.id;
    let mut anim = AnimState::new(anim_type);
    anim.name = name;
    {
        let mut sim = borrow_state_mut(state)?;
        let group = sim
            .animation_groups
            .get_mut(&group_id)
            .ok_or_else(|| runtime_error("Animation group not found"))?;
        let idx = group.animations.len();
        group.animations.push(anim);
        sim.anim_frame_to_anim.insert(child_id, (group_id, idx));
        sim.widgets.register(child);
        sim.widgets.add_child(group_frame_id, child_id);
        sim.invalidate_strata_buckets();
    }
    let val = frame_ref(state, child_id)?;
    state.push(val);
    Ok(1)
}

/// CreateControlPoint([name]) -> controlPoint
fn create_control_point(state: &mut LuaState) -> LuaResult<u32> {
    use crate::widget::{Frame, WidgetType};
    let parent_id = frame_id_from_stack(state, 1)?;
    let name_raw: Option<String> = Option::<String>::from_stack(state, 2)?;
    let name = name_raw.map(|n| {
        let sim = borrow_state(state).ok();
        if let Some(sim) = sim {
            crate::lua_api::globals::create_frame::apply_parent_sub(&n, Some(parent_id), &sim)
        } else {
            n
        }
    });
    let mut child = Frame::new(WidgetType::Frame, name.clone(), Some(parent_id));
    child.object_type_name = Some("ControlPoint".to_string());
    let child_id = child.id;
    {
        let mut sim = borrow_state_mut(state)?;
        sim.widgets.register(child);
        sim.widgets.add_child(parent_id, child_id);
        sim.invalidate_strata_buckets();
    }
    let val = frame_ref(state, child_id)?;
    state.push(val);
    Ok(1)
}

fn animation_set_duration(state: &mut LuaState) -> LuaResult<u32> {
    let animation_frame_id = frame_id_from_stack(state, 1)?;
    let duration = match stack_val(state, 2) {
        Val::Num(value) => value.max(0.0),
        _ => 0.0,
    };
    let mut sim = borrow_state_mut(state)?;
    if let Some((group_id, animation_index)) =
        sim.anim_frame_to_anim.get(&animation_frame_id).copied()
        && let Some(group) = sim.animation_groups.get_mut(&group_id)
        && let Some(animation) = group.animations.get_mut(animation_index)
    {
        animation.duration = duration;
    }
    Ok(0)
}

fn animation_set_order(state: &mut LuaState) -> LuaResult<u32> {
    let animation_frame_id = frame_id_from_stack(state, 1)?;
    let order = match stack_val(state, 2) {
        Val::Num(value) if value >= 0.0 => value as u32,
        _ => 0,
    };
    let mut sim = borrow_state_mut(state)?;
    if let Some((group_id, animation_index)) =
        sim.anim_frame_to_anim.get(&animation_frame_id).copied()
        && let Some(group) = sim.animation_groups.get_mut(&group_id)
        && let Some(animation) = group.animations.get_mut(animation_index)
    {
        animation.order = order;
    }
    Ok(0)
}

fn animation_set_start_delay(state: &mut LuaState) -> LuaResult<u32> {
    let animation_frame_id = frame_id_from_stack(state, 1)?;
    let start_delay = match stack_val(state, 2) {
        Val::Num(value) => value.max(0.0),
        _ => 0.0,
    };
    let mut sim = borrow_state_mut(state)?;
    if let Some((group_id, animation_index)) =
        sim.anim_frame_to_anim.get(&animation_frame_id).copied()
        && let Some(group) = sim.animation_groups.get_mut(&group_id)
        && let Some(animation) = group.animations.get_mut(animation_index)
    {
        animation.start_delay = start_delay;
    }
    Ok(0)
}

fn animation_set_end_delay(state: &mut LuaState) -> LuaResult<u32> {
    let animation_frame_id = frame_id_from_stack(state, 1)?;
    let end_delay = match stack_val(state, 2) {
        Val::Num(value) => value.max(0.0),
        _ => 0.0,
    };
    let mut sim = borrow_state_mut(state)?;
    if let Some((group_id, animation_index)) =
        sim.anim_frame_to_anim.get(&animation_frame_id).copied()
        && let Some(group) = sim.animation_groups.get_mut(&group_id)
        && let Some(animation) = group.animations.get_mut(animation_index)
    {
        animation.end_delay = end_delay;
    }
    Ok(0)
}

fn animation_group_set_looping(state: &mut LuaState) -> LuaResult<u32> {
    let group_frame_id = frame_id_from_stack(state, 1)?;
    let looping = opt_string(state, 2).unwrap_or_default();
    let mut sim = borrow_state_mut(state)?;
    if let Some(group_id) = sim.anim_frame_to_group.get(&group_frame_id).copied()
        && let Some(group) = sim.animation_groups.get_mut(&group_id)
    {
        group.looping = crate::lua_api::animation::LoopType::from_str(&looping);
    }
    Ok(0)
}

fn animation_group_play(state: &mut LuaState) -> LuaResult<u32> {
    let group_frame_id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(group_id) = sim.anim_frame_to_group.get(&group_frame_id).copied()
        && let Some(group) = sim.animation_groups.get_mut(&group_id)
    {
        group.playing = true;
        group.paused = false;
        group.done = false;
        group.pending_finish = false;
    }
    Ok(0)
}

fn animation_group_stop(state: &mut LuaState) -> LuaResult<u32> {
    let group_frame_id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(group_id) = sim.anim_frame_to_group.get(&group_frame_id).copied()
        && let Some(group) = sim.animation_groups.get_mut(&group_id)
    {
        group.playing = false;
        group.paused = false;
        group.pending_finish = false;
        group.elapsed = 0.0;
        for animation in &mut group.animations {
            animation.elapsed = 0.0;
        }
    }
    Ok(0)
}

fn animation_group_is_playing(state: &mut LuaState) -> LuaResult<u32> {
    let group_frame_id = frame_id_from_stack(state, 1)?;
    let playing = {
        let sim = borrow_state(state)?;
        sim.anim_frame_to_group
            .get(&group_frame_id)
            .and_then(|group_id| {
                sim.animation_groups
                    .get(group_id)
                    .map(|group| group.playing)
            })
            .unwrap_or(false)
    };
    state.push(Val::Bool(playing));
    Ok(1)
}

fn animation_config_noop(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn animation_group_set_to_final_alpha(state: &mut LuaState) -> LuaResult<u32> {
    let group_frame_id = frame_id_from_stack(state, 1)?;
    let set_to_final_alpha = matches!(stack_val(state, 2), Val::Bool(true));
    let mut sim = borrow_state_mut(state)?;
    if let Some(group_id) = sim.anim_frame_to_group.get(&group_frame_id).copied()
        && let Some(group) = sim.animation_groups.get_mut(&group_id)
    {
        group.set_to_final_alpha = set_to_final_alpha;
    }
    Ok(0)
}

// ── register_all ──────────────────────────────────────────────────────────────

/// Register all button, anchor, hierarchy, and create methods on the given metatable.
pub fn register_all(state: &mut LuaState, table: GcRef<Table>) -> LuaResult<()> {
    // Button: font objects
    table_set_rust_fn(state, table, "SetNormalFontObject", set_normal_font_object)?;
    table_set_rust_fn(state, table, "GetNormalFontObject", get_normal_font_object)?;
    table_set_rust_fn(
        state,
        table,
        "SetHighlightFontObject",
        set_highlight_font_object,
    )?;
    table_set_rust_fn(
        state,
        table,
        "GetHighlightFontObject",
        get_highlight_font_object,
    )?;
    table_set_rust_fn(
        state,
        table,
        "SetDisabledFontObject",
        set_disabled_font_object,
    )?;
    table_set_rust_fn(
        state,
        table,
        "GetDisabledFontObject",
        get_disabled_font_object,
    )?;

    // Button: pushed text offset
    table_set_rust_fn(state, table, "SetPushedTextOffset", set_pushed_text_offset)?;
    table_set_rust_fn(state, table, "GetPushedTextOffset", get_pushed_text_offset)?;

    // Button: texture getters
    table_set_rust_fn(state, table, "GetNormalTexture", get_normal_texture)?;
    table_set_rust_fn(state, table, "GetHighlightTexture", get_highlight_texture)?;
    table_set_rust_fn(state, table, "GetPushedTexture", get_pushed_texture)?;
    table_set_rust_fn(state, table, "GetDisabledTexture", get_disabled_texture)?;

    // Button: texture setters
    table_set_rust_fn(state, table, "SetNormalTexture", set_normal_texture)?;
    table_set_rust_fn(state, table, "SetHighlightTexture", set_highlight_texture)?;
    table_set_rust_fn(state, table, "SetPushedTexture", set_pushed_texture)?;
    table_set_rust_fn(state, table, "SetDisabledTexture", set_disabled_texture)?;

    // Button: atlas setters
    table_set_rust_fn(state, table, "SetNormalAtlas", set_normal_atlas)?;
    table_set_rust_fn(state, table, "SetPushedAtlas", set_pushed_atlas)?;
    table_set_rust_fn(state, table, "SetDisabledAtlas", set_disabled_atlas)?;
    table_set_rust_fn(state, table, "SetHighlightAtlas", set_highlight_atlas)?;

    // Button: checked textures
    table_set_rust_fn(state, table, "SetCheckedTexture", set_checked_texture)?;
    table_set_rust_fn(
        state,
        table,
        "SetDisabledCheckedTexture",
        set_disabled_checked_texture,
    )?;
    table_set_rust_fn(
        state,
        table,
        "GetDisabledCheckedTexture",
        get_disabled_checked_texture,
    )?;

    // Button: clear textures
    table_set_rust_fn(state, table, "ClearNormalTexture", clear_normal_texture)?;
    table_set_rust_fn(
        state,
        table,
        "ClearHighlightTexture",
        clear_highlight_texture,
    )?;
    table_set_rust_fn(state, table, "ClearPushedTexture", clear_pushed_texture)?;
    table_set_rust_fn(state, table, "ClearDisabledTexture", clear_disabled_texture)?;

    // Button: three-slice
    table_set_rust_fn(state, table, "SetLeftTexture", set_left_texture)?;
    table_set_rust_fn(state, table, "SetMiddleTexture", set_middle_texture)?;
    table_set_rust_fn(state, table, "SetRightTexture", set_right_texture)?;

    // Button: font string
    table_set_rust_fn(state, table, "GetFontString", get_font_string)?;
    table_set_rust_fn(state, table, "SetFontString", set_font_string)?;

    // Anchor methods
    table_set_rust_fn(state, table, "SetPoint", set_point)?;
    table_set_rust_fn(state, table, "ClearAllPoints", clear_all_points)?;
    table_set_rust_fn(state, table, "ClearPoint", clear_point)?;
    table_set_rust_fn(state, table, "ClearPointsOffset", clear_points_offset)?;
    table_set_rust_fn(state, table, "AdjustPointsOffset", adjust_points_offset)?;
    table_set_rust_fn(state, table, "SetAllPoints", set_all_points)?;
    table_set_rust_fn(state, table, "GetPoint", get_point)?;
    table_set_rust_fn(state, table, "GetNumPoints", get_num_points)?;
    table_set_rust_fn(state, table, "GetPointByName", get_point_by_name)?;

    // Hierarchy methods
    table_set_rust_fn(state, table, "GetParent", get_parent)?;
    table_set_rust_fn(state, table, "SetParent", set_parent)?;
    table_set_rust_fn(state, table, "GetNumChildren", get_num_children)?;
    table_set_rust_fn(state, table, "GetChildren", get_children)?;
    table_set_rust_fn(state, table, "GetNumRegions", get_num_regions)?;
    table_set_rust_fn(state, table, "GetRegions", get_regions)?;
    table_set_rust_fn(state, table, "GetAdditionalRegions", get_additional_regions)?;
    table_set_rust_fn(state, table, "GetParentKey", get_parent_key)?;
    table_set_rust_fn(state, table, "SetParentKey", set_parent_key)?;

    // Create methods
    table_set_rust_fn(state, table, "CreateTexture", create_texture)?;
    table_set_rust_fn(state, table, "CreateMaskTexture", create_mask_texture)?;
    table_set_rust_fn(state, table, "AddMaskTexture", add_mask_texture)?;
    table_set_rust_fn(state, table, "RemoveMaskTexture", remove_mask_texture)?;
    table_set_rust_fn(state, table, "GetNumMaskTextures", get_num_mask_textures)?;
    table_set_rust_fn(state, table, "GetMaskTexture", get_mask_texture)?;
    table_set_rust_fn(state, table, "CreateLine", create_line)?;
    table_set_rust_fn(state, table, "CreateFontString", create_font_string)?;
    table_set_rust_fn(state, table, "AttachTexture", attach_texture)?;
    table_set_rust_fn(state, table, "AttachFontString", attach_font_string)?;
    table_set_rust_fn(state, table, "GetAnimationGroups", get_animation_groups)?;
    table_set_rust_fn(state, table, "CreateAnimationGroup", create_animation_group)?;
    table_set_rust_fn(state, table, "CreateAnimation", create_animation)?;
    table_set_rust_fn(state, table, "Play", animation_group_play)?;
    table_set_rust_fn(state, table, "Stop", animation_group_stop)?;
    table_set_rust_fn(state, table, "IsPlaying", animation_group_is_playing)?;
    table_set_rust_fn(state, table, "SetLooping", animation_group_set_looping)?;
    table_set_rust_fn(state, table, "SetDuration", animation_set_duration)?;
    table_set_rust_fn(state, table, "SetOrder", animation_set_order)?;
    table_set_rust_fn(state, table, "SetStartDelay", animation_set_start_delay)?;
    table_set_rust_fn(state, table, "SetEndDelay", animation_set_end_delay)?;
    table_set_rust_fn(
        state,
        table,
        "SetToFinalAlpha",
        animation_group_set_to_final_alpha,
    )?;
    table_set_rust_fn(state, table, "SetSmoothing", animation_config_noop)?;
    table_set_rust_fn(state, table, "SetFromAlpha", animation_config_noop)?;
    table_set_rust_fn(state, table, "SetToAlpha", animation_config_noop)?;
    table_set_rust_fn(state, table, "SetOffset", animation_config_noop)?;
    table_set_rust_fn(state, table, "SetScale", animation_config_noop)?;
    table_set_rust_fn(state, table, "SetScaleFrom", animation_config_noop)?;
    table_set_rust_fn(state, table, "SetScaleTo", animation_config_noop)?;
    table_set_rust_fn(state, table, "SetDegrees", animation_config_noop)?;
    table_set_rust_fn(state, table, "SetChildKey", animation_config_noop)?;
    table_set_rust_fn(state, table, "SetTargetName", animation_config_noop)?;
    table_set_rust_fn(state, table, "SetTargetKey", animation_config_noop)?;
    table_set_rust_fn(state, table, "SetFlipBookRows", animation_config_noop)?;
    table_set_rust_fn(state, table, "SetFlipBookColumns", animation_config_noop)?;
    table_set_rust_fn(state, table, "CreateControlPoint", create_control_point)?;

    Ok(())
}
