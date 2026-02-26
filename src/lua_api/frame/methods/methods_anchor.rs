//! Anchor/point methods: SetPoint, ClearAllPoints, SetAllPoints, GetPoint, etc.

use super::super::handle::{extract_frame_id, frame_ref, FrameRef};
use crate::lua_api::frame::handle::get_sim_state;
use crate::lua_api::script_helpers::lua_error;
use mlua::Value;
use std::collections::HashMap;

/// Add anchor/point methods to the frame methods table.
pub fn add_anchor_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_set_point_method(methods);
    add_clear_and_adjust_methods(methods);
    add_set_all_points_method(methods);
    add_get_point_methods(methods);
}

/// Helper to extract numeric value from Value (handles both Number and Integer).
fn get_number(v: &Value) -> Option<f32> {
    match v {
        Value::Number(n) => Some(*n as f32),
        Value::Integer(n) => Some(*n as f32),
        _ => None,
    }
}

/// Helper to extract frame ID from Value.
///
/// Handles UserData (FrameRef) and String (global name lookup via `_G`),
/// matching WoW's SetPoint behavior where string frame names are resolved
/// to the corresponding frame object.
fn get_frame_id(lua: &mlua::Lua, v: &Value) -> Option<usize> {
    match v {
        ref v @ Value::UserData(_) => {
            extract_frame_id(v).map(|id| id as usize)
        }
        Value::String(s) => {
            let name = s.to_string_lossy();
            if let Ok(val) = lua.globals().get::<Value>(name.as_str()) {
                return extract_frame_id(&val).map(|id| id as usize);
            }
            None
        }
        _ => None,
    }
}

/// Resolve a relative point from an optional Value, defaulting to the main point.
/// Returns Err(raw_string) if the value is a string that doesn't match a valid point.
fn resolve_relative_point(
    v: Option<&Value>,
    default: crate::widget::AnchorPoint,
) -> Result<crate::widget::AnchorPoint, String> {
    match v {
        Some(Value::String(s)) => {
            let s = s.to_string_lossy();
            crate::widget::AnchorPoint::from_str(&s)
                .ok_or_else(|| format!("Frame:SetPoint(): Unknown region point {s}"))
        }
        _ => Ok(default),
    }
}

/// Extract an anchor point string from a Value, defaulting to "CENTER".
fn extract_point_str(v: Option<&Value>) -> String {
    v.and_then(|v| {
        if let Value::String(s) = v {
            Some(s.to_string_lossy().to_string())
        } else {
            None
        }
    })
    .unwrap_or_else(|| "CENTER".to_string())
}

/// Parse variable SetPoint arguments into (relative_to, relative_point, x_ofs, y_ofs).
/// Returns Err(msg) for invalid relative point names.
/// Returns (relative_to, relative_point, x_ofs, y_ofs, explicit_relative).
/// `explicit_relative` is true when the caller provided a relativeTo argument
/// (even if nil). When false, the caller should resolve to parent.
fn parse_set_point_args(
    lua: &mlua::Lua,
    args: &[Value],
    point: crate::widget::AnchorPoint,
) -> Result<(Option<usize>, crate::widget::AnchorPoint, f32, f32, bool), String> {
    match args.len() {
        1 => Ok((None, point, 0.0, 0.0, false)),
        2 | 3 => parse_set_point_2_or_3(lua, args, point),
        _ => parse_set_point_full(lua, args, point),
    }
}

/// Parse SetPoint with 2 or 3 arguments (after the point name).
fn parse_set_point_2_or_3(
    lua: &mlua::Lua,
    args: &[Value],
    point: crate::widget::AnchorPoint,
) -> Result<(Option<usize>, crate::widget::AnchorPoint, f32, f32, bool), String> {
    let x = args.get(1).and_then(get_number);
    let y = args.get(2).and_then(get_number);
    if let (Some(x), Some(y)) = (x, y) {
        // SetPoint("point", x, y) — no explicit relativeTo, resolve to parent
        Ok((None, point, x, y, false))
    } else {
        // Explicit relativeTo argument (could be nil, frame, or string)
        let rel_to = args.get(1).and_then(|v| get_frame_id(lua, v));
        let rel_point = resolve_relative_point(args.get(2), point)?;
        Ok((rel_to, rel_point, 0.0, 0.0, true))
    }
}

/// Parse SetPoint with 4+ arguments (full form with relativeTo, relativePoint, x, y).
fn parse_set_point_full(
    lua: &mlua::Lua,
    args: &[Value],
    point: crate::widget::AnchorPoint,
) -> Result<(Option<usize>, crate::widget::AnchorPoint, f32, f32, bool), String> {
    let rel_to = args.get(1).and_then(|v| get_frame_id(lua, v));
    let rel_point = resolve_relative_point(args.get(2), point)?;
    let x = args.get(3).and_then(get_number).unwrap_or(0.0);
    let y = args.get(4).and_then(get_number).unwrap_or(0.0);
    Ok((rel_to, rel_point, x, y, true))
}

/// SetPoint(point, relativeTo, relativePoint, xOfs, yOfs)
fn add_set_point_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetPoint", |lua, this, args: mlua::MultiValue| {
        let id = this.0;
        let args: Vec<Value> = args.into_iter().collect();
        if args.is_empty() {
            return Err(lua_error(lua,
                "Frame:SetPoint(): Usage: (\"point\" [, region or nil] [, \"relativePoint\"] [, offsetX, offsetY]"
            ));
        }
        let point_str = extract_point_str(args.first());
        let point = match crate::widget::AnchorPoint::from_str(&point_str) {
            Some(p) => p,
            None => return Err(lua_error(lua,
                format!("Frame:SetPoint(): Invalid region point {point_str}")
            )),
        };
        let (mut relative_to, relative_point, x_ofs, y_ofs, explicit_relative) =
            parse_set_point_args(lua, &args, point)
                .map_err(|msg| lua_error(lua, msg))?;

        if !explicit_relative && relative_to.is_none() {
            let state_rc = get_sim_state(lua);
            let state = state_rc.borrow();
            if let Some(frame) = state.widgets.get(id) {
                relative_to = frame.parent_id.map(|pid| pid as usize);
            }
        }

        let state_rc = get_sim_state(lua);
        check_anchor_cycle(lua, &state_rc.borrow(), id, relative_to, "Frame:SetPoint")?;
        if is_duplicate_anchor(&state_rc.borrow(), id, relative_to, point, relative_point, x_ofs, y_ofs) {
            return Ok(());
        }
        apply_set_point(&state_rc, id, point, relative_to, relative_point, x_ofs, y_ofs);
        Ok(())
    });
}

/// Raise a Lua error if anchoring `id` to `relative_to` would create a cycle.
fn check_anchor_cycle(
    lua: &mlua::Lua,
    state: &crate::lua_api::SimState,
    id: u64,
    relative_to: Option<usize>,
    action: &str,
) -> mlua::Result<()> {
    let Some(rel_id) = relative_to else { return Ok(()) };
    let rel_id = rel_id as u64;
    if rel_id == id {
        return Err(lua_error(lua, format!(
            "Action[SetPoint] failed because[Cannot anchor to itself]: attempted from: {action}."
        )));
    }
    if let Some((x, seen)) = find_cycle_node(state, id, rel_id) {
        return Err(build_cycle_error(lua, action, rel_id, x, &seen));
    }
    Ok(())
}

/// BFS from rel_id looking for any node that anchors back to `id`.
/// Returns (dependent_node, seen_map) if a cycle is found.
fn find_cycle_node(
    state: &crate::lua_api::SimState,
    id: u64,
    rel_id: u64,
) -> Option<(u64, HashMap<u64, u64>)> {
    let mut stack: Vec<u64> = vec![rel_id];
    let mut seen: HashMap<u64, u64> = HashMap::new();
    while let Some(x) = stack.pop() {
        if let Some(frame) = state.widgets.get(x) {
            for anchor in &frame.anchors {
                if let Some(anchor_target) = anchor.relative_to_id {
                    let y = anchor_target as u64;
                    if y == id {
                        return Some((x, seen));
                    } else if !seen.contains_key(&y) {
                        seen.insert(y, x);
                        stack.push(y);
                    }
                }
            }
        }
    }
    None
}

/// Format a frame ID for error messages like WoW: 8-digit uppercase hex.
///
/// WoW's `tostring(frame)` returns `"Type: 0xHHHHHHHH"` and error messages
/// use the hex ID extracted via `rstr()` in test code.
fn frame_label(_lua: &mlua::Lua, id: u64) -> String {
    format!("{:08X}", id)
}

/// Build the cycle error message including the ancestor chain.
fn build_cycle_error(
    lua: &mlua::Lua,
    action: &str,
    rel_id: u64,
    x: u64,
    seen: &HashMap<u64, u64>,
) -> mlua::Error {
    let mut anc: Vec<String> = Vec::new();
    let mut z = seen.get(&x).copied();
    while let Some(ancestor) = z {
        anc.push(format!("[{}]", frame_label(lua, ancestor)));
        z = seen.get(&ancestor).copied();
    }
    let rel = frame_label(lua, rel_id);
    let dep = frame_label(lua, x);
    let base = format!(
        "Action[SetPoint] failed because[Cannot anchor to a region dependent on it]: \
attempted from: {action}.\nRelative: [{rel}]\nDependent: [{dep}]"
    );
    let extra = if anc.is_empty() {
        String::new()
    } else {
        format!("\nDependent ancestors:\n{}", anc.join("\n"))
    };
    lua_error(lua, format!("{base}{extra}"))
}

/// Check if the anchor already matches (skip redundant updates).
fn is_duplicate_anchor(
    state: &crate::lua_api::SimState,
    id: u64,
    relative_to: Option<usize>,
    point: crate::widget::AnchorPoint,
    relative_point: crate::widget::AnchorPoint,
    x_ofs: f32,
    y_ofs: f32,
) -> bool {
    if let Some(frame) = state.widgets.get(id)
        && let Some(existing) = frame.anchors.iter().find(|a| a.point == point)
        && existing.relative_to_id == relative_to
        && existing.relative_point == relative_point
        && existing.x_offset == x_ofs
        && existing.y_offset == y_ofs
    {
        return true;
    }
    false
}

/// Apply the SetPoint mutation to the widget state.
fn apply_set_point(
    state_rc: &std::cell::RefCell<crate::lua_api::SimState>,
    id: u64,
    point: crate::widget::AnchorPoint,
    relative_to: Option<usize>,
    relative_point: crate::widget::AnchorPoint,
    x_ofs: f32,
    y_ofs: f32,
) {
    let mut state = state_rc.borrow_mut();
    if let Some(frame) = state.widgets.get(id) {
        if let Some(old) = frame.anchors.iter().find(|a| a.point == point) {
            if let Some(old_target) = old.relative_to_id {
                state.widgets.remove_anchor_dependent(old_target as u64, id);
            }
        }
    }
    if let Some(rel_id) = relative_to {
        state.widgets.add_anchor_dependent(rel_id as u64, id);
    }
    if let Some(frame) = state.widgets.get_mut_visual(id) {
        frame.set_point(point, relative_to, relative_point, x_ofs, y_ofs);
    }
    state.widgets.mark_rect_dirty(id);
    state.invalidate_layout_with_dependents(id);
}

fn add_clear_and_adjust_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_clear_all_points(methods);
    add_clear_point(methods);
    add_adjust_points(methods);
}

fn add_clear_all_points<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("ClearAllPoints", |lua, this, ()| {
        let id = this.0;
        let state_rc = get_sim_state(lua);
        let already_empty = state_rc.borrow().widgets.get(id)
            .map(|f| f.anchors.is_empty()).unwrap_or(true);
        if !already_empty {
            let mut state = state_rc.borrow_mut();
            state.widgets.remove_all_anchor_dependents_for(id);
            if let Some(frame) = state.widgets.get_mut_visual(id) {
                frame.clear_all_points();
            }
            state.widgets.mark_rect_dirty(id);
            state.invalidate_layout(id);
        }
        Ok(())
    });
}

fn add_clear_point<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("ClearPoint", |lua, this, point_name: String| {
        let id = this.0;
        let point = crate::widget::AnchorPoint::from_str(&point_name);
        if let Some(point) = point {
            let state_rc = get_sim_state(lua);
            let mut state = state_rc.borrow_mut();
            if let Some(frame) = state.widgets.get(id) {
                if let Some(anchor) = frame.anchors.iter().find(|a| a.point == point) {
                    if let Some(target) = anchor.relative_to_id {
                        state.widgets.remove_anchor_dependent(target as u64, id);
                    }
                }
            }
            if let Some(frame) = state.widgets.get_mut_visual(id) {
                frame.anchors.retain(|a| a.point != point);
            }
            state.widgets.mark_rect_dirty(id);
            state.invalidate_layout(id);
        }
        Ok(())
    });
}

fn add_adjust_points<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("ClearPointsOffset", |_lua, _this, ()| Ok(()));

    methods.add_method("AdjustPointsOffset", |lua, this, (x_offset, y_offset): (f32, f32)| {
        let id = this.0;
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(id) {
            for anchor in &mut frame.anchors {
                anchor.x_offset += x_offset;
                anchor.y_offset += y_offset;
            }
        }
        state.widgets.mark_rect_dirty(id);
        state.invalidate_layout(id);
        Ok(())
    });
}

/// SetAllPoints(relativeTo) - sets TOPLEFT and BOTTOMRIGHT to fill a relative frame.
fn add_set_all_points_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetAllPoints", |lua, this, arg: mlua::MultiValue| {
        let id = this.0;
        let first = arg.get(0).cloned().unwrap_or(Value::Nil);
        let has_arg = !arg.is_empty();
        let (should_set, relative_to_id) = resolve_set_all_points_target(lua, id, &first, has_arg);
        if should_set {
            let state_rc = get_sim_state(lua);
            check_anchor_cycle(lua, &state_rc.borrow(), id, relative_to_id, "Frame:SetAllPoints")?;
            apply_set_all_points(&state_rc, id, relative_to_id);
        }
        Ok(())
    });
}

/// Determine the (should_set, relative_to_id) for SetAllPoints.
fn resolve_set_all_points_target(
    lua: &mlua::Lua,
    id: u64,
    first: &Value,
    has_arg: bool,
) -> (bool, Option<usize>) {
    match first {
        Value::Boolean(false) => (false, None),
        Value::Boolean(true) => {
            let state_rc = get_sim_state(lua);
            let state = state_rc.borrow();
            let parent_id = state.widgets.get(id).and_then(|f| f.parent_id).map(|p| p as usize);
            (true, parent_id)
        }
        ref v @ Value::UserData(_) => {
            (true, extract_frame_id(v).map(|id| id as usize))
        }
        _ if has_arg => (true, None),
        _ => {
            let state_rc = get_sim_state(lua);
            let state = state_rc.borrow();
            let parent_id = state.widgets.get(id).and_then(|f| f.parent_id).map(|p| p as usize);
            (true, parent_id)
        }
    }
}

/// Apply SetAllPoints mutation: clear anchors and set TOPLEFT + BOTTOMRIGHT.
fn apply_set_all_points(
    state_rc: &std::cell::RefCell<crate::lua_api::SimState>,
    id: u64,
    relative_to_id: Option<usize>,
) {
    let mut state = state_rc.borrow_mut();
    state.widgets.remove_all_anchor_dependents_for(id);
    if let Some(rel_id) = relative_to_id {
        state.widgets.add_anchor_dependent(rel_id as u64, id);
    }
    if let Some(frame) = state.widgets.get_mut_visual(id) {
        frame.clear_all_points();
        frame.set_point(
            crate::widget::AnchorPoint::TopLeft,
            relative_to_id,
            crate::widget::AnchorPoint::TopLeft,
            0.0, 0.0,
        );
        frame.set_point(
            crate::widget::AnchorPoint::BottomRight,
            relative_to_id,
            crate::widget::AnchorPoint::BottomRight,
            0.0, 0.0,
        );
    }
    state.widgets.mark_rect_dirty(id);
    state.invalidate_layout(id);
}

fn add_get_point_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_get_point(methods);
    add_get_num_points(methods);
    add_get_point_by_name(methods);
}

/// GetPoint(index) - return anchor details at the given 1-based index.
fn add_get_point<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetPoint", |lua, this, index: Option<i32>| {
        let id = this.0;
        let idx = (index.unwrap_or(1) - 1) as usize;
        let anchor_data = {
            let state_rc = get_sim_state(lua);
            let state = state_rc.borrow();
            let Some(frame) = state.widgets.get(id) else {
                return Ok(mlua::MultiValue::new());
            };
            let mut sorted: Vec<_> = frame.anchors.iter().collect();
            sorted.sort_by_key(|a| a.point.sort_key());
            let Some(anchor) = sorted.get(idx) else {
                return Ok(mlua::MultiValue::new());
            };
            (anchor.point, anchor.relative_to_id, anchor.relative_point, anchor.x_offset, anchor.y_offset)
        };
        let (point, relative_to_id, relative_point, x_offset, y_offset) = anchor_data;
        let relative_to = match relative_to_id {
            Some(rid) => frame_ref(lua, rid as u64)?,
            None => Value::Nil,
        };
        Ok(mlua::MultiValue::from_vec(vec![
            Value::String(lua.create_string(point.as_str())?),
            relative_to,
            Value::String(lua.create_string(relative_point.as_str())?),
            Value::Number(x_offset as f64),
            Value::Number(y_offset as f64),
        ]))
    });
}

fn add_get_num_points<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetNumPoints", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let count = state.widgets.get(this.0).map(|f| f.anchors.len()).unwrap_or(0);
        Ok(count as i32)
    });
}

fn add_get_point_by_name<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetPointByName", |lua, this, point_name: String| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let Some(frame) = state.widgets.get(this.0) else {
            return Ok(mlua::MultiValue::new());
        };
        let point_upper = point_name.to_uppercase();
        for anchor in &frame.anchors {
            if anchor.point.as_str().to_uppercase() == point_upper {
                return Ok(mlua::MultiValue::from_vec(vec![
                    Value::String(lua.create_string(anchor.point.as_str())?),
                    Value::Nil,
                    Value::String(lua.create_string(anchor.relative_point.as_str())?),
                    Value::Number(anchor.x_offset as f64),
                    Value::Number(anchor.y_offset as f64),
                ]));
            }
        }
        Ok(mlua::MultiValue::new())
    });
}
