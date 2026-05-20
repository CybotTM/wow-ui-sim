use super::test_support::*;
use super::*;
use crate::screen::ScreenKind;

#[test]
fn party_member_button_click_targets_party_unit_through_mouse_path() {
    let mut app = build_test_app(ScreenKind::Game);

    {
        let env = app.env.borrow();
        env.exec(
            r#"
            A_Admin.SetPartySize(1)
            A_Admin.SetPartyMember(1, "Healer", 5, 80)
            TargetUnit("player")

            SimPartyMemberFrame1 = CreateFrame("Button", "SimPartyMemberFrame1", UIParent)
            SimPartyMemberFrame1:SetSize(120, 53)
            SimPartyMemberFrame1:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 22, -147)
            SimPartyMemberFrame1:RegisterForClicks("AnyUp")
            SimPartyMemberFrame1:SetScript("OnClick", function()
                TargetUnit("party1")
            end)

            SimPartyMemberOverlay = CreateFrame("Frame", "SimPartyMemberOverlay", SimPartyMemberFrame1)
            SimPartyMemberOverlay:SetAllPoints(SimPartyMemberFrame1)

            SimPartyMemberName = SimPartyMemberFrame1:CreateFontString(nil, "ARTWORK")
            SimPartyMemberName:SetPoint("TOPLEFT", SimPartyMemberFrame1, "TOPLEFT", 46, -6)
            SimPartyMemberName:SetText("Healer")
            "#,
        )
        .expect("party member mouse target setup should succeed");
    }

    rebuild_hittable_cache(&app);
    let click_pos = Point::new(82.0, 173.5);
    app.handle_mouse_down(click_pos);
    app.handle_mouse_up(click_pos);

    let target_name: String = app
        .env
        .borrow()
        .eval("return UnitName('target')")
        .expect("target name should be readable after party click");
    assert_eq!(
        target_name, "Healer",
        "clicking a party-member frame through the GUI mouse path should target party1"
    );
}

#[test]
fn mouse_enabled_tooltip_without_click_handler_does_not_block_party_click() {
    let mut app = build_test_app(ScreenKind::Game);

    {
        let env = app.env.borrow();
        env.exec(
            r#"
            A_Admin.SetPartySize(1)
            A_Admin.SetPartyMember(1, "Healer", 5, 80)
            TargetUnit("player")

            SimPartyMemberFrame1 = CreateFrame("Button", "SimPartyMemberFrame1", UIParent)
            SimPartyMemberFrame1:SetSize(120, 53)
            SimPartyMemberFrame1:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 22, -147)
            SimPartyMemberFrame1:EnableMouse(true)
            SimPartyMemberFrame1:RegisterForClicks("AnyUp")
            SimPartyMemberFrame1:SetScript("OnClick", function()
                TargetUnit("party1")
            end)

            SimPartyBuffTooltip = CreateFrame("Frame", "SimPartyBuffTooltip", UIParent)
            SimPartyBuffTooltip:SetFrameStrata("TOOLTIP")
            SimPartyBuffTooltip:SetSize(32, 32)
            SimPartyBuffTooltip:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 69, -172)
            SimPartyBuffTooltip:EnableMouse(true)
            "#,
        )
        .expect("party member tooltip setup should succeed");
    }

    rebuild_hittable_cache(&app);
    let click_pos = Point::new(82.0, 173.5);
    app.handle_mouse_move(click_pos);
    app.handle_mouse_down(click_pos);
    app.handle_mouse_up(click_pos);

    let target_name: String = app
        .env
        .borrow()
        .eval("return UnitName('target')")
        .expect("target name should be readable after tooltip-overlapped party click");
    assert_eq!(
        target_name, "Healer",
        "mouse-enabled tooltip frames without click handlers should not swallow party clicks"
    );
}
