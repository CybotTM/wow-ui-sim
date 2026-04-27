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

use wow_ui_sim::lua_api::{
    AdventureMapInset, AdventureMapQuestInfo, AdventureMapQuestOffer, AdventureMapQuestPortrait,
    AdventureMapZoneChoice, WowLuaEnv,
};

#[test]
fn c_adventure_map_namespace_is_a_table() {
    let env = WowLuaEnv::new().expect("env");
    let kind: String = env.eval("return type(C_AdventureMap)").unwrap();
    assert_eq!(kind, "table");
}

#[test]
fn get_map_id_is_a_function() {
    let env = WowLuaEnv::new().expect("env");
    let kind: String = env
        .eval("return type(C_AdventureMap.GetMapID)")
        .unwrap();
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

    let kind: String = env
        .eval("return type(C_AdventureMap.GetMapID())")
        .unwrap();
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

    let count: f64 = env
        .eval("return C_AdventureMap.GetNumMapInsets()")
        .unwrap();
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

    let count: f64 = env
        .eval("return C_AdventureMap.GetNumMapInsets()")
        .unwrap();
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
        detail_tiles: vec![
            1_001, 1_002, 1_003, 1_004, 1_005, 1_006, 1_007, 1_008,
        ],
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
    assert_eq!(zone_description, "Reclaim the lost magic of the night elves.");
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
    env.state().borrow_mut().adventure_map.zone_choices =
        vec![sample_zone_choice(), second];

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
    env.state().borrow_mut().adventure_map.zone_choices =
        vec![sample_zone_choice(), second];

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

fn sample_quest_offer() -> AdventureMapQuestOffer {
    AdventureMapQuestOffer {
        quest_id: 41_653,
        is_trivial: false,
        frequency: 1,
        is_legendary: false,
        title: "The Tidestone of Golganneth".to_string(),
        description: "Recover the Pillar of Creation.".to_string(),
        normalized_x: 0.55,
        normalized_y: 0.62,
        inset_index: None,
    }
}

#[test]
fn get_num_quest_offers_is_a_function() {
    let env = WowLuaEnv::new().expect("env");
    let kind: String = env
        .eval("return type(C_AdventureMap.GetNumQuestOffers)")
        .unwrap();
    assert_eq!(kind, "function");
}

#[test]
fn get_num_quest_offers_defaults_to_zero() {
    let env = WowLuaEnv::new().expect("env");
    let count: f64 = env
        .eval("return C_AdventureMap.GetNumQuestOffers()")
        .unwrap();
    assert!(
        count.abs() < 1e-6,
        "GetNumQuestOffers must return 0 (not nil) before any offer is published"
    );
}

#[test]
fn get_num_quest_offers_returns_a_number_type() {
    let env = WowLuaEnv::new().expect("env");
    let kind: String = env
        .eval("return type(C_AdventureMap.GetNumQuestOffers())")
        .unwrap();
    assert_eq!(kind, "number");
}

#[test]
fn get_num_quest_offers_returns_seeded_length() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().adventure_map.quest_offers = vec![
        sample_quest_offer(),
        sample_quest_offer(),
        sample_quest_offer(),
    ];

    let count: f64 = env
        .eval("return C_AdventureMap.GetNumQuestOffers()")
        .unwrap();
    assert!((count - 3.0).abs() < 1e-6);
}

#[test]
fn quest_offer_data_provider_pattern_iterates_each_offer() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().adventure_map.quest_offers =
        vec![sample_quest_offer(), sample_quest_offer()];

    env.exec(
        r#"
        _G.__offer_count = 0
        for offerIndex = 1, C_AdventureMap.GetNumQuestOffers() do
            _G.__offer_count = _G.__offer_count + 1
        end
        "#,
    )
    .unwrap();

    let visited: f64 = env.eval("return _G.__offer_count").unwrap();
    assert!(
        (visited - 2.0).abs() < 1e-6,
        "AM_QuestOfferDataProvider:RefreshAllData loop must iterate the seeded count"
    );
}

#[test]
fn get_quest_offer_info_is_a_function() {
    let env = WowLuaEnv::new().expect("env");
    let kind: String = env
        .eval("return type(C_AdventureMap.GetQuestOfferInfo)")
        .unwrap();
    assert_eq!(kind, "function");
}

#[test]
fn get_quest_offer_info_returns_no_values_when_unloaded() {
    let env = WowLuaEnv::new().expect("env");
    let count: f64 = env
        .eval("return select('#', C_AdventureMap.GetQuestOfferInfo(1))")
        .unwrap();
    assert!(count.abs() < 1e-6);
}

#[test]
fn get_quest_offer_info_returns_no_values_for_out_of_range_index() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().adventure_map.quest_offers = vec![sample_quest_offer()];

    let count: f64 = env
        .eval("return select('#', C_AdventureMap.GetQuestOfferInfo(2))")
        .unwrap();
    assert!(count.abs() < 1e-6);
}

#[test]
fn get_quest_offer_info_returns_no_values_for_non_positive_index() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().adventure_map.quest_offers = vec![sample_quest_offer()];

    let zero_count: f64 = env
        .eval("return select('#', C_AdventureMap.GetQuestOfferInfo(0))")
        .unwrap();
    let negative_count: f64 = env
        .eval("return select('#', C_AdventureMap.GetQuestOfferInfo(-1))")
        .unwrap();
    assert!(zero_count.abs() < 1e-6);
    assert!(negative_count.abs() < 1e-6);
}

#[test]
fn get_quest_offer_info_returns_nine_descriptor_values() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().adventure_map.quest_offers = vec![sample_quest_offer()];

    env.exec(
        "questID, isTrivial, frequency, isLegendary, title, description, \
         normalizedX, normalizedY, insetIndex = C_AdventureMap.GetQuestOfferInfo(1)",
    )
    .unwrap();

    let arity: f64 = env
        .eval("return select('#', C_AdventureMap.GetQuestOfferInfo(1))")
        .unwrap();
    let quest_id: f64 = env.eval("return questID").unwrap();
    let is_trivial: bool = env.eval("return isTrivial").unwrap();
    let frequency: f64 = env.eval("return frequency").unwrap();
    let is_legendary: bool = env.eval("return isLegendary").unwrap();
    let title: String = env.eval("return title").unwrap();
    let description: String = env.eval("return description").unwrap();
    let normalized_x: f64 = env.eval("return normalizedX").unwrap();
    let normalized_y: f64 = env.eval("return normalizedY").unwrap();
    let inset_is_nil: bool = env.eval("return insetIndex == nil").unwrap();

    assert!((arity - 9.0).abs() < 1e-6);
    assert!((quest_id - 41_653.0).abs() < 1e-6);
    assert!(!is_trivial);
    assert!((frequency - 1.0).abs() < 1e-6);
    assert!(!is_legendary);
    assert_eq!(title, "The Tidestone of Golganneth");
    assert_eq!(description, "Recover the Pillar of Creation.");
    assert!((normalized_x - 0.55).abs() < 1e-6);
    assert!((normalized_y - 0.62).abs() < 1e-6);
    assert!(
        inset_is_nil,
        "insetIndex must be nil when offer.inset_index is None so the canvas pin path runs"
    );
}

#[test]
fn get_quest_offer_info_returns_inset_index_when_set() {
    let env = WowLuaEnv::new().expect("env");
    let mut offer = sample_quest_offer();
    offer.inset_index = Some(2);
    env.state().borrow_mut().adventure_map.quest_offers = vec![offer];

    let inset: f64 = env
        .eval(
            "local _, _, _, _, _, _, _, _, insetIndex = C_AdventureMap.GetQuestOfferInfo(1) \
             return insetIndex",
        )
        .unwrap();
    assert!((inset - 2.0).abs() < 1e-6);
}

#[test]
fn get_quest_offer_info_propagates_trivial_and_legendary_flags() {
    let env = WowLuaEnv::new().expect("env");
    let mut trivial = sample_quest_offer();
    trivial.is_trivial = true;
    let mut legendary = sample_quest_offer();
    legendary.is_legendary = true;
    env.state().borrow_mut().adventure_map.quest_offers = vec![trivial, legendary];

    let first_trivial: bool = env
        .eval("local _, t = C_AdventureMap.GetQuestOfferInfo(1) return t")
        .unwrap();
    let second_legendary: bool = env
        .eval("local _, _, _, l = C_AdventureMap.GetQuestOfferInfo(2) return l")
        .unwrap();
    assert!(first_trivial);
    assert!(second_legendary);
}

#[test]
fn get_quest_offer_info_indexes_one_based() {
    let env = WowLuaEnv::new().expect("env");
    let mut second = sample_quest_offer();
    second.quest_id = 41_654;
    second.title = "Stormheim".to_string();
    env.state().borrow_mut().adventure_map.quest_offers =
        vec![sample_quest_offer(), second];

    let first_id: f64 = env
        .eval("local id = C_AdventureMap.GetQuestOfferInfo(1) return id")
        .unwrap();
    let second_id: f64 = env
        .eval("local id = C_AdventureMap.GetQuestOfferInfo(2) return id")
        .unwrap();
    assert!((first_id - 41_653.0).abs() < 1e-6);
    assert!((second_id - 41_654.0).abs() < 1e-6);
}

#[test]
fn quest_offer_data_provider_pattern_collects_each_offer() {
    let env = WowLuaEnv::new().expect("env");
    let mut second = sample_quest_offer();
    second.quest_id = 41_654;
    env.state().borrow_mut().adventure_map.quest_offers =
        vec![sample_quest_offer(), second];

    env.exec(
        r#"
        _G.__offer_ids = {}
        for offerIndex = 1, C_AdventureMap.GetNumQuestOffers() do
            local questID = C_AdventureMap.GetQuestOfferInfo(offerIndex)
            _G.__offer_ids[offerIndex] = questID
        end
        "#,
    )
    .unwrap();

    let count: f64 = env.eval("return #_G.__offer_ids").unwrap();
    let first: f64 = env.eval("return _G.__offer_ids[1]").unwrap();
    let second_id: f64 = env.eval("return _G.__offer_ids[2]").unwrap();
    assert!((count - 2.0).abs() < 1e-6);
    assert!((first - 41_653.0).abs() < 1e-6);
    assert!((second_id - 41_654.0).abs() < 1e-6);
}

fn sample_quest_info() -> AdventureMapQuestInfo {
    AdventureMapQuestInfo {
        title: "Curse of the Drowned".to_string(),
        description: "Investigate the source of the curse plaguing Azsuna.".to_string(),
        objective_text: "Cleanse 5 Drowned Souls.".to_string(),
    }
}

#[test]
fn get_quest_info_is_a_function() {
    let env = WowLuaEnv::new().expect("env");
    let kind: String = env
        .eval("return type(C_AdventureMap.GetQuestInfo)")
        .unwrap();
    assert_eq!(kind, "function");
}

#[test]
fn get_quest_info_returns_no_values_for_unknown_quest() {
    let env = WowLuaEnv::new().expect("env");
    let count: f64 = env
        .eval("return select('#', C_AdventureMap.GetQuestInfo(99999))")
        .unwrap();
    assert!(
        count.abs() < 1e-6,
        "GetQuestInfo must return zero values for unknown quests so the dialog's \
         `if descriptionText then` guard short-circuits"
    );
}

#[test]
fn get_quest_info_returns_no_values_for_non_numeric_argument() {
    let env = WowLuaEnv::new().expect("env");
    let count: f64 = env
        .eval("return select('#', C_AdventureMap.GetQuestInfo('not-a-number'))")
        .unwrap();
    assert!(count.abs() < 1e-6);
}

#[test]
fn get_quest_info_returns_three_descriptor_strings() {
    let env = WowLuaEnv::new().expect("env");
    env.state()
        .borrow_mut()
        .adventure_map
        .quest_info
        .insert(40_519, sample_quest_info());

    env.exec(
        "questTitle, descriptionText, objectiveText = C_AdventureMap.GetQuestInfo(40519)",
    )
    .unwrap();

    let arity: f64 = env
        .eval("return select('#', C_AdventureMap.GetQuestInfo(40519))")
        .unwrap();
    let title: String = env.eval("return questTitle").unwrap();
    let description: String = env.eval("return descriptionText").unwrap();
    let objective: String = env.eval("return objectiveText").unwrap();

    assert!((arity - 3.0).abs() < 1e-6);
    assert_eq!(title, "Curse of the Drowned");
    assert_eq!(description, "Investigate the source of the curse plaguing Azsuna.");
    assert_eq!(objective, "Cleanse 5 Drowned Souls.");
}

#[test]
fn get_quest_info_keys_off_quest_id() {
    let env = WowLuaEnv::new().expect("env");
    let mut other = sample_quest_info();
    other.title = "Highmountain Tribes".to_string();
    let mut state = env.state().borrow_mut();
    state.adventure_map.quest_info.insert(40_519, sample_quest_info());
    state.adventure_map.quest_info.insert(40_521, other);
    drop(state);

    let first_title: String = env
        .eval("local t = C_AdventureMap.GetQuestInfo(40519) return t")
        .unwrap();
    let second_title: String = env
        .eval("local t = C_AdventureMap.GetQuestInfo(40521) return t")
        .unwrap();
    assert_eq!(first_title, "Curse of the Drowned");
    assert_eq!(second_title, "Highmountain Tribes");
}

#[test]
fn refresh_details_pattern_short_circuits_on_unknown_quest() {
    let env = WowLuaEnv::new().expect("env");
    env.exec(
        r#"
        _G.__rendered = false
        local _, descriptionText = C_AdventureMap.GetQuestInfo(123456)
        if descriptionText then
            _G.__rendered = true
        end
        "#,
    )
    .unwrap();

    let rendered: bool = env.eval("return _G.__rendered").unwrap();
    assert!(
        !rendered,
        "RefreshDetails-style guard must skip the body when descriptionText is nil"
    );
}

#[test]
fn refresh_details_pattern_renders_known_quest() {
    let env = WowLuaEnv::new().expect("env");
    env.state()
        .borrow_mut()
        .adventure_map
        .quest_info
        .insert(40_519, sample_quest_info());

    env.exec(
        r#"
        _G.__title_seen = nil
        local questTitle, descriptionText, objectiveText = C_AdventureMap.GetQuestInfo(40519)
        if descriptionText then
            _G.__title_seen = questTitle
        end
        "#,
    )
    .unwrap();

    let title: String = env.eval("return _G.__title_seen").unwrap();
    assert_eq!(title, "Curse of the Drowned");
}

fn sample_quest_portrait() -> AdventureMapQuestPortrait {
    AdventureMapQuestPortrait {
        portrait_display_id: 50_523,
        mount_portrait_display_id: 0,
        model_scene_id: Some(33),
        text: "The tides themselves cry out for justice.".to_string(),
        name: "Lady Hyrja".to_string(),
    }
}

#[test]
fn get_quest_portrait_info_is_a_function() {
    let env = WowLuaEnv::new().expect("env");
    let kind: String = env
        .eval("return type(C_AdventureMap.GetQuestPortraitInfo)")
        .unwrap();
    assert_eq!(kind, "function");
}

#[test]
fn get_quest_portrait_info_returns_no_values_for_unknown_quest() {
    let env = WowLuaEnv::new().expect("env");
    let count: f64 = env
        .eval("return select('#', C_AdventureMap.GetQuestPortraitInfo(99999))")
        .unwrap();
    assert!(
        count.abs() < 1e-6,
        "GetQuestPortraitInfo must return zero values for unknown quests so the \
         dialog's `if portraitInfo and ...` guard short-circuits"
    );
}

#[test]
fn get_quest_portrait_info_returns_no_values_for_non_numeric_argument() {
    let env = WowLuaEnv::new().expect("env");
    let count: f64 = env
        .eval("return select('#', C_AdventureMap.GetQuestPortraitInfo('not-a-number'))")
        .unwrap();
    assert!(count.abs() < 1e-6);
}

#[test]
fn get_quest_portrait_info_returns_a_table() {
    let env = WowLuaEnv::new().expect("env");
    env.state()
        .borrow_mut()
        .adventure_map
        .quest_portraits
        .insert(40_519, sample_quest_portrait());

    let kind: String = env
        .eval("return type(C_AdventureMap.GetQuestPortraitInfo(40519))")
        .unwrap();
    assert_eq!(kind, "table");
}

#[test]
fn get_quest_portrait_info_populates_documented_fields() {
    let env = WowLuaEnv::new().expect("env");
    env.state()
        .borrow_mut()
        .adventure_map
        .quest_portraits
        .insert(40_519, sample_quest_portrait());

    env.exec("portraitInfo = C_AdventureMap.GetQuestPortraitInfo(40519)")
        .unwrap();

    let portrait_id: f64 = env.eval("return portraitInfo.portraitDisplayID").unwrap();
    let mount_id: f64 = env.eval("return portraitInfo.mountPortraitDisplayID").unwrap();
    let scene_id: f64 = env.eval("return portraitInfo.modelSceneID").unwrap();
    let text: String = env.eval("return portraitInfo.text").unwrap();
    let name: String = env.eval("return portraitInfo.name").unwrap();

    assert!((portrait_id - 50_523.0).abs() < 1e-6);
    assert!((mount_id - 0.0).abs() < 1e-6);
    assert!((scene_id - 33.0).abs() < 1e-6);
    assert_eq!(text, "The tides themselves cry out for justice.");
    assert_eq!(name, "Lady Hyrja");
}

#[test]
fn get_quest_portrait_info_returns_nil_model_scene_when_unset() {
    let env = WowLuaEnv::new().expect("env");
    let mut portrait = sample_quest_portrait();
    portrait.model_scene_id = None;
    env.state()
        .borrow_mut()
        .adventure_map
        .quest_portraits
        .insert(40_519, portrait);

    let scene_is_nil: bool = env
        .eval(
            "local p = C_AdventureMap.GetQuestPortraitInfo(40519) \
             return p.modelSceneID == nil",
        )
        .unwrap();
    assert!(
        scene_is_nil,
        "modelSceneID is documented Nilable; legacy display-id portraits should leave it unset"
    );
}

#[test]
fn refresh_portrait_pattern_skips_when_display_id_zero() {
    let env = WowLuaEnv::new().expect("env");
    let mut portrait = sample_quest_portrait();
    portrait.portrait_display_id = 0;
    env.state()
        .borrow_mut()
        .adventure_map
        .quest_portraits
        .insert(40_519, portrait);

    env.exec(
        r#"
        _G.__shown = false
        local portraitInfo = C_AdventureMap.GetQuestPortraitInfo(40519)
        if portraitInfo and portraitInfo.portraitDisplayID ~= 0 then
            _G.__shown = true
        end
        "#,
    )
    .unwrap();

    let shown: bool = env.eval("return _G.__shown").unwrap();
    assert!(
        !shown,
        "RefreshPortrait pattern must skip when portraitDisplayID is 0"
    );
}

#[test]
fn refresh_portrait_pattern_renders_when_display_id_nonzero() {
    let env = WowLuaEnv::new().expect("env");
    env.state()
        .borrow_mut()
        .adventure_map
        .quest_portraits
        .insert(40_519, sample_quest_portrait());

    env.exec(
        r#"
        _G.__name_seen = nil
        local portraitInfo = C_AdventureMap.GetQuestPortraitInfo(40519)
        if portraitInfo and portraitInfo.portraitDisplayID ~= 0 then
            _G.__name_seen = portraitInfo.name
        end
        "#,
    )
    .unwrap();

    let name: String = env.eval("return _G.__name_seen").unwrap();
    assert_eq!(name, "Lady Hyrja");
}

#[test]
fn refresh_portrait_pattern_short_circuits_on_unknown_quest() {
    let env = WowLuaEnv::new().expect("env");
    env.exec(
        r#"
        _G.__shown = false
        local portraitInfo = C_AdventureMap.GetQuestPortraitInfo(123456)
        if portraitInfo and portraitInfo.portraitDisplayID ~= 0 then
            _G.__shown = true
        end
        "#,
    )
    .unwrap();

    let shown: bool = env.eval("return _G.__shown").unwrap();
    assert!(
        !shown,
        "Missing quest portraits must yield nil so the `if portraitInfo and ...` guard skips"
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
