//! Active-count tooltip branch for `ArdenwealdGardeningButtonMixin:OnEnter`.

use wow_ui_sim::lua_api::WowLuaEnv;

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_ArdenwealdGardening";

#[test]
fn onenter_active_branch_shows_header_and_active_count_only() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        seed_active_garden(env);

        let tooltip = open_active_garden_tooltip(env);

        assert_active_tooltip(tooltip);
    });
}

type ActiveTooltipProbe = (bool, f64, String, String, String, String, String, String);

fn seed_active_garden(env: &WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    state.gardenweald.active = 3;
    state.gardenweald.ready = 0;
    state.gardenweald.remaining_seconds = 600;
}

fn open_active_garden_tooltip(env: &WowLuaEnv) -> ActiveTooltipProbe {
    env.eval(
        r#"
        local parent = CreateFrame("Frame", "ArdenwealdGardeningActiveTooltipParent", UIParent)
        ArdenwealdGardening.Create(parent)
        local button = ArdenwealdGardeningButtonTemplate

        GameTooltip:ClearLines()
        button:GetScript("OnEnter")(button)

        local firstLine = GameTooltip:GetLeftLine(1)
        local secondLine = GameTooltip:GetLeftLine(2)
        local readyLine = GARDENWEALD_STATUS_READY_COUNT:format(1)
        local expectedActive = GARDENWEALD_STATUS_ACTIVE_COUNT:format(3, "10 |4minute:minutes;")

        return GameTooltip:IsShown(),
               GameTooltip:NumLines(),
               firstLine and firstLine:GetText() or "",
               secondLine and secondLine:GetText() or "",
               expectedActive,
               readyLine,
               GARDENWEALD_STATUS_DORMANT,
               GARDENWEALD_STATUS_HEADER
        "#,
    )
    .expect("Ardenweald Gardening active tooltip probe must run cleanly")
}

fn assert_active_tooltip(tooltip: ActiveTooltipProbe) {
    let (
        is_shown,
        line_count,
        header_text,
        active_text,
        expected_active_text,
        ready_text,
        dormant_text,
        expected_header_text,
    ) = tooltip;

    assert!(is_shown, "OnEnter must show GameTooltip");
    assert_eq!(line_count, 2.0, "active-only branch must emit two lines");
    assert_eq!(
        header_text, expected_header_text,
        "first tooltip line must be GARDENWEALD_STATUS_HEADER"
    );
    assert_eq!(
        active_text, expected_active_text,
        "second tooltip line must use GARDENWEALD_STATUS_ACTIVE_COUNT"
    );
    assert!(
        active_text != ready_text,
        "active-only branch must not emit a ready-count line"
    );
    assert!(
        active_text != dormant_text,
        "active-only branch must not emit the dormant line"
    );
}
