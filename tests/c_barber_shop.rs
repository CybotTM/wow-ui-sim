//! Integration tests for the `C_BarberShop` surface registered in
//! `src/c_api/c_barber_shop.rs`. Exercises every method
//! `Blizzard_BarbershopUI` calls and verifies the event dispatch
//! contract for each side-effect.

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::{
    BarberShopAlternateFormRace, BarberShopCategory, BarberShopCharacterData, BarberShopOption,
};

const FEATURE_FLAG_MOUNTS: i32 = 16;
const UNIT_SEX_MALE: i32 = 0;
const UNIT_SEX_FEMALE: i32 = 1;

fn seeded_character() -> BarberShopCharacterData {
    BarberShopCharacterData {
        name: "Snorlax".to_string(),
        file_name: "snorlax".to_string(),
        alternate_form_race: Some(BarberShopAlternateFormRace {
            race_id: 22,
            name: "Worgen".to_string(),
            file_name: "worgen".to_string(),
            create_screen_icon_atlas: "raceicon-worgen-male".to_string(),
        }),
        create_screen_icon_atlas: "raceicon-human-male".to_string(),
        sex: UNIT_SEX_MALE,
    }
}

fn seeded_categories() -> Vec<BarberShopCategory> {
    vec![BarberShopCategory {
        name: "Hair".to_string(),
        options: vec![
            BarberShopOption {
                option_id: 31,
                name: "Hair Style".to_string(),
                current_choice_id: Some(102),
            },
            BarberShopOption {
                option_id: 32,
                name: "Hair Color".to_string(),
                current_choice_id: None,
            },
        ],
    }]
}

#[test]
fn namespace_is_present() {
    let env = WowLuaEnv::new().expect("env");
    let ns_type: String = env.eval("return type(C_BarberShop)").unwrap();
    assert_eq!(ns_type, "table");
    let cancel_type: String = env.eval("return type(C_BarberShop.Cancel)").unwrap();
    assert_eq!(cancel_type, "function");
}

#[test]
fn has_customization_feature_defaults_false() {
    let env = WowLuaEnv::new().expect("env");
    let has_feature: bool = env
        .eval("return C_BarberShop.HasCustomizationFeature(16)")
        .unwrap();
    assert!(!has_feature);
}

#[test]
fn has_customization_feature_matches_bitflag() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().barber_shop.feature_flags = FEATURE_FLAG_MOUNTS | 1;
    let has_mounts: bool = env
        .eval("return C_BarberShop.HasCustomizationFeature(16)")
        .unwrap();
    let has_unset: bool = env
        .eval("return C_BarberShop.HasCustomizationFeature(2)")
        .unwrap();
    assert!(has_mounts);
    assert!(!has_unset);
}

#[test]
fn get_current_character_data_defaults_nil() {
    let env = WowLuaEnv::new().expect("env");
    let nil: bool = env
        .eval("return C_BarberShop.GetCurrentCharacterData() == nil")
        .unwrap();
    assert!(nil);
}

#[test]
fn get_current_character_data_returns_canonical_shape() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().barber_shop.current_character = Some(seeded_character());
    let (name, sex, alt_race_id, icon_atlas): (String, i32, i32, String) = env
        .eval(
            r#"
            local d = C_BarberShop.GetCurrentCharacterData()
            return d.name, d.sex, d.alternateFormRaceData.raceID, d.createScreenIconAtlas
            "#,
        )
        .unwrap();
    assert_eq!(name, "Snorlax");
    assert_eq!(sex, UNIT_SEX_MALE);
    assert_eq!(alt_race_id, 22);
    assert_eq!(icon_atlas, "raceicon-human-male");
}

#[test]
fn is_viewing_altered_form_reads_state() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().barber_shop.viewing_altered_form = true;
    let viewing: bool = env
        .eval("return C_BarberShop.IsViewingAlteredForm()")
        .unwrap();
    assert!(viewing);
}

#[test]
fn get_viewing_chr_model_returns_nil_when_unset() {
    let env = WowLuaEnv::new().expect("env");
    let nil: bool = env
        .eval("return C_BarberShop.GetViewingChrModel() == nil")
        .unwrap();
    assert!(nil);
}

#[test]
fn get_viewing_chr_model_returns_id_when_set() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().barber_shop.viewing_chr_model = Some(2107);
    let model_id: i32 = env.eval("return C_BarberShop.GetViewingChrModel()").unwrap();
    assert_eq!(model_id, 2107);
}

#[test]
fn cancel_fires_barber_shop_result_with_false() {
    let env = WowLuaEnv::new().expect("env");
    let success: bool = env
        .eval(
            r#"
            local got
            local f = CreateFrame("Frame")
            f:RegisterEvent("BARBER_SHOP_RESULT")
            f:SetScript("OnEvent", function(_, _, ok) got = ok end)
            C_BarberShop.Cancel()
            return got
            "#,
        )
        .unwrap();
    assert!(!success);
}

#[test]
fn reset_customization_choices_clears_state_and_fires_event() {
    let env = WowLuaEnv::new().expect("env");
    {
        let mut sim = env.state().borrow_mut();
        sim.barber_shop.choices.insert(31, 100);
        sim.barber_shop.preview_choices.insert(32, 200);
        sim.barber_shop.has_changes = true;
    }
    let event_count: i32 = env
        .eval(
            r#"
            local count = 0
            local f = CreateFrame("Frame")
            f:RegisterEvent("BARBER_SHOP_FORCE_CUSTOMIZATIONS_UPDATE")
            f:SetScript("OnEvent", function() count = count + 1 end)
            local force = false
            C_BarberShop.ResetCustomizationChoices(force)
            return count
            "#,
        )
        .unwrap();
    assert_eq!(event_count, 1);
    let sim = env.state().borrow();
    assert!(sim.barber_shop.choices.is_empty());
    assert!(sim.barber_shop.preview_choices.is_empty());
    assert!(!sim.barber_shop.has_changes);
}

#[test]
fn apply_customization_choices_folds_preview_and_fires_events() {
    let env = WowLuaEnv::new().expect("env");
    {
        let mut sim = env.state().borrow_mut();
        sim.barber_shop.preview_choices.insert(31, 555);
        sim.barber_shop.has_changes = true;
    }
    let (success, result_log, applied_count): (bool, bool, i32) = env
        .eval(
            r#"
            local result, applied = nil, 0
            local f = CreateFrame("Frame")
            f:RegisterEvent("BARBER_SHOP_RESULT")
            f:RegisterEvent("BARBER_SHOP_APPEARANCE_APPLIED")
            f:SetScript("OnEvent", function(_, event, ok)
                if event == "BARBER_SHOP_RESULT" then result = ok
                else applied = applied + 1 end
            end)
            local ret = C_BarberShop.ApplyCustomizationChoices()
            return ret, result, applied
            "#,
        )
        .unwrap();
    assert!(success);
    assert!(result_log);
    assert_eq!(applied_count, 1);
    let sim = env.state().borrow();
    assert_eq!(sim.barber_shop.choices.get(&31).copied(), Some(555));
    assert!(sim.barber_shop.preview_choices.is_empty());
    assert!(!sim.barber_shop.has_changes);
}

#[test]
fn has_any_changes_reads_state() {
    let env = WowLuaEnv::new().expect("env");
    let initial: bool = env.eval("return C_BarberShop.HasAnyChanges()").unwrap();
    assert!(!initial);
    env.state().borrow_mut().barber_shop.has_changes = true;
    let dirty: bool = env.eval("return C_BarberShop.HasAnyChanges()").unwrap();
    assert!(dirty);
}

#[test]
fn get_available_customizations_returns_nil_by_default() {
    let env = WowLuaEnv::new().expect("env");
    let nil: bool = env
        .eval("return C_BarberShop.GetAvailableCustomizations() == nil")
        .unwrap();
    assert!(nil);
}

#[test]
fn get_available_customizations_returns_seeded_categories() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().barber_shop.available_customizations = Some(seeded_categories());
    let (count, name, option_id, second_choice_nil): (i32, String, i32, bool) = env
        .eval(
            r#"
            local cats = C_BarberShop.GetAvailableCustomizations()
            return #cats, cats[1].name, cats[1].options[1].optionID, cats[1].options[2].currentChoice == nil
            "#,
        )
        .unwrap();
    assert_eq!(count, 1);
    assert_eq!(name, "Hair");
    assert_eq!(option_id, 31);
    assert!(second_choice_nil);
}

#[test]
fn set_customization_choice_writes_state_and_fires_cost_update() {
    let env = WowLuaEnv::new().expect("env");
    let event_count: i32 = env
        .eval(
            r#"
            local count = 0
            local f = CreateFrame("Frame")
            f:RegisterEvent("BARBER_SHOP_COST_UPDATE")
            f:SetScript("OnEvent", function() count = count + 1 end)
            C_BarberShop.SetCustomizationChoice(31, 700)
            return count
            "#,
        )
        .unwrap();
    assert_eq!(event_count, 1);
    let sim = env.state().borrow();
    assert_eq!(sim.barber_shop.choices.get(&31).copied(), Some(700));
    assert!(sim.barber_shop.has_changes);
}

#[test]
fn clear_preview_choices_drops_only_preview_when_flag_false() {
    let env = WowLuaEnv::new().expect("env");
    {
        let mut sim = env.state().borrow_mut();
        sim.barber_shop.choices.insert(1, 10);
        sim.barber_shop.preview_choices.insert(2, 20);
        sim.barber_shop.has_changes = true;
    }
    env.eval::<()>("C_BarberShop.ClearPreviewChoices(false)").unwrap();
    let sim = env.state().borrow();
    assert!(sim.barber_shop.preview_choices.is_empty());
    assert_eq!(sim.barber_shop.choices.get(&1).copied(), Some(10));
    assert!(sim.barber_shop.has_changes);
}

#[test]
fn clear_preview_choices_clears_saved_when_flag_true() {
    let env = WowLuaEnv::new().expect("env");
    {
        let mut sim = env.state().borrow_mut();
        sim.barber_shop.choices.insert(1, 10);
        sim.barber_shop.preview_choices.insert(2, 20);
        sim.barber_shop.has_changes = true;
    }
    env.eval::<()>("C_BarberShop.ClearPreviewChoices(true)").unwrap();
    let sim = env.state().borrow();
    assert!(sim.barber_shop.preview_choices.is_empty());
    assert!(sim.barber_shop.choices.is_empty());
    assert!(!sim.barber_shop.has_changes);
}

#[test]
fn preview_customization_choice_does_not_touch_saved_or_has_changes() {
    let env = WowLuaEnv::new().expect("env");
    env.eval::<()>("C_BarberShop.PreviewCustomizationChoice(40, 99)")
        .unwrap();
    let sim = env.state().borrow();
    assert_eq!(sim.barber_shop.preview_choices.get(&40).copied(), Some(99));
    assert!(sim.barber_shop.choices.is_empty());
    assert!(!sim.barber_shop.has_changes);
}

#[test]
fn mark_seen_helpers_record_into_state() {
    let env = WowLuaEnv::new().expect("env");
    env.eval::<()>(
        r#"
        C_BarberShop.MarkCustomizationChoiceAsSeen(101)
        C_BarberShop.MarkCustomizationChoiceAsSeen(102)
        C_BarberShop.MarkCustomizationOptionAsSeen(31)
        C_BarberShop.SaveSeenChoices()
        "#,
    )
    .unwrap();
    let sim = env.state().borrow();
    assert!(sim.barber_shop.seen_choices.contains(&101));
    assert!(sim.barber_shop.seen_choices.contains(&102));
    assert!(sim.barber_shop.seen_options.contains(&31));
}

#[test]
fn camera_zoom_round_trips_through_state() {
    let env = WowLuaEnv::new().expect("env");
    env.eval::<()>("C_BarberShop.SetCameraZoomLevel(3, false)").unwrap();
    let zoom: f64 = env.eval("return C_BarberShop.GetCurrentCameraZoom()").unwrap();
    assert!((zoom - 3.0).abs() < 1e-6);
    env.eval::<()>("C_BarberShop.ZoomCamera(0.5)").unwrap();
    let zoom_after: f64 = env.eval("return C_BarberShop.GetCurrentCameraZoom()").unwrap();
    assert!((zoom_after - 3.5).abs() < 1e-6);
}

#[test]
fn rotate_and_reset_camera_fire_camera_event() {
    let env = WowLuaEnv::new().expect("env");
    let count: i32 = env
        .eval(
            r#"
            local count = 0
            local f = CreateFrame("Frame")
            f:RegisterEvent("BARBER_SHOP_CAMERA_VALUES_UPDATED")
            f:SetScript("OnEvent", function() count = count + 1 end)
            C_BarberShop.RotateCamera(45)
            C_BarberShop.ResetCameraRotation()
            return count
            "#,
        )
        .unwrap();
    assert_eq!(count, 2);
}

#[test]
fn set_viewing_altered_form_writes_state_and_fires_force_update() {
    let env = WowLuaEnv::new().expect("env");
    let count: i32 = env
        .eval(
            r#"
            local count = 0
            local f = CreateFrame("Frame")
            f:RegisterEvent("BARBER_SHOP_FORCE_CUSTOMIZATIONS_UPDATE")
            f:SetScript("OnEvent", function() count = count + 1 end)
            C_BarberShop.SetViewingAlteredForm(true)
            return count
            "#,
        )
        .unwrap();
    assert_eq!(count, 1);
    assert!(env.state().borrow().barber_shop.viewing_altered_form);
}

#[test]
fn set_viewing_shapeshift_form_accepts_nil_and_id() {
    let env = WowLuaEnv::new().expect("env");
    env.eval::<()>("C_BarberShop.SetViewingShapeshiftForm(31)").unwrap();
    assert_eq!(
        env.state().borrow().barber_shop.viewing_shapeshift_form,
        Some(31)
    );
    env.eval::<()>("C_BarberShop.SetViewingShapeshiftForm(nil)").unwrap();
    assert!(
        env.state()
            .borrow()
            .barber_shop
            .viewing_shapeshift_form
            .is_none()
    );
}

#[test]
fn set_viewing_chr_model_writes_state_and_fires_camera_event() {
    let env = WowLuaEnv::new().expect("env");
    let count: i32 = env
        .eval(
            r#"
            local count = 0
            local f = CreateFrame("Frame")
            f:RegisterEvent("BARBER_SHOP_CAMERA_VALUES_UPDATED")
            f:SetScript("OnEvent", function() count = count + 1 end)
            C_BarberShop.SetViewingChrModel(900)
            return count
            "#,
        )
        .unwrap();
    assert_eq!(count, 1);
    assert_eq!(env.state().borrow().barber_shop.viewing_chr_model, Some(900));
}

#[test]
fn set_model_dress_state_writes_field() {
    let env = WowLuaEnv::new().expect("env");
    env.eval::<()>("C_BarberShop.SetModelDressState(true)").unwrap();
    assert!(env.state().borrow().barber_shop.model_dressed);
    env.eval::<()>("C_BarberShop.SetModelDressState(false)").unwrap();
    assert!(!env.state().borrow().barber_shop.model_dressed);
}

#[test]
fn set_camera_distance_offset_writes_field() {
    let env = WowLuaEnv::new().expect("env");
    env.eval::<()>("C_BarberShop.SetCameraDistanceOffset(1.25)").unwrap();
    let offset = env.state().borrow().barber_shop.camera_distance_offset;
    assert!((offset - 1.25).abs() < 1e-6);
}

#[test]
fn randomize_customization_choices_fires_force_update() {
    let env = WowLuaEnv::new().expect("env");
    let count: i32 = env
        .eval(
            r#"
            local count = 0
            local f = CreateFrame("Frame")
            f:RegisterEvent("BARBER_SHOP_FORCE_CUSTOMIZATIONS_UPDATE")
            f:SetScript("OnEvent", function() count = count + 1 end)
            C_BarberShop.RandomizeCustomizationChoices()
            return count
            "#,
        )
        .unwrap();
    assert_eq!(count, 1);
}

#[test]
fn set_selected_sex_updates_character_and_fires_camera_event() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().barber_shop.current_character = Some(seeded_character());
    let count: i32 = env
        .eval(
            r#"
            local count = 0
            local f = CreateFrame("Frame")
            f:RegisterEvent("BARBER_SHOP_CAMERA_VALUES_UPDATED")
            f:SetScript("OnEvent", function() count = count + 1 end)
            C_BarberShop.SetSelectedSex(1)
            return count
            "#,
        )
        .unwrap();
    assert_eq!(count, 1);
    let sim = env.state().borrow();
    let sex = sim.barber_shop.current_character.as_ref().unwrap().sex;
    assert_eq!(sex, UNIT_SEX_FEMALE);
}

#[test]
fn set_selected_sex_when_no_character_does_not_panic() {
    let env = WowLuaEnv::new().expect("env");
    env.eval::<()>("C_BarberShop.SetSelectedSex(1)").unwrap();
    assert!(env.state().borrow().barber_shop.current_character.is_none());
}
