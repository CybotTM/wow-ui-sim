//! SOLVE button routing for `Blizzard_ArchaeologyUI`.

use crate::common::blizzard_addon_harness::{
    load_blizzard_addon_closure_into_env, new_blizzard_addon_env,
};
use crate::common::panel_fixtures::{blizzard_ui_dir, load_panel_addons};
use wow_ui_sim::event::EventArg;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::{ArchaeologyArtifact, ArchaeologyRace, SelectedArtifact};
use wow_ui_sim::startup::settle_headless_startup;

const ROOT: &str = "Blizzard_ArchaeologyUI";
const ARTIFACT_NAME: &str = "Belt Buckle of Zaldarinnu";

#[test]
fn archaeology_solve_button_click_solves_selected_artifact() {
    let ui_dir = blizzard_ui_dir();
    let env = new_blizzard_addon_env(&ui_dir);

    load_panel_addons(&env);
    seed_solvable_artifact(&env);
    load_blizzard_addon_closure_into_env(&env, &ui_dir, &[ROOT], &[]);
    env.apply_post_load_workarounds();
    settle_headless_startup(&env);

    click_solve_button(&env);

    let state = env.state();
    let sim = state.borrow();
    let selected = sim
        .archaeology
        .selected
        .as_ref()
        .expect("solving must keep the selected artifact slot");
    assert_eq!(
        selected.base_progress, 0,
        "`SolveArtifact` must clear base progress after the SOLVE button click"
    );
    assert_eq!(
        selected.adjust_progress, 0,
        "`SolveArtifact` must clear adjusted progress after the SOLVE button click"
    );
    assert_completion_event_was_queued(&sim.events);
}

fn seed_solvable_artifact(env: &WowLuaEnv) {
    let state = env.state();
    let mut sim = state.borrow_mut();
    sim.archaeology.races = vec![ArchaeologyRace {
        name: "Dwarf".to_string(),
        texture: 460983,
        race_item_id: 0,
        currency_amount: 40,
        project_amount: 35,
        artifacts: vec![ArchaeologyArtifact::default()],
    }];
    sim.archaeology.selected = Some(SelectedArtifact {
        race_id: 1,
        artifact_id: None,
        name: ARTIFACT_NAME.to_string(),
        description: "An ornate belt buckle.".to_string(),
        rarity: 0,
        icon: 134419,
        spell_description: "Solving rewards 50g.".to_string(),
        num_sockets: 0,
        base_progress: 30,
        adjust_progress: 5,
        total_cost: 35,
        can_solve: true,
        ..SelectedArtifact::default()
    });
}

fn click_solve_button(env: &WowLuaEnv) {
    env.exec(
        r#"
        ArchaeologyFrame_ShowArtifact(1)
        assert(ArchaeologyFrame.artifactPage.solveFrame.solveButton:IsEnabled())
        ArchaeologyFrame.artifactPage.solveFrame.solveButton:Click()
        "#,
    )
    .expect("artifact-page SOLVE button click must run cleanly");
}

fn assert_completion_event_was_queued(events: &wow_ui_sim::event::EventQueue) {
    let event = events
        .pending()
        .iter()
        .find(|event| event.name == "RESEARCH_ARTIFACT_COMPLETE")
        .expect("SOLVE button click must queue RESEARCH_ARTIFACT_COMPLETE");
    assert!(
        matches!(&event.args[0], EventArg::String(name) if name == ARTIFACT_NAME),
        "RESEARCH_ARTIFACT_COMPLETE must carry the solved artifact name"
    );
}
