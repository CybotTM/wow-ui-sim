//! Integration tests for C_EncounterJournal.GetEncounterInfo / GetInstanceInfo.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

// ---------------------------------------------------------------------------
// GetEncounterInfo
// ---------------------------------------------------------------------------

#[test]
fn test_get_encounter_info_known_returns_fields() {
    let env = env();
    let (name, enc_id, journal_inst_id, inst_id): (String, i64, i64, i64) = env
        .eval(
            r#"
            local name, _, eid, _, _, jiid, _, iid = C_EncounterJournal.GetEncounterInfo(2587)
            return name, eid, jiid, iid
            "#,
        )
        .unwrap();
    assert_eq!(name, "Eranog");
    assert_eq!(enc_id, 2587_i64);
    assert_eq!(journal_inst_id, 1193_i64);
    assert_eq!(inst_id, 1200_i64);
}

#[test]
fn test_get_encounter_info_unknown_returns_nothing() {
    let env = env();
    let is_nil: bool = env
        .eval(
            r#"
            local name = C_EncounterJournal.GetEncounterInfo(99999)
            return name == nil
            "#,
        )
        .unwrap();
    assert!(is_nil);
}

#[test]
fn test_get_encounter_info_fyrakk() {
    let env = env();
    let (name, inst_id): (String, i64) = env
        .eval(
            r#"
            local name, _, _, _, _, _, _, iid = C_EncounterJournal.GetEncounterInfo(2737)
            return name, iid
            "#,
        )
        .unwrap();
    assert_eq!(name, "Fyrakk the Blazing");
    assert_eq!(inst_id, 2549_i64);
}

#[test]
fn test_get_encounter_info_queen_ansurek() {
    let env = env();
    let (name, inst_id): (String, i64) = env
        .eval(
            r#"
            local name, _, _, _, _, _, _, iid = C_EncounterJournal.GetEncounterInfo(2922)
            return name, iid
            "#,
        )
        .unwrap();
    assert_eq!(name, "Queen Ansurek");
    assert_eq!(inst_id, 2657_i64);
}

#[test]
fn test_get_encounter_info_full_tuple_count() {
    let env = env();
    let count: i64 = env
        .eval(
            r#"
            local a,b,c,d,e,f,g,h = C_EncounterJournal.GetEncounterInfo(2902)
            local n = 0
            for _, v in ipairs({a,b,c,d,e,f,g,h}) do n = n + 1 end
            -- count non-nil manually since select('#',...) won't work here
            local t = {a,b,c,d,e,f,g,h}
            return #t
            "#,
        )
        .unwrap();
    assert_eq!(count, 8_i64);
}

// ---------------------------------------------------------------------------
// GetInstanceInfo
// ---------------------------------------------------------------------------

#[test]
fn test_get_instance_info_known_returns_name() {
    let env = env();
    let name: String = env
        .eval(
            r#"
            local name = C_EncounterJournal.GetInstanceInfo(2657)
            return name
            "#,
        )
        .unwrap();
    assert_eq!(name, "Nerub-ar Palace");
}

#[test]
fn test_get_instance_info_unknown_returns_nothing() {
    let env = env();
    let is_nil: bool = env
        .eval(
            r#"
            local name = C_EncounterJournal.GetInstanceInfo(99999)
            return name == nil
            "#,
        )
        .unwrap();
    assert!(is_nil);
}

#[test]
fn test_get_instance_info_amirdrassil() {
    let env = env();
    let (name, bg_image): (String, String) = env
        .eval(
            r#"
            local name, _, bg = C_EncounterJournal.GetInstanceInfo(2549)
            return name, bg
            "#,
        )
        .unwrap();
    assert_eq!(name, "Amirdrassil, the Dream's Hope");
    assert!(
        bg_image.contains("Amirdrassil"),
        "expected Amirdrassil in bg_image, got: {bg_image}"
    );
}

#[test]
fn test_encounter_journal_tier_selection_round_trips() {
    let env = env();
    let (initial_tier, selected_tier): (i64, i64) = env
        .eval(
            r#"
            C_EncounterJournal.InitalizeSelectedTier()
            local initialTier = EJ_GetCurrentTier()
            EJ_SelectTier(11)
            return initialTier, EJ_GetCurrentTier()
            "#,
        )
        .unwrap();
    assert_eq!(initial_tier, 10_i64);
    assert_eq!(selected_tier, 11_i64);
}

#[test]
fn encounter_journal_search_surface_is_available_and_empty() {
    let env = env();
    env.exec(
        r#"
        assert(type(EJ_EndSearch) == "function")
        assert(type(EJ_ClearSearch) == "function")
        assert(type(EJ_SetSearch) == "function")
        assert(type(EJ_GetSearchSize) == "function")
        assert(type(EJ_GetSearchProgress) == "function")
        assert(type(EJ_GetNumSearchResults) == "function")
        assert(type(EJ_GetSearchResult) == "function")
        assert(type(EJ_IsSearchFinished) == "function")

        EJ_SetSearch("fyrakk")
        EJ_EndSearch()
        EJ_ClearSearch()

        assert(EJ_GetSearchSize() == 0)
        assert(EJ_GetSearchProgress() == 0)
        assert(EJ_GetNumSearchResults() == 0)
        assert(EJ_GetSearchResult(1) == nil)
        assert(EJ_IsSearchFinished() == true)
        "#,
    )
    .unwrap();
}
