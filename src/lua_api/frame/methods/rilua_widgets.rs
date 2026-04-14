//! rilua RustFn equivalents of widget-specific frame methods.
//!
//! Covers: Cooldown, EditBox, Slider/CheckButton/StatusBar shared value,
//! StatusBar, Model/ModelScene, and GameTooltip widget methods.
//!
//! Each method is a plain `fn(&mut LuaState) -> LuaResult<u32>` (i.e. a
//! `RustFn`) that uses the helpers in `rilua_methods` to extract the frame
//! ID from the first stack argument and borrow `SimState`.
//!
//! Complex operations that depend on mlua-specific sub-systems (mixin
//! overrides, sync_child_to_lua, fire_tooltip_script, etc.) are stubbed
//! with a `TODO` comment.

use crate::lua_api::rilua_methods::{
    borrow_state, borrow_state_mut, create_string, frame_id_from_stack, frame_ref, val_to_string,
};
use crate::lua_bridge::{stack_val, table_set_rust_fn, IntoStack};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{runtime_error, LuaResult, Val};

// ---------------------------------------------------------------------------
// Helpers shared across widget methods
// ---------------------------------------------------------------------------

fn val_to_f64(val: Val) -> f64 {
    match val {
        Val::Num(n) => n,
        _ => 0.0,
    }
}

fn val_to_bool(val: Val) -> bool {
    matches!(val, Val::Bool(true))
}

fn opt_string(state: &LuaState, index: i32) -> Option<String> {
    val_to_string(state, stack_val(state, index))
}

/// Parse (start, duration, mod_rate) from stack positions 2, 3, 4.
fn parse_f64_triple(state: &LuaState) -> (f64, f64, f64) {
    let start = val_to_f64(stack_val(state, 2));
    let duration = val_to_f64(stack_val(state, 3));
    let mod_rate = val_to_f64(stack_val(state, 4));
    (start, duration, mod_rate)
}

fn normalize_mod_rate(r: f64) -> f64 {
    if r <= 0.0 { 1.0 } else { r }
}

fn apply_cooldown_state(frame: &mut crate::widget::Frame, start: f64, duration: f64, mod_rate: f64) {
    frame.cooldown_start = start;
    frame.cooldown_duration = duration;
    frame.cooldown_display_duration_ms = duration.max(0.0) * 1000.0;
    frame.cooldown_mod_rate = normalize_mod_rate(mod_rate);
}

fn clear_cooldown_timing(frame: &mut crate::widget::Frame) {
    frame.cooldown_start = 0.0;
    frame.cooldown_duration = 0.0;
    frame.cooldown_display_duration_ms = 0.0;
    frame.cooldown_mod_rate = 1.0;
}

// ---------------------------------------------------------------------------
// Cooldown methods
// ---------------------------------------------------------------------------

fn cooldown_set_cooldown(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let start = val_to_f64(stack_val(state, 2));
    let duration = val_to_f64(stack_val(state, 3));
    let mod_rate = normalize_mod_rate(val_to_f64(stack_val(state, 4)));
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        apply_cooldown_state(f, start, duration, mod_rate);
    }
    Ok(0)
}

fn cooldown_set_cooldown_unix(state: &mut LuaState) -> LuaResult<u32> {
    // Same semantics as SetCooldown — forward to the same impl.
    cooldown_set_cooldown(state)
}

fn cooldown_set_cooldown_from_expiration_time(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let expiration = val_to_f64(stack_val(state, 2));
    let duration = val_to_f64(stack_val(state, 3));
    let mod_rate = normalize_mod_rate(val_to_f64(stack_val(state, 4)));
    let start = expiration - duration;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        apply_cooldown_state(f, start, duration, mod_rate);
    }
    Ok(0)
}

fn cooldown_set_cooldown_duration(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let duration = val_to_f64(stack_val(state, 2));
    let mod_rate = normalize_mod_rate(val_to_f64(stack_val(state, 3)));
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        let start = f.cooldown_start;
        apply_cooldown_state(f, start, duration, mod_rate);
    }
    Ok(0)
}

fn cooldown_get_cooldown_times(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let (s, d) = sim
        .widgets
        .get(id)
        .map(|f| (f.cooldown_start, f.cooldown_duration))
        .unwrap_or((0.0, 0.0));
    drop(sim);
    (s, d).into_stack(state)
}

fn cooldown_get_cooldown_duration(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .map(|f| f.cooldown_duration)
        .unwrap_or(0.0);
    drop(sim);
    v.into_stack(state)
}

fn cooldown_get_cooldown_display_duration(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .map(|f| f.cooldown_display_duration_ms)
        .unwrap_or(0.0);
    drop(sim);
    v.into_stack(state)
}

fn cooldown_clear(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        clear_cooldown_timing(f);
    }
    Ok(0)
}

fn cooldown_pause(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.cooldown_paused = true;
    }
    Ok(0)
}

fn cooldown_resume(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.cooldown_paused = false;
    }
    Ok(0)
}

fn cooldown_is_paused(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .map(|f| f.cooldown_paused)
        .unwrap_or(false);
    drop(sim);
    v.into_stack(state)
}

fn cooldown_set_draw_swipe(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let draw = val_to_bool(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.cooldown_draw_swipe = draw;
    }
    Ok(0)
}

fn cooldown_get_draw_swipe(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .map(|f| f.cooldown_draw_swipe)
        .unwrap_or(true);
    drop(sim);
    v.into_stack(state)
}

fn cooldown_set_draw_edge(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let draw = val_to_bool(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.cooldown_draw_edge = draw;
    }
    Ok(0)
}

fn cooldown_get_draw_edge(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .map(|f| f.cooldown_draw_edge)
        .unwrap_or(false);
    drop(sim);
    v.into_stack(state)
}

fn cooldown_set_draw_bling(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let draw = val_to_bool(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.cooldown_draw_bling = draw;
    }
    Ok(0)
}

fn cooldown_get_draw_bling(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .map(|f| f.cooldown_draw_bling)
        .unwrap_or(true);
    drop(sim);
    v.into_stack(state)
}

fn cooldown_set_reverse(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let rev = val_to_bool(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.cooldown_reverse = rev;
    }
    Ok(0)
}

fn cooldown_get_reverse(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .map(|f| f.cooldown_reverse)
        .unwrap_or(false);
    drop(sim);
    v.into_stack(state)
}

fn cooldown_set_hide_countdown_numbers(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let hide = val_to_bool(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.cooldown_hide_countdown = hide;
    }
    Ok(0)
}

fn cooldown_get_hide_countdown_numbers(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .map(|f| f.cooldown_hide_countdown)
        .unwrap_or(false);
    drop(sim);
    v.into_stack(state)
}

fn cooldown_set_edge_scale(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let scale = val_to_f64(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.cooldown_edge_scale = scale;
    }
    Ok(0)
}

fn cooldown_get_edge_scale(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .map(|f| f.cooldown_edge_scale)
        .unwrap_or(1.0);
    drop(sim);
    v.into_stack(state)
}

fn cooldown_set_minimum_countdown_duration(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let ms = val_to_f64(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.cooldown_min_countdown_duration_ms = ms;
    }
    Ok(0)
}

fn cooldown_get_minimum_countdown_duration(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .map(|f| f.cooldown_min_countdown_duration_ms)
        .unwrap_or(0.0);
    drop(sim);
    v.into_stack(state)
}

fn cooldown_set_use_aura_display_time(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let enabled = val_to_bool(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.cooldown_use_aura_display_time = enabled;
    }
    Ok(0)
}

fn cooldown_get_use_aura_display_time(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .map(|f| f.cooldown_use_aura_display_time)
        .unwrap_or(false);
    drop(sim);
    v.into_stack(state)
}

fn cooldown_set_use_circular_edge(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let enabled = val_to_bool(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.cooldown_use_circular_edge = enabled;
    }
    Ok(0)
}

fn cooldown_set_countdown_abbrev_threshold(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let threshold = val_to_f64(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.cooldown_countdown_abbrev_threshold_seconds = threshold;
    }
    Ok(0)
}

fn cooldown_set_swipe_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let path = opt_string(state, 2);
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.cooldown_swipe_texture = path;
    }
    Ok(0)
}

fn cooldown_set_bling_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let path = opt_string(state, 2);
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.cooldown_bling_texture = path;
    }
    Ok(0)
}

fn cooldown_set_countdown_font(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let font_name = opt_string(state, 2);
    if let Some(font_name) = font_name {
        let mut sim = borrow_state_mut(state)?;
        // Find existing countdown font string child if any.
        let child_id = sim
            .widgets
            .get(id)
            .and_then(|f| f.cooldown_countdown_font_string_id);
        if let Some(child_id) = child_id {
            if let Some(child) = sim.widgets.get_mut_visual(child_id) {
                child.font = Some(font_name);
            }
        }
        // TODO: create countdown FontString child if missing (requires sync_child_to_rilua)
    }
    Ok(0)
}

fn cooldown_get_countdown_font_string(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let child_id = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .and_then(|f| f.cooldown_countdown_font_string_id)
    };
    // TODO: create the countdown child if missing (needs sync_child_to_rilua)
    match child_id {
        Some(cid) => {
            let val = frame_ref(state, cid)?;
            val.into_stack(state)
        }
        None => {
            state.push(Val::Nil);
            Ok(1)
        }
    }
}

fn cooldown_set_from_duration_object(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: parse duration object (Table/UserData with GetStartTime/GetTotalDuration/GetModRate/IsZero)
    // For now this is a no-op stub.
    Ok(0)
}

fn cooldown_set_edge_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let path = opt_string(state, 2);
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.cooldown_edge_texture = path;
    }
    Ok(0)
}

fn cooldown_set_swipe_color(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let r = val_to_f64(stack_val(state, 2)) as f32;
    let g = val_to_f64(stack_val(state, 3)) as f32;
    let b = val_to_f64(stack_val(state, 4)) as f32;
    let a = val_to_f64(stack_val(state, 5)) as f32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.attributes.insert(
            "__swipe_color".to_string(),
            crate::widget::AttributeValue::String(format!("{},{},{},{}", r, g, b, a)),
        );
    }
    Ok(0)
}

fn cooldown_set_edge_color(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let r = val_to_f64(stack_val(state, 2)) as f32;
    let g = val_to_f64(stack_val(state, 3)) as f32;
    let b = val_to_f64(stack_val(state, 4)) as f32;
    let a = val_to_f64(stack_val(state, 5)) as f32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.cooldown_edge_color = crate::widget::Color::new(r, g, b, a);
    }
    Ok(0)
}

fn cooldown_set_tex_coord_range(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: parse two Vector2 args
    Ok(0)
}

// ---------------------------------------------------------------------------
// EditBox methods
// ---------------------------------------------------------------------------

fn editbox_set_focus(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let old_focus = {
        let mut sim = borrow_state_mut(state)?;
        let old = sim.focused_frame_id;
        sim.focused_frame_id = Some(id);
        old
    };
    if old_focus != Some(id) {
        // TODO: fire OnEditFocusLost on old_focus, OnEditFocusGained on id
        // (requires rilua-side script dispatch)
    }
    Ok(0)
}

fn editbox_clear_focus(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let cleared = {
        let mut sim = borrow_state_mut(state)?;
        if sim.focused_frame_id == Some(id) {
            sim.focused_frame_id = None;
            true
        } else {
            false
        }
    };
    if cleared {
        // TODO: fire OnEditFocusLost
    }
    Ok(0)
}

fn editbox_has_focus(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim.focused_frame_id == Some(id);
    drop(sim);
    v.into_stack(state)
}

fn editbox_has_text(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .and_then(|f| f.text.as_ref())
        .is_some_and(|t| !t.is_empty());
    drop(sim);
    v.into_stack(state)
}

fn editbox_set_cursor_position(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let pos = val_to_f64(stack_val(state, 2)) as i32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        let len = f.text.as_deref().unwrap_or("").chars().count() as i32;
        f.editbox_cursor_pos = pos.clamp(0, len);
    }
    Ok(0)
}

fn editbox_get_cursor_position(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim.widgets.get(id).map(|f| f.editbox_cursor_pos).unwrap_or(0);
    drop(sim);
    (v as f64).into_stack(state)
}

fn editbox_get_num_letters(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .and_then(|f| f.text.as_ref())
        .map(|t| t.chars().count())
        .unwrap_or(0);
    drop(sim);
    (v as f64).into_stack(state)
}

fn editbox_set_max_letters(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let max = val_to_f64(stack_val(state, 2)) as i32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.editbox_max_letters = max;
    }
    Ok(0)
}

fn editbox_get_max_letters(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim.widgets.get(id).map(|f| f.editbox_max_letters).unwrap_or(0);
    drop(sim);
    (v as f64).into_stack(state)
}

fn editbox_set_multi_line(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = val_to_bool(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.editbox_multi_line = value;
    }
    Ok(0)
}

fn editbox_is_multi_line(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim.widgets.get(id).map(|f| f.editbox_multi_line).unwrap_or(false);
    drop(sim);
    v.into_stack(state)
}

fn editbox_set_auto_focus(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = val_to_bool(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.editbox_auto_focus = value;
    }
    Ok(0)
}

fn editbox_is_auto_focus(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim.widgets.get(id).map(|f| f.editbox_auto_focus).unwrap_or(false);
    drop(sim);
    v.into_stack(state)
}

fn editbox_set_numeric(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = val_to_bool(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.editbox_numeric = value;
    }
    Ok(0)
}

fn editbox_is_numeric(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim.widgets.get(id).map(|f| f.editbox_numeric).unwrap_or(false);
    drop(sim);
    v.into_stack(state)
}

fn editbox_set_password(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = val_to_bool(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.editbox_password = value;
    }
    Ok(0)
}

fn editbox_is_password(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim.widgets.get(id).map(|f| f.editbox_password).unwrap_or(false);
    drop(sim);
    v.into_stack(state)
}

fn editbox_set_number(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let n = val_to_f64(stack_val(state, 2));
    let s = n.to_string();
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.text_stripped = Some(s.clone());
        f.text = Some(s);
    }
    Ok(0)
}

fn editbox_get_number(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .and_then(|f| f.text.as_ref())
        .and_then(|t| t.parse::<f64>().ok())
        .unwrap_or(0.0);
    drop(sim);
    v.into_stack(state)
}

fn editbox_add_history_line(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let text = opt_string(state, 2).unwrap_or_default();
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.editbox_history.push(text);
        let max = f.editbox_history_max;
        if max > 0 && f.editbox_history.len() > max as usize {
            f.editbox_history.remove(0);
        }
    }
    Ok(0)
}

fn editbox_get_history_lines(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .map(|f| f.editbox_history.len())
        .unwrap_or(0);
    drop(sim);
    (v as f64).into_stack(state)
}

fn editbox_set_history_lines(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let max = val_to_f64(stack_val(state, 2)) as i32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.editbox_history_max = max;
    }
    Ok(0)
}

fn editbox_clear_history(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.editbox_history.clear();
    }
    Ok(0)
}

fn editbox_get_input_language(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let lang = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|f| f.editbox_input_language.clone())
            .unwrap_or_else(|| "ROMAN".to_string())
    };
    let val = create_string(state, &lang);
    val.into_stack(state)
}

fn editbox_toggle_input_language(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.editbox_input_language = if f.editbox_input_language == "ROMAN" {
            "NATIVE".to_string()
        } else {
            "ROMAN".to_string()
        };
    }
    Ok(0)
}

fn editbox_reset_input_mode(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.editbox_input_language = "ROMAN".to_string();
    }
    Ok(0)
}

fn editbox_set_text_insets(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let l = val_to_f64(stack_val(state, 2)) as f32;
    let r = val_to_f64(stack_val(state, 3)) as f32;
    let t = val_to_f64(stack_val(state, 4)) as f32;
    let b = val_to_f64(stack_val(state, 5)) as f32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.editbox_text_insets = (l, r, t, b);
    }
    Ok(0)
}

fn editbox_get_text_insets(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let (l, r, t, b) = sim
        .widgets
        .get(id)
        .map(|f| f.editbox_text_insets)
        .unwrap_or((0.0, 0.0, 0.0, 0.0));
    drop(sim);
    (l as f64, r as f64, t as f64, b as f64).into_stack(state)
}

fn editbox_get_display_text(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let text = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .and_then(|f| f.text.clone())
            .unwrap_or_default()
    };
    let val = create_string(state, &text);
    val.into_stack(state)
}

fn editbox_insert(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let text = opt_string(state, 2).unwrap_or_default();
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        let pos = f.editbox_cursor_pos.max(0) as usize;
        let current = f.text.get_or_insert_with(String::new);
        let insert_at = pos.min(current.len());
        current.insert_str(insert_at, &text);
        f.editbox_cursor_pos = (insert_at + text.len()) as i32;
    }
    Ok(0)
}

fn editbox_set_blink_speed(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let speed = val_to_f64(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.editbox_blink_speed = speed;
    }
    Ok(0)
}

fn editbox_get_blink_speed(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim.widgets.get(id).map(|f| f.editbox_blink_speed).unwrap_or(0.5);
    drop(sim);
    v.into_stack(state)
}

fn editbox_set_alt_arrow_key_mode(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = val_to_bool(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.editbox_alt_arrow_key_mode = value;
    }
    Ok(0)
}

fn editbox_get_alt_arrow_key_mode(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .map(|f| f.editbox_alt_arrow_key_mode)
        .unwrap_or(false);
    drop(sim);
    v.into_stack(state)
}

fn editbox_highlight_text(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let start = val_to_f64(stack_val(state, 2)) as i32;
    let end_raw = stack_val(state, 3);
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        let len = f.text.as_deref().unwrap_or("").chars().count() as i32;
        let end = match end_raw {
            Val::Num(n) => n as i32,
            _ => len,
        };
        let (s, e) = normalize_highlight_range(start, end, len);
        f.editbox_highlight_range = Some((s, e));
    }
    Ok(0)
}

fn editbox_clear_highlight_text(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.editbox_highlight_range = None;
    }
    Ok(0)
}

fn normalize_highlight_range(start: i32, end: i32, len: i32) -> (i32, i32) {
    let s = start.clamp(0, len);
    let e = end.clamp(0, len);
    if s <= e { (s, e) } else { (e, s) }
}

// ---------------------------------------------------------------------------
// Slider methods
// ---------------------------------------------------------------------------

fn slider_set_value_step(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let step = val_to_f64(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.slider_step = step;
    }
    Ok(0)
}

fn slider_get_value_step(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim.widgets.get(id).map(|f| f.slider_step).unwrap_or(1.0);
    drop(sim);
    v.into_stack(state)
}

fn slider_set_orientation(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let orientation = opt_string(state, 2)
        .map(|s| s.to_uppercase())
        .unwrap_or_else(|| "HORIZONTAL".to_string());
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.slider_orientation = orientation;
    }
    Ok(0)
}

fn slider_get_orientation(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let orientation = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|f| f.slider_orientation.clone())
            .unwrap_or_else(|| "HORIZONTAL".to_string())
    };
    let val = create_string(state, &orientation);
    val.into_stack(state)
}

fn slider_set_obey_step_on_drag(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let obey = val_to_bool(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.slider_obey_step_on_drag = obey;
    }
    Ok(0)
}

fn slider_get_obey_step_on_drag(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .map(|f| f.slider_obey_step_on_drag)
        .unwrap_or(false);
    drop(sim);
    v.into_stack(state)
}

fn slider_set_steps_per_page(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let steps = val_to_f64(stack_val(state, 2)) as i32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.slider_steps_per_page = steps;
    }
    Ok(0)
}

fn slider_get_steps_per_page(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim.widgets.get(id).map(|f| f.slider_steps_per_page).unwrap_or(1);
    drop(sim);
    (v as f64).into_stack(state)
}

fn slider_is_dragging_thumb(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim.active_slider_thumb_drag_frame == Some(id);
    drop(sim);
    v.into_stack(state)
}

// ---------------------------------------------------------------------------
// Shared value methods (Slider + StatusBar)
// ---------------------------------------------------------------------------

fn shared_set_value(state: &mut LuaState) -> LuaResult<u32> {
    use crate::widget::WidgetType;
    let id = frame_id_from_stack(state, 1)?;
    let value = val_to_f64(stack_val(state, 2));
    let wtype = {
        let sim = borrow_state(state)?;
        sim.widgets.get(id).map(|f| f.widget_type)
    };
    match wtype {
        Some(WidgetType::Slider) => {
            let clamped = {
                let mut sim = borrow_state_mut(state)?;
                let Some(f) = sim.widgets.get(id) else {
                    return Ok(0);
                };
                let clamped = value.clamp(f.slider_min, f.slider_max);
                if clamped == f.slider_value {
                    return Ok(0);
                }
                sim.widgets.get_mut_visual(id).unwrap().slider_value = clamped;
                clamped
            };
            // TODO: fire OnValueChanged (needs rilua script dispatch)
            let _ = clamped;
        }
        Some(WidgetType::StatusBar) => {
            let mut sim = borrow_state_mut(state)?;
            if let Some(f) = sim.widgets.get(id) {
                let clamped = value.clamp(f.statusbar_min, f.statusbar_max);
                if clamped != f.statusbar_value {
                    if let Some(f) = sim.widgets.get_mut_visual(id) {
                        f.statusbar_value = clamped;
                        f.statusbar_interpolated_value = clamped;
                        f.statusbar_interpolation_target = None;
                    }
                }
            }
        }
        _ => {}
    }
    Ok(0)
}

fn shared_get_value(state: &mut LuaState) -> LuaResult<u32> {
    use crate::widget::WidgetType;
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .map(|f| match f.widget_type {
            WidgetType::Slider => f.slider_value,
            WidgetType::StatusBar => f.statusbar_value,
            _ => 0.0,
        })
        .unwrap_or(0.0);
    drop(sim);
    v.into_stack(state)
}

fn shared_set_min_max_values(state: &mut LuaState) -> LuaResult<u32> {
    use crate::widget::WidgetType;
    let id = frame_id_from_stack(state, 1)?;
    let min = val_to_f64(stack_val(state, 2));
    let max = val_to_f64(stack_val(state, 3));
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        match f.widget_type {
            WidgetType::Slider => {
                f.slider_min = min;
                f.slider_max = max;
                f.slider_value = f.slider_value.clamp(min, max);
            }
            WidgetType::StatusBar => {
                f.statusbar_min = min;
                f.statusbar_max = max;
                f.statusbar_value = f.statusbar_value.clamp(min, max);
                f.statusbar_interpolated_value = f.statusbar_interpolated_value.clamp(min, max);
                f.statusbar_interpolation_target = f
                    .statusbar_interpolation_target
                    .map(|t| t.clamp(min, max))
                    .filter(|&t| t != f.statusbar_interpolated_value);
            }
            _ => {}
        }
    }
    Ok(0)
}

fn shared_get_min_max_values(state: &mut LuaState) -> LuaResult<u32> {
    use crate::widget::WidgetType;
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let (min, max) = sim
        .widgets
        .get(id)
        .map(|f| match f.widget_type {
            WidgetType::Slider => (f.slider_min, f.slider_max),
            WidgetType::StatusBar => (f.statusbar_min, f.statusbar_max),
            _ => (0.0, 1.0),
        })
        .unwrap_or((0.0, 1.0));
    drop(sim);
    (min, max).into_stack(state)
}

// ---------------------------------------------------------------------------
// CheckButton methods
// ---------------------------------------------------------------------------

fn checkbutton_set_checked(state: &mut LuaState) -> LuaResult<u32> {
    use crate::widget::AttributeValue;
    let id = frame_id_from_stack(state, 1)?;
    let checked = val_to_bool(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    let already = sim
        .widgets
        .get(id)
        .and_then(|f| f.attributes.get("__checked"))
        .map(|v| matches!(v, AttributeValue::Boolean(b) if *b == checked))
        .unwrap_or(false);
    if already {
        return Ok(0);
    }
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.attributes
            .insert("__checked".to_string(), AttributeValue::Boolean(checked));
    }
    let checked_tex_id = sim
        .widgets
        .get(id)
        .and_then(|f| f.children_keys.get("CheckedTexture").copied());
    if let Some(tex_id) = checked_tex_id {
        sim.set_frame_visible(tex_id, checked);
    }
    Ok(0)
}

fn checkbutton_get_checked(state: &mut LuaState) -> LuaResult<u32> {
    use crate::widget::AttributeValue;
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .and_then(|f| f.attributes.get("__checked"))
        .map(|v| matches!(v, AttributeValue::Boolean(true)))
        .unwrap_or(false);
    drop(sim);
    v.into_stack(state)
}

// ---------------------------------------------------------------------------
// StatusBar methods
// ---------------------------------------------------------------------------

fn statusbar_set_status_bar_color(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let r = val_to_f64(stack_val(state, 2)) as f32;
    let g = val_to_f64(stack_val(state, 3)) as f32;
    let b = val_to_f64(stack_val(state, 4)) as f32;
    let a = val_to_f64(stack_val(state, 5)) as f32;
    let color = crate::widget::Color::new(r, g, b, a);
    let mut sim = borrow_state_mut(state)?;
    let bar_id = statusbar_child_id_inner(&sim, id);
    if let Some(bar_id) = bar_id {
        if let Some(bar) = sim.widgets.get_mut_visual(bar_id) {
            bar.vertex_color = Some(color);
        }
    }
    Ok(0)
}

fn statusbar_get_status_bar_color(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let bar_id = statusbar_child_id_inner(&sim, id);
    let (r, g, b, a) = bar_id
        .and_then(|bid| sim.widgets.get(bid))
        .and_then(|f| f.vertex_color)
        .map(|c| (c.r as f64, c.g as f64, c.b as f64, c.a as f64))
        .unwrap_or((1.0, 1.0, 1.0, 1.0));
    drop(sim);
    (r, g, b, a).into_stack(state)
}

fn statusbar_child_id_inner(sim: &crate::lua_api::SimState, id: u64) -> Option<u64> {
    let bar_id = sim.widgets.get(id)?.statusbar_bar_id?;
    sim.widgets
        .get(bar_id)
        .is_some_and(|f| f.parent_id == Some(id))
        .then_some(bar_id)
}

fn statusbar_set_fill_style(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let style = opt_string(state, 2).unwrap_or_else(|| "STANDARD".to_string());
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.statusbar_fill_style = style;
    }
    Ok(0)
}

fn statusbar_get_fill_style(state: &mut LuaState) -> LuaResult<u32> {
    let val = create_string(state, "STANDARD");
    val.into_stack(state)
}

fn statusbar_set_reverse_fill(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let reverse = val_to_bool(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.statusbar_reverse_fill = reverse;
    }
    Ok(0)
}

fn statusbar_get_reverse_fill(state: &mut LuaState) -> LuaResult<u32> {
    false.into_stack(state)
}

fn statusbar_get_interpolated_value(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .map(|f| f.statusbar_interpolated_value)
        .unwrap_or(0.0);
    drop(sim);
    v.into_stack(state)
}

fn statusbar_is_interpolating(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .and_then(|f| f.statusbar_interpolation_target)
        .is_some();
    drop(sim);
    v.into_stack(state)
}

fn statusbar_set_to_target_value(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        if let Some(target) = f.statusbar_interpolation_target.take() {
            f.statusbar_interpolated_value = target;
        }
    }
    Ok(0)
}

fn statusbar_set_desaturated(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let desat = val_to_bool(stack_val(state, 2));
    apply_statusbar_desaturation_inner(state, id, if desat { 1.0 } else { 0.0 });
    Ok(0)
}

fn statusbar_get_desaturated(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .map(|f| f.statusbar_desaturation > 0.0)
        .unwrap_or(false);
    drop(sim);
    v.into_stack(state)
}

fn statusbar_set_desaturation(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let desat = val_to_f64(stack_val(state, 2));
    apply_statusbar_desaturation_inner(state, id, desat);
    Ok(0)
}

fn statusbar_get_desaturation(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .map(|f| f.statusbar_desaturation)
        .unwrap_or(0.0);
    drop(sim);
    v.into_stack(state)
}

fn apply_statusbar_desaturation_inner(state: &mut LuaState, id: u64, desaturation: f64) {
    let clamped = desaturation.clamp(0.0, 1.0);
    if let Ok(mut sim) = borrow_state_mut(state) {
        let child_id = sim.widgets.get(id).and_then(|f| f.statusbar_bar_id);
        if let Some(f) = sim.widgets.get_mut_visual(id) {
            f.statusbar_desaturation = clamped;
        }
        if let Some(child_id) = child_id {
            let is_desat = sim.widgets.get(id).map(|f| f.statusbar_desaturation > 0.0).unwrap_or(false);
            if let Some(child) = sim.widgets.get_mut_visual(child_id) {
                child.desaturated = is_desat;
            }
        }
    }
}

fn statusbar_get_rotates_texture(state: &mut LuaState) -> LuaResult<u32> {
    false.into_stack(state)
}

fn statusbar_set_rotates_texture(state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn statusbar_is_status_bar_desaturated(state: &mut LuaState) -> LuaResult<u32> {
    statusbar_get_desaturated(state)
}

fn statusbar_get_status_bar_desaturation(state: &mut LuaState) -> LuaResult<u32> {
    statusbar_get_desaturation(state)
}

fn statusbar_get_status_bar_desaturated(state: &mut LuaState) -> LuaResult<u32> {
    statusbar_get_desaturated(state)
}

// ---------------------------------------------------------------------------
// Model methods
// ---------------------------------------------------------------------------

fn model_set_model(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let path = opt_string(state, 2);
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.model_path = path;
        f.model_file_id = None;
    }
    Ok(0)
}

fn model_get_model(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let path = {
        let sim = borrow_state(state)?;
        sim.widgets.get(id).and_then(|f| f.model_path.clone()).unwrap_or_default()
    };
    let val = create_string(state, &path);
    val.into_stack(state)
}

fn model_set_model_scale(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let scale = val_to_f64(stack_val(state, 2)) as f32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.model_transform.scale = scale;
    }
    Ok(0)
}

fn model_get_model_scale(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .map(|f| f.model_transform.scale as f64)
        .unwrap_or(1.0);
    drop(sim);
    v.into_stack(state)
}

fn model_set_position(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let x = val_to_f64(stack_val(state, 2)) as f32;
    let y = val_to_f64(stack_val(state, 3)) as f32;
    let z = val_to_f64(stack_val(state, 4)) as f32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.model_transform.position = (x, y, z);
    }
    Ok(0)
}

fn model_get_position(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let (x, y, z) = sim
        .widgets
        .get(id)
        .map(|f| {
            let p = f.model_transform.position;
            (p.0 as f64, p.1 as f64, p.2 as f64)
        })
        .unwrap_or((0.0, 0.0, 0.0));
    drop(sim);
    (x, y, z).into_stack(state)
}

fn model_set_facing(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let rad = val_to_f64(stack_val(state, 2)) as f32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.model_transform.facing = rad;
    }
    Ok(0)
}

fn model_get_facing(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .map(|f| f.model_transform.facing as f64)
        .unwrap_or(0.0);
    drop(sim);
    v.into_stack(state)
}

fn model_set_rotation(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let rad = val_to_f64(stack_val(state, 2)) as f32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.rotation = rad;
    }
    Ok(0)
}

fn model_set_animation(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let anim_id = val_to_f64(stack_val(state, 2)) as i32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.model_appearance.animation_id = Some(anim_id);
    }
    Ok(0)
}

fn model_set_display_info(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let display_id = val_to_f64(stack_val(state, 2)) as i32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.model_path = None;
        f.model_file_id = None;
        f.model_appearance.display_info = Some(display_id);
        f.model_appearance.creature_id = None;
    }
    Ok(0)
}

fn model_set_creature(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let creature_id = val_to_f64(stack_val(state, 2)) as i32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.model_path = None;
        f.model_file_id = None;
        f.model_appearance.display_info = None;
        f.model_appearance.creature_id = Some(creature_id);
    }
    Ok(0)
}

fn model_clear_model(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.model_path = None;
        f.model_file_id = None;
        f.model_appearance.display_info = None;
        f.model_appearance.creature_id = None;
        f.model_appearance.animation_id = None;
        f.model_appearance.sequence_id = None;
        f.model_appearance.sequence_time_ms = None;
    }
    Ok(0)
}

fn model_get_model_file_id(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim.widgets.get(id).and_then(|f| f.model_file_id).unwrap_or(0);
    drop(sim);
    (v as f64).into_stack(state)
}

fn model_set_model_alpha(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let alpha = val_to_f64(stack_val(state, 2)) as f32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.model_rendering.alpha = alpha;
    }
    Ok(0)
}

fn model_get_model_alpha(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .map(|f| f.model_rendering.alpha as f64)
        .unwrap_or(1.0);
    drop(sim);
    v.into_stack(state)
}

fn model_set_sequence(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let seq = val_to_f64(stack_val(state, 2)) as i32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.model_appearance.sequence_id = Some(seq);
        f.model_appearance.sequence_time_ms = None;
    }
    Ok(0)
}

fn model_set_sequence_time(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let seq = val_to_f64(stack_val(state, 2)) as i32;
    let time = val_to_f64(stack_val(state, 3)) as i32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.model_appearance.sequence_id = Some(seq);
        f.model_appearance.sequence_time_ms = Some(time);
    }
    Ok(0)
}

fn model_get_camera_distance(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .map(|f| f.model_transform.camera.distance as f64)
        .unwrap_or(0.0);
    drop(sim);
    v.into_stack(state)
}

fn model_set_camera_distance(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let dist = val_to_f64(stack_val(state, 2)) as f32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.model_transform.camera.distance = dist;
    }
    Ok(0)
}

fn model_get_camera_facing(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .widgets
        .get(id)
        .map(|f| f.model_transform.camera.facing as f64)
        .unwrap_or(0.0);
    drop(sim);
    v.into_stack(state)
}

fn model_set_camera_facing(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let rad = val_to_f64(stack_val(state, 2)) as f32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.model_transform.camera.facing = rad;
    }
    Ok(0)
}

fn model_get_camera_target(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let t = sim
        .widgets
        .get(id)
        .map(|f| f.model_transform.camera.target)
        .unwrap_or((0.0, 0.0, 0.0));
    drop(sim);
    (t.0 as f64, t.1 as f64, t.2 as f64).into_stack(state)
}

fn model_set_camera_target(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let x = val_to_f64(stack_val(state, 2)) as f32;
    let y = val_to_f64(stack_val(state, 3)) as f32;
    let z = val_to_f64(stack_val(state, 4)) as f32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.model_transform.camera.target = (x, y, z);
    }
    Ok(0)
}

fn model_stub_variadic(state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn model_stub_nil(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    Ok(1)
}

fn model_stub_zero(state: &mut LuaState) -> LuaResult<u32> {
    0.0_f64.into_stack(state)
}

fn model_stub_one(state: &mut LuaState) -> LuaResult<u32> {
    1.0_f64.into_stack(state)
}

fn model_stub_false(state: &mut LuaState) -> LuaResult<u32> {
    false.into_stack(state)
}

// ---------------------------------------------------------------------------
// Tooltip methods (GameTooltip)
// ---------------------------------------------------------------------------

fn tooltip_clear_lines(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(td) = sim.tooltips.get_mut(&id) {
        td.lines.clear();
        td.spell_id = None;
    }
    drop(sim);
    // TODO: fire OnTooltipCleared script
    Ok(0)
}

fn tooltip_add_line(state: &mut LuaState) -> LuaResult<u32> {
    use crate::lua_api::tooltip::TooltipLine;
    let id = frame_id_from_stack(state, 1)?;
    let text = opt_string(state, 2).unwrap_or_default();
    let r = val_to_f64(stack_val(state, 3)) as f32;
    let g = val_to_f64(stack_val(state, 4)) as f32;
    let b = val_to_f64(stack_val(state, 5)) as f32;
    let wrap = val_to_bool(stack_val(state, 6));
    let mut sim = borrow_state_mut(state)?;
    if let Some(td) = sim.tooltips.get_mut(&id) {
        td.lines.push(TooltipLine {
            left_text: text,
            left_color: (r, g, b),
            right_text: None,
            right_color: (1.0, 1.0, 1.0),
            wrap,
            texture: None,
        });
    }
    Ok(0)
}

fn tooltip_add_double_line(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: full double-line impl (complex, needs right_text / right_color parsing)
    tooltip_add_line(state)
}

fn tooltip_num_lines(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim.tooltips.get(&id).map(|td| td.lines.len()).unwrap_or(0);
    drop(sim);
    (v as f64).into_stack(state)
}

fn tooltip_set_custom_line_spacing(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let spacing = val_to_f64(stack_val(state, 2)) as f32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(td) = sim.tooltips.get_mut(&id) {
        td.line_spacing = Some(spacing);
    }
    Ok(0)
}

fn tooltip_get_custom_line_spacing(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim
        .tooltips
        .get(&id)
        .and_then(|td| td.line_spacing)
        .map(|s| s as f64)
        .unwrap_or(0.0);
    drop(sim);
    v.into_stack(state)
}

fn tooltip_set_minimum_width(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let width = val_to_f64(stack_val(state, 2)) as f32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(td) = sim.tooltips.get_mut(&id) {
        td.min_width = width;
    }
    Ok(0)
}

fn tooltip_get_minimum_width(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim.tooltips.get(&id).map(|td| td.min_width as f64).unwrap_or(0.0);
    drop(sim);
    v.into_stack(state)
}

fn tooltip_set_allow_show_with_no_lines(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = val_to_bool(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    if let Some(td) = sim.tooltips.get_mut(&id) {
        td.allow_show_with_no_lines = value;
    }
    Ok(0)
}

fn tooltip_set_custom_word_wrap_min_width(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let width = val_to_f64(stack_val(state, 2)) as f32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(td) = sim.tooltips.get_mut(&id) {
        td.custom_word_wrap_min_width = Some(width);
    }
    Ok(0)
}

fn tooltip_set_shrink_to_fit_wrapped(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let value = val_to_bool(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    if let Some(td) = sim.tooltips.get_mut(&id) {
        td.shrink_to_fit_wrapped = value;
    }
    Ok(0)
}

fn tooltip_get_spell(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let spell_id = {
        let sim = borrow_state(state)?;
        sim.tooltips.get(&id).and_then(|td| td.spell_id)
    };
    match spell_id {
        Some(id) => {
            let name = crate::spells::get_spell(id)
                .map(|s| s.name.to_string())
                .unwrap_or_else(|| format!("Spell {}", id));
            let name_val = create_string(state, &name);
            name_val.into_stack(state)?;
            (id as f64).into_stack(state)?;
            Ok(2)
        }
        None => {
            state.push(Val::Nil);
            state.push(Val::Nil);
            Ok(2)
        }
    }
}

fn tooltip_get_unit(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    state.push(Val::Nil);
    Ok(2)
}

fn tooltip_get_item(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    state.push(Val::Nil);
    Ok(2)
}

fn tooltip_set_padding(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let padding = val_to_f64(stack_val(state, 2)) as f32;
    let mut sim = borrow_state_mut(state)?;
    if let Some(td) = sim.tooltips.get_mut(&id) {
        td.padding = padding;
    }
    Ok(0)
}

fn tooltip_get_padding(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let v = sim.tooltips.get(&id).map(|td| td.padding as f64).unwrap_or(0.0);
    drop(sim);
    v.into_stack(state)
}

fn tooltip_clear_padding(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(td) = sim.tooltips.get_mut(&id) {
        td.padding = 0.0;
    }
    Ok(0)
}

fn tooltip_append_text(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let text = opt_string(state, 2).unwrap_or_default();
    let mut sim = borrow_state_mut(state)?;
    if let Some(td) = sim.tooltips.get_mut(&id) {
        if let Some(last) = td.lines.last_mut() {
            last.left_text.push_str(&text);
        }
    }
    Ok(0)
}

fn tooltip_set_spell_by_id(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: populate_spell_tooltip + fire OnTooltipSetSpell
    Ok(0)
}

fn tooltip_set_item_by_id(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: populate_item_tooltip + fire OnTooltipSetItem
    Ok(0)
}

fn tooltip_set_hyperlink(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: parse item/spell hyperlink and populate
    Ok(0)
}

fn tooltip_set_unit(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: set_unit_for_tooltip
    Ok(0)
}

fn tooltip_set_unit_buff(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: lookup_aura + populate_aura_tooltip
    Ok(0)
}

fn tooltip_set_unit_debuff(state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn tooltip_set_unit_aura(state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn tooltip_set_owner(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: set_owner_impl (complex: anchor type, parent frame, etc.)
    Ok(0)
}

fn tooltip_set_anchor_type(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: set_anchor_type_impl
    Ok(0)
}

fn tooltip_copy_tooltip(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: copy_tooltip_impl
    Ok(0)
}

fn tooltip_set_frame_stack(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: set_frame_stack_impl
    Ok(0)
}

fn tooltip_add_fonts_strings(state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

// ---------------------------------------------------------------------------
// register_all
// ---------------------------------------------------------------------------

/// Register all widget-specific methods on the frame metatable.
///
/// Call this after the standard frame metatable has been created,
/// passing its `GcRef<Table>` as `metatable`.
pub fn register_all(state: &mut LuaState, metatable: GcRef<Table>) -> LuaResult<()> {
    // --- Cooldown ---
    table_set_rust_fn(state, metatable, "SetCooldown", cooldown_set_cooldown)?;
    table_set_rust_fn(state, metatable, "SetCooldownUNIX", cooldown_set_cooldown_unix)?;
    table_set_rust_fn(state, metatable, "SetCooldownFromExpirationTime", cooldown_set_cooldown_from_expiration_time)?;
    table_set_rust_fn(state, metatable, "SetCooldownDuration", cooldown_set_cooldown_duration)?;
    table_set_rust_fn(state, metatable, "SetCooldownFromDurationObject", cooldown_set_from_duration_object)?;
    table_set_rust_fn(state, metatable, "GetCooldownTimes", cooldown_get_cooldown_times)?;
    table_set_rust_fn(state, metatable, "GetCooldownDuration", cooldown_get_cooldown_duration)?;
    table_set_rust_fn(state, metatable, "GetCooldownDisplayDuration", cooldown_get_cooldown_display_duration)?;
    table_set_rust_fn(state, metatable, "Clear", cooldown_clear)?;
    table_set_rust_fn(state, metatable, "Pause", cooldown_pause)?;
    table_set_rust_fn(state, metatable, "Resume", cooldown_resume)?;
    table_set_rust_fn(state, metatable, "IsPaused", cooldown_is_paused)?;
    table_set_rust_fn(state, metatable, "SetDrawSwipe", cooldown_set_draw_swipe)?;
    table_set_rust_fn(state, metatable, "GetDrawSwipe", cooldown_get_draw_swipe)?;
    table_set_rust_fn(state, metatable, "SetDrawEdge", cooldown_set_draw_edge)?;
    table_set_rust_fn(state, metatable, "GetDrawEdge", cooldown_get_draw_edge)?;
    table_set_rust_fn(state, metatable, "SetDrawBling", cooldown_set_draw_bling)?;
    table_set_rust_fn(state, metatable, "GetDrawBling", cooldown_get_draw_bling)?;
    table_set_rust_fn(state, metatable, "SetReverse", cooldown_set_reverse)?;
    table_set_rust_fn(state, metatable, "GetReverse", cooldown_get_reverse)?;
    table_set_rust_fn(state, metatable, "SetHideCountdownNumbers", cooldown_set_hide_countdown_numbers)?;
    table_set_rust_fn(state, metatable, "GetHideCountdownNumbers", cooldown_get_hide_countdown_numbers)?;
    table_set_rust_fn(state, metatable, "SetEdgeScale", cooldown_set_edge_scale)?;
    table_set_rust_fn(state, metatable, "GetEdgeScale", cooldown_get_edge_scale)?;
    table_set_rust_fn(state, metatable, "SetMinimumCountdownDuration", cooldown_set_minimum_countdown_duration)?;
    table_set_rust_fn(state, metatable, "GetMinimumCountdownDuration", cooldown_get_minimum_countdown_duration)?;
    table_set_rust_fn(state, metatable, "SetUseAuraDisplayTime", cooldown_set_use_aura_display_time)?;
    table_set_rust_fn(state, metatable, "GetUseAuraDisplayTime", cooldown_get_use_aura_display_time)?;
    table_set_rust_fn(state, metatable, "SetUseCircularEdge", cooldown_set_use_circular_edge)?;
    table_set_rust_fn(state, metatable, "SetCountdownAbbrevThreshold", cooldown_set_countdown_abbrev_threshold)?;
    table_set_rust_fn(state, metatable, "SetSwipeTexture", cooldown_set_swipe_texture)?;
    table_set_rust_fn(state, metatable, "SetEdgeTexture", cooldown_set_edge_texture)?;
    table_set_rust_fn(state, metatable, "SetBlingTexture", cooldown_set_bling_texture)?;
    table_set_rust_fn(state, metatable, "SetCountdownFont", cooldown_set_countdown_font)?;
    table_set_rust_fn(state, metatable, "GetCountdownFontString", cooldown_get_countdown_font_string)?;
    table_set_rust_fn(state, metatable, "SetSwipeColor", cooldown_set_swipe_color)?;
    table_set_rust_fn(state, metatable, "SetEdgeColor", cooldown_set_edge_color)?;
    table_set_rust_fn(state, metatable, "SetTexCoordRange", cooldown_set_tex_coord_range)?;

    // --- EditBox ---
    table_set_rust_fn(state, metatable, "SetFocus", editbox_set_focus)?;
    table_set_rust_fn(state, metatable, "ClearFocus", editbox_clear_focus)?;
    table_set_rust_fn(state, metatable, "HasFocus", editbox_has_focus)?;
    table_set_rust_fn(state, metatable, "HasText", editbox_has_text)?;
    table_set_rust_fn(state, metatable, "SetCursorPosition", editbox_set_cursor_position)?;
    table_set_rust_fn(state, metatable, "GetCursorPosition", editbox_get_cursor_position)?;
    table_set_rust_fn(state, metatable, "GetNumLetters", editbox_get_num_letters)?;
    table_set_rust_fn(state, metatable, "SetMaxLetters", editbox_set_max_letters)?;
    table_set_rust_fn(state, metatable, "GetMaxLetters", editbox_get_max_letters)?;
    table_set_rust_fn(state, metatable, "SetMultiLine", editbox_set_multi_line)?;
    table_set_rust_fn(state, metatable, "IsMultiLine", editbox_is_multi_line)?;
    table_set_rust_fn(state, metatable, "SetAutoFocus", editbox_set_auto_focus)?;
    table_set_rust_fn(state, metatable, "IsAutoFocus", editbox_is_auto_focus)?;
    table_set_rust_fn(state, metatable, "SetNumeric", editbox_set_numeric)?;
    table_set_rust_fn(state, metatable, "IsNumeric", editbox_is_numeric)?;
    table_set_rust_fn(state, metatable, "SetPassword", editbox_set_password)?;
    table_set_rust_fn(state, metatable, "IsPassword", editbox_is_password)?;
    table_set_rust_fn(state, metatable, "SetNumber", editbox_set_number)?;
    table_set_rust_fn(state, metatable, "GetNumber", editbox_get_number)?;
    table_set_rust_fn(state, metatable, "AddHistoryLine", editbox_add_history_line)?;
    table_set_rust_fn(state, metatable, "GetHistoryLines", editbox_get_history_lines)?;
    table_set_rust_fn(state, metatable, "SetHistoryLines", editbox_set_history_lines)?;
    table_set_rust_fn(state, metatable, "ClearHistory", editbox_clear_history)?;
    table_set_rust_fn(state, metatable, "GetInputLanguage", editbox_get_input_language)?;
    table_set_rust_fn(state, metatable, "ToggleInputLanguage", editbox_toggle_input_language)?;
    table_set_rust_fn(state, metatable, "ResetInputMode", editbox_reset_input_mode)?;
    table_set_rust_fn(state, metatable, "SetTextInsets", editbox_set_text_insets)?;
    table_set_rust_fn(state, metatable, "GetTextInsets", editbox_get_text_insets)?;
    table_set_rust_fn(state, metatable, "GetDisplayText", editbox_get_display_text)?;
    table_set_rust_fn(state, metatable, "Insert", editbox_insert)?;
    table_set_rust_fn(state, metatable, "SetBlinkSpeed", editbox_set_blink_speed)?;
    table_set_rust_fn(state, metatable, "GetBlinkSpeed", editbox_get_blink_speed)?;
    table_set_rust_fn(state, metatable, "SetAltArrowKeyMode", editbox_set_alt_arrow_key_mode)?;
    table_set_rust_fn(state, metatable, "GetAltArrowKeyMode", editbox_get_alt_arrow_key_mode)?;
    table_set_rust_fn(state, metatable, "HighlightText", editbox_highlight_text)?;
    table_set_rust_fn(state, metatable, "ClearHighlightText", editbox_clear_highlight_text)?;

    // --- Slider ---
    table_set_rust_fn(state, metatable, "SetValueStep", slider_set_value_step)?;
    table_set_rust_fn(state, metatable, "GetValueStep", slider_get_value_step)?;
    table_set_rust_fn(state, metatable, "SetOrientation", slider_set_orientation)?;
    table_set_rust_fn(state, metatable, "GetOrientation", slider_get_orientation)?;
    table_set_rust_fn(state, metatable, "SetObeyStepOnDrag", slider_set_obey_step_on_drag)?;
    table_set_rust_fn(state, metatable, "GetObeyStepOnDrag", slider_get_obey_step_on_drag)?;
    table_set_rust_fn(state, metatable, "SetStepsPerPage", slider_set_steps_per_page)?;
    table_set_rust_fn(state, metatable, "GetStepsPerPage", slider_get_steps_per_page)?;
    table_set_rust_fn(state, metatable, "IsDraggingThumb", slider_is_dragging_thumb)?;

    // --- Shared value (Slider + StatusBar) ---
    table_set_rust_fn(state, metatable, "SetValue", shared_set_value)?;
    table_set_rust_fn(state, metatable, "GetValue", shared_get_value)?;
    table_set_rust_fn(state, metatable, "SetMinMaxValues", shared_set_min_max_values)?;
    table_set_rust_fn(state, metatable, "GetMinMaxValues", shared_get_min_max_values)?;

    // --- CheckButton ---
    table_set_rust_fn(state, metatable, "SetChecked", checkbutton_set_checked)?;
    table_set_rust_fn(state, metatable, "GetChecked", checkbutton_get_checked)?;

    // --- StatusBar ---
    table_set_rust_fn(state, metatable, "SetStatusBarColor", statusbar_set_status_bar_color)?;
    table_set_rust_fn(state, metatable, "GetStatusBarColor", statusbar_get_status_bar_color)?;
    table_set_rust_fn(state, metatable, "SetFillStyle", statusbar_set_fill_style)?;
    table_set_rust_fn(state, metatable, "GetFillStyle", statusbar_get_fill_style)?;
    table_set_rust_fn(state, metatable, "SetReverseFill", statusbar_set_reverse_fill)?;
    table_set_rust_fn(state, metatable, "GetReverseFill", statusbar_get_reverse_fill)?;
    table_set_rust_fn(state, metatable, "GetInterpolatedValue", statusbar_get_interpolated_value)?;
    table_set_rust_fn(state, metatable, "IsInterpolating", statusbar_is_interpolating)?;
    table_set_rust_fn(state, metatable, "SetToTargetValue", statusbar_set_to_target_value)?;
    table_set_rust_fn(state, metatable, "SetStatusBarDesaturated", statusbar_set_desaturated)?;
    table_set_rust_fn(state, metatable, "GetStatusBarDesaturated", statusbar_get_status_bar_desaturated)?;
    table_set_rust_fn(state, metatable, "SetStatusBarDesaturation", statusbar_set_desaturation)?;
    table_set_rust_fn(state, metatable, "GetStatusBarDesaturation", statusbar_get_status_bar_desaturation)?;
    table_set_rust_fn(state, metatable, "IsStatusBarDesaturated", statusbar_is_status_bar_desaturated)?;
    table_set_rust_fn(state, metatable, "SetRotatesTexture", statusbar_set_rotates_texture)?;
    table_set_rust_fn(state, metatable, "GetRotatesTexture", statusbar_get_rotates_texture)?;

    // --- Model ---
    table_set_rust_fn(state, metatable, "SetModel", model_set_model)?;
    table_set_rust_fn(state, metatable, "GetModel", model_get_model)?;
    table_set_rust_fn(state, metatable, "SetModelScale", model_set_model_scale)?;
    table_set_rust_fn(state, metatable, "GetModelScale", model_get_model_scale)?;
    table_set_rust_fn(state, metatable, "SetPosition", model_set_position)?;
    table_set_rust_fn(state, metatable, "GetPosition", model_get_position)?;
    table_set_rust_fn(state, metatable, "SetFacing", model_set_facing)?;
    table_set_rust_fn(state, metatable, "GetFacing", model_get_facing)?;
    table_set_rust_fn(state, metatable, "SetRotation", model_set_rotation)?;
    table_set_rust_fn(state, metatable, "SetAnimation", model_set_animation)?;
    table_set_rust_fn(state, metatable, "SetDisplayInfo", model_set_display_info)?;
    table_set_rust_fn(state, metatable, "SetCreature", model_set_creature)?;
    table_set_rust_fn(state, metatable, "ClearModel", model_clear_model)?;
    table_set_rust_fn(state, metatable, "GetModelFileID", model_get_model_file_id)?;
    table_set_rust_fn(state, metatable, "SetModelAlpha", model_set_model_alpha)?;
    table_set_rust_fn(state, metatable, "GetModelAlpha", model_get_model_alpha)?;
    table_set_rust_fn(state, metatable, "SetSequence", model_set_sequence)?;
    table_set_rust_fn(state, metatable, "SetSequenceTime", model_set_sequence_time)?;
    table_set_rust_fn(state, metatable, "GetCameraDistance", model_get_camera_distance)?;
    table_set_rust_fn(state, metatable, "SetCameraDistance", model_set_camera_distance)?;
    table_set_rust_fn(state, metatable, "GetCameraFacing", model_get_camera_facing)?;
    table_set_rust_fn(state, metatable, "SetCameraFacing", model_set_camera_facing)?;
    table_set_rust_fn(state, metatable, "GetCameraTarget", model_get_camera_target)?;
    table_set_rust_fn(state, metatable, "SetCameraTarget", model_set_camera_target)?;
    // Model stubs: complex camera / rendering ops
    table_set_rust_fn(state, metatable, "SetAutoDress", model_stub_variadic)?;
    table_set_rust_fn(state, metatable, "SetCamDistanceScale", model_stub_variadic)?;
    table_set_rust_fn(state, metatable, "SetCamera", model_stub_variadic)?;
    table_set_rust_fn(state, metatable, "SetPortraitZoom", model_stub_variadic)?;
    table_set_rust_fn(state, metatable, "SetDesaturation", model_stub_variadic)?;
    table_set_rust_fn(state, metatable, "SetLight", model_stub_variadic)?;
    table_set_rust_fn(state, metatable, "ResetLights", model_stub_variadic)?;
    table_set_rust_fn(state, metatable, "RefreshUnit", model_stub_variadic)?;
    table_set_rust_fn(state, metatable, "RefreshCamera", model_stub_variadic)?;
    table_set_rust_fn(state, metatable, "TransitionToModelSceneID", model_stub_variadic)?;
    table_set_rust_fn(state, metatable, "SetFromModelSceneID", model_stub_variadic)?;
    table_set_rust_fn(state, metatable, "CycleVariation", model_stub_variadic)?;
    table_set_rust_fn(state, metatable, "GetModelSceneID", model_stub_zero)?;
    table_set_rust_fn(state, metatable, "GetCamDistanceScale", model_stub_one)?;
    table_set_rust_fn(state, metatable, "HasCustomCamera", model_stub_false)?;
    table_set_rust_fn(state, metatable, "GetPaused", model_stub_false)?;
    table_set_rust_fn(state, metatable, "HasAttachmentPoints", model_stub_false)?;
    table_set_rust_fn(state, metatable, "GetLight", model_stub_nil)?;
    table_set_rust_fn(state, metatable, "AdvanceTime", model_stub_variadic)?;
    table_set_rust_fn(state, metatable, "ClearTransform", model_stub_variadic)?;
    table_set_rust_fn(state, metatable, "SetTransform", model_stub_variadic)?;
    table_set_rust_fn(state, metatable, "SetPitch", model_stub_variadic)?;
    table_set_rust_fn(state, metatable, "GetPitch", model_stub_zero)?;
    table_set_rust_fn(state, metatable, "SetRoll", model_stub_variadic)?;
    table_set_rust_fn(state, metatable, "GetRoll", model_stub_zero)?;
    table_set_rust_fn(state, metatable, "GetWorldScale", model_stub_one)?;
    table_set_rust_fn(state, metatable, "TransformCameraSpaceToModelSpace", model_stub_nil)?;
    table_set_rust_fn(state, metatable, "UseModelCenterToTransform", model_stub_variadic)?;
    table_set_rust_fn(state, metatable, "IsUsingModelCenterToTransform", model_stub_false)?;
    table_set_rust_fn(state, metatable, "SetViewTranslation", model_stub_variadic)?;
    table_set_rust_fn(state, metatable, "SetModelDrawLayer", model_stub_variadic)?;
    table_set_rust_fn(state, metatable, "ReplaceIconTexture", model_stub_variadic)?;
    table_set_rust_fn(state, metatable, "SetGlow", model_stub_variadic)?;
    table_set_rust_fn(state, metatable, "SetGradientMask", model_stub_variadic)?;
    table_set_rust_fn(state, metatable, "SetShadowEffect", model_stub_variadic)?;
    table_set_rust_fn(state, metatable, "GetShadowEffect", model_stub_zero)?;
    table_set_rust_fn(state, metatable, "SetParticlesEnabled", model_stub_variadic)?;
    table_set_rust_fn(state, metatable, "SetUseGBuffer", model_stub_variadic)?;
    table_set_rust_fn(state, metatable, "SetCustomCamera", model_stub_variadic)?;
    table_set_rust_fn(state, metatable, "MakeCurrentCameraCustom", model_stub_variadic)?;
    table_set_rust_fn(state, metatable, "GetUpperEmblemTexture", model_stub_nil)?;
    table_set_rust_fn(state, metatable, "GetLowerEmblemTexture", model_stub_nil)?;

    // --- Tooltip ---
    table_set_rust_fn(state, metatable, "ClearLines", tooltip_clear_lines)?;
    table_set_rust_fn(state, metatable, "AddLine", tooltip_add_line)?;
    table_set_rust_fn(state, metatable, "AddDoubleLine", tooltip_add_double_line)?;
    table_set_rust_fn(state, metatable, "NumLines", tooltip_num_lines)?;
    table_set_rust_fn(state, metatable, "GetNumLines", tooltip_num_lines)?;
    table_set_rust_fn(state, metatable, "SetCustomLineSpacing", tooltip_set_custom_line_spacing)?;
    table_set_rust_fn(state, metatable, "GetCustomLineSpacing", tooltip_get_custom_line_spacing)?;
    table_set_rust_fn(state, metatable, "SetMinimumWidth", tooltip_set_minimum_width)?;
    table_set_rust_fn(state, metatable, "GetMinimumWidth", tooltip_get_minimum_width)?;
    table_set_rust_fn(state, metatable, "SetAllowShowWithNoLines", tooltip_set_allow_show_with_no_lines)?;
    table_set_rust_fn(state, metatable, "SetCustomWordWrapMinWidth", tooltip_set_custom_word_wrap_min_width)?;
    table_set_rust_fn(state, metatable, "SetShrinkToFitWrapped", tooltip_set_shrink_to_fit_wrapped)?;
    table_set_rust_fn(state, metatable, "GetSpell", tooltip_get_spell)?;
    table_set_rust_fn(state, metatable, "GetUnit", tooltip_get_unit)?;
    table_set_rust_fn(state, metatable, "GetItem", tooltip_get_item)?;
    table_set_rust_fn(state, metatable, "SetPadding", tooltip_set_padding)?;
    table_set_rust_fn(state, metatable, "GetPadding", tooltip_get_padding)?;
    table_set_rust_fn(state, metatable, "ClearPadding", tooltip_clear_padding)?;
    table_set_rust_fn(state, metatable, "AppendText", tooltip_append_text)?;
    table_set_rust_fn(state, metatable, "SetSpellByID", tooltip_set_spell_by_id)?;
    table_set_rust_fn(state, metatable, "SetItemByID", tooltip_set_item_by_id)?;
    table_set_rust_fn(state, metatable, "SetHyperlink", tooltip_set_hyperlink)?;
    table_set_rust_fn(state, metatable, "SetUnit", tooltip_set_unit)?;
    table_set_rust_fn(state, metatable, "SetUnitBuff", tooltip_set_unit_buff)?;
    table_set_rust_fn(state, metatable, "SetUnitDebuff", tooltip_set_unit_debuff)?;
    table_set_rust_fn(state, metatable, "SetUnitAura", tooltip_set_unit_aura)?;
    table_set_rust_fn(state, metatable, "SetOwner", tooltip_set_owner)?;
    table_set_rust_fn(state, metatable, "SetAnchorType", tooltip_set_anchor_type)?;
    table_set_rust_fn(state, metatable, "CopyTooltip", tooltip_copy_tooltip)?;
    table_set_rust_fn(state, metatable, "SetFrameStack", tooltip_set_frame_stack)?;
    table_set_rust_fn(state, metatable, "AddFontStrings", tooltip_add_fonts_strings)?;

    Ok(())
}
