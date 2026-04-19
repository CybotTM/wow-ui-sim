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
    let reverse = matches!(stack_val(state, 2), Val::Bool(true));
    let mut sim = borrow_state_mut(state)?;
    if let Some(group_id) = resolve_animation_group_id(&sim, group_frame_id)
        && let Some(group) = sim.animation_groups.get_mut(&group_id)
    {
        group.playing = true;
        group.paused = false;
        group.done = false;
        group.pending_finish = false;
        group.reverse = reverse;
    }
    Ok(0)
}

pub(super) fn animation_group_pause(state: &mut LuaState) -> LuaResult<u32> {
    let group_frame_id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(group_id) = resolve_animation_group_id(&sim, group_frame_id)
        && let Some(group) = sim.animation_groups.get_mut(&group_id)
        && group.playing
    {
        group.playing = false;
        group.paused = true;
    }
    Ok(0)
}

pub(super) fn animation_group_stop(state: &mut LuaState) -> LuaResult<u32> {
    let group_frame_id = frame_id_from_stack(state, 1)?;
    let mut sim = borrow_state_mut(state)?;
    if let Some(group_id) = resolve_animation_group_id(&sim, group_frame_id)
        && let Some(group) = sim.animation_groups.get_mut(&group_id)
    {
        group.playing = false;
        group.paused = false;
        group.done = true;
        group.pending_finish = false;
        group.elapsed = 0.0;
        for animation in &mut group.animations {
            animation.elapsed = 0.0;
        }
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
        if let Some(group_id) = sim.anim_frame_to_group.get(&animation_frame_id).copied() {
            sim.animation_groups
                .get(&group_id)
                .map(group_elapsed_progress)
                .unwrap_or(0.0)
        } else {
            sim.anim_frame_to_anim
                .get(&animation_frame_id)
                .and_then(|(group_id, animation_index)| {
                    sim.animation_groups.get(group_id).and_then(|group| {
                        group.animations.get(*animation_index).map(|animation| {
                            let total = animation.duration.max(0.0);
                            if total <= 0.0 {
                                0.0
                            } else {
                                (animation.elapsed / total).clamp(0.0, 1.0)
                            }
                        })
                    })
                })
                .unwrap_or(0.0)
        }
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
                    .map(|animation| f(animation))
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
    let mut loop_scripts = Vec::new();
    let mut sim = env.state().borrow_mut();
    let group_ids: Vec<u64> = sim.animation_groups.keys().copied().collect();
    for group_id in group_ids {
        let Some(result) =
            advance_animation_group(&mut sim, group_id, elapsed, &mut finished_scripts)
        else {
            continue;
        };
        apply_animation_group_outcome(&mut sim, &result);
        for _ in 0..result.loop_count {
            loop_scripts.push(result.frame_id);
        }
    }
    drop(sim);

    fire_animation_group_scripts(env, "OnLoop", loop_scripts)?;
    fire_animation_group_scripts(env, "OnFinished", finished_scripts)?;
    Ok(())
}

struct AnimationGroupAdvance {
    owner_id: u64,
    parent_effective_alpha: f32,
    pending_alpha: Option<f64>,
    restore_saved_alpha: Option<f32>,
    loop_count: u32,
    frame_id: u64,
}

fn advance_animation_group(
    sim: &mut crate::lua_api::state::SimState,
    group_id: u64,
    elapsed: f64,
    finished_scripts: &mut Vec<u64>,
) -> Option<AnimationGroupAdvance> {
    let owner_id = sim.animation_groups.get(&group_id)?.owner_frame_id;
    let parent_effective_alpha = sim
        .widgets
        .get(owner_id)
        .and_then(|frame| frame.parent_id)
        .and_then(|parent_id| sim.widgets.get(parent_id))
        .map(|parent| parent.effective_alpha)
        .unwrap_or(1.0_f32);
    let saved_alpha = sim
        .widgets
        .get(owner_id)
        .map(|frame| frame.alpha)
        .unwrap_or(1.0_f32);
    let mut loop_count = 0u32;
    let (pending_alpha, restore_saved_alpha, frame_id) = {
        let group = sim.animation_groups.get_mut(&group_id)?;
        if !group.playing || group.paused {
            return None;
        }

        group.saved_alphas.entry(owner_id).or_insert(saved_alpha);
        let total_duration = current_group_total_duration(group);

        if group.pending_finish {
            finished_scripts.push(finish_group_now(group, total_duration));
        } else {
            advance_group_elapsed(
                group,
                elapsed,
                total_duration,
                &mut loop_count,
                finished_scripts,
            );
        }

        sync_animation_elapsed(group);
        let pending_alpha = group_current_alpha(group, group.elapsed);
        let frame_id = animation_group_frame_id(group);
        let restore_saved_alpha = if !group.playing && group.done && !group.set_to_final_alpha {
            group.saved_alphas.get(&owner_id).copied()
        } else {
            None
        };
        (pending_alpha, restore_saved_alpha, frame_id)
    };

    Some(AnimationGroupAdvance {
        owner_id,
        parent_effective_alpha,
        pending_alpha,
        restore_saved_alpha,
        loop_count,
        frame_id,
    })
}

fn advance_group_elapsed(
    group: &mut crate::lua_api::animation::AnimGroupState,
    elapsed: f64,
    total_duration: f64,
    loop_count: &mut u32,
    finished_scripts: &mut Vec<u64>,
) {
    let advance = elapsed * group.speed_multiplier.max(0.0);
    group.elapsed += advance;

    if total_duration <= 0.0 {
        finished_scripts.push(finish_group_now(group, total_duration));
        return;
    }

    match group.looping {
        crate::lua_api::animation::LoopType::None => {
            if group.elapsed >= total_duration {
                finished_scripts.push(finish_group_now(group, total_duration));
            }
        }
        crate::lua_api::animation::LoopType::Repeat => {
            while group.elapsed >= total_duration {
                group.elapsed -= total_duration;
                *loop_count += 1;
            }
        }
        crate::lua_api::animation::LoopType::Bounce => {
            while group.elapsed >= total_duration {
                group.elapsed -= total_duration;
                group.reverse = !group.reverse;
                *loop_count += 1;
            }
        }
    }
}

fn apply_animation_group_outcome(
    sim: &mut crate::lua_api::state::SimState,
    result: &AnimationGroupAdvance,
) {
    let mut alpha_changed = false;
    if let Some(alpha) = result.pending_alpha
        && let Some(frame) = sim.widgets.get_mut_visual(result.owner_id)
        && (frame.alpha as f64 - alpha).abs() > f32::EPSILON as f64
    {
        frame.alpha = alpha as f32;
        alpha_changed = true;
    }

    if let Some(saved_alpha) = result.restore_saved_alpha
        && let Some(frame) = sim.widgets.get_mut_visual(result.owner_id)
        && (frame.alpha - saved_alpha).abs() > f32::EPSILON
    {
        frame.alpha = saved_alpha;
        alpha_changed = true;
    }

    if alpha_changed {
        sim.widgets
            .propagate_effective_alpha(result.owner_id, result.parent_effective_alpha);
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

fn group_current_alpha(
    group: &crate::lua_api::animation::AnimGroupState,
    elapsed: f64,
) -> Option<f64> {
    use std::collections::BTreeMap;

    let mut groups: BTreeMap<u32, Vec<&crate::lua_api::animation::AnimState>> = BTreeMap::new();
    for animation in &group.animations {
        groups.entry(animation.order).or_default().push(animation);
    }
    let mut remaining = elapsed;
    for (_order, anims) in groups {
        let order_duration = anims
            .iter()
            .map(|anim| anim.total_time())
            .fold(0.0, f64::max);
        let within_order = remaining.min(order_duration);
        let mut current = None;
        for anim in anims {
            if let Some(alpha) = current_animation_alpha(anim, within_order) {
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
