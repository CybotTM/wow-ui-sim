//! Tests for button-specific methods in methods_button.rs.
//!
//! Covers: font objects, texture getters/setters, enable/disable state,
//! click handling, RegisterForClicks, button state, GetFontString, and
//! three-slice texture methods.

use wow_ui_sim::lua_api::WowLuaEnv;

// ============================================================================
// Font Object Methods
// ============================================================================

#[test]
fn test_set_and_get_normal_font_object() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local btn = CreateFrame("Button", "TestFontObjBtn", UIParent)
        local font = CreateFrame("Frame", "TestFontObj", UIParent)
        btn:SetNormalFontObject(font)
    "#,
    )
    .unwrap();

    let result: bool = env
        .eval("return TestFontObjBtn:GetNormalFontObject() == TestFontObj")
        .unwrap();
    assert!(
        result,
        "GetNormalFontObject should return the font set via SetNormalFontObject"
    );
}

#[test]
fn test_set_and_get_highlight_font_object() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local btn = CreateFrame("Button", "TestHlFontBtn", UIParent)
        local font = CreateFrame("Frame", "TestHlFont", UIParent)
        btn:SetHighlightFontObject(font)
    "#,
    )
    .unwrap();

    let result: bool = env
        .eval("return TestHlFontBtn:GetHighlightFontObject() == TestHlFont")
        .unwrap();
    assert!(
        result,
        "GetHighlightFontObject should return the font set via SetHighlightFontObject"
    );
}

#[test]
fn test_set_and_get_disabled_font_object() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local btn = CreateFrame("Button", "TestDisFontBtn", UIParent)
        local font = CreateFrame("Frame", "TestDisFont", UIParent)
        btn:SetDisabledFontObject(font)
    "#,
    )
    .unwrap();

    let result: bool = env
        .eval("return TestDisFontBtn:GetDisabledFontObject() == TestDisFont")
        .unwrap();
    assert!(
        result,
        "GetDisabledFontObject should return the font set via SetDisabledFontObject"
    );
}

#[test]
fn test_get_font_object_returns_nil_when_unset() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local btn = CreateFrame("Button", "TestNoFontBtn", UIParent)
    "#,
    )
    .unwrap();

    let normal_nil: bool = env
        .eval("return TestNoFontBtn:GetNormalFontObject() == nil")
        .unwrap();
    let highlight_nil: bool = env
        .eval("return TestNoFontBtn:GetHighlightFontObject() == nil")
        .unwrap();
    let disabled_nil: bool = env
        .eval("return TestNoFontBtn:GetDisabledFontObject() == nil")
        .unwrap();

    assert!(
        normal_nil,
        "GetNormalFontObject should return nil when unset"
    );
    assert!(
        highlight_nil,
        "GetHighlightFontObject should return nil when unset"
    );
    assert!(
        disabled_nil,
        "GetDisabledFontObject should return nil when unset"
    );
}

// ============================================================================
// Pushed Text Offset Methods
// ============================================================================

#[test]
fn test_pushed_text_offset() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local btn = CreateFrame("Button", "TestPushOffBtn", UIParent)
        btn:SetPushedTextOffset(2.5, -1.0)
    "#,
    )
    .unwrap();

    let (x, y): (f64, f64) = env
        .eval("return TestPushOffBtn:GetPushedTextOffset()")
        .unwrap();
    assert_eq!(x, 2.5);
    assert_eq!(y, -1.0);
}

#[test]
fn test_clear_button_texture_methods_clear_parent_fields_and_child_textures() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local btn = CreateFrame("Button", "TestClearBtnTex", UIParent)
        btn:SetNormalTexture("Interface/Buttons/UI-Panel-Button-Up")
        btn:SetHighlightTexture("Interface/Buttons/UI-Panel-Button-Highlight")
        btn:SetPushedTexture("Interface/Buttons/UI-Panel-Button-Down")
        btn:SetDisabledTexture("Interface/Buttons/UI-Panel-Button-Disabled")
        btn:ClearNormalTexture()
        btn:ClearHighlightTexture()
        btn:ClearPushedTexture()
        btn:ClearDisabledTexture()
    "#,
    )
    .unwrap();

    let state = env.state().borrow();
    let button_id = state.widgets.get_id_by_name("TestClearBtnTex").unwrap();
    let button = state.widgets.get(button_id).unwrap();
    assert_eq!(button.normal_texture, None);
    assert_eq!(button.highlight_texture, None);
    assert_eq!(button.pushed_texture, None);
    assert_eq!(button.disabled_texture, None);

    for key in [
        "NormalTexture",
        "HighlightTexture",
        "PushedTexture",
        "DisabledTexture",
    ] {
        let child_id = button
            .children_keys
            .get(key)
            .copied()
            .unwrap_or_else(|| panic!("Expected {key} child to exist"));
        let child = state.widgets.get(child_id).unwrap();
        assert_eq!(
            child.texture, None,
            "{key} child texture should be cleared on the child widget too"
        );
    }
}

// ============================================================================
// GetFontString Method
// ============================================================================

#[test]
fn test_get_font_string_returns_text_child() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local btn = CreateFrame("Button", "TestGetFontStr", UIParent)
        btn:SetText("Hello")
    "#,
    )
    .unwrap();

    // Verify GetFontString returns the Text child (a FontString)
    let not_nil: bool = env
        .eval("return TestGetFontStr:GetFontString() ~= nil")
        .unwrap();
    assert!(not_nil, "GetFontString should return a non-nil value");

    let obj_type: String = env
        .eval("return TestGetFontStr:GetFontString():GetObjectType()")
        .unwrap();
    assert_eq!(
        obj_type, "FontString",
        "GetFontString should return a FontString"
    );

    // Verify the Rust side has the Text child registered
    let state = env.state().borrow();
    let btn_id = state.widgets.get_id_by_name("TestGetFontStr").unwrap();
    let btn = state.widgets.get(btn_id).unwrap();
    assert!(
        btn.children_keys.contains_key("Text"),
        "Button should have a Text child in children_keys"
    );
}

#[test]
fn test_get_font_string_nil_when_no_text() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local frame = CreateFrame("Button", "TestNoTextFrame", UIParent)
    "#,
    )
    .unwrap();

    let is_nil: bool = env
        .eval("return TestNoTextFrame:GetFontString() == nil")
        .unwrap();
    assert!(
        is_nil,
        "GetFontString should return nil for frame with no Text child"
    );
}

// ============================================================================
// Enable/Disable State Methods
// ============================================================================

#[test]
fn test_button_enabled_by_default() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(r#"local btn = CreateFrame("Button", "TestEnabledDef", UIParent)"#)
        .unwrap();

    let enabled: bool = env.eval("return TestEnabledDef:IsEnabled()").unwrap();
    assert!(enabled, "Buttons should be enabled by default");
}

#[test]
fn test_set_enabled_false() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local btn = CreateFrame("Button", "TestSetEnFalse", UIParent)
        btn:SetEnabled(false)
    "#,
    )
    .unwrap();

    let enabled: bool = env.eval("return TestSetEnFalse:IsEnabled()").unwrap();
    assert!(
        !enabled,
        "Button should be disabled after SetEnabled(false)"
    );
}

#[test]
fn test_set_enabled_true() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local btn = CreateFrame("Button", "TestSetEnTrue", UIParent)
        btn:SetEnabled(false)
        btn:SetEnabled(true)
    "#,
    )
    .unwrap();

    let enabled: bool = env.eval("return TestSetEnTrue:IsEnabled()").unwrap();
    assert!(enabled, "Button should be enabled after SetEnabled(true)");
}

#[test]
fn test_enable_method() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local btn = CreateFrame("Button", "TestEnMethod", UIParent)
        btn:SetEnabled(false)
        btn:Enable()
    "#,
    )
    .unwrap();

    let enabled: bool = env.eval("return TestEnMethod:IsEnabled()").unwrap();
    assert!(enabled, "Button should be enabled after Enable()");
}

#[test]
fn test_disable_method() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local btn = CreateFrame("Button", "TestDisMethod", UIParent)
        btn:Disable()
    "#,
    )
    .unwrap();

    let enabled: bool = env.eval("return TestDisMethod:IsEnabled()").unwrap();
    assert!(!enabled, "Button should be disabled after Disable()");
}

#[test]
fn test_motion_scripts_while_disabled_round_trip() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local btn = CreateFrame("Button", "TestMotionScriptsWhileDisabled", UIParent)
        __motion_default = btn:GetMotionScriptsWhileDisabled()
        btn:SetMotionScriptsWhileDisabled(true)
        __motion_enabled = btn:GetMotionScriptsWhileDisabled()
        btn:SetMotionScriptsWhileDisabled(false)
        __motion_disabled = btn:GetMotionScriptsWhileDisabled()
    "#,
    )
    .unwrap();

    let (default_state, enabled_state, disabled_state): (bool, bool, bool) = env
        .eval("return __motion_default, __motion_enabled, __motion_disabled")
        .unwrap();

    assert!(
        !default_state,
        "motion scripts should default to disabled on new buttons"
    );
    assert!(
        enabled_state,
        "SetMotionScriptsWhileDisabled(true) should persist true"
    );
    assert!(
        !disabled_state,
        "SetMotionScriptsWhileDisabled(false) should clear the flag"
    );
}

// ============================================================================
// Click Method
// ============================================================================

#[test]
fn test_click_fires_onclick_handler() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local btn = CreateFrame("Button", "TestClickBtn", UIParent)
        __test_click_fired = false
        btn:SetScript("OnClick", function(self, button, down)
            __test_click_fired = true
            __test_click_button = button
            __test_click_down = down
        end)
        btn:Click()
    "#,
    )
    .unwrap();

    let fired: bool = env.eval("return __test_click_fired").unwrap();
    let button: String = env.eval("return __test_click_button").unwrap();
    let down: bool = env.eval("return __test_click_down").unwrap();

    assert!(fired, "Click() should fire OnClick handler");
    assert_eq!(button, "LeftButton", "Click() should pass 'LeftButton'");
    assert!(!down, "Click() should pass false for down");
}

#[test]
fn test_click_no_handler_does_not_error() {
    let env = WowLuaEnv::new().unwrap();

    // Click() on button with no OnClick handler should not error
    env.exec(
        r#"
        local btn = CreateFrame("Button", "TestClickNoHandler", UIParent)
        btn:Click()
    "#,
    )
    .unwrap();
}

// ============================================================================
// RegisterForClicks Method
// ============================================================================

#[test]
fn test_register_for_clicks_no_error() {
    let env = WowLuaEnv::new().unwrap();

    // RegisterForClicks is a stub but should not error
    env.exec(
        r#"
        local btn = CreateFrame("Button", "TestRegClicks", UIParent)
        btn:RegisterForClicks("AnyUp")
        btn:RegisterForClicks("LeftButtonUp", "RightButtonUp")
    "#,
    )
    .unwrap();
}

// ============================================================================
// Button State Methods
// ============================================================================

#[test]
fn test_set_and_get_button_state() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local btn = CreateFrame("Button", "TestBtnState", UIParent)
        btn:SetButtonState("PUSHED", true)
    "#,
    )
    .unwrap();

    let state: String = env.eval("return TestBtnState:GetButtonState()").unwrap();
    assert_eq!(state, "PUSHED");
}

// ============================================================================
// SetFontString Method (stub)
// ============================================================================

#[test]
fn test_set_font_string_no_error() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local btn = CreateFrame("Button", "TestSetFontStr", UIParent)
        local fs = btn:GetFontString()
        btn:SetFontString(fs)
    "#,
    )
    .unwrap();
}
