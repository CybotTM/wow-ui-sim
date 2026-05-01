//! Tests for EditBox, CheckButton, SimpleHTML, and frame property methods.
//! Widget misc tests are in `widget_misc_methods.rs`.

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

#[test]
fn test_button_get_text_height_measures_button_text() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local button = CreateFrame("Button", "TextHeightButton", UIParent)
        button:SetText("Quest")
    "#,
    )
    .unwrap();

    let height: f64 = env.eval("return TextHeightButton:GetTextHeight()").unwrap();
    assert!(
        height > 0.0,
        "Button:GetTextHeight should measure button text"
    );
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

#[test]
fn test_simplehtml_indented_word_wrap_round_trips_by_text_type() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local sh = CreateFrame("SimpleHTML", "TestSHIndentWrap", UIParent)
        sh:SetIndentedWordWrap("p", true)
    "#,
    )
    .unwrap();

    let enabled: bool = env
        .eval(r#"return TestSHIndentWrap:GetIndentedWordWrap("p")"#)
        .unwrap();
    assert!(
        enabled,
        "SimpleHTML should report the stored indented word wrap state for the text type"
    );

    let default_other: bool = env
        .eval(r#"return TestSHIndentWrap:GetIndentedWordWrap("h1")"#)
        .unwrap();
    assert!(
        !default_other,
        "SimpleHTML should default to false for text types without stored wrap state"
    );
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
// StartSizing / StopMovingOrSizing / Resize bounds
// ============================================================================

#[test]
fn test_start_sizing_requires_resizable() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local f = CreateFrame("Frame", "TestSizingNotResizable", UIParent)
        f:SetSize(200, 100)
        f:StartSizing("BOTTOMRIGHT")
    "#,
    )
    .unwrap();

    let state = env.state().borrow();
    let id = state
        .widgets
        .get_id_by_name("TestSizingNotResizable")
        .unwrap();
    let frame = state.widgets.get(id).unwrap();
    assert!(
        !frame.is_sizing,
        "StartSizing should be ignored when frame is not resizable"
    );
}

#[test]
fn test_start_sizing_sets_state() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local f = CreateFrame("Frame", "TestSizingState", UIParent)
        f:SetSize(200, 100)
        f:SetResizable(true)
        f:StartSizing("BOTTOMRIGHT")
    "#,
    )
    .unwrap();

    let state = env.state().borrow();
    let id = state.widgets.get_id_by_name("TestSizingState").unwrap();
    let frame = state.widgets.get(id).unwrap();
    assert!(frame.is_sizing, "StartSizing should set is_sizing flag");
    assert_eq!(
        frame.sizing_point,
        wow_ui_sim::widget::AnchorPoint::BottomRight
    );
}

#[test]
fn test_start_sizing_bottomleft() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local f = CreateFrame("Frame", "TestSizingBL", UIParent)
        f:SetSize(200, 100)
        f:SetResizable(true)
        f:StartSizing("BOTTOMLEFT")
    "#,
    )
    .unwrap();

    let state = env.state().borrow();
    let id = state.widgets.get_id_by_name("TestSizingBL").unwrap();
    let frame = state.widgets.get(id).unwrap();
    assert!(frame.is_sizing);
    assert_eq!(
        frame.sizing_point,
        wow_ui_sim::widget::AnchorPoint::BottomLeft
    );
}

#[test]
fn test_start_sizing_defaults_to_bottomright() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local f = CreateFrame("Frame", "TestSizingDefault", UIParent)
        f:SetSize(200, 100)
        f:SetResizable(true)
        f:StartSizing()
    "#,
    )
    .unwrap();

    let state = env.state().borrow();
    let id = state.widgets.get_id_by_name("TestSizingDefault").unwrap();
    let frame = state.widgets.get(id).unwrap();
    assert!(frame.is_sizing);
    assert_eq!(
        frame.sizing_point,
        wow_ui_sim::widget::AnchorPoint::BottomRight,
        "StartSizing with no argument should default to BOTTOMRIGHT"
    );
}

#[test]
fn test_stop_moving_or_sizing_clears_sizing() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local f = CreateFrame("Frame", "TestStopSizing", UIParent)
        f:SetSize(200, 100)
        f:SetResizable(true)
        f:StartSizing("BOTTOMRIGHT")
    "#,
    )
    .unwrap();

    {
        let state = env.state().borrow();
        let id = state.widgets.get_id_by_name("TestStopSizing").unwrap();
        assert!(state.widgets.get(id).unwrap().is_sizing);
    }

    env.exec("TestStopSizing:StopMovingOrSizing()").unwrap();

    let state = env.state().borrow();
    let id = state.widgets.get_id_by_name("TestStopSizing").unwrap();
    let frame = state.widgets.get(id).unwrap();
    assert!(
        !frame.is_sizing,
        "StopMovingOrSizing should clear is_sizing"
    );
    assert!(
        frame.user_placed,
        "StopMovingOrSizing should set user_placed after sizing"
    );
}

#[test]
fn test_resize_bounds_clamp() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local f = CreateFrame("Frame", "TestResizeBounds", UIParent)
        f:SetSize(200, 100)
        f:SetResizable(true)
        f:SetResizeBounds(100, 50, 300, 200)
    "#,
    )
    .unwrap();

    let state = env.state().borrow();
    let id = state.widgets.get_id_by_name("TestResizeBounds").unwrap();
    let frame = state.widgets.get(id).unwrap();
    assert_eq!(frame.resize_bounds_min, (100.0, 50.0));
    assert_eq!(frame.resize_bounds_max, Some((300.0, 200.0)));
}

// ============================================================================
// Alpha: SetAlpha / GetAlpha on frames
// ============================================================================

#[test]
fn test_set_alpha_zero_persists() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local f = CreateFrame("Button", "TestAlphaZeroBtn", UIParent)
        f:SetAlpha(0)
    "#,
    )
    .unwrap();

    let alpha: f64 = env.eval("return TestAlphaZeroBtn:GetAlpha()").unwrap();
    assert!(
        alpha.abs() < 0.001,
        "Button with SetAlpha(0) should have alpha=0, got {alpha}"
    );

    let state = env.state().borrow();
    let id = state.widgets.get_id_by_name("TestAlphaZeroBtn").unwrap();
    let frame = state.widgets.get(id).unwrap();
    assert!(
        frame.alpha.abs() < 0.001,
        "Rust frame.alpha should be 0, got {}",
        frame.alpha
    );
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
