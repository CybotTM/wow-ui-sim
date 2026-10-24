//! CF-2 D: selected Vanilla XML group pins consuming supplied map display input.
#![cfg(any(feature = "client-era", feature = "client-anniversary"))]

use std::path::Path;

use wow_ui_sim::client_profile::{ACTIVE, ClientProfile};
use wow_ui_sim::loader::discover_blizzard_addon_closure_for_screen_with_overrides;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::toc::TocFile;

const HIDDEN_MAP: i32 = 84;
const VISIBLE_MAP: i32 = 85;
// UnitPositionFrameTemplate is supplied by the selected UIPanels TOC, not a
// hand-built frame or an explicitly executed cached Lua file.
const ROOTS: &[&str] = &["Blizzard_UIPanels_Game", "Blizzard_WorldMap"];

fn print_selected_loader_chain(ui: &Path) {
    assert!(matches!(
        ACTIVE,
        ClientProfile::Era | ClientProfile::Anniversary
    ));
    let closure =
        discover_blizzard_addon_closure_for_screen_with_overrides(ui, ScreenKind::Game, ROOTS, &[]);
    for (addon, toc_path) in &closure {
        let contents = std::fs::read_to_string(toc_path).expect("read selected TOC receipt");
        let toc = TocFile::parse(toc_path.parent().unwrap(), &contents);
        assert!(toc.allows_screen(ScreenKind::Game));
        assert!(!toc.is_game_type_restricted());
        eprintln!("CF-2 {ACTIVE:?} selected {addon}: {}", toc_path.display());
        eprintln!(
            "CF-2 metadata {:?}; selected files {:?}",
            toc.metadata, toc.files
        );
    }
    for (addon, expected) in [
        ("Blizzard_WorldMap", "Blizzard_WorldMap_Vanilla.toc"),
        (
            "Blizzard_SharedMapDataProviders",
            "Blizzard_SharedMapDataProviders_Classic.toc",
        ),
        (
            "Blizzard_UIPanels_Game",
            "Blizzard_UIPanels_Game_Classic.toc",
        ),
    ] {
        let (_, toc) = closure
            .iter()
            .find(|(name, _)| name == addon)
            .expect("selected dependency");
        assert_eq!(toc.file_name().unwrap().to_str().unwrap(), expected);
    }
}

fn load_original_group_pin() -> WowLuaEnv {
    let ui = crate::common::blizzard_addon_harness::blizzard_ui_dir();
    print_selected_loader_chain(&ui);
    let (env, loaded) =
        crate::common::blizzard_addon_harness::build_blizzard_addon_closure_env(&ui, ROOTS, &[]);
    assert!(loaded.iter().any(|name| name == "Blizzard_WorldMap"));
    // Preserve initial loading errors even if later fixture setup fails. They
    // are not the assigned missing-call RED and must not be silently cleared.
    eprintln!(
        "CF-2 initial loading diagnostics: {:?}",
        env.state().borrow().lua_errors
    );
    env.exec(
        r#"
        local info = debug.getinfo(GroupMembersPinMixin.OnMapChanged, 'S')
        local suffix = 'Blizzard_SharedMapDataProviders/GroupMembersDataProvider.lua'
        local source = info.source:gsub('\\', '/')
        assert(info.what == 'Lua' and source:sub(-#suffix) == suffix,
            'must invoke the selected original group-members consumer')
        print('CF-2 consumer provenance', source, info.linedefined, info.lastlinedefined)
        assert(WorldMapFrame and WorldMapFrame.ScrollContainer, 'normal XML world map')
        assert(WorldMapFrame.pinPools, 'original world-map OnLoad initialized pin pools')
        Cf2GroupProvider = CreateFromMixins(GroupMembersDataProviderMixin)
        -- Unrelated player-ping input; ShouldShowUnit remains the original method.
        Cf2GroupProvider.unitPinSizes = {player = 0, party = 16, raid = 16}
        Cf2GroupPin = WorldMapFrame:AcquirePin('GroupMembersPinTemplate', Cf2GroupProvider)
        assert(Cf2GroupPin:GetObjectType() == 'UnitPositionFrame', 'selected XML pin type')
        assert(Cf2GroupPin.OnMapChanged == GroupMembersPinMixin.OnMapChanged,
            'XML installed the original consumer, not a replacement mixin')
        assert(Cf2GroupPin:GetMap() == WorldMapFrame, 'original map owns the pin')
        "#,
    )
    .expect("selected TOC/XML world map and original GroupMembersPinTemplate setup");
    env
}

fn assert_original_group_pin(env: &WowLuaEnv, map: i32, shown: bool) {
    let errors_before = env.state().borrow().lua_errors.len();
    // Supply the selected map, then call the original consumer directly. Do not
    // run unrelated detail-layer refresh or replace GetMap/GetMapID/ShouldShowUnit.
    env.exec(&format!(
        "WorldMapFrame.mapID = {map}; Cf2GroupPin:OnMapChanged()"
    ))
    .expect("original GroupMembersDataProvider.lua:114 must read supplied input");
    let actual: bool = env.eval("return Cf2GroupPin:IsShown()").unwrap();
    assert_eq!(actual, shown, "group icon visibility on supplied map {map}");
    if shown {
        let pin_map: i32 = env.eval("return Cf2GroupPin:GetUiMapID()").unwrap();
        assert_eq!(pin_map, map, "visible pin binds selected map");
    }
    assert_eq!(env.state().borrow().lua_errors.len(), errors_before);
}

fn supply_map_inputs(env: &WowLuaEnv, hide84: bool, hide85: bool) {
    let mut state = env.state().borrow_mut();
    state.map_display_hide_icons.insert(HIDDEN_MAP, hide84);
    state.map_display_hide_icons.insert(VISIBLE_MAP, hide85);
}

#[test]
fn original_group_pin_tracks_map_display_visibility_and_live_updates() {
    let first = load_original_group_pin();
    supply_map_inputs(&first, true, false);
    assert_original_group_pin(&first, HIDDEN_MAP, false);
    assert_original_group_pin(&first, VISIBLE_MAP, true);
    assert_original_group_pin(&first, HIDDEN_MAP, false);
    assert_original_group_pin(&first, VISIBLE_MAP, true);

    let second = load_original_group_pin();
    supply_map_inputs(&second, false, true);
    assert_original_group_pin(&second, HIDDEN_MAP, true);
    assert_original_group_pin(&second, VISIBLE_MAP, false);
    assert_original_group_pin(&first, HIDDEN_MAP, false);
    assert_original_group_pin(&first, VISIBLE_MAP, true);

    first
        .state()
        .borrow_mut()
        .map_display_hide_icons
        .insert(HIDDEN_MAP, false);
    assert_original_group_pin(&first, HIDDEN_MAP, true);
    assert_original_group_pin(&first, VISIBLE_MAP, true);
    first
        .state()
        .borrow_mut()
        .map_display_hide_icons
        .insert(VISIBLE_MAP, true);
    assert_original_group_pin(&first, VISIBLE_MAP, false);
    assert_original_group_pin(&first, HIDDEN_MAP, true);
    assert_original_group_pin(&first, VISIBLE_MAP, false);
    assert_original_group_pin(&second, HIDDEN_MAP, true);
    assert_original_group_pin(&second, VISIBLE_MAP, false);

    supply_map_inputs(&first, true, false);
    assert_original_group_pin(&first, HIDDEN_MAP, false);
    assert_original_group_pin(&first, VISIBLE_MAP, true);
    assert_original_group_pin(&second, HIDDEN_MAP, true);
    assert_original_group_pin(&second, VISIBLE_MAP, false);
}

/// API preservation only; not a second original-consumer RED selector.
#[test]
fn map_display_absence_removal_and_supplied_false_preserve_return_arity() {
    let env = WowLuaEnv::new().expect("map display preservation environment");
    assert!(env.state().borrow().map_display_hide_icons.is_empty());
    env.exec("assert(select('#', C_Map.GetMapDisplayInfo(84)) == 0)")
        .expect("catalog presence alone supplies no map display input");
    supply_map_inputs(&env, true, false);
    env.exec(
        r#"
        local function capture(expected, ...)
            assert(select('#', ...) == 1, 'exactly one supplied result')
            local value = ...
            assert(type(value) == 'boolean' and value == expected, 'supplied boolean, not DTO')
        end
        capture(true, C_Map.GetMapDisplayInfo(84))
        capture(false, C_Map.GetMapDisplayInfo(85))
        "#,
    )
    .expect("explicit false is one boolean result");
    env.state()
        .borrow_mut()
        .map_display_hide_icons
        .remove(&VISIBLE_MAP);
    env.exec(
        "assert(select('#', C_Map.GetMapDisplayInfo(85)) == 0); assert(C_Map.GetMapDisplayInfo(84) == true)",
    )
    .expect("removed false input returns zero values and leaves the other map unchanged");
}
