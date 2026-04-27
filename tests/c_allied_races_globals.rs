//! Integration tests for the `C_AlliedRaces` surface registered in
//! `src/c_api/c_allied_races.rs`. Drives `Blizzard_AlliedRacesUI`'s
//! `LoadRaceData` path which expects every canonical allied race to
//! resolve and the returned `bannerColor` to expose `ColorMixin:GetRGB`.

use wow_ui_sim::lua_api::{AlliedRaceInfo, WowLuaEnv};

const CANONICAL_RACE_FILE_STRINGS: &[&str] = &[
    "lightforgeddraenei",
    "darkirondwarf",
    "voidelf",
    "mechagnome",
    "vulpera",
    "zandalaritroll",
    "highmountaintauren",
    "nightborne",
    "magharorc",
    "earthendwarf",
];

fn race_id_for(file_string: &str) -> i64 {
    let env = WowLuaEnv::new().expect("env");
    let state = env.state().borrow();
    state
        .allied_races
        .values()
        .find(|info| info.race_file_string == file_string)
        .unwrap_or_else(|| panic!("missing canonical race {file_string}"))
        .race_id
}

#[test]
fn unknown_race_id_returns_nil() {
    let env = WowLuaEnv::new().expect("env");
    let nil: bool = env
        .eval("return C_AlliedRaces.GetRaceInfoByID(99999) == nil")
        .unwrap();
    assert!(nil, "GetRaceInfoByID should return nil for unknown ids");
}

#[test]
fn missing_arg_returns_nothing() {
    let env = WowLuaEnv::new().expect("env");
    let count: f64 = env
        .eval("return select('#', C_AlliedRaces.GetRaceInfoByID())")
        .unwrap();
    assert_eq!(count, 0.0, "GetRaceInfoByID() with no args returns nothing");
}

#[test]
fn canonical_race_ids_are_seeded() {
    let env = WowLuaEnv::new().expect("env");
    let count = env.state().borrow().allied_races.len();
    assert_eq!(count, 10, "expected 10 canonical allied races to be seeded");
    for file_string in CANONICAL_RACE_FILE_STRINGS {
        let race_id = race_id_for(file_string);
        let resolved: bool = env
            .eval(&format!(
                "return C_AlliedRaces.GetRaceInfoByID({race_id}) ~= nil"
            ))
            .unwrap();
        assert!(
            resolved,
            "GetRaceInfoByID({race_id}) [{file_string}] should resolve"
        );
    }
}

#[test]
fn race_info_table_exposes_documented_fields() {
    let env = WowLuaEnv::new().expect("env");
    let lightforged_id = race_id_for("lightforgeddraenei");
    env.exec(&format!(
        "info = C_AlliedRaces.GetRaceInfoByID({lightforged_id})"
    ))
    .unwrap();

    let race_id: f64 = env.eval("return info.raceID").unwrap();
    let male_model_id: f64 = env.eval("return info.maleModelID").unwrap();
    let female_model_id: f64 = env.eval("return info.femaleModelID").unwrap();
    let male_name: String = env.eval("return info.maleName").unwrap();
    let female_name: String = env.eval("return info.femaleName").unwrap();
    let description: String = env.eval("return info.description").unwrap();
    let race_file_string: String = env.eval("return info.raceFileString").unwrap();
    let crest_atlas: String = env.eval("return info.crestAtlas").unwrap();
    let model_background_atlas: String = env.eval("return info.modelBackgroundAtlas").unwrap();
    let achievement_count: f64 = env.eval("return #info.achievementIds").unwrap();

    assert_eq!(race_id, lightforged_id as f64);
    assert_eq!(male_model_id, 82_729.0);
    assert_eq!(female_model_id, 82_730.0);
    assert_eq!(male_name, "Lightforged Draenei");
    assert_eq!(female_name, "Lightforged Draenei");
    assert_eq!(race_file_string, "lightforgeddraenei");
    assert_eq!(crest_atlas, "alliedraces-icon-lightforgeddraenei");
    assert_eq!(
        model_background_atlas,
        "alliedraces-background-lightforgeddraenei"
    );
    assert!(!description.is_empty());
    assert!(achievement_count >= 1.0);
}

#[test]
fn banner_color_supports_color_mixin_get_rgb() {
    let env = WowLuaEnv::new().expect("env");
    let nightborne_id = race_id_for("nightborne");
    env.exec(&format!(
        "info = C_AlliedRaces.GetRaceInfoByID({nightborne_id})"
    ))
    .unwrap();

    let r: f64 = env
        .eval("local r,_,_ = info.bannerColor:GetRGB() return r")
        .unwrap();
    let g: f64 = env
        .eval("local _,g,_ = info.bannerColor:GetRGB() return g")
        .unwrap();
    let b: f64 = env
        .eval("local _,_,b = info.bannerColor:GetRGB() return b")
        .unwrap();
    assert!((r - 0.62).abs() < 1e-3);
    assert!((g - 0.39).abs() < 1e-3);
    assert!((b - 0.85).abs() < 1e-3);
}

#[test]
fn race_info_reflects_state_mutations() {
    let env = WowLuaEnv::new().expect("env");
    env.state().borrow_mut().allied_races.insert(
        9999,
        AlliedRaceInfo {
            race_id: 9999,
            male_model_id: 1,
            female_model_id: 2,
            achievement_ids: vec![100, 200, 300],
            male_name: "TestRaceMale".to_string(),
            female_name: "TestRaceFemale".to_string(),
            description: "A test race".to_string(),
            race_file_string: "testrace".to_string(),
            crest_atlas: "test-crest".to_string(),
            model_background_atlas: "test-background".to_string(),
            banner_color: (0.5, 0.25, 0.75),
        },
    );

    let male_name: String = env
        .eval("return C_AlliedRaces.GetRaceInfoByID(9999).maleName")
        .unwrap();
    assert_eq!(male_name, "TestRaceMale");

    let third_achievement: f64 = env
        .eval("return C_AlliedRaces.GetRaceInfoByID(9999).achievementIds[3]")
        .unwrap();
    assert_eq!(third_achievement, 300.0);
}

#[test]
fn allied_races_load_data_path_runs_without_errors() {
    let env = WowLuaEnv::new().expect("env");
    let voidelf_id = race_id_for("voidelf");
    env.exec(&format!(
        r#"
        local raceInfo = C_AlliedRaces.GetRaceInfoByID({voidelf_id})
        assert(raceInfo, "raceInfo should not be nil")
        local r, g, b = raceInfo.bannerColor:GetRGB()
        assert(type(r) == "number" and type(g) == "number" and type(b) == "number",
            "bannerColor:GetRGB should return three numbers")
        assert(type(raceInfo.maleModelID) == "number")
        assert(type(raceInfo.femaleModelID) == "number")
        assert(type(raceInfo.achievementIds) == "table")
        assert(type(raceInfo.crestAtlas) == "string")
        assert(type(raceInfo.modelBackgroundAtlas) == "string")
        ALLIED_RACES_LOAD_DATA_OK = true
    "#
    ))
    .unwrap();

    let ok: bool = env
        .eval("return ALLIED_RACES_LOAD_DATA_OK == true")
        .unwrap();
    assert!(
        ok,
        "AlliedRacesFrameMixin:LoadRaceData-style probe should succeed"
    );
}
