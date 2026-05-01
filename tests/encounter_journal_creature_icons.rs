use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn missing_creature_icon_uses_blizzard_boss_button_default() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");

    let uses_default: bool = env
        .eval(
            r#"
            local bossImage = select(5, EJ_GetCreatureInfo(1, 2773))
                or "Interface\\EncounterJournal\\UI-EJ-BOSS-Default"
            return bossImage == "Interface\\EncounterJournal\\UI-EJ-BOSS-Default"
            "#,
        )
        .expect("boss icon fallback probe should run");

    assert!(
        uses_default,
        "Blizzard boss buttons rely on nil, not 0, to fall back to the default boss icon"
    );
}

#[test]
fn creature_icon_file_data_ids_still_round_trip_when_present() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");

    let icon_file_id: i64 = env
        .eval("return select(5, EJ_GetCreatureInfo(1, 2583))")
        .expect("creature icon probe should run");

    assert_eq!(
        icon_file_id, 5_907_251,
        "non-zero Encounter Journal creature icon fileDataIDs must still be returned"
    );
}
