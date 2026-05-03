//! BAG_UPDATE_DELAYED keystone refresh for `Blizzard_ArchaeologyUI`.

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
fn archaeology_bag_update_delayed_refreshes_keystone_socket_icons() {
    let ui_dir = blizzard_ui_dir();
    let env = new_blizzard_addon_env(&ui_dir);

    load_panel_addons(&env);
    seed_artifact_with_two_keystone_sockets(&env);
    load_blizzard_addon_closure_into_env(&env, &ui_dir, &[ROOT], &[]);
    env.apply_post_load_workarounds();
    settle_headless_startup(&env);

    let before_event = show_artifact_and_probe_keystones(&env);
    assert_keystone_icons_match_initial_socket_state(before_event);

    mutate_socket_state_for_bag_update(&env);
    let after_event = fire_bag_update_and_probe_keystones(&env);
    assert_keystone_icons_match_mutated_socket_state(after_event);
}

type KeystoneIconProbe = (bool, bool, bool, bool);

fn seed_artifact_with_two_keystone_sockets(env: &WowLuaEnv) {
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
        name: "Clockwork Gnome".to_string(),
        description: "A tiny mechanical assistant.".to_string(),
        rarity: 0,
        icon: 134419,
        spell_description: "Clicks and whirs.".to_string(),
        num_sockets: 2,
        sockets: vec![false, true],
        base_progress: 20,
        adjust_progress: 12,
        total_cost: 35,
        can_solve: false,
        ..SelectedArtifact::default()
    });
}

fn show_artifact_and_probe_keystones(env: &WowLuaEnv) -> KeystoneIconProbe {
    env.eval(
        r#"
        ArchaeologyFrame:Show()
        ArchaeologyFrame_ShowArtifact(1)

        return ArchaeologyFrame.artifactPage.solveFrame.keystone1.icon:IsShown(),
               ArchaeologyFrame.artifactPage.solveFrame.keystone2.icon:IsShown(),
               ArchaeologyFrame.artifactPage.solveFrame.keystone1:IsShown(),
               ArchaeologyFrame.artifactPage.solveFrame.keystone2:IsShown()
        "#,
    )
    .expect("initial archaeology keystone probe must run cleanly")
}

fn mutate_socket_state_for_bag_update(env: &WowLuaEnv) {
    let state = env.state();
    let mut sim = state.borrow_mut();
    let selected = sim
        .archaeology
        .selected
        .as_mut()
        .expect("test setup must keep a selected archaeology artifact");
    selected.sockets = vec![true, false];
    selected.adjust_progress = 12;
}

fn fire_bag_update_and_probe_keystones(env: &WowLuaEnv) -> KeystoneIconProbe {
    env.eval(
        r#"
        FireEvent("BAG_UPDATE_DELAYED")

        return ArchaeologyFrame.artifactPage.solveFrame.keystone1.icon:IsShown(),
               ArchaeologyFrame.artifactPage.solveFrame.keystone2.icon:IsShown(),
               ArchaeologyFrame.artifactPage.solveFrame.keystone1:IsShown(),
               ArchaeologyFrame.artifactPage.solveFrame.keystone2:IsShown()
        "#,
    )
    .expect("BAG_UPDATE_DELAYED archaeology keystone probe must run cleanly")
}

fn assert_keystone_icons_match_initial_socket_state(probe: KeystoneIconProbe) {
    let (first_icon_shown, second_icon_shown, first_slot_shown, second_slot_shown) = probe;

    assert!(
        first_slot_shown,
        "first keystone slot must be visible for a two-socket artifact"
    );
    assert!(
        second_slot_shown,
        "second keystone slot must be visible for a two-socket artifact"
    );
    assert!(
        !first_icon_shown,
        "initial first socket is empty, so keystone1 icon must be hidden"
    );
    assert!(
        second_icon_shown,
        "initial second socket is filled, so keystone2 icon must be shown"
    );
}

fn assert_keystone_icons_match_mutated_socket_state(probe: KeystoneIconProbe) {
    let (first_icon_shown, second_icon_shown, first_slot_shown, second_slot_shown) = probe;

    assert!(
        first_slot_shown,
        "first keystone slot must stay visible after BAG_UPDATE_DELAYED"
    );
    assert!(
        second_slot_shown,
        "second keystone slot must stay visible after BAG_UPDATE_DELAYED"
    );
    assert!(
        first_icon_shown,
        "BAG_UPDATE_DELAYED must re-read socket 1 and show the newly-filled keystone icon"
    );
    assert!(
        !second_icon_shown,
        "BAG_UPDATE_DELAYED must re-read socket 2 and hide the newly-emptied keystone icon"
    );
}
