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

pub(super) fn animation_group_stop(state: &mut LuaState) -> LuaResult<u32> {
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
    push_anim_field(state, |a| a.duration)
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
