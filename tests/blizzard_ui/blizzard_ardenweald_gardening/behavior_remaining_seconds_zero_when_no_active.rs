//! `remainingSeconds` is ignored when no Ardenweald garden seed is active.

use wow_ui_sim::lua_api::WowLuaEnv;

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_ArdenwealdGardening";

#[test]
fn onenter_does_not_format_remaining_seconds_without_active_seed() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        seed_ready_garden_with_large_remaining_seconds(env);

        let tooltip = open_ready_tooltip_with_large_remaining_seconds(env);

        assert_remaining_seconds_was_not_formatted(tooltip);
    });
}

type NoActiveDurationProbe = (f64, bool, bool, String);

fn seed_ready_garden_with_large_remaining_seconds(env: &WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    state.gardenweald.active = 0;
    state.gardenweald.ready = 1;
    state.gardenweald.remaining_seconds = 999_999;
}

fn open_ready_tooltip_with_large_remaining_seconds(env: &WowLuaEnv) -> NoActiveDurationProbe {
    env.eval(
        r#"
        local parent = CreateFrame("Frame", nil, UIParent)
        ArdenwealdGardening.Create(parent)
        local button = ArdenwealdGardeningButtonTemplate

        GameTooltip:ClearLines()
        button:GetScript("OnEnter")(button)

        local expectedReady = GARDENWEALD_STATUS_READY_COUNT:format(1)
        local formattedDuration = "12 |4day:days;"
        local sawFormattedDuration = false

        for lineIndex = 1, GameTooltip:NumLines() do
            local line = GameTooltip:GetLeftLine(lineIndex)
            local text = line and line:GetText() or ""
            if string.find(text, formattedDuration, 1, true) then
                sawFormattedDuration = true
            end
        end

        local secondLine = GameTooltip:GetLeftLine(2)
        return GameTooltip:NumLines(),
               secondLine and secondLine:GetText() == expectedReady,
               sawFormattedDuration,
               formattedDuration
        "#,
    )
    .expect("Ardenweald Gardening no-active duration probe must run cleanly")
}

fn assert_remaining_seconds_was_not_formatted(probe: NoActiveDurationProbe) {
    let (line_count, ready_line_matches, saw_formatted_duration, formatted_duration) = probe;

    assert_eq!(
        line_count, 2.0,
        "ready-without-active branch must emit only header and ready lines"
    );
    assert!(
        ready_line_matches,
        "ready branch must still emit GARDENWEALD_STATUS_READY_COUNT"
    );
    assert!(
        !saw_formatted_duration,
        "active formatter must not run or leak `{formatted_duration}` when active == 0"
    );
}
