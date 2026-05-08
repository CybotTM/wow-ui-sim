//! Summary-page mouse-wheel pagination for `Blizzard_ArchaeologyUI`.

use crate::common::blizzard_addon_harness::{
    load_blizzard_addon_closure_into_env, new_blizzard_addon_env,
};
use crate::common::panel_fixtures::{blizzard_ui_dir, load_panel_addons};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::{ArchaeologyArtifact, ArchaeologyRace};
use wow_ui_sim::startup::settle_headless_startup;

const ROOT: &str = "Blizzard_ArchaeologyUI";
const SEEDED_RACE_COUNT: usize = 24;

#[test]
fn archaeology_summary_page_mouse_wheel_paginates_between_full_pages() {
    let ui_dir = blizzard_ui_dir();
    let env = new_blizzard_addon_env(&ui_dir);

    load_panel_addons(&env);
    seed_two_full_summary_pages(&env);
    load_blizzard_addon_closure_into_env(&env, &ui_dir, &[ROOT], &[]);
    env.apply_post_load_workarounds();
    settle_headless_startup(&env);

    let probe = wheel_summary_page_probe(&env);
    assert_summary_page_mouse_wheel_pagination(probe);
}

struct SummaryWheelProbe {
    initial_page: i32,
    after_first_down: i32,
    after_second_down: i32,
    after_first_up: i32,
    after_second_up: i32,
    first_page_first_row: String,
    second_page_first_row: String,
    returned_first_row: String,
}

type SummaryWheelTuple = (i32, i32, i32, i32, i32, String, String, String);

fn seed_two_full_summary_pages(env: &WowLuaEnv) {
    let state = env.state();
    let mut sim = state.borrow_mut();
    sim.archaeology.races = (1..=SEEDED_RACE_COUNT)
        .map(|index| ArchaeologyRace {
            name: format!("Race {index:02}"),
            texture: 460983,
            race_item_id: 63127,
            currency_amount: index as i32,
            project_amount: 35,
            artifacts: vec![ArchaeologyArtifact::default()],
        })
        .collect();
}

fn wheel_summary_page_probe(env: &WowLuaEnv) -> SummaryWheelProbe {
    let tuple = eval_summary_wheel_probe(env);
    SummaryWheelProbe::from(tuple)
}

fn eval_summary_wheel_probe(env: &WowLuaEnv) -> SummaryWheelTuple {
    env.eval(
        r#"
        local function wheel(delta)
            local script = ArchaeologyFrame:GetScript("OnMouseWheel")
            script(ArchaeologyFrame, delta)
        end

        ArchaeologyFrame.tab1:Click()
        local initialPage = ArchaeologyFrame.currentFrame.currentPage
        local firstPageFirstRow = ArchaeologyFrame.summaryPage.race1.raceName:GetText()

        wheel(-1)
        local afterFirstDown = ArchaeologyFrame.currentFrame.currentPage
        local secondPageFirstRow = ArchaeologyFrame.summaryPage.race1.raceName:GetText()

        wheel(-1)
        local afterSecondDown = ArchaeologyFrame.currentFrame.currentPage

        wheel(1)
        local afterFirstUp = ArchaeologyFrame.currentFrame.currentPage
        local returnedFirstRow = ArchaeologyFrame.summaryPage.race1.raceName:GetText()

        wheel(1)
        local afterSecondUp = ArchaeologyFrame.currentFrame.currentPage

        return initialPage,
               afterFirstDown,
               afterSecondDown,
               afterFirstUp,
               afterSecondUp,
               firstPageFirstRow,
               secondPageFirstRow,
               returnedFirstRow
        "#,
    )
    .expect("ArchaeologyFrame summary-page mouse-wheel probe must run cleanly")
}

impl From<SummaryWheelTuple> for SummaryWheelProbe {
    fn from(tuple: SummaryWheelTuple) -> Self {
        let (
            initial_page,
            after_first_down,
            after_second_down,
            after_first_up,
            after_second_up,
            first_page_first_row,
            second_page_first_row,
            returned_first_row,
        ) = tuple;

        Self {
            initial_page,
            after_first_down,
            after_second_down,
            after_first_up,
            after_second_up,
            first_page_first_row,
            second_page_first_row,
            returned_first_row,
        }
    }
}

fn assert_summary_page_mouse_wheel_pagination(probe: SummaryWheelProbe) {
    assert_summary_page_wheel_moves_between_pages(&probe);
    assert_summary_page_rows_refresh_after_wheel(&probe);
}

fn assert_summary_page_wheel_moves_between_pages(probe: &SummaryWheelProbe) {
    assert_eq!(
        probe.initial_page, 1,
        "summaryPage must start on page 1 after selecting the summary tab"
    );
    assert_eq!(
        probe.after_first_down, 2,
        "wheel-down on summaryPage must advance to page 2 when nextPageButton is enabled"
    );
    assert_eq!(
        probe.after_second_down, 2,
        "wheel-down at the last full summary page must be a no-op"
    );
    assert_eq!(
        probe.after_first_up, 1,
        "wheel-up on page 2 must return summaryPage to page 1"
    );
    assert_eq!(
        probe.after_second_up, 1,
        "wheel-up on the first summary page must be a no-op"
    );
}

fn assert_summary_page_rows_refresh_after_wheel(probe: &SummaryWheelProbe) {
    assert_eq!(
        probe.first_page_first_row, "Race 01|n1/35",
        "first summary page must render race 1 in row 1"
    );
    assert_eq!(
        probe.second_page_first_row, "Race 13|n13/35",
        "second summary page must render race 13 in row 1"
    );
    assert_eq!(
        probe.returned_first_row, probe.first_page_first_row,
        "returning to page 1 must restore the first page's row contents"
    );
}
