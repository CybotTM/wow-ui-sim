//! Cooldown widget methods.

use super::shared::{animation_group_id_for_frame, opt_string, val_to_bool, val_to_f64};
use crate::lua_api::rilua_methods::{
    borrow_state, borrow_state_mut, create_string, frame_id_from_stack, frame_ref,
};
use crate::lua_bridge::{IntoStack, stack_val, table_set_rust_fn};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

fn normalize_mod_rate(r: f64) -> f64 {
    if r <= 0.0 { 1.0 } else { r }
}

fn apply_cooldown_state(
    frame: &mut crate::widget::Frame,
    start: f64,
    duration: f64,
    mod_rate: f64,
) {
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

pub(super) fn set_cooldown(state: &mut LuaState) -> LuaResult<u32> {
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

pub(super) fn set_cooldown_unix(state: &mut LuaState) -> LuaResult<u32> {
    set_cooldown(state)
}

pub(super) fn set_cooldown_from_expiration_time(state: &mut LuaState) -> LuaResult<u32> {
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

pub(super) fn set_cooldown_duration(state: &mut LuaState) -> LuaResult<u32> {
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

pub(super) fn get_cooldown_times(state: &mut LuaState) -> LuaResult<u32> {
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

pub(super) fn get_cooldown_duration(state: &mut LuaState) -> LuaResult<u32> {
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

pub(super) fn get_cooldown_display_duration(state: &mut LuaState) -> LuaResult<u32> {
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

pub(super) fn clear(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        clear_cooldown_timing(f);
    }
    Ok(0)
}

pub(super) fn pause(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(group_id) = animation_group_id_for_frame(&sim, id)
        && let Some(group) = sim.animation_groups.get_mut(&group_id)
    {
        group.paused = true;
        group.playing = false;
        return Ok(0);
    }
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.cooldown_paused = true;
    }
    Ok(0)
}

pub(super) fn resume(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(group_id) = animation_group_id_for_frame(&sim, id)
        && let Some(group) = sim.animation_groups.get_mut(&group_id)
    {
        group.paused = false;
        group.playing = true;
        return Ok(0);
    }
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.cooldown_paused = false;
    }
    Ok(0)
}

pub(super) fn is_paused(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    if let Some(group_id) = animation_group_id_for_frame(&sim, id) {
        let paused = sim
            .animation_groups
            .get(&group_id)
            .map(|group| group.paused)
            .unwrap_or(false);
        drop(sim);
        return paused.into_stack(state);
    }
    let v = sim
        .widgets
        .get(id)
        .map(|f| f.cooldown_paused)
        .unwrap_or(false);
    drop(sim);
    v.into_stack(state)
}

pub(super) fn set_draw_swipe(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let draw = val_to_bool(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.cooldown_draw_swipe = draw;
    }
    Ok(0)
}

pub(super) fn get_draw_swipe(state: &mut LuaState) -> LuaResult<u32> {
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

pub(super) fn set_draw_edge(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let draw = val_to_bool(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.cooldown_draw_edge = draw;
    }
    Ok(0)
}

pub(super) fn get_draw_edge(state: &mut LuaState) -> LuaResult<u32> {
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

pub(super) fn set_draw_bling(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let draw = val_to_bool(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.cooldown_draw_bling = draw;
    }
    Ok(0)
}

pub(super) fn get_draw_bling(state: &mut LuaState) -> LuaResult<u32> {
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

pub(super) fn set_reverse(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let rev = val_to_bool(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.cooldown_reverse = rev;
    }
    Ok(0)
}

pub(super) fn get_reverse(state: &mut LuaState) -> LuaResult<u32> {
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

pub(super) fn set_hide_countdown_numbers(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let hide = val_to_bool(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.cooldown_hide_countdown = hide;
    }
    Ok(0)
}

pub(super) fn get_hide_countdown_numbers(state: &mut LuaState) -> LuaResult<u32> {
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

pub(super) fn set_edge_scale(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let scale = val_to_f64(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.cooldown_edge_scale = scale;
    }
    Ok(0)
}

pub(super) fn get_edge_scale(state: &mut LuaState) -> LuaResult<u32> {
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

pub(super) fn set_minimum_countdown_duration(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let ms = val_to_f64(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.cooldown_min_countdown_duration_ms = ms;
    }
    Ok(0)
}

pub(super) fn get_minimum_countdown_duration(state: &mut LuaState) -> LuaResult<u32> {
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

pub(super) fn set_use_aura_display_time(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let enabled = val_to_bool(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.cooldown_use_aura_display_time = enabled;
    }
    Ok(0)
}

pub(super) fn get_use_aura_display_time(state: &mut LuaState) -> LuaResult<u32> {
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

pub(super) fn set_use_circular_edge(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let enabled = val_to_bool(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.cooldown_use_circular_edge = enabled;
    }
    Ok(0)
}

pub(super) fn set_countdown_abbrev_threshold(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let threshold = val_to_f64(stack_val(state, 2));
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.cooldown_countdown_abbrev_threshold_seconds = threshold;
    }
    Ok(0)
}

pub(super) fn set_swipe_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let path = opt_string(state, 2);
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.cooldown_swipe_texture = path;
    }
    Ok(0)
}

pub(super) fn set_bling_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let path = opt_string(state, 2);
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.cooldown_bling_texture = path;
    }
    Ok(0)
}

pub(super) fn set_countdown_font(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let Some(font_name) = opt_string(state, 2) else {
        return Ok(0);
    };
    let mut sim = borrow_state_mut(state)?;
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
    Ok(0)
}

pub(super) fn get_countdown_font_string(state: &mut LuaState) -> LuaResult<u32> {
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

pub(super) fn set_from_duration_object(_state: &mut LuaState) -> LuaResult<u32> {
    // TODO: parse duration object (Table/UserData with GetStartTime/GetTotalDuration/GetModRate/IsZero)
    Ok(0)
}

pub(super) fn set_edge_texture(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let path = opt_string(state, 2);
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.cooldown_edge_texture = path;
    }
    Ok(0)
}

pub(super) fn set_swipe_color(state: &mut LuaState) -> LuaResult<u32> {
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

pub(super) fn set_edge_color(state: &mut LuaState) -> LuaResult<u32> {
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

pub(super) fn set_tex_coord_range(_state: &mut LuaState) -> LuaResult<u32> {
    // TODO: parse two Vector2 args
    Ok(0)
}

// ---------------------------------------------------------------------------
// register_cooldown
// ---------------------------------------------------------------------------

pub(super) fn register_cooldown(state: &mut LuaState, metatable: GcRef<Table>) -> LuaResult<()> {
    table_set_rust_fn(state, metatable, "SetCooldown", set_cooldown)?;
    table_set_rust_fn(state, metatable, "SetCooldownUNIX", set_cooldown_unix)?;
    table_set_rust_fn(
        state,
        metatable,
        "SetCooldownFromExpirationTime",
        set_cooldown_from_expiration_time,
    )?;
    table_set_rust_fn(
        state,
        metatable,
        "SetCooldownDuration",
        set_cooldown_duration,
    )?;
    table_set_rust_fn(
        state,
        metatable,
        "SetCooldownFromDurationObject",
        set_from_duration_object,
    )?;
    table_set_rust_fn(state, metatable, "GetCooldownTimes", get_cooldown_times)?;
    table_set_rust_fn(
        state,
        metatable,
        "GetCooldownDuration",
        get_cooldown_duration,
    )?;
    table_set_rust_fn(
        state,
        metatable,
        "GetCooldownDisplayDuration",
        get_cooldown_display_duration,
    )?;
    table_set_rust_fn(state, metatable, "Clear", clear)?;
    table_set_rust_fn(state, metatable, "Pause", pause)?;
    table_set_rust_fn(state, metatable, "Resume", resume)?;
    table_set_rust_fn(state, metatable, "IsPaused", is_paused)?;
    table_set_rust_fn(state, metatable, "SetDrawSwipe", set_draw_swipe)?;
    table_set_rust_fn(state, metatable, "GetDrawSwipe", get_draw_swipe)?;
    table_set_rust_fn(state, metatable, "SetDrawEdge", set_draw_edge)?;
    table_set_rust_fn(state, metatable, "GetDrawEdge", get_draw_edge)?;
    table_set_rust_fn(state, metatable, "SetDrawBling", set_draw_bling)?;
    table_set_rust_fn(state, metatable, "GetDrawBling", get_draw_bling)?;
    table_set_rust_fn(state, metatable, "SetReverse", set_reverse)?;
    table_set_rust_fn(state, metatable, "GetReverse", get_reverse)?;
    table_set_rust_fn(
        state,
        metatable,
        "SetHideCountdownNumbers",
        set_hide_countdown_numbers,
    )?;
    table_set_rust_fn(
        state,
        metatable,
        "GetHideCountdownNumbers",
        get_hide_countdown_numbers,
    )?;
    table_set_rust_fn(state, metatable, "SetEdgeScale", set_edge_scale)?;
    table_set_rust_fn(state, metatable, "GetEdgeScale", get_edge_scale)?;
    table_set_rust_fn(
        state,
        metatable,
        "SetMinimumCountdownDuration",
        set_minimum_countdown_duration,
    )?;
    table_set_rust_fn(
        state,
        metatable,
        "GetMinimumCountdownDuration",
        get_minimum_countdown_duration,
    )?;
    table_set_rust_fn(
        state,
        metatable,
        "SetUseAuraDisplayTime",
        set_use_aura_display_time,
    )?;
    table_set_rust_fn(
        state,
        metatable,
        "GetUseAuraDisplayTime",
        get_use_aura_display_time,
    )?;
    table_set_rust_fn(
        state,
        metatable,
        "SetUseCircularEdge",
        set_use_circular_edge,
    )?;
    table_set_rust_fn(
        state,
        metatable,
        "SetCountdownAbbrevThreshold",
        set_countdown_abbrev_threshold,
    )?;
    table_set_rust_fn(state, metatable, "SetSwipeTexture", set_swipe_texture)?;
    table_set_rust_fn(state, metatable, "SetEdgeTexture", set_edge_texture)?;
    table_set_rust_fn(state, metatable, "SetBlingTexture", set_bling_texture)?;
    table_set_rust_fn(state, metatable, "SetCountdownFont", set_countdown_font)?;
    table_set_rust_fn(
        state,
        metatable,
        "GetCountdownFontString",
        get_countdown_font_string,
    )?;
    table_set_rust_fn(state, metatable, "SetSwipeColor", set_swipe_color)?;
    table_set_rust_fn(state, metatable, "SetEdgeColor", set_edge_color)?;
    table_set_rust_fn(state, metatable, "SetTexCoordRange", set_tex_coord_range)?;
    Ok(())
}
