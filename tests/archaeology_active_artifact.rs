//! Integration tests for the legacy archaeology active-artifact surface
//! (`GetSelectedArtifactInfo`, `SetSelectedArtifact`, `GetArtifactProgress`,
//! `CanSolveArtifact`, `SolveArtifact`) consumed by
//! `Blizzard_ArchaeologyUI/Blizzard_ArchaeologyUI.lua` lines 323, 326, 335,
//! 599, 605, and the SOLVE button in `Blizzard_ArchaeologyUI.xml:818`.

use wow_ui_sim::event::EventArg;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::SelectedArtifact;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

fn seed_selected(env: &WowLuaEnv, selected: SelectedArtifact) {
    let state = env.state();
    let mut sim = state.borrow_mut();
    sim.archaeology.selected = Some(selected);
}

fn solve_ready_selected() -> SelectedArtifact {
    SelectedArtifact {
        race_id: 1,
        artifact_id: None,
        name: "Belt Buckle of Zaldarinnu".to_string(),
        description: "An ornate belt buckle.".to_string(),
        rarity: 0,
        icon: 134419,
        spell_description: "Solving rewards 50g.".to_string(),
        num_sockets: 2,
        bg_texture: "DwarfArchRare".to_string(),
        spell_id: 88910,
        base_progress: 30,
        adjust_progress: 5,
        total_cost: 35,
        can_solve: true,
    }
}

#[test]
fn get_selected_artifact_info_returns_nothing_without_selection() {
    let env = env();
    let result: Option<String> = env.eval("return (GetSelectedArtifactInfo())").unwrap();
    assert!(
        result.is_none(),
        "with no artifact selected the info call must yield nil so the artifact page short-circuits",
    );
}

#[test]
fn get_selected_artifact_info_returns_eight_values() {
    let env = env();
    seed_selected(&env, solve_ready_selected());
    let (name, description, rarity, icon, spell_description, num_sockets, bg_texture, spell_id): (
        String,
        String,
        i32,
        i32,
        String,
        i32,
        String,
        i32,
    ) = env.eval("return GetSelectedArtifactInfo()").unwrap();
    assert_eq!(name, "Belt Buckle of Zaldarinnu");
    assert_eq!(description, "An ornate belt buckle.");
    assert_eq!(rarity, 0);
    assert_eq!(icon, 134419);
    assert_eq!(spell_description, "Solving rewards 50g.");
    assert_eq!(num_sockets, 2);
    assert_eq!(bg_texture, "DwarfArchRare");
    assert_eq!(spell_id, 88910);
}

#[test]
fn set_selected_artifact_stores_race_id_only() {
    let env = env();
    env.exec("SetSelectedArtifact(3)").unwrap();
    let st = env.state().borrow();
    let selected = st
        .archaeology
        .selected
        .as_ref()
        .expect("SetSelectedArtifact must seed the slot");
    assert_eq!(selected.race_id, 3);
    assert!(
        selected.artifact_id.is_none(),
        "the single-arg form selects the race's pending artifact, not a historical one",
    );
}

#[test]
fn set_selected_artifact_stores_race_id_and_artifact_id() {
    let env = env();
    env.exec("SetSelectedArtifact(2, 7)").unwrap();
    let st = env.state().borrow();
    let selected = st.archaeology.selected.as_ref().unwrap();
    assert_eq!(selected.race_id, 2);
    assert_eq!(selected.artifact_id, Some(7));
}

#[test]
fn set_selected_artifact_preserves_existing_progress_fields() {
    let env = env();
    seed_selected(&env, solve_ready_selected());
    env.exec("SetSelectedArtifact(9, 4)").unwrap();
    let st = env.state().borrow();
    let selected = st.archaeology.selected.as_ref().unwrap();
    assert_eq!(selected.race_id, 9, "race_id must update to the new value");
    assert_eq!(selected.artifact_id, Some(4));
    assert_eq!(
        selected.base_progress, 30,
        "the rest of the artifact info is preserved so callers can re-target the same artifact without losing fragments",
    );
    assert_eq!(selected.total_cost, 35);
    assert!(selected.can_solve);
}

#[test]
fn get_artifact_progress_returns_zeros_without_selection() {
    let env = env();
    let (base, adjust, total): (i32, i32, i32) = env.eval("return GetArtifactProgress()").unwrap();
    assert_eq!(base, 0);
    assert_eq!(adjust, 0);
    assert_eq!(total, 0);
}

#[test]
fn get_artifact_progress_returns_three_values() {
    let env = env();
    seed_selected(&env, solve_ready_selected());
    let (base, adjust, total): (i32, i32, i32) = env.eval("return GetArtifactProgress()").unwrap();
    assert_eq!(base, 30);
    assert_eq!(adjust, 5);
    assert_eq!(total, 35);
}

#[test]
fn can_solve_artifact_returns_false_without_selection() {
    let env = env();
    let result: bool = env.eval("return CanSolveArtifact()").unwrap();
    assert!(!result);
}

#[test]
fn can_solve_artifact_reflects_selected_flag() {
    let env = env();
    seed_selected(&env, solve_ready_selected());
    let result: bool = env.eval("return CanSolveArtifact()").unwrap();
    assert!(result, "selected.can_solve = true must surface as Lua true");
}

#[test]
fn can_solve_artifact_returns_false_when_progress_insufficient() {
    let env = env();
    let mut selected = solve_ready_selected();
    selected.base_progress = 10;
    selected.adjust_progress = 0;
    selected.can_solve = false;
    seed_selected(&env, selected);
    let result: bool = env.eval("return CanSolveArtifact()").unwrap();
    assert!(!result);
}

#[test]
fn solve_artifact_zeros_progress_and_clears_can_solve() {
    let env = env();
    seed_selected(&env, solve_ready_selected());
    env.exec("SolveArtifact()").unwrap();
    let st = env.state().borrow();
    let selected = st.archaeology.selected.as_ref().unwrap();
    assert_eq!(selected.base_progress, 0);
    assert_eq!(selected.adjust_progress, 0);
    assert!(!selected.can_solve);
}

#[test]
fn solve_artifact_fires_research_artifact_complete_with_name() {
    let env = env();
    seed_selected(&env, solve_ready_selected());
    env.exec("SolveArtifact()").unwrap();
    let st = env.state().borrow();
    let event = st
        .events
        .pending()
        .iter()
        .find(|e| e.name == "RESEARCH_ARTIFACT_COMPLETE")
        .expect("RESEARCH_ARTIFACT_COMPLETE must fire");
    assert!(
        matches!(&event.args[0], EventArg::String(s) if s == "Belt Buckle of Zaldarinnu"),
        "the artifact name is the documented payload of RESEARCH_ARTIFACT_COMPLETE",
    );
}

#[test]
fn solve_artifact_is_noop_without_selection() {
    let env = env();
    env.exec("SolveArtifact()").unwrap();
    let st = env.state().borrow();
    assert!(
        st.archaeology.selected.is_none(),
        "no selection means nothing to mutate",
    );
    let fired = st
        .events
        .pending()
        .iter()
        .any(|e| e.name == "RESEARCH_ARTIFACT_COMPLETE");
    assert!(
        !fired,
        "without an artifact to solve, no completion event should fire",
    );
}
