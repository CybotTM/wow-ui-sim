//! Animation group and animation creation/control methods.

use crate::lua_api::methods::{borrow_state, borrow_state_mut, frame_id_from_stack, frame_ref};
use crate::lua_bridge::{FromStack, stack_val};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val, runtime_error};

use super::font_strings::resolve_child_name;
use super::shared::bind_named_child_global;

// ── Resolve helpers ───────────────────────────────────────────────────────────

pub(super) fn resolve_animation_group_id(
    sim: &crate::lua_api::SimState,
    frame_id: u64,
) -> Option<u64> {
    sim.anim_frame_to_group.get(&frame_id).copied().or_else(|| {
        sim.anim_frame_to_anim
            .get(&frame_id)
            .map(|(group_id, _)| *group_id)
    })
}

pub(super) fn resolve_anim_target_id(
    sim: &crate::lua_api::SimState,
    owner_id: u64,
    child_key: Option<&str>,
) -> Option<u64> {
    let Some(key) = child_key else {
        return Some(owner_id);
    };
    sim.widgets.get(owner_id).and_then(|owner| {
        owner
            .children_keys
            .get(key)
            .copied()
            .or_else(|| find_child_by_key(sim, owner, key))
    })
}

fn find_child_by_key(
    sim: &crate::lua_api::SimState,
    owner: &crate::widget::Frame,
    key: &str,
) -> Option<u64> {
    owner.children.iter().copied().find(|child_id| {
        sim.widgets.get(*child_id).is_some_and(|child| {
            child.parent_key.as_deref() == Some(key) || child.name.as_deref() == Some(key)
        })
    })
}

// ── Animation group queries ───────────────────────────────────────────────────

pub(super) fn get_animation_groups(state: &mut LuaState) -> LuaResult<u32> {
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

pub(super) fn get_animations(state: &mut LuaState) -> LuaResult<u32> {
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

pub(super) fn get_animation_target(state: &mut LuaState) -> LuaResult<u32> {
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

pub(super) fn get_region_parent(state: &mut LuaState) -> LuaResult<u32> {
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

// ── Animation group control ───────────────────────────────────────────────────

pub(super) fn animation_group_play(state: &mut LuaState) -> LuaResult<u32> {
    let group_frame_id = frame_id_from_stack(state, 1)?;
    let reverse = !matches!(stack_val(state, 2), Val::Nil | Val::Bool(false));
    let mut sim = borrow_state_mut(state)?;
    if let Some(group_id) = resolve_animation_group_id(&sim, group_frame_id) {
        if let Some(group) = sim.animation_groups.get_mut(&group_id) {
            group.playing = true;
            group.paused = false;
            group.done = false;
            group.pending_finish = false;
            group.reverse = reverse;
        }
        apply_group_flipbook_state(&mut sim, group_id);
        sync_action_bar_busy_for_group(&mut sim, group_id);
    }
    Ok(0)
}

pub(super) fn animation_group_pause(state: &mut LuaState) -> LuaResult<u32> {
    let group_frame_id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(group_id) = resolve_animation_group_id(&sim, group_frame_id) {
        if let Some(group) = sim.animation_groups.get_mut(&group_id)
            && group.playing
        {
            group.playing = false;
            group.paused = true;
        }
        sync_action_bar_busy_for_group(&mut sim, group_id);
    }
    Ok(0)
}

pub(super) fn animation_group_stop(state: &mut LuaState) -> LuaResult<u32> {
    let group_frame_id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(group_id) = resolve_animation_group_id(&sim, group_frame_id) {
        if let Some(group) = sim.animation_groups.get_mut(&group_id) {
            group.playing = false;
            group.paused = false;
            group.done = true;
            group.pending_finish = false;
            group.elapsed = 0.0;
            for animation in &mut group.animations {
                animation.elapsed = 0.0;
            }
        }
        apply_group_flipbook_state(&mut sim, group_id);
        sync_action_bar_busy_for_group(&mut sim, group_id);
    }
    Ok(0)
}

pub(super) fn animation_group_is_playing(state: &mut LuaState) -> LuaResult<u32> {
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

pub(super) fn animation_group_is_paused(state: &mut LuaState) -> LuaResult<u32> {
    let group_frame_id = frame_id_from_stack(state, 1)?;
    let paused = {
        let sim = borrow_state(state)?;
        resolve_animation_group_id(&sim, group_frame_id)
            .and_then(|group_id| {
                sim.animation_groups
                    .get(&group_id)
                    .map(|group| group.paused)
            })
            .unwrap_or(false)
    };
    state.push(Val::Bool(paused));
    Ok(1)
}

pub(super) fn animation_group_set_playing(state: &mut LuaState) -> LuaResult<u32> {
    let playing = match stack_val(state, 2) {
        Val::Bool(value) => value,
        Val::Nil => false,
        _ => true,
    };
    if playing {
        animation_group_play(state)
    } else {
        animation_group_stop(state)
    }
}

pub(super) fn animation_group_restart(state: &mut LuaState) -> LuaResult<u32> {
    let frame_id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(group_id) = resolve_animation_group_id(&sim, frame_id) {
        if let Some(group) = sim.animation_groups.get_mut(&group_id) {
            group.playing = true;
            group.paused = false;
            group.done = false;
            group.pending_finish = false;
            group.elapsed = 0.0;
            for animation in &mut group.animations {
                animation.elapsed = 0.0;
            }
        }
        apply_group_flipbook_state(&mut sim, group_id);
    }
    Ok(0)
}

pub(super) fn animation_group_finish(state: &mut LuaState) -> LuaResult<u32> {
    let frame_id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(group_id) = resolve_animation_group_id(&sim, frame_id)
        && let Some(group) = sim.animation_groups.get_mut(&group_id)
    {
        group.playing = true;
        group.paused = false;
        group.pending_finish = true;
    }
    Ok(0)
}

pub(super) fn animation_group_is_done(state: &mut LuaState) -> LuaResult<u32> {
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

pub(super) fn animation_group_get_duration(state: &mut LuaState) -> LuaResult<u32> {
    let frame_id = frame_id_from_stack(state, 1)?;
    let duration = {
        let sim = borrow_state(state)?;
        resolve_animation_group_id(&sim, frame_id)
            .and_then(|group_id| sim.animation_groups.get(&group_id))
            .map(animation_group_total_duration)
            .unwrap_or(0.0)
    };
    state.push(Val::Num(duration));
    Ok(1)
}

pub(super) fn animation_group_set_looping(state: &mut LuaState) -> LuaResult<u32> {
    use super::shared::opt_string;
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

pub(super) fn animation_group_get_looping(state: &mut LuaState) -> LuaResult<u32> {
    let group_frame_id = frame_id_from_stack(state, 1)?;
    let looping = {
        let sim = borrow_state(state)?;
        resolve_animation_group_id(&sim, group_frame_id)
            .and_then(|group_id| sim.animation_groups.get(&group_id))
            .map(group_loop_name)
            .unwrap_or("NONE")
    };
    let value = crate::lua_api::methods::create_string_static(state, looping);
    state.push(value);
    Ok(1)
}

pub(super) fn animation_group_get_loop_state(state: &mut LuaState) -> LuaResult<u32> {
    animation_group_get_looping(state)
}

pub(super) fn animation_group_set_animation_speed_multiplier(
    state: &mut LuaState,
) -> LuaResult<u32> {
    let group_frame_id = frame_id_from_stack(state, 1)?;
    let speed = match stack_val(state, 2) {
        Val::Num(value) if value.is_finite() => value,
        _ => 1.0,
    };
    let mut sim = borrow_state_mut(state)?;
    if let Some(group_id) = sim.anim_frame_to_group.get(&group_frame_id).copied()
        && let Some(group) = sim.animation_groups.get_mut(&group_id)
    {
        group.speed_multiplier = speed.max(0.0);
    }
    Ok(0)
}

pub(super) fn animation_group_get_animation_speed_multiplier(
    state: &mut LuaState,
) -> LuaResult<u32> {
    let group_frame_id = frame_id_from_stack(state, 1)?;
    let speed = {
        let sim = borrow_state(state)?;
        resolve_animation_group_id(&sim, group_frame_id)
            .and_then(|group_id| sim.animation_groups.get(&group_id))
            .map(|group| group.speed_multiplier)
            .unwrap_or(1.0)
    };
    state.push(Val::Num(speed));
    Ok(1)
}

pub(super) fn animation_group_set_to_final_alpha(state: &mut LuaState) -> LuaResult<u32> {
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

pub(super) fn animation_group_is_set_to_final_alpha(state: &mut LuaState) -> LuaResult<u32> {
    let group_frame_id = frame_id_from_stack(state, 1)?;
    let value = {
        let sim = borrow_state(state)?;
        resolve_animation_group_id(&sim, group_frame_id)
            .and_then(|group_id| sim.animation_groups.get(&group_id))
            .is_some_and(|group| group.set_to_final_alpha)
    };
    push_group_bool(state, value)
}

pub(super) fn animation_group_get_to_final_alpha(state: &mut LuaState) -> LuaResult<u32> {
    animation_group_is_set_to_final_alpha(state)
}

pub(super) fn animation_group_is_pending_finish(state: &mut LuaState) -> LuaResult<u32> {
    let group_frame_id = frame_id_from_stack(state, 1)?;
    let value = {
        let sim = borrow_state(state)?;
        resolve_animation_group_id(&sim, group_frame_id)
            .and_then(|group_id| sim.animation_groups.get(&group_id))
            .is_some_and(|group| group.pending_finish)
    };
    push_group_bool(state, value)
}

pub(super) fn animation_group_is_reverse(state: &mut LuaState) -> LuaResult<u32> {
    let group_frame_id = frame_id_from_stack(state, 1)?;
    let value = {
        let sim = borrow_state(state)?;
        resolve_animation_group_id(&sim, group_frame_id)
            .and_then(|group_id| sim.animation_groups.get(&group_id))
            .is_some_and(|group| group.reverse)
    };
    push_group_bool(state, value)
}

pub(super) fn animation_group_get_elapsed(state: &mut LuaState) -> LuaResult<u32> {
    let group_frame_id = frame_id_from_stack(state, 1)?;
    let value = {
        let sim = borrow_state(state)?;
        resolve_animation_group_id(&sim, group_frame_id)
            .and_then(|group_id| sim.animation_groups.get(&group_id))
            .map(|group| group.elapsed)
            .unwrap_or(0.0)
    };
    push_group_num(state, value)
}

pub(super) fn animation_group_get_progress(state: &mut LuaState) -> LuaResult<u32> {
    let group_frame_id = frame_id_from_stack(state, 1)?;
    let value = {
        let sim = borrow_state(state)?;
        resolve_animation_group_id(&sim, group_frame_id)
            .and_then(|group_id| sim.animation_groups.get(&group_id))
            .map(group_elapsed_progress)
            .unwrap_or(0.0)
    };
    push_group_num(state, value)
}

pub(super) fn animation_group_get_smooth_progress(state: &mut LuaState) -> LuaResult<u32> {
    animation_group_get_progress(state)
}

pub(super) fn animation_group_remove_animations(state: &mut LuaState) -> LuaResult<u32> {
    let group_frame_id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    let Some(group_id) = sim.anim_frame_to_group.get(&group_frame_id).copied() else {
        return Ok(0);
    };
    let removed: Vec<u64> = sim
        .anim_frame_to_anim
        .iter()
        .filter_map(|(&frame_id, &(mapped_group_id, _))| {
            (mapped_group_id == group_id).then_some(frame_id)
        })
        .collect();
    sim.anim_frame_to_anim
        .retain(|_, (mapped_group_id, _)| *mapped_group_id != group_id);
    if let Some(group) = sim.animation_groups.get_mut(&group_id) {
        group.animations.clear();
        group.elapsed = 0.0;
        group.pending_finish = false;
        group.done = false;
    }
    for frame_id in removed {
        if let Some(frame) = sim.widgets.get_mut(frame_id) {
            frame.parent_id = Some(group_frame_id);
        }
    }
    Ok(0)
}

// ── Animation state accessors ─────────────────────────────────────────────────

pub(super) fn animation_set_smoothing(state: &mut LuaState) -> LuaResult<u32> {
    let smoothing = super::shared::opt_string(state, 2).unwrap_or_else(|| "NONE".to_string());
    with_animation_state_mut(state, |a| a.smoothing = smoothing.clone())?;
    Ok(0)
}

pub(super) fn animation_get_smoothing(state: &mut LuaState) -> LuaResult<u32> {
    let value = {
        let animation_frame_id = frame_id_from_stack(state, 1)?;
        let sim = borrow_state(state)?;
        sim.anim_frame_to_anim
            .get(&animation_frame_id)
            .and_then(|(group_id, animation_index)| {
                sim.animation_groups
                    .get(group_id)
                    .and_then(|group| group.animations.get(*animation_index))
            })
            .map(|animation| animation_smoothing_name(animation).to_string())
            .unwrap_or_else(|| "NONE".to_string())
    };
    let value = crate::lua_api::methods::create_string(state, &value);
    state.push(value);
    Ok(1)
}

pub(super) fn animation_set_from_alpha(state: &mut LuaState) -> LuaResult<u32> {
    let value = match stack_val(state, 2) {
        Val::Num(n) => n,
        _ => 0.0,
    };
    with_animation_state_mut(state, |a| a.from_alpha = value)?;
    Ok(0)
}

pub(super) fn animation_get_from_alpha(state: &mut LuaState) -> LuaResult<u32> {
    push_anim_field(state, |a| a.from_alpha)
}

pub(super) fn animation_set_to_alpha(state: &mut LuaState) -> LuaResult<u32> {
    let value = match stack_val(state, 2) {
        Val::Num(n) => n,
        _ => 1.0,
    };
    with_animation_state_mut(state, |a| a.to_alpha = value)?;
    Ok(0)
}

pub(super) fn animation_get_to_alpha(state: &mut LuaState) -> LuaResult<u32> {
    push_anim_field(state, |a| a.to_alpha)
}

pub(super) fn animation_set_change(state: &mut LuaState) -> LuaResult<u32> {
    let change = match stack_val(state, 2) {
        Val::Num(n) => n,
        _ => 0.0,
    };
    with_animation_state_mut(state, |a| a.to_alpha = a.from_alpha + change)?;
    Ok(0)
}

pub(super) fn animation_get_order(state: &mut LuaState) -> LuaResult<u32> {
    push_anim_field(state, |a| a.order as f64)
}

pub(super) fn animation_get_start_delay(state: &mut LuaState) -> LuaResult<u32> {
    push_anim_field(state, |a| a.start_delay)
}

pub(super) fn animation_get_end_delay(state: &mut LuaState) -> LuaResult<u32> {
    push_anim_field(state, |a| a.end_delay)
}

pub(super) fn animation_get_elapsed(state: &mut LuaState) -> LuaResult<u32> {
    let frame_id = frame_id_from_stack(state, 1)?;
    let value = {
        let sim = borrow_state(state)?;
        if let Some(group_id) = sim.anim_frame_to_group.get(&frame_id).copied() {
            sim.animation_groups
                .get(&group_id)
                .map(|group| group.elapsed)
                .unwrap_or(0.0)
        } else {
            sim.anim_frame_to_anim
                .get(&frame_id)
                .and_then(|(group_id, animation_index)| {
                    sim.animation_groups
                        .get(group_id)
                        .and_then(|group| group.animations.get(*animation_index))
                        .map(|animation| animation.elapsed)
                })
                .unwrap_or(0.0)
        }
    };
    push_group_num(state, value)
}

pub(super) fn animation_get_progress(state: &mut LuaState) -> LuaResult<u32> {
    let animation_frame_id = frame_id_from_stack(state, 1)?;
    let value = {
        let sim = borrow_state(state)?;
        animation_progress_for_frame(&sim, animation_frame_id)
    };
    push_group_num(state, value)
}

pub(super) fn animation_get_smooth_progress(state: &mut LuaState) -> LuaResult<u32> {
    animation_get_progress(state)
}

pub(super) fn animation_is_stopped(state: &mut LuaState) -> LuaResult<u32> {
    let animation_frame_id = frame_id_from_stack(state, 1)?;
    let value = {
        let sim = borrow_state(state)?;
        sim.anim_frame_to_anim
            .get(&animation_frame_id)
            .and_then(|(group_id, _)| sim.animation_groups.get(group_id))
            .is_none_or(|group| !group.playing)
    };
    push_group_bool(state, value)
}

pub(super) fn animation_is_delaying(state: &mut LuaState) -> LuaResult<u32> {
    let animation_frame_id = frame_id_from_stack(state, 1)?;
    let value = {
        let sim = borrow_state(state)?;
        sim.anim_frame_to_anim
            .get(&animation_frame_id)
            .and_then(|(group_id, animation_index)| {
                sim.animation_groups.get(group_id).and_then(|group| {
                    group.animations.get(*animation_index).map(|animation| {
                        animation.elapsed < animation.start_delay && animation.start_delay > 0.0
                    })
                })
            })
            .unwrap_or(false)
    };
    push_group_bool(state, value)
}

// ── Animation state setters ───────────────────────────────────────────────────

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

pub(super) fn animation_set_duration(state: &mut LuaState) -> LuaResult<u32> {
    let duration = animation_numeric_arg(state, 2);
    with_animation_state_mut(state, |a| a.duration = duration)?;
    Ok(0)
}

pub(super) fn animation_get_duration(state: &mut LuaState) -> LuaResult<u32> {
    let frame_id = frame_id_from_stack(state, 1)?;
    let value = {
        let sim = borrow_state(state)?;
        if let Some(group_id) = sim.anim_frame_to_group.get(&frame_id).copied() {
            sim.animation_groups
                .get(&group_id)
                .map(animation_group_total_duration)
                .unwrap_or(0.0)
        } else {
            sim.anim_frame_to_anim
                .get(&frame_id)
                .and_then(|(group_id, animation_index)| {
                    sim.animation_groups
                        .get(group_id)
                        .and_then(|group| group.animations.get(*animation_index))
                        .map(|animation| animation.duration)
                })
                .unwrap_or(0.0)
        }
    };
    push_group_num(state, value)
}

pub(super) fn animation_set_order(state: &mut LuaState) -> LuaResult<u32> {
    let order = match stack_val(state, 2) {
        Val::Num(value) if value >= 0.0 => value as u32,
        _ => 0,
    };
    with_animation_state_mut(state, |a| a.order = order)?;
    Ok(0)
}

pub(super) fn animation_set_start_delay(state: &mut LuaState) -> LuaResult<u32> {
    let start_delay = animation_numeric_arg(state, 2);
    with_animation_state_mut(state, |a| a.start_delay = start_delay)?;
    Ok(0)
}

pub(super) fn animation_set_end_delay(state: &mut LuaState) -> LuaResult<u32> {
    let end_delay = animation_numeric_arg(state, 2);
    with_animation_state_mut(state, |a| a.end_delay = end_delay)?;
    Ok(0)
}

pub(super) fn set_animation_child_key(state: &mut LuaState) -> LuaResult<u32> {
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

// ── Flipbook animation ────────────────────────────────────────────────────────

pub(super) fn animation_set_flipbook_rows(state: &mut LuaState) -> LuaResult<u32> {
    let rows = animation_numeric_arg(state, 2) as u32;
    with_animation_state_mut(state, |a| a.flipbook_rows = rows)?;
    Ok(0)
}

pub(super) fn animation_get_flipbook_rows(state: &mut LuaState) -> LuaResult<u32> {
    push_anim_field(state, |a| a.flipbook_rows as f64)
}

pub(super) fn animation_set_flipbook_columns(state: &mut LuaState) -> LuaResult<u32> {
    let columns = animation_numeric_arg(state, 2) as u32;
    with_animation_state_mut(state, |a| a.flipbook_columns = columns)?;
    Ok(0)
}

pub(super) fn animation_get_flipbook_columns(state: &mut LuaState) -> LuaResult<u32> {
    push_anim_field(state, |a| a.flipbook_columns as f64)
}

pub(super) fn animation_set_flipbook_frames(state: &mut LuaState) -> LuaResult<u32> {
    let frames = animation_numeric_arg(state, 2) as u32;
    with_animation_state_mut(state, |a| a.flipbook_frames = frames)?;
    Ok(0)
}

pub(super) fn animation_get_flipbook_frames(state: &mut LuaState) -> LuaResult<u32> {
    push_anim_field(state, |a| a.flipbook_frames as f64)
}

pub(super) fn animation_set_flipbook_frame_width(state: &mut LuaState) -> LuaResult<u32> {
    let width = animation_numeric_arg(state, 2);
    with_animation_state_mut(state, |a| a.flipbook_frame_width = width)?;
    Ok(0)
}

pub(super) fn animation_get_flipbook_frame_width(state: &mut LuaState) -> LuaResult<u32> {
    push_anim_field(state, |a| a.flipbook_frame_width)
}

pub(super) fn animation_set_flipbook_frame_height(state: &mut LuaState) -> LuaResult<u32> {
    let height = animation_numeric_arg(state, 2);
    with_animation_state_mut(state, |a| a.flipbook_frame_height = height)?;
    Ok(0)
}

pub(super) fn animation_get_flipbook_frame_height(state: &mut LuaState) -> LuaResult<u32> {
    push_anim_field(state, |a| a.flipbook_frame_height)
}

fn push_anim_field<F>(state: &mut LuaState, f: F) -> LuaResult<u32>
where
    F: Fn(&crate::lua_api::animation::AnimState) -> f64,
{
    let animation_frame_id = frame_id_from_stack(state, 1)?;
    let value = {
        let sim = borrow_state(state)?;
        sim.anim_frame_to_anim
            .get(&animation_frame_id)
            .and_then(|(group_id, animation_index)| {
                sim.animation_groups
                    .get(group_id)
                    .and_then(|group| group.animations.get(*animation_index))
                    .map(&f)
            })
            .unwrap_or(0.0)
    };
    state.push(Val::Num(value));
    Ok(1)
}

fn animation_group_total_duration(group: &crate::lua_api::animation::AnimGroupState) -> f64 {
    let mut duration_by_order = std::collections::BTreeMap::<u32, f64>::new();
    for animation in &group.animations {
        let total_time = animation.total_time();
        duration_by_order
            .entry(animation.order)
            .and_modify(|current| *current = current.max(total_time))
            .or_insert(total_time);
    }
    duration_by_order.into_values().sum()
}

fn animation_group_frame_id(group: &crate::lua_api::animation::AnimGroupState) -> u64 {
    group.frame_id.unwrap_or(group.owner_frame_id)
}

fn push_group_bool(state: &mut LuaState, value: bool) -> LuaResult<u32> {
    state.push(Val::Bool(value));
    Ok(1)
}

fn push_group_num(state: &mut LuaState, value: f64) -> LuaResult<u32> {
    state.push(Val::Num(value));
    Ok(1)
}

fn current_group_total_duration(group: &crate::lua_api::animation::AnimGroupState) -> f64 {
    animation_group_total_duration(group)
}

fn group_elapsed_progress(group: &crate::lua_api::animation::AnimGroupState) -> f64 {
    let duration = current_group_total_duration(group);
    if duration <= 0.0 {
        0.0
    } else {
        (group.elapsed / duration).clamp(0.0, 1.0)
    }
}

fn animation_progress_for_frame(sim: &crate::lua_api::SimState, animation_frame_id: u64) -> f64 {
    sim.anim_frame_to_group
        .get(&animation_frame_id)
        .copied()
        .and_then(|group_id| sim.animation_groups.get(&group_id))
        .map(group_elapsed_progress)
        .or_else(|| animation_child_progress_for_frame(sim, animation_frame_id))
        .unwrap_or(0.0)
}

fn animation_child_progress_for_frame(
    sim: &crate::lua_api::SimState,
    animation_frame_id: u64,
) -> Option<f64> {
    let (group_id, animation_index) = sim.anim_frame_to_anim.get(&animation_frame_id)?;
    let group = sim.animation_groups.get(group_id)?;
    let animation = group.animations.get(*animation_index)?;
    Some(animation_elapsed_progress(animation))
}

fn animation_elapsed_progress(animation: &crate::lua_api::animation::AnimState) -> f64 {
    let duration = animation.duration.max(0.0);
    if duration <= 0.0 {
        0.0
    } else {
        (animation.elapsed / duration).clamp(0.0, 1.0)
    }
}

fn animation_smoothing_name(animation: &crate::lua_api::animation::AnimState) -> &str {
    animation.smoothing.as_str()
}

fn group_loop_name(group: &crate::lua_api::animation::AnimGroupState) -> &'static str {
    match group.looping {
        crate::lua_api::animation::LoopType::None => "NONE",
        crate::lua_api::animation::LoopType::Repeat => "REPEAT",
        crate::lua_api::animation::LoopType::Bounce => "BOUNCE",
    }
}

fn finish_group_now(
    group: &mut crate::lua_api::animation::AnimGroupState,
    total_duration: f64,
) -> u64 {
    group.elapsed = total_duration.max(0.0);
    group.playing = false;
    group.paused = false;
    group.done = true;
    group.pending_finish = false;
    sync_animation_elapsed(group);
    animation_group_frame_id(group)
}

pub(crate) fn advance_animation_groups(
    env: &crate::lua_api::env::WowLuaEnv,
    elapsed: f64,
) -> crate::Result<()> {
    let mut finished_scripts = Vec::new();
    let mut finished_animation_scripts = Vec::new();
    let mut loop_scripts = Vec::new();
    let mut sim = env.state().borrow_mut();
    let group_ids: Vec<u64> = sim.animation_groups.keys().copied().collect();
    for group_id in group_ids {
        let Some(result) = advance_animation_group(
            &mut sim,
            group_id,
            elapsed,
            &mut finished_scripts,
            &mut finished_animation_scripts,
        ) else {
            continue;
        };
        apply_animation_group_outcome(&mut sim, &result);
        sync_action_bar_busy_for_group(&mut sim, group_id);
        for _ in 0..result.loop_count {
            loop_scripts.push(result.frame_id);
        }
    }
    drop(sim);

    fire_animation_group_scripts(env, "OnLoop", loop_scripts)?;
    fire_animation_group_scripts(env, "OnFinished", finished_animation_scripts)?;
    fire_animation_group_scripts(env, "OnFinished", finished_scripts)?;
    Ok(())
}

pub(crate) fn stop_animation_groups_for_hidden_subtree(
    sim: &mut crate::lua_api::state::SimState,
    root_id: u64,
) {
    let mut subtree_ids = std::collections::HashSet::new();
    collect_subtree_ids(sim, root_id, &mut subtree_ids);

    let group_ids: Vec<u64> = sim
        .animation_groups
        .iter()
        .filter_map(|(&group_id, group)| {
            subtree_ids
                .contains(&group.owner_frame_id)
                .then_some(group_id)
        })
        .collect();

    for group_id in group_ids {
        if let Some(group) = sim.animation_groups.get_mut(&group_id) {
            stop_group(group);
        }
        apply_group_flipbook_state(sim, group_id);
        sync_action_bar_busy_for_group(sim, group_id);
    }
}

fn collect_subtree_ids(
    sim: &crate::lua_api::state::SimState,
    frame_id: u64,
    subtree_ids: &mut std::collections::HashSet<u64>,
) {
    if !subtree_ids.insert(frame_id) {
        return;
    }

    let children = sim
        .widgets
        .get(frame_id)
        .map(|frame| frame.children.clone())
        .unwrap_or_default();
    for child_id in children {
        collect_subtree_ids(sim, child_id, subtree_ids);
    }
}

fn stop_group(group: &mut crate::lua_api::animation::AnimGroupState) {
    group.playing = false;
    group.paused = false;
    group.done = true;
    group.pending_finish = false;
    group.elapsed = 0.0;
    for animation in &mut group.animations {
        animation.elapsed = 0.0;
    }
}

struct AnimationGroupAdvance {
    owner_id: u64,
    alpha_updates: Vec<AlphaUpdate>,
    flipbook_updates: Vec<FlipbookUpdate>,
    loop_count: u32,
    frame_id: u64,
}

struct AlphaUpdate {
    target_id: u64,
    pending_alpha: Option<f64>,
    restore_saved_alpha: Option<f32>,
}

struct FlipbookUpdate {
    child_key: Option<String>,
    frame_index: u32,
    rows: u32,
    columns: u32,
    frames: u32,
}

fn advance_animation_group(
    sim: &mut crate::lua_api::state::SimState,
    group_id: u64,
    elapsed: f64,
    finished_scripts: &mut Vec<u64>,
    finished_animation_scripts: &mut Vec<u64>,
) -> Option<AnimationGroupAdvance> {
    let (owner_id, alpha_target_ids_by_animation) = {
        let group = sim.animation_groups.get(&group_id)?;
        // Most registered animation groups are idle; avoid any expensive
        // target-resolution work unless the group is actively ticking.
        if !group.playing || group.paused {
            return None;
        }
        (
            group.owner_frame_id,
            resolve_group_alpha_targets(sim, group),
        )
    };
    let unique_alpha_target_ids = unique_alpha_targets(&alpha_target_ids_by_animation);
    let saved_alphas: std::collections::HashMap<u64, f32> = unique_alpha_target_ids
        .iter()
        .copied()
        .map(|target_id| {
            let alpha = sim
                .widgets
                .get(target_id)
                .map(|frame| frame.alpha)
                .unwrap_or(1.0);
            (target_id, alpha)
        })
        .collect();
    let mut loop_count = 0u32;
    let (group_finished, alpha_updates, flipbook_updates, frame_id) = {
        let group = sim.animation_groups.get_mut(&group_id)?;

        for (&target_id, &saved_alpha) in &saved_alphas {
            group.saved_alphas.entry(target_id).or_insert(saved_alpha);
        }
        let total_duration = current_group_total_duration(group);

        let group_finished = advance_group_playback(
            group,
            elapsed,
            total_duration,
            &mut loop_count,
            finished_scripts,
        );

        sync_animation_elapsed(group);
        let mut alpha_updates = Vec::new();
        for target_id in unique_alpha_target_ids.iter().copied() {
            let pending_alpha = group_current_alpha_for_target(
                group,
                group.elapsed,
                target_id,
                &alpha_target_ids_by_animation,
            );
            let restore_saved_alpha = if !group.playing && group.done && !group.set_to_final_alpha {
                group.saved_alphas.get(&target_id).copied()
            } else {
                None
            };
            alpha_updates.push(AlphaUpdate {
                target_id,
                pending_alpha,
                restore_saved_alpha,
            });
        }
        let flipbook_updates = collect_group_flipbook_updates(group);
        let frame_id = animation_group_frame_id(group);
        (group_finished, alpha_updates, flipbook_updates, frame_id)
    };

    if group_finished {
        finished_animation_scripts.extend(animation_frame_ids_for_group(sim, group_id));
    }

    Some(AnimationGroupAdvance {
        owner_id,
        alpha_updates,
        flipbook_updates,
        loop_count,
        frame_id,
    })
}

fn advance_group_playback(
    group: &mut crate::lua_api::animation::AnimGroupState,
    elapsed: f64,
    total_duration: f64,
    loop_count: &mut u32,
    finished_scripts: &mut Vec<u64>,
) -> bool {
    if group.pending_finish {
        finished_scripts.push(finish_group_now(group, total_duration));
        return true;
    }

    let was_done = group.done;
    advance_group_elapsed(group, elapsed, total_duration, loop_count, finished_scripts);
    !was_done && group.done && !group.playing
}

fn animation_frame_ids_for_group(sim: &crate::lua_api::state::SimState, group_id: u64) -> Vec<u64> {
    let mut frame_ids: Vec<(usize, u64)> = sim
        .anim_frame_to_anim
        .iter()
        .filter_map(|(&frame_id, &(mapped_group_id, animation_index))| {
            (mapped_group_id == group_id).then_some((animation_index, frame_id))
        })
        .collect();
    frame_ids.sort_unstable_by_key(|(animation_index, _)| *animation_index);
    frame_ids
        .into_iter()
        .map(|(_, frame_id)| frame_id)
        .collect()
}

fn advance_group_elapsed(
    group: &mut crate::lua_api::animation::AnimGroupState,
    elapsed: f64,
    total_duration: f64,
    loop_count: &mut u32,
    finished_scripts: &mut Vec<u64>,
) {
    let advance = elapsed * group.speed_multiplier.max(0.0);
    if group.reverse {
        group.elapsed -= advance;
    } else {
        group.elapsed += advance;
    }

    if total_duration <= 0.0 {
        finished_scripts.push(finish_group_now(group, total_duration));
        return;
    }

    match group.looping {
        crate::lua_api::animation::LoopType::None => {
            finish_unlooped_group_at_boundary(group, total_duration, finished_scripts)
        }
        crate::lua_api::animation::LoopType::Repeat => {
            wrap_repeating_group_elapsed(group, total_duration, loop_count);
        }
        crate::lua_api::animation::LoopType::Bounce => {
            bounce_group_elapsed_at_boundaries(group, total_duration, loop_count);
        }
    }
}

fn finish_unlooped_group_at_boundary(
    group: &mut crate::lua_api::animation::AnimGroupState,
    total_duration: f64,
    finished_scripts: &mut Vec<u64>,
) {
    let finish_elapsed = if group.reverse && group.elapsed <= 0.0 {
        Some(0.0)
    } else if !group.reverse && group.elapsed >= total_duration {
        Some(total_duration)
    } else {
        None
    };

    if let Some(elapsed) = finish_elapsed {
        finished_scripts.push(finish_group_now(group, elapsed));
    }
}

fn wrap_repeating_group_elapsed(
    group: &mut crate::lua_api::animation::AnimGroupState,
    total_duration: f64,
    loop_count: &mut u32,
) {
    if group.reverse {
        wrap_reverse_repeating_group(group, total_duration, loop_count);
    } else {
        wrap_forward_repeating_group(group, total_duration, loop_count);
    }
}

fn wrap_reverse_repeating_group(
    group: &mut crate::lua_api::animation::AnimGroupState,
    total_duration: f64,
    loop_count: &mut u32,
) {
    while group.elapsed < 0.0 {
        group.elapsed += total_duration;
        *loop_count += 1;
    }
}

fn wrap_forward_repeating_group(
    group: &mut crate::lua_api::animation::AnimGroupState,
    total_duration: f64,
    loop_count: &mut u32,
) {
    while group.elapsed >= total_duration {
        group.elapsed -= total_duration;
        *loop_count += 1;
    }
}

fn bounce_group_elapsed_at_boundaries(
    group: &mut crate::lua_api::animation::AnimGroupState,
    total_duration: f64,
    loop_count: &mut u32,
) {
    while is_group_elapsed_outside_bounds(group.elapsed, total_duration) {
        reflect_bouncing_group_elapsed(group, total_duration);
        group.reverse = !group.reverse;
        *loop_count += 1;
    }
}

fn is_group_elapsed_outside_bounds(elapsed: f64, total_duration: f64) -> bool {
    elapsed >= total_duration || elapsed < 0.0
}

fn reflect_bouncing_group_elapsed(
    group: &mut crate::lua_api::animation::AnimGroupState,
    total_duration: f64,
) {
    if group.elapsed >= total_duration {
        group.elapsed = (group.elapsed - total_duration).max(0.0);
    } else {
        group.elapsed = (-group.elapsed).min(total_duration);
    }
}

fn apply_animation_group_outcome(
    sim: &mut crate::lua_api::state::SimState,
    result: &AnimationGroupAdvance,
) {
    let mut changed_alpha_targets = Vec::new();
    for alpha_update in &result.alpha_updates {
        let mut changed = false;
        if let Some(alpha) = alpha_update.pending_alpha
            && let Some(frame) = sim.widgets.get_mut_visual(alpha_update.target_id)
            && (frame.alpha as f64 - alpha).abs() > f32::EPSILON as f64
        {
            frame.alpha = alpha as f32;
            changed = true;
        }

        if let Some(saved_alpha) = alpha_update.restore_saved_alpha
            && let Some(frame) = sim.widgets.get_mut_visual(alpha_update.target_id)
            && (frame.alpha - saved_alpha).abs() > f32::EPSILON
        {
            frame.alpha = saved_alpha;
            changed = true;
        }

        if changed {
            changed_alpha_targets.push(alpha_update.target_id);
        }
    }

    apply_group_flipbook_updates(sim, result.owner_id, &result.flipbook_updates);

    for target_id in changed_alpha_targets {
        let parent_effective_alpha = sim
            .widgets
            .get(target_id)
            .and_then(|frame| frame.parent_id)
            .and_then(|parent_id| sim.widgets.get(parent_id))
            .map(|parent| parent.effective_alpha)
            .unwrap_or(1.0_f32);
        sim.widgets
            .propagate_effective_alpha(target_id, parent_effective_alpha);
    }
}

fn fire_animation_group_scripts(
    env: &crate::lua_api::env::WowLuaEnv,
    handler_name: &str,
    frame_ids: Vec<u64>,
) -> crate::Result<()> {
    for frame_id in frame_ids {
        env.fire_script_handler(frame_id, handler_name, Vec::new())?;
    }
    Ok(())
}

fn resolve_group_alpha_targets(
    sim: &crate::lua_api::state::SimState,
    group: &crate::lua_api::animation::AnimGroupState,
) -> Vec<Option<u64>> {
    group
        .animations
        .iter()
        .map(|animation| {
            if !matches!(
                animation.anim_type,
                crate::lua_api::animation::AnimationType::Alpha
            ) {
                return None;
            }
            resolve_anim_target_id(sim, group.owner_frame_id, animation.child_key.as_deref())
        })
        .collect()
}

fn unique_alpha_targets(alpha_targets_by_animation: &[Option<u64>]) -> Vec<u64> {
    let mut seen = std::collections::HashSet::new();
    let mut unique = Vec::new();
    for target_id in alpha_targets_by_animation.iter().flatten().copied() {
        if seen.insert(target_id) {
            unique.push(target_id);
        }
    }
    unique
}

fn group_current_alpha_for_target(
    group: &crate::lua_api::animation::AnimGroupState,
    elapsed: f64,
    target_id: u64,
    alpha_targets_by_animation: &[Option<u64>],
) -> Option<f64> {
    use std::collections::BTreeMap;

    let mut groups: BTreeMap<u32, Vec<usize>> = BTreeMap::new();
    for (index, animation) in group.animations.iter().enumerate() {
        groups.entry(animation.order).or_default().push(index);
    }
    let mut remaining = elapsed;
    for (_order, animation_indices) in groups {
        let order_duration = animation_indices
            .iter()
            .map(|&index| group.animations[index].total_time())
            .fold(0.0, f64::max);
        let within_order = remaining.min(order_duration);
        let mut current = None;
        for &index in &animation_indices {
            if alpha_targets_by_animation.get(index).copied().flatten() != Some(target_id) {
                continue;
            }
            if let Some(alpha) = current_animation_alpha(&group.animations[index], within_order) {
                current = Some(alpha);
            }
        }
        if remaining <= order_duration {
            return current;
        }
        remaining -= order_duration;
    }
    None
}

fn current_animation_alpha(
    animation: &crate::lua_api::animation::AnimState,
    within_order: f64,
) -> Option<f64> {
    if !matches!(
        animation.anim_type,
        crate::lua_api::animation::AnimationType::Alpha
    ) {
        return None;
    }

    let start = animation.start_delay.max(0.0);
    let end = start + animation.duration.max(0.0);
    let alpha = if within_order <= start {
        animation.from_alpha
    } else if within_order >= end || animation.duration <= 0.0 {
        animation.to_alpha
    } else {
        let progress = ((within_order - start) / animation.duration).clamp(0.0, 1.0);
        animation.from_alpha + (animation.to_alpha - animation.from_alpha) * progress
    };
    Some(alpha)
}

fn apply_group_flipbook_state(sim: &mut crate::lua_api::state::SimState, group_id: u64) {
    let Some(group) = sim.animation_groups.get(&group_id) else {
        return;
    };
    let owner_id = group.owner_frame_id;
    let updates = collect_group_flipbook_updates(group);
    apply_group_flipbook_updates(sim, owner_id, &updates);
}

fn collect_group_flipbook_updates(
    group: &crate::lua_api::animation::AnimGroupState,
) -> Vec<FlipbookUpdate> {
    group
        .animations
        .iter()
        .filter_map(|animation| {
            flipbook_frame_index(animation).map(|frame_index| FlipbookUpdate {
                child_key: animation.child_key.clone(),
                frame_index,
                rows: animation.flipbook_rows,
                columns: animation.flipbook_columns,
                frames: animation.flipbook_frames,
            })
        })
        .collect()
}

fn sync_action_bar_busy_for_group(sim: &mut crate::lua_api::state::SimState, group_id: u64) {
    let Some(group) = sim.animation_groups.get(&group_id) else {
        return;
    };
    if is_override_action_bar_slideout(sim, group) {
        sim.action_bar_state.busy = group.playing;
    }
}

fn is_override_action_bar_slideout(
    sim: &crate::lua_api::state::SimState,
    group: &crate::lua_api::animation::AnimGroupState,
) -> bool {
    let owner_is_override_bar = sim
        .widgets
        .get(group.owner_frame_id)
        .and_then(|owner| owner.name.as_deref())
        == Some("OverrideActionBar");
    if !owner_is_override_bar {
        return false;
    }

    let Some(group_frame_id) = group.frame_id else {
        return false;
    };
    let owner_child_key_matches = sim
        .widgets
        .get(group.owner_frame_id)
        .and_then(|owner| owner.children_keys.get("slideOut"))
        .copied()
        == Some(group_frame_id);
    let group_parent_key_matches = sim
        .widgets
        .get(group_frame_id)
        .and_then(|frame| frame.parent_key.as_deref())
        == Some("slideOut");
    owner_child_key_matches || group_parent_key_matches
}

fn apply_group_flipbook_updates(
    sim: &mut crate::lua_api::state::SimState,
    owner_id: u64,
    updates: &[FlipbookUpdate],
) {
    for update in updates {
        let Some(target_id) = resolve_anim_target_id(sim, owner_id, update.child_key.as_deref())
        else {
            continue;
        };
        let Some(frame) = sim.widgets.get_mut_visual(target_id) else {
            continue;
        };
        if let Some(tex_coords) = flipbook_tex_coords(
            frame.atlas_tex_coords.or(frame.tex_coords),
            update.rows,
            update.columns,
            update.frames,
            update.frame_index,
        ) {
            frame.tex_coords = Some(tex_coords);
            frame.tex_coords_quad = None;
        }
    }
}

fn flipbook_frame_index(animation: &crate::lua_api::animation::AnimState) -> Option<u32> {
    if !matches!(
        animation.anim_type,
        crate::lua_api::animation::AnimationType::FlipBook
    ) {
        return None;
    }
    let frames = animation.flipbook_frames;
    let columns = animation.flipbook_columns;
    let rows = animation.flipbook_rows;
    if frames == 0 || columns == 0 || rows == 0 {
        return None;
    }
    let duration = animation.duration.max(0.0);
    let frame_index = if duration <= 0.0 {
        0
    } else {
        let progress = (animation.elapsed / duration).clamp(0.0, 1.0);
        let candidate = (progress * frames as f64).floor() as u32;
        candidate.min(frames.saturating_sub(1))
    };
    Some(frame_index)
}

fn flipbook_tex_coords(
    atlas_tex_coords: Option<(f32, f32, f32, f32)>,
    rows: u32,
    columns: u32,
    frames: u32,
    frame_index: u32,
) -> Option<(f32, f32, f32, f32)> {
    let (al, ar, at, ab) = atlas_tex_coords?;
    if rows == 0 || columns == 0 || frames == 0 {
        return None;
    }
    let frame_index = frame_index.min(frames.saturating_sub(1));
    let row = frame_index / columns;
    if row >= rows {
        return None;
    }
    let col = frame_index % columns;
    let width = (ar - al) / columns as f32;
    let height = (ab - at) / rows as f32;
    let left = al + col as f32 * width;
    let right = left + width;
    let top = at + row as f32 * height;
    let bottom = top + height;
    Some((left, right, top, bottom))
}

fn sync_animation_elapsed(group: &mut crate::lua_api::animation::AnimGroupState) {
    use std::collections::BTreeMap;

    let mut groups: BTreeMap<u32, Vec<usize>> = BTreeMap::new();
    for (index, animation) in group.animations.iter().enumerate() {
        groups.entry(animation.order).or_default().push(index);
    }

    let mut remaining = group.elapsed;
    for (_order, indices) in groups {
        let order_duration = indices
            .iter()
            .map(|&index| group.animations[index].total_time())
            .fold(0.0, f64::max);
        let within_order = remaining.min(order_duration);
        for index in indices {
            let animation = &mut group.animations[index];
            let start = animation.start_delay.max(0.0);
            let end = start + animation.duration.max(0.0);
            animation.elapsed = if within_order <= start {
                0.0
            } else if within_order >= end || animation.duration <= 0.0 {
                animation.duration.max(0.0)
            } else {
                (within_order - start).clamp(0.0, animation.duration.max(0.0))
            };
        }
        if remaining <= order_duration {
            break;
        }
        remaining -= order_duration;
    }
}

// ── No-op stubs ───────────────────────────────────────────────────────────────

pub(super) fn animation_config_noop(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

pub(super) fn set_scale_dispatch(state: &mut LuaState) -> LuaResult<u32> {
    use crate::lua_api::methods::extract_frame_id;
    if extract_frame_id(state, stack_val(state, 1)).is_some() {
        return crate::lua_api::frame::methods::core_state::set_scale(state);
    }
    animation_config_noop(state)
}

// ── Creation ──────────────────────────────────────────────────────────────────

/// CreateAnimationGroup([name [, inherits]]) -> animationGroup
pub(super) fn create_animation_group(state: &mut LuaState) -> LuaResult<u32> {
    use crate::lua_api::animation::AnimGroupState;
    use crate::widget::{Frame, WidgetType};
    let parent_id = frame_id_from_stack(state, 1)?;
    let name_raw: Option<String> = Option::<String>::from_stack(state, 2)?;
    let _inherits: Option<String> = Option::<String>::from_stack(state, 3)?;
    let name = resolve_child_name(state, name_raw, parent_id);
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
pub(super) fn create_animation(state: &mut LuaState) -> LuaResult<u32> {
    use crate::lua_api::animation::AnimationType;
    let group_frame_id = frame_id_from_stack(state, 1)?;
    let anim_type_str = super::shared::opt_string(state, 2);
    let anim_name_raw: Option<String> = Option::<String>::from_stack(state, 3)?;

    let group_id = lookup_anim_group_id(state, group_frame_id)?;
    let anim_type = AnimationType::from_str(anim_type_str.as_deref().unwrap_or("Animation"));
    let name = resolve_child_name(state, anim_name_raw, group_frame_id);
    let (child_id, anim) = build_animation_child(state, group_frame_id, name.clone(), anim_type)?;
    register_animation(state, group_frame_id, group_id, child_id, name, anim)?;

    let val = frame_ref(state, child_id)?;
    state.push(val);
    Ok(1)
}

fn lookup_anim_group_id(state: &mut LuaState, group_frame_id: u64) -> LuaResult<u64> {
    let sim = borrow_state(state)?;
    sim.anim_frame_to_group
        .get(&group_frame_id)
        .copied()
        .ok_or_else(|| runtime_error("CreateAnimation called on non-AnimationGroup"))
}

fn build_animation_child(
    state: &mut LuaState,
    group_frame_id: u64,
    name: Option<String>,
    anim_type: crate::lua_api::animation::AnimationType,
) -> LuaResult<(u64, crate::lua_api::animation::AnimState)> {
    use crate::lua_api::animation::AnimState;
    use crate::widget::{Frame, WidgetType};
    let mut child = Frame::new(WidgetType::Frame, name.clone(), Some(group_frame_id));
    child.object_type_name = Some(anim_type.as_str().to_string());
    let child_id = child.id;
    let mut anim = AnimState::new(anim_type);
    anim.name = name;

    let mut sim = borrow_state_mut(state)?;
    sim.widgets.register(child);
    Ok((child_id, anim))
}

fn register_animation(
    state: &mut LuaState,
    group_frame_id: u64,
    group_id: u64,
    child_id: u64,
    _name: Option<String>,
    anim: crate::lua_api::animation::AnimState,
) -> LuaResult<()> {
    let mut sim = borrow_state_mut(state)?;
    let group = sim
        .animation_groups
        .get_mut(&group_id)
        .ok_or_else(|| runtime_error("Animation group not found"))?;
    let idx = group.animations.len();
    group.animations.push(anim);
    sim.anim_frame_to_anim.insert(child_id, (group_id, idx));
    sim.widgets.add_child(group_frame_id, child_id);
    sim.invalidate_strata_buckets();
    Ok(())
}

/// CreateControlPoint([name]) -> controlPoint
pub(super) fn create_control_point(state: &mut LuaState) -> LuaResult<u32> {
    use crate::widget::{Frame, WidgetType};
    let parent_id = frame_id_from_stack(state, 1)?;
    let name_raw: Option<String> = Option::<String>::from_stack(state, 2)?;
    let name = resolve_child_name(state, name_raw, parent_id);
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
