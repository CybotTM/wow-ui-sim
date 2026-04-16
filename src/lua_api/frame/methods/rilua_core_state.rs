//! rilua RustFn equivalents of frame state methods.
//!
//! Covers the methods from `methods_core_state.rs`, `methods_visibility.rs`,
//! and `methods_core_region.rs`. Each function maps 1-to-1 with its mlua
//! counterpart using `frame_id_from_stack` + `borrow_state`/`borrow_state_mut`.

use crate::lua_api::rilua_methods::{
    borrow_state, borrow_state_mut, create_string, extract_frame_id, frame_ref,
};
use crate::lua_api::rilua_script_helpers::call_error_handler_state;
use crate::lua_api::rilua_script_helpers::get_script as get_rilua_script;
use crate::lua_bridge::{FromStack, stack_val, table_set_rust_fn};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val, runtime_error};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Extract a frame id from argument `index` (1-based).
fn frame_id(state: &LuaState, index: i32) -> LuaResult<u64> {
    crate::lua_api::rilua_methods::frame_id_from_stack(state, index)
}

/// Extract a bool from argument `index`. Nil → false, anything else → true.
fn arg_bool(state: &LuaState, index: i32) -> bool {
    bool::from_stack(state, index).unwrap_or(false)
}

/// Extract an f32 from argument `index`. Falls back to 0.0 on error.
/// Extract an optional f32 from argument `index`. Returns 0.0 for nil/absent.
fn opt_f32(state: &LuaState, index: i32) -> f32 {
    match stack_val(state, index) {
        Val::Num(n) => n as f32,
        _ => 0.0,
    }
}

fn has_queryable_rect(frame: &crate::widget::Frame, id: u64) -> bool {
    !frame.anchors.is_empty() || frame.name.as_deref() == Some("UIParent") || id == 1
}

fn raw_frame_size(state: &crate::lua_api::state::SimState, id: u64) -> (f32, f32) {
    state
        .widgets
        .get(id)
        .map(|frame| (frame.width, frame.height))
        .unwrap_or((0.0, 0.0))
}

fn resolved_frame_size(state: &crate::lua_api::state::SimState, id: u64) -> (f32, f32) {
    state
        .widgets
        .get(id)
        .map(|frame| {
            if let Some(rect) = frame.layout_rect {
                let eff_scale = frame.effective_scale.max(1e-6);
                (rect.width / eff_scale, rect.height / eff_scale)
            } else {
                (frame.width, frame.height)
            }
        })
        .unwrap_or((0.0, 0.0))
}

fn frame_size(state: &mut LuaState, id: u64, raw: bool) -> LuaResult<(f32, f32)> {
    let mut sim = borrow_state_mut(state)?;
    if !raw
        && sim
            .widgets
            .get(id)
            .is_some_and(|frame| has_queryable_rect(frame, id))
    {
        sim.resolve_rect_if_dirty(id);
    }
    let size = if raw {
        raw_frame_size(&sim, id)
    } else {
        resolved_frame_size(&sim, id)
    };
    Ok(size)
}

struct ExplicitSizeState {
    width: f32,
    height: f32,
    width_is_text_auto: bool,
}

fn current_explicit_size_state(
    state: &crate::lua_api::state::SimState,
    id: u64,
) -> Option<ExplicitSizeState> {
    state.widgets.get(id).map(|frame| ExplicitSizeState {
        width: frame.width,
        height: frame.height,
        width_is_text_auto: frame.width_is_text_auto,
    })
}

fn clear_auto_width_flag(state: &mut crate::lua_api::state::SimState, id: u64) {
    if let Some(frame) = state.widgets.get_mut(id) {
        frame.width_is_text_auto = false;
    }
}

fn apply_explicit_size(
    state: &mut crate::lua_api::state::SimState,
    id: u64,
    width: f32,
    height: f32,
) {
    if let Some(frame) = state.widgets.get_mut_visual(id) {
        frame.set_size(width, height);
        frame.width_is_text_auto = false;
    }
    state.widgets.mark_rect_dirty(id);
}

fn apply_explicit_width(state: &mut crate::lua_api::state::SimState, id: u64, width: f32) {
    if let Some(frame) = state.widgets.get_mut_visual(id) {
        frame.width = width;
        frame.width_is_text_auto = false;
    }
    state.widgets.mark_rect_dirty(id);
}

fn apply_explicit_height(state: &mut crate::lua_api::state::SimState, id: u64, height: f32) {
    if let Some(frame) = state.widgets.get_mut_visual(id) {
        frame.height = height;
    }
    state.widgets.mark_rect_dirty(id);
}

// ---------------------------------------------------------------------------
// Visibility: Show / Hide / SetShown
// ---------------------------------------------------------------------------

pub fn get_width(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let ignore = bool::from_stack(state, 2).ok().unwrap_or(false);
    let (width, _) = frame_size(state, id, ignore)?;
    state.push(Val::Num(width as f64));
    Ok(1)
}

pub fn get_height(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let ignore = bool::from_stack(state, 2).ok().unwrap_or(false);
    let (_, height) = frame_size(state, id, ignore)?;
    state.push(Val::Num(height as f64));
    Ok(1)
}

pub fn get_size(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let ignore = bool::from_stack(state, 2).ok().unwrap_or(false);
    let (width, height) = frame_size(state, id, ignore)?;
    state.push(Val::Num(width as f64));
    state.push(Val::Num(height as f64));
    Ok(2)
}

pub fn set_size(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let width = opt_f32(state, 2);
    let height = opt_f32(state, 3);
    let mut sim = borrow_state_mut(state)?;
    let Some(current) = current_explicit_size_state(&sim, id) else {
        return Ok(0);
    };

    let size_changed = current.width != width || current.height != height;
    if !size_changed {
        if current.width_is_text_auto {
            clear_auto_width_flag(&mut sim, id);
        }
        return Ok(0);
    }

    apply_explicit_size(&mut sim, id, width, height);
    Ok(0)
}

pub fn set_width(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let width = opt_f32(state, 2);
    let mut sim = borrow_state_mut(state)?;
    let Some(current) = current_explicit_size_state(&sim, id) else {
        return Ok(0);
    };

    if current.width == width {
        if current.width_is_text_auto {
            clear_auto_width_flag(&mut sim, id);
        }
        return Ok(0);
    }

    apply_explicit_width(&mut sim, id, width);
    Ok(0)
}

pub fn set_height(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let height = opt_f32(state, 2);
    let mut sim = borrow_state_mut(state)?;
    let Some(current_height) = sim.widgets.get(id).map(|frame| frame.height) else {
        return Ok(0);
    };

    if current_height == height {
        return Ok(0);
    }

    apply_explicit_height(&mut sim, id, height);
    Ok(0)
}

pub fn show(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let changed = set_frame_visible(state, id, true)?;
    if changed {
        fire_visibility_handler(state, id, "OnShow");
    }
    Ok(0)
}

pub fn hide(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let changed = set_frame_visible(state, id, false)?;
    if changed {
        fire_visibility_handler(state, id, "OnHide");
    }
    Ok(0)
}

pub fn set_shown(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let shown = arg_bool(state, 2);
    let changed = set_frame_visible(state, id, shown)?;
    if changed {
        let handler_name = if shown { "OnShow" } else { "OnHide" };
        fire_visibility_handler(state, id, handler_name);
    }
    Ok(0)
}

fn set_frame_visible(state: &mut LuaState, id: u64, shown: bool) -> LuaResult<bool> {
    let mut sim = borrow_state_mut(state)?;
    let was_visible = sim
        .widgets
        .get(id)
        .map(|frame| frame.visible)
        .unwrap_or(false);
    sim.set_frame_visible(id, shown);
    Ok(was_visible != shown)
}

fn fire_visibility_handler(state: &mut LuaState, frame_id: u64, handler_name: &str) {
    let Some(handler) = get_rilua_script(state, frame_id, handler_name) else {
        return;
    };
    let Ok(frame) = frame_ref(state, frame_id) else {
        return;
    };
    if let Err(error_msg) =
        crate::lua_api::rilua_script_helpers::protected_lua_pcall_state(state, handler, &[frame])
    {
        call_error_handler_state(state, &error_msg);
    }
}

// ---------------------------------------------------------------------------
// IsVisible / IsShown
// ---------------------------------------------------------------------------

pub fn is_visible(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let sim = borrow_state(state)?;
    let result = sim.widgets.is_ancestor_visible(id);
    drop(sim);
    state.push(Val::Bool(result));
    Ok(1)
}

pub fn is_shown(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let sim = borrow_state(state)?;
    let result = sim.widgets.get(id).map(|f| f.visible).unwrap_or(false);
    drop(sim);
    state.push(Val::Bool(result));
    Ok(1)
}

// ---------------------------------------------------------------------------
// CollapseLayout
// ---------------------------------------------------------------------------

pub fn set_collapses_layout(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let val = arg_bool(state, 2);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.collapses_layout = val;
    }
    Ok(0)
}

pub fn collapses_layout(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let sim = borrow_state(state)?;
    let result = sim
        .widgets
        .get(id)
        .map(|f| f.collapses_layout)
        .unwrap_or(false);
    drop(sim);
    state.push(Val::Bool(result));
    Ok(1)
}

pub fn is_collapsed(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let sim = borrow_state(state)?;
    let result = is_collapsed_impl(&sim, id);
    drop(sim);
    state.push(Val::Bool(result));
    Ok(1)
}

/// IsMenuOpen() — returns false (menus are never open in headless mode).
pub fn is_menu_open(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn is_collapsed_impl(state: &crate::lua_api::SimState, id: u64) -> bool {
    let frame = match state.widgets.get(id) {
        Some(f) => f,
        None => return false,
    };
    if !frame.collapses_layout {
        return false;
    }
    let mut visible = frame.visible;
    let mut cur_parent = frame.parent_id;
    while visible {
        match cur_parent.and_then(|pid| state.widgets.get(pid)) {
            Some(p) if p.visible => cur_parent = p.parent_id,
            Some(_) => {
                visible = false;
            }
            None => break,
        }
    }
    !visible
}

// ---------------------------------------------------------------------------
// Alpha
// ---------------------------------------------------------------------------

pub fn set_alpha(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let alpha = opt_f32(state, 2);
    let clamped = alpha.clamp(0.0, 1.0);
    let mut sim = borrow_state_mut(state)?;
    let changed = sim
        .widgets
        .get(id)
        .map(|f| f.alpha != clamped)
        .unwrap_or(false);
    if changed {
        let parent_eff = sim
            .widgets
            .get(id)
            .and_then(|f| f.parent_id)
            .and_then(|pid| sim.widgets.get(pid))
            .map(|p| p.effective_alpha)
            .unwrap_or(1.0);
        if let Some(frame) = sim.widgets.get_mut_visual(id) {
            frame.alpha = clamped;
        }
        sim.widgets.propagate_effective_alpha(id, parent_eff);
    }
    Ok(0)
}

pub fn get_alpha(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let sim = borrow_state(state)?;
    let result = sim.widgets.get(id).map(|f| f.alpha).unwrap_or(1.0);
    drop(sim);
    state.push(Val::Num(result as f64));
    Ok(1)
}

pub fn get_effective_alpha(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let sim = borrow_state(state)?;
    let result = sim
        .widgets
        .get(id)
        .map(|f| f.effective_alpha)
        .unwrap_or(1.0);
    drop(sim);
    state.push(Val::Num(result as f64));
    Ok(1)
}

pub fn set_alpha_from_boolean(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let flag = arg_bool(state, 2);
    let new_alpha: f32 = if flag { 1.0 } else { 0.0 };
    let mut sim = borrow_state_mut(state)?;
    let changed = sim
        .widgets
        .get(id)
        .map(|f| f.alpha != new_alpha)
        .unwrap_or(false);
    if !changed {
        return Ok(0);
    }
    let parent_eff = sim
        .widgets
        .get(id)
        .and_then(|f| f.parent_id)
        .and_then(|pid| sim.widgets.get(pid))
        .map(|p| p.effective_alpha)
        .unwrap_or(1.0);
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.alpha = new_alpha;
    }
    sim.widgets.propagate_effective_alpha(id, parent_eff);
    Ok(0)
}

pub fn set_ignore_parent_alpha(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let ignore = arg_bool(state, 2);
    let mut sim = borrow_state_mut(state)?;
    let parent_eff = sim
        .widgets
        .get(id)
        .and_then(|f| f.parent_id)
        .and_then(|pid| sim.widgets.get(pid))
        .map(|p| p.effective_alpha)
        .unwrap_or(1.0);
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.ignore_parent_alpha = ignore;
    }
    sim.widgets.propagate_effective_alpha(id, parent_eff);
    Ok(0)
}

pub fn get_ignore_parent_alpha(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let sim = borrow_state(state)?;
    let result = sim
        .widgets
        .get(id)
        .map(|f| f.ignore_parent_alpha)
        .unwrap_or(false);
    drop(sim);
    state.push(Val::Bool(result));
    Ok(1)
}

pub fn is_ignoring_parent_alpha(state: &mut LuaState) -> LuaResult<u32> {
    get_ignore_parent_alpha(state)
}

// ---------------------------------------------------------------------------
// Frame Strata
// ---------------------------------------------------------------------------

pub fn set_frame_strata(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let strata = String::from_stack(state, 2)?;
    let mut sim = borrow_state_mut(state)?;
    let Some(s) = crate::widget::FrameStrata::from_str(&strata) else {
        return Ok(0);
    };
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.frame_strata = s;
        frame.has_fixed_frame_strata = true;
    }
    let mut queue: Vec<u64> = sim
        .widgets
        .get(id)
        .map(|f| f.children.clone())
        .unwrap_or_default();
    while let Some(child_id) = queue.pop() {
        let Some(child) = sim.widgets.get_mut_visual(child_id) else {
            continue;
        };
        if child.has_fixed_frame_strata {
            continue;
        }
        child.frame_strata = s;
        queue.extend(child.children.iter().copied());
    }
    sim.invalidate_strata_buckets();
    Ok(0)
}

pub fn get_frame_strata(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let strata: &'static str = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|f| f.frame_strata.as_str())
            .unwrap_or("MEDIUM")
    };
    let val = create_string(state, strata);
    state.push(val);
    Ok(1)
}

pub fn set_fixed_frame_strata(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let fixed = arg_bool(state, 2);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.has_fixed_frame_strata = fixed;
    }
    Ok(0)
}

pub fn has_fixed_frame_strata(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let sim = borrow_state(state)?;
    let result = sim
        .widgets
        .get(id)
        .map(|f| f.has_fixed_frame_strata)
        .unwrap_or(false);
    drop(sim);
    state.push(Val::Bool(result));
    Ok(1)
}

// ---------------------------------------------------------------------------
// Frame Level
// ---------------------------------------------------------------------------

pub fn set_frame_level(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let level = i32::from_stack(state, 2)?;
    let mut sim = borrow_state_mut(state)?;
    let Some(current_level) = sim.widgets.get(id).map(|frame| frame.frame_level) else {
        return Ok(0);
    };
    if current_level == level {
        return Ok(0);
    }
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.frame_level = level;
    }
    super::methods_hierarchy::propagate_strata_level_pub(&mut sim.widgets, id);
    sim.invalidate_strata_buckets();
    Ok(0)
}

pub fn get_frame_level(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let sim = borrow_state(state)?;
    let result = sim.widgets.get(id).map(|f| f.frame_level).unwrap_or(0);
    drop(sim);
    state.push(Val::Num(result as f64));
    Ok(1)
}

pub fn set_fixed_frame_level(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let fixed = arg_bool(state, 2);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.has_fixed_frame_level = fixed;
    }
    Ok(0)
}

pub fn has_fixed_frame_level(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let sim = borrow_state(state)?;
    let result = sim
        .widgets
        .get(id)
        .map(|f| f.has_fixed_frame_level)
        .unwrap_or(false);
    drop(sim);
    state.push(Val::Bool(result));
    Ok(1)
}

// ---------------------------------------------------------------------------
// Toplevel
// ---------------------------------------------------------------------------

pub fn set_toplevel(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let toplevel = arg_bool(state, 2);
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut(id) {
        f.toplevel = toplevel;
    }
    Ok(0)
}

pub fn is_toplevel(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let sim = borrow_state(state)?;
    let result = sim.widgets.get(id).map(|f| f.toplevel).unwrap_or(false);
    drop(sim);
    state.push(Val::Bool(result));
    Ok(1)
}

// ---------------------------------------------------------------------------
// ID / MapID
// ---------------------------------------------------------------------------

pub fn set_id(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let user_id = i32::from_stack(state, 2)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut(id) {
        f.user_id = user_id;
    }
    Ok(0)
}

pub fn get_id(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let sim = borrow_state(state)?;
    let result = sim.widgets.get(id).map(|f| f.user_id).unwrap_or(0);
    drop(sim);
    state.push(Val::Num(result as f64));
    Ok(1)
}

pub fn get_map_id(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let sim = borrow_state(state)?;
    let result = sim
        .quest_blobs
        .get(&id)
        .map(|b| b.map_id as i32)
        .unwrap_or(0);
    drop(sim);
    state.push(Val::Num(result as f64));
    Ok(1)
}

pub fn get_ui_map_id(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let sim = borrow_state(state)?;
    let result = sim
        .fog_of_war_frames
        .get(&id)
        .and_then(|fog| fog.ui_map_id)
        .or_else(|| {
            sim.unit_position_frames
                .get(&id)
                .and_then(|unit_state| unit_state.ui_map_id)
        })
        .unwrap_or_else(|| {
            sim.quest_blobs
                .get(&id)
                .map(|b| b.map_id as i32)
                .unwrap_or(0)
        });
    drop(sim);
    state.push(Val::Num(result as f64));
    Ok(1)
}

pub fn set_map_id(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let map_id = i32::from_stack(state, 2)?;
    let mut sim = borrow_state_mut(state)?;
    let is_fog = sim
        .widgets
        .get(id)
        .and_then(|frame| frame.object_type_name.as_deref())
        .is_some_and(|name| name.eq_ignore_ascii_case("FogOfWarFrame"));
    if is_fog {
        sim.fog_of_war_frames.entry(id).or_default().ui_map_id = Some(map_id);
        if let Some(frame) = sim.widgets.get_mut_visual(id) {
            frame.fog_of_war_ui_map_id = Some(map_id);
        }
        return Ok(0);
    }
    if sim.widgets.get(id).is_some_and(|frame| {
        frame
            .object_type_name
            .as_deref()
            .is_some_and(|name| name.eq_ignore_ascii_case("UnitPositionFrame"))
    }) {
        sim.unit_position_frames
            .entry(id)
            .or_insert_with(|| crate::lua_api::state::UnitPositionFrameState {
                ui_map_id: None,
                units: Vec::new(),
                unit_colors: std::collections::HashMap::new(),
                mouse_over_units: Vec::new(),
                player_ping_scale: 1.0,
                player_ping_textures: std::collections::HashMap::new(),
                player_ping_active: false,
                player_ping_duration: None,
                player_ping_fade_duration: None,
                is_finalized: false,
            })
            .ui_map_id = Some(map_id);
        return Ok(0);
    }
    let blob = sim.quest_blobs.entry(id).or_default();
    blob.map_id = map_id as u32;
    Ok(0)
}

// ---------------------------------------------------------------------------
// Mouse / keyboard enable
// ---------------------------------------------------------------------------

pub fn enable_mouse(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let enable = arg_bool(state, 2);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut(id) {
        frame.mouse_enabled = enable;
    }
    Ok(0)
}

pub fn is_mouse_enabled(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let sim = borrow_state(state)?;
    let result = sim
        .widgets
        .get(id)
        .map(|f| f.mouse_enabled)
        .unwrap_or(false);
    drop(sim);
    state.push(Val::Bool(result));
    Ok(1)
}

pub fn enable_mouse_wheel(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let enable = arg_bool(state, 2);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut(id) {
        frame.mouse_wheel_enabled = enable;
    }
    Ok(0)
}

pub fn is_mouse_wheel_enabled(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let sim = borrow_state(state)?;
    let result = sim
        .widgets
        .get(id)
        .map(|f| f.mouse_wheel_enabled)
        .unwrap_or(false);
    drop(sim);
    state.push(Val::Bool(result));
    Ok(1)
}

pub fn enable_keyboard(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let enable = arg_bool(state, 2);
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut(id) {
        f.keyboard_enabled = enable;
    }
    Ok(0)
}

pub fn is_keyboard_enabled(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let sim = borrow_state(state)?;
    let result = sim
        .widgets
        .get(id)
        .map(|f| f.keyboard_enabled)
        .unwrap_or(false);
    drop(sim);
    state.push(Val::Bool(result));
    Ok(1)
}

pub fn register_for_mouse(state: &mut LuaState) -> LuaResult<u32> {
    let _id = frame_id(state, 1)?;
    // Variadic args ignored — stub only.
    Ok(0)
}

// ---------------------------------------------------------------------------
// Mouse motion
// ---------------------------------------------------------------------------

pub fn enable_mouse_motion(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let enable = arg_bool(state, 2);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut(id) {
        frame.mouse_motion_enabled = enable;
    }
    Ok(0)
}

pub fn is_mouse_motion_enabled(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let sim = borrow_state(state)?;
    let result = sim
        .widgets
        .get(id)
        .map(|f| f.mouse_motion_enabled)
        .unwrap_or(false);
    drop(sim);
    state.push(Val::Bool(result));
    Ok(1)
}

pub fn set_mouse_motion_enabled(state: &mut LuaState) -> LuaResult<u32> {
    enable_mouse_motion(state)
}

pub fn set_mouse_click_enabled(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let enable = arg_bool(state, 2);
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut(id) {
        frame.mouse_enabled = enable;
    }
    Ok(0)
}

pub fn is_mouse_click_enabled(state: &mut LuaState) -> LuaResult<u32> {
    is_mouse_enabled(state)
}

// ---------------------------------------------------------------------------
// Scale
// ---------------------------------------------------------------------------

pub fn get_scale(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let sim = borrow_state(state)?;
    let result = sim.widgets.get(id).map(|f| f.scale).unwrap_or(1.0);
    drop(sim);
    state.push(Val::Num(result as f64));
    Ok(1)
}

pub fn get_effective_scale(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let sim = borrow_state(state)?;
    let result = sim
        .widgets
        .get(id)
        .map(|f| f.effective_scale)
        .unwrap_or(1.0);
    drop(sim);
    state.push(Val::Num(result as f64));
    Ok(1)
}

pub fn set_scale(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let scale = match stack_val(state, 2) {
        Val::Num(n) => n as f32,
        _ => return Ok(0),
    };
    if scale <= 0.0 {
        return Err(runtime_error("Frame:SetScale(): Scale must be > 0"));
    }
    let mut sim = borrow_state_mut(state)?;
    let changed = sim
        .widgets
        .get(id)
        .map(|f| f.scale != scale)
        .unwrap_or(false);
    if !changed {
        return Ok(0);
    }
    let parent_eff_scale = sim
        .widgets
        .get(id)
        .and_then(|f| f.parent_id)
        .and_then(|pid| sim.widgets.get(pid))
        .map(|p| p.effective_scale)
        .unwrap_or(1.0);
    if let Some(f) = sim.widgets.get_mut_visual(id) {
        f.scale = scale;
    }
    sim.widgets.propagate_effective_scale(id, parent_eff_scale);
    sim.widgets.mark_rect_dirty(id);
    Ok(0)
}

pub fn set_ignore_parent_scale(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let ignore = arg_bool(state, 2);
    let mut sim = borrow_state_mut(state)?;
    let changed = sim
        .widgets
        .get(id)
        .map(|f| f.ignore_parent_scale != ignore)
        .unwrap_or(false);
    if !changed {
        return Ok(0);
    }
    let parent_eff_scale = sim
        .widgets
        .get(id)
        .and_then(|f| f.parent_id)
        .and_then(|pid| sim.widgets.get(pid))
        .map(|p| p.effective_scale)
        .unwrap_or(1.0);
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.ignore_parent_scale = ignore;
    }
    sim.widgets.propagate_effective_scale(id, parent_eff_scale);
    sim.widgets.mark_rect_dirty(id);
    Ok(0)
}

pub fn get_ignore_parent_scale(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let sim = borrow_state(state)?;
    let result = sim
        .widgets
        .get(id)
        .map(|f| f.ignore_parent_scale)
        .unwrap_or(false);
    drop(sim);
    state.push(Val::Bool(result));
    Ok(1)
}

pub fn is_ignoring_parent_scale(state: &mut LuaState) -> LuaResult<u32> {
    get_ignore_parent_scale(state)
}

// ---------------------------------------------------------------------------
// Region queries (methods_core_region.rs)
// ---------------------------------------------------------------------------

pub fn is_rect_valid(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let sim = borrow_state(state)?;
    let has_anchors = sim
        .widgets
        .get(id)
        .map(|f| !f.anchors.is_empty())
        .unwrap_or(false);
    let result = if !has_anchors {
        false
    } else {
        !sim.widgets.is_rect_dirty(id)
    };
    drop(sim);
    state.push(Val::Bool(result));
    Ok(1)
}

pub fn is_mouse_motion_focus(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let sim = borrow_state(state)?;
    let result = sim.hovered_frame == Some(id);
    drop(sim);
    state.push(Val::Bool(result));
    Ok(1)
}

pub fn is_object_loaded(state: &mut LuaState) -> LuaResult<u32> {
    let _id = frame_id(state, 1)?;
    state.push(Val::Bool(true));
    Ok(1)
}

pub fn is_mouse_over(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let left = opt_f32(state, 2);
    let right = opt_f32(state, 3);
    let top = opt_f32(state, 4);
    let bottom = opt_f32(state, 5);
    {
        let needs_resolve = borrow_state(state)?.widgets.is_rect_dirty(id);
        if needs_resolve {
            borrow_state_mut(state)?.resolve_rect_if_dirty(id);
        }
    }
    let sim = borrow_state(state)?;
    let result = is_mouse_over_bounds(&sim, id, left, right, top, bottom);
    drop(sim);
    state.push(Val::Bool(result));
    Ok(1)
}

fn is_mouse_over_bounds(
    state: &crate::lua_api::SimState,
    id: u64,
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
) -> bool {
    let Some((mouse_x, mouse_y)) = state.mouse_position else {
        return false;
    };
    let Some(frame) = state.widgets.get(id) else {
        return false;
    };
    if !frame.visible || frame.effective_alpha <= 0.0 || !frame.mouse_enabled {
        return false;
    }
    let Some(rect) = frame.layout_rect else {
        return false;
    };
    mouse_x >= rect.x - left
        && mouse_x <= rect.x + rect.width + right
        && mouse_y >= rect.y - top
        && mouse_y <= rect.y + rect.height + bottom
}

pub fn stop_animating(state: &mut LuaState) -> LuaResult<u32> {
    let _id = frame_id(state, 1)?;
    Ok(0)
}

pub fn get_source_location(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let sim = borrow_state(state)?;
    let Some(frame) = sim.widgets.get(id) else {
        drop(sim);
        state.push(Val::Nil);
        return Ok(1);
    };
    let owner_addon = frame.owner_addon;
    let location = source_location_for_owner(&sim, owner_addon);
    drop(sim);
    match location {
        Some(loc) => {
            let val = create_string(state, &loc);
            state.push(val);
        }
        None => {
            state.push(Val::Nil);
        }
    }
    Ok(1)
}

fn source_location_for_owner(
    state: &crate::lua_api::state::SimState,
    owner_addon: Option<u16>,
) -> Option<String> {
    let addon = owner_addon.and_then(|idx| state.addons.get(idx as usize))?;
    let folder = addon.folder_name.as_str();
    if folder == "__BuiltIn" {
        return Some("Interface/FrameXML".to_string());
    }
    Some(format!("Interface/AddOns/{folder}"))
}

pub fn intersects(state: &mut LuaState) -> LuaResult<u32> {
    let this_id = frame_id(state, 1)?;
    let other_val = stack_val(state, 2);
    let Some(other_id) = extract_frame_id(state, other_val) else {
        state.push(Val::Bool(false));
        return Ok(1);
    };
    {
        let mut sim = borrow_state_mut(state)?;
        sim.resolve_rect_if_dirty(this_id);
        sim.resolve_rect_if_dirty(other_id);
    }
    let sim = borrow_state(state)?;
    let Some(this_rect) = sim.widgets.get(this_id).and_then(|f| f.layout_rect) else {
        drop(sim);
        state.push(Val::Bool(false));
        return Ok(1);
    };
    let Some(other_rect) = sim.widgets.get(other_id).and_then(|f| f.layout_rect) else {
        drop(sim);
        state.push(Val::Bool(false));
        return Ok(1);
    };
    let result = layout_rects_intersect(this_rect, other_rect);
    drop(sim);
    state.push(Val::Bool(result));
    Ok(1)
}

fn layout_rects_intersect(a: crate::LayoutRect, b: crate::LayoutRect) -> bool {
    let left = a.x.max(b.x);
    let top = a.y.max(b.y);
    let right = (a.x + a.width).min(b.x + b.width);
    let bottom = (a.y + a.height).min(b.y + b.height);
    right > left && bottom > top
}

pub fn is_draw_layer_enabled(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let layer_name = String::from_stack(state, 2)?;
    let Some(layer) = crate::widget::DrawLayer::from_str(&layer_name) else {
        state.push(Val::Bool(false));
        return Ok(1);
    };
    let sim = borrow_state(state)?;
    let result = sim
        .widgets
        .get(id)
        .map(|frame| frame.is_draw_layer_enabled(layer))
        .unwrap_or(false);
    drop(sim);
    state.push(Val::Bool(result));
    Ok(1)
}

pub fn set_draw_layer_enabled(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let layer_name = String::from_stack(state, 2)?;
    let enabled = arg_bool(state, 3);
    let Some(layer) = crate::widget::DrawLayer::from_str(&layer_name) else {
        return Ok(0);
    };
    let mut sim = borrow_state_mut(state)?;
    if let Some(frame) = sim.widgets.get_mut_visual(id) {
        frame.set_draw_layer_enabled(layer, enabled);
    }
    Ok(0)
}

pub fn get_name(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let name = {
        let sim = borrow_state(state)?;
        sim.widgets.get(id).and_then(|frame| frame.name.clone())
    };
    let name_val = match name {
        Some(name) => create_string(state, &name),
        None => Val::Nil,
    };
    state.push(name_val);
    Ok(1)
}

pub fn get_debug_name(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let debug_name = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|frame| {
                frame
                    .name
                    .clone()
                    .unwrap_or_else(|| format!("{}:{id}", frame.widget_type.as_str()))
            })
            .unwrap_or_else(|| format!("Frame:{id}"))
    };
    let debug_name_val = create_string(state, &debug_name);
    state.push(debug_name_val);
    Ok(1)
}

pub fn get_object_type(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let object_type = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|frame| {
                if matches!(frame.widget_type, crate::widget::WidgetType::WorldFrame) {
                    return "Frame".to_string();
                }
                frame
                    .object_type_name
                    .clone()
                    .unwrap_or_else(|| frame.widget_type.as_str().to_string())
            })
            .unwrap_or_else(|| "Frame".to_string())
    };
    let object_type_val = create_string(state, &object_type);
    state.push(object_type_val);
    Ok(1)
}

pub fn is_object_type(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id(state, 1)?;
    let requested = String::from_stack(state, 2)?;
    let result = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(id)
            .map(|frame| {
                if matches!(frame.widget_type, crate::widget::WidgetType::WorldFrame) {
                    return requested.eq_ignore_ascii_case("WorldFrame")
                        || requested.eq_ignore_ascii_case("Region");
                }
                let actual = frame
                    .object_type_name
                    .as_deref()
                    .unwrap_or(frame.widget_type.as_str());
                actual.eq_ignore_ascii_case(&requested)
                    || frame.widget_type.as_str().eq_ignore_ascii_case(&requested)
            })
            .unwrap_or(false)
    };
    state.push(Val::Bool(result));
    Ok(1)
}

// ---------------------------------------------------------------------------
// Registration
// ---------------------------------------------------------------------------

pub fn register_all(state: &mut LuaState, mt: GcRef<Table>) -> LuaResult<()> {
    // Size
    table_set_rust_fn(state, mt, "GetWidth", get_width)?;
    table_set_rust_fn(state, mt, "GetHeight", get_height)?;
    table_set_rust_fn(state, mt, "GetSize", get_size)?;
    table_set_rust_fn(state, mt, "SetSize", set_size)?;
    table_set_rust_fn(state, mt, "SetWidth", set_width)?;
    table_set_rust_fn(state, mt, "SetHeight", set_height)?;
    // Visibility
    table_set_rust_fn(state, mt, "Show", show)?;
    table_set_rust_fn(state, mt, "Hide", hide)?;
    table_set_rust_fn(state, mt, "SetShown", set_shown)?;
    table_set_rust_fn(state, mt, "IsVisible", is_visible)?;
    table_set_rust_fn(state, mt, "IsShown", is_shown)?;
    // CollapseLayout
    table_set_rust_fn(state, mt, "SetCollapsesLayout", set_collapses_layout)?;
    table_set_rust_fn(state, mt, "CollapsesLayout", collapses_layout)?;
    table_set_rust_fn(state, mt, "IsCollapsed", is_collapsed)?;
    // Dropdown menus (always closed in headless mode)
    table_set_rust_fn(state, mt, "IsMenuOpen", is_menu_open)?;
    // Alpha
    table_set_rust_fn(state, mt, "SetAlpha", set_alpha)?;
    table_set_rust_fn(state, mt, "GetAlpha", get_alpha)?;
    table_set_rust_fn(state, mt, "GetEffectiveAlpha", get_effective_alpha)?;
    table_set_rust_fn(state, mt, "SetAlphaFromBoolean", set_alpha_from_boolean)?;
    table_set_rust_fn(state, mt, "SetIgnoreParentAlpha", set_ignore_parent_alpha)?;
    table_set_rust_fn(state, mt, "GetIgnoreParentAlpha", get_ignore_parent_alpha)?;
    table_set_rust_fn(state, mt, "IsIgnoringParentAlpha", is_ignoring_parent_alpha)?;
    // Frame strata
    table_set_rust_fn(state, mt, "SetFrameStrata", set_frame_strata)?;
    table_set_rust_fn(state, mt, "GetFrameStrata", get_frame_strata)?;
    table_set_rust_fn(state, mt, "SetFixedFrameStrata", set_fixed_frame_strata)?;
    table_set_rust_fn(state, mt, "HasFixedFrameStrata", has_fixed_frame_strata)?;
    // Frame level
    table_set_rust_fn(state, mt, "SetFrameLevel", set_frame_level)?;
    table_set_rust_fn(state, mt, "GetFrameLevel", get_frame_level)?;
    table_set_rust_fn(state, mt, "SetFixedFrameLevel", set_fixed_frame_level)?;
    table_set_rust_fn(state, mt, "HasFixedFrameLevel", has_fixed_frame_level)?;
    // Identity
    table_set_rust_fn(state, mt, "GetName", get_name)?;
    table_set_rust_fn(state, mt, "GetDebugName", get_debug_name)?;
    table_set_rust_fn(state, mt, "GetObjectType", get_object_type)?;
    table_set_rust_fn(state, mt, "IsObjectType", is_object_type)?;
    // Toplevel
    table_set_rust_fn(state, mt, "SetToplevel", set_toplevel)?;
    table_set_rust_fn(state, mt, "IsToplevel", is_toplevel)?;
    // ID / MapID
    table_set_rust_fn(state, mt, "SetID", set_id)?;
    table_set_rust_fn(state, mt, "GetID", get_id)?;
    table_set_rust_fn(state, mt, "GetMapID", get_map_id)?;
    table_set_rust_fn(state, mt, "GetUiMapID", get_ui_map_id)?;
    table_set_rust_fn(state, mt, "SetMapID", set_map_id)?;
    table_set_rust_fn(state, mt, "SetUiMapID", set_map_id)?;
    // Mouse / keyboard
    table_set_rust_fn(state, mt, "EnableMouse", enable_mouse)?;
    table_set_rust_fn(state, mt, "IsMouseEnabled", is_mouse_enabled)?;
    table_set_rust_fn(state, mt, "EnableMouseWheel", enable_mouse_wheel)?;
    table_set_rust_fn(state, mt, "IsMouseWheelEnabled", is_mouse_wheel_enabled)?;
    table_set_rust_fn(state, mt, "EnableKeyboard", enable_keyboard)?;
    table_set_rust_fn(state, mt, "IsKeyboardEnabled", is_keyboard_enabled)?;
    table_set_rust_fn(state, mt, "RegisterForMouse", register_for_mouse)?;
    table_set_rust_fn(state, mt, "EnableMouseMotion", enable_mouse_motion)?;
    table_set_rust_fn(state, mt, "IsMouseMotionEnabled", is_mouse_motion_enabled)?;
    table_set_rust_fn(state, mt, "SetMouseMotionEnabled", set_mouse_motion_enabled)?;
    table_set_rust_fn(state, mt, "SetMouseClickEnabled", set_mouse_click_enabled)?;
    table_set_rust_fn(state, mt, "IsMouseClickEnabled", is_mouse_click_enabled)?;
    // Scale
    table_set_rust_fn(state, mt, "GetScale", get_scale)?;
    table_set_rust_fn(state, mt, "GetEffectiveScale", get_effective_scale)?;
    table_set_rust_fn(state, mt, "SetScale", set_scale)?;
    table_set_rust_fn(state, mt, "SetIgnoreParentScale", set_ignore_parent_scale)?;
    table_set_rust_fn(state, mt, "GetIgnoreParentScale", get_ignore_parent_scale)?;
    table_set_rust_fn(state, mt, "IsIgnoringParentScale", is_ignoring_parent_scale)?;
    // Region queries
    table_set_rust_fn(state, mt, "IsRectValid", is_rect_valid)?;
    table_set_rust_fn(state, mt, "IsMouseMotionFocus", is_mouse_motion_focus)?;
    table_set_rust_fn(state, mt, "IsObjectLoaded", is_object_loaded)?;
    table_set_rust_fn(state, mt, "IsMouseOver", is_mouse_over)?;
    table_set_rust_fn(state, mt, "StopAnimating", stop_animating)?;
    table_set_rust_fn(state, mt, "GetSourceLocation", get_source_location)?;
    table_set_rust_fn(state, mt, "Intersects", intersects)?;
    table_set_rust_fn(state, mt, "IsDrawLayerEnabled", is_draw_layer_enabled)?;
    table_set_rust_fn(state, mt, "SetDrawLayerEnabled", set_draw_layer_enabled)?;
    Ok(())
}
