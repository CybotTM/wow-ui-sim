//! Integration tests for the legacy archaeology keystone-socket surface
//! (`ItemAddedToArtifact`, `SocketItemToArtifact`, `RemoveItemFromArtifact`)
//! consumed by `Blizzard_ArchaeologyUI/Blizzard_ArchaeologyUI.lua:357` (per-
//! socket icon-state read) and `:710-715` (`ArchaeologyFrame_KeyStoneClick`,
//! which toggles via this surface).

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::SelectedArtifact;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

fn seed_two_socket_artifact(env: &WowLuaEnv) {
    let state = env.state();
    let mut sim = state.borrow_mut();
    sim.archaeology.keystone_value = 12;
    sim.archaeology.selected = Some(SelectedArtifact {
        race_id: 1,
        artifact_id: None,
        name: "Belt Buckle of Zaldarinnu".to_string(),
        num_sockets: 2,
        sockets: vec![false, false],
        base_progress: 20,
        adjust_progress: 0,
        total_cost: 50,
        ..SelectedArtifact::default()
    });
}

#[test]
fn item_added_to_artifact_returns_false_without_selection() {
    let env = env();
    let result: bool = env.eval("return ItemAddedToArtifact(1)").unwrap();
    assert!(
        !result,
        "no selected artifact = no socket can be filled = false",
    );
}

#[test]
fn item_added_to_artifact_returns_false_for_empty_socket() {
    let env = env();
    seed_two_socket_artifact(&env);
    let result: bool = env.eval("return ItemAddedToArtifact(1)").unwrap();
    assert!(!result);
}

#[test]
fn item_added_to_artifact_reflects_seeded_socket_state() {
    let env = env();
    seed_two_socket_artifact(&env);
    {
        let state = env.state();
        let mut sim = state.borrow_mut();
        sim.archaeology.selected.as_mut().unwrap().sockets = vec![true, false];
    }
    let socket_one: bool = env.eval("return ItemAddedToArtifact(1)").unwrap();
    let socket_two: bool = env.eval("return ItemAddedToArtifact(2)").unwrap();
    assert!(socket_one, "seeded socket 1 must read true");
    assert!(!socket_two, "seeded socket 2 must read false");
}

#[test]
fn item_added_to_artifact_returns_false_for_out_of_range_index() {
    let env = env();
    seed_two_socket_artifact(&env);
    let high: bool = env.eval("return ItemAddedToArtifact(99)").unwrap();
    let zero: bool = env.eval("return ItemAddedToArtifact(0)").unwrap();
    assert!(!high, "indices past num_sockets must read false, not panic");
    assert!(!zero, "1-based: index 0 must read false");
}

#[test]
fn socket_item_to_artifact_fills_leftmost_empty_first() {
    let env = env();
    seed_two_socket_artifact(&env);
    env.exec("SocketItemToArtifact()").unwrap();
    let st = env.state().borrow();
    let sockets = &st.archaeology.selected.as_ref().unwrap().sockets;
    assert_eq!(
        sockets,
        &vec![true, false],
        "first socket call must fill index 0 (leftmost-empty)",
    );
}

#[test]
fn socket_item_to_artifact_advances_to_next_empty_socket() {
    let env = env();
    seed_two_socket_artifact(&env);
    env.exec("SocketItemToArtifact(); SocketItemToArtifact()")
        .unwrap();
    let st = env.state().borrow();
    let sockets = &st.archaeology.selected.as_ref().unwrap().sockets;
    assert_eq!(sockets, &vec![true, true], "both sockets fill in order");
}

#[test]
fn socket_item_to_artifact_increments_adjust_progress_by_keystone_value() {
    let env = env();
    seed_two_socket_artifact(&env);
    env.exec("SocketItemToArtifact()").unwrap();
    let st = env.state().borrow();
    let selected = st.archaeology.selected.as_ref().unwrap();
    assert_eq!(
        selected.adjust_progress, 12,
        "adjust_progress gains exactly keystone_value (12) per slotted keystone",
    );
}

#[test]
fn socket_item_to_artifact_is_noop_when_all_sockets_filled() {
    let env = env();
    seed_two_socket_artifact(&env);
    {
        let state = env.state();
        let mut sim = state.borrow_mut();
        let sel = sim.archaeology.selected.as_mut().unwrap();
        sel.sockets = vec![true, true];
        sel.adjust_progress = 24;
    }
    env.exec("SocketItemToArtifact()").unwrap();
    let st = env.state().borrow();
    let selected = st.archaeology.selected.as_ref().unwrap();
    assert_eq!(selected.sockets, vec![true, true]);
    assert_eq!(
        selected.adjust_progress, 24,
        "no empty socket = no progress change",
    );
}

#[test]
fn socket_item_to_artifact_is_noop_without_selection() {
    let env = env();
    env.exec("SocketItemToArtifact()").unwrap();
    let st = env.state().borrow();
    assert!(st.archaeology.selected.is_none());
}

#[test]
fn socket_item_to_artifact_normalizes_undersized_sockets_vec() {
    let env = env();
    seed_two_socket_artifact(&env);
    {
        let state = env.state();
        let mut sim = state.borrow_mut();
        sim.archaeology.selected.as_mut().unwrap().sockets.clear();
    }
    env.exec("SocketItemToArtifact()").unwrap();
    let st = env.state().borrow();
    let sockets = &st.archaeology.selected.as_ref().unwrap().sockets;
    assert_eq!(
        sockets,
        &vec![true, false],
        "an empty sockets vec is grown to num_sockets before filling",
    );
}

#[test]
fn remove_item_from_artifact_clears_rightmost_set_socket() {
    let env = env();
    seed_two_socket_artifact(&env);
    {
        let state = env.state();
        let mut sim = state.borrow_mut();
        sim.archaeology.selected.as_mut().unwrap().sockets = vec![true, true];
    }
    env.exec("RemoveItemFromArtifact()").unwrap();
    let st = env.state().borrow();
    let sockets = &st.archaeology.selected.as_ref().unwrap().sockets;
    assert_eq!(
        sockets,
        &vec![true, false],
        "removal must empty the rightmost-set index, not the leftmost",
    );
}

#[test]
fn remove_item_from_artifact_decrements_adjust_progress_by_keystone_value() {
    let env = env();
    seed_two_socket_artifact(&env);
    {
        let state = env.state();
        let mut sim = state.borrow_mut();
        let sel = sim.archaeology.selected.as_mut().unwrap();
        sel.sockets = vec![true, true];
        sel.adjust_progress = 24;
    }
    env.exec("RemoveItemFromArtifact()").unwrap();
    let st = env.state().borrow();
    let selected = st.archaeology.selected.as_ref().unwrap();
    assert_eq!(
        selected.adjust_progress, 12,
        "removal subtracts exactly keystone_value, restoring the pre-socket value",
    );
}

#[test]
fn remove_item_from_artifact_is_noop_when_no_socket_filled() {
    let env = env();
    seed_two_socket_artifact(&env);
    env.exec("RemoveItemFromArtifact()").unwrap();
    let st = env.state().borrow();
    let selected = st.archaeology.selected.as_ref().unwrap();
    assert_eq!(selected.sockets, vec![false, false]);
    assert_eq!(
        selected.adjust_progress, 0,
        "with no keystone to remove, adjust_progress must be untouched",
    );
}

#[test]
fn remove_item_from_artifact_is_noop_without_selection() {
    let env = env();
    env.exec("RemoveItemFromArtifact()").unwrap();
    let st = env.state().borrow();
    assert!(st.archaeology.selected.is_none());
}

#[test]
fn keystone_click_round_trip_via_lua() {
    let env = env();
    seed_two_socket_artifact(&env);
    // Mirrors Blizzard_ArchaeologyUI.lua:710-715 ArchaeologyFrame_KeyStoneClick:
    // socket → unsocket via the same surface the addon uses.
    env.exec(
        r#"
        SocketItemToArtifact()
        local first_after_socket = ItemAddedToArtifact(1)
        if not first_after_socket then error("expected socket 1 filled after first call") end
        RemoveItemFromArtifact()
        if ItemAddedToArtifact(1) then error("expected socket 1 emptied after remove") end
    "#,
    )
    .unwrap();
    let st = env.state().borrow();
    let selected = st.archaeology.selected.as_ref().unwrap();
    assert_eq!(selected.sockets, vec![false, false]);
    assert_eq!(
        selected.adjust_progress, 0,
        "round-trip socket+remove leaves adjust_progress unchanged",
    );
}
