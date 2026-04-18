use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn spell_alert_manager_tracks_button_alert_state() {
    let env = env();
    let (before, during, alert_type, after): (bool, bool, i32, bool) = env
        .eval(
            r#"
            local button = CreateFrame("Button", "SpellAlertStateButton", UIParent)
            local before = ActionButtonSpellAlertManager:HasAlert(button)

            ActionButtonSpellAlertManager:ShowAlert(button)
            local during, alertType = ActionButtonSpellAlertManager:HasAlert(button)

            ActionButtonSpellAlertManager:HideAlert(button)
            local after = ActionButtonSpellAlertManager:HasAlert(button)

            return before, during, alertType, after
            "#,
        )
        .unwrap();
    assert!(!before, "button should start without an alert");
    assert!(during, "ShowAlert should register active alert state");
    assert_eq!(
        alert_type, 1,
        "default startup stub should record the default alert type"
    );
    assert!(!after, "HideAlert should clear active alert state");
}

#[test]
fn spell_alert_manager_creates_and_toggles_alert_frame() {
    let env = env();
    let (has_frame, shown_during, shown_after, same_frame): (bool, bool, bool, bool) = env
        .eval(
            r#"
            local button = CreateFrame("Button", "SpellAlertVisualButton", UIParent)

            ActionButtonSpellAlertManager:ShowAlert(button)
            local firstFrame = button.SpellActivationAlert
            local hasFrame = firstFrame ~= nil
            local shownDuring = firstFrame ~= nil and firstFrame:IsShown()

            ActionButtonSpellAlertManager:HideAlert(button)
            local shownAfter = firstFrame ~= nil and firstFrame:IsShown()

            ActionButtonSpellAlertManager:ShowAlert(button)
            local sameFrame = firstFrame ~= nil and firstFrame == button.SpellActivationAlert

            return hasFrame, shownDuring, shownAfter, sameFrame
            "#,
        )
        .unwrap();
    assert!(
        has_frame,
        "ShowAlert should expose button.SpellActivationAlert"
    );
    assert!(
        shown_during,
        "ShowAlert should create and show an alert frame"
    );
    assert!(!shown_after, "HideAlert should hide the alert frame");
    assert!(
        same_frame,
        "subsequent alerts should reuse the existing alert frame"
    );
}
