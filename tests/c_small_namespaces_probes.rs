//! Tests for small C_* namespace registrations in
//! `missing_surface/small_namespaces.rs`.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

// ── C_TrophyHall ─────────────────────────────────────────────────────────────

#[test]
fn trophy_hall_get_trophy_info_returns_nil() {
    let env = env();
    let result: Option<bool> = env.eval("return C_TrophyHall.GetTrophyInfo(1)").unwrap();
    assert!(result.is_none(), "GetTrophyInfo must return nil");
}

// ── C_StableInfo ─────────────────────────────────────────────────────────────

#[test]
fn stable_info_is_at_pet_stable_false_by_default() {
    let env = env();
    let result: bool = env.eval("return C_StableInfo.IsAtPetStable()").unwrap();
    assert!(!result, "IsAtPetStable() must be false when stables closed");
}

#[test]
fn stable_info_is_at_pet_stable_reflects_sim_state() {
    let env = env();
    {
        let mut sim = env.state().borrow_mut();
        sim.pet_stables_open = true;
    }
    let result: bool = env.eval("return C_StableInfo.IsAtPetStable()").unwrap();
    assert!(
        result,
        "IsAtPetStable() must be true when pet_stables_open=true"
    );
}

// ── C_GarrisonInfo ───────────────────────────────────────────────────────────

#[test]
fn garrison_info_has_garrison_false() {
    let env = env();
    let result: bool = env.eval("return C_GarrisonInfo.HasGarrison()").unwrap();
    assert!(!result, "HasGarrison() must be false");
}

#[test]
fn garrison_info_get_garrison_type_zero() {
    let env = env();
    let result: i32 = env.eval("return C_GarrisonInfo.GetGarrisonType()").unwrap();
    assert_eq!(result, 0, "GetGarrisonType() must return 0");
}

// ── C_Map ────────────────────────────────────────────────────────────────────

#[test]
fn map_is_map_valid_for_navigation_false() {
    let env = env();
    let result: bool = env
        .eval("return C_Map.IsMapValidForNavigation(1234)")
        .unwrap();
    assert!(!result, "IsMapValidForNavigation() must be false");
}

// ── C_PvP ─────────────────────────────────────────────────────────────────────

#[test]
fn pvp_is_match_considered_arena_false() {
    let env = env();
    let result: bool = env.eval("return C_PvP.IsMatchConsideredArena()").unwrap();
    assert!(!result, "IsMatchConsideredArena() must be false");
}

// ── C_LossOfControl ──────────────────────────────────────────────────────────

#[test]
fn loss_of_control_get_active_data_returns_nil() {
    let env = env();
    let result: Option<bool> = env
        .eval("return C_LossOfControl.GetActiveLossOfControlData(1)")
        .unwrap();
    assert!(
        result.is_none(),
        "GetActiveLossOfControlData() must return nil"
    );
}

#[test]
fn loss_of_control_get_active_data_count_is_zero() {
    let env = env();
    let count: i32 = env
        .eval("return C_LossOfControl.GetActiveLossOfControlDataCount()")
        .unwrap();
    assert_eq!(count, 0, "GetActiveLossOfControlDataCount() must return 0");
}

// ── C_Bank ───────────────────────────────────────────────────────────────────

#[test]
fn bank_has_full_bank_access_true() {
    let env = env();
    let result: bool = env.eval("return C_Bank.HasFullBankAccess()").unwrap();
    assert!(result, "HasFullBankAccess() must return true");
}

// ── C_MajorFactions ──────────────────────────────────────────────────────────

#[test]
fn major_factions_default_to_empty_iterable_state() {
    let env = env();
    let (count, hidden, as_journey): (i32, bool, bool) = env
        .eval(
            r#"
            local ids = C_MajorFactions.GetMajorFactionIDs(10)
            local data = C_MajorFactions.GetMajorFactionData(2507)
            return #ids, C_MajorFactions.IsMajorFactionHiddenFromExpansionPage(data.factionID), C_MajorFactions.ShouldDisplayMajorFactionAsJourney(data.factionID)
            "#,
        )
        .unwrap();
    assert_eq!(
        count, 0,
        "GetMajorFactionIDs() should default to an empty table"
    );
    assert!(
        !hidden,
        "IsMajorFactionHiddenFromExpansionPage() should default to false"
    );
    assert!(
        !as_journey,
        "ShouldDisplayMajorFactionAsJourney() should default to false"
    );
}
