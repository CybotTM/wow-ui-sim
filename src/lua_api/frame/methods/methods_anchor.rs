//! Anchor/point methods: SetPoint, ClearAllPoints, SetAllPoints, GetPoint, etc.

use crate::lua_api::frame::handle::{extract_frame_id, frame_lud, get_sim_state, lud_to_id};
use crate::lua_api::script_helpers::lua_error;
use mlua::{LightUserData, Lua, Value};
use std::collections::HashMap;

/// Add anchor/point methods to the frame methods table.
pub fn add_anchor_methods(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    add_set_point_method(lua, methods)?;
    add_clear_and_adjust_methods(lua, methods)?;
    add_set_all_points_method(lua, methods)?;
    add_get_point_methods(lua, methods)?;
    Ok(())
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
/// Handles both LightUserData (direct frame reference) and String (global name
/// lookup via `_G`), matching WoW's SetPoint behavior where string frame
/// names are resolved to the corresponding frame object.
fn get_frame_id(lua: &Lua, v: &Value) -> Option<usize> {
    match v {
        Value::LightUserData(lud) => Some(lud_to_id(*lud) as usize),
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
    lua: &Lua,
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
    lua: &Lua,
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
    lua: &Lua,
    args: &[Value],
    point: crate::widget::AnchorPoint,
) -> Result<(Option<usize>, crate::widget::AnchorPoint, f32, f32, bool), String> {
    // Explicit relativeTo argument (could be nil, frame, or string)
    let rel_to = args.get(1).and_then(|v| get_frame_id(lua, v));
    let rel_point = resolve_relative_point(args.get(2), point)?;
    let x = args.get(3).and_then(get_number).unwrap_or(0.0);
    let y = args.get(4).and_then(get_number).unwrap_or(0.0);
    Ok((rel_to, rel_point, x, y, true))
}

/// SetPoint(point, relativeTo, relativePoint, xOfs, yOfs)
fn add_set_point_method(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    methods.set("SetPoint", lua.create_function(|lua, (ud, args): (LightUserData, mlua::MultiValue)| {
        let id = lud_to_id(ud);
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

        // When no explicit relativeTo was given (e.g. SetPoint("CENTER") or
        // SetPoint("CENTER", x, y)), anchor to parent frame. Explicit nil
        // (SetPoint("CENTER", nil, ...)) anchors to the screen.
        if !explicit_relative && relative_to.is_none() {
            let state_rc = get_sim_state(lua);
            let state = state_rc.borrow();
            if let Some(frame) = state.widgets.get(id as u64) {
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
    })?)?;
    Ok(())
}

/// Raise a Lua error if anchoring `id` to `relative_to` would create a cycle.
///
/// Matches wowless error format exactly: self-anchors produce a simple message;
/// indirect cycles include Relative/Dependent frame hex IDs and optional
/// Dependent ancestors list.  Frame hex IDs are `format!("{:x}", id)` because
/// `frame_lud(id)` stores the id as the pointer value, so Lua's `tostring()`
/// gives `"userdata: 0x{id:x}"` and `rstr()` extracts the part after `0x`.
fn check_anchor_cycle(
    lua: &Lua,
    state: &crate::lua_api::SimState,
    id: u64,
    relative_to: Option<usize>,
    action: &str,
) -> mlua::Result<()> {
    let Some(rel_id) = relative_to else { return Ok(()) };
    let rel_id = rel_id as u64;

    // Self-anchor is a special case with a different message.
    if rel_id == id {
        return Err(lua_error(lua, format!(
            "Action[SetPoint] failed because[Cannot anchor to itself]: attempted from: {action}."
        )));
    }

    // BFS (LIFO/stack order, matching wowless) starting from rel_id.
    // For each visited node x, check if any of x's anchor targets == id.
    // seen[y] = x means y was discovered while processing x.
    let mut stack: Vec<u64> = vec![rel_id];
    let mut seen: HashMap<u64, u64> = HashMap::new();

    while let Some(x) = stack.pop() {
        if let Some(frame) = state.widgets.get(x) {
            for anchor in &frame.anchors {
                if let Some(anchor_target) = anchor.relative_to_id {
                    let y = anchor_target as u64;
                    if y == id {
                        // Cycle found: x anchors to id (the frame being anchored).
                        // Build ancestor chain by following seen[] from x back toward rel_id.
                        let mut anc: Vec<String> = Vec::new();
                        let mut z = seen.get(&x).copied();
                        while let Some(ancestor) = z {
                            anc.push(format!("[{ancestor:x}]"));
                            z = seen.get(&ancestor).copied();
                        }
                        let base = format!(
                            "Action[SetPoint] failed because[Cannot anchor to a region dependent on it]: \
attempted from: {action}.\nRelative: [{rel_id:x}]\nDependent: [{x:x}]"
                        );
                        let extra = if anc.is_empty() {
                            String::new()
                        } else {
                            format!("\nDependent ancestors:\n{}", anc.join("\n"))
                        };
                        return Err(lua_error(lua, format!("{base}{extra}")));
                    } else if !seen.contains_key(&y) {
                        seen.insert(y, x);
                        stack.push(y);
                    }
                }
            }
        }
    }

    Ok(())
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
    // Update reverse anchor index: remove old target, add new target
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

/// ClearAllPoints(), ClearPoint(point), ClearPointsOffset(), AdjustPointsOffset(x, y)
fn add_clear_and_adjust_methods(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    methods.set("ClearAllPoints", lua.create_function(|lua, ud: LightUserData| {
        let id = lud_to_id(ud);
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
    })?)?;

    // ClearPoint(point) - remove a specific anchor by point name
    methods.set("ClearPoint", lua.create_function(|lua, (ud, point_name): (LightUserData, String)| {
        let id = lud_to_id(ud);
        let point = crate::widget::AnchorPoint::from_str(&point_name);
        if let Some(point) = point {
            let state_rc = get_sim_state(lua);
            let mut state = state_rc.borrow_mut();
            // Remove reverse index entry for the cleared anchor
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
    })?)?;

    // ClearPointsOffset() - stub
    methods.set("ClearPointsOffset", lua.create_function(
        |_lua, _ud: LightUserData| Ok(()),
    )?)?;

    methods.set("AdjustPointsOffset", lua.create_function(
        |lua, (ud, x_offset, y_offset): (LightUserData, f32, f32)| {
            let id = lud_to_id(ud);
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
        },
    )?)?;

    Ok(())
}

/// SetAllPoints(relativeTo) - sets TOPLEFT and BOTTOMRIGHT to fill a relative frame.
fn add_set_all_points_method(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    methods.set("SetAllPoints", lua.create_function(|lua, (ud, arg): (LightUserData, mlua::MultiValue)| {
        let id = lud_to_id(ud);
        let first = arg.get(0).cloned().unwrap_or(Value::Nil);
        let has_arg = !arg.is_empty();

        let (should_set, relative_to_id) = match &first {
            Value::Boolean(false) => (false, None),
            Value::Boolean(true) => {
                // SetAllPoints(true) → anchor to parent (same as no-argument form).
                let state_rc = get_sim_state(lua);
                let state = state_rc.borrow();
                let frame = state.widgets.get(id);
                let is_default = frame.map(|f| f.default_parent).unwrap_or(true);
                if is_default {
                    (true, None)
                } else {
                    let parent_id = frame.and_then(|f| f.parent_id).map(|p| p as usize);
                    (true, parent_id)
                }
            }
            Value::LightUserData(lud) => (true, Some(lud_to_id(*lud) as usize)),
            _ if has_arg => (true, None), // explicit nil → screen
            _ => {
                // No argument → implicit parent. If the parent was defaulted (not explicitly
                // set by the caller), store None so GetPoint returns nil, matching wowless
                // headless behavior where the default parent is nil.
                // If the parent was explicitly set via SetParent, store the parent's ID.
                let state_rc = get_sim_state(lua);
                let state = state_rc.borrow();
                let frame = state.widgets.get(id);
                let is_default = frame.map(|f| f.default_parent).unwrap_or(true);
                if is_default {
                    (true, None)
                } else {
                    let parent_id = frame.and_then(|f| f.parent_id).map(|p| p as usize);
                    (true, parent_id)
                }
            }
        };

        if should_set {
            let state_rc = get_sim_state(lua);
            check_anchor_cycle(lua, &state_rc.borrow(), id, relative_to_id, "Frame:SetAllPoints")?;
            apply_set_all_points(&state_rc, id, relative_to_id);
        }
        Ok(())
    })?)?;
    Ok(())
}

/// Apply SetAllPoints mutation: clear anchors and set TOPLEFT + BOTTOMRIGHT.
fn apply_set_all_points(
    state_rc: &std::cell::RefCell<crate::lua_api::SimState>,
    id: u64,
    relative_to_id: Option<usize>,
) {
    let mut state = state_rc.borrow_mut();

    // Update reverse index: remove old, add new
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
            0.0,
            0.0,
        );
        frame.set_point(
            crate::widget::AnchorPoint::BottomRight,
            relative_to_id,
            crate::widget::AnchorPoint::BottomRight,
            0.0,
            0.0,
        );
    }
    state.widgets.mark_rect_dirty(id);
    state.invalidate_layout(id);
}

/// GetPoint, GetNumPoints, GetPointByName - querying anchor points.
fn add_get_point_methods(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    add_get_point(lua, methods)?;
    add_get_num_points(lua, methods)?;
    add_get_point_by_name(lua, methods)?;
    Ok(())
}

/// GetPoint(index) - return anchor details at the given 1-based index.
///
/// Anchors are returned sorted by canonical AnchorPoint order (TOPLEFT=0..BOTTOMRIGHT=8),
/// not insertion order. relativeTo is nil when no explicit relative frame was set.
fn add_get_point(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    methods.set("GetPoint", lua.create_function(|lua, (ud, index): (LightUserData, Option<i32>)| {
        let id = lud_to_id(ud);
        let idx = (index.unwrap_or(1) - 1) as usize;
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        if let Some(frame) = state.widgets.get(id) {
            let mut sorted: Vec<_> = frame.anchors.iter().collect();
            sorted.sort_by_key(|a| a.point.sort_key());
            if let Some(anchor) = sorted.get(idx) {
                let relative_to = match anchor.relative_to_id {
                    Some(rid) => frame_lud(rid as u64),
                    None => Value::Nil,
                };
                return Ok(mlua::MultiValue::from_vec(vec![
                    Value::String(lua.create_string(anchor.point.as_str())?),
                    relative_to,
                    Value::String(lua.create_string(anchor.relative_point.as_str())?),
                    Value::Number(anchor.x_offset as f64),
                    Value::Number(anchor.y_offset as f64),
                ]));
            }
        }
        Ok(mlua::MultiValue::new())
    })?)?;
    Ok(())
}

/// GetNumPoints() - return the number of anchors on this frame.
fn add_get_num_points(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    methods.set("GetNumPoints", lua.create_function(|lua, ud: LightUserData| {
        let id = lud_to_id(ud);
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let count = state
            .widgets
            .get(id)
            .map(|f| f.anchors.len())
            .unwrap_or(0);
        Ok(count as i32)
    })?)?;
    Ok(())
}

/// GetPointByName(pointName) - return anchor details by point name string.
fn add_get_point_by_name(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    methods.set("GetPointByName", lua.create_function(
        |lua, (ud, point_name): (LightUserData, String)| {
            let id = lud_to_id(ud);
            let state_rc = get_sim_state(lua);
            let state = state_rc.borrow();
            if let Some(frame) = state.widgets.get(id) {
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
            }
            Ok(mlua::MultiValue::new())
        },
    )?)?;
    Ok(())
}
