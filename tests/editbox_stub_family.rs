use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn editbox_stub_family_methods_persist_runtime_state() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local eb = CreateFrame("EditBox", "StubFamilyEB", UIParent)

            if eb:GetAltArrowKeyMode() ~= false then
                return "alt_arrow_should_default_false"
            end
            if eb:IsAlphabeticOnly() ~= false then
                return "alphabetic_only_should_default_false"
            end
            if eb:IsNumericFullRange() ~= false then
                return "numeric_full_range_should_default_false"
            end
            if eb:IsSecureText() ~= false then
                return "secure_text_should_default_false"
            end
            if eb:GetVisibleTextByteLimit() ~= 0 then
                return "visible_text_byte_limit_should_default_zero"
            end
            if eb:GetInputLanguage() ~= "ROMAN" then
                return "input_language_should_default_roman"
            end
            if eb:HasText() ~= false then
                return "has_text_should_default_false"
            end
            local r, g, b, a = eb:GetHighlightColor()
            if r ~= 1 or g ~= 1 or b ~= 1 or a ~= 1 then
                return "highlight_color_should_default_white"
            end

            eb:SetAltArrowKeyMode(true)
            eb:SetAlphabeticOnly(true)
            eb:SetNumericFullRange(true)
            eb:SetSecureText(true)
            eb:SetVisibleTextByteLimit(32)
            eb:SetSecurityDisablePaste()
            eb:SetHighlightColor(0.25, 0.5, 0.75, 0.9)

            eb:AddHistoryLine("one")
            eb:AddHistoryLine("two")
            if eb:GetHistoryLines() ~= 2 then
                return "history_should_have_two_lines"
            end
            eb:ClearHistory()
            if eb:GetHistoryLines() ~= 0 then
                return "clear_history_should_empty_history"
            end

            eb:ToggleInputLanguage()
            if eb:GetInputLanguage() ~= "NATIVE" then
                return "toggle_input_language_should_switch_to_native"
            end
            eb:ResetInputMode()

            if eb:GetAltArrowKeyMode() ~= true then
                return "alt_arrow_state_not_persisted"
            end
            if eb:IsAlphabeticOnly() ~= true then
                return "alphabetic_only_state_not_persisted"
            end
            if eb:IsNumericFullRange() ~= true then
                return "numeric_full_range_state_not_persisted"
            end
            if eb:IsSecureText() ~= true then
                return "secure_text_state_not_persisted"
            end
            if eb:GetVisibleTextByteLimit() ~= 32 then
                return "visible_text_byte_limit_not_persisted"
            end
            if eb:GetInputLanguage() ~= "ROMAN" then
                return "reset_input_mode_should_restore_roman"
            end

            eb:SetText("Visible text")
            if eb:HasText() ~= true then
                return "has_text_should_be_true_after_set_text"
            end
            if eb:GetDisplayText() ~= "Visible text" then
                return "display_text_should_reflect_current_text"
            end
            local hr, hg, hb, ha = eb:GetHighlightColor()
            local function approx_eq(a, b)
                return math.abs(a - b) < 0.0001
            end
            if not approx_eq(hr, 0.25) or not approx_eq(hg, 0.5) or not approx_eq(hb, 0.75) or not approx_eq(ha, 0.9) then
                return "highlight_color_should_round_trip"
            end
            eb:HighlightText(2, 7)
            eb:ClearHighlightText()
            eb:HighlightText()

            return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(
        result, "ok",
        "EditBox stub-family methods should persist real state instead of no-oping"
    );

    let frame_id = env
        .state()
        .borrow()
        .widgets
        .get_id_by_name("StubFamilyEB")
        .expect("StubFamilyEB should exist");
    let state = env.state().borrow();
    let frame = state
        .widgets
        .get(frame_id)
        .expect("StubFamilyEB frame should exist");
    assert!(
        frame.editbox_security_disable_paste,
        "SetSecurityDisablePaste should persist the editbox paste-disable flag"
    );
    assert_eq!(
        frame.editbox_highlight_color,
        wow_ui_sim::widget::Color::new(0.25, 0.5, 0.75, 0.9),
        "SetHighlightColor should persist the editbox highlight color"
    );
    assert_eq!(
        frame.editbox_highlight_range,
        Some((0, "Visible text".chars().count() as i32)),
        "HighlightText() without args should select the full current text"
    );
}
