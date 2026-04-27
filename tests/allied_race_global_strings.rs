//! Integration tests for the allied-races global string surface.
//!
//! Drives `Blizzard_AlliedRacesFrameUI.lua:92` (`SetRaceNameForGender`
//! reads `_G["RACE_INFO_"..fileString]`) and the inline XML string
//! references at `Blizzard_AlliedRacesFrameUI.xml:145` (`text="RACIAL_TRAITS"`)
//! and `:163` (`KeyValue title=ALLIED_RACE_UNLOCK_TEXT`). The simulator
//! resolves these via `data/global_strings.rs` (auto-generated from
//! `GlobalStrings.csv`); this test pins the contract so a future
//! regeneration that drops them is caught immediately.

use wow_ui_sim::lua_api::WowLuaEnv;

const ALLIED_RACE_FILE_STRINGS: &[&str] = &[
    "LIGHTFORGEDDRAENEI",
    "DARKIRONDWARF",
    "VOIDELF",
    "MECHAGNOME",
    "VULPERA",
    "ZANDALARITROLL",
    "HIGHMOUNTAINTAUREN",
    "NIGHTBORNE",
    "MAGHARORC",
    "EARTHENDWARF",
];

#[test]
fn racial_traits_string_global_resolves_to_canonical_value() {
    let env = WowLuaEnv::new().expect("env");
    let value: String = env.eval("return RACIAL_TRAITS").unwrap();
    assert_eq!(
        value, "Racial Traits:",
        "RACIAL_TRAITS is referenced verbatim in Blizzard_AlliedRacesFrameUI.xml:145"
    );
}

#[test]
fn allied_race_unlock_text_string_global_resolves_to_canonical_value() {
    let env = WowLuaEnv::new().expect("env");
    let value: String = env.eval("return ALLIED_RACE_UNLOCK_TEXT").unwrap();
    assert_eq!(
        value, "To unlock this race:",
        "ALLIED_RACE_UNLOCK_TEXT is the KeyValue title for the unlock objectives panel"
    );
}

#[test]
fn race_info_globals_resolve_for_every_allied_race() {
    let env = WowLuaEnv::new().expect("env");
    for file_string in ALLIED_RACE_FILE_STRINGS {
        let global_name = format!("RACE_INFO_{file_string}");
        let kind: String = env
            .eval(&format!("return type(_G[\"{global_name}\"])"))
            .unwrap();
        assert_eq!(
            kind, "string",
            "{global_name} must resolve so SetRaceNameForGender can render the description"
        );
        let value: String = env.eval(&format!("return _G[\"{global_name}\"]")).unwrap();
        assert!(
            !value.is_empty(),
            "{global_name} must be non-empty — empty descriptions render as blank cards"
        );
    }
}

#[test]
fn set_race_name_for_gender_lookup_pattern_resolves() {
    // Mirrors the runtime pattern at AlliedRacesFrameUI.lua:92:
    //     descriptionString = _G["RACE_INFO_"..fileString]
    let env = WowLuaEnv::new().expect("env");
    let resolved: String = env
        .eval(
            r#"
            local fileString = "VOIDELF"
            return _G["RACE_INFO_" .. fileString]
            "#,
        )
        .unwrap();
    assert!(
        resolved.contains("Void"),
        "the dynamic _G[\"RACE_INFO_\"..fileString] lookup must hit the same string the addon expects"
    );
}
