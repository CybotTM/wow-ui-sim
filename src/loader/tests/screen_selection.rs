use std::path::PathBuf;

use crate::loader::discover_blizzard_addons_for_screen;
use crate::screen::ScreenKind;

fn blizzard_ui_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI")
}

fn addon_names(screen: ScreenKind) -> Vec<String> {
    discover_blizzard_addons_for_screen(&blizzard_ui_dir(), screen)
        .into_iter()
        .map(|(name, _)| name)
        .collect()
}

#[test]
fn game_screen_discovers_game_addons_not_glue_base() {
    let addons = addon_names(ScreenKind::Game);
    assert!(addons.iter().any(|name| name == "Blizzard_UIParent"));
    assert!(!addons.iter().any(|name| name == "Blizzard_GlueParent"));
    assert!(!addons.iter().any(|name| name == "Blizzard_GlueXML"));
}

#[test]
fn login_screen_discovers_glue_addons_not_game_base() {
    let addons = addon_names(ScreenKind::Login);
    assert!(addons.iter().any(|name| name == "Blizzard_GlueParent"));
    assert!(addons.iter().any(|name| name == "Blizzard_GlueXML"));
    assert!(!addons.iter().any(|name| name == "Blizzard_UIParent"));
    assert!(!addons.iter().any(|name| name == "Blizzard_CharacterCreate"));
    assert!(!addons.iter().any(|name| name == "Blizzard_CharacterCustomize"));
}

#[test]
fn character_select_screen_uses_glue_addon_set() {
    let addons = addon_names(ScreenKind::CharacterSelect);
    assert!(addons.iter().any(|name| name == "Blizzard_GlueParent"));
    assert!(addons.iter().any(|name| name == "Blizzard_GlueXML"));
    assert!(!addons.iter().any(|name| name == "Blizzard_UIParent"));
    assert!(addons.iter().any(|name| name == "Blizzard_CharacterCreate"));
}
