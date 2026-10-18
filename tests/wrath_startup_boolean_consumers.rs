//! Approved temporary Wrath policies through unchanged startup consumers.
#![cfg(feature = "client-wrath")]
#[path = "common/wrath_full_ui.rs"]
mod wrath_full_ui;
use wrath_full_ui::load_original_wrath_startup;

#[test]
fn wrath_original_worldmap_skips_unavailable_debug_zone_map() {
    let env = load_original_wrath_startup();
    let errors: Vec<_> = env
        .state()
        .borrow()
        .lua_errors
        .iter()
        .filter(|message| message.contains("WorldMapFrame.lua:363"))
        .cloned()
        .collect();
    assert!(
        errors.is_empty(),
        "original OnLoad capability boundary: {errors:?}"
    );
    env.exec(
        r#"
        assert(HasDebugZoneMap() == false)
        assert(NUM_WORLDMAP_DEBUG_ZONEMAP == 0)
        assert(WorldMapDetailFrame.zoneMap == nil)
    "#,
    )
    .unwrap();
}

#[test]
fn wrath_original_worldmap_onload_reads_empty_debug_objects() {
    let env = load_original_wrath_startup();
    let errors: Vec<_> = env
        .state()
        .borrow()
        .lua_errors
        .iter()
        .filter(|message| message.contains("WorldMapFrame.lua:402"))
        .cloned()
        .collect();
    assert!(
        errors.is_empty(),
        "original OnLoad debug-object count: {errors:?}"
    );
    env.exec(
        r#"
        assert(GetNumMapDebugObjects() == 0)
        assert(NUM_WORLDMAP_DEBUG_OBJECTS == 0)
        "#,
    )
    .unwrap();
}

#[test]
fn wrath_original_worldmap_onload_hides_empty_floor_selector() {
    let env = load_original_wrath_startup();
    let errors: Vec<_> = env
        .state()
        .borrow()
        .lua_errors
        .iter()
        .filter(|message| message.contains("WorldMapFrame.lua:592"))
        .cloned()
        .collect();
    assert!(
        errors.is_empty(),
        "original OnLoad dungeon-floor count: {errors:?}"
    );
    env.exec(
        r#"
        assert(GetNumDungeonMapLevels() == 0)
        WorldMapLevelDropDown:Show()
        WorldMapLevelUpButton:Show()
        WorldMapLevelDownButton:Show()
        WorldMapLevelDropDown_Update()
        assert(not WorldMapLevelDropDown:IsShown())
        assert(not WorldMapLevelUpButton:IsShown())
        assert(not WorldMapLevelDownButton:IsShown())
        "#,
    )
    .unwrap();
}

#[test]
fn wrath_original_player_world_entry_hides_unsupplied_voice_indicator() {
    let env = load_original_wrath_startup();
    let errors: Vec<_> = env
        .state()
        .borrow()
        .lua_errors
        .iter()
        .filter(|message| message.contains("UnitIsTalking"))
        .cloned()
        .collect();
    assert!(
        errors.is_empty(),
        "original world-entry voice boundary: {errors:?}"
    );
    env.exec(
        r#"
        assert(UnitIsTalking(UnitName('player')) == false)
        for _, name in ipairs({'', 'Unknown speaker', 'Jöhn', string.char(255, 0, 128)}) do
            assert(UnitIsTalking(name) == false)
        end
        assert(not PlayerSpeakerFrame:IsShown())
    "#,
    )
    .unwrap();
}
