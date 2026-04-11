//! Tests for draw layers, frame buffers, drag, propagation, gamepad, and alpha gradient methods.
use super::*;

#[test]
fn test_draw_layer_enabled_round_trip_tracks_per_layer_frame_state() {
    let env = WowLuaEnv::new().unwrap();
    env.eval::<()>(
        r#"
        local f = CreateFrame("Frame", "LayerToggleFrame", UIParent)

        assert(f:IsDrawLayerEnabled("BACKGROUND") == true, "background should default enabled")
        assert(f:IsDrawLayerEnabled("BORDER") == true, "border should default enabled")

        f:SetDrawLayerEnabled("BORDER", false)
        assert(f:IsDrawLayerEnabled("BORDER") == false, "border should disable")
        assert(f:IsDrawLayerEnabled("BACKGROUND") == true, "background should stay enabled")

        f:SetDrawLayerEnabled("BORDER", true)
        assert(f:IsDrawLayerEnabled("BORDER") == true, "border should re-enable")

        DRAW_LAYER_ENABLED_TEST_OK = true
    "#,
    )
    .unwrap();

    let ok: bool = env
        .eval("return DRAW_LAYER_ENABLED_TEST_OK == true")
        .unwrap();
    assert!(
        ok,
        "SetDrawLayerEnabled / IsDrawLayerEnabled Lua round-trip failed",
    );
}

#[test]
fn test_draw_layer_legacy_toggle_methods_update_layer_state() {
    let env = WowLuaEnv::new().unwrap();
    env.eval::<()>(
        r#"
        local f = CreateFrame("Frame", "LegacyLayerToggleFrame", UIParent)

        assert(f:IsDrawLayerEnabled("ARTWORK") == true, "artwork should default enabled")

        f:DisableDrawLayer("ARTWORK")
        assert(f:IsDrawLayerEnabled("ARTWORK") == false, "DisableDrawLayer should disable artwork")

        f:EnableDrawLayer("ARTWORK")
        assert(f:IsDrawLayerEnabled("ARTWORK") == true, "EnableDrawLayer should re-enable artwork")

        DRAW_LAYER_LEGACY_TOGGLE_OK = true
    "#,
    )
    .unwrap();

    let ok: bool = env
        .eval("return DRAW_LAYER_LEGACY_TOGGLE_OK == true")
        .unwrap();
    assert!(
        ok,
        "EnableDrawLayer / DisableDrawLayer Lua round-trip failed"
    );
}

#[test]
fn test_frame_buffer_methods_persist_flag_and_rotate_child_textures() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        local frame = CreateFrame("Frame", "FrameBufferFrame", UIParent)
        local first = frame:CreateTexture(nil, "ARTWORK")
        local second = frame:CreateTexture(nil, "OVERLAY")

        assert(not frame:IsFrameBuffer(), "frame buffer flag should default false")

        frame:SetIsFrameBuffer(true)
        assert(frame:IsFrameBuffer(), "frame buffer flag should enable")

        frame:RotateTextures(math.pi / 2)
        assert(math.abs(first:GetRotation() - (math.pi / 2)) < 0.001, "first child texture should rotate")
        assert(math.abs(second:GetRotation() - (math.pi / 2)) < 0.001, "second child texture should rotate")

        frame:SetIsFrameBuffer(false)
        assert(not frame:IsFrameBuffer(), "frame buffer flag should disable")

        FRAME_BUFFER_OK = true
    "#,
    )
    .unwrap();

    let ok: bool = env.eval("return FRAME_BUFFER_OK == true").unwrap();
    assert!(ok, "frame buffer flag/rotation round-trip should succeed");
}

#[test]
fn test_bounds_position_methods_use_geometry_and_persisted_insets() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        local frame = CreateFrame("Frame", "BoundsFrame", UIParent)
        frame:SetSize(120, 45)
        frame:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 10, -20)

        local left0, bottom0, width0, height0 = frame:GetBoundsRect()
        assert(left0 == 10 and width0 == 120 and height0 == 45, "GetBoundsRect should reflect initial geometry")

        frame:SetPointsOffset(30, -40)
        local _, _, _, x, y = frame:GetPoint(1)
        assert(x == 30 and y == -40, "SetPointsOffset should overwrite anchor offsets")

        local left1, bottom1, width1, height1 = frame:GetBoundsRect()
        assert(left1 == 30 and width1 == 120 and height1 == 45, "GetBoundsRect should reflect updated anchor geometry")

        frame:SetClampRectInsets(1, 2, 3, 4)
        local l, r, t, b = frame:GetClampRectInsets()
        assert(l == 1 and r == 2 and t == 3 and b == 4, "GetClampRectInsets should return persisted inset values")

        BOUNDS_POSITION_OK = true
    "#,
    )
    .unwrap();

    let ok: bool = env.eval("return BOUNDS_POSITION_OK == true").unwrap();
    assert!(ok, "bounds/position geometry round-trip should succeed");
}

#[test]
fn test_drag_methods_transfer_and_clear_active_drag_frame() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        DragSourceFrame = CreateFrame("Frame", "DragSourceFrame", UIParent)
        DragDelegateFrame = CreateFrame("Frame", "DragDelegateFrame", UIParent)
    "#,
    )
    .unwrap();

    let source_id = env
        .state()
        .borrow()
        .widgets
        .get_id_by_name("DragSourceFrame")
        .unwrap();
    let delegate_id = env
        .state()
        .borrow()
        .widgets
        .get_id_by_name("DragDelegateFrame")
        .unwrap();

    env.state().borrow_mut().active_drag_frame = Some(source_id);

    let intercepted: bool = env
        .eval("return DragSourceFrame:InterceptStartDrag(DragDelegateFrame)")
        .unwrap();
    assert!(
        intercepted,
        "drag interception should succeed for an active source frame"
    );

    let source_dragging: bool = env.eval("return DragSourceFrame:IsDragging()").unwrap();
    let delegate_dragging: bool = env.eval("return DragDelegateFrame:IsDragging()").unwrap();
    assert!(
        !source_dragging,
        "source frame should stop reporting dragging after interception"
    );
    assert!(
        delegate_dragging,
        "delegate frame should report dragging after interception"
    );
    assert_eq!(
        env.state().borrow().active_drag_frame,
        Some(delegate_id),
        "delegate should become the active drag frame"
    );

    env.exec("DragDelegateFrame:AbortDrag()").unwrap();

    let delegate_dragging_after_abort: bool =
        env.eval("return DragDelegateFrame:IsDragging()").unwrap();
    assert!(
        !delegate_dragging_after_abort,
        "AbortDrag should clear dragging state for the active drag frame"
    );
    assert_eq!(
        env.state().borrow().active_drag_frame,
        None,
        "AbortDrag should clear the active drag frame"
    );
}

#[test]
fn test_propagation_methods_round_trip_frame_flags() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        PropagationFrame = CreateFrame("Frame", "PropagationFrame", UIParent)

        assert(not PropagationFrame:CanPropagateMouseClicks(), "mouse clicks should default false")
        assert(not PropagationFrame:CanPropagateMouseMotion(), "mouse motion should default false")
        assert(not PropagationFrame:DoesHyperlinkPropagateToParent(), "hyperlink propagation should default false")

        PropagationFrame:SetPropagateMouseClicks(true)
        PropagationFrame:SetPropagateMouseMotion(true)
        PropagationFrame:SetHyperlinkPropagateToParent(true)

        assert(PropagationFrame:CanPropagateMouseClicks(), "mouse clicks should enable")
        assert(PropagationFrame:CanPropagateMouseMotion(), "mouse motion should enable")
        assert(PropagationFrame:DoesHyperlinkPropagateToParent(), "hyperlink propagation should enable")

        PropagationFrame:SetPropagateMouseClicks(false)
        PropagationFrame:SetPropagateMouseMotion(false)
        PropagationFrame:SetHyperlinkPropagateToParent(false)

        assert(not PropagationFrame:CanPropagateMouseClicks(), "mouse clicks should disable")
        assert(not PropagationFrame:CanPropagateMouseMotion(), "mouse motion should disable")
        assert(not PropagationFrame:DoesHyperlinkPropagateToParent(), "hyperlink propagation should disable")

        PROPAGATION_FLAGS_OK = true
    "#,
    )
    .unwrap();

    let ok: bool = env.eval("return PROPAGATION_FLAGS_OK == true").unwrap();
    assert!(ok, "propagation flag round-trip should succeed");
}

#[test]
fn test_gamepad_methods_round_trip_frame_state() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        GamePadFrame = CreateFrame("Frame", "GamePadFrame", UIParent)

        assert(not GamePadFrame:IsGamePadButtonEnabled(), "gamepad button should default false")
        assert(not GamePadFrame:IsGamePadStickEnabled(), "gamepad stick should default false")
        assert(not GamePadFrame:ShouldButtonPassThrough("LeftButton"), "button passthrough should default false")

        GamePadFrame:EnableGamePadButton(true)
        GamePadFrame:EnableGamePadStick(true)
        GamePadFrame:SetPassThroughButtons("LeftButton", "RightButton")

        assert(GamePadFrame:IsGamePadButtonEnabled(), "gamepad button should enable")
        assert(GamePadFrame:IsGamePadStickEnabled(), "gamepad stick should enable")
        assert(GamePadFrame:ShouldButtonPassThrough("LeftButton"), "left button should pass through after configuration")
        assert(GamePadFrame:ShouldButtonPassThrough("RIGHTBUTTON"), "button passthrough should be case-insensitive")
        assert(not GamePadFrame:ShouldButtonPassThrough("MiddleButton"), "unconfigured buttons should not pass through")

        GamePadFrame:EnableGamePadButton(false)
        GamePadFrame:EnableGamePadStick(false)
        GamePadFrame:SetPassThroughButtons()

        assert(not GamePadFrame:IsGamePadButtonEnabled(), "gamepad button should disable")
        assert(not GamePadFrame:IsGamePadStickEnabled(), "gamepad stick should disable")
        assert(not GamePadFrame:ShouldButtonPassThrough("LeftButton"), "button passthrough should clear")

        GAMEPAD_FLAGS_OK = true
    "#,
    )
    .unwrap();

    let ok: bool = env.eval("return GAMEPAD_FLAGS_OK == true").unwrap();
    assert!(ok, "gamepad flag round-trip should succeed");
}

#[test]
fn test_alpha_gradient_surface_round_trip_frame_state() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        AlphaGradientFrame = CreateFrame("Frame", "AlphaGradientFrame", UIParent)

        assert(not AlphaGradientFrame:HasAlphaGradient(), "alpha gradient should default disabled")

        AlphaGradientFrame:SetAlphaGradient(2, { x = 0.25, y = 0.75 })
        assert(AlphaGradientFrame:HasAlphaGradient(), "alpha gradient should enable after SetAlphaGradient")

        AlphaGradientFrame:ClearAlphaGradient()
        assert(not AlphaGradientFrame:HasAlphaGradient(), "alpha gradient should clear after ClearAlphaGradient")

        ALPHA_GRADIENT_OK = true
    "#,
    )
    .unwrap();

    let ok: bool = env.eval("return ALPHA_GRADIENT_OK == true").unwrap();
    assert!(ok, "alpha gradient round-trip should succeed");
}

#[test]
fn test_font_string_set_alpha_gradient_accepts_legacy_arguments() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        local frame = CreateFrame("Frame", nil, UIParent)
        local fs = frame:CreateFontString(nil, "OVERLAY")
        fs:SetText("Hello World")

        local ok, applied = pcall(function()
            return fs:SetAlphaGradient(0, 50)
        end)

        assert(ok, "FontString:SetAlphaGradient should not error for legacy arguments")
        assert(applied == true, "FontString:SetAlphaGradient should report success")

        FONTSTRING_ALPHA_GRADIENT_OK = true
    "#,
    )
    .unwrap();

    let ok: bool = env
        .eval("return FONTSTRING_ALPHA_GRADIENT_OK == true")
        .unwrap();
    assert!(ok, "FontString alpha gradient compatibility should succeed");
}
