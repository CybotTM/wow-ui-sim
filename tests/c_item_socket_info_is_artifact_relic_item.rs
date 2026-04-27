//! Integration tests for `C_ItemSocketInfo.IsArtifactRelicItem`,
//! the artifact-panel probe used by
//! `Blizzard_ArtifactUI.lua:270-296` when hovering a bag item to
//! decide whether to highlight a relic slot. Driven by
//! `state.artifact_relic_items: HashSet<i32>`.

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn is_artifact_relic_item_returns_false_for_unknown_id() {
    let env = WowLuaEnv::new().expect("env");
    let known: bool = env
        .eval("return C_ItemSocketInfo.IsArtifactRelicItem(123)")
        .unwrap();
    assert!(!known);
}

#[test]
fn is_artifact_relic_item_returns_true_for_state_seeded_id() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().artifact_relic_items.insert(456);
    let known: bool = env
        .eval("return C_ItemSocketInfo.IsArtifactRelicItem(456)")
        .unwrap();
    assert!(known);
}

#[test]
fn is_artifact_relic_item_accepts_string_item_link() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().artifact_relic_items.insert(789);
    let known: bool = env
        .eval("return C_ItemSocketInfo.IsArtifactRelicItem(\"item:789\")")
        .unwrap();
    assert!(known);
}

#[test]
fn is_artifact_relic_item_returns_false_for_non_item_argument() {
    let env = WowLuaEnv::new().expect("env");
    let known: bool = env
        .eval("return C_ItemSocketInfo.IsArtifactRelicItem(nil)")
        .unwrap();
    assert!(!known);
}
