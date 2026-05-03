//! SecondsFormatter output used by `ArdenwealdGardeningButtonMixin:OnEnter`.

use wow_ui_sim::lua_api::WowLuaEnv;

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_ArdenwealdGardening";

#[test]
fn seconds_formatter_formats_active_garden_durations() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        assert_active_duration_format(env, 0, "0 |4minute:minutes;");
        assert_active_duration_format(env, 60, "1 |4minute:minutes;");
        assert_active_duration_format(env, 3_600, "1 |4hour:hours;");
        assert_active_duration_format(env, 86_400, "1 |4day:days;");
    });
}

type DurationProbe = (f64, String, String);

fn assert_active_duration_format(env: &WowLuaEnv, remaining_seconds: i64, expected_time: &str) {
    seed_active_garden_duration(env, remaining_seconds);

    let (line_count, active_text, expected_active_text) =
        open_active_duration_tooltip(env, expected_time);

    assert_eq!(line_count, 2.0, "active duration probe must emit two lines");
    assert_eq!(
        active_text, expected_active_text,
        "`ArdenwealdGardeningSecondsFormatter` output must feed the active-count line"
    );
}

fn seed_active_garden_duration(env: &WowLuaEnv, remaining_seconds: i64) {
    let mut state = env.state().borrow_mut();
    state.gardenweald.active = 1;
    state.gardenweald.ready = 0;
    state.gardenweald.remaining_seconds = remaining_seconds;
}

fn open_active_duration_tooltip(env: &WowLuaEnv, expected_time: &str) -> DurationProbe {
    let lua = format!(
        r#"
        local parent = CreateFrame("Frame", nil, UIParent)
        ArdenwealdGardening.Create(parent)
        local button = ArdenwealdGardeningButtonTemplate

        GameTooltip:ClearLines()
        button:GetScript("OnEnter")(button)

        local activeLine = GameTooltip:GetLeftLine(2)
        local expectedActive = GARDENWEALD_STATUS_ACTIVE_COUNT:format(1, {expected_time:?})

        return GameTooltip:NumLines(),
               activeLine and activeLine:GetText() or "",
               expectedActive
        "#
    );

    env.eval(&lua)
        .expect("Ardenweald Gardening duration tooltip probe must run cleanly")
}
