use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::texture::TextureManager;

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

#[test]
fn magisters_terrace_creature_icon_file_data_ids_round_trip() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");

    let ids: Vec<i64> = env
        .eval(
            r#"
            local ids = {}
            for _, encounterID in ipairs({2659, 2660, 2661, 2662}) do
                local iconFileID = select(5, EJ_GetCreatureInfo(1, encounterID))
                table.insert(ids, iconFileID)
            end
            return ids
            "#,
        )
        .expect("Magisters' Terrace creature icon probe should run");

    assert_eq!(ids, vec![7_433_976, 7_372_494, 7_372_512, 7_372_491]);
}

#[test]
fn magisters_terrace_boss_portraits_load_from_file_data_ids() {
    let mut textures = TextureManager::new();
    for file_data_id in [7_433_976, 7_372_494, 7_372_512, 7_372_491] {
        let path = wow_ui_sim::manifest_interface_data::get_texture_path(file_data_id)
            .unwrap_or_else(|| panic!("missing manifest path for {file_data_id}"));
        let wow_path = format!("Interface\\{}", path.replace('/', "\\"));
        let loaded = textures.load(&wow_path);
        assert!(
            loaded.is_some(),
            "missing boss portrait texture {file_data_id} at {wow_path}"
        );
    }
}
