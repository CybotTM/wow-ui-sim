//! Miscellaneous frame-type-specific method stubs (Minimap, ScrollingMessage, Alerts, etc.).

use super::super::handle::FrameRef;
use super::methods_core::lockdown_blocked;
use crate::lua_api::frame::handle::{extract_frame_id, frame_ref, get_sim_state};
use crate::widget::WidgetRegistry;
use mlua::{MultiValue, Value};

/// Add all miscellaneous frame-type-specific methods.
pub fn add_misc_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_minimap_methods(methods);
    add_scrolling_message_methods(methods);
    add_alert_and_data_provider_methods(methods);
    add_drag_stubs(methods);
    add_propagation_stubs(methods);
    add_gamepad_methods(methods);
    add_alpha_gradient_methods(methods);
    add_draw_layer_stubs(methods);
    add_frame_buffer_stubs(methods);
    add_bounds_position_stubs(methods);
    add_attribute_stubs(methods);
    add_frame_level_stubs(methods);
    add_secret_protected_stubs(methods);
    add_flatten_render_methods(methods);
    add_window_display_methods(methods);
    add_misc_stubs(methods);
    add_specialized_frame_stubs(methods);
}

/// Stubs for specialized frame types (QuestPOI, FogOfWar, UnitPosition, etc.).
fn add_specialized_frame_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_menu_frame_stubs(methods);
    add_quest_poi_frame_methods(methods);
    methods.add_method("GetUiMapID", |_, _, ()| Ok(mlua::Value::Nil)); // FogOfWarFrame
    add_quest_blob_methods(methods);
    add_unit_position_frame_stubs(methods);
}

fn add_menu_frame_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("IsMenuOpen", |_, _this, ()| Ok(false));
    methods.add_method("SetOwningDialog", |_, _this, _dialog: Value| Ok(()));
    methods.add_method("RegisterFontStrings", |_, _this, _args: MultiValue| Ok(()));
    methods.add_method("RegisterFrames", |_, _this, _args: MultiValue| Ok(()));
    methods.add_method(
        "RegisterBackgroundTexture",
        |_, _this, _args: MultiValue| Ok(()),
    );
}

fn add_quest_poi_frame_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetFillTexture", |_, _, _: mlua::MultiValue| Ok(()));
    methods.add_method("SetBorderTexture", |_, _, _: mlua::MultiValue| Ok(()));
    methods.add_method("SetFillAlpha", |_, _, _: mlua::MultiValue| Ok(()));
    methods.add_method("SetBorderAlpha", |_, _, _: mlua::MultiValue| Ok(()));
    methods.add_method("SetBorderScalar", |_, _, _: mlua::MultiValue| Ok(()));
    methods.add_method("UpdateMouseOverTooltip", |lua, this, (x, y): (f64, f64)| {
        update_mouse_over_tooltip(lua, this.0, x, y)
    });
}

fn update_mouse_over_tooltip(
    lua: &mlua::Lua,
    frame_id: u64,
    x: f64,
    y: f64,
) -> mlua::Result<(Value, Value)> {
    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    let blob_state = match state.quest_blobs.get(&frame_id) {
        Some(bs) if !bs.active_quests.is_empty() => bs,
        _ => return Ok((Value::Nil, Value::Nil)),
    };
    match crate::quest_poi_blobs::hit_test_blobs(
        &blob_state.active_quests,
        blob_state.map_id,
        x as f32,
        y as f32,
    ) {
        Some((quest_id, count)) => Ok((
            Value::Integer(quest_id as i64),
            Value::Integer(count as i64),
        )),
        None => Ok((Value::Nil, Value::Nil)),
    }
}

fn add_unit_position_frame_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("ClearUnits", |_, _, ()| Ok(()));
    methods.add_method("AddUnit", |_, _, _: mlua::MultiValue| Ok(()));
    methods.add_method("FinalizeUnits", |_, _, ()| Ok(()));
    methods.add_method("SetUiMapID", |_, _, _map_id: i32| Ok(()));
    methods.add_method("SetUnitColor", |_, _, _: mlua::MultiValue| Ok(()));
    // Blizzard's UnitPositionFrame expects varargs unit tokens from
    // GetMouseOverUnits(); return no values when no units are hovered.
    methods.add_method("GetMouseOverUnits", |_, _, ()| Ok(MultiValue::new()));
    methods.add_method("GetPlayerPingScale", |_, _, ()| Ok(1.0_f64));
    methods.add_method("SetPlayerPingTexture", |_, _, _: mlua::MultiValue| Ok(()));
    methods.add_method("SetPlayerPingScale", |_, _, _: mlua::MultiValue| Ok(()));
    methods.add_method("StartPlayerPing", |_, _, _: mlua::MultiValue| Ok(()));
    methods.add_method("StopPlayerPing", |_, _, ()| Ok(()));
}

/// Quest blob methods for QuestPOIFrame (DrawBlob, DrawNone, SetMapID).
fn add_quest_blob_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    use crate::lua_api::state::QuestBlobState;

    methods.add_method("DrawBlob", |lua, this, args: mlua::MultiValue| {
        let mut iter = args.into_iter();
        let quest_id = match iter.next() {
            Some(Value::Integer(n)) => n as u32,
            Some(Value::Number(n)) => n as u32,
            _ => return Ok(()),
        };
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        let blob = state
            .quest_blobs
            .entry(this.0)
            .or_insert_with(|| QuestBlobState {
                map_id: 0,
                active_quests: Vec::new(),
            });
        if !blob.active_quests.contains(&quest_id) {
            blob.active_quests.push(quest_id);
        }
        Ok(())
    });

    methods.add_method("DrawNone", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(blob) = state.quest_blobs.get_mut(&this.0) {
            blob.active_quests.clear();
        }
        Ok(())
    });

    // GetTooltipIndex(i) → POI index for tooltip line ordering.
    // Identity mapping: tooltip index equals the input index.
    methods.add_method("GetTooltipIndex", |_, _, index: i32| Ok(index));
}

/// Drag/Input stubs.
fn add_drag_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("AbortDrag", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if state.active_drag_frame == Some(this.0) {
            state.set_active_drag_frame(None);
        }
        Ok(())
    });
    methods.add_method("InterceptStartDrag", |lua, this, delegate: Value| {
        let Some(delegate_id) = extract_frame_id(&delegate) else {
            return Ok(false);
        };
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if state.active_drag_frame != Some(this.0) {
            return Ok(false);
        }
        if state.widgets.get(delegate_id).is_none() {
            return Ok(false);
        }
        state.set_active_drag_frame(Some(delegate_id));
        Ok(true)
    });
    methods.add_method("IsDragging", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state.active_drag_frame == Some(this.0))
    });
}

/// Mouse/Input Propagation stubs.
fn add_propagation_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("CanPropagateMouseClicks", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|frame| frame.propagate_mouse_clicks)
            .unwrap_or(false))
    });
    methods.add_method("CanPropagateMouseMotion", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|frame| frame.propagate_mouse_motion)
            .unwrap_or(false))
    });
    methods.add_method("DoesHyperlinkPropagateToParent", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|frame| frame.propagate_hyperlinks_to_parent)
            .unwrap_or(false))
    });
    methods.add_method("SetHyperlinkPropagateToParent", |lua, this, value: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut(this.0) {
            frame.propagate_hyperlinks_to_parent = value;
        }
        Ok(())
    });
    methods.add_method("SetPropagateMouseClicks", |lua, this, value: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut(this.0) {
            frame.propagate_mouse_clicks = value;
        }
        Ok(())
    });
    methods.add_method("SetPropagateMouseMotion", |lua, this, value: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut(this.0) {
            frame.propagate_mouse_motion = value;
        }
        Ok(())
    });
}

fn add_gamepad_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("EnableGamePadButton", |lua, this, enabled: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut(this.0) {
            frame.gamepad_button_enabled = enabled;
        }
        Ok(())
    });
    methods.add_method("EnableGamePadStick", |lua, this, enabled: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut(this.0) {
            frame.gamepad_stick_enabled = enabled;
        }
        Ok(())
    });
    methods.add_method("IsGamePadButtonEnabled", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|frame| frame.gamepad_button_enabled)
            .unwrap_or(false))
    });
    methods.add_method("IsGamePadStickEnabled", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|frame| frame.gamepad_stick_enabled)
            .unwrap_or(false))
    });
    methods.add_method("ShouldButtonPassThrough", |lua, this, button: String| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let normalized_button = button.to_ascii_lowercase();
        Ok(state
            .widgets
            .get(this.0)
            .map(|frame| frame.pass_through_buttons.contains(&normalized_button))
            .unwrap_or(false))
    });
}

/// Alpha/Gradient state.
fn add_alpha_gradient_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("ClearAlphaGradient", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut(this.0) {
            frame.alpha_gradients.clear();
        }
        Ok(())
    });
    methods.add_method("HasAlphaGradient", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|frame| !frame.alpha_gradients.is_empty())
            .unwrap_or(false))
    });
    methods.add_method("IsIgnoringParentAlpha", |_, _this, ()| Ok(false));
    methods.add_method("IsIgnoringParentScale", |_, _this, ()| Ok(false));
    methods.add_method("SetAlphaGradient", |lua, this, args: MultiValue| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        let Some(frame) = state.widgets.get_mut(this.0) else {
            return Ok(Value::Nil);
        };
        let Some((index, gradient)) = parse_alpha_gradient_args(&args) else {
            return Ok(set_alpha_gradient_result(frame.widget_type, false));
        };
        frame.alpha_gradients.insert(index, gradient);
        Ok(set_alpha_gradient_result(frame.widget_type, true))
    });
}

fn parse_alpha_gradient_args(args: &MultiValue) -> Option<(i32, crate::widget::AlphaGradient)> {
    let args_vec: Vec<&Value> = args.iter().collect();
    match args_vec.as_slice() {
        [index, Value::Table(_)] | [index, Value::Table(_), ..] => Some((
            alpha_gradient_index(index)?,
            alpha_gradient_from_value(args_vec[1])?,
        )),
        [start, length] => Some((
            0,
            crate::widget::AlphaGradient {
                start: alpha_gradient_number(start)?,
                length: alpha_gradient_number(length)?,
            },
        )),
        _ => None,
    }
}

fn alpha_gradient_index(value: &Value) -> Option<i32> {
    match value {
        Value::Integer(n) => Some(*n as i32),
        Value::Number(n) => Some(*n as i32),
        _ => None,
    }
}

fn alpha_gradient_number(value: &Value) -> Option<f32> {
    match value {
        Value::Integer(n) => Some(*n as f32),
        Value::Number(n) => Some(*n as f32),
        _ => None,
    }
}

fn alpha_gradient_from_value(value: &Value) -> Option<crate::widget::AlphaGradient> {
    match value {
        Value::Table(table) => Some(crate::widget::AlphaGradient {
            start: table
                .get::<Option<f32>>("x")
                .ok()
                .flatten()
                .or_else(|| table.get::<Option<f32>>(1).ok().flatten())?,
            length: table
                .get::<Option<f32>>("y")
                .ok()
                .flatten()
                .or_else(|| table.get::<Option<f32>>(2).ok().flatten())?,
        }),
        _ => None,
    }
}

fn set_alpha_gradient_result(widget_type: crate::widget::WidgetType, applied: bool) -> Value {
    if widget_type == crate::widget::WidgetType::FontString {
        Value::Boolean(applied)
    } else {
        Value::Nil
    }
}

/// Draw Layer stubs.
fn add_draw_layer_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("DisableDrawLayer", |lua, this, layer: String| {
        set_draw_layer_enabled(lua, this.0, &layer, false)
    });
    methods.add_method("EnableDrawLayer", |lua, this, layer: String| {
        set_draw_layer_enabled(lua, this.0, &layer, true)
    });
}

fn set_draw_layer_enabled(
    lua: &mlua::Lua,
    frame_id: u64,
    layer: &str,
    enabled: bool,
) -> mlua::Result<()> {
    let Some(layer) = draw_layer_from_name(layer) else {
        return Ok(());
    };
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    if let Some(frame) = state.widgets.get_mut_visual(frame_id) {
        frame.set_draw_layer_enabled(layer, enabled);
    }
    Ok(())
}

fn draw_layer_from_name(layer: &str) -> Option<crate::widget::DrawLayer> {
    crate::widget::DrawLayer::from_str(layer)
}

/// Frame Buffer/Rendering stubs.
fn add_frame_buffer_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("IsFrameBuffer", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|frame| frame.is_frame_buffer)
            .unwrap_or(false))
    });
    methods.add_method("RotateTextures", |lua, this, args: MultiValue| {
        let radians = frame_buffer_rotation_radians(&args);
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        rotate_descendant_textures(&mut state, this.0, radians);
        Ok(())
    });
    methods.add_method("SetIsFrameBuffer", |lua, this, enabled: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut(this.0) {
            frame.is_frame_buffer = enabled;
        }
        Ok(())
    });
}

fn frame_buffer_rotation_radians(args: &MultiValue) -> f32 {
    args.front().and_then(rotation_arg_to_f32).unwrap_or(0.0)
}

fn rotation_arg_to_f32(value: &Value) -> Option<f32> {
    match value {
        Value::Number(n) => Some(*n as f32),
        Value::Integer(n) => Some(*n as f32),
        _ => None,
    }
}

fn rotate_descendant_textures(state: &mut crate::lua_api::SimState, frame_id: u64, radians: f32) {
    let mut pending = vec![frame_id];
    while let Some(current_id) = pending.pop() {
        let Some(frame) = state.widgets.get(current_id) else {
            continue;
        };
        let child_ids = frame.children.clone();
        pending.extend(child_ids.iter().copied());
        for child_id in child_ids {
            if let Some(child) = state.widgets.get_mut_visual(child_id)
                && child.widget_type == crate::widget::WidgetType::Texture
            {
                child.rotation = radians;
            }
        }
    }
}

/// Bounds/Position stubs.
fn add_bounds_position_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetBoundsRect", |lua, this, ()| {
        let Some(resolved) = super::methods_rect::resolve_and_extract(lua, this.0) else {
            return Ok(mlua::MultiValue::new());
        };
        let (left, bottom, width, height) = super::methods_rect::to_wow_rect(&resolved);
        Ok(mlua::MultiValue::from_vec(vec![
            Value::Number(left as f64),
            Value::Number(bottom as f64),
            Value::Number(width as f64),
            Value::Number(height as f64),
        ]))
    });
    methods.add_method("GetClampRectInsets", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let (left, right, top, bottom) = state
            .widgets
            .get(this.0)
            .map(|frame| frame.clamp_rect_insets)
            .unwrap_or((0.0, 0.0, 0.0, 0.0));
        Ok((left as f64, right as f64, top as f64, bottom as f64))
    });
    methods.add_method("SetPointsOffset", |lua, this, (x, y): (f64, f64)| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            for anchor in &mut frame.anchors {
                anchor.x_offset = x as f32;
                anchor.y_offset = y as f32;
            }
        }
        state.widgets.mark_rect_dirty(this.0);
        Ok(())
    });
}

/// Attribute stubs.
fn add_attribute_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("CanChangeAttribute", |_, _this, ()| Ok(true));
    methods.add_method("ClearAttribute", |_, _this, _key: String| Ok(()));
    methods.add_method("ClearParentKey", |_, _this, ()| Ok(()));
}

/// Frame Level/Hierarchy methods.
fn add_frame_level_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("Lower", |lua, this, ()| {
        get_sim_state(lua).borrow_mut().lower_frame(this.0);
        Ok(())
    });
    methods.add_method("Raise", |lua, this, ()| {
        get_sim_state(lua).borrow_mut().raise_frame(this.0);
        Ok(())
    });
    methods.add_method(
        "GetHighestFrameLevel",
        |lua, this, iterate_all_children: Option<bool>| {
            let state_rc = get_sim_state(lua);
            let state = state_rc.borrow();
            Ok(get_highest_frame_level(
                &state.widgets,
                this.0,
                iterate_all_children.unwrap_or(false),
            ))
        },
    );
    methods.add_method("GetRaisedFrameLevel", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(get_raised_frame_level(&state.widgets, this.0))
    });
    methods.add_method("IsUsingParentLevel", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .widgets
            .get(this.0)
            .map(|frame| !frame.has_fixed_frame_level)
            .unwrap_or(false))
    });
    methods.add_method(
        "SetUsingParentLevel",
        |lua, this, using_parent_level: bool| {
            let id = this.0;
            if lockdown_blocked(lua, id, "SetUsingParentLevel") {
                return Ok(());
            }
            let state_rc = get_sim_state(lua);
            let mut state = state_rc.borrow_mut();
            let inherited_level = inherited_parent_level(&state.widgets, id);
            if let Some(frame) = state.widgets.get_mut_visual(id) {
                frame.has_fixed_frame_level = !using_parent_level;
                if let Some(level) = inherited_level.filter(|_| using_parent_level) {
                    frame.frame_level = level;
                }
            }
            super::methods_hierarchy::propagate_strata_level_pub(&mut state.widgets, id);
            Ok(())
        },
    );
}

fn get_highest_frame_level(
    widgets: &WidgetRegistry,
    root_id: u64,
    iterate_all_children: bool,
) -> i32 {
    let Some(root) = widgets.get(root_id) else {
        return 0;
    };
    if !iterate_all_children {
        return root.frame_level;
    }
    let mut highest = root.frame_level;
    let mut queue = root.children.clone();
    while let Some(child_id) = queue.pop() {
        let Some(child) = widgets.get(child_id) else {
            continue;
        };
        highest = highest.max(child.frame_level);
        queue.extend(child.children.iter().copied());
    }
    highest
}

fn get_raised_frame_level(widgets: &WidgetRegistry, id: u64) -> i32 {
    widgets
        .get(id)
        .map(|frame| frame.frame_level + frame.raise_order)
        .unwrap_or(0)
}

fn inherited_parent_level(widgets: &WidgetRegistry, id: u64) -> Option<i32> {
    let frame = widgets.get(id)?;
    let parent_level = widgets.get(frame.parent_id?)?.frame_level;
    Some(parent_level + frame.frame_level_offset.unwrap_or(1))
}

/// Secret/Protected stubs.
fn add_secret_protected_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_secret_query_methods(methods);
    add_secret_mutation_methods(methods);
}

fn add_secret_query_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("HasAnySecretAspect", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(frame_has_any_secret_aspect(&state.widgets, this.0))
    });
    methods.add_method("HasSecretAspect", |lua, this, aspect: Value| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(frame_has_secret_aspect(&state.widgets, this.0, &aspect))
    });
    methods.add_method("HasSecretValues", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(frame_has_secret_values(&state.widgets, this.0))
    });
    methods.add_method("IsAnchoringRestricted", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(frame_is_anchoring_restricted(&state.widgets, this.0))
    });
    methods.add_method("IsAnchoringSecret", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(frame_is_anchoring_secret(&state.widgets, this.0))
    });
    methods.add_method("IsPreventingSecretValues", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(frame_is_preventing_secret_values(&state.widgets, this.0))
    });
}

fn add_secret_mutation_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_is_protected_stub(methods);
    add_protect_method(methods);
    methods.add_method("SetPreventSecretValues", |lua, this, prevent: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut(this.0) {
            frame.prevent_secret_values = prevent;
        }
        Ok(())
    });
}

fn add_is_protected_stub<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("IsProtected", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let is_protected = state
            .widgets
            .get(this.0)
            .map(|f| f.is_protected)
            .unwrap_or(false);
        Ok((is_protected, is_protected))
    });
}

fn add_protect_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("Protect", |lua, this, ()| {
        let caller_secure = lua
            .globals()
            .get::<mlua::Function>("issecure")
            .and_then(|f| f.call::<bool>(()))
            .unwrap_or(false);
        if !caller_secure {
            return Ok(());
        }
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut(this.0) {
            frame.is_protected = true;
        }
        Ok(())
    });
}

fn frame_has_any_secret_aspect(widgets: &WidgetRegistry, id: u64) -> bool {
    frame_has_secret_values(widgets, id) || frame_is_anchoring_restricted(widgets, id)
}

fn frame_has_secret_aspect(widgets: &WidgetRegistry, id: u64, aspect: &Value) -> bool {
    secret_aspect_value(aspect)
        .is_some_and(|aspect_value| aspect_value == 1 && frame_has_any_secret_aspect(widgets, id))
}

fn secret_aspect_value(aspect: &Value) -> Option<i64> {
    match aspect {
        Value::Integer(value) => Some(*value),
        Value::Number(value) => Some(*value as i64),
        _ => None,
    }
}

fn frame_has_secret_values(widgets: &WidgetRegistry, id: u64) -> bool {
    widgets
        .get(id)
        .map(|frame| frame.prevent_secret_values)
        .unwrap_or(false)
}

fn frame_is_anchoring_restricted(widgets: &WidgetRegistry, id: u64) -> bool {
    widgets
        .get(id)
        .map(|frame| frame.forbidden || frame.is_protected)
        .unwrap_or(false)
}

fn frame_is_anchoring_secret(widgets: &WidgetRegistry, id: u64) -> bool {
    frame_has_secret_values(widgets, id)
}

fn frame_is_preventing_secret_values(widgets: &WidgetRegistry, id: u64) -> bool {
    widgets
        .get(id)
        .map(|frame| frame.prevent_secret_values)
        .unwrap_or(false)
}

fn frame_flattens_render_layers(widgets: &WidgetRegistry, id: u64) -> bool {
    widgets
        .get(id)
        .map(|frame| frame.flattens_render_layers)
        .unwrap_or(false)
}

fn frame_effectively_flattens_render_layers(widgets: &WidgetRegistry, id: u64) -> bool {
    let mut current_id = Some(id);

    while let Some(frame_id) = current_id {
        let Some(frame) = widgets.get(frame_id) else {
            return false;
        };
        if frame.flattens_render_layers {
            return true;
        }
        current_id = frame.parent_id;
    }

    false
}

/// Flatten/render layer methods.
fn add_flatten_render_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetEffectivelyFlattensRenderLayers", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(frame_effectively_flattens_render_layers(
            &state.widgets,
            this.0,
        ))
    });
    methods.add_method("GetFlattensRenderLayers", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(frame_flattens_render_layers(&state.widgets, this.0))
    });
}

fn frame_dont_save_position(widgets: &WidgetRegistry, id: u64) -> bool {
    widgets
        .get(id)
        .map(|frame| frame.dont_save_position)
        .unwrap_or(false)
}

fn frame_highlight_locked(widgets: &WidgetRegistry, id: u64) -> bool {
    widgets
        .get(id)
        .map(|frame| frame.highlight_locked)
        .unwrap_or(false)
}

fn frame_ignoring_children_for_bounds(widgets: &WidgetRegistry, id: u64) -> bool {
    widgets
        .get(id)
        .map(|frame| frame.ignoring_children_for_bounds)
        .unwrap_or(false)
}

fn collect_frame_and_descendant_ids(
    widgets: &WidgetRegistry,
    root_id: u64,
    exclude_root: bool,
) -> Vec<u64> {
    let mut ids = Vec::new();
    let mut stack = vec![root_id];

    while let Some(frame_id) = stack.pop() {
        if !(exclude_root && frame_id == root_id) {
            ids.push(frame_id);
        }
        if let Some(frame) = widgets.get(frame_id) {
            stack.extend(frame.children.iter().rev().copied());
        }
    }

    ids
}

fn desaturate_frame_hierarchy(
    state_rc: std::rc::Rc<std::cell::RefCell<crate::lua_api::SimState>>,
    root_id: u64,
    desaturation: f64,
    exclude_root: bool,
) {
    let mut state = state_rc.borrow_mut();
    let ids = collect_frame_and_descendant_ids(&state.widgets, root_id, exclude_root);
    let desaturated = desaturation > 0.0;

    for frame_id in ids {
        if let Some(frame) = state.widgets.get_mut_visual(frame_id) {
            frame.desaturated = desaturated;
        }
    }
}

fn get_frame_window(lua: &mlua::Lua, frame_id: u64) -> mlua::Result<Value> {
    let fields = frame_fields(lua, frame_id)?;
    fields.get("window")
}

fn set_frame_window(lua: &mlua::Lua, frame_id: u64, window: Value) -> mlua::Result<()> {
    let fields = frame_fields(lua, frame_id)?;
    fields.set("window", window)
}

/// Window/display methods.
fn add_window_display_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetDontSavePosition", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(frame_dont_save_position(&state.widgets, this.0))
    });
    methods.add_method("GetWindow", |lua, this, ()| get_frame_window(lua, this.0));
    methods.add_method("SetWindow", |lua, this, window: Value| {
        set_frame_window(lua, this.0, window)
    });
}

/// Miscellaneous stubs.
fn add_misc_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method(
        "DesaturateHierarchy",
        |lua, this, (desaturation, exclude_root): (f64, Option<bool>)| {
            desaturate_frame_hierarchy(
                get_sim_state(lua),
                this.0,
                desaturation,
                exclude_root.unwrap_or(false),
            );
            Ok(())
        },
    );
    methods.add_method("IsHighlightLocked", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(frame_highlight_locked(&state.widgets, this.0))
    });
    methods.add_method("IsIgnoringChildrenForBounds", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(frame_ignoring_children_for_bounds(&state.widgets, this.0))
    });
    methods.add_method(
        "RegisterUnitEventCallback",
        |_, _this, _args: MultiValue| Ok(()),
    );
    methods.add_method("SetHighlightLocked", |lua, this, locked: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            frame.highlight_locked = locked;
        }
        Ok(())
    });
    methods.add_method("SetIgnoringChildrenForBounds", |lua, this, ignore: bool| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut(this.0) {
            frame.ignoring_children_for_bounds = ignore;
        }
        Ok(())
    });
    methods.add_method("SetToDefaults", |_, _this, ()| Ok(()));
    methods.add_method("SetPlayerTexture", |_, _this, _args: MultiValue| Ok(()));
}

/// Minimap and WorldMap stubs.
fn add_minimap_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_minimap_core_methods(methods);
    add_minimap_texture_setters(methods);
    add_minimap_blob_setters(methods);
    // GetCanvas() - for WorldMapFrame (returns self as the canvas)
    methods.add_method("GetCanvas", |lua, this, ()| frame_ref(lua, this.0));
}

/// Minimap core: zoom, ping, blips.
fn add_minimap_core_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetZoom", |lua, this, ()| get_frame_zoom(lua, this.0));
    methods.add_method("SetZoom", |lua, this, zoom: i32| {
        set_frame_zoom(lua, this.0, zoom)
    });
    methods.add_method("GetZoomLevels", |_, _this, ()| Ok(5));
    methods.add_method("GetPingPosition", |_, _this, ()| Ok((0.0f64, 0.0f64)));
    methods.add_method("PingLocation", |_, _this, (_x, _y): (f64, f64)| Ok(()));
    methods.add_method("UpdateBlips", |_, _this, ()| Ok(()));
}

/// Minimap texture setters (no-op stubs).
fn add_minimap_texture_setters<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_minimap_texture_setter(methods, "SetBlipTexture", |frame, asset| {
        frame.minimap_blip_texture = asset;
    });
    add_minimap_texture_setter(methods, "SetMaskTexture", |frame, asset| {
        frame.minimap_mask_texture = asset;
    });
    add_minimap_texture_setter(methods, "SetIconTexture", |frame, asset| {
        frame.minimap_icon_texture = asset;
    });
    add_minimap_texture_setter(methods, "SetPOIArrowTexture", |frame, asset| {
        frame.minimap_poi_arrow_texture = asset;
    });
    add_minimap_texture_setter(methods, "SetCorpsePOIArrowTexture", |frame, asset| {
        frame.minimap_corpse_poi_arrow_texture = asset;
    });
    add_minimap_texture_setter(methods, "SetStaticPOIArrowTexture", |frame, asset| {
        frame.minimap_static_poi_arrow_texture = asset;
    });
}

fn add_minimap_texture_setter<M, F>(methods: &mut M, name: &'static str, setter: F)
where
    M: mlua::UserDataMethods<FrameRef>,
    F: Fn(&mut crate::widget::Frame, Option<String>) + Copy + 'static,
{
    methods.add_method(name, move |lua, this, asset: Value| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            setter(frame, texture_asset_to_string(&asset)?);
        }
        Ok(())
    });
}

fn texture_asset_to_string(asset: &Value) -> mlua::Result<Option<String>> {
    match asset {
        Value::Nil => Ok(None),
        Value::String(value) => Ok(Some(value.to_string_lossy().to_string())),
        Value::Integer(value) => Ok(Some(value.to_string())),
        Value::Number(value) => Ok(Some(value.to_string())),
        other => Err(mlua::Error::runtime(format!(
            "expected texture asset string/number/nil, got {}",
            other.type_name()
        ))),
    }
}

/// Minimap quest/task/arch blob setters.
fn add_minimap_blob_setters<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_minimap_blob_family(methods, BlobFamily::Quest);
    add_minimap_blob_family(methods, BlobFamily::Task);
    add_minimap_blob_family(methods, BlobFamily::Arch);
}

#[derive(Clone, Copy)]
enum BlobFamily {
    Quest,
    Task,
    Arch,
}

#[derive(Clone, Copy)]
enum BlobLayer {
    Inside,
    Outside,
}

struct BlobMethodNames {
    inside_texture: &'static str,
    inside_alpha: &'static str,
    outside_texture: &'static str,
    outside_alpha: &'static str,
    ring_texture: &'static str,
    ring_alpha: &'static str,
    ring_scalar: &'static str,
}

fn add_minimap_blob_family<M: mlua::UserDataMethods<FrameRef>>(
    methods: &mut M,
    family: BlobFamily,
) {
    let names = minimap_blob_method_names(family);
    add_minimap_blob_texture_setter(methods, names.inside_texture, family, BlobLayer::Inside);
    add_minimap_blob_alpha_setter(methods, names.inside_alpha, family, BlobLayer::Inside);
    add_minimap_blob_texture_setter(methods, names.outside_texture, family, BlobLayer::Outside);
    add_minimap_blob_alpha_setter(methods, names.outside_alpha, family, BlobLayer::Outside);
    add_minimap_blob_ring_texture_setter(methods, names.ring_texture, family);
    add_minimap_blob_ring_scalar_setter(methods, names.ring_scalar, family);
    add_minimap_blob_ring_alpha_setter(methods, names.ring_alpha, family);
}

fn minimap_blob_method_names(family: BlobFamily) -> BlobMethodNames {
    match family {
        BlobFamily::Quest => BlobMethodNames {
            inside_texture: "SetQuestBlobInsideTexture",
            inside_alpha: "SetQuestBlobInsideAlpha",
            outside_texture: "SetQuestBlobOutsideTexture",
            outside_alpha: "SetQuestBlobOutsideAlpha",
            ring_texture: "SetQuestBlobRingTexture",
            ring_alpha: "SetQuestBlobRingAlpha",
            ring_scalar: "SetQuestBlobRingScalar",
        },
        BlobFamily::Task => BlobMethodNames {
            inside_texture: "SetTaskBlobInsideTexture",
            inside_alpha: "SetTaskBlobInsideAlpha",
            outside_texture: "SetTaskBlobOutsideTexture",
            outside_alpha: "SetTaskBlobOutsideAlpha",
            ring_texture: "SetTaskBlobRingTexture",
            ring_alpha: "SetTaskBlobRingAlpha",
            ring_scalar: "SetTaskBlobRingScalar",
        },
        BlobFamily::Arch => BlobMethodNames {
            inside_texture: "SetArchBlobInsideTexture",
            inside_alpha: "SetArchBlobInsideAlpha",
            outside_texture: "SetArchBlobOutsideTexture",
            outside_alpha: "SetArchBlobOutsideAlpha",
            ring_texture: "SetArchBlobRingTexture",
            ring_alpha: "SetArchBlobRingAlpha",
            ring_scalar: "SetArchBlobRingScalar",
        },
    }
}

fn add_minimap_blob_texture_setter<M: mlua::UserDataMethods<FrameRef>>(
    methods: &mut M,
    name: &'static str,
    family: BlobFamily,
    layer: BlobLayer,
) {
    methods.add_method(name, move |lua, this, asset: Value| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            let texture = texture_asset_to_string(&asset)?;
            let layer_style = minimap_blob_layer_mut(frame, family, layer);
            layer_style.texture = texture;
        }
        Ok(())
    });
}

fn add_minimap_blob_alpha_setter<M: mlua::UserDataMethods<FrameRef>>(
    methods: &mut M,
    name: &'static str,
    family: BlobFamily,
    layer: BlobLayer,
) {
    methods.add_method(name, move |lua, this, alpha: f64| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            let layer_style = minimap_blob_layer_mut(frame, family, layer);
            layer_style.alpha = alpha;
        }
        Ok(())
    });
}

fn add_minimap_blob_ring_texture_setter<M: mlua::UserDataMethods<FrameRef>>(
    methods: &mut M,
    name: &'static str,
    family: BlobFamily,
) {
    methods.add_method(name, move |lua, this, asset: Value| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            let texture = texture_asset_to_string(&asset)?;
            let ring_style = minimap_blob_ring_mut(frame, family);
            ring_style.texture = texture;
        }
        Ok(())
    });
}

fn add_minimap_blob_ring_alpha_setter<M: mlua::UserDataMethods<FrameRef>>(
    methods: &mut M,
    name: &'static str,
    family: BlobFamily,
) {
    methods.add_method(name, move |lua, this, alpha: f64| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            let ring_style = minimap_blob_ring_mut(frame, family);
            ring_style.alpha = alpha;
        }
        Ok(())
    });
}

fn add_minimap_blob_ring_scalar_setter<M: mlua::UserDataMethods<FrameRef>>(
    methods: &mut M,
    name: &'static str,
    family: BlobFamily,
) {
    methods.add_method(name, move |lua, this, scalar: f64| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(frame) = state.widgets.get_mut_visual(this.0) {
            let ring_style = minimap_blob_ring_mut(frame, family);
            ring_style.scalar = scalar;
        }
        Ok(())
    });
}

fn minimap_blob_layer_mut(
    frame: &mut crate::widget::Frame,
    family: BlobFamily,
    layer: BlobLayer,
) -> &mut crate::widget::MinimapBlobLayerStyle {
    match (family, layer) {
        (BlobFamily::Quest, BlobLayer::Inside) => &mut frame.quest_blob_inside,
        (BlobFamily::Quest, BlobLayer::Outside) => &mut frame.quest_blob_outside,
        (BlobFamily::Task, BlobLayer::Inside) => &mut frame.task_blob_inside,
        (BlobFamily::Task, BlobLayer::Outside) => &mut frame.task_blob_outside,
        (BlobFamily::Arch, BlobLayer::Inside) => &mut frame.arch_blob_inside,
        (BlobFamily::Arch, BlobLayer::Outside) => &mut frame.arch_blob_outside,
    }
}

fn minimap_blob_ring_mut(
    frame: &mut crate::widget::Frame,
    family: BlobFamily,
) -> &mut crate::widget::MinimapBlobRingStyle {
    match family {
        BlobFamily::Quest => &mut frame.quest_blob_ring,
        BlobFamily::Task => &mut frame.task_blob_ring,
        BlobFamily::Arch => &mut frame.arch_blob_ring,
    }
}

/// ScrollingMessageFrame and EditBox stubs.
fn add_scrolling_message_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetTextCopyable", |_, _this, _copyable: bool| Ok(()));
    methods.add_method("SetInsertMode", |_, _this, _mode: String| Ok(()));
    methods.add_method("SetFading", |_, _this, _fading: bool| Ok(()));
    methods.add_method("SetFadeDuration", |_, _this, _duration: f32| Ok(()));
    methods.add_method("SetTimeVisible", |_, _this, _time: f32| Ok(()));
}

/// Alert subsystem, data provider, and EditMode stubs.
fn add_alert_and_data_provider_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_alert_subsystem_stub(methods);
    add_data_provider_stubs(methods);
    add_edit_mode_stubs(methods);
}

/// AddQueuedAlertFrameSubSystem stub returning a table with no-op alert methods.
fn add_alert_subsystem_stub<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method(
        "AddQueuedAlertFrameSubSystem",
        |lua, _this, _args: MultiValue| {
            let subsystem = lua.create_table()?;
            subsystem.set(
                "SetCanShowMoreConditionFunc",
                lua.create_function(|_, (_self, _func): (Value, Value)| Ok(()))?,
            )?;
            subsystem.set(
                "AddAlert",
                lua.create_function(|_, _args: MultiValue| Ok(()))?,
            )?;
            subsystem.set(
                "RemoveAlert",
                lua.create_function(|_, _args: MultiValue| Ok(()))?,
            )?;
            subsystem.set(
                "ClearAllAlerts",
                lua.create_function(|_, _self: Value| Ok(()))?,
            )?;
            Ok(Value::Table(subsystem))
        },
    );
}

/// WorldMapFrame data provider stubs and UseRaidStylePartyFrames.
fn add_data_provider_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("AddDataProvider", |lua, this, provider: Value| {
        add_frame_data_provider(lua, this.0, provider)
    });
    methods.add_method("RemoveDataProvider", |lua, this, provider: Value| {
        remove_frame_data_provider(lua, this.0, provider)
    });
    methods.add_method("UseRaidStylePartyFrames", |_, _this, ()| Ok(false));
}

/// EditModeSystemMixin stubs: delegate to mixin override or return safe defaults.
fn add_edit_mode_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("IsInDefaultPosition", |lua, this, ()| {
        let id = this.0;
        if let Some((func, ud)) =
            super::methods_helpers::get_mixin_override(lua, id, "IsInDefaultPosition")
        {
            return func.call::<bool>(ud);
        }
        Ok(true)
    });
    methods.add_method("IsInitialized", |lua, this, ()| {
        let id = this.0;
        if let Some((func, ud)) =
            super::methods_helpers::get_mixin_override(lua, id, "IsInitialized")
        {
            return func.call::<bool>(ud);
        }
        Ok(false)
    });
}

fn get_frame_zoom(lua: &mlua::Lua, frame_id: u64) -> mlua::Result<i32> {
    let fields = frame_fields(lua, frame_id)?;
    match fields.get::<Value>("zoom")? {
        Value::Integer(zoom) => Ok(zoom as i32),
        Value::Number(zoom) => Ok(zoom as i32),
        _ => Ok(0),
    }
}

fn set_frame_zoom(lua: &mlua::Lua, frame_id: u64, zoom: i32) -> mlua::Result<()> {
    let fields = frame_fields(lua, frame_id)?;
    fields.set("zoom", zoom.clamp(0, 5))
}

fn add_frame_data_provider(lua: &mlua::Lua, frame_id: u64, provider: Value) -> mlua::Result<()> {
    let providers = frame_data_providers(lua, frame_id)?;
    if table_contains_value(&providers, &provider)? {
        return Ok(());
    }
    let next_index = providers.raw_len() + 1;
    providers.raw_set(next_index, provider)
}

fn remove_frame_data_provider(lua: &mlua::Lua, frame_id: u64, provider: Value) -> mlua::Result<()> {
    let providers = frame_data_providers(lua, frame_id)?;
    remove_matching_value(&providers, &provider)
}

fn frame_data_providers(lua: &mlua::Lua, frame_id: u64) -> mlua::Result<mlua::Table> {
    let fields = frame_fields(lua, frame_id)?;
    match fields.get::<Value>("dataProviders")? {
        Value::Table(table) => Ok(table),
        _ => {
            let table = lua.create_table()?;
            fields.set("dataProviders", table.clone())?;
            Ok(table)
        }
    }
}

fn frame_fields(lua: &mlua::Lua, frame_id: u64) -> mlua::Result<mlua::Table> {
    let frame = frame_ref(lua, frame_id)?;
    match frame {
        Value::UserData(ud) => ud.user_value(),
        _ => lua.create_table(),
    }
}

fn table_contains_value(table: &mlua::Table, expected: &Value) -> mlua::Result<bool> {
    for value in table.sequence_values::<Value>() {
        if value? == *expected {
            return Ok(true);
        }
    }
    Ok(false)
}

fn remove_matching_value(table: &mlua::Table, expected: &Value) -> mlua::Result<()> {
    let mut next_index = 1;
    let mut kept = Vec::new();
    for value in table.sequence_values::<Value>() {
        let value = value?;
        if value != *expected {
            kept.push(value);
        }
    }
    table.clear()?;
    for value in kept {
        table.raw_set(next_index, value)?;
        next_index += 1;
    }
    Ok(())
}
