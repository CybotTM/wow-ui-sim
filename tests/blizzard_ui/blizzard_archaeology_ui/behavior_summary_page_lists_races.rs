//! Summary-page race button population for `Blizzard_ArchaeologyUI`.

use crate::common::blizzard_addon_harness::{
    load_blizzard_addon_closure_into_env, new_blizzard_addon_env,
};
use crate::common::panel_fixtures::{blizzard_ui_dir, load_panel_addons};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::{ArchaeologyArtifact, ArchaeologyRace};
use wow_ui_sim::startup::settle_headless_startup;

const ROOT: &str = "Blizzard_ArchaeologyUI";
const SEEDED_RACE_COUNT: usize = 3;

#[test]
fn archaeology_summary_page_lists_seeded_races_and_hides_empty_slots() {
    let ui_dir = blizzard_ui_dir();
    let env = new_blizzard_addon_env(&ui_dir);

    load_panel_addons(&env);
    seed_archaeology_races(&env);
    load_blizzard_addon_closure_into_env(&env, &ui_dir, &[ROOT], &[]);
    env.apply_post_load_workarounds();
    settle_headless_startup(&env);

    let summary: SummaryProbe = env
        .eval(
            r#"
            local summaryPage = ArchaeologyFrame.summaryPage
            ArchaeologyFrame_UpdateSummary(summaryPage)

            local maxRaces = ARCHAEOLOGY_MAX_RACES
            local visibleRows = {}
            local hiddenRows = {}

            for raceIndex = 1, GetNumArchaeologyRaces() do
                local name, _, _, currencyAmount, projectAmount = GetArchaeologyRaceInfo(raceIndex, false)
                local raceButton = summaryPage["race"..raceIndex]
                local expectedText = name.."|n"..currencyAmount.."/"..projectAmount
                local actualText = raceButton.raceName:GetText() or ""
                table.insert(visibleRows, table.concat({
                    raceIndex,
                    tostring(raceButton:IsShown()),
                    actualText,
                    expectedText,
                }, "\t"))
            end

            for raceIndex = GetNumArchaeologyRaces() + 1, maxRaces do
                local raceButton = summaryPage["race"..raceIndex]
                table.insert(hiddenRows, raceIndex.."\t"..tostring(raceButton:IsShown()))
            end

            return maxRaces, table.concat(visibleRows, "\n"), table.concat(hiddenRows, "\n")
            "#,
        )
        .expect("ArchaeologyFrame summary-page race probe must run cleanly");

    assert_summary_page_lists_seeded_races(summary);
}

type SummaryProbe = (i32, String, String);

fn seed_archaeology_races(env: &WowLuaEnv) {
    let state = env.state();
    let mut sim = state.borrow_mut();
    sim.archaeology.races = vec![
        archaeology_race("Dwarf", 460983, 63127, 12, 35, 2),
        archaeology_race("Troll", 460982, 63128, 7, 45, 1),
        archaeology_race("Night Elf", 442739, 64395, 20, 50, 3),
    ];
}

fn archaeology_race(
    name: &str,
    texture: u32,
    race_item_id: u32,
    currency_amount: i32,
    project_amount: i32,
    artifact_count: usize,
) -> ArchaeologyRace {
    ArchaeologyRace {
        name: name.to_string(),
        texture,
        race_item_id,
        currency_amount,
        project_amount,
        artifacts: vec![ArchaeologyArtifact::default(); artifact_count],
    }
}

fn assert_summary_page_lists_seeded_races(summary: SummaryProbe) {
    let (max_races, visible_rows, hidden_rows) = summary;

    assert_visible_race_rows(&visible_rows);
    assert_hidden_race_rows(max_races, &hidden_rows);
}

fn assert_visible_race_rows(visible_rows: &str) {
    let visible_lines = visible_rows.lines().collect::<Vec<_>>();
    assert_eq!(
        visible_lines.len(),
        SEEDED_RACE_COUNT,
        "first {SEEDED_RACE_COUNT} summary-page race buttons must be represented"
    );

    for line in visible_lines {
        let columns = line.split('\t').collect::<Vec<_>>();
        assert_eq!(
            columns.len(),
            4,
            "visible-row probe must serialize index, visibility, actual text, and expected text"
        );
        assert_eq!(
            columns[1], "true",
            "seeded race button {} must be visible",
            columns[0]
        );
        assert_eq!(
            columns[2], columns[3],
            "seeded race button {} text must match GetArchaeologyRaceInfo-derived text",
            columns[0]
        );
    }
}

fn assert_hidden_race_rows(max_races: i32, hidden_rows: &str) {
    let hidden_lines = hidden_rows.lines().collect::<Vec<_>>();
    let expected_hidden_count = max_races as usize - SEEDED_RACE_COUNT;
    assert_eq!(
        hidden_lines.len(),
        expected_hidden_count,
        "unseeded summary-page race slots must be represented through ARCHAEOLOGY_MAX_RACES"
    );

    for line in hidden_lines {
        let columns = line.split('\t').collect::<Vec<_>>();
        assert_eq!(
            columns.len(),
            2,
            "hidden-row probe must serialize index and visibility"
        );
        assert_eq!(
            columns[1], "false",
            "unseeded race button {} must be hidden",
            columns[0]
        );
    }
}
