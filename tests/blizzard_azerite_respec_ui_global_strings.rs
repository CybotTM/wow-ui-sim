#![cfg(feature = "client-retail")]
//! Integration tests for the `Blizzard_AzeriteRespecUI` global-string
//! surface.
//!
//! Drives the title `SetTitle(AZERITE_RESPEC_TITLE)` at
//! `Blizzard_AzeriteRespecUI.lua:7`, the reforge button label
//! (`AZERITE_RESPEC_BUTTON`), the empty-frame tutorial
//! (`AZERITE_RESPEC_TUTORIAL_TEXT`), and the four
//! `UIErrorsFrame:AddExternalErrorMessage(...)` calls
//! (`ITEM_IS_NOT_AZERITE_EMPOWERED`,
//! `AZERITE_EMPOWERED_REFORGE_NO_CHOICES_TO_UNDO`,
//! `NOT_ENOUGH_GOLD_FOR_AZERITE_RESPEC`).
//!
//! The simulator resolves these globals through the auto-generated
//! `data/global_strings.rs`. This test pins the contract — a future
//! regeneration that drops any of them would render `nil` in the
//! addon's title bar and error messages.

use wow_ui_sim::lua_api::WowLuaEnv;

const AZERITE_RESPEC_GLOBALS: &[&str] = &[
    "AZERITE_RESPEC_TITLE",
    "AZERITE_RESPEC_BUTTON",
    "AZERITE_RESPEC_TUTORIAL_TEXT",
    "ITEM_IS_NOT_AZERITE_EMPOWERED",
    "AZERITE_EMPOWERED_REFORGE_NO_CHOICES_TO_UNDO",
    "NOT_ENOUGH_GOLD_FOR_AZERITE_RESPEC",
];

#[test]
fn every_azerite_respec_string_resolves_to_a_string() {
    let env = WowLuaEnv::new().expect("env");
    for name in AZERITE_RESPEC_GLOBALS {
        let kind: String = env.eval(&format!("return type(_G[\"{name}\"])")).unwrap();
        assert_eq!(
            kind, "string",
            "{name} must resolve to a string for Blizzard_AzeriteRespecUI to render correctly"
        );
        let value: String = env.eval(&format!("return _G[\"{name}\"]")).unwrap();
        assert!(
            !value.is_empty(),
            "{name} must be non-empty — empty strings render as blank labels"
        );
    }
}

#[test]
fn azerite_respec_title_matches_canonical_blizzard_text() {
    let env = WowLuaEnv::new().expect("env");
    let value: String = env.eval("return AZERITE_RESPEC_TITLE").unwrap();
    assert_eq!(
        value, "Azerite Reforger",
        "AZERITE_RESPEC_TITLE is read verbatim by SetTitle at Blizzard_AzeriteRespecUI.lua:7"
    );
}

#[test]
fn azerite_respec_button_matches_canonical_blizzard_text() {
    let env = WowLuaEnv::new().expect("env");
    let value: String = env.eval("return AZERITE_RESPEC_BUTTON").unwrap();
    assert_eq!(value, "Reforge");
}

#[test]
fn azerite_respec_tutorial_text_matches_canonical_blizzard_text() {
    let env = WowLuaEnv::new().expect("env");
    let value: String = env.eval("return AZERITE_RESPEC_TUTORIAL_TEXT").unwrap();
    assert_eq!(
        value, "Drag a piece of Azerite Armor here to reforge its powers.",
        "AZERITE_RESPEC_TUTORIAL_TEXT shows in the empty-slot frame and is canonicalized in data/global_strings.rs"
    );
}

#[test]
fn azerite_respec_error_messages_match_canonical_blizzard_text() {
    let env = WowLuaEnv::new().expect("env");
    let item_not_empowered: String = env.eval("return ITEM_IS_NOT_AZERITE_EMPOWERED").unwrap();
    let no_choices: String = env
        .eval("return AZERITE_EMPOWERED_REFORGE_NO_CHOICES_TO_UNDO")
        .unwrap();
    let not_enough_gold: String = env
        .eval("return NOT_ENOUGH_GOLD_FOR_AZERITE_RESPEC")
        .unwrap();
    assert_eq!(item_not_empowered, "Item is not Azerite empowered");
    assert_eq!(no_choices, "No power choices to undo.");
    assert_eq!(not_enough_gold, "Not enough gold.");
}
