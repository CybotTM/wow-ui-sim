//! Miscellaneous frame-type-specific method stubs (Minimap, ScrollingMessage, Alerts, etc.).

use super::super::handle::FrameRef;
use crate::lua_api::frame::handle::{frame_ref, get_sim_state};
use mlua::{MultiValue, Value};

/// Add all miscellaneous frame-type-specific methods.
pub fn add_misc_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_minimap_methods(methods);
    add_scrolling_message_methods(methods);
    add_alert_and_data_provider_methods(methods);
    add_drag_stubs(methods);
    add_propagation_stubs(methods);
    add_gamepad_stubs(methods);
    add_alpha_gradient_stubs(methods);
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
    methods.add_method("IsMenuOpen", |_, _this, ()| Ok(false));
    methods.add_method("SetOwningDialog", |_, _this, _dialog: Value| Ok(()));
    methods.add_method("RegisterFontStrings", |_, _this, _args: MultiValue| Ok(()));
    methods.add_method("RegisterFrames", |_, _this, _args: MultiValue| Ok(()));
    methods.add_method(
        "RegisterBackgroundTexture",
        |_, _this, _args: MultiValue| Ok(()),
    );
    // QuestPOIFrame
    methods.add_method("SetFillTexture", |_, _, _: mlua::MultiValue| Ok(()));
    methods.add_method("SetBorderTexture", |_, _, _: mlua::MultiValue| Ok(()));
    methods.add_method("SetFillAlpha", |_, _, _: mlua::MultiValue| Ok(()));
    methods.add_method("SetBorderAlpha", |_, _, _: mlua::MultiValue| Ok(()));
    methods.add_method("SetBorderScalar", |_, _, _: mlua::MultiValue| Ok(()));
    // FogOfWarFrame
    methods.add_method("GetUiMapID", |_, _, ()| Ok(mlua::Value::Nil));
    // Blob frame (QuestBlobDataProvider)
    methods.add_method("DrawNone", |_, _, ()| Ok(()));
    methods.add_method("DrawBlob", |_, _, _: mlua::MultiValue| Ok(()));
    // UnitPositionFrame
    methods.add_method("SetPlayerPingTexture", |_, _, _: mlua::MultiValue| Ok(()));
    methods.add_method("SetPlayerPingScale", |_, _, _: mlua::MultiValue| Ok(()));
    methods.add_method("StopPlayerPing", |_, _, ()| Ok(()));
}

/// Drag/Input stubs.
fn add_drag_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("AbortDrag", |_, _this, ()| Ok(()));
    methods.add_method("InterceptStartDrag", |_, _this, _flag: bool| Ok(()));
    methods.add_method("IsDragging", |_, _this, ()| Ok(false));
}

/// Mouse/Input Propagation stubs.
fn add_propagation_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("CanPropagateMouseClicks", |_, _this, ()| Ok(false));
    methods.add_method("CanPropagateMouseMotion", |_, _this, ()| Ok(false));
    methods.add_method("DoesHyperlinkPropagateToParent", |_, _this, ()| Ok(false));
    methods.add_method("SetHyperlinkPropagateToParent", |_, _this, _: bool| Ok(()));
    methods.add_method("SetPropagateMouseClicks", |_, _this, _: bool| Ok(()));
    methods.add_method("SetPropagateMouseMotion", |_, _this, _: bool| Ok(()));
}

/// GamePad stubs.
fn add_gamepad_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("EnableGamePadButton", |_, _this, _: bool| Ok(()));
    methods.add_method("EnableGamePadStick", |_, _this, _: bool| Ok(()));
    methods.add_method("IsGamePadButtonEnabled", |_, _this, ()| Ok(false));
    methods.add_method("IsGamePadStickEnabled", |_, _this, ()| Ok(false));
    methods.add_method("ShouldButtonPassThrough", |_, _this, ()| Ok(false));
}

/// Alpha/Gradient stubs.
fn add_alpha_gradient_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("ClearAlphaGradient", |_, _this, ()| Ok(()));
    methods.add_method("HasAlphaGradient", |_, _this, ()| Ok(false));
    methods.add_method("IsIgnoringParentAlpha", |_, _this, ()| Ok(false));
    methods.add_method("IsIgnoringParentScale", |_, _this, ()| Ok(false));
    methods.add_method("SetAlphaGradient", |_, _this, _args: MultiValue| Ok(()));
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
    methods.add_method("HasAnySecretAspect", |_, _this, ()| Ok(false));
    methods.add_method("HasSecretAspect", |_, _this, _aspect: Value| Ok(false));
    methods.add_method("HasSecretValues", |_, _this, ()| Ok(false));
    methods.add_method("IsAnchoringRestricted", |_, _this, ()| Ok(false));
    methods.add_method("IsAnchoringSecret", |_, _this, ()| Ok(false));
    methods.add_method("IsPreventingSecretValues", |_, _this, ()| Ok(false));
    methods.add_method("IsProtected", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let is_protected = state
            .widgets
            .get(this.0)
            .map(|f| f.is_protected)
            .unwrap_or(false);
        // (isProtected, isProtectedExplicitly) — both are static frame properties,
        // NOT affected by combat state. Combat lockdown controls whether
        // protected frames block insecure calls, but IsProtected() itself
        // always returns the same values.
        Ok((is_protected, is_protected))
    });

    methods.add_method("Protect", |lua, this, ()| {
        // Only secure (Blizzard) code can protect a frame.
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

    methods.add_method("SetPreventSecretValues", |_, _this, _: bool| Ok(()));
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
    methods.add_method("GetZoom", |_, _this, ()| Ok(0));
    methods.add_method("SetZoom", |_, _this, _zoom: i32| Ok(()));
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
    methods.add_method(
        "SetQuestBlobInsideTexture",
        |_, _this, _asset: Value| Ok(()),
    );
    methods.add_method("SetQuestBlobInsideAlpha", |_, _this, _alpha: f32| Ok(()));
    methods.add_method("SetQuestBlobOutsideTexture", |_, _this, _asset: Value| {
        Ok(())
    });
    methods.add_method("SetQuestBlobOutsideAlpha", |_, _this, _alpha: f32| Ok(()));
    methods.add_method("SetQuestBlobRingTexture", |_, _this, _asset: Value| Ok(()));
    methods.add_method("SetQuestBlobRingScalar", |_, _this, _scalar: f32| Ok(()));
    methods.add_method("SetQuestBlobRingAlpha", |_, _this, _alpha: f32| Ok(()));
    methods.add_method("SetTaskBlobInsideTexture", |_, _this, _asset: Value| Ok(()));
    methods.add_method("SetTaskBlobInsideAlpha", |_, _this, _alpha: f32| Ok(()));
    methods.add_method(
        "SetTaskBlobOutsideTexture",
        |_, _this, _asset: Value| Ok(()),
    );
    methods.add_method("SetTaskBlobOutsideAlpha", |_, _this, _alpha: f32| Ok(()));
    methods.add_method("SetTaskBlobRingTexture", |_, _this, _asset: Value| Ok(()));
    methods.add_method("SetTaskBlobRingScalar", |_, _this, _scalar: f32| Ok(()));
    methods.add_method("SetTaskBlobRingAlpha", |_, _this, _alpha: f32| Ok(()));
    methods.add_method("SetArchBlobInsideTexture", |_, _this, _asset: Value| Ok(()));
    methods.add_method("SetArchBlobInsideAlpha", |_, _this, _alpha: f32| Ok(()));
    methods.add_method(
        "SetArchBlobOutsideTexture",
        |_, _this, _asset: Value| Ok(()),
    );
    methods.add_method("SetArchBlobOutsideAlpha", |_, _this, _alpha: f32| Ok(()));
    methods.add_method("SetArchBlobRingTexture", |_, _this, _asset: Value| Ok(()));
    methods.add_method("SetArchBlobRingScalar", |_, _this, _scalar: f32| Ok(()));
    methods.add_method("SetArchBlobRingAlpha", |_, _this, _alpha: f32| Ok(()));
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
    methods.add_method("AddDataProvider", |_, _this, _provider: Value| Ok(()));
    methods.add_method("RemoveDataProvider", |_, _this, _provider: Value| Ok(()));
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
