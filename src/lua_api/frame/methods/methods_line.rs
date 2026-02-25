//! Line-specific methods: SetStartPoint, SetEndPoint, SetThickness, and getters.

use super::super::handle::FrameRef;
use crate::lua_api::frame::handle::{extract_frame_id, frame_ref, get_sim_state};
use crate::widget::{AnchorPoint, LineAnchor};
use mlua::Value;

pub fn add_line_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetStartPoint", |lua, this, args: mlua::MultiValue| {
        set_line_point(lua, this.0, args, true)
    });

    methods.add_method("SetEndPoint", |lua, this, args: mlua::MultiValue| {
        set_line_point(lua, this.0, args, false)
    });

    methods.add_method("SetThickness", |lua, this, thickness: f32| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(f) = state.widgets.get_mut_visual(this.0) {
            f.line_thickness = thickness;
        }
        Ok(())
    });

    methods.add_method("GetStartPoint", |lua, this, ()| {
        get_line_point(lua, this.0, true)
    });

    methods.add_method("GetEndPoint", |lua, this, ()| {
        get_line_point(lua, this.0, false)
    });

    methods.add_method("GetThickness", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let thickness = state.widgets.get(this.0).map_or(1.0, |f| f.line_thickness);
        Ok(thickness)
    });
}

fn set_line_point(lua: &mlua::Lua, id: u64, args: mlua::MultiValue, is_start: bool) -> mlua::Result<()> {
    let args: Vec<Value> = args.into_iter().collect();

    let point_str = match args.first() {
        Some(Value::String(s)) => s.to_string_lossy().to_string(),
        _ => return Ok(()),
    };
    let point = AnchorPoint::from_str(&point_str).unwrap_or(AnchorPoint::Center);

    let target_id = args.get(1).and_then(extract_frame_id);

    let x_offset = match args.get(2) {
        Some(Value::Number(n)) => *n as f32,
        Some(Value::Integer(n)) => *n as f32,
        _ => 0.0,
    };
    let y_offset = match args.get(3) {
        Some(Value::Number(n)) => *n as f32,
        Some(Value::Integer(n)) => *n as f32,
        _ => 0.0,
    };

    let anchor = LineAnchor { point, target_id, x_offset, y_offset };

    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    if let Some(f) = state.widgets.get_mut_visual(id) {
        if is_start {
            f.line_start = Some(anchor);
        } else {
            f.line_end = Some(anchor);
        }
    }
    Ok(())
}

fn get_line_point(lua: &mlua::Lua, id: u64, is_start: bool) -> mlua::Result<mlua::MultiValue> {
    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    let anchor = state.widgets.get(id).and_then(|f| {
        if is_start { f.line_start.as_ref() } else { f.line_end.as_ref() }
    });

    let Some(anchor) = anchor else {
        return Ok(mlua::MultiValue::new());
    };

    let point_str = lua.create_string(anchor.point.as_str())?;
    let target: Value = if let Some(tid) = anchor.target_id {
        frame_ref(lua, tid)?
    } else {
        Value::Nil
    };

    Ok(mlua::MultiValue::from_iter([
        Value::String(point_str),
        target,
        Value::Number(anchor.x_offset as f64),
        Value::Number(anchor.y_offset as f64),
    ]))
}
