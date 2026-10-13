//! Supplied Path points. Ordering and NONE are explicit simulator policies.
use crate::lua_api::animation::{AnimState, AnimationType, ControlPointState};
use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, create_string, frame_id_from_stack, frame_ref,
};
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use crate::widget::{Frame, WidgetType};
use rilua::vm::closure::RustFn;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val, runtime_error};

pub(super) fn attach_path_methods(state: &mut LuaState, frame: Val) -> LuaResult<()> {
    attach_methods(
        state,
        frame,
        &[
            ("CreateControlPoint", create_control_point),
            ("GetControlPoints", get_control_points),
            ("GetMaxControlPointOrder", get_max_control_point_order),
            ("SetCurveType", set_curve_type),
            ("GetCurveType", get_curve_type),
        ],
    )
}

fn attach_control_point_methods(state: &mut LuaState, frame: Val) -> LuaResult<()> {
    attach_methods(
        state,
        frame,
        &[
            ("SetOrder", set_point_order),
            ("GetOrder", get_point_order),
            ("SetOffset", set_point_offset),
            ("GetOffset", get_point_offset),
        ],
    )
}

fn attach_methods(
    state: &mut LuaState,
    frame: Val,
    methods: &[(&'static str, RustFn)],
) -> LuaResult<()> {
    let Val::Table(table) = frame else {
        return Err(runtime_error("animation reference must be a table"));
    };
    for (name, function) in methods {
        table_set_rust_fn_static(state, table, name, *function)?;
    }
    Ok(())
}

fn find_path(sim: &crate::lua_api::SimState, frame_id: u64) -> LuaResult<&AnimState> {
    let (group_id, index) = sim
        .anim_frame_to_anim
        .get(&frame_id)
        .ok_or_else(|| runtime_error("expected a Path animation"))?;
    let animation = sim
        .animation_groups
        .get(group_id)
        .and_then(|group| group.animations.get(*index))
        .filter(|animation| animation.anim_type == AnimationType::Path)
        .ok_or_else(|| runtime_error("expected a Path animation"))?;
    Ok(animation)
}

fn mutate_path(
    state: &LuaState,
    frame_id: u64,
    update: impl FnOnce(&mut AnimState) -> LuaResult<()>,
) -> LuaResult<()> {
    let mut sim = borrow_state_mut(state)?;
    find_path(&sim, frame_id)?;
    let (group_id, index) = sim.anim_frame_to_anim[&frame_id];
    let group = sim
        .animation_groups
        .get_mut(&group_id)
        .ok_or_else(|| runtime_error("Path group not found"))?;
    update(&mut group.animations[index])
}

/// CreateControlPoint([name [, templateName [, order]]]). Templates are out of scope.
fn create_control_point(state: &mut LuaState) -> LuaResult<u32> {
    let parent_id = frame_id_from_stack(state, 1)?;
    let name_raw = Option::<String>::from_stack(state, 2)?;
    let order = Option::<u32>::from_stack(state, 4)?.unwrap_or(1);
    let name = super::super::font_strings::resolve_child_name(state, name_raw, parent_id);
    let mut child = Frame::new(WidgetType::Frame, name.clone(), Some(parent_id));
    child.object_type_name = Some("ControlPoint".to_string());
    let child_id = child.id;
    mutate_path(state, parent_id, |animation| {
        animation.control_points.push(ControlPointState {
            frame_id: child_id,
            order,
            offset: [0.0; 2],
        });
        Ok(())
    })?;
    register_control_point(state, parent_id, child)?;
    if let Some(name) = &name {
        super::super::shared::bind_named_child_global(state, name, child_id)?;
    }
    let frame = frame_ref(state, child_id)?;
    attach_control_point_methods(state, frame)?;
    state.push(frame);
    Ok(1)
}

fn register_control_point(state: &LuaState, parent_id: u64, child: Frame) -> LuaResult<()> {
    let child_id = child.id;
    let mut sim = borrow_state_mut(state)?;
    sim.widgets.register(child);
    sim.widgets.add_child(parent_id, child_id);
    sim.invalidate_strata_buckets();
    Ok(())
}

fn get_control_points(state: &mut LuaState) -> LuaResult<u32> {
    let frame_id = frame_id_from_stack(state, 1)?;
    let ids: Vec<_> = {
        let sim = borrow_state(state)?;
        find_path(&sim, frame_id)?
            .ordered_control_points()
            .iter()
            .map(|point| point.frame_id)
            .collect()
    };
    let count = ids.len() as u32;
    for id in ids {
        let frame = frame_ref(state, id)?;
        state.push(frame);
    }
    Ok(count)
}

fn get_max_control_point_order(state: &mut LuaState) -> LuaResult<u32> {
    let frame_id = frame_id_from_stack(state, 1)?;
    let order = {
        let sim = borrow_state(state)?;
        find_path(&sim, frame_id)?
            .control_points
            .iter()
            .map(|point| point.order)
            .max()
            .unwrap_or(0)
    };
    state.push(Val::Num(order as f64));
    Ok(1)
}

fn set_curve_type(state: &mut LuaState) -> LuaResult<u32> {
    let frame_id = frame_id_from_stack(state, 1)?;
    let curve = String::from_stack(state, 2)?;
    if curve != "NONE" {
        return Err(runtime_error("only Path curve NONE is supported"));
    }
    mutate_path(state, frame_id, |animation| {
        animation.curve = curve;
        Ok(())
    })?;
    Ok(0)
}

fn get_curve_type(state: &mut LuaState) -> LuaResult<u32> {
    let frame_id = frame_id_from_stack(state, 1)?;
    let curve = {
        let sim = borrow_state(state)?;
        find_path(&sim, frame_id)?.curve.clone()
    };
    let value = create_string(state, &curve);
    state.push(value);
    Ok(1)
}

fn read_point(state: &LuaState, frame_id: u64) -> LuaResult<ControlPointState> {
    let sim = borrow_state(state)?;
    let parent_id = sim
        .widgets
        .get(frame_id)
        .and_then(|frame| frame.parent_id)
        .ok_or_else(|| runtime_error("ControlPoint parent not found"))?;
    find_path(&sim, parent_id)?
        .control_points
        .iter()
        .find(|point| point.frame_id == frame_id)
        .cloned()
        .ok_or_else(|| runtime_error("ControlPoint not found"))
}

fn mutate_point(
    state: &LuaState,
    frame_id: u64,
    update: impl FnOnce(&mut ControlPointState),
) -> LuaResult<()> {
    let parent_id = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(frame_id)
            .and_then(|frame| frame.parent_id)
            .ok_or_else(|| runtime_error("ControlPoint parent not found"))?
    };
    mutate_path(state, parent_id, |animation| {
        let point = animation
            .control_points
            .iter_mut()
            .find(|point| point.frame_id == frame_id)
            .ok_or_else(|| runtime_error("ControlPoint not found"))?;
        update(point);
        Ok(())
    })
}

fn set_point_order(state: &mut LuaState) -> LuaResult<u32> {
    let frame_id = frame_id_from_stack(state, 1)?;
    let order = u32::from_stack(state, 2)?;
    mutate_point(state, frame_id, |point| point.order = order)?;
    Ok(0)
}

fn get_point_order(state: &mut LuaState) -> LuaResult<u32> {
    let frame_id = frame_id_from_stack(state, 1)?;
    let point = read_point(state, frame_id)?;
    state.push(Val::Num(point.order as f64));
    Ok(1)
}

fn set_point_offset(state: &mut LuaState) -> LuaResult<u32> {
    let frame_id = frame_id_from_stack(state, 1)?;
    let offset = [f64::from_stack(state, 2)?, f64::from_stack(state, 3)?];
    mutate_point(state, frame_id, |point| point.offset = offset)?;
    Ok(0)
}

fn get_point_offset(state: &mut LuaState) -> LuaResult<u32> {
    let frame_id = frame_id_from_stack(state, 1)?;
    let point = read_point(state, frame_id)?;
    for value in point.offset {
        state.push(Val::Num(value));
    }
    Ok(2)
}
