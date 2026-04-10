//! Miscellaneous frame-type-specific method stubs (Minimap, ScrollingMessage, Alerts, etc.).

use super::super::handle::FrameRef;
use crate::lua_api::frame::handle::{extract_frame_id, frame_ref, get_sim_state};
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
    add_flatten_render_stubs(methods);
    add_window_display_stubs(methods);
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
    methods.add_method("DisableDrawLayer", |_, _this, _layer: String| Ok(()));
    methods.add_method("EnableDrawLayer", |_, _this, _layer: String| Ok(()));
}

/// Frame Buffer/Rendering stubs.
fn add_frame_buffer_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("IsFrameBuffer", |_, _this, ()| Ok(false));
    methods.add_method("RotateTextures", |_, _this, _args: MultiValue| Ok(()));
    methods.add_method("SetIsFrameBuffer", |_, _this, _: bool| Ok(()));
}

/// Bounds/Position stubs.
fn add_bounds_position_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetBoundsRect", |_, _this, ()| {
        Ok((0.0f64, 0.0f64, 0.0f64, 0.0f64))
    });
    methods.add_method("GetClampRectInsets", |_, _this, ()| {
        Ok((0.0f64, 0.0f64, 0.0f64, 0.0f64))
    });
    methods.add_method("SetPointsOffset", |_, _this, (_x, _y): (f64, f64)| Ok(()));
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
    methods.add_method("GetHighestFrameLevel", |_, _this, ()| Ok(0i32));
    methods.add_method("GetRaisedFrameLevel", |_, _this, ()| Ok(0i32));
    methods.add_method("IsUsingParentLevel", |_, _this, ()| Ok(false));
    methods.add_method("SetUsingParentLevel", |_, _this, _: bool| Ok(()));
}

/// Secret/Protected stubs.
fn add_secret_protected_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_secret_boolean_stub(methods, "HasAnySecretAspect");
    add_secret_boolean_stub_with_arg(methods, "HasSecretAspect");
    add_secret_boolean_stub(methods, "HasSecretValues");
    add_secret_boolean_stub(methods, "IsAnchoringRestricted");
    add_secret_boolean_stub(methods, "IsAnchoringSecret");
    add_secret_boolean_stub(methods, "IsPreventingSecretValues");
    add_is_protected_stub(methods);
    add_protect_method(methods);
    methods.add_method("SetPreventSecretValues", |_, _this, _: bool| Ok(()));
}

fn add_secret_boolean_stub<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M, name: &str) {
    methods.add_method(name, |_, _this, ()| Ok(false));
}

fn add_secret_boolean_stub_with_arg<M: mlua::UserDataMethods<FrameRef>>(
    methods: &mut M,
    name: &str,
) {
    methods.add_method(name, |_, _this, _arg: Value| Ok(false));
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

/// Flatten/Render stubs.
fn add_flatten_render_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetEffectivelyFlattensRenderLayers", |_, _this, ()| {
        Ok(false)
    });
    methods.add_method("GetFlattensRenderLayers", |_, _this, ()| Ok(false));
}

/// Window/Display stubs.
fn add_window_display_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetDontSavePosition", |_, _this, ()| Ok(false));
    methods.add_method("GetWindow", |_, _this, ()| Ok(Value::Nil));
    methods.add_method("SetWindow", |_, _this, _args: MultiValue| Ok(()));
}

/// Miscellaneous stubs.
fn add_misc_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("DesaturateHierarchy", |_, _this, _: bool| Ok(()));
    methods.add_method("IsHighlightLocked", |_, _this, ()| Ok(false));
    methods.add_method("IsIgnoringChildrenForBounds", |_, _this, ()| Ok(false));
    methods.add_method(
        "RegisterUnitEventCallback",
        |_, _this, _args: MultiValue| Ok(()),
    );
    methods.add_method("SetHighlightLocked", |_, _this, _: bool| Ok(()));
    methods.add_method("SetIgnoringChildrenForBounds", |_, _this, _: bool| Ok(()));
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
    methods.add_method("SetBlipTexture", |_, _this, _asset: Value| Ok(()));
    methods.add_method("SetMaskTexture", |_, _this, _asset: Value| Ok(()));
    methods.add_method("SetIconTexture", |_, _this, _asset: Value| Ok(()));
    methods.add_method("SetPOIArrowTexture", |_, _this, _asset: Value| Ok(()));
    methods.add_method("SetCorpsePOIArrowTexture", |_, _this, _asset: Value| Ok(()));
    methods.add_method("SetStaticPOIArrowTexture", |_, _this, _asset: Value| Ok(()));
}

/// Minimap quest/task/arch blob setters (no-op stubs).
fn add_minimap_blob_setters<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_minimap_blob_family(methods, "Quest");
    add_minimap_blob_family(methods, "Task");
    add_minimap_blob_family(methods, "Arch");
}

fn add_minimap_blob_family<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M, prefix: &str) {
    add_minimap_blob_texture_stub(methods, &format!("Set{prefix}BlobInsideTexture"));
    add_minimap_blob_alpha_stub(methods, &format!("Set{prefix}BlobInsideAlpha"));
    add_minimap_blob_texture_stub(methods, &format!("Set{prefix}BlobOutsideTexture"));
    add_minimap_blob_alpha_stub(methods, &format!("Set{prefix}BlobOutsideAlpha"));
    add_minimap_blob_texture_stub(methods, &format!("Set{prefix}BlobRingTexture"));
    add_minimap_blob_scalar_stub(methods, &format!("Set{prefix}BlobRingScalar"));
    add_minimap_blob_alpha_stub(methods, &format!("Set{prefix}BlobRingAlpha"));
}

fn add_minimap_blob_texture_stub<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M, name: &str) {
    methods.add_method(name, |_, _this, _asset: Value| Ok(()));
}

fn add_minimap_blob_alpha_stub<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M, name: &str) {
    methods.add_method(name, |_, _this, _alpha: f32| Ok(()));
}

fn add_minimap_blob_scalar_stub<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M, name: &str) {
    methods.add_method(name, |_, _this, _scalar: f32| Ok(()));
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
