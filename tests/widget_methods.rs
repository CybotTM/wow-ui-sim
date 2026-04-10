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
fn test_widget_misc_setup_and_alert_methods_persist_runtime_fields() {
    let env = WowLuaEnv::new().unwrap();

    let result: (bool, bool, bool) = env
        .eval(
            r#"
            local frame = CreateFrame("Frame", "TestWidgetMiscFrame", UIParent)
            local fallbackGenerator = function() end
            frame:SetupMenu(fallbackGenerator)
            local fields = debug.getfenv(frame)[1]
            local fallbackStored = fields.menuGenerator == fallbackGenerator

            local overrideFrame = CreateFrame("Frame", "TestWidgetMiscOverrideFrame", UIParent)
            local overrideFields = debug.getfenv(overrideFrame)[1]
            local overrideGenerator = function() end
            overrideFields.SetupMenu = function(self, generator)
                rawset(overrideFields, "calledGenerator", generator)
            end
            overrideFrame:SetupMenu(overrideGenerator)
            local overrideCalled = overrideFields.calledGenerator == overrideGenerator

            local container = CreateFrame("Frame", "TestWidgetMiscContainer", UIParent)
            frame:SetAlertContainer(container)
            local alertStored = fields.alertContainer == container

            return fallbackStored, overrideCalled, alertStored
        "#,
        )
        .unwrap();

    assert!(
        result.0,
        "SetupMenu should store menuGenerator without an override"
    );
    assert!(
        result.1,
        "SetupMenu should delegate to an existing mixin override instead of shadowing it"
    );
    assert!(
        result.2,
        "SetAlertContainer should store the container reference on the frame"
    );
}

#[test]
fn test_widget_misc_item_button_methods_delegate_or_store_state() {
    let env = WowLuaEnv::new().unwrap();

    let result: (bool, bool, bool, bool, bool) = env
        .eval(
            r#"
            local frame = CreateFrame("Frame", "TestWidgetMiscItemFrame", UIParent)
            local fields = debug.getfenv(frame)[1]
            local translator = function(selection) return selection end
            frame:SetSelectionTranslator(translator)
            local translatorStored = fields.selectionTranslator == translator

            frame:SetItemButtonScale(0.65)
            local scaleStored = fields.itemButtonScale == 0.65

            local overrideFrame = CreateFrame("Frame", "TestWidgetMiscItemOverrideFrame", UIParent)
            local overrideFields = debug.getfenv(overrideFrame)[1]
            local overrideTranslator = function(selection) return selection.data end
            overrideFields.SetSelectionTranslator = function(self, value)
                rawset(overrideFields, "storedTranslator", value)
            end
            overrideFields.SetItemButtonScale = function(self, value)
                rawset(overrideFields, "storedScale", value)
            end
            overrideFields.UpdateItemContextMatching = function(self)
                rawset(overrideFields, "updateCalled", true)
            end

            overrideFrame:SetSelectionTranslator(overrideTranslator)
            overrideFrame:SetItemButtonScale(1.4)
            overrideFrame:UpdateItemContextMatching()

            return translatorStored,
                scaleStored,
                overrideFields.storedTranslator == overrideTranslator,
                overrideFields.storedScale == 1.4,
                overrideFields.updateCalled == true
        "#,
        )
        .unwrap();

    assert!(
        result.0,
        "SetSelectionTranslator should store the fallback translator on the frame"
    );
    assert!(
        result.1,
        "SetItemButtonScale should store the fallback scale on the frame"
    );
    assert!(
        result.2,
        "SetSelectionTranslator should delegate to an existing mixin override"
    );
    assert!(
        result.3,
        "SetItemButtonScale should delegate to an existing mixin override"
    );
    assert!(
        result.4,
        "UpdateItemContextMatching should delegate to an existing mixin override"
    );
}

#[test]
fn test_widget_misc_visual_methods_delegate_or_store_state() {
    let env = WowLuaEnv::new().unwrap();

    let result: (bool, bool, bool, bool) = env
        .eval(
            r##"
            local frame = CreateFrame("Frame", "TestWidgetMiscVisualFrame", UIParent)
            local frameFields = debug.getfenv(frame)[1]
            frame:SetDefaultText("fallback")
            local defaultStored = frameFields.defaultText == "fallback"

            local texture = frame:CreateTexture(nil, "ARTWORK")
            local textureFields = debug.getfenv(texture)[1]
            texture:SetVisuals("left", 42, true)
            local visualStored = textureFields.visualArgs
                and textureFields.visualArgs[1] == "left"
                and textureFields.visualArgs[2] == 42
                and textureFields.visualArgs[3] == true

            local overrideFrame = CreateFrame("Frame", "TestWidgetMiscVisualOverrideFrame", UIParent)
            local overrideFrameFields = debug.getfenv(overrideFrame)[1]
            overrideFrameFields.SetDefaultText = function(self, text)
                rawset(overrideFrameFields, "storedDefaultText", text)
            end
            overrideFrameFields.UpdateHeight = function(self)
                rawset(overrideFrameFields, "heightUpdated", true)
            end

            local overrideTexture = overrideFrame:CreateTexture(nil, "ARTWORK")
            local overrideTextureFields = debug.getfenv(overrideTexture)[1]
            overrideTextureFields.SetVisuals = function(self, ...)
                rawset(overrideTextureFields, "visualCount", select("#", ...))
                rawset(overrideTextureFields, "visualFirst", (...))
            end

            overrideFrame:SetDefaultText("override")
            overrideFrame:UpdateHeight()
            overrideTexture:SetVisuals("tierSlot", 7, false)

            return defaultStored,
                visualStored,
                overrideFrameFields.storedDefaultText == "override"
                    and overrideTextureFields.visualCount == 3
                    and overrideTextureFields.visualFirst == "tierSlot",
                overrideFrameFields.heightUpdated == true
        "##,
        )
        .unwrap();

    assert!(
        result.0,
        "SetDefaultText should store fallback default text on the frame"
    );
    assert!(
        result.1,
        "SetVisuals should store fallback visual arguments on the frame"
    );
    assert!(
        result.2,
        "SetDefaultText and SetVisuals should delegate to existing mixin overrides"
    );
    assert!(
        result.3,
        "UpdateHeight should delegate to an existing mixin override"
    );
}

#[test]
fn test_statusbar_set_color_fill_aliases_statusbar_color_state() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local sb = CreateFrame("StatusBar", "TestStatusBarColorFill", UIParent)
        sb:SetStatusBarTexture("Interface\\Buttons\\WHITE8X8")
        sb:SetColorFill(0.6, 0.5, 0.4, 0.3)
    "#,
    )
    .unwrap();

    let (r, g, b, a): (f32, f32, f32, f32) = env
        .eval("return TestStatusBarColorFill:GetStatusBarColor()")
        .unwrap();
    assert!((r - 0.6).abs() < 0.001);
    assert!((g - 0.5).abs() < 0.001);
    assert!((b - 0.4).abs() < 0.001);
    assert!((a - 0.3).abs() < 0.001);
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

#[test]
fn test_player_model_set_model_persists_path_and_clears_file_id() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local pm = CreateFrame("PlayerModel", "TestPlayerModelSetModel", UIParent)
        pm:SetModel("Creature/Dragon/Dragon.m2")
    "#,
    )
    .unwrap();

    let model_path: String = env
        .eval("return TestPlayerModelSetModel:GetModel()")
        .unwrap();
    assert_eq!(model_path, "Creature/Dragon/Dragon.m2");

    let model_id = env
        .state()
        .borrow()
        .widgets
        .get_id_by_name("TestPlayerModelSetModel")
        .unwrap();
    let state = env.state().borrow();
    let frame = state.widgets.get(model_id).unwrap();
    assert_eq!(
        frame.model_path.as_deref(),
        Some("Creature/Dragon/Dragon.m2")
    );
    assert_eq!(frame.model_file_id, None);
}

#[test]
fn test_player_model_transform_and_camera_methods_persist_state() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local pm = CreateFrame("PlayerModel", "TestPlayerModelTransformCamera", UIParent)
        pm:SetModelScale(1.75)
        pm:SetPosition(10.5, -2.25, 8.0)
        pm:SetFacing(1.125)
        pm:SetCameraDistance(23.5)
        pm:SetCameraFacing(0.875)
        pm:SetCameraTarget(4.0, 5.5, -6.25)
        pm:SetCameraRoll(0.375)
    "#,
    )
    .unwrap();

    let model_scale: f64 = env
        .eval("return TestPlayerModelTransformCamera:GetModelScale()")
        .unwrap();
    let position: (f64, f64, f64) = env
        .eval("return TestPlayerModelTransformCamera:GetPosition()")
        .unwrap();
    let facing: f64 = env
        .eval("return TestPlayerModelTransformCamera:GetFacing()")
        .unwrap();
    let camera_distance: f64 = env
        .eval("return TestPlayerModelTransformCamera:GetCameraDistance()")
        .unwrap();
    let camera_facing: f64 = env
        .eval("return TestPlayerModelTransformCamera:GetCameraFacing()")
        .unwrap();
    let camera_target: (f64, f64, f64) = env
        .eval("return TestPlayerModelTransformCamera:GetCameraTarget()")
        .unwrap();
    let camera_roll: f64 = env
        .eval("return TestPlayerModelTransformCamera:GetCameraRoll()")
        .unwrap();

    assert!((model_scale - 1.75).abs() < 0.001);
    assert!((position.0 - 10.5).abs() < 0.001);
    assert!((position.1 + 2.25).abs() < 0.001);
    assert!((position.2 - 8.0).abs() < 0.001);
    assert!((facing - 1.125).abs() < 0.001);
    assert!((camera_distance - 23.5).abs() < 0.001);
    assert!((camera_facing - 0.875).abs() < 0.001);
    assert!((camera_target.0 - 4.0).abs() < 0.001);
    assert!((camera_target.1 - 5.5).abs() < 0.001);
    assert!((camera_target.2 + 6.25).abs() < 0.001);
    assert!((camera_roll - 0.375).abs() < 0.001);

    let model_id = env
        .state()
        .borrow()
        .widgets
        .get_id_by_name("TestPlayerModelTransformCamera")
        .unwrap();
    let state = env.state().borrow();
    let frame = state.widgets.get(model_id).unwrap();
    assert!((frame.model_transform.scale - 1.75).abs() < 0.001);
    assert!((frame.model_transform.position.0 - 10.5).abs() < 0.001);
    assert!((frame.model_transform.position.1 + 2.25).abs() < 0.001);
    assert!((frame.model_transform.position.2 - 8.0).abs() < 0.001);
    assert!((frame.model_transform.facing - 1.125).abs() < 0.001);
    assert!((frame.model_transform.camera.distance - 23.5).abs() < 0.001);
    assert!((frame.model_transform.camera.facing - 0.875).abs() < 0.001);
    assert!((frame.model_transform.camera.target.0 - 4.0).abs() < 0.001);
    assert!((frame.model_transform.camera.target.1 - 5.5).abs() < 0.001);
    assert!((frame.model_transform.camera.target.2 + 6.25).abs() < 0.001);
    assert!((frame.model_transform.camera.roll - 0.375).abs() < 0.001);
}

#[test]
fn test_player_model_appearance_and_state_methods_persist_and_clear_state() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local pm = CreateFrame("PlayerModel", "TestPlayerModelAppearanceState", UIParent)
        pm:SetModel("Creature/Dragon/Dragon.m2")
        pm:SetDisplayInfo(1234)
    "#,
    )
    .unwrap();

    let display_info: i64 = env
        .eval("return TestPlayerModelAppearanceState:GetDisplayInfo()")
        .unwrap();
    assert_eq!(display_info, 1234);

    let model_id = env
        .state()
        .borrow()
        .widgets
        .get_id_by_name("TestPlayerModelAppearanceState")
        .unwrap();

    {
        let state = env.state().borrow();
        let frame = state.widgets.get(model_id).unwrap();
        assert_eq!(frame.model_path, None);
        assert_eq!(frame.model_file_id, None);
        assert_eq!(frame.model_appearance.display_info, Some(1234));
        assert_eq!(frame.model_appearance.creature_id, None);
    }

    env.exec(
        r#"
        TestPlayerModelAppearanceState:SetCreature(5678)
        TestPlayerModelAppearanceState:SetAnimation(42)
        TestPlayerModelAppearanceState:SetSequence(7)
        TestPlayerModelAppearanceState:RefreshUnit()
        TestPlayerModelAppearanceState:RefreshCamera()
    "#,
    )
    .unwrap();

    let has_animation: bool = env
        .eval("return TestPlayerModelAppearanceState:HasAnimation()")
        .unwrap();
    assert!(
        has_animation,
        "SetAnimation should make HasAnimation return true"
    );

    {
        let state = env.state().borrow();
        let frame = state.widgets.get(model_id).unwrap();
        assert_eq!(frame.model_appearance.display_info, None);
        assert_eq!(frame.model_appearance.creature_id, Some(5678));
        assert_eq!(frame.model_appearance.animation_id, Some(42));
        assert_eq!(frame.model_appearance.sequence_id, Some(7));
        assert_eq!(frame.model_appearance.sequence_time_ms, None);
        assert_eq!(frame.model_appearance.refresh_unit_count, 1);
        assert_eq!(frame.model_appearance.refresh_camera_count, 1);
    }

    env.exec("TestPlayerModelAppearanceState:SetSequenceTime(7, 250)")
        .unwrap();

    {
        let state = env.state().borrow();
        let frame = state.widgets.get(model_id).unwrap();
        assert_eq!(frame.model_appearance.sequence_id, Some(7));
        assert_eq!(frame.model_appearance.sequence_time_ms, Some(250));
    }

    env.exec("TestPlayerModelAppearanceState:ClearModel()")
        .unwrap();

    let cleared: (i64, String, bool) = env
        .eval(
            r#"
            return TestPlayerModelAppearanceState:GetDisplayInfo(),
                   TestPlayerModelAppearanceState:GetModel(),
                   TestPlayerModelAppearanceState:HasAnimation()
        "#,
        )
        .unwrap();
    assert_eq!(cleared.0, 0);
    assert_eq!(cleared.1, "");
    assert!(
        !cleared.2,
        "ClearModel should clear the active animation state"
    );

    {
        let state = env.state().borrow();
        let frame = state.widgets.get(model_id).unwrap();
        assert_eq!(frame.model_path, None);
        assert_eq!(frame.model_file_id, None);
        assert_eq!(frame.model_appearance.display_info, None);
        assert_eq!(frame.model_appearance.creature_id, None);
        assert_eq!(frame.model_appearance.animation_id, None);
        assert_eq!(frame.model_appearance.sequence_id, None);
        assert_eq!(frame.model_appearance.sequence_time_ms, None);
        assert_eq!(frame.model_appearance.refresh_unit_count, 1);
        assert_eq!(frame.model_appearance.refresh_camera_count, 1);
    }
}

#[test]
fn test_player_model_rendering_flag_methods_persist_state() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local pm = CreateFrame("PlayerModel", "TestPlayerModelRenderingFlags", UIParent)
        pm:SetModelAlpha(0.35)
        pm:SetShadowEffect(0.8)
        pm:SetParticlesEnabled(true)
        pm:SetUseGBuffer(true)
    "#,
    )
    .unwrap();

    let render_state: (f64, f64) = env
        .eval(
            r#"
            return TestPlayerModelRenderingFlags:GetModelAlpha(),
                   TestPlayerModelRenderingFlags:GetShadowEffect()
        "#,
        )
        .unwrap();
    assert!((render_state.0 - 0.35).abs() < 0.001);
    assert!((render_state.1 - 0.8).abs() < 0.001);

    let model_id = env
        .state()
        .borrow()
        .widgets
        .get_id_by_name("TestPlayerModelRenderingFlags")
        .unwrap();
    let state = env.state().borrow();
    let frame = state.widgets.get(model_id).unwrap();
    assert!((frame.model_rendering.alpha - 0.35).abs() < 0.001);
    assert!((frame.model_rendering.shadow_effect - 0.8).abs() < 0.001);
    assert!(frame.model_rendering.particles_enabled);
    assert!(frame.model_rendering.use_gbuffer);
}

#[test]
fn test_player_model_specific_methods_persist_state() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local pm = CreateFrame("PlayerModel", "TestPlayerModelSpecificState", UIParent)
        pm:SetDoBlend(true)
        pm:SetKeepModelOnHide(true)
        pm:SetItem(19019)
        pm:SetItemAppearance(4242)
        pm:PlayAnimKit(777)
    "#,
    )
    .unwrap();

    let lua_state: (bool, bool, bool) = env
        .eval(
            r#"
            return TestPlayerModelSpecificState:CanSetUnit(),
                   TestPlayerModelSpecificState:GetDoBlend(),
                   TestPlayerModelSpecificState:GetKeepModelOnHide()
        "#,
        )
        .unwrap();
    assert!(
        lua_state.0,
        "PlayerModel should report unit assignment support"
    );
    assert!(
        lua_state.1,
        "SetDoBlend should round-trip through GetDoBlend"
    );
    assert!(
        lua_state.2,
        "SetKeepModelOnHide should round-trip through GetKeepModelOnHide"
    );

    let model_id = env
        .state()
        .borrow()
        .widgets
        .get_id_by_name("TestPlayerModelSpecificState")
        .unwrap();

    {
        let state = env.state().borrow();
        let frame = state.widgets.get(model_id).unwrap();
        assert!(frame.player_model_state.do_blend);
        assert!(frame.player_model_state.keep_model_on_hide);
        assert_eq!(frame.player_model_state.last_item.as_deref(), Some("19019"));
        assert_eq!(
            frame.player_model_state.last_item_appearance.as_deref(),
            Some("4242")
        );
        assert_eq!(frame.player_model_state.active_anim_kit, Some(777));
    }

    env.exec("TestPlayerModelSpecificState:StopAnimKit()")
        .unwrap();

    let state = env.state().borrow();
    let frame = state.widgets.get(model_id).unwrap();
    assert_eq!(frame.player_model_state.active_anim_kit, None);
}

#[test]
fn test_model_scene_camera_light_and_fog_methods_persist_state() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local scene = CreateFrame("ModelScene", "TestModelSceneState", UIParent)
        scene:SetCameraPosition(1.5, -2.25, 3.75)
        scene:SetCameraOrientationByAxisVectors(0, 0, 1, 1, 0, 0, 0, 1, 0)
        scene:SetCameraFieldOfView(1.125)
        scene:SetCameraNearClip(0.25)
        scene:SetCameraFarClip(250.0)
        scene:SetLightType(2)
        scene:SetLightPosition(4.5, 5.5, -6.5)
        scene:SetLightDirection(0.1, -0.2, 0.3)
        scene:SetLightAmbientColor(0.11, 0.22, 0.33)
        scene:SetLightDiffuseColor(0.44, 0.55, 0.66)
        scene:SetLightVisible(false)
        scene:SetFogNear(7.5)
        scene:SetFogFar(8.5)
        scene:SetFogColor(0.7, 0.8, 0.9)
        scene:SetPaused(true, false)
        scene:SetViewInsets(10, 20, 30, 40)
    "#,
    )
    .unwrap();

    let camera_position: (f64, f64, f64) = env
        .eval("return TestModelSceneState:GetCameraPosition()")
        .unwrap();
    let camera_forward: (f64, f64, f64) = env
        .eval("return TestModelSceneState:GetCameraForward()")
        .unwrap();
    let camera_right: (f64, f64, f64) = env
        .eval("return TestModelSceneState:GetCameraRight()")
        .unwrap();
    let camera_up: (f64, f64, f64) = env
        .eval("return TestModelSceneState:GetCameraUp()")
        .unwrap();
    let field_of_view: f64 = env
        .eval("return TestModelSceneState:GetCameraFieldOfView()")
        .unwrap();
    let near_clip: f64 = env
        .eval("return TestModelSceneState:GetCameraNearClip()")
        .unwrap();
    let far_clip: f64 = env
        .eval("return TestModelSceneState:GetCameraFarClip()")
        .unwrap();
    let light_type: i64 = env
        .eval("return TestModelSceneState:GetLightType()")
        .unwrap();
    let light_position: (f64, f64, f64) = env
        .eval("return TestModelSceneState:GetLightPosition()")
        .unwrap();
    let light_direction: (f64, f64, f64) = env
        .eval("return TestModelSceneState:GetLightDirection()")
        .unwrap();
    let ambient_color: (f64, f64, f64) = env
        .eval("return TestModelSceneState:GetLightAmbientColor()")
        .unwrap();
    let diffuse_color: (f64, f64, f64) = env
        .eval("return TestModelSceneState:GetLightDiffuseColor()")
        .unwrap();
    let light_visible: bool = env
        .eval("return TestModelSceneState:IsLightVisible()")
        .unwrap();
    let fog_near: f64 = env.eval("return TestModelSceneState:GetFogNear()").unwrap();
    let fog_far: f64 = env.eval("return TestModelSceneState:GetFogFar()").unwrap();
    let fog_color: (f64, f64, f64) = env
        .eval("return TestModelSceneState:GetFogColor()")
        .unwrap();
    let paused: bool = env.eval("return TestModelSceneState:GetPaused()").unwrap();
    let view_insets: (f64, f64, f64, f64) = env
        .eval("return TestModelSceneState:GetViewInsets()")
        .unwrap();

    assert!((camera_position.0 - 1.5).abs() < 0.001);
    assert!((camera_position.1 + 2.25).abs() < 0.001);
    assert!((camera_position.2 - 3.75).abs() < 0.001);
    assert_eq!(camera_forward, (0.0, 0.0, 1.0));
    assert_eq!(camera_right, (1.0, 0.0, 0.0));
    assert_eq!(camera_up, (0.0, 1.0, 0.0));
    assert!((field_of_view - 1.125).abs() < 0.001);
    assert!((near_clip - 0.25).abs() < 0.001);
    assert!((far_clip - 250.0).abs() < 0.001);
    assert_eq!(light_type, 2);
    assert!((light_position.0 - 4.5).abs() < 0.001);
    assert!((light_position.1 - 5.5).abs() < 0.001);
    assert!((light_position.2 + 6.5).abs() < 0.001);
    assert!((light_direction.0 - 0.1).abs() < 0.001);
    assert!((light_direction.1 + 0.2).abs() < 0.001);
    assert!((light_direction.2 - 0.3).abs() < 0.001);
    assert!((ambient_color.0 - 0.11).abs() < 0.001);
    assert!((ambient_color.1 - 0.22).abs() < 0.001);
    assert!((ambient_color.2 - 0.33).abs() < 0.001);
    assert!((diffuse_color.0 - 0.44).abs() < 0.001);
    assert!((diffuse_color.1 - 0.55).abs() < 0.001);
    assert!((diffuse_color.2 - 0.66).abs() < 0.001);
    assert!(!light_visible);
    assert!((fog_near - 7.5).abs() < 0.001);
    assert!((fog_far - 8.5).abs() < 0.001);
    assert!((fog_color.0 - 0.7).abs() < 0.001);
    assert!((fog_color.1 - 0.8).abs() < 0.001);
    assert!((fog_color.2 - 0.9).abs() < 0.001);
    assert!(paused);
    assert_eq!(view_insets, (10.0, 20.0, 30.0, 40.0));

    let scene_id = env
        .state()
        .borrow()
        .widgets
        .get_id_by_name("TestModelSceneState")
        .unwrap();
    let state = env.state().borrow();
    let frame = state.widgets.get(scene_id).unwrap();
    assert!((frame.model_scene_state.camera.position.0 - 1.5).abs() < 0.001);
    assert!((frame.model_scene_state.camera.position.1 + 2.25).abs() < 0.001);
    assert!((frame.model_scene_state.camera.position.2 - 3.75).abs() < 0.001);
    assert_eq!(frame.model_scene_state.camera.forward, (0.0, 0.0, 1.0));
    assert_eq!(frame.model_scene_state.camera.right, (1.0, 0.0, 0.0));
    assert_eq!(frame.model_scene_state.camera.up, (0.0, 1.0, 0.0));
    assert!((frame.model_scene_state.camera.field_of_view - 1.125).abs() < 0.001);
    assert!((frame.model_scene_state.camera.near_clip - 0.25).abs() < 0.001);
    assert!((frame.model_scene_state.camera.far_clip - 250.0).abs() < 0.001);
    assert_eq!(frame.model_scene_state.light.light_type, 2);
    assert!((frame.model_scene_state.light.position.0 - 4.5).abs() < 0.001);
    assert!((frame.model_scene_state.light.position.1 - 5.5).abs() < 0.001);
    assert!((frame.model_scene_state.light.position.2 + 6.5).abs() < 0.001);
    assert!((frame.model_scene_state.light.direction.0 - 0.1).abs() < 0.001);
    assert!((frame.model_scene_state.light.direction.1 + 0.2).abs() < 0.001);
    assert!((frame.model_scene_state.light.direction.2 - 0.3).abs() < 0.001);
    assert!((frame.model_scene_state.light.ambient_color.r - 0.11).abs() < 0.001);
    assert!((frame.model_scene_state.light.ambient_color.g - 0.22).abs() < 0.001);
    assert!((frame.model_scene_state.light.ambient_color.b - 0.33).abs() < 0.001);
    assert!((frame.model_scene_state.light.diffuse_color.r - 0.44).abs() < 0.001);
    assert!((frame.model_scene_state.light.diffuse_color.g - 0.55).abs() < 0.001);
    assert!((frame.model_scene_state.light.diffuse_color.b - 0.66).abs() < 0.001);
    assert!(!frame.model_scene_state.light.visible);
    assert!((frame.model_scene_state.fog.near - 7.5).abs() < 0.001);
    assert!((frame.model_scene_state.fog.far - 8.5).abs() < 0.001);
    assert!((frame.model_scene_state.fog.color.r - 0.7).abs() < 0.001);
    assert!((frame.model_scene_state.fog.color.g - 0.8).abs() < 0.001);
    assert!((frame.model_scene_state.fog.color.b - 0.9).abs() < 0.001);
    assert!(frame.model_scene_state.paused);
    assert_eq!(
        frame.model_scene_state.view_insets,
        (10.0, 20.0, 30.0, 40.0)
    );
}

#[test]
fn test_model_scene_project_3d_point_uses_camera_projection() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local scene = CreateFrame("ModelScene", "TestModelSceneProjection", UIParent)
        scene:SetSize(400, 200)
        scene:SetCameraPosition(1.0, 2.0, 3.0)
        scene:SetCameraFieldOfView(1.0)
        scene:SetViewInsets(10, 20, 30, 40)
        scene:SetViewTranslation(12, -6)

        _G.scene_projection = {
            center = { scene:Project3DPointTo2D(1.0, 2.0, 13.0) },
            offset = { scene:Project3DPointTo2D(3.0, 4.0, 13.0) },
            behind = { scene:Project3DPointTo2D(1.0, 2.0, 2.0) },
        }
    "#,
    )
    .unwrap();

    let center: (f64, f64, f64) = env
        .eval(
            r#"
            local p = _G.scene_projection.center
            return p[1], p[2], p[3]
        "#,
        )
        .unwrap();
    let offset: (f64, f64, f64) = env
        .eval(
            r#"
            local p = _G.scene_projection.offset
            return p[1], p[2], p[3]
        "#,
        )
        .unwrap();
    let behind: mlua::Value = env.eval("_G.scene_projection.behind[1]").unwrap();

    assert!((center.0 - 197.0).abs() < 0.001);
    assert!((center.1 - 59.0).abs() < 0.001);
    assert!((center.2 - 0.9009009).abs() < 0.001);
    assert!((offset.0 - 220.796340).abs() < 0.001);
    assert!((offset.1 - 82.796340).abs() < 0.001);
    assert!((offset.2 - 0.9009009).abs() < 0.001);
    assert!(matches!(behind, mlua::Value::Nil));
}

#[test]
fn test_model_scene_actor_management_tracks_created_indexed_and_taken_actors() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local scene = CreateFrame("ModelScene", "TestModelSceneActors", UIParent)
        local actor1 = scene:CreateActor("FirstActor", "ModelSceneActorTemplate")
        local actor2 = scene:CreateActor("SecondActor", "ModelSceneActorTemplate")
        local count_after_create = scene:GetNumActors()
        local actor1_is_index1 = scene:GetActorAtIndex(1) == actor1
        local actor2_is_index2 = scene:GetActorAtIndex(2) == actor2
        local missing = scene:GetActorAtIndex(3)
        local taken = scene:TakeActor()

        _G.actor_scene_state = {
            actor1_ok = actor1 ~= nil,
            actor2_ok = actor2 ~= nil,
            count_after_create = count_after_create,
            actor1_is_index1 = actor1_is_index1,
            actor2_is_index2 = actor2_is_index2,
            missing_is_nil = missing == nil,
            taken_is_actor2 = taken == actor2,
            count_after_take = scene:GetNumActors(),
            actor1_still_index1 = scene:GetActorAtIndex(1) == actor1,
            actor2_removed = scene:GetActorAtIndex(2) == nil,
        }
    "#,
    )
    .unwrap();

    let actor_state: (bool, bool, i64, bool, bool, bool, bool, i64, bool, bool) = env
        .eval(
            r#"
            local s = _G.actor_scene_state
            return s.actor1_ok,
                   s.actor2_ok,
                   s.count_after_create,
                   s.actor1_is_index1,
                   s.actor2_is_index2,
                   s.missing_is_nil,
                   s.taken_is_actor2,
                   s.count_after_take,
                   s.actor1_still_index1,
                   s.actor2_removed
        "#,
        )
        .unwrap();

    assert!(actor_state.0);
    assert!(actor_state.1);
    assert_eq!(actor_state.2, 2);
    assert!(actor_state.3);
    assert!(actor_state.4);
    assert!(actor_state.5);
    assert!(actor_state.6);
    assert_eq!(actor_state.7, 1);
    assert!(actor_state.8);
    assert!(actor_state.9);

    let scene_id = env
        .state()
        .borrow()
        .widgets
        .get_id_by_name("TestModelSceneActors")
        .unwrap();
    let first_actor_id = env
        .state()
        .borrow()
        .widgets
        .get_id_by_name("FirstActor")
        .unwrap();
    let second_actor_id = env
        .state()
        .borrow()
        .widgets
        .get_id_by_name("SecondActor")
        .unwrap();

    let state = env.state().borrow();
    let scene = state.widgets.get(scene_id).unwrap();
    assert_eq!(scene.model_scene_actor_ids, vec![first_actor_id]);

    let first_actor = state.widgets.get(first_actor_id).unwrap();
    assert_eq!(first_actor.parent_id, Some(scene_id));
    assert_eq!(
        first_actor.object_type_name.as_deref(),
        Some("ModelSceneActor")
    );

    let second_actor = state.widgets.get(second_actor_id).unwrap();
    assert_eq!(second_actor.parent_id, None);
    assert_eq!(
        second_actor.object_type_name.as_deref(),
        Some("ModelSceneActor")
    );
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
