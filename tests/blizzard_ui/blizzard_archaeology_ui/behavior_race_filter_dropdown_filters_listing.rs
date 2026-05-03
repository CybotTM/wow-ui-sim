//! Race-filter dropdown behavior for `Blizzard_ArchaeologyUI`.

use crate::common::blizzard_addon_harness::{
    load_blizzard_addon_closure_into_env, new_blizzard_addon_env,
};
use crate::common::panel_fixtures::{blizzard_ui_dir, load_panel_addons};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::{ArchaeologyArtifact, ArchaeologyRace};
use wow_ui_sim::startup::settle_headless_startup;

const ROOT: &str = "Blizzard_ArchaeologyUI";
const SELECTED_RACE_INDEX: i32 = 3;
const RACE_FILTER_PROBE_LUA: &str = r#"
ArchaeologyFrame.tab2:Click()
ArchaeologyFrame.completedPage.currentPage = 2

local capturedTag
local radios = {}
local rootDescription = {
    SetTag = function(_, tag)
        capturedTag = tag
    end,
    CreateRadio = function(_, text, isSelected, setSelected, value)
        local radio = {
            text = text,
            isSelected = isSelected,
            setSelected = setSelected,
            value = value,
            enabled = true,
            SetEnabled = function(self, enabled)
                self.enabled = enabled
            end,
        }
        table.insert(radios, radio)
        return radio
    end,
}

local generator = ArchaeologyFrame.RaceFilterDropdown.menuGenerator
generator(ArchaeologyFrame.RaceFilterDropdown, rootDescription)

local expectedRadioCount = 1
for raceIndex = 1, GetNumArchaeologyRaces() do
    if GetNumArtifactsByRace(raceIndex) > 0 then
        expectedRadioCount = expectedRadioCount + 1
    end
end

local radioTexts = {}
for _, radio in ipairs(radios) do
    table.insert(radioTexts, radio.text)
end

radios[3].setSelected(radios[3].value)

return capturedTag,
       #radios,
       expectedRadioCount,
       table.concat(radioTexts, "\n"),
       ArchaeologyFrame.currentFrame.raceFilter,
       ArchaeologyFrame.currentFrame.currentPage,
       ArchaeologyFrame.completedPage.currData.raceIndex
"#;

#[test]
fn archaeology_race_filter_dropdown_lists_project_races_and_selects_filter() {
    let ui_dir = blizzard_ui_dir();
    let env = new_blizzard_addon_env(&ui_dir);

    load_panel_addons(&env);
    seed_race_filter_history(&env);
    load_blizzard_addon_closure_into_env(&env, &ui_dir, &[ROOT], &[]);
    env.apply_post_load_workarounds();
    settle_headless_startup(&env);

    let probe = collect_race_filter_probe(&env);

    assert_eq!(probe.menu_tag, "MENU_ARCHAEOLOGY_RACE_FILTER");
    assert_eq!(
        probe.radio_count, probe.expected_radio_count,
        "dropdown must expose ALL plus one radio per archaeology race with projects"
    );
    assert_eq!(probe.radio_texts, "All\nDwarf\nTroll");
    assert_eq!(
        probe.selected_filter, SELECTED_RACE_INDEX,
        "selecting a race radio must update `currentFrame.raceFilter`"
    );
    assert_eq!(
        probe.current_page, 1,
        "selecting a race radio must reset completedPage.currentPage to 1"
    );
    assert_eq!(
        probe.curr_data_race_index, SELECTED_RACE_INDEX,
        "completedPage currData.raceIndex must follow the selected race filter"
    );
}

struct RaceFilterProbe {
    menu_tag: String,
    radio_count: i32,
    expected_radio_count: i32,
    radio_texts: String,
    selected_filter: i32,
    current_page: i32,
    curr_data_race_index: i32,
}

type RaceFilterProbeTuple = (String, i32, i32, String, i32, i32, i32);

fn seed_race_filter_history(env: &WowLuaEnv) {
    let state = env.state();
    let mut sim = state.borrow_mut();
    sim.archaeology.history_available = true;
    sim.archaeology.races = vec![
        race_with_artifacts("Dwarf", 460983, 2),
        race_with_artifacts("Orc", 460984, 0),
        race_with_artifacts("Troll", 460982, 2),
    ];
}

fn race_with_artifacts(name: &str, texture: u32, artifact_count: usize) -> ArchaeologyRace {
    ArchaeologyRace {
        name: name.to_string(),
        texture,
        race_item_id: 63127,
        currency_amount: 0,
        project_amount: 35,
        artifacts: artifacts_for_race(name, artifact_count),
    }
}

fn artifacts_for_race(prefix: &str, artifact_count: usize) -> Vec<ArchaeologyArtifact> {
    (1..=artifact_count)
        .map(|index| ArchaeologyArtifact {
            name: format!("{prefix} Artifact {index}"),
            description: format!("{prefix} Artifact {index} description"),
            rarity: 1,
            icon: 134419,
            spell_description: format!("{prefix} Artifact {index} spell text"),
            spell_id: 88910,
            first_completion_time: 1_700_000_000 + index as i64,
            completion_count: 1,
        })
        .collect()
}

fn collect_race_filter_probe(env: &WowLuaEnv) -> RaceFilterProbe {
    let probe_tuple = env
        .eval(RACE_FILTER_PROBE_LUA)
        .expect("ArchaeologyFrame race-filter dropdown probe must run cleanly");

    race_filter_probe_from_tuple(probe_tuple)
}

fn race_filter_probe_from_tuple(probe: RaceFilterProbeTuple) -> RaceFilterProbe {
    let (
        menu_tag,
        radio_count,
        expected_radio_count,
        radio_texts,
        selected_filter,
        current_page,
        curr_data_race_index,
    ) = probe;

    RaceFilterProbe {
        menu_tag,
        radio_count,
        expected_radio_count,
        radio_texts,
        selected_filter,
        current_page,
        curr_data_race_index,
    }
}
