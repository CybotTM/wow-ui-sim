//! Active-plus-ready tooltip branch for `ArdenwealdGardeningButtonMixin:OnEnter`.

use wow_ui_sim::lua_api::WowLuaEnv;

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_ArdenwealdGardening";

#[test]
fn onenter_active_and_ready_branch_inserts_spacer_between_lines() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        seed_active_and_ready_garden(env);

        let tooltip = open_active_and_ready_garden_tooltip(env);

        assert_active_and_ready_tooltip(tooltip);
    });
}

type ActiveAndReadyTooltipProbe = (
    bool,
    f64,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
);

fn seed_active_and_ready_garden(env: &WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    state.gardenweald.active = 2;
    state.gardenweald.ready = 1;
    state.gardenweald.remaining_seconds = 30;
}

fn open_active_and_ready_garden_tooltip(env: &WowLuaEnv) -> ActiveAndReadyTooltipProbe {
    env.eval(
        r#"
        local parent = CreateFrame("Frame", "ArdenwealdGardeningActiveReadyTooltipParent", UIParent)
        ArdenwealdGardening.Create(parent)
        local button = ArdenwealdGardeningButtonTemplate

        GameTooltip:ClearLines()
        button:GetScript("OnEnter")(button)

        local headerLine = GameTooltip:GetLeftLine(1)
        local activeLine = GameTooltip:GetLeftLine(2)
        local spacerLine = GameTooltip:GetLeftLine(3)
        local readyLine = GameTooltip:GetLeftLine(4)
        local expectedActive = GARDENWEALD_STATUS_ACTIVE_COUNT:format(2, "< 1 |4minute:minutes;")
        local expectedReady = GARDENWEALD_STATUS_READY_COUNT:format(1)

        return GameTooltip:IsShown(),
               GameTooltip:NumLines(),
               headerLine and headerLine:GetText() or "",
               activeLine and activeLine:GetText() or "",
               spacerLine and spacerLine:GetText() or "",
               readyLine and readyLine:GetText() or "",
               expectedActive,
               expectedReady,
               GARDENWEALD_STATUS_DORMANT,
               GARDENWEALD_STATUS_HEADER
        "#,
    )
    .expect("Ardenweald Gardening active-plus-ready tooltip probe must run cleanly")
}

fn assert_active_and_ready_tooltip(tooltip: ActiveAndReadyTooltipProbe) {
    let tooltip = ActiveAndReadyTooltip::from(tooltip);

    assert_active_and_ready_lines(tooltip.lines());
    assert_no_dormant_line(&tooltip);
}

struct ActiveAndReadyTooltip {
    is_shown: bool,
    line_count: f64,
    header_text: String,
    active_text: String,
    spacer_text: String,
    ready_text: String,
    expected_active_text: String,
    expected_ready_text: String,
    dormant_text: String,
    expected_header_text: String,
}

impl From<ActiveAndReadyTooltipProbe> for ActiveAndReadyTooltip {
    fn from(tooltip: ActiveAndReadyTooltipProbe) -> Self {
        let (
            is_shown,
            line_count,
            header_text,
            active_text,
            spacer_text,
            ready_text,
            expected_active_text,
            expected_ready_text,
            dormant_text,
            expected_header_text,
        ) = tooltip;

        Self {
            is_shown,
            line_count,
            header_text,
            active_text,
            spacer_text,
            ready_text,
            expected_active_text,
            expected_ready_text,
            dormant_text,
            expected_header_text,
        }
    }
}

impl ActiveAndReadyTooltip {
    fn lines(&self) -> ActiveAndReadyLines<'_> {
        ActiveAndReadyLines {
            is_shown: self.is_shown,
            line_count: self.line_count,
            header_text: &self.header_text,
            active_text: &self.active_text,
            spacer_text: &self.spacer_text,
            ready_text: &self.ready_text,
            expected_active_text: &self.expected_active_text,
            expected_ready_text: &self.expected_ready_text,
            expected_header_text: &self.expected_header_text,
        }
    }
}

fn assert_no_dormant_line(tooltip: &ActiveAndReadyTooltip) {
    assert!(
        tooltip.ready_text != tooltip.dormant_text,
        "active-plus-ready branch must not emit the dormant line"
    );
}

struct ActiveAndReadyLines<'a> {
    is_shown: bool,
    line_count: f64,
    header_text: &'a str,
    active_text: &'a str,
    spacer_text: &'a str,
    ready_text: &'a str,
    expected_active_text: &'a str,
    expected_ready_text: &'a str,
    expected_header_text: &'a str,
}

fn assert_active_and_ready_lines(lines: ActiveAndReadyLines<'_>) {
    assert!(lines.is_shown, "OnEnter must show GameTooltip");
    assert_eq!(
        lines.line_count, 4.0,
        "active-plus-ready branch must emit header, active, spacer, and ready lines"
    );
    assert_eq!(
        lines.header_text, lines.expected_header_text,
        "first tooltip line must be GARDENWEALD_STATUS_HEADER"
    );
    assert_eq!(
        lines.active_text, lines.expected_active_text,
        "second tooltip line must use GARDENWEALD_STATUS_ACTIVE_COUNT"
    );
    assert_eq!(
        lines.spacer_text, " ",
        "third tooltip line must be a blank spacer"
    );
    assert_eq!(
        lines.ready_text, lines.expected_ready_text,
        "fourth tooltip line must use GARDENWEALD_STATUS_READY_COUNT"
    );
}
