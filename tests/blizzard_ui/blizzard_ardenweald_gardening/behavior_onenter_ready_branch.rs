//! Ready-count tooltip branch for `ArdenwealdGardeningButtonMixin:OnEnter`.

use wow_ui_sim::lua_api::WowLuaEnv;

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_ArdenwealdGardening";

#[test]
fn onenter_ready_branch_shows_ready_count_without_spacer() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        seed_ready_garden(env);

        let tooltip = open_ready_garden_tooltip(env);

        assert_ready_tooltip(tooltip);
    });
}

type ReadyTooltipProbe = (bool, f64, String, String, String, String, String, String);

fn seed_ready_garden(env: &WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    state.gardenweald.active = 0;
    state.gardenweald.ready = 4;
    state.gardenweald.remaining_seconds = 0;
}

fn open_ready_garden_tooltip(env: &WowLuaEnv) -> ReadyTooltipProbe {
    env.eval(
        r#"
        local parent = CreateFrame("Frame", "ArdenwealdGardeningReadyTooltipParent", UIParent)
        ArdenwealdGardening.Create(parent)
        local button = ArdenwealdGardeningButtonTemplate

        GameTooltip:ClearLines()
        button:GetScript("OnEnter")(button)

        local firstLine = GameTooltip:GetLeftLine(1)
        local secondLine = GameTooltip:GetLeftLine(2)
        local activeLine = GARDENWEALD_STATUS_ACTIVE_COUNT:format(1, "1 |4minute:minutes;")
        local expectedReady = GARDENWEALD_STATUS_READY_COUNT:format(4)

        return GameTooltip:IsShown(),
               GameTooltip:NumLines(),
               firstLine and firstLine:GetText() or "",
               secondLine and secondLine:GetText() or "",
               expectedReady,
               activeLine,
               GARDENWEALD_STATUS_DORMANT,
               GARDENWEALD_STATUS_HEADER
        "#,
    )
    .expect("Ardenweald Gardening ready tooltip probe must run cleanly")
}

fn assert_ready_tooltip(tooltip: ReadyTooltipProbe) {
    let (
        is_shown,
        line_count,
        header_text,
        ready_text,
        expected_ready_text,
        active_text,
        dormant_text,
        expected_header_text,
    ) = tooltip;

    assert!(is_shown, "OnEnter must show GameTooltip");
    assert_eq!(
        line_count, 2.0,
        "ready-only branch must emit header and ready line without a blank spacer"
    );
    assert_eq!(
        header_text, expected_header_text,
        "first tooltip line must be GARDENWEALD_STATUS_HEADER"
    );
    assert_eq!(
        ready_text, expected_ready_text,
        "second tooltip line must use GARDENWEALD_STATUS_READY_COUNT"
    );
    assert!(
        ready_text != active_text,
        "ready-only branch must not emit an active-count line"
    );
    assert!(
        ready_text != dormant_text,
        "ready-only branch must not emit the dormant line"
    );
}
