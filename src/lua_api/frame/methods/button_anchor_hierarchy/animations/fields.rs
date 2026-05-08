use super::{animation_group_total_duration, push_group_num};
use crate::lua_api::methods::{borrow_state, borrow_state_mut, frame_id_from_stack};
use crate::lua_bridge::{FromStack, stack_val};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

// ── Animation state setters ───────────────────────────────────────────────────

fn animation_numeric_arg(state: &LuaState, index: i32) -> f64 {
    match stack_val(state, index) {
        Val::Num(value) => value.max(0.0),
        _ => 0.0,
    }
}

pub(super) fn with_animation_state_mut<F>(state: &mut LuaState, f: F) -> LuaResult<()>
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

pub(in crate::lua_api::frame::methods::button_anchor_hierarchy) fn animation_set_duration(
    state: &mut LuaState,
) -> LuaResult<u32> {
    let duration = animation_numeric_arg(state, 2);
    with_animation_state_mut(state, |a| a.duration = duration)?;
    Ok(0)
}

pub(in crate::lua_api::frame::methods::button_anchor_hierarchy) fn animation_get_duration(
    state: &mut LuaState,
) -> LuaResult<u32> {
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

pub(in crate::lua_api::frame::methods::button_anchor_hierarchy) fn animation_set_order(
    state: &mut LuaState,
) -> LuaResult<u32> {
    let order = match stack_val(state, 2) {
        Val::Num(value) if value >= 0.0 => value as u32,
        _ => 0,
    };
    with_animation_state_mut(state, |a| a.order = order)?;
    Ok(0)
}

pub(in crate::lua_api::frame::methods::button_anchor_hierarchy) fn animation_set_start_delay(
    state: &mut LuaState,
) -> LuaResult<u32> {
    let start_delay = animation_numeric_arg(state, 2);
    with_animation_state_mut(state, |a| a.start_delay = start_delay)?;
    Ok(0)
}

pub(in crate::lua_api::frame::methods::button_anchor_hierarchy) fn animation_set_end_delay(
    state: &mut LuaState,
) -> LuaResult<u32> {
    let end_delay = animation_numeric_arg(state, 2);
    with_animation_state_mut(state, |a| a.end_delay = end_delay)?;
    Ok(0)
}

pub(in crate::lua_api::frame::methods::button_anchor_hierarchy) fn set_animation_child_key(
    state: &mut LuaState,
) -> LuaResult<u32> {
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

pub(in crate::lua_api::frame::methods::button_anchor_hierarchy) fn animation_set_flipbook_rows(
    state: &mut LuaState,
) -> LuaResult<u32> {
    let rows = animation_numeric_arg(state, 2) as u32;
    with_animation_state_mut(state, |a| a.flipbook_rows = rows)?;
    Ok(0)
}

pub(in crate::lua_api::frame::methods::button_anchor_hierarchy) fn animation_get_flipbook_rows(
    state: &mut LuaState,
) -> LuaResult<u32> {
    push_anim_field(state, |a| a.flipbook_rows as f64)
}

pub(in crate::lua_api::frame::methods::button_anchor_hierarchy) fn animation_set_flipbook_columns(
    state: &mut LuaState,
) -> LuaResult<u32> {
    let columns = animation_numeric_arg(state, 2) as u32;
    with_animation_state_mut(state, |a| a.flipbook_columns = columns)?;
    Ok(0)
}

pub(in crate::lua_api::frame::methods::button_anchor_hierarchy) fn animation_get_flipbook_columns(
    state: &mut LuaState,
) -> LuaResult<u32> {
    push_anim_field(state, |a| a.flipbook_columns as f64)
}

pub(in crate::lua_api::frame::methods::button_anchor_hierarchy) fn animation_set_flipbook_frames(
    state: &mut LuaState,
) -> LuaResult<u32> {
    let frames = animation_numeric_arg(state, 2) as u32;
    with_animation_state_mut(state, |a| a.flipbook_frames = frames)?;
    Ok(0)
}

pub(in crate::lua_api::frame::methods::button_anchor_hierarchy) fn animation_get_flipbook_frames(
    state: &mut LuaState,
) -> LuaResult<u32> {
    push_anim_field(state, |a| a.flipbook_frames as f64)
}

pub(in crate::lua_api::frame::methods::button_anchor_hierarchy) fn animation_set_flipbook_frame_width(
    state: &mut LuaState,
) -> LuaResult<u32> {
    let width = animation_numeric_arg(state, 2);
    with_animation_state_mut(state, |a| a.flipbook_frame_width = width)?;
    Ok(0)
}

pub(in crate::lua_api::frame::methods::button_anchor_hierarchy) fn animation_get_flipbook_frame_width(
    state: &mut LuaState,
) -> LuaResult<u32> {
    push_anim_field(state, |a| a.flipbook_frame_width)
}

pub(in crate::lua_api::frame::methods::button_anchor_hierarchy) fn animation_set_flipbook_frame_height(
    state: &mut LuaState,
) -> LuaResult<u32> {
    let height = animation_numeric_arg(state, 2);
    with_animation_state_mut(state, |a| a.flipbook_frame_height = height)?;
    Ok(0)
}

pub(in crate::lua_api::frame::methods::button_anchor_hierarchy) fn animation_get_flipbook_frame_height(
    state: &mut LuaState,
) -> LuaResult<u32> {
    push_anim_field(state, |a| a.flipbook_frame_height)
}

pub(super) fn push_anim_field<F>(state: &mut LuaState, f: F) -> LuaResult<u32>
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
