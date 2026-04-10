//! Tests for widget-specific methods (methods_widget.rs):
//! EditBox, CheckButton, ColorSelect, SimpleHTML, Drag/Moving.

use wow_ui_sim::lua_api::WowLuaEnv;

// ============================================================================
// EditBox: SetFocus / ClearFocus / HasFocus
// ============================================================================

#[test]
fn test_editbox_focus() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(r#"local eb = CreateFrame("EditBox", "TestEB", UIParent)"#)
        .unwrap();

    let has_focus: bool = env.eval("return TestEB:HasFocus()").unwrap();
    assert!(!has_focus, "EditBox should not have focus initially");

    env.exec("TestEB:SetFocus()").unwrap();
    let has_focus: bool = env.eval("return TestEB:HasFocus()").unwrap();
    assert!(has_focus, "EditBox should have focus after SetFocus");

    env.exec("TestEB:ClearFocus()").unwrap();
    let has_focus: bool = env.eval("return TestEB:HasFocus()").unwrap();
    assert!(!has_focus, "EditBox should not have focus after ClearFocus");
}

#[test]
fn test_editbox_focus_switches_between_frames() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local eb1 = CreateFrame("EditBox", "TestEB1", UIParent)
        local eb2 = CreateFrame("EditBox", "TestEB2", UIParent)
        eb1:SetFocus()
    "#,
    )
    .unwrap();

    let eb1_focus: bool = env.eval("return TestEB1:HasFocus()").unwrap();
    assert!(eb1_focus);

    env.exec("TestEB2:SetFocus()").unwrap();
    // Only eb2 should have focus (eb1 doesn't auto-lose)
    let eb2_focus: bool = env.eval("return TestEB2:HasFocus()").unwrap();
    assert!(eb2_focus);
}

// ============================================================================
// EditBox: SetNumber / GetNumber
// ============================================================================

#[test]
fn test_editbox_set_get_number() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local eb = CreateFrame("EditBox", "TestEBNum", UIParent)
        eb:SetNumber(42.5)
    "#,
    )
    .unwrap();

    let num: f64 = env.eval("return TestEBNum:GetNumber()").unwrap();
    assert!((num - 42.5).abs() < 0.01);
}

#[test]
fn test_editbox_get_number_default() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(r#"local eb = CreateFrame("EditBox", "TestEBNumDef", UIParent)"#)
        .unwrap();

    let num: f64 = env.eval("return TestEBNumDef:GetNumber()").unwrap();
    assert_eq!(num, 0.0, "GetNumber should return 0 when no text set");
}

// ============================================================================
// CheckButton: SetChecked / GetChecked
// ============================================================================

#[test]
fn test_checkbutton_checked_state() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(r#"local cb = CreateFrame("CheckButton", "TestCB", UIParent)"#)
        .unwrap();

    let checked: bool = env.eval("return TestCB:GetChecked()").unwrap();
    assert!(!checked, "CheckButton should be unchecked initially");

    env.exec("TestCB:SetChecked(true)").unwrap();
    let checked: bool = env.eval("return TestCB:GetChecked()").unwrap();
    assert!(
        checked,
        "CheckButton should be checked after SetChecked(true)"
    );

    env.exec("TestCB:SetChecked(false)").unwrap();
    let checked: bool = env.eval("return TestCB:GetChecked()").unwrap();
    assert!(
        !checked,
        "CheckButton should be unchecked after SetChecked(false)"
    );
}

// ============================================================================
// ColorSelect: SetColorRGB / GetColorRGB
// ============================================================================

#[test]
fn test_colorselect_rgb() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local cs = CreateFrame("ColorSelect", "TestCS", UIParent)
        cs:SetColorRGB(0.5, 0.6, 0.7)
    "#,
    )
    .unwrap();

    let (r, g, b): (f64, f64, f64) = env.eval("return TestCS:GetColorRGB()").unwrap();
    assert!((r - 0.5).abs() < 0.001);
    assert!((g - 0.6).abs() < 0.001);
    assert!((b - 0.7).abs() < 0.001);
}

#[test]
fn test_colorselect_rgb_defaults() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(r#"local cs = CreateFrame("ColorSelect", "TestCSDef", UIParent)"#)
        .unwrap();

    let (r, g, b): (f64, f64, f64) = env.eval("return TestCSDef:GetColorRGB()").unwrap();
    assert_eq!(r, 1.0);
    assert_eq!(g, 1.0);
    assert_eq!(b, 1.0);
}

// ============================================================================
// ColorSelect: SetColorHSV / GetColorHSV
// ============================================================================

#[test]
fn test_colorselect_hsv_roundtrip() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local cs = CreateFrame("ColorSelect", "TestCSHSV", UIParent)
        cs:SetColorHSV(120, 0.5, 0.8)
    "#,
    )
    .unwrap();

    let (h, s, v): (f64, f64, f64) = env.eval("return TestCSHSV:GetColorHSV()").unwrap();
    assert!((h - 120.0).abs() < 0.01);
    assert!((s - 0.5).abs() < 0.01);
    assert!((v - 0.8).abs() < 0.01);
}

#[test]
fn test_colorselect_hsv_to_rgb_red() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local cs = CreateFrame("ColorSelect", "TestCSRed", UIParent)
        cs:SetColorHSV(0, 1, 1)
    "#,
    )
    .unwrap();

    // HSV(0, 1, 1) should be pure red RGB(1, 0, 0)
    let (r, g, b): (f64, f64, f64) = env.eval("return TestCSRed:GetColorRGB()").unwrap();
    assert!((r - 1.0).abs() < 0.01);
    assert!(g.abs() < 0.01);
    assert!(b.abs() < 0.01);
}

#[test]
fn test_colorselect_hsv_to_rgb_green() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local cs = CreateFrame("ColorSelect", "TestCSGreen", UIParent)
        cs:SetColorHSV(120, 1, 1)
    "#,
    )
    .unwrap();

    // HSV(120, 1, 1) should be pure green RGB(0, 1, 0)
    let (r, g, b): (f64, f64, f64) = env.eval("return TestCSGreen:GetColorRGB()").unwrap();
    assert!(r.abs() < 0.01);
    assert!((g - 1.0).abs() < 0.01);
    assert!(b.abs() < 0.01);
}

#[test]
fn test_colorselect_rgb_to_hsv_conversion() {
    let env = WowLuaEnv::new().unwrap();

    // Set via RGB, then read via HSV
    env.exec(
        r#"
        local cs = CreateFrame("ColorSelect", "TestCSConv", UIParent)
        cs:SetColorRGB(1, 0, 0)
    "#,
    )
    .unwrap();

    // Pure red should be HSV(0, 1, 1)
    let (h, s, v): (f64, f64, f64) = env.eval("return TestCSConv:GetColorHSV()").unwrap();
    assert!(h.abs() < 0.01, "Hue for red should be ~0, got {}", h);
    assert!(
        (s - 1.0).abs() < 0.01,
        "Saturation for red should be 1, got {}",
        s
    );
    assert!(
        (v - 1.0).abs() < 0.01,
        "Value for red should be 1, got {}",
        v
    );
}

#[test]
fn test_colorselect_alpha_defaults_to_one_and_round_trips() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local cs = CreateFrame("ColorSelect", "TestCSAlpha", UIParent)
    "#,
    )
    .unwrap();

    let default_alpha: f64 = env.eval("return TestCSAlpha:GetColorAlpha()").unwrap();
    assert!(
        (default_alpha - 1.0).abs() < 0.001,
        "ColorSelect alpha should default to fully opaque"
    );

    env.exec("TestCSAlpha:SetColorAlpha(0.35)").unwrap();

    let alpha: f64 = env.eval("return TestCSAlpha:GetColorAlpha()").unwrap();
    assert!((alpha - 0.35).abs() < 0.001);
}

#[test]
fn test_colorselect_alpha_is_preserved_across_rgb_and_hsv_updates() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local cs = CreateFrame("ColorSelect", "TestCSAlphaPreserve", UIParent)
        cs:SetColorRGB(0.2, 0.3, 0.4)
        cs:SetColorAlpha(0.6)
        cs:SetColorHSV(120, 0.5, 0.8)
    "#,
    )
    .unwrap();

    let (alpha, r, g, b, h, s, v): (f64, f64, f64, f64, f64, f64, f64) = env
        .eval(
            r#"
            local r, g, b = TestCSAlphaPreserve:GetColorRGB()
            local h, s, v = TestCSAlphaPreserve:GetColorHSV()
            return TestCSAlphaPreserve:GetColorAlpha(), r, g, b, h, s, v
            "#,
        )
        .unwrap();

    assert!(
        (alpha - 0.6).abs() < 0.001,
        "SetColorHSV should not wipe stored alpha"
    );
    assert!((r - 0.4).abs() < 0.01);
    assert!((g - 0.8).abs() < 0.01);
    assert!((b - 0.4).abs() < 0.01);
    assert!((h - 120.0).abs() < 0.01);
    assert!((s - 0.5).abs() < 0.01);
    assert!((v - 0.8).abs() < 0.01);

    env.exec("TestCSAlphaPreserve:SetColorRGB(0.9, 0.1, 0.2)")
        .unwrap();
    let (alpha_after_rgb, r_after_rgb, g_after_rgb, b_after_rgb): (f64, f64, f64, f64) = env
        .eval(
            r#"
            local r, g, b = TestCSAlphaPreserve:GetColorRGB()
            return TestCSAlphaPreserve:GetColorAlpha(), r, g, b
            "#,
        )
        .unwrap();

    assert!(
        (alpha_after_rgb - 0.6).abs() < 0.001,
        "SetColorRGB should not wipe stored alpha"
    );
    assert!((r_after_rgb - 0.9).abs() < 0.01);
    assert!((g_after_rgb - 0.1).abs() < 0.01);
    assert!((b_after_rgb - 0.2).abs() < 0.01);
}

#[test]
fn test_colorselect_texture_getters_default_nil() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(r#"local cs = CreateFrame("ColorSelect", "TestCSTexNil", UIParent)"#)
        .unwrap();

    let getters_are_nil: (bool, bool, bool, bool, bool, bool) = env
        .eval(
            r#"
            return TestCSTexNil:GetColorAlphaTexture() == nil,
                   TestCSTexNil:GetColorAlphaThumbTexture() == nil,
                   TestCSTexNil:GetColorValueTexture() == nil,
                   TestCSTexNil:GetColorValueThumbTexture() == nil,
                   TestCSTexNil:GetColorWheelTexture() == nil,
                   TestCSTexNil:GetColorWheelThumbTexture() == nil
            "#,
        )
        .unwrap();

    assert_eq!(getters_are_nil, (true, true, true, true, true, true));
}

#[test]
fn test_colorselect_primary_texture_setters_create_child_textures() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local cs = CreateFrame("ColorSelect", "TestCSTex", UIParent)
        cs:SetColorAlphaTexture("Interface\\Buttons\\WHITE8X8")
        cs:SetColorValueTexture("Interface\\Buttons\\WHITE8X8")
        cs:SetColorWheelTexture("Interface\\Buttons\\WHITE8X8")
    "#,
    )
    .unwrap();

    let result: (String, String, String, String, String, String) = env
        .eval(
            r#"
            return TestCSTex:GetColorAlphaTexture():GetObjectType(),
                   TestCSTex:GetColorAlphaTexture():GetParent():GetName(),
                   TestCSTex:GetColorValueTexture():GetObjectType(),
                   TestCSTex:GetColorValueTexture():GetParent():GetName(),
                   TestCSTex:GetColorWheelTexture():GetObjectType(),
                   TestCSTex:GetColorWheelTexture():GetParent():GetName()
            "#,
        )
        .unwrap();

    assert_eq!(result.0, "Texture");
    assert_eq!(result.1, "TestCSTex");
    assert_eq!(result.2, "Texture");
    assert_eq!(result.3, "TestCSTex");
    assert_eq!(result.4, "Texture");
    assert_eq!(result.5, "TestCSTex");
}

#[test]
fn test_colorselect_thumb_texture_setters_round_trip_userdata() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local cs = CreateFrame("ColorSelect", "TestCSThumbs", UIParent)
        TestCSAlphaThumb = cs:CreateTexture(nil, "ARTWORK")
        TestCSValueThumb = cs:CreateTexture(nil, "ARTWORK")
        TestCSWheelThumb = cs:CreateTexture(nil, "ARTWORK")
        cs:SetColorAlphaThumbTexture(TestCSAlphaThumb)
        cs:SetColorValueThumbTexture(TestCSValueThumb)
        cs:SetColorWheelThumbTexture(TestCSWheelThumb)
    "#,
    )
    .unwrap();

    let result: (bool, bool, bool) = env
        .eval(
            r#"
            return TestCSThumbs:GetColorAlphaThumbTexture() == TestCSAlphaThumb,
                   TestCSThumbs:GetColorValueThumbTexture() == TestCSValueThumb,
                   TestCSThumbs:GetColorWheelThumbTexture() == TestCSWheelThumb
            "#,
        )
        .unwrap();

    assert_eq!(result, (true, true, true));
}

#[test]
fn test_colorselect_clear_color_wheel_texture_clears_getter() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local cs = CreateFrame("ColorSelect", "TestCSClearWheel", UIParent)
        cs:SetColorWheelTexture("Interface\\Buttons\\WHITE8X8")
        TestCSWheelBeforeClear = cs:GetColorWheelTexture()
        cs:ClearColorWheelTexture()
    "#,
    )
    .unwrap();

    let result: (bool, bool) = env
        .eval(
            r#"
            return TestCSWheelBeforeClear ~= nil,
                   TestCSClearWheel:GetColorWheelTexture() == nil
            "#,
        )
        .unwrap();
    assert!(
        result.0,
        "SetColorWheelTexture should create a retrievable wheel texture before clear"
    );
    assert!(result.1, "Cleared color wheel getter should return nil");
}

#[test]
fn test_statusbar_texture_and_color_methods_still_resolve() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local sb = CreateFrame("StatusBar", "TestStatusBarMethods", UIParent)
        sb:SetStatusBarTexture("Interface\\Buttons\\WHITE8X8")
        sb:SetStatusBarColor(0.1, 0.2, 0.3, 0.4)
        sb:SetFillStyle("REVERSE")
    "#,
    )
    .unwrap();

    let has_texture: bool = env
        .eval("return type(TestStatusBarMethods:GetStatusBarTexture()) == 'table'")
        .unwrap();
    assert!(
        has_texture,
        "StatusBar should expose StatusBarTexture child"
    );

    let (r, g, b, a): (f32, f32, f32, f32) = env
        .eval("return TestStatusBarMethods:GetStatusBarColor()")
        .unwrap();
    assert!((r - 0.1).abs() < 0.001);
    assert!((g - 0.2).abs() < 0.001);
    assert!((b - 0.3).abs() < 0.001);
    assert!((a - 0.4).abs() < 0.001);
}

#[test]
fn test_statusbar_interpolation_methods_track_target_and_displayed_value() {
    let env = WowLuaEnv::new().unwrap();

    let result: (f64, f64, bool, f64, bool) = env
        .eval(
            r#"
            local sb = CreateFrame("StatusBar", "TestStatusBarInterpolation", UIParent)
            sb:SetMinMaxValues(0, 1)
            sb:SetValue(0.25)
            sb:SetValue(0.75, "Smooth")
            local targetValue = sb:GetValue()
            local displayedValue = sb:GetInterpolatedValue()
            local isInterpolating = sb:IsInterpolating()
            sb:SetToTargetValue()
            local snappedValue = sb:GetInterpolatedValue()
            local isStillInterpolating = sb:IsInterpolating()
            return targetValue, displayedValue, isInterpolating, snappedValue, isStillInterpolating
        "#,
        )
        .unwrap();

    assert!((result.0 - 0.75).abs() < 0.001);
    assert!(
        (result.1 - 0.25).abs() < 0.001,
        "displayed value should remain at the previous bar value until snapped"
    );
    assert!(result.2, "status bar should report active interpolation");
    assert!((result.3 - 0.75).abs() < 0.001);
    assert!(
        !result.4,
        "SetToTargetValue should finish interpolation and clear the flag"
    );
}

#[test]
fn test_statusbar_desaturation_methods_share_persisted_state() {
    let env = WowLuaEnv::new().unwrap();

    let result: (f64, bool, bool, f64, bool, bool) = env
        .eval(
            r#"
            local sb = CreateFrame("StatusBar", "TestStatusBarDesaturation", UIParent)
            sb:SetStatusBarTexture("Interface\\Buttons\\WHITE8X8")
            local texture = sb:GetStatusBarTexture()

            sb:SetStatusBarDesaturation(0.4)
            local normalized = sb:GetStatusBarDesaturation()
            local isDesaturated = sb:IsStatusBarDesaturated()
            local textureIsDesaturated = texture:IsDesaturated()

            sb:SetStatusBarDesaturated(false)
            local clearedNormalized = sb:GetStatusBarDesaturation()
            local clearedBool = sb:IsStatusBarDesaturated()
            local clearedTexture = texture:IsDesaturated()

            return normalized, isDesaturated, textureIsDesaturated, clearedNormalized, clearedBool, clearedTexture
        "#,
        )
        .unwrap();

    assert!((result.0 - 0.4).abs() < 0.001);
    assert!(
        result.1,
        "normalized desaturation should mark the status bar desaturated"
    );
    assert!(
        result.2,
        "status bar texture child should inherit desaturation"
    );
    assert_eq!(result.3, 0.0);
    assert!(
        !result.4,
        "bool state should clear through SetStatusBarDesaturated(false)"
    );
    assert!(
        !result.5,
        "texture state should clear through SetStatusBarDesaturated(false)"
    );
}

#[test]
fn test_statusbar_timer_duration_round_trips_duration_object() {
    let env = WowLuaEnv::new().unwrap();

    let result: (bool, String, bool) = env
        .eval(
            r#"
            local sb = CreateFrame("StatusBar", "TestStatusBarTimerDuration", UIParent)
            local duration = C_DurationUtil.CreateDuration()
            duration.debugTag = "statusbar-timer"
            sb:SetTimerDuration(duration, Enum.StatusBarInterpolation.Immediate, Enum.StatusBarTimerDirection.RemainingTime)
            local stored = sb:GetTimerDuration()
            return rawequal(duration, stored), stored.debugTag, type(stored) == "userdata"
        "#,
        )
        .unwrap();

    assert!(
        result.0,
        "GetTimerDuration should return the same duration object"
    );
    assert_eq!(result.1, "statusbar-timer");
    assert!(
        result.2,
        "status bar timer should stay a LuaDurationObject userdata"
    );
}

#[test]
fn test_player_model_methods_still_resolve() {
    let env = WowLuaEnv::new().unwrap();

    let result: (bool, bool, bool) = env
        .eval(
            r#"
            local pm = CreateFrame("PlayerModel", "TestPlayerModelMethods", UIParent)
            return type(pm.ApplySpellVisualKit) == "function",
                   type(pm.SetKeepModelOnHide) == "function",
                   type(pm.GetDisplayInfo) == "function"
            "#,
        )
        .unwrap();

    assert!(result.0, "PlayerModel should expose ApplySpellVisualKit");
    assert!(result.1, "PlayerModel should expose SetKeepModelOnHide");
    assert!(result.2, "PlayerModel should expose GetDisplayInfo");
}

// ============================================================================
// SimpleHTML: SetHyperlinkFormat / GetHyperlinkFormat
// ============================================================================

#[test]
fn test_simplehtml_hyperlink_format() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local sh = CreateFrame("SimpleHTML", "TestSH", UIParent)
        sh:SetHyperlinkFormat("|H%s|h[%s]|h")
    "#,
    )
    .unwrap();

    let fmt: String = env.eval("return TestSH:GetHyperlinkFormat()").unwrap();
    assert_eq!(fmt, "|H%s|h[%s]|h");
}

// ============================================================================
// SimpleHTML: SetHyperlinksEnabled / GetHyperlinksEnabled
// ============================================================================

#[test]
fn test_simplehtml_hyperlinks_enabled() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local sh = CreateFrame("SimpleHTML", "TestSHEnabled", UIParent)
        sh:SetHyperlinksEnabled(false)
    "#,
    )
    .unwrap();

    let enabled: bool = env
        .eval("return TestSHEnabled:GetHyperlinksEnabled()")
        .unwrap();
    assert!(!enabled);
}

// ============================================================================
// SimpleHTML: SetText strips HTML tags
// ============================================================================

#[test]
fn test_simplehtml_settext_strips_tags() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local sh = CreateFrame("SimpleHTML", "TestSHText", UIParent)
        sh:SetText("<p>Hello <b>World</b></p>")
    "#,
    )
    .unwrap();

    let text: String = env.eval("return TestSHText:GetText()").unwrap();
    assert_eq!(text, "Hello World", "HTML tags should be stripped");
}

// ============================================================================
// Drag/Moving: SetMovable / IsMovable / StartMoving / StopMovingOrSizing
// ============================================================================

#[test]
fn test_movable_set_get() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local f = CreateFrame("Frame", "TestMovable", UIParent)
        f:SetMovable(true)
    "#,
    )
    .unwrap();

    let movable: bool = env.eval("return TestMovable:IsMovable()").unwrap();
    assert!(movable);
}

#[test]
fn test_resizable_set_get() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local f = CreateFrame("Frame", "TestResizable", UIParent)
        f:SetResizable(true)
    "#,
    )
    .unwrap();

    let resizable: bool = env.eval("return TestResizable:IsResizable()").unwrap();
    assert!(resizable);
}

#[test]
fn test_clamped_to_screen_set_get() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local f = CreateFrame("Frame", "TestClamped", UIParent)
        f:SetClampedToScreen(true)
    "#,
    )
    .unwrap();

    let clamped: bool = env.eval("return TestClamped:IsClampedToScreen()").unwrap();
    assert!(clamped);
}

// ============================================================================
// Button / CheckButton / EditBox: mouse enabled by default
// ============================================================================

#[test]
fn test_button_mouse_enabled_by_default() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(r#"CreateFrame("Button", "TestMouseBtn", UIParent)"#)
        .unwrap();
    let enabled: bool = env
        .eval("return TestMouseBtn:IsMouseClickEnabled()")
        .unwrap();
    assert!(enabled, "Button should have mouse enabled by default");
}

#[test]
fn test_checkbutton_mouse_enabled_by_default() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(r#"CreateFrame("CheckButton", "TestMouseCB", UIParent)"#)
        .unwrap();
    let enabled: bool = env
        .eval("return TestMouseCB:IsMouseClickEnabled()")
        .unwrap();
    assert!(enabled, "CheckButton should have mouse enabled by default");
}

#[test]
fn test_frame_mouse_disabled_by_default() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(r#"CreateFrame("Frame", "TestMouseFrame", UIParent)"#)
        .unwrap();
    let enabled: bool = env
        .eval("return TestMouseFrame:IsMouseClickEnabled()")
        .unwrap();
    assert!(!enabled, "Frame should not have mouse enabled by default");
}
