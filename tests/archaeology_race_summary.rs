//! Integration tests for the legacy archaeology race-summary surface
//! (`GetArchaeologyInfo`, `GetNumArchaeologyRaces`,
//! `GetArchaeologyRaceInfo`, `GetNumArtifactsByRace`) consumed by
//! `Blizzard_ArchaeologyUI/Blizzard_ArchaeologyUI.lua` lines 102, 138,
//! 141, 173, 270, 421, 487 during `ArchaeologyFrame_OnLoad` and the
//! summary/dropdown update paths.

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::{ArchaeologyArtifact, ArchaeologyRace, ArchaeologyState};

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

fn seed_two_races(env: &WowLuaEnv) {
    let state = env.state();
    let mut sim = state.borrow_mut();
    sim.archaeology = ArchaeologyState {
        profession_name: "Archaeology".to_string(),
        races: vec![
            ArchaeologyRace {
                name: "Dwarf".to_string(),
                texture: 460983,
                race_item_id: 63127,
                currency_amount: 12,
                project_amount: 35,
                artifacts: vec![
                    ArchaeologyArtifact::default(),
                    ArchaeologyArtifact::default(),
                ],
            },
            ArchaeologyRace {
                name: "Troll".to_string(),
                texture: 460982,
                race_item_id: 63128,
                currency_amount: 0,
                project_amount: 45,
                artifacts: vec![ArchaeologyArtifact::default()],
            },
        ],
        selected: None,
        keystone_value: 0,
        history_available: false,
        last_close_request: None,
    };
}

#[test]
fn get_archaeology_info_returns_default_profession_name() {
    let env = env();
    let name: String = env.eval("return GetArchaeologyInfo()").unwrap();
    assert_eq!(
        name, "Archaeology",
        "default ArchaeologyState surfaces the localized 'Archaeology' literal so OnLoad's SetTitle call has a real string",
    );
}

#[test]
fn get_archaeology_info_round_trips_state_mutation() {
    let env = env();
    {
        let state = env.state();
        let mut sim = state.borrow_mut();
        sim.archaeology.profession_name = "Archéologie".to_string();
    }
    let name: String = env.eval("return GetArchaeologyInfo()").unwrap();
    assert_eq!(
        name, "Archéologie",
        "profession_name mutation must round-trip into the Lua return",
    );
}

#[test]
fn get_num_archaeology_races_defaults_to_zero() {
    let env = env();
    let count: i32 = env.eval("return GetNumArchaeologyRaces()").unwrap();
    assert_eq!(
        count, 0,
        "fresh sim has no seeded races, so the dropdown's `for raceIndex = 1, GetNumArchaeologyRaces()` loop never enters",
    );
}

#[test]
fn get_num_archaeology_races_reflects_seeded_count() {
    let env = env();
    seed_two_races(&env);
    let count: i32 = env.eval("return GetNumArchaeologyRaces()").unwrap();
    assert_eq!(count, 2, "2 races seeded = 2 returned");
}

#[test]
fn get_archaeology_race_info_returns_five_values_for_valid_index() {
    let env = env();
    seed_two_races(&env);
    let (name, texture, race_item_id, currency_amount, project_amount): (
        String,
        i32,
        i32,
        i32,
        i32,
    ) = env.eval("return GetArchaeologyRaceInfo(1)").unwrap();
    assert_eq!(name, "Dwarf");
    assert_eq!(texture, 460983);
    assert_eq!(race_item_id, 63127);
    assert_eq!(currency_amount, 12);
    assert_eq!(project_amount, 35);
}

#[test]
fn get_archaeology_race_info_resolves_second_index() {
    let env = env();
    seed_two_races(&env);
    let (name, _, _, currency_amount, project_amount): (String, i32, i32, i32, i32) =
        env.eval("return GetArchaeologyRaceInfo(2)").unwrap();
    assert_eq!(name, "Troll");
    assert_eq!(currency_amount, 0);
    assert_eq!(project_amount, 45);
}

#[test]
fn get_archaeology_race_info_returns_nil_for_out_of_range() {
    let env = env();
    seed_two_races(&env);
    let returned: Option<String> = env.eval("return (GetArchaeologyRaceInfo(99))").unwrap();
    assert!(
        returned.is_none(),
        "out-of-range raceIndex must yield nil so the dropdown's `if numProjects > 0` guard short-circuits cleanly",
    );
}

#[test]
fn get_archaeology_race_info_returns_nil_for_zero_index() {
    let env = env();
    seed_two_races(&env);
    let returned: Option<String> = env.eval("return (GetArchaeologyRaceInfo(0))").unwrap();
    assert!(
        returned.is_none(),
        "raceIndex is 1-based; 0 must yield nil rather than aliasing race 1",
    );
}

#[test]
fn get_archaeology_race_info_accepts_optional_get_current_artifact_arg() {
    let env = env();
    seed_two_races(&env);
    let name_with_flag: String = env
        .eval("return (GetArchaeologyRaceInfo(1, true))")
        .unwrap();
    let name_without_flag: String = env.eval("return (GetArchaeologyRaceInfo(1))").unwrap();
    assert_eq!(
        name_with_flag, name_without_flag,
        "the optional second arg must not change the returned name (race-summary surface ignores it; the active-artifact branch is filed separately)",
    );
}

#[test]
fn get_num_artifacts_by_race_returns_seeded_count() {
    let env = env();
    seed_two_races(&env);
    let count_one: i32 = env.eval("return GetNumArtifactsByRace(1)").unwrap();
    let count_two: i32 = env.eval("return GetNumArtifactsByRace(2)").unwrap();
    assert_eq!(count_one, 2, "race 1 has 2 artifacts seeded");
    assert_eq!(count_two, 1, "race 2 has 1 artifact seeded");
}

#[test]
fn get_num_artifacts_by_race_returns_zero_for_out_of_range() {
    let env = env();
    seed_two_races(&env);
    let count: i32 = env.eval("return GetNumArtifactsByRace(99)").unwrap();
    assert_eq!(
        count, 0,
        "out-of-range race index must return 0 so the OnLoad `if numProjects > 0` guard skips the slot",
    );
}

#[test]
fn legacy_archaeology_globals_are_registered_as_functions() {
    let env = env();
    let kinds: (String, String, String, String) = env
        .eval(
            r#"
            return type(GetArchaeologyInfo),
                   type(GetNumArchaeologyRaces),
                   type(GetArchaeologyRaceInfo),
                   type(GetNumArtifactsByRace)
            "#,
        )
        .unwrap();
    assert_eq!(kinds.0, "function");
    assert_eq!(kinds.1, "function");
    assert_eq!(kinds.2, "function");
    assert_eq!(kinds.3, "function");
}
