//! Active-artifact page rendering for `Blizzard_ArchaeologyUI`.

use crate::common::blizzard_addon_harness::{
    load_blizzard_addon_closure_into_env, new_blizzard_addon_env,
};
use crate::common::panel_fixtures::{blizzard_ui_dir, load_panel_addons};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::{ArchaeologyArtifact, ArchaeologyRace, SelectedArtifact};
use wow_ui_sim::startup::settle_headless_startup;

const ROOT: &str = "Blizzard_ArchaeologyUI";
const RACE_ID: i32 = 1;
const ARTIFACT_NAME: &str = "Puzzle Box of Yogg-Saron";
const ARTIFACT_DESCRIPTION: &str = "A box covered in maddening titan glyphs.";
const ARTIFACT_SPELL_DESCRIPTION: &str = "Whispers a terrible truth.";

#[test]
fn archaeology_artifact_page_renders_selected_artifact_and_solve_state() {
    let ui_dir = blizzard_ui_dir();
    let env = new_blizzard_addon_env(&ui_dir);

    load_panel_addons(&env);
    seed_selected_artifact(&env, true);
    load_blizzard_addon_closure_into_env(&env, &ui_dir, &[ROOT], &[]);
    env.apply_post_load_workarounds();
    settle_headless_startup(&env);

    let solvable_page = show_artifact_and_probe(&env);
    assert_artifact_page_matches_selected(&solvable_page, true);

    set_can_solve_artifact(&env, false);
    let unsolvable_page = show_artifact_and_probe(&env);
    assert_artifact_page_matches_selected(&unsolvable_page, false);
}

type ArtifactPageProbe = (bool, String, String, String, bool, bool);

fn seed_selected_artifact(env: &WowLuaEnv, can_solve: bool) {
    let state = env.state();
    let mut sim = state.borrow_mut();
    sim.archaeology.races = vec![ArchaeologyRace {
        name: "Old God".to_string(),
        texture: 460983,
        race_item_id: 0,
        currency_amount: 40,
        project_amount: 35,
        artifacts: vec![ArchaeologyArtifact::default()],
    }];
    sim.archaeology.selected = Some(SelectedArtifact {
        race_id: RACE_ID,
        artifact_id: None,
        name: ARTIFACT_NAME.to_string(),
        description: ARTIFACT_DESCRIPTION.to_string(),
        rarity: 0,
        icon: 134400,
        spell_description: ARTIFACT_SPELL_DESCRIPTION.to_string(),
        num_sockets: 0,
        base_progress: 35,
        adjust_progress: 5,
        total_cost: 35,
        can_solve,
        ..SelectedArtifact::default()
    });
}

fn set_can_solve_artifact(env: &WowLuaEnv, can_solve: bool) {
    let state = env.state();
    let mut sim = state.borrow_mut();
    let selected = sim
        .archaeology
        .selected
        .as_mut()
        .expect("test setup must keep a selected artifact");
    selected.can_solve = can_solve;
}

fn show_artifact_and_probe(env: &WowLuaEnv) -> ArtifactPageProbe {
    env.eval(
        r#"
        ArchaeologyFrame_ShowArtifact(1)

        local page = ArchaeologyFrame.artifactPage
        local _, _, _, _, spellDescription = GetSelectedArtifactInfo()
        return ArchaeologyFrame.currentFrame == page,
               page.artifactName:GetText(),
               page.historyScroll.child.text:GetText(),
               spellDescription,
               page.solveFrame.solveButton:IsEnabled(),
               CanSolveArtifact()
        "#,
    )
    .expect("ArchaeologyFrame artifact-page probe must run cleanly")
}

fn assert_artifact_page_matches_selected(probe: &ArtifactPageProbe, expected_can_solve: bool) {
    let (
        current_frame_is_artifact_page,
        artifact_name,
        artifact_description,
        spell_description,
        solve_button_enabled,
        can_solve,
    ) = probe;

    assert!(
        *current_frame_is_artifact_page,
        "`ArchaeologyFrame_ShowArtifact` must switch `currentFrame` to `artifactPage`"
    );
    assert_eq!(
        artifact_name, ARTIFACT_NAME,
        "`artifactPage.artifactName` must render the selected artifact name"
    );
    assert_eq!(
        artifact_description, ARTIFACT_DESCRIPTION,
        "`artifactPage.historyScroll.child.text` must render the selected artifact description"
    );
    assert_eq!(
        spell_description, ARTIFACT_SPELL_DESCRIPTION,
        "`GetSelectedArtifactInfo` must expose the selected artifact spell description"
    );
    assert_eq!(
        *can_solve, expected_can_solve,
        "`CanSolveArtifact()` must reflect the seeded selected artifact"
    );
    assert_eq!(
        solve_button_enabled, can_solve,
        "`solveButton` enabled state must match `CanSolveArtifact()`"
    );
}
