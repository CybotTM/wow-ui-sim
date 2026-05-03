//! Keystone socket button routing for `Blizzard_ArchaeologyUI`.

use crate::common::blizzard_addon_harness::{
    load_blizzard_addon_closure_into_env, new_blizzard_addon_env,
};
use crate::common::panel_fixtures::{blizzard_ui_dir, load_panel_addons};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::{ArchaeologyArtifact, ArchaeologyRace, SelectedArtifact};
use wow_ui_sim::startup::settle_headless_startup;

const ROOT: &str = "Blizzard_ArchaeologyUI";
const ROUGH_STONE_ITEM_ID: u32 = 2835;

#[test]
fn archaeology_keystone_click_toggles_first_socket() {
    let ui_dir = blizzard_ui_dir();
    let env = new_blizzard_addon_env(&ui_dir);

    load_panel_addons(&env);
    seed_artifact_with_empty_keystone_socket(&env);
    load_blizzard_addon_closure_into_env(&env, &ui_dir, &[ROOT], &[]);
    env.apply_post_load_workarounds();
    settle_headless_startup(&env);

    let toggles: KeystoneToggleProbe = env
        .eval(
            r#"
            ArchaeologyFrame_ShowArtifact(1)
            local keystone = ArchaeologyFrame.artifactPage.solveFrame.keystone1
            local before = ItemAddedToArtifact(1)
            local shownBeforeClick = keystone:IsShown()

            keystone:Click()
            local afterFirstClick = ItemAddedToArtifact(1)

            keystone:Click()
            local afterSecondClick = ItemAddedToArtifact(1)

            return before, shownBeforeClick, afterFirstClick, afterSecondClick
            "#,
        )
        .expect("ArchaeologyFrame keystone click probe must run cleanly");

    assert_keystone_click_toggled_socket(toggles);
}

type KeystoneToggleProbe = (bool, bool, bool, bool);

fn seed_artifact_with_empty_keystone_socket(env: &WowLuaEnv) {
    let state = env.state();
    let mut sim = state.borrow_mut();
    sim.archaeology.keystone_value = 12;
    sim.archaeology.races = vec![ArchaeologyRace {
        name: "Dwarf".to_string(),
        texture: 460983,
        race_item_id: ROUGH_STONE_ITEM_ID,
        currency_amount: 40,
        project_amount: 35,
        artifacts: vec![ArchaeologyArtifact::default()],
    }];
    sim.archaeology.selected = Some(SelectedArtifact {
        race_id: 1,
        artifact_id: None,
        name: "Belt Buckle of Zaldarinnu".to_string(),
        description: "An ornate belt buckle.".to_string(),
        rarity: 0,
        icon: 134419,
        spell_description: "Solving rewards 50g.".to_string(),
        num_sockets: 1,
        sockets: vec![false],
        base_progress: 20,
        adjust_progress: 0,
        total_cost: 35,
        can_solve: false,
        ..SelectedArtifact::default()
    });
}

fn assert_keystone_click_toggled_socket(toggles: KeystoneToggleProbe) {
    let (before, shown_before_click, after_first_click, after_second_click) = toggles;

    assert!(
        !before,
        "seeded first archaeology socket must start empty before clicking"
    );
    assert!(
        shown_before_click,
        "first keystone button must be visible before the click path is exercised"
    );
    assert!(
        after_first_click,
        "first click must invoke `SocketItemToArtifact` and fill socket 1"
    );
    assert!(
        !after_second_click,
        "second click must invoke `RemoveItemFromArtifact` and empty socket 1"
    );
}
