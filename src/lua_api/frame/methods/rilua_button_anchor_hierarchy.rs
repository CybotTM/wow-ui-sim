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
    borrow_state, borrow_state_mut, call_function_state, create_string, extract_frame_id,
    frame_id_from_stack, frame_ref, registry_table_or_create, sync_child_to_rilua, table_get,
    table_set, val_to_string,
};
use crate::lua_api::rilua_script_helpers::{
    call_error_handler_state, get_script as get_rilua_script,
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

fn resolve_anchor_target_id(state: &mut LuaState, value: Val) -> Option<usize> {
    if let Some(id) = extract_frame_id(state, value) {
        return Some(id as usize);
    }

    let name = val_to_string(state, value)?;
    let key_ref = state.gc.intern_string(name.as_bytes());
    let global = state
        .gc
        .tables
        .get(state.global)
        .map(|table| table.get_str(key_ref, &state.gc.string_arena))
        .unwrap_or(Val::Nil);
    extract_frame_id(state, global).map(|id| id as usize)
}

fn resolve_relative_point_from_val(
    state: &mut LuaState,
    value: Val,
    default: crate::widget::AnchorPoint,
) -> LuaResult<crate::widget::AnchorPoint> {
    match value {
        Val::Nil => Ok(default),
        Val::Str(_) => {
            let point_name = val_to_string(state, value).unwrap_or_default();
            crate::widget::AnchorPoint::from_str(&point_name).ok_or_else(|| {
                runtime_error(format!(
                    "Frame:SetPoint(): Unknown region point {point_name}"
                ))
            })
        }
        _ => Ok(default),
    }
}

fn parse_set_point_args(
    state: &mut LuaState,
    point: crate::widget::AnchorPoint,
) -> LuaResult<(Option<usize>, crate::widget::AnchorPoint, f32, f32)> {
    let arg3 = stack_val(state, 3);
    let arg4 = stack_val(state, 4);
    let arg5 = stack_val(state, 5);
    let arg6 = stack_val(state, 6);

    if arg3 == Val::Nil {
        return Ok((None, point, 0.0, 0.0));
    }

    let x = match arg3 {
        Val::Num(n) => Some(n as f32),
        _ => None,
    };
    let y = match arg4 {
        Val::Num(n) => Some(n as f32),
        _ => None,
    };
    if let (Some(x_offset), Some(y_offset)) = (x, y) {
        return Ok((None, point, x_offset, y_offset));
    }

    let relative_to = resolve_anchor_target_id(state, arg3);
    if matches!(arg4, Val::Num(_)) {
        let x_offset = match arg4 {
            Val::Num(n) => n as f32,
            _ => 0.0,
        };
        let y_offset = match arg5 {
            Val::Num(n) => n as f32,
            _ => 0.0,
        };
        return Ok((relative_to, point, x_offset, y_offset));
    }

    let relative_point = resolve_relative_point_from_val(state, arg4, point)?;
    let x_offset = match arg5 {
        Val::Num(n) => n as f32,
        _ => 0.0,
    };
    let y_offset = match arg6 {
        Val::Num(n) => n as f32,
        _ => 0.0,
    };
    Ok((relative_to, relative_point, x_offset, y_offset))
}

fn parse_line_anchor_args(
    state: &mut LuaState,
) -> LuaResult<(crate::widget::AnchorPoint, Option<u64>, f32, f32)> {
    let point_name = String::from_stack(state, 2)?;
    let point = crate::widget::AnchorPoint::from_str(&point_name).ok_or_else(|| {
        runtime_error(format!(
            "Line anchor point must be a valid region point, got {point_name}"
        ))
    })?;

    let arg3 = stack_val(state, 3);
    let arg4 = stack_val(state, 4);
    let arg5 = stack_val(state, 5);

    let x = match arg3 {
        Val::Num(n) => Some(n as f32),
        _ => None,
    };
    let y = match arg4 {
        Val::Num(n) => Some(n as f32),
        _ => None,
    };
    if let (Some(x_offset), Some(y_offset)) = (x, y) {
        return Ok((point, None, x_offset, y_offset));
    }

    let target_id = resolve_anchor_target_id(state, arg3).map(|id| id as u64);
    let x_offset = match arg4 {
        Val::Num(n) => n as f32,
        _ => 0.0,
    };
    let y_offset = match arg5 {
        Val::Num(n) => n as f32,
        _ => 0.0,
    };
    Ok((point, target_id, x_offset, y_offset))
}

fn bind_named_child_global(state: &mut LuaState, name: &str, child_id: u64) -> LuaResult<()> {
    let child_ref = frame_ref(state, child_id)?;
    let key = state.gc.intern_string(name.as_bytes());
    if let Some(globals) = state.gc.tables.get_mut(state.global) {
        let _ = globals.raw_set(Val::Str(key), child_ref, &state.gc.string_arena);
    }
    Ok(())
}

fn frame_global_or_ref(state: &mut LuaState, id: u64) -> LuaResult<Val> {
    let frame_name = {
        let sim = borrow_state(state)?;
        sim.widgets.get(id).and_then(|frame| frame.name.clone())
    };
    if let Some(name) = frame_name {
        let key_ref = state.gc.intern_string(name.as_bytes());
        let global = state
            .gc
            .tables
            .get(state.global)
            .map(|table| table.get_str(key_ref, &state.gc.string_arena))
            .unwrap_or(Val::Nil);
        if global != Val::Nil {
            return Ok(global);
        }
    }
    frame_ref(state, id)
}

fn set_line_endpoint(state: &mut LuaState, is_start: bool) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let (point, target_id, x_offset, y_offset) = parse_line_anchor_args(state)?;

    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id)
        && frame.widget_type == crate::widget::WidgetType::Line
    {
        let anchor = crate::widget::LineAnchor {
            point,
            target_id,
            x_offset,
            y_offset,
        };
        if is_start {
            frame.line_start = Some(anchor);
        } else {
            frame.line_end = Some(anchor);
        }
        sim.widgets.mark_rect_dirty(id);
    }

    Ok(0)
}

fn get_line_endpoint(state: &mut LuaState, is_start: bool) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let anchor = {
        let sim = borrow_state(state)?;
        let Some(frame) = sim.widgets.get(id) else {
            return Ok(0);
        };
        if frame.widget_type != crate::widget::WidgetType::Line {
            return Ok(0);
        }
        if is_start {
            frame.line_start.clone()
        } else {
            frame.line_end.clone()
        }
    };

    let Some(anchor) = anchor else {
        return Ok(0);
    };

    let point_val = create_string(state, anchor.point.as_str());
    let target_val = match anchor.target_id {
        Some(target_id) => frame_global_or_ref(state, target_id)?,
        None => Val::Nil,
    };
    state.push(point_val);
    state.push(target_val);
    state.push(Val::Num(anchor.x_offset as f64));
    state.push(Val::Num(anchor.y_offset as f64));
    Ok(4)
}

fn set_start_point(state: &mut LuaState) -> LuaResult<u32> {
    set_line_endpoint(state, true)
}

fn get_start_point(state: &mut LuaState) -> LuaResult<u32> {
    get_line_endpoint(state, true)
}

fn set_end_point(state: &mut LuaState) -> LuaResult<u32> {
    set_line_endpoint(state, false)
}

fn get_end_point(state: &mut LuaState) -> LuaResult<u32> {
    get_line_endpoint(state, false)
}

fn button_enabled(frame: &crate::widget::Frame) -> bool {
    frame
        .attributes
        .get("__enabled")
        .and_then(|value| match value {
            crate::widget::AttributeValue::Boolean(value) => Some(*value),
            _ => None,
        })
        .unwrap_or(true)
}

fn sync_button_slot_visibility(sim: &mut crate::lua_api::SimState, button_id: u64) {
    for key in [
        "NormalTexture",
        "PushedTexture",
        "DisabledTexture",
        "HighlightTexture",
        "CheckedTexture",
        "DisabledCheckedTexture",
    ] {
        let child_id = sim
            .widgets
            .get(button_id)
            .and_then(|button| button.children_keys.get(key).copied());
        if let Some(child_id) = child_id {
            let should_show = button_texture_should_show(sim, button_id, key);
            sim.widgets.set_visible(child_id, should_show);
        }
    }
}

fn set_button_enabled_value(state: &mut LuaState, id: u64, enabled: bool) -> LuaResult<()> {
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.attributes.insert(
            "__enabled".to_string(),
            crate::widget::AttributeValue::Boolean(enabled),
        );
    }
    sync_button_slot_visibility(&mut sim, id);
    Ok(())
}

fn push_button_state_name(state: &mut LuaState, pushed: bool) -> LuaResult<u32> {
    let name = if pushed { "PUSHED" } else { "NORMAL" };
    let name_val = create_string(state, name);
    state.push(name_val);
    Ok(1)
}

fn set_item_button_scale(state: &mut LuaState) -> LuaResult<u32> {
    let self_table = Val::from_stack(state, 1)?;
    let scale = f64::from_stack(state, 2)?;
    let count = table_get(state, self_table, "Count");
    if let Some(count_id) = extract_frame_id(state, count) {
        let mut sim = borrow_state_mut(state)?;
        if let Some(frame) = sim.widgets.get_mut_visual(count_id) {
            frame.scale = scale as f32;
        }
    }
    Ok(0)
}

fn calculate_action(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let result = {
        let sim = borrow_state(state)?;
        let frame = sim.widgets.get(id);
        let button_id = frame.map(|widget| widget.user_id).unwrap_or(0);
        if button_id > 0 {
            button_id
        } else {
            frame
                .and_then(|widget| widget.attributes.get("action"))
                .and_then(|value| match value {
                    crate::widget::AttributeValue::Number(number) => Some(*number as i32),
                    _ => None,
                })
                .unwrap_or(1)
        }
    };
    state.push(Val::Num(result as f64));
    Ok(1)
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
    table_set(state, store, &format!("{id}:normal"), font_object);
    Ok(0)
}

/// GetNormalFontObject() -> fontObject
fn get_normal_font_object(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let store = get_or_create_button_font_store(state);
    let font_object = table_get(state, store, &format!("{id}:normal"));
    state.push(font_object);
    Ok(1)
}

/// SetHighlightFontObject(fontObject)
fn set_highlight_font_object(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let font_object = Val::from_stack(state, 2)?;
    let store = get_or_create_button_font_store(state);
    table_set(state, store, &format!("{id}:highlight"), font_object);
    Ok(0)
}

/// GetHighlightFontObject() -> fontObject
fn get_highlight_font_object(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let store = get_or_create_button_font_store(state);
    let font_object = table_get(state, store, &format!("{id}:highlight"));
    state.push(font_object);
    Ok(1)
}

/// SetDisabledFontObject(fontObject)
fn set_disabled_font_object(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let font_object = Val::from_stack(state, 2)?;
    let store = get_or_create_button_font_store(state);
    table_set(state, store, &format!("{id}:disabled"), font_object);
    Ok(0)
}

/// GetDisabledFontObject() -> fontObject
fn get_disabled_font_object(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let store = get_or_create_button_font_store(state);
    let font_object = table_get(state, store, &format!("{id}:disabled"));
    state.push(font_object);
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

/// GetCheckedTexture() -> texture
fn get_checked_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    push_button_texture_child(state, id, "CheckedTexture")
}

// ── Texture setter helpers ────────────────────────────────────────────────────

/// Determine visibility for a button texture child based on button state.
fn button_texture_should_show(
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
                .and_then(|value| match value {
                    crate::widget::AttributeValue::Boolean(value) => Some(*value),
                    _ => None,
                })
                .unwrap_or(true);
            let checked = frame
                .attributes
                .get("__checked")
                .and_then(|value| match value {
                    crate::widget::AttributeValue::Boolean(value) => Some(*value),
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
        "CheckedTexture" => enabled && checked,
        "DisabledCheckedTexture" => !enabled && checked,
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

    let tex_id =
        super::methods_helpers::get_or_create_button_texture(&mut sim, button_id, parent_key);
    if let Some(tex) = sim.widgets.get_mut_visual(tex_id) {
        tex.texture = resolved_path;
        tex.tex_coords = tex_coords;
        tex.atlas_tex_coords = tex_coords;
        tex.texture_file_data_id = file_data_id;
    }
    let should_show = button_texture_should_show(&sim, button_id, parent_key);
    sim.widgets.set_visible(tex_id, should_show);
    drop(sim);
    let _ = sync_child_to_rilua(state, button_id, parent_key, tex_id);
    Ok(())
}

fn is_enabled(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let enabled = {
        let sim = borrow_state(state)?;
        sim.widgets.get(id).map(button_enabled).unwrap_or(true)
    };
    state.push(Val::Bool(enabled));
    Ok(1)
}

fn set_enabled(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let enabled = bool::from_stack(state, 2).ok().unwrap_or(true);
    set_button_enabled_value(state, id, enabled)?;
    Ok(0)
}

fn enable(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    set_button_enabled_value(state, id, true)?;
    Ok(0)
}

fn disable(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    set_button_enabled_value(state, id, false)?;
    Ok(0)
}

fn register_for_clicks(state: &mut LuaState) -> LuaResult<u32> {
    let _id = frame_id_from_stack(state, 1)?;
    Ok(0)
}

fn set_button_state(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let state_name = String::from_stack(state, 2)?;
    let pushed = state_name.eq_ignore_ascii_case("PUSHED");
    {
        let mut sim = borrow_state_mut(state)?;
        if let Some(frame) = sim.widgets.get_mut_visual(id) {
            frame.button_state = if pushed { 1 } else { 0 };
        }
        sync_button_slot_visibility(&mut sim, id);
    }
    Ok(0)
}

fn get_button_state(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let pushed = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|frame| frame.button_state == 1)
            .unwrap_or(false)
    };
    push_button_state_name(state, pushed)
}

fn click(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let Some(handler) = get_rilua_script(state, id, "OnClick") else {
        return Ok(0);
    };
    if matches!(handler, Val::Nil) {
        return Ok(0);
    }
    let self_ref = frame_ref(state, id)?;
    let button = create_string(state, "LeftButton");
    let args = [self_ref, button, Val::Bool(false)];
    if let Err(error) = call_function_state(state, handler, &args) {
        call_error_handler_state(state, &error.to_string());
    }
    Ok(0)
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
    let tex_id = ensure_button_texture_child(state, button_id, parent_key)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(tid) = tex_id {
        let already_set = sim
            .widgets
            .get(tid)
            .map(|t| t.atlas.as_deref() == Some(atlas_name))
            .unwrap_or(false);
        if !already_set && let Some(tex) = sim.widgets.get_mut_visual(tid) {
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

/// Return the button's existing named texture child (`NormalTexture`,
/// `DisabledTexture`, etc.), creating it on demand.
///
/// Blizzard's `Set{Normal,Pushed,Disabled,Highlight}Atlas` is expected
/// to leave the button with a real child Texture so subsequent
/// `Get{Normal,...}Texture()` calls return it. Without this, code like
/// `SetDesaturation(self:GetDisabledTexture(), true)` in
/// LFDMicroButtonMixin:OnLoad errors on a nil texture.
fn ensure_button_texture_child(
    state: &mut LuaState,
    button_id: u64,
    parent_key: &str,
) -> LuaResult<Option<u64>> {
    use crate::widget::{Frame, WidgetType};
    {
        let sim = borrow_state(state)?;
        if let Some(existing) = sim
            .widgets
            .get(button_id)
            .and_then(|f| f.children_keys.get(parent_key).copied())
        {
            return Ok(Some(existing));
        }
    }
    let texture = Frame::new(WidgetType::Texture, None, Some(button_id));
    let child_id = texture.id;
    let mut sim = borrow_state_mut(state)?;
    sim.widgets.register(texture);
    sim.widgets.add_child(button_id, child_id);
    if let Some(parent) = sim.widgets.get_mut(button_id) {
        parent
            .children_keys
            .insert(parent_key.to_string(), child_id);
    }
    sim.invalidate_strata_buckets();
    Ok(Some(child_id))
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
    apply_texture_path_to_button(state, id, "CheckedTexture", texture, |f, path, _coords| {
        f.checked_texture = path;
    })?;
    Ok(0)
}

/// SetDisabledCheckedTexture(texture)
fn set_disabled_checked_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let texture = Val::from_stack(state, 2)?;
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
        sim.widgets.get(id).and_then(|frame| {
            frame.children_keys.get("Text").copied().or_else(|| {
                let fallback_name = frame.name.as_ref().map(|name| format!("{name}Text"))?;
                let child_id = sim.widgets.get_id_by_name(&fallback_name)?;
                let child = sim.widgets.get(child_id)?;
                (child.parent_id == Some(id)
                    && child.widget_type == crate::widget::WidgetType::FontString)
                    .then_some(child_id)
            })
        })
    };
    match text_id {
        Some(tid) => {
            let val = frame_ref(state, tid)?;
            state.push(val);
            Ok(1)
        }
        None => {
            let fallback = {
                let sim = borrow_state(state)?;
                sim.widgets.get(id).map(|frame| {
                    (
                        matches!(
                            frame.widget_type,
                            crate::widget::WidgetType::Button
                                | crate::widget::WidgetType::CheckButton
                        ),
                        frame.name.as_ref().map(|name| format!("{name}Text")),
                        frame.text.clone().unwrap_or_default(),
                    )
                })
            };
            let Some((is_button, child_name, text_value)) = fallback else {
                state.push(Val::Nil);
                return Ok(1);
            };
            if !is_button {
                state.push(Val::Nil);
                return Ok(1);
            }

            let child_id = {
                use crate::widget::{Frame, WidgetType};

                let mut font_string = Frame::new(WidgetType::FontString, child_name, Some(id));
                font_string.parent_key = Some("Text".to_string());
                font_string.text = Some(text_value.clone());
                font_string.text_stripped = Some(crate::render::strip_wow_markup(&text_value));
                super::methods_helpers::set_all_points_anchors_pub(&mut font_string, id);
                let child_id = font_string.id;

                let mut sim = borrow_state_mut(state)?;
                sim.widgets.register(font_string);
                sim.widgets.add_child(id, child_id);
                if let Some(button) = sim.widgets.get_mut_visual(id) {
                    button.children_keys.insert("Text".to_string(), child_id);
                }
                sim.invalidate_strata_buckets();
                child_id
            };

            let _ = sync_child_to_rilua(state, id, "Text", child_id);
            let val = frame_ref(state, child_id)?;
            state.push(val);
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
            let rel_val = frame_global_or_ref(state, rid as u64)?;
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
            let rel_val = frame_global_or_ref(state, rid as u64)?;
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
fn set_point(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let point_name = String::from_stack(state, 2)?;
    let Some(point) = crate::widget::AnchorPoint::from_str(&point_name) else {
        return Err(runtime_error(format!(
            "Frame:SetPoint(): Invalid region point {point_name}"
        )));
    };

    let (mut relative_to, relative_point, x_offset, y_offset) = parse_set_point_args(state, point)?;
    if relative_to.is_none() {
        relative_to = {
            let sim = borrow_state(state)?;
            sim.widgets
                .get(id)
                .and_then(|f| f.parent_id)
                .map(|pid| pid as usize)
        };
    }

    let mut sim = borrow_state_mut(state)?;
    if let Some(rel_id) = relative_to {
        if sim.widgets.would_create_anchor_cycle(id, rel_id as u64) {
            return Err(runtime_error(
                "Action[SetPoint] failed because[Cannot anchor to itself or create anchor cycle].",
            ));
        }
    }
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
        frame.set_point(point, relative_to, relative_point, x_offset, y_offset);
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
    let Some(mask_id) = extract_frame_id(state, Val::from_stack(state, 2)?) else {
        return Ok(0);
    };

    let mut sim = borrow_state_mut(state)?;
    let is_mask = sim.widgets.get(mask_id).map(|f| f.is_mask).unwrap_or(false);
    if !is_mask {
        return Ok(0);
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
    // Named FontStrings must be reachable as `_G[name]`, just like named
    // frames and textures. Blizzard XML (`ZoneText.xml`'s
    // `PVPArenaTextString`) and Lua code (`SubZoneText_OnLoad`) both
    // dereference the FontString by its global name. Without this bind,
    // the FontString exists in our widget registry but Lua sees `nil`.
    if let Some(ref n) = name {
        bind_named_child_global(state, n, child_id)?;
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

/// GetAnimations() -> animation1, animation2, ...
fn get_animations(state: &mut LuaState) -> LuaResult<u32> {
    let group_frame_id = frame_id_from_stack(state, 1)?;
    let mut animation_frame_ids: Vec<(usize, u64)> = {
        let sim = borrow_state(state)?;
        let Some(group_id) = sim.anim_frame_to_group.get(&group_frame_id).copied() else {
            return Ok(0);
        };
        sim.anim_frame_to_anim
            .iter()
            .filter_map(|(&frame_id, &(mapped_group_id, animation_index))| {
                (mapped_group_id == group_id).then_some((animation_index, frame_id))
            })
            .collect()
    };
    animation_frame_ids.sort_unstable_by_key(|(animation_index, _)| *animation_index);
    let count = animation_frame_ids.len() as u32;
    for (_, frame_id) in animation_frame_ids {
        let animation_ref = frame_ref(state, frame_id)?;
        state.push(animation_ref);
    }
    Ok(count)
}

fn resolve_anim_target_id(
    sim: &crate::lua_api::SimState,
    owner_id: u64,
    child_key: Option<&str>,
) -> Option<u64> {
    match child_key {
        Some(key) => sim.widgets.get(owner_id).and_then(|owner| {
            owner.children_keys.get(key).copied().or_else(|| {
                owner.children.iter().copied().find(|child_id| {
                    sim.widgets.get(*child_id).is_some_and(|child| {
                        child.parent_key.as_deref() == Some(key)
                            || child.name.as_deref() == Some(key)
                    })
                })
            })
        }),
        None => Some(owner_id),
    }
}

fn get_animation_target(state: &mut LuaState) -> LuaResult<u32> {
    let animation_frame_id = frame_id_from_stack(state, 1)?;
    let target_id = {
        let sim = borrow_state(state)?;
        let Some((group_id, animation_index)) = sim.anim_frame_to_anim.get(&animation_frame_id)
        else {
            return Ok(0);
        };
        let Some(group) = sim.animation_groups.get(group_id) else {
            return Ok(0);
        };
        let child_key = group
            .animations
            .get(*animation_index)
            .and_then(|animation| animation.child_key.as_deref());
        resolve_anim_target_id(&sim, group.owner_frame_id, child_key)
    };
    let Some(target_id) = target_id else {
        return Ok(0);
    };
    let target_ref = frame_ref(state, target_id)?;
    state.push(target_ref);
    Ok(1)
}

fn set_animation_child_key(state: &mut LuaState) -> LuaResult<u32> {
    let animation_frame_id = frame_id_from_stack(state, 1)?;
    let child_key = String::from_stack(state, 2)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some((group_id, animation_index)) =
        sim.anim_frame_to_anim.get(&animation_frame_id).copied()
        && let Some(group) = sim.animation_groups.get_mut(&group_id)
        && let Some(animation) = group.animations.get_mut(animation_index)
    {
        animation.child_key = Some(child_key);
    }
    Ok(0)
}

fn get_region_parent(state: &mut LuaState) -> LuaResult<u32> {
    let animation_frame_id = frame_id_from_stack(state, 1)?;
    let owner_id = {
        let sim = borrow_state(state)?;
        let Some((group_id, _)) = sim.anim_frame_to_anim.get(&animation_frame_id) else {
            return Ok(0);
        };
        sim.animation_groups
            .get(group_id)
            .map(|group| group.owner_frame_id)
    };
    let Some(owner_id) = owner_id else {
        return Ok(0);
    };
    let owner_ref = frame_ref(state, owner_id)?;
    state.push(owner_ref);
    Ok(1)
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
    if let Some(ref n) = name {
        bind_named_child_global(state, n, child_id)?;
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
    if let Some(group_id) = resolve_animation_group_id(&sim, group_frame_id)
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
    if let Some(group_id) = resolve_animation_group_id(&sim, group_frame_id)
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
        resolve_animation_group_id(&sim, group_frame_id)
            .and_then(|group_id| {
                sim.animation_groups
                    .get(&group_id)
                    .map(|group| group.playing)
            })
            .unwrap_or(false)
    };
    state.push(Val::Bool(playing));
    Ok(1)
}

fn resolve_animation_group_id(sim: &crate::lua_api::SimState, frame_id: u64) -> Option<u64> {
    sim.anim_frame_to_group.get(&frame_id).copied().or_else(|| {
        sim.anim_frame_to_anim
            .get(&frame_id)
            .map(|(group_id, _)| *group_id)
    })
}

fn animation_group_restart(state: &mut LuaState) -> LuaResult<u32> {
    let frame_id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(group_id) = resolve_animation_group_id(&sim, frame_id)
        && let Some(group) = sim.animation_groups.get_mut(&group_id)
    {
        group.playing = true;
        group.paused = false;
        group.done = false;
        group.pending_finish = false;
        group.elapsed = 0.0;
        for animation in &mut group.animations {
            animation.elapsed = 0.0;
        }
    }
    Ok(0)
}

fn animation_group_finish(state: &mut LuaState) -> LuaResult<u32> {
    let frame_id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(group_id) = resolve_animation_group_id(&sim, frame_id)
        && let Some(group) = sim.animation_groups.get_mut(&group_id)
    {
        group.playing = false;
        group.paused = false;
        group.done = true;
        group.pending_finish = false;
        for animation in &mut group.animations {
            animation.elapsed = animation.duration;
        }
    }
    Ok(0)
}

fn animation_group_is_done(state: &mut LuaState) -> LuaResult<u32> {
    let frame_id = frame_id_from_stack(state, 1)?;
    let done = {
        let sim = borrow_state(state)?;
        resolve_animation_group_id(&sim, frame_id)
            .and_then(|group_id| sim.animation_groups.get(&group_id).map(|group| group.done))
            .unwrap_or(false)
    };
    state.push(Val::Bool(done));
    Ok(1)
}

fn animation_config_noop(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn animation_numeric_arg(state: &LuaState, index: i32) -> f64 {
    match stack_val(state, index) {
        Val::Num(value) => value.max(0.0),
        _ => 0.0,
    }
}

fn with_animation_state_mut<F>(state: &mut LuaState, f: F) -> LuaResult<()>
where
    F: FnOnce(&mut crate::lua_api::animation::AnimState),
{
    let animation_frame_id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some((group_id, animation_index)) =
        sim.anim_frame_to_anim.get(&animation_frame_id).copied()
        && let Some(group) = sim.animation_groups.get_mut(&group_id)
        && let Some(animation) = group.animations.get_mut(animation_index)
    {
        f(animation);
    }
    Ok(())
}

fn animation_set_flipbook_rows(state: &mut LuaState) -> LuaResult<u32> {
    let rows = animation_numeric_arg(state, 2) as u32;
    with_animation_state_mut(state, |animation| animation.flipbook_rows = rows)?;
    Ok(0)
}

fn animation_get_flipbook_rows(state: &mut LuaState) -> LuaResult<u32> {
    let animation_frame_id = frame_id_from_stack(state, 1)?;
    let rows = {
        let sim = borrow_state(state)?;
        sim.anim_frame_to_anim
            .get(&animation_frame_id)
            .and_then(|(group_id, animation_index)| {
                sim.animation_groups
                    .get(group_id)
                    .and_then(|group| group.animations.get(*animation_index))
                    .map(|animation| animation.flipbook_rows)
            })
            .unwrap_or(0)
    };
    state.push(Val::Num(rows as f64));
    Ok(1)
}

fn animation_set_flipbook_columns(state: &mut LuaState) -> LuaResult<u32> {
    let columns = animation_numeric_arg(state, 2) as u32;
    with_animation_state_mut(state, |animation| animation.flipbook_columns = columns)?;
    Ok(0)
}

fn animation_get_flipbook_columns(state: &mut LuaState) -> LuaResult<u32> {
    let animation_frame_id = frame_id_from_stack(state, 1)?;
    let columns = {
        let sim = borrow_state(state)?;
        sim.anim_frame_to_anim
            .get(&animation_frame_id)
            .and_then(|(group_id, animation_index)| {
                sim.animation_groups
                    .get(group_id)
                    .and_then(|group| group.animations.get(*animation_index))
                    .map(|animation| animation.flipbook_columns)
            })
            .unwrap_or(0)
    };
    state.push(Val::Num(columns as f64));
    Ok(1)
}

fn animation_set_flipbook_frames(state: &mut LuaState) -> LuaResult<u32> {
    let frames = animation_numeric_arg(state, 2) as u32;
    with_animation_state_mut(state, |animation| animation.flipbook_frames = frames)?;
    Ok(0)
}

fn animation_get_flipbook_frames(state: &mut LuaState) -> LuaResult<u32> {
    let animation_frame_id = frame_id_from_stack(state, 1)?;
    let frames = {
        let sim = borrow_state(state)?;
        sim.anim_frame_to_anim
            .get(&animation_frame_id)
            .and_then(|(group_id, animation_index)| {
                sim.animation_groups
                    .get(group_id)
                    .and_then(|group| group.animations.get(*animation_index))
                    .map(|animation| animation.flipbook_frames)
            })
            .unwrap_or(0)
    };
    state.push(Val::Num(frames as f64));
    Ok(1)
}

fn animation_set_flipbook_frame_width(state: &mut LuaState) -> LuaResult<u32> {
    let width = animation_numeric_arg(state, 2);
    with_animation_state_mut(state, |animation| animation.flipbook_frame_width = width)?;
    Ok(0)
}

fn animation_get_flipbook_frame_width(state: &mut LuaState) -> LuaResult<u32> {
    let animation_frame_id = frame_id_from_stack(state, 1)?;
    let width = {
        let sim = borrow_state(state)?;
        sim.anim_frame_to_anim
            .get(&animation_frame_id)
            .and_then(|(group_id, animation_index)| {
                sim.animation_groups
                    .get(group_id)
                    .and_then(|group| group.animations.get(*animation_index))
                    .map(|animation| animation.flipbook_frame_width)
            })
            .unwrap_or(0.0)
    };
    state.push(Val::Num(width));
    Ok(1)
}

fn animation_set_flipbook_frame_height(state: &mut LuaState) -> LuaResult<u32> {
    let height = animation_numeric_arg(state, 2);
    with_animation_state_mut(state, |animation| animation.flipbook_frame_height = height)?;
    Ok(0)
}

fn animation_get_flipbook_frame_height(state: &mut LuaState) -> LuaResult<u32> {
    let animation_frame_id = frame_id_from_stack(state, 1)?;
    let height = {
        let sim = borrow_state(state)?;
        sim.anim_frame_to_anim
            .get(&animation_frame_id)
            .and_then(|(group_id, animation_index)| {
                sim.animation_groups
                    .get(group_id)
                    .and_then(|group| group.animations.get(*animation_index))
                    .map(|animation| animation.flipbook_frame_height)
            })
            .unwrap_or(0.0)
    };
    state.push(Val::Num(height));
    Ok(1)
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
    table_set_rust_fn(state, table, "GetCheckedTexture", get_checked_texture)?;

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
    table_set_rust_fn(state, table, "IsEnabled", is_enabled)?;
    table_set_rust_fn(state, table, "SetEnabled", set_enabled)?;
    table_set_rust_fn(state, table, "Enable", enable)?;
    table_set_rust_fn(state, table, "Disable", disable)?;
    table_set_rust_fn(state, table, "RegisterForClicks", register_for_clicks)?;
    table_set_rust_fn(state, table, "SetButtonState", set_button_state)?;
    table_set_rust_fn(state, table, "GetButtonState", get_button_state)?;
    table_set_rust_fn(state, table, "Click", click)?;
    table_set_rust_fn(state, table, "SetItemButtonScale", set_item_button_scale)?;
    table_set_rust_fn(state, table, "CalculateAction", calculate_action)?;

    // Anchor methods
    table_set_rust_fn(state, table, "SetPoint", set_point)?;
    table_set_rust_fn(state, table, "SetStartPoint", set_start_point)?;
    table_set_rust_fn(state, table, "SetEndPoint", set_end_point)?;
    table_set_rust_fn(state, table, "ClearAllPoints", clear_all_points)?;
    table_set_rust_fn(state, table, "ClearPoint", clear_point)?;
    table_set_rust_fn(state, table, "ClearPointsOffset", clear_points_offset)?;
    table_set_rust_fn(state, table, "AdjustPointsOffset", adjust_points_offset)?;
    table_set_rust_fn(state, table, "SetAllPoints", set_all_points)?;
    table_set_rust_fn(state, table, "GetPoint", get_point)?;
    table_set_rust_fn(state, table, "GetStartPoint", get_start_point)?;
    table_set_rust_fn(state, table, "GetEndPoint", get_end_point)?;
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
    table_set_rust_fn(state, table, "GetAnimations", get_animations)?;
    table_set_rust_fn(state, table, "CreateAnimationGroup", create_animation_group)?;
    table_set_rust_fn(state, table, "CreateAnimation", create_animation)?;
    table_set_rust_fn(state, table, "Play", animation_group_play)?;
    table_set_rust_fn(state, table, "Restart", animation_group_restart)?;
    table_set_rust_fn(state, table, "Stop", animation_group_stop)?;
    table_set_rust_fn(state, table, "Finish", animation_group_finish)?;
    table_set_rust_fn(state, table, "IsPlaying", animation_group_is_playing)?;
    table_set_rust_fn(state, table, "IsDone", animation_group_is_done)?;
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
    table_set_rust_fn(state, table, "GetTarget", get_animation_target)?;
    table_set_rust_fn(state, table, "GetRegionParent", get_region_parent)?;
    table_set_rust_fn(state, table, "SetChildKey", set_animation_child_key)?;
    table_set_rust_fn(state, table, "SetTargetName", animation_config_noop)?;
    table_set_rust_fn(state, table, "SetTargetKey", animation_config_noop)?;
    table_set_rust_fn(state, table, "SetFlipBookRows", animation_set_flipbook_rows)?;
    table_set_rust_fn(state, table, "GetFlipBookRows", animation_get_flipbook_rows)?;
    table_set_rust_fn(
        state,
        table,
        "SetFlipBookColumns",
        animation_set_flipbook_columns,
    )?;
    table_set_rust_fn(
        state,
        table,
        "GetFlipBookColumns",
        animation_get_flipbook_columns,
    )?;
    table_set_rust_fn(
        state,
        table,
        "SetFlipBookFrames",
        animation_set_flipbook_frames,
    )?;
    table_set_rust_fn(
        state,
        table,
        "GetFlipBookFrames",
        animation_get_flipbook_frames,
    )?;
    table_set_rust_fn(
        state,
        table,
        "SetFlipBookFrameWidth",
        animation_set_flipbook_frame_width,
    )?;
    table_set_rust_fn(
        state,
        table,
        "GetFlipBookFrameWidth",
        animation_get_flipbook_frame_width,
    )?;
    table_set_rust_fn(
        state,
        table,
        "SetFlipBookFrameHeight",
        animation_set_flipbook_frame_height,
    )?;
    table_set_rust_fn(
        state,
        table,
        "GetFlipBookFrameHeight",
        animation_get_flipbook_frame_height,
    )?;
    table_set_rust_fn(state, table, "CreateControlPoint", create_control_point)?;

    Ok(())
}
