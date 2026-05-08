use super::sample_zone_choice;
use wow_ui_sim::lua_api::{
    AdventureMapQuestInfo, AdventureMapQuestOffer, AdventureMapQuestPortrait, WowLuaEnv,
};

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
    env.state().borrow_mut().adventure_map.quest_offers = vec![sample_quest_offer(), second];

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
    env.state().borrow_mut().adventure_map.quest_offers = vec![sample_quest_offer(), second];

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

    env.exec("questTitle, descriptionText, objectiveText = C_AdventureMap.GetQuestInfo(40519)")
        .unwrap();

    let arity: f64 = env
        .eval("return select('#', C_AdventureMap.GetQuestInfo(40519))")
        .unwrap();
    let title: String = env.eval("return questTitle").unwrap();
    let description: String = env.eval("return descriptionText").unwrap();
    let objective: String = env.eval("return objectiveText").unwrap();

    assert!((arity - 3.0).abs() < 1e-6);
    assert_eq!(title, "Curse of the Drowned");
    assert_eq!(
        description,
        "Investigate the source of the curse plaguing Azsuna."
    );
    assert_eq!(objective, "Cleanse 5 Drowned Souls.");
}

#[test]
fn get_quest_info_keys_off_quest_id() {
    let env = WowLuaEnv::new().expect("env");
    let mut other = sample_quest_info();
    other.title = "Highmountain Tribes".to_string();
    let mut state = env.state().borrow_mut();
    state
        .adventure_map
        .quest_info
        .insert(40_519, sample_quest_info());
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
    let mount_id: f64 = env
        .eval("return portraitInfo.mountPortraitDisplayID")
        .unwrap();
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
fn start_quest_is_a_function() {
    let env = WowLuaEnv::new().expect("env");
    let kind: String = env.eval("return type(C_AdventureMap.StartQuest)").unwrap();
    assert_eq!(kind, "function");
}

#[test]
fn start_quest_returns_no_values() {
    let env = WowLuaEnv::new().expect("env");
    let nothing: bool = env
        .eval("return select('#', C_AdventureMap.StartQuest(40519)) == 0")
        .unwrap();
    assert!(nothing, "StartQuest should return zero values");
}

#[test]
fn start_quest_appends_to_quest_log() {
    let env = WowLuaEnv::new().expect("env");
    assert!(
        !env.state().borrow().quest_log.contains(&40_519),
        "quest_log should not contain the quest before StartQuest"
    );

    env.exec("C_AdventureMap.StartQuest(40519)").unwrap();

    assert!(
        env.state().borrow().quest_log.contains(&40_519),
        "StartQuest must append the quest id to state.quest_log"
    );
}

#[test]
fn start_quest_does_not_duplicate_existing_log_entry() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().quest_log = vec![40_519];

    env.exec("C_AdventureMap.StartQuest(40519)").unwrap();

    let count = env
        .state()
        .borrow()
        .quest_log
        .iter()
        .filter(|id| **id == 40_519)
        .count();
    assert_eq!(
        count, 1,
        "StartQuest must not duplicate an existing log entry"
    );
}

#[test]
fn start_quest_removes_matching_offer_pin() {
    let env = WowLuaEnv::new().expect("env");
    let mut accepted = sample_quest_offer();
    accepted.quest_id = 40_519;
    let mut other = sample_quest_offer();
    other.quest_id = 40_520;
    env.state().borrow_mut().adventure_map.quest_offers = vec![accepted, other];

    env.exec("C_AdventureMap.StartQuest(40519)").unwrap();

    let remaining_ids: Vec<i64> = env
        .state()
        .borrow()
        .adventure_map
        .quest_offers
        .iter()
        .map(|offer| offer.quest_id)
        .collect();
    assert_eq!(
        remaining_ids,
        vec![40_520],
        "StartQuest must remove the accepted offer and leave others intact"
    );
}

#[test]
fn start_quest_removes_matching_zone_choice() {
    let env = WowLuaEnv::new().expect("env");
    let mut chosen = sample_zone_choice();
    chosen.quest_id = 40_519;
    let mut other = sample_zone_choice();
    other.quest_id = 40_521;
    env.state().borrow_mut().adventure_map.zone_choices = vec![chosen, other];

    env.exec("C_AdventureMap.StartQuest(40519)").unwrap();

    let remaining_ids: Vec<i64> = env
        .state()
        .borrow()
        .adventure_map
        .zone_choices
        .iter()
        .map(|choice| choice.quest_id)
        .collect();
    assert_eq!(
        remaining_ids,
        vec![40_521],
        "StartQuest must remove the accepted zone choice and leave others intact"
    );
}

#[test]
fn start_quest_queues_quest_accepted_event() {
    let env = WowLuaEnv::new().expect("env");
    env.exec("C_AdventureMap.StartQuest(40519)").unwrap();

    let st = env.state().borrow();
    let event = st
        .events
        .pending()
        .iter()
        .find(|e| e.name == "QUEST_ACCEPTED")
        .expect("StartQuest must queue QUEST_ACCEPTED");
    let payload = event
        .args
        .first()
        .expect("QUEST_ACCEPTED must carry the questID payload");
    let id = match payload {
        wow_ui_sim::event::EventArg::Number(n) => *n,
        other => panic!("expected questID number, got {other:?}"),
    };
    assert!(
        (id - 40_519.0).abs() < 1e-6,
        "QUEST_ACCEPTED payload must be the accepted questID"
    );
}

#[test]
fn start_quest_short_circuits_on_non_numeric_argument() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().adventure_map.quest_offers = vec![sample_quest_offer()];

    env.exec("C_AdventureMap.StartQuest('not-a-number')")
        .unwrap();

    assert_eq!(
        env.state().borrow().adventure_map.quest_offers.len(),
        1,
        "Non-numeric arg must leave offers untouched"
    );
    assert!(
        env.state().borrow().quest_log.is_empty(),
        "Non-numeric arg must leave the quest log untouched"
    );
}
