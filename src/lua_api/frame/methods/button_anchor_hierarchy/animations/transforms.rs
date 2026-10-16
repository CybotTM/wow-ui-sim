//! Supplied transform configuration. No Scale/Rotation rendering is implied.
use super::fields::with_animation_state_mut;
use crate::lua_api::animation::{AnimState, AnimationType};
use crate::lua_api::methods::{borrow_state, create_string, frame_id_from_stack};
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::closure::RustFn;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val, runtime_error};

const TRANSLATION_METHODS: &[(&str, RustFn)] =
    &[("SetOffset", set_offset), ("GetOffset", get_offset)];
const SCALE_METHODS: &[(&str, RustFn)] = &[
    ("SetScale", set_scale),
    ("GetScale", get_scale),
    ("SetScaleFrom", set_scale_from),
    ("GetScaleFrom", get_scale_from),
    ("SetScaleTo", set_scale_to),
    ("GetScaleTo", get_scale_to),
    ("SetOrigin", set_origin),
    ("GetOrigin", get_origin),
];
const ROTATION_METHODS: &[(&str, RustFn)] = &[
    ("SetDegrees", set_degrees),
    ("GetDegrees", get_degrees),
    ("SetRadians", set_radians),
    ("GetRadians", get_radians),
    ("SetOrigin", set_origin),
    ("GetOrigin", get_origin),
];

pub(super) fn attach_transform_methods(
    state: &mut LuaState,
    frame: Val,
    anim_type: AnimationType,
) -> LuaResult<()> {
    let methods = match anim_type {
        AnimationType::Translation => TRANSLATION_METHODS,
        AnimationType::Scale => SCALE_METHODS,
        AnimationType::Rotation => ROTATION_METHODS,
        _ => return Ok(()),
    };
    let Val::Table(table) = frame else {
        return Err(runtime_error("animation reference must be a table"));
    };
    for (name, function) in methods {
        table_set_rust_fn_static(state, table, name, *function)?;
    }
    Ok(())
}

fn read_animation<R>(state: &LuaState, select: impl FnOnce(&AnimState) -> R) -> LuaResult<R> {
    let frame_id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let animation = sim
        .anim_frame_to_anim
        .get(&frame_id)
        .and_then(|(group_id, index)| sim.animation_groups.get(group_id)?.animations.get(*index))
        .ok_or_else(|| runtime_error("expected an animation"))?;
    Ok(select(animation))
}

fn set_pair(
    state: &mut LuaState,
    select: impl FnOnce(&mut AnimState) -> &mut [f64; 2],
) -> LuaResult<u32> {
    let pair = [f64::from_stack(state, 2)?, f64::from_stack(state, 3)?];
    with_animation_state_mut(state, |animation| *select(animation) = pair)?;
    Ok(0)
}

fn push_pair(state: &mut LuaState, select: impl FnOnce(&AnimState) -> [f64; 2]) -> LuaResult<u32> {
    let pair = read_animation(state, select)?;
    for value in pair {
        state.push(Val::Num(value));
    }
    Ok(2)
}

fn set_offset(state: &mut LuaState) -> LuaResult<u32> {
    set_pair(state, |animation| &mut animation.translation_offset)
}

fn get_offset(state: &mut LuaState) -> LuaResult<u32> {
    push_pair(state, |animation| animation.translation_offset)
}

fn set_scale(state: &mut LuaState) -> LuaResult<u32> {
    set_pair(state, |animation| &mut animation.scale)
}

fn get_scale(state: &mut LuaState) -> LuaResult<u32> {
    push_pair(state, |animation| animation.scale)
}

fn set_scale_from(state: &mut LuaState) -> LuaResult<u32> {
    set_pair(state, |animation| &mut animation.scale_from)
}

fn get_scale_from(state: &mut LuaState) -> LuaResult<u32> {
    push_pair(state, |animation| animation.scale_from)
}

fn set_scale_to(state: &mut LuaState) -> LuaResult<u32> {
    set_pair(state, |animation| &mut animation.scale_to)
}

fn get_scale_to(state: &mut LuaState) -> LuaResult<u32> {
    push_pair(state, |animation| animation.scale_to)
}

fn set_degrees(state: &mut LuaState) -> LuaResult<u32> {
    let degrees = f64::from_stack(state, 2)?;
    with_animation_state_mut(state, |animation| animation.rotation_degrees = degrees)?;
    Ok(0)
}

fn set_radians(state: &mut LuaState) -> LuaResult<u32> {
    let radians = f64::from_stack(state, 2)?;
    with_animation_state_mut(state, |animation| {
        animation.rotation_degrees = radians.to_degrees()
    })?;
    Ok(0)
}

fn get_degrees(state: &mut LuaState) -> LuaResult<u32> {
    let degrees = read_animation(state, |animation| animation.rotation_degrees)?;
    state.push(Val::Num(degrees));
    Ok(1)
}

fn get_radians(state: &mut LuaState) -> LuaResult<u32> {
    let degrees = read_animation(state, |animation| animation.rotation_degrees)?;
    state.push(Val::Num(degrees.to_radians()));
    Ok(1)
}

fn set_origin(state: &mut LuaState) -> LuaResult<u32> {
    let point = String::from_stack(state, 2)?;
    let offset = [f64::from_stack(state, 3)?, f64::from_stack(state, 4)?];
    with_animation_state_mut(state, |animation| {
        animation.origin_point = point;
        animation.origin_offset = offset;
    })?;
    Ok(0)
}

fn get_origin(state: &mut LuaState) -> LuaResult<u32> {
    let (point, [x, y]) = read_animation(state, |animation| {
        (animation.origin_point.clone(), animation.origin_offset)
    })?;
    let point = create_string(state, &point);
    state.push(point);
    state.push(Val::Num(x));
    state.push(Val::Num(y));
    Ok(3)
}
