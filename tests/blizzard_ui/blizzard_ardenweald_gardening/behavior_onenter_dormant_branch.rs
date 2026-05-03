//! Dormant tooltip branch for `ArdenwealdGardeningButtonMixin:OnEnter`.

use wow_ui_sim::lua_api::WowLuaEnv;

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_ArdenwealdGardening";

#[test]
fn onenter_dormant_branch_shows_dormant_line_only() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        seed_dormant_garden(env);

        let tooltip = open_dormant_garden_tooltip(env);

        assert_dormant_tooltip(tooltip);
    });
}

type DormantTooltipProbe = (bool, f64, String, String, String, String, String);

fn seed_dormant_garden(env: &WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    state.gardenweald.active = 0;
    state.gardenweald.ready = 0;
    state.gardenweald.remaining_seconds = 0;
}

fn open_dormant_garden_tooltip(env: &WowLuaEnv) -> DormantTooltipProbe {
    env.eval(
        r#"
        local parent = CreateFrame("Frame", "ArdenwealdGardeningDormantTooltipParent", UIParent)
        ArdenwealdGardening.Create(parent)
        local button = ArdenwealdGardeningButtonTemplate

        GameTooltip:ClearLines()
        button:GetScript("OnEnter")(button)

        local firstLine = GameTooltip:GetLeftLine(1)
        local secondLine = GameTooltip:GetLeftLine(2)
        local activeLine = GARDENWEALD_STATUS_ACTIVE_COUNT:format(1, "1 |4minute:minutes;")
        local readyLine = GARDENWEALD_STATUS_READY_COUNT:format(1)

        return GameTooltip:IsShown(),
               GameTooltip:NumLines(),
               firstLine and firstLine:GetText() or "",
               secondLine and secondLine:GetText() or "",
               GARDENWEALD_STATUS_DORMANT,
               activeLine,
               readyLine
        "#,
    )
    .expect("Ardenweald Gardening dormant tooltip probe must run cleanly")
}

fn assert_dormant_tooltip(tooltip: DormantTooltipProbe) {
    let (
        is_shown,
        line_count,
        header_text,
        dormant_text,
        expected_dormant_text,
        active_text,
        ready_text,
    ) = tooltip;

    assert!(is_shown, "OnEnter must show GameTooltip");
    assert_eq!(line_count, 2.0, "dormant branch must emit two lines");
    assert_eq!(
        header_text, "Queen's Conservatory",
        "first tooltip line must be GARDENWEALD_STATUS_HEADER"
    );
    assert_eq!(
        dormant_text, expected_dormant_text,
        "second tooltip line must use GARDENWEALD_STATUS_DORMANT"
    );
    assert!(
        dormant_text != active_text,
        "dormant branch must not emit an active-count line"
    );
    assert!(
        dormant_text != ready_text,
        "dormant branch must not emit a ready-count line"
    );
}
