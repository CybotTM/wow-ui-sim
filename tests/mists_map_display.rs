//! Mists stock WorldMap provider consuming supplied map display visibility.
#![cfg(feature = "client-mists")]

use wow_ui_sim::client_profile::{ACTIVE, ClientProfile};
use wow_ui_sim::loader::discover_blizzard_addon_closure_for_screen_with_overrides;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::toc::TocFile;

fn print_selected_group_sources() -> std::path::PathBuf {
    assert_eq!(ACTIVE, ClientProfile::Mists);
    let ui = crate::common::blizzard_addon_harness::blizzard_ui_dir();
    let closure = discover_blizzard_addon_closure_for_screen_with_overrides(
        &ui,
        ScreenKind::Game,
        &["Blizzard_WorldMap"],
        &[],
    );
    for (name, expected) in [
        ("Blizzard_WorldMap", "Blizzard_WorldMap_Mists.toc"),
        (
            "Blizzard_SharedMapDataProviders",
            "Blizzard_SharedMapDataProviders_Mists.toc",
        ),
    ] {
        let (_, path) = closure.iter().find(|(addon, _)| addon == name).unwrap();
        assert_eq!(path.file_name().unwrap().to_str().unwrap(), expected);
        let contents = std::fs::read_to_string(path).unwrap();
        let toc = TocFile::parse(path.parent().unwrap(), &contents);
        assert!(toc.allows_screen(ScreenKind::Game));
        assert!(!toc.is_game_type_restricted());
        eprintln!(
            "selected {name}: {} {:?} {:?}",
            path.display(),
            toc.metadata,
            toc.files
        );
    }
    ui
}

fn load_original_group_provider() -> WowLuaEnv {
    let ui = print_selected_group_sources();
    let env = WowLuaEnv::new().expect("Mists group provider environment");
    env.set_screen_size(1024.0, 768.0);
    env.state().borrow_mut().addon_base_paths = vec![ui.clone()];
    for addon in
        wow_ui_sim::loader::discover_blizzard_startup_addons_for_screen(&ui, ScreenKind::Game)
    {
        wow_ui_sim::loader::load_startup_addon(
            &env.loader_env(),
            &addon.toc_path,
            addon.kind,
            None,
        )
        .expect("original startup addon load");
    }
    env.apply_post_load_workarounds();
    eprintln!("initial diagnostics: {:?}", env.state().borrow().lua_errors);
    assert_original_group_provider(&env);
    env
}

fn assert_original_group_provider(env: &WowLuaEnv) {
    env.exec(r#"
        assert(WorldMapFrame and WorldMapFrame.pinPools, 'original XML map initialized')
        for provider in pairs(WorldMapFrame.dataProviders) do
            if provider.OnMapChanged == GroupMembersDataProviderMixin.OnMapChanged then
                GroupProviderUnderTest = provider
            end
        end
        assert(GroupProviderUnderTest and GroupProviderUnderTest.pin, 'original OnLoad installed group provider and pin')
        local pin = GroupProviderUnderTest.pin
        assert(pin:GetObjectType() == 'UnitPositionFrame', 'selected XML pin')
        assert(pin.OnMapChanged == GroupMembersPinMixin.OnMapChanged, 'original XML pin method')
        assert(pin:GetMap() == WorldMapFrame, 'original map owns pin')
        for _, callback in ipairs({GroupProviderUnderTest.OnMapChanged, pin.OnMapChanged}) do
            local info = debug.getinfo(callback, 'S')
            local suffix = 'Blizzard_SharedMapDataProviders/GroupMembersDataProvider.lua'
            local source = info.source:gsub('\\', '/')
            assert(info.what == 'Lua' and source:sub(-#suffix) == suffix, 'original cached consumer')
            print('group consumer provenance', source, info.linedefined, info.lastlinedefined)
        end
    "#).expect("selected original group provider");
}

fn supply_display_input(env: &WowLuaEnv, hide84: bool, hide85: bool) {
    let mut sim = env.state().borrow_mut();
    sim.map_display_hide_icons.insert(84, hide84);
    sim.map_display_hide_icons.insert(85, hide85);
}

fn assert_group_visibility(env: &WowLuaEnv, map: i32, shown: bool) {
    let before = env.state().borrow().lua_errors.len();
    env.exec(&format!(
        "WorldMapFrame.mapID = {map}; GroupProviderUnderTest:OnMapChanged()"
    ))
    .expect("original GroupMembersDataProvider.lua:114 must read supplied display input");
    let actual: bool = env
        .eval("return GroupProviderUnderTest.pin:IsShown()")
        .unwrap();
    assert_eq!(actual, shown, "original group pin on map {map}");
    if shown {
        let actual_map: i32 = env
            .eval("return GroupProviderUnderTest.pin:GetUiMapID()")
            .unwrap();
        assert_eq!(actual_map, map, "shown original pin binds selected map");
    }
    assert_eq!(
        env.state().borrow().lua_errors.len(),
        before,
        "no hidden callback errors"
    );
}

#[test]
fn original_group_provider_tracks_supplied_display_and_live_changes() {
    let first = load_original_group_provider();
    supply_display_input(&first, true, false);
    assert_group_visibility(&first, 84, false);
    assert_group_visibility(&first, 85, true);
    assert_group_visibility(&first, 84, false);
    assert_group_visibility(&first, 85, true);

    let second = load_original_group_provider();
    supply_display_input(&second, false, true);
    assert_group_visibility(&second, 84, true);
    assert_group_visibility(&second, 85, false);
    assert_group_visibility(&first, 84, false);
    assert_group_visibility(&first, 85, true);

    first
        .state()
        .borrow_mut()
        .map_display_hide_icons
        .insert(84, false);
    assert_group_visibility(&first, 84, true);
    first
        .state()
        .borrow_mut()
        .map_display_hide_icons
        .insert(85, true);
    assert_group_visibility(&first, 85, false);
    assert_group_visibility(&first, 84, true);
    supply_display_input(&first, true, false);
    assert_group_visibility(&first, 84, false);
    assert_group_visibility(&first, 85, true);
    assert_group_visibility(&second, 84, true);
    assert_group_visibility(&second, 85, false);
}

/// Declaration/return-policy preservation, not an independent consumer RED.
#[test]
fn map_display_preserves_absence_false_and_removal_arity() {
    let env = WowLuaEnv::new().unwrap();
    assert!(env.state().borrow().map_display_hide_icons.is_empty());
    env.exec("assert(select('#', C_Map.GetMapDisplayInfo(84)) == 0)")
        .unwrap();
    supply_display_input(&env, true, false);
    env.exec(
        r#"
        local function capture(expected, ...)
            assert(select('#', ...) == 1, 'one supplied result')
            local value = ...
            assert(type(value) == 'boolean' and value == expected, 'hideIcons boolean, not struct')
        end
        capture(true, C_Map.GetMapDisplayInfo(84))
        capture(false, C_Map.GetMapDisplayInfo(85))
    "#,
    )
    .unwrap();
    env.state().borrow_mut().map_display_hide_icons.remove(&85);
    env.exec("assert(select('#', C_Map.GetMapDisplayInfo(85)) == 0); assert(C_Map.GetMapDisplayInfo(84) == true)").unwrap();
}
