//! Approved temporary Wrath policies through unchanged startup consumers.
#![cfg(feature = "client-wrath")]
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;

fn load_original_wrath_startup() -> WowLuaEnv {
    let env = WowLuaEnv::new().unwrap();
    env.set_screen_size(1600.0, 1200.0);
    env.set_font_system(std::rc::Rc::new(std::cell::RefCell::new(
        wow_ui_sim::render::font::WowFontSystem::new_without_casc(),
    )));
    env.set_screen_mode(ScreenKind::Game);
    let ui = wow_ui_sim::paths::default_blizzard_ui_addons_path().unwrap();
    env.state().borrow_mut().addon_base_paths = vec![ui.clone()];
    let toc = wow_ui_sim::client_profile::blizzard_ui_framexml_toc().unwrap();
    wow_ui_sim::loader::load_addon(&env.loader_env(), &toc).unwrap();
    for addon in
        wow_ui_sim::loader::discover_blizzard_startup_addons_for_screen(&ui, ScreenKind::Game)
    {
        wow_ui_sim::loader::load_startup_addon(
            &env.loader_env(),
            &addon.toc_path,
            addon.kind,
            None,
        )
        .unwrap();
    }
    env.apply_post_load_workarounds();
    wow_ui_sim::startup::collect_lua_error_startup(&env);
    env
}

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
