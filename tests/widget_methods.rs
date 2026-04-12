//! Tests for EditBox, CheckButton, widget misc, SimpleHTML, and frame property methods.

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
// widget_misc
// ============================================================================

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
fn test_widget_misc_is_menu_open_uses_dropdown_menu_state() {
    let env = WowLuaEnv::new().unwrap();

    let result: (bool, bool) = env
        .eval(
            r#"
            local frame = CreateFrame("Frame", "TestWidgetMiscMenuFrame", UIParent)
            local fields = debug.getfenv(frame)[1]
            local closed = frame:IsMenuOpen() == false

            fields.menu = {}
            local opened = frame:IsMenuOpen() == true

            return closed, opened
        "#,
        )
        .unwrap();

    assert!(result.0, "Frames should report no menu by default");
    assert!(
        result.1,
        "Frames should report an open menu when their active menu field is set"
    );
}

#[test]
fn test_widget_misc_set_owning_dialog_delegates_or_stores_state() {
    let env = WowLuaEnv::new().unwrap();

    let result: (bool, bool) = env
        .eval(
            r#"
            local frame = CreateFrame("Frame", "TestWidgetMiscOwningDialogFrame", UIParent)
            local dialog = CreateFrame("Frame", "TestWidgetMiscOwningDialogDialog", UIParent)
            local fields = debug.getfenv(frame)[1]
            frame:SetOwningDialog(dialog)
            local stored = fields.owningDialog == dialog

            local overrideFrame = CreateFrame("Frame", "TestWidgetMiscOwningDialogOverrideFrame", UIParent)
            local overrideDialog = CreateFrame("Frame", "TestWidgetMiscOwningDialogOverrideDialog", UIParent)
            local overrideFields = debug.getfenv(overrideFrame)[1]
            overrideFields.SetOwningDialog = function(self, value)
                rawset(overrideFields, "storedDialog", value)
            end
            overrideFrame:SetOwningDialog(overrideDialog)
            local delegated = overrideFields.storedDialog == overrideDialog

            return stored, delegated
        "#,
        )
        .unwrap();

    assert!(
        result.0,
        "SetOwningDialog should store the owning dialog on the frame"
    );
    assert!(
        result.1,
        "SetOwningDialog should delegate to an existing mixin override instead of shadowing it"
    );
}

#[test]
fn test_widget_misc_registration_methods_delegate_or_store_state() {
    let env = WowLuaEnv::new().unwrap();

    let result: (bool, bool, bool, bool) = env
        .eval(
            r##"
            local frame = CreateFrame("Frame", "TestWidgetMiscRegistrationFrame", UIParent)
            local fontA = frame:CreateFontString(nil, "OVERLAY")
            local fontB = frame:CreateFontString(nil, "OVERLAY")
            local childA = CreateFrame("Frame", nil, frame)
            local childB = CreateFrame("Frame", nil, frame)
            local background = frame:CreateTexture(nil, "BACKGROUND")
            local fields = debug.getfenv(frame)[1]

            frame:RegisterFontStrings(fontA, fontB)
            frame:RegisterFrames(childA, childB)
            frame:RegisterBackgroundTexture(background, "guildrename")

            local fontStringsStored = fields.fontStrings and fields.fontStrings[1] == fontA and fields.fontStrings[2] == fontB
            local framesStored = fields.frames and fields.frames[1] == childA and fields.frames[2] == childB
            local backgroundStored = fields.backgroundTexture == background and fields.textureKit == "guildrename"

            local overrideFrame = CreateFrame("Frame", "TestWidgetMiscRegistrationOverrideFrame", UIParent)
            local overrideFont = overrideFrame:CreateFontString(nil, "OVERLAY")
            local overrideFields = debug.getfenv(overrideFrame)[1]
            overrideFields.RegisterFontStrings = function(self, ...)
                rawset(overrideFields, "storedCount", select("#", ...))
                rawset(overrideFields, "storedFirst", ...)
            end
            overrideFrame:RegisterFontStrings(overrideFont)
            local delegated = overrideFields.storedCount == 1 and overrideFields.storedFirst == overrideFont

            return fontStringsStored, framesStored, backgroundStored, delegated
        "##,
        )
        .unwrap();

    assert!(
        result.0,
        "RegisterFontStrings should store the registered font strings on the frame"
    );
    assert!(
        result.1,
        "RegisterFrames should store the registered frames on the frame"
    );
    assert!(
        result.2,
        "RegisterBackgroundTexture should store the background texture and texture kit"
    );
    assert!(
        result.3,
        "RegisterFontStrings should delegate to an existing mixin override instead of shadowing it"
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
fn test_widget_misc_widget_set_methods_delegate_or_store_registration() {
    let env = WowLuaEnv::new().unwrap();

    let result: (bool, bool, bool) = env
        .eval(
            r##"
            local frame = CreateFrame("Frame", "TestWidgetSetFrame", UIParent)
            local fields = debug.getfenv(frame)[1]
            local layout = function() end
            local init = function() end

            frame:RegisterForWidgetSet(5501, layout, init, "player")
            local fallbackStored = fields.widgetSetRegistration
                and fields.widgetSetRegistration.widgetSetID == 5501
                and fields.widgetSetRegistration.widgetLayoutFunction == layout
                and fields.widgetSetRegistration.widgetInitFunction == init
                and fields.widgetSetRegistration.attachedUnitInfo == "player"

            frame:UnregisterForWidgetSet()
            local fallbackCleared = fields.widgetSetRegistration == nil

            local overrideFrame = CreateFrame("Frame", "TestWidgetSetOverrideFrame", UIParent)
            local overrideFields = debug.getfenv(overrideFrame)[1]
            overrideFields.RegisterForWidgetSet = function(self, ...)
                rawset(overrideFields, "registerCount", select("#", ...))
                rawset(overrideFields, "registeredWidgetSetID", ...)
            end
            overrideFields.UnregisterForWidgetSet = function(self, ...)
                rawset(overrideFields, "unregisterCount", select("#", ...))
                rawset(overrideFields, "unregisteredWidgetSetID", ...)
            end

            overrideFrame:RegisterForWidgetSet(4402, layout, init, "target")
            overrideFrame:UnregisterForWidgetSet(4402)

            return fallbackStored == true,
                fallbackCleared == true,
                overrideFields.registerCount == 4
                    and overrideFields.registeredWidgetSetID == 4402
                    and overrideFields.unregisterCount == 1
                    and overrideFields.unregisteredWidgetSetID == 4402
        "##,
        )
        .unwrap();

    assert!(
        result.0,
        "RegisterForWidgetSet should store fallback registration data on the frame"
    );
    assert!(
        result.1,
        "UnregisterForWidgetSet should clear the fallback widget set registration"
    );
    assert!(
        result.2,
        "Widget set methods should delegate to existing mixin overrides"
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
