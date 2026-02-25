//! Miscellaneous frame-type-specific method stubs (Minimap, ScrollingMessage, Alerts, etc.).

use crate::lua_api::frame::handle::{frame_lud, lud_to_id};
use mlua::{LightUserData, Lua, MultiValue, Value};

/// Add all miscellaneous frame-type-specific methods.
pub fn add_misc_methods(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    add_minimap_methods(lua, methods)?;
    add_scrolling_message_methods(lua, methods)?;
    add_alert_and_data_provider_methods(lua, methods)?;
    add_drag_stubs(lua, methods)?;
    add_propagation_stubs(lua, methods)?;
    add_gamepad_stubs(lua, methods)?;
    add_alpha_gradient_stubs(lua, methods)?;
    add_draw_layer_stubs(lua, methods)?;
    add_frame_buffer_stubs(lua, methods)?;
    add_bounds_position_stubs(lua, methods)?;
    add_attribute_stubs(lua, methods)?;
    add_frame_level_stubs(lua, methods)?;
    add_secret_protected_stubs(lua, methods)?;
    add_flatten_render_stubs(lua, methods)?;
    add_window_display_stubs(lua, methods)?;
    add_misc_stubs(lua, methods)?;
    // DropdownButtonMixin stub
    methods.set("IsMenuOpen", lua.create_function(|_, _ud: LightUserData| Ok(false))?)?;
    // StaticPopupElementMixin stub (dialog ownership tracking)
    methods.set("SetOwningDialog", lua.create_function(|_, (_ud, _dialog): (LightUserData, Value)| Ok(()))?)?;
    // GuildRenameFrameMixin / layout tracking methods (no-op in simulator)
    methods.set("RegisterFontStrings", lua.create_function(|_, _args: MultiValue| Ok(()))?)?;
    methods.set("RegisterFrames", lua.create_function(|_, _args: MultiValue| Ok(()))?)?;
    methods.set("RegisterBackgroundTexture", lua.create_function(|_, _args: MultiValue| Ok(()))?)?;
    Ok(())
}

/// Drag/Input stubs.
fn add_drag_stubs(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    methods.set("AbortDrag", lua.create_function(|_, _ud: LightUserData| Ok(()))?)?;
    methods.set("InterceptStartDrag", lua.create_function(|_, (_ud, _flag): (LightUserData, bool)| Ok(()))?)?;
    methods.set("IsDragging", lua.create_function(|_, _ud: LightUserData| Ok(false))?)?;
    Ok(())
}

/// Mouse/Input Propagation stubs.
fn add_propagation_stubs(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    methods.set("CanPropagateMouseClicks", lua.create_function(|_, _ud: LightUserData| Ok(false))?)?;
    methods.set("CanPropagateMouseMotion", lua.create_function(|_, _ud: LightUserData| Ok(false))?)?;
    methods.set("DoesHyperlinkPropagateToParent", lua.create_function(|_, _ud: LightUserData| Ok(false))?)?;
    methods.set("SetHyperlinkPropagateToParent", lua.create_function(|_, (_ud, _): (LightUserData, bool)| Ok(()))?)?;
    methods.set("SetPropagateMouseClicks", lua.create_function(|_, (_ud, _): (LightUserData, bool)| Ok(()))?)?;
    methods.set("SetPropagateMouseMotion", lua.create_function(|_, (_ud, _): (LightUserData, bool)| Ok(()))?)?;
    Ok(())
}

/// GamePad stubs.
fn add_gamepad_stubs(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    methods.set("EnableGamePadButton", lua.create_function(|_, (_ud, _): (LightUserData, bool)| Ok(()))?)?;
    methods.set("EnableGamePadStick", lua.create_function(|_, (_ud, _): (LightUserData, bool)| Ok(()))?)?;
    methods.set("IsGamePadButtonEnabled", lua.create_function(|_, _ud: LightUserData| Ok(false))?)?;
    methods.set("IsGamePadStickEnabled", lua.create_function(|_, _ud: LightUserData| Ok(false))?)?;
    methods.set("ShouldButtonPassThrough", lua.create_function(|_, _ud: LightUserData| Ok(false))?)?;
    Ok(())
}

/// Alpha/Gradient stubs.
fn add_alpha_gradient_stubs(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    methods.set("ClearAlphaGradient", lua.create_function(|_, _ud: LightUserData| Ok(()))?)?;
    methods.set("HasAlphaGradient", lua.create_function(|_, _ud: LightUserData| Ok(false))?)?;
    methods.set("IsIgnoringParentAlpha", lua.create_function(|_, _ud: LightUserData| Ok(false))?)?;
    methods.set("IsIgnoringParentScale", lua.create_function(|_, _ud: LightUserData| Ok(false))?)?;
    methods.set("SetAlphaGradient", lua.create_function(|_, (_ud, _args): (LightUserData, MultiValue)| Ok(()))?)?;
    Ok(())
}

/// Draw Layer stubs.
fn add_draw_layer_stubs(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    methods.set("DisableDrawLayer", lua.create_function(|_, (_ud, _layer): (LightUserData, String)| Ok(()))?)?;
    methods.set("EnableDrawLayer", lua.create_function(|_, (_ud, _layer): (LightUserData, String)| Ok(()))?)?;
    Ok(())
}

/// Frame Buffer/Rendering stubs.
fn add_frame_buffer_stubs(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    methods.set("IsFrameBuffer", lua.create_function(|_, _ud: LightUserData| Ok(false))?)?;
    methods.set("RotateTextures", lua.create_function(|_, (_ud, _args): (LightUserData, MultiValue)| Ok(()))?)?;
    methods.set("SetIsFrameBuffer", lua.create_function(|_, (_ud, _): (LightUserData, bool)| Ok(()))?)?;
    Ok(())
}

/// Bounds/Position stubs.
fn add_bounds_position_stubs(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    methods.set("GetBoundsRect", lua.create_function(|_, _ud: LightUserData| {
        Ok((0.0f64, 0.0f64, 0.0f64, 0.0f64))
    })?)?;
    methods.set("GetClampRectInsets", lua.create_function(|_, _ud: LightUserData| {
        Ok((0.0f64, 0.0f64, 0.0f64, 0.0f64))
    })?)?;
    methods.set("SetPointsOffset", lua.create_function(|_, (_ud, _x, _y): (LightUserData, f64, f64)| Ok(()))?)?;
    Ok(())
}

/// Attribute stubs.
fn add_attribute_stubs(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    methods.set("CanChangeAttribute", lua.create_function(|_, _ud: LightUserData| Ok(true))?)?;
    methods.set("ClearAttribute", lua.create_function(|_, (_ud, _key): (LightUserData, String)| Ok(()))?)?;
    methods.set("ClearParentKey", lua.create_function(|_, _ud: LightUserData| Ok(()))?)?;
    Ok(())
}

/// Frame Level/Hierarchy stubs.
fn add_frame_level_stubs(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    methods.set("GetHighestFrameLevel", lua.create_function(|_, _ud: LightUserData| Ok(0i32))?)?;
    methods.set("GetRaisedFrameLevel", lua.create_function(|_, _ud: LightUserData| Ok(0i32))?)?;
    methods.set("IsUsingParentLevel", lua.create_function(|_, _ud: LightUserData| Ok(false))?)?;
    methods.set("SetUsingParentLevel", lua.create_function(|_, (_ud, _): (LightUserData, bool)| Ok(()))?)?;
    Ok(())
}

/// Secret/Protected stubs.
fn add_secret_protected_stubs(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    methods.set("HasAnySecretAspect", lua.create_function(|_, _ud: LightUserData| Ok(false))?)?;
    methods.set("HasSecretAspect", lua.create_function(|_, (_ud, _aspect): (LightUserData, Value)| Ok(false))?)?;
    methods.set("HasSecretValues", lua.create_function(|_, _ud: LightUserData| Ok(false))?)?;
    methods.set("IsAnchoringRestricted", lua.create_function(|_, _ud: LightUserData| Ok(false))?)?;
    methods.set("IsAnchoringSecret", lua.create_function(|_, _ud: LightUserData| Ok(false))?)?;
    methods.set("IsPreventingSecretValues", lua.create_function(|_, _ud: LightUserData| Ok(false))?)?;
    methods.set("IsProtected", lua.create_function(|_, _ud: LightUserData| Ok(false))?)?;
    methods.set("SetPreventSecretValues", lua.create_function(|_, (_ud, _): (LightUserData, bool)| Ok(()))?)?;
    Ok(())
}

/// Flatten/Render stubs.
fn add_flatten_render_stubs(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    methods.set("GetEffectivelyFlattensRenderLayers", lua.create_function(|_, _ud: LightUserData| Ok(false))?)?;
    methods.set("GetFlattensRenderLayers", lua.create_function(|_, _ud: LightUserData| Ok(false))?)?;
    Ok(())
}

/// Window/Display stubs.
fn add_window_display_stubs(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    methods.set("GetDontSavePosition", lua.create_function(|_, _ud: LightUserData| Ok(false))?)?;
    methods.set("GetWindow", lua.create_function(|_, _ud: LightUserData| Ok(Value::Nil))?)?;
    methods.set("SetWindow", lua.create_function(|_, (_ud, _args): (LightUserData, MultiValue)| Ok(()))?)?;
    Ok(())
}

/// Miscellaneous stubs.
fn add_misc_stubs(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    methods.set("DesaturateHierarchy", lua.create_function(|_, (_ud, _): (LightUserData, bool)| Ok(()))?)?;
    methods.set("IsHighlightLocked", lua.create_function(|_, _ud: LightUserData| Ok(false))?)?;
    methods.set("IsIgnoringChildrenForBounds", lua.create_function(|_, _ud: LightUserData| Ok(false))?)?;
    methods.set("RegisterUnitEventCallback", lua.create_function(|_, (_ud, _args): (LightUserData, MultiValue)| Ok(()))?)?;
    methods.set("SetHighlightLocked", lua.create_function(|_, (_ud, _): (LightUserData, bool)| Ok(()))?)?;
    methods.set("SetIgnoringChildrenForBounds", lua.create_function(|_, (_ud, _): (LightUserData, bool)| Ok(()))?)?;
    methods.set("SetToDefaults", lua.create_function(|_, _ud: LightUserData| Ok(()))?)?;
    methods.set("SetPlayerTexture", lua.create_function(|_, (_ud, _args): (LightUserData, MultiValue)| Ok(()))?)?;
    Ok(())
}

/// Minimap and WorldMap stubs.
fn add_minimap_methods(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    add_minimap_core_methods(lua, methods)?;
    add_minimap_texture_setters(lua, methods)?;
    add_minimap_blob_setters(lua, methods)?;
    // GetCanvas() - for WorldMapFrame (returns self as the canvas)
    methods.set("GetCanvas", lua.create_function(|_, ud: LightUserData| {
        let id = lud_to_id(ud);
        Ok(frame_lud(id))
    })?)?;
    Ok(())
}

/// Minimap core: zoom, ping, blips.
fn add_minimap_core_methods(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    methods.set("GetZoom", lua.create_function(|_, _ud: LightUserData| Ok(0))?)?;
    methods.set("SetZoom", lua.create_function(|_, (_ud, _zoom): (LightUserData, i32)| Ok(()))?)?;
    methods.set("GetZoomLevels", lua.create_function(|_, _ud: LightUserData| Ok(5))?)?;
    methods.set("GetPingPosition", lua.create_function(|_, _ud: LightUserData| Ok((0.0f64, 0.0f64)))?)?;
    methods.set("PingLocation", lua.create_function(|_, (_ud, _x, _y): (LightUserData, f64, f64)| Ok(()))?)?;
    methods.set("UpdateBlips", lua.create_function(|_, _ud: LightUserData| Ok(()))?)?;
    Ok(())
}

/// Minimap texture setters (no-op stubs).
fn add_minimap_texture_setters(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    methods.set("SetBlipTexture", lua.create_function(|_, (_ud, _asset): (LightUserData, Value)| Ok(()))?)?;
    methods.set("SetMaskTexture", lua.create_function(|_, (_ud, _asset): (LightUserData, Value)| Ok(()))?)?;
    methods.set("SetIconTexture", lua.create_function(|_, (_ud, _asset): (LightUserData, Value)| Ok(()))?)?;
    methods.set("SetPOIArrowTexture", lua.create_function(|_, (_ud, _asset): (LightUserData, Value)| Ok(()))?)?;
    methods.set("SetCorpsePOIArrowTexture", lua.create_function(|_, (_ud, _asset): (LightUserData, Value)| Ok(()))?)?;
    methods.set("SetStaticPOIArrowTexture", lua.create_function(|_, (_ud, _asset): (LightUserData, Value)| Ok(()))?)?;
    Ok(())
}

/// Minimap quest/task/arch blob setters (no-op stubs).
fn add_minimap_blob_setters(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    methods.set("SetQuestBlobInsideTexture", lua.create_function(|_, (_ud, _asset): (LightUserData, Value)| Ok(()))?)?;
    methods.set("SetQuestBlobInsideAlpha", lua.create_function(|_, (_ud, _alpha): (LightUserData, f32)| Ok(()))?)?;
    methods.set("SetQuestBlobOutsideTexture", lua.create_function(|_, (_ud, _asset): (LightUserData, Value)| Ok(()))?)?;
    methods.set("SetQuestBlobOutsideAlpha", lua.create_function(|_, (_ud, _alpha): (LightUserData, f32)| Ok(()))?)?;
    methods.set("SetQuestBlobRingTexture", lua.create_function(|_, (_ud, _asset): (LightUserData, Value)| Ok(()))?)?;
    methods.set("SetQuestBlobRingScalar", lua.create_function(|_, (_ud, _scalar): (LightUserData, f32)| Ok(()))?)?;
    methods.set("SetQuestBlobRingAlpha", lua.create_function(|_, (_ud, _alpha): (LightUserData, f32)| Ok(()))?)?;
    methods.set("SetTaskBlobInsideTexture", lua.create_function(|_, (_ud, _asset): (LightUserData, Value)| Ok(()))?)?;
    methods.set("SetTaskBlobInsideAlpha", lua.create_function(|_, (_ud, _alpha): (LightUserData, f32)| Ok(()))?)?;
    methods.set("SetTaskBlobOutsideTexture", lua.create_function(|_, (_ud, _asset): (LightUserData, Value)| Ok(()))?)?;
    methods.set("SetTaskBlobOutsideAlpha", lua.create_function(|_, (_ud, _alpha): (LightUserData, f32)| Ok(()))?)?;
    methods.set("SetTaskBlobRingTexture", lua.create_function(|_, (_ud, _asset): (LightUserData, Value)| Ok(()))?)?;
    methods.set("SetTaskBlobRingScalar", lua.create_function(|_, (_ud, _scalar): (LightUserData, f32)| Ok(()))?)?;
    methods.set("SetTaskBlobRingAlpha", lua.create_function(|_, (_ud, _alpha): (LightUserData, f32)| Ok(()))?)?;
    methods.set("SetArchBlobInsideTexture", lua.create_function(|_, (_ud, _asset): (LightUserData, Value)| Ok(()))?)?;
    methods.set("SetArchBlobInsideAlpha", lua.create_function(|_, (_ud, _alpha): (LightUserData, f32)| Ok(()))?)?;
    methods.set("SetArchBlobOutsideTexture", lua.create_function(|_, (_ud, _asset): (LightUserData, Value)| Ok(()))?)?;
    methods.set("SetArchBlobOutsideAlpha", lua.create_function(|_, (_ud, _alpha): (LightUserData, f32)| Ok(()))?)?;
    methods.set("SetArchBlobRingTexture", lua.create_function(|_, (_ud, _asset): (LightUserData, Value)| Ok(()))?)?;
    methods.set("SetArchBlobRingScalar", lua.create_function(|_, (_ud, _scalar): (LightUserData, f32)| Ok(()))?)?;
    methods.set("SetArchBlobRingAlpha", lua.create_function(|_, (_ud, _alpha): (LightUserData, f32)| Ok(()))?)?;
    Ok(())
}

/// ScrollingMessageFrame and EditBox stubs.
fn add_scrolling_message_methods(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    methods.set("SetTextCopyable", lua.create_function(|_, (_ud, _copyable): (LightUserData, bool)| Ok(()))?)?;
    methods.set("SetInsertMode", lua.create_function(|_, (_ud, _mode): (LightUserData, String)| Ok(()))?)?;
    methods.set("SetFading", lua.create_function(|_, (_ud, _fading): (LightUserData, bool)| Ok(()))?)?;
    methods.set("SetFadeDuration", lua.create_function(|_, (_ud, _duration): (LightUserData, f32)| Ok(()))?)?;
    methods.set("SetTimeVisible", lua.create_function(|_, (_ud, _time): (LightUserData, f32)| Ok(()))?)?;
    Ok(())
}

/// Alert subsystem, data provider, and EditMode stubs.
fn add_alert_and_data_provider_methods(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    add_alert_subsystem_stub(lua, methods)?;
    add_data_provider_stubs(lua, methods)?;
    add_edit_mode_stubs(lua, methods)?;
    Ok(())
}

/// AddQueuedAlertFrameSubSystem stub returning a table with no-op alert methods.
fn add_alert_subsystem_stub(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    methods.set("AddQueuedAlertFrameSubSystem", lua.create_function(|lua, (_ud, _args): (LightUserData, MultiValue)| {
        let subsystem = lua.create_table()?;
        subsystem.set("SetCanShowMoreConditionFunc",
            lua.create_function(|_, (_self, _func): (Value, Value)| Ok(()))?)?;
        subsystem.set("AddAlert",
            lua.create_function(|_, _args: MultiValue| Ok(()))?)?;
        subsystem.set("RemoveAlert",
            lua.create_function(|_, _args: MultiValue| Ok(()))?)?;
        subsystem.set("ClearAllAlerts",
            lua.create_function(|_, _self: Value| Ok(()))?)?;
        Ok(Value::Table(subsystem))
    })?)?;
    Ok(())
}

/// WorldMapFrame data provider stubs and UseRaidStylePartyFrames.
fn add_data_provider_stubs(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    methods.set("AddDataProvider", lua.create_function(|_, (_ud, _provider): (LightUserData, Value)| Ok(()))?)?;
    methods.set("RemoveDataProvider", lua.create_function(|_, (_ud, _provider): (LightUserData, Value)| Ok(()))?)?;
    methods.set("UseRaidStylePartyFrames", lua.create_function(|_, _ud: LightUserData| Ok(false))?)?;
    Ok(())
}

/// EditModeSystemMixin stubs: delegate to mixin override or return safe defaults.
fn add_edit_mode_stubs(lua: &Lua, methods: &mlua::Table) -> mlua::Result<()> {
    methods.set("IsInDefaultPosition", lua.create_function(|lua, ud: LightUserData| {
        let id = lud_to_id(ud);
        if let Some((func, ud)) = super::methods_helpers::get_mixin_override(lua, id, "IsInDefaultPosition") {
            return func.call::<bool>(ud);
        }
        Ok(true)
    })?)?;
    methods.set("IsInitialized", lua.create_function(|lua, ud: LightUserData| {
        let id = lud_to_id(ud);
        if let Some((func, ud)) = super::methods_helpers::get_mixin_override(lua, id, "IsInitialized") {
            return func.call::<bool>(ud);
        }
        Ok(false)
    })?)?;
    Ok(())
}
