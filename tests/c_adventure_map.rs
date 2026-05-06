//! Integration tests for the `C_AdventureMap` namespace registered in
//! `src/lua_api/globals/adventure_map.rs`.
//!
//! `GetMapID()` is read during `AdventureMapMixin:OnShow` and forwarded to
//! `MapCanvasMixin:SetMapID` (see `Blizzard_AdventureMap.lua:45`).
//!
//! `Close()` is an async hint to the server that the player closed the
//! adventure map; the simulator stamps `state.adventure_map.last_closed`
//! with the elapsed game time so tests can assert it was invoked. The
//! function reference is stored directly on
//! `UIPanelWindows["AdventureMapFrame"].showFailedFunc`, so it must be
//! present at addon load time (see `Blizzard_AdventureMap.lua:56`).

use wow_ui_sim::lua_api::{AdventureMapInset, AdventureMapZoneChoice, WowLuaEnv};

#[path = "c_adventure_map/quests.rs"]
mod quests;

#[test]
fn c_adventure_map_namespace_is_a_table() {
    let env = WowLuaEnv::new().expect("env");
    let kind: String = env.eval("return type(C_AdventureMap)").unwrap();
    assert_eq!(kind, "table");
}

#[test]
fn get_map_id_is_a_function() {
    let env = WowLuaEnv::new().expect("env");
    let kind: String = env.eval("return type(C_AdventureMap.GetMapID)").unwrap();
    assert_eq!(kind, "function");
}

#[test]
fn get_map_id_defaults_to_zero() {
    let env = WowLuaEnv::new().expect("env");
    let map_id: f64 = env.eval("return C_AdventureMap.GetMapID()").unwrap();
    assert!(map_id.abs() < 1e-6);
}

#[test]
fn get_map_id_returns_seeded_value() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().adventure_map.map_id = 619;

    let map_id: f64 = env.eval("return C_AdventureMap.GetMapID()").unwrap();
    assert!((map_id - 619.0).abs() < 1e-6);
}

#[test]
fn get_map_id_returns_a_number_type() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().adventure_map.map_id = 42;

    let kind: String = env.eval("return type(C_AdventureMap.GetMapID())").unwrap();
    assert_eq!(kind, "number");
}

#[test]
fn close_is_a_function() {
    let env = WowLuaEnv::new().expect("env");
    let kind: String = env.eval("return type(C_AdventureMap.Close)").unwrap();
    assert_eq!(kind, "function");
}

#[test]
fn close_returns_no_values() {
    let env = WowLuaEnv::new().expect("env");
    let nothing: bool = env
        .eval("return select('#', C_AdventureMap.Close()) == 0")
        .unwrap();
    assert!(nothing, "Close should return zero values");
}

#[test]
fn close_records_a_timestamp_on_state() {
    let env = WowLuaEnv::new().expect("env");
    assert!(
        env.state().borrow().adventure_map.last_closed.is_none(),
        "last_closed should be None before any Close call"
    );

    env.exec("C_AdventureMap.Close()").unwrap();

    let last_closed = env
        .state()
        .borrow()
        .adventure_map
        .last_closed
        .expect("Close should populate last_closed");
    assert!(
        last_closed >= 0.0,
        "last_closed should be a non-negative elapsed-time value, got {last_closed}"
    );
}

#[test]
fn close_overwrites_previous_timestamp() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().adventure_map.last_closed = Some(1.0);

    env.exec("C_AdventureMap.Close()").unwrap();

    let last_closed = env.state().borrow().adventure_map.last_closed.unwrap();
    assert!(
        last_closed != 1.0,
        "Close should overwrite the seed timestamp"
    );
}

#[test]
fn close_can_be_stored_as_a_direct_reference() {
    let env = WowLuaEnv::new().expect("env");
    env.exec(
        r#"
        UIPanelWindows = UIPanelWindows or {}
        UIPanelWindows.AdventureMapFrame = { showFailedFunc = C_AdventureMap.Close }
        UIPanelWindows.AdventureMapFrame.showFailedFunc()
        "#,
    )
    .unwrap();

    assert!(
        env.state().borrow().adventure_map.last_closed.is_some(),
        "showFailedFunc reference should reach the simulator and stamp last_closed"
    );
}

#[test]
fn get_num_map_insets_is_a_function() {
    let env = WowLuaEnv::new().expect("env");
    let kind: String = env
        .eval("return type(C_AdventureMap.GetNumMapInsets)")
        .unwrap();
    assert_eq!(kind, "function");
}

#[test]
fn get_num_map_insets_returns_nil_when_unloaded() {
    let env = WowLuaEnv::new().expect("env");
    let is_nil: bool = env
        .eval("return C_AdventureMap.GetNumMapInsets() == nil")
        .unwrap();
    assert!(
        is_nil,
        "GetNumMapInsets must return nil before inset metadata is published \
         so AdventureMapMixin:RefreshInsets can short-circuit"
    );
}

#[test]
fn get_num_map_insets_returns_zero_when_loaded_empty() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().adventure_map.insets = Some(Vec::new());

    let count: f64 = env.eval("return C_AdventureMap.GetNumMapInsets()").unwrap();
    assert!(count.abs() < 1e-6);
}

#[test]
fn get_num_map_insets_returns_seeded_length() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().adventure_map.insets = Some(vec![
        AdventureMapInset::default(),
        AdventureMapInset::default(),
        AdventureMapInset::default(),
    ]);

    let count: f64 = env.eval("return C_AdventureMap.GetNumMapInsets()").unwrap();
    assert!((count - 3.0).abs() < 1e-6);
}

#[test]
fn refresh_insets_guard_short_circuits_on_nil() {
    let env = WowLuaEnv::new().expect("env");
    env.exec(
        r#"
        _G.__refresh_ran = false
        local numInsets = C_AdventureMap.GetNumMapInsets()
        if numInsets and numInsets > 0 then
            _G.__refresh_ran = true
        end
        "#,
    )
    .unwrap();

    let ran: bool = env.eval("return _G.__refresh_ran").unwrap();
    assert!(
        !ran,
        "RefreshInsets-style guard must skip the body when the count is nil"
    );
}

fn sample_inset() -> AdventureMapInset {
    AdventureMapInset {
        map_id: 627,
        title: "Stormheim".to_string(),
        description: "Vrykul homeland and the Halls of Valor.".to_string(),
        collapsed_icon: "AdventureMapIcon-Stormheim".to_string(),
        area_table_id: 7558,
        num_detail_tiles: 8,
        normalized_x: 0.42,
        normalized_y: 0.18,
        detail_tiles: vec![1_001, 1_002, 1_003, 1_004, 1_005, 1_006, 1_007, 1_008],
    }
}

#[test]
fn get_map_inset_info_is_a_function() {
    let env = WowLuaEnv::new().expect("env");
    let kind: String = env
        .eval("return type(C_AdventureMap.GetMapInsetInfo)")
        .unwrap();
    assert_eq!(kind, "function");
}

#[test]
fn get_map_inset_info_returns_no_values_when_unloaded() {
    let env = WowLuaEnv::new().expect("env");
    let count: f64 = env
        .eval("return select('#', C_AdventureMap.GetMapInsetInfo(1))")
        .unwrap();
    assert!(count.abs() < 1e-6);
}

#[test]
fn get_map_inset_info_returns_no_values_for_out_of_range_index() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().adventure_map.insets = Some(vec![sample_inset()]);

    let count: f64 = env
        .eval("return select('#', C_AdventureMap.GetMapInsetInfo(2))")
        .unwrap();
    assert!(count.abs() < 1e-6);
}

#[test]
fn get_map_inset_info_returns_no_values_for_non_positive_index() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().adventure_map.insets = Some(vec![sample_inset()]);

    let zero_count: f64 = env
        .eval("return select('#', C_AdventureMap.GetMapInsetInfo(0))")
        .unwrap();
    let negative_count: f64 = env
        .eval("return select('#', C_AdventureMap.GetMapInsetInfo(-1))")
        .unwrap();
    assert!(zero_count.abs() < 1e-6);
    assert!(negative_count.abs() < 1e-6);
}

#[test]
fn get_map_inset_info_returns_eight_descriptor_values() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().adventure_map.insets = Some(vec![sample_inset()]);

    env.exec(
        "mapID, title, description, collapsedIcon, areaTableID, \
         numDetailTiles, normalizedX, normalizedY = \
         C_AdventureMap.GetMapInsetInfo(1)",
    )
    .unwrap();

    let map_id: f64 = env.eval("return mapID").unwrap();
    let title: String = env.eval("return title").unwrap();
    let description: String = env.eval("return description").unwrap();
    let collapsed_icon: String = env.eval("return collapsedIcon").unwrap();
    let area_table_id: f64 = env.eval("return areaTableID").unwrap();
    let num_detail_tiles: f64 = env.eval("return numDetailTiles").unwrap();
    let normalized_x: f64 = env.eval("return normalizedX").unwrap();
    let normalized_y: f64 = env.eval("return normalizedY").unwrap();

    assert!((map_id - 627.0).abs() < 1e-6);
    assert_eq!(title, "Stormheim");
    assert_eq!(description, "Vrykul homeland and the Halls of Valor.");
    assert_eq!(collapsed_icon, "AdventureMapIcon-Stormheim");
    assert!((area_table_id - 7558.0).abs() < 1e-6);
    assert!((num_detail_tiles - 8.0).abs() < 1e-6);
    assert!((normalized_x - 0.42).abs() < 1e-6);
    assert!((normalized_y - 0.18).abs() < 1e-6);
}

#[test]
fn get_map_inset_info_indexes_one_based() {
    let env = WowLuaEnv::new().expect("env");
    let mut second = sample_inset();
    second.map_id = 630;
    second.title = "Suramar".to_string();
    env.state().borrow_mut().adventure_map.insets = Some(vec![sample_inset(), second]);

    let map_ids: (f64, f64) = env
        .eval(
            "local a = C_AdventureMap.GetMapInsetInfo(1) \
             local b = C_AdventureMap.GetMapInsetInfo(2) \
             return a, b",
        )
        .unwrap();
    assert!((map_ids.0 - 627.0).abs() < 1e-6);
    assert!((map_ids.1 - 630.0).abs() < 1e-6);
}

#[test]
fn is_map_inset_expanded_pattern_uses_only_first_return() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().adventure_map.insets = Some(vec![sample_inset()]);

    let map_id: f64 = env
        .eval("local mapID = C_AdventureMap.GetMapInsetInfo(1) return mapID")
        .unwrap();
    assert!(
        (map_id - 627.0).abs() < 1e-6,
        "IsMapInsetExpanded pattern must read mapID as the first return value"
    );
}

#[test]
fn get_map_inset_detail_tile_info_is_a_function() {
    let env = WowLuaEnv::new().expect("env");
    let kind: String = env
        .eval("return type(C_AdventureMap.GetMapInsetDetailTileInfo)")
        .unwrap();
    assert_eq!(kind, "function");
}

#[test]
fn get_map_inset_detail_tile_info_returns_seeded_id() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().adventure_map.insets = Some(vec![sample_inset()]);

    let id: f64 = env
        .eval("return C_AdventureMap.GetMapInsetDetailTileInfo(1, 3)")
        .unwrap();
    assert!((id - 1_003.0).abs() < 1e-6);
}

#[test]
fn get_map_inset_detail_tile_info_indexes_one_based() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().adventure_map.insets = Some(vec![sample_inset()]);

    let first: f64 = env
        .eval("return C_AdventureMap.GetMapInsetDetailTileInfo(1, 1)")
        .unwrap();
    let last: f64 = env
        .eval("return C_AdventureMap.GetMapInsetDetailTileInfo(1, 8)")
        .unwrap();
    assert!((first - 1_001.0).abs() < 1e-6);
    assert!((last - 1_008.0).abs() < 1e-6);
}

#[test]
fn get_map_inset_detail_tile_info_returns_no_values_when_unloaded() {
    let env = WowLuaEnv::new().expect("env");
    let count: f64 = env
        .eval("return select('#', C_AdventureMap.GetMapInsetDetailTileInfo(1, 1))")
        .unwrap();
    assert!(count.abs() < 1e-6);
}

#[test]
fn get_map_inset_detail_tile_info_returns_no_values_for_invalid_inset() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().adventure_map.insets = Some(vec![sample_inset()]);

    let count: f64 = env
        .eval("return select('#', C_AdventureMap.GetMapInsetDetailTileInfo(2, 1))")
        .unwrap();
    assert!(count.abs() < 1e-6);
}

#[test]
fn get_map_inset_detail_tile_info_returns_no_values_for_out_of_range_tile() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().adventure_map.insets = Some(vec![sample_inset()]);

    let count: f64 = env
        .eval("return select('#', C_AdventureMap.GetMapInsetDetailTileInfo(1, 9))")
        .unwrap();
    assert!(count.abs() < 1e-6);
}

#[test]
fn get_map_inset_detail_tile_info_returns_no_values_for_non_positive_tile() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().adventure_map.insets = Some(vec![sample_inset()]);

    let zero_count: f64 = env
        .eval("return select('#', C_AdventureMap.GetMapInsetDetailTileInfo(1, 0))")
        .unwrap();
    let negative_count: f64 = env
        .eval("return select('#', C_AdventureMap.GetMapInsetDetailTileInfo(1, -1))")
        .unwrap();
    assert!(zero_count.abs() < 1e-6);
    assert!(negative_count.abs() < 1e-6);
}

#[test]
fn get_num_zone_choices_is_a_function() {
    let env = WowLuaEnv::new().expect("env");
    let kind: String = env
        .eval("return type(C_AdventureMap.GetNumZoneChoices)")
        .unwrap();
    assert_eq!(kind, "function");
}

#[test]
fn get_num_zone_choices_defaults_to_zero() {
    let env = WowLuaEnv::new().expect("env");
    let count: f64 = env
        .eval("return C_AdventureMap.GetNumZoneChoices()")
        .unwrap();
    assert!(
        count.abs() < 1e-6,
        "GetNumZoneChoices must return 0 (not nil) before any choice is published"
    );
}

#[test]
fn get_num_zone_choices_returns_a_number_type() {
    let env = WowLuaEnv::new().expect("env");
    let kind: String = env
        .eval("return type(C_AdventureMap.GetNumZoneChoices())")
        .unwrap();
    assert_eq!(kind, "number");
}

#[test]
fn get_num_zone_choices_returns_seeded_length() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().adventure_map.zone_choices = vec![
        AdventureMapZoneChoice::default(),
        AdventureMapZoneChoice::default(),
        AdventureMapZoneChoice::default(),
        AdventureMapZoneChoice::default(),
    ];

    let count: f64 = env
        .eval("return C_AdventureMap.GetNumZoneChoices()")
        .unwrap();
    assert!((count - 4.0).abs() < 1e-6);
}

#[test]
fn get_num_zone_choices_supports_iteration_loop() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().adventure_map.zone_choices = vec![
        AdventureMapZoneChoice::default(),
        AdventureMapZoneChoice::default(),
    ];

    env.exec(
        r#"
        _G.__visited = 0
        for _ = 1, C_AdventureMap.GetNumZoneChoices() do
            _G.__visited = _G.__visited + 1
        end
        "#,
    )
    .unwrap();

    let visited: f64 = env.eval("return _G.__visited").unwrap();
    assert!(
        (visited - 2.0).abs() < 1e-6,
        "for-loop bound by GetNumZoneChoices must iterate exactly the seeded count"
    );
}

fn sample_zone_choice() -> AdventureMapZoneChoice {
    AdventureMapZoneChoice {
        quest_id: 40_519,
        texture_kit: "alliance".to_string(),
        name: "Azsuna".to_string(),
        zone_description: "Reclaim the lost magic of the night elves.".to_string(),
        normalized_x: 0.31,
        normalized_y: 0.55,
    }
}

#[test]
fn get_zone_choice_info_is_a_function() {
    let env = WowLuaEnv::new().expect("env");
    let kind: String = env
        .eval("return type(C_AdventureMap.GetZoneChoiceInfo)")
        .unwrap();
    assert_eq!(kind, "function");
}

#[test]
fn get_zone_choice_info_returns_no_values_when_unloaded() {
    let env = WowLuaEnv::new().expect("env");
    let count: f64 = env
        .eval("return select('#', C_AdventureMap.GetZoneChoiceInfo(1))")
        .unwrap();
    assert!(count.abs() < 1e-6);
}

#[test]
fn get_zone_choice_info_returns_no_values_for_out_of_range_index() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().adventure_map.zone_choices = vec![sample_zone_choice()];

    let count: f64 = env
        .eval("return select('#', C_AdventureMap.GetZoneChoiceInfo(2))")
        .unwrap();
    assert!(count.abs() < 1e-6);
}

#[test]
fn get_zone_choice_info_returns_no_values_for_non_positive_index() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().adventure_map.zone_choices = vec![sample_zone_choice()];

    let zero_count: f64 = env
        .eval("return select('#', C_AdventureMap.GetZoneChoiceInfo(0))")
        .unwrap();
    let negative_count: f64 = env
        .eval("return select('#', C_AdventureMap.GetZoneChoiceInfo(-1))")
        .unwrap();
    assert!(zero_count.abs() < 1e-6);
    assert!(negative_count.abs() < 1e-6);
}

#[test]
fn get_zone_choice_info_returns_six_descriptor_values() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().adventure_map.zone_choices = vec![sample_zone_choice()];

    env.exec(
        "questID, textureKit, name, zoneDescription, normalizedX, normalizedY = \
         C_AdventureMap.GetZoneChoiceInfo(1)",
    )
    .unwrap();

    let quest_id: f64 = env.eval("return questID").unwrap();
    let texture_kit: String = env.eval("return textureKit").unwrap();
    let name: String = env.eval("return name").unwrap();
    let zone_description: String = env.eval("return zoneDescription").unwrap();
    let normalized_x: f64 = env.eval("return normalizedX").unwrap();
    let normalized_y: f64 = env.eval("return normalizedY").unwrap();

    assert!((quest_id - 40_519.0).abs() < 1e-6);
    assert_eq!(texture_kit, "alliance");
    assert_eq!(name, "Azsuna");
    assert_eq!(
        zone_description,
        "Reclaim the lost magic of the night elves."
    );
    assert!((normalized_x - 0.31).abs() < 1e-6);
    assert!((normalized_y - 0.55).abs() < 1e-6);
}

#[test]
fn get_zone_choice_info_indexes_one_based() {
    let env = WowLuaEnv::new().expect("env");
    let mut second = sample_zone_choice();
    second.quest_id = 40_521;
    second.texture_kit = "horde".to_string();
    second.name = "Highmountain".to_string();
    env.state().borrow_mut().adventure_map.zone_choices = vec![sample_zone_choice(), second];

    let first_id: f64 = env
        .eval("local id = C_AdventureMap.GetZoneChoiceInfo(1) return id")
        .unwrap();
    let second_id: f64 = env
        .eval("local id = C_AdventureMap.GetZoneChoiceInfo(2) return id")
        .unwrap();
    let second_kit: String = env
        .eval("local _, kit = C_AdventureMap.GetZoneChoiceInfo(2) return kit")
        .unwrap();

    assert!((first_id - 40_519.0).abs() < 1e-6);
    assert!((second_id - 40_521.0).abs() < 1e-6);
    assert_eq!(second_kit, "horde");
}

#[test]
fn quest_choice_data_provider_pattern_collects_each_choice() {
    let env = WowLuaEnv::new().expect("env");
    let mut second = sample_zone_choice();
    second.quest_id = 40_521;
    second.name = "Highmountain".to_string();
    env.state().borrow_mut().adventure_map.zone_choices = vec![sample_zone_choice(), second];

    env.exec(
        r#"
        _G.__choice_ids = {}
        for i = 1, C_AdventureMap.GetNumZoneChoices() do
            local questID = C_AdventureMap.GetZoneChoiceInfo(i)
            _G.__choice_ids[i] = questID
        end
        "#,
    )
    .unwrap();

    let count: f64 = env.eval("return #_G.__choice_ids").unwrap();
    let first: f64 = env.eval("return _G.__choice_ids[1]").unwrap();
    let second_id: f64 = env.eval("return _G.__choice_ids[2]").unwrap();
    assert!((count - 2.0).abs() < 1e-6);
    assert!((first - 40_519.0).abs() < 1e-6);
    assert!((second_id - 40_521.0).abs() < 1e-6);
}

#[test]
fn get_adventure_map_texture_kit_is_a_function() {
    let env = WowLuaEnv::new().expect("env");
    let kind: String = env
        .eval("return type(C_AdventureMap.GetAdventureMapTextureKit)")
        .unwrap();
    assert_eq!(kind, "function");
}

#[test]
fn get_adventure_map_texture_kit_defaults_to_empty_string() {
    let env = WowLuaEnv::new().expect("env");
    let kit: String = env
        .eval("return C_AdventureMap.GetAdventureMapTextureKit()")
        .unwrap();
    assert_eq!(kit, "");
}

#[test]
fn get_adventure_map_texture_kit_returns_string_type() {
    let env = WowLuaEnv::new().expect("env");
    let kind: String = env
        .eval("return type(C_AdventureMap.GetAdventureMapTextureKit())")
        .unwrap();
    assert_eq!(kind, "string");
}

#[test]
fn get_adventure_map_texture_kit_returns_seeded_value() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().adventure_map.texture_kit = "midnight".to_string();

    let kit: String = env
        .eval("return C_AdventureMap.GetAdventureMapTextureKit()")
        .unwrap();
    assert_eq!(kit, "midnight");
}

#[test]
fn get_adventure_map_texture_kit_drives_dialog_portrait_branch() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().adventure_map.texture_kit = "midnight".to_string();

    env.exec(
        r#"
        local kit = C_AdventureMap.GetAdventureMapTextureKit()
        if kit == "midnight" then
            _G.__portrait_atlas = "ui-prey-scoutingmap"
        else
            _G.__portrait_atlas = "FXAM-QuestBang"
        end
        "#,
    )
    .unwrap();

    let atlas: String = env.eval("return _G.__portrait_atlas").unwrap();
    assert_eq!(
        atlas, "ui-prey-scoutingmap",
        "midnight kit must drive the scouting-map portrait branch"
    );
}

#[test]
fn build_detail_tiles_pattern_iterates_over_num_detail_tiles() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().adventure_map.insets = Some(vec![sample_inset()]);

    env.exec(
        r#"
        local _, _, _, _, _, numDetailTiles = C_AdventureMap.GetMapInsetInfo(1)
        _G.__tile_ids = {}
        for i = 1, numDetailTiles do
            _G.__tile_ids[i] = C_AdventureMap.GetMapInsetDetailTileInfo(1, i)
        end
        "#,
    )
    .unwrap();

    let count: f64 = env.eval("return #_G.__tile_ids").unwrap();
    let first: f64 = env.eval("return _G.__tile_ids[1]").unwrap();
    let last: f64 = env.eval("return _G.__tile_ids[8]").unwrap();
    assert!((count - 8.0).abs() < 1e-6);
    assert!((first - 1_001.0).abs() < 1e-6);
    assert!((last - 1_008.0).abs() < 1e-6);
}
