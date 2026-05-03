//! Completed-page pagination for `Blizzard_ArchaeologyUI`.

use crate::common::blizzard_addon_harness::{
    load_blizzard_addon_closure_into_env, new_blizzard_addon_env,
};
use crate::common::panel_fixtures::{blizzard_ui_dir, load_panel_addons};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::{ArchaeologyArtifact, ArchaeologyRace};
use wow_ui_sim::startup::settle_headless_startup;

const ROOT: &str = "Blizzard_ArchaeologyUI";
const COMPLETED_ROWS_PER_PAGE: usize = 12;
const ARTIFACTS_PER_RACE: usize = 13;

#[test]
fn archaeology_completed_page_paginates_completed_artifacts() {
    let ui_dir = blizzard_ui_dir();
    let env = new_blizzard_addon_env(&ui_dir);

    load_panel_addons(&env);
    seed_completed_artifact_history(&env);
    load_blizzard_addon_closure_into_env(&env, &ui_dir, &[ROOT], &[]);
    env.apply_post_load_workarounds();
    settle_headless_startup(&env);

    let pages = collect_completed_page_probe(&env);

    assert_completed_page_history_is_available(&pages);
    assert_page_entries(&pages.page_one_names, &expected_page_one_names());
    assert_page_entries(&pages.page_two_names, &expected_page_two_names());
    assert_eq!(
        pages.page_two_number, 2,
        "clicking the completed-page next button must advance `currentPage` to 2"
    );
}

struct CompletedPageProbe {
    history_available: bool,
    page_one_number: i32,
    page_one_names: String,
    page_two_number: i32,
    page_two_names: String,
}

fn seed_completed_artifact_history(env: &WowLuaEnv) {
    let state = env.state();
    let mut sim = state.borrow_mut();
    sim.archaeology.history_available = true;
    sim.archaeology.races = vec![
        completed_race("Dwarf", 460983, "Dwarf Archive"),
        completed_race("Troll", 460982, "Troll Archive"),
    ];
}

fn completed_race(name: &str, texture: u32, artifact_prefix: &str) -> ArchaeologyRace {
    ArchaeologyRace {
        name: name.to_string(),
        texture,
        race_item_id: 63127,
        currency_amount: 0,
        project_amount: 35,
        artifacts: completed_artifacts(artifact_prefix),
    }
}

fn completed_artifacts(prefix: &str) -> Vec<ArchaeologyArtifact> {
    (1..=ARTIFACTS_PER_RACE)
        .map(|index| ArchaeologyArtifact {
            name: format!("{prefix} {index:02}"),
            description: format!("{prefix} {index:02} description"),
            rarity: 1,
            icon: 134419,
            spell_description: format!("{prefix} {index:02} spell text"),
            spell_id: 88910,
            first_completion_time: 1_700_000_000 + index as i64,
            completion_count: 1,
        })
        .collect()
}

fn collect_completed_page_probe(env: &WowLuaEnv) -> CompletedPageProbe {
    let (history_available, page_one_number, page_one_names, page_two_number, page_two_names) = env
        .eval(
            r#"
            local function visibleArtifactNames()
                local names = {}
                for index = 1, ARCHAEOLOGY_MAX_COMPLETED_SHOWN do
                    local button = ArchaeologyFrame.completedPage["artifact"..index]
                    if button:IsShown() then
                        table.insert(names, button.artifactName:GetText())
                    end
                end
                return table.concat(names, "\n")
            end

            ArchaeologyFrame.tab2:Click()
            local historyAvailable = IsArtifactCompletionHistoryAvailable()
            local pageOneNumber = ArchaeologyFrame.completedPage.currentPage
            local pageOneNames = visibleArtifactNames()

            ArchaeologyFrame.completedPage.nextPageButton:Click()
            local pageTwoNumber = ArchaeologyFrame.completedPage.currentPage
            local pageTwoNames = visibleArtifactNames()

            return historyAvailable, pageOneNumber, pageOneNames, pageTwoNumber, pageTwoNames
            "#,
        )
        .expect("ArchaeologyFrame completed-page pagination probe must run cleanly");

    CompletedPageProbe {
        history_available,
        page_one_number,
        page_one_names,
        page_two_number,
        page_two_names,
    }
}

fn assert_completed_page_history_is_available(pages: &CompletedPageProbe) {
    assert!(
        pages.history_available,
        "`IsArtifactCompletionHistoryAvailable()` must reflect seeded history state"
    );
    assert_eq!(
        pages.page_one_number, 1,
        "completed page must start on page 1 after selecting the completed tab"
    );
}

fn expected_page_one_names() -> Vec<String> {
    (1..=COMPLETED_ROWS_PER_PAGE)
        .map(|index| format!("Dwarf Archive {index:02}"))
        .collect()
}

fn expected_page_two_names() -> Vec<String> {
    let mut names = vec!["Dwarf Archive 13".to_string()];
    names.extend((1..COMPLETED_ROWS_PER_PAGE).map(|index| format!("Troll Archive {index:02}")));
    names
}

fn assert_page_entries(serialized_names: &str, expected_names: &[String]) {
    let names = serialized_names.lines().collect::<Vec<_>>();
    assert_eq!(
        names.len(),
        COMPLETED_ROWS_PER_PAGE,
        "completed page must show {COMPLETED_ROWS_PER_PAGE} artifact rows"
    );

    for (offset, name) in names.iter().enumerate() {
        assert_eq!(
            *name,
            expected_names[offset],
            "completed-page row {} must render the expected artifact",
            offset + 1
        );
    }
}
