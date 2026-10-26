//! Preserve non-fog pins across the real post-load workaround boundary.
#![cfg(any(
    feature = "client-retail",
    feature = "client-mists",
    feature = "client-era",
    feature = "client-anniversary"
))]

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;

fn load_original_world_map() -> WowLuaEnv {
    let ui = crate::common::blizzard_addon_harness::blizzard_ui_dir();
    let env = WowLuaEnv::new().unwrap();
    env.set_screen_size(1024.0, 768.0);
    env.state().borrow_mut().addon_base_paths = vec![ui.clone()];
    for addon in
        wow_ui_sim::loader::discover_blizzard_startup_addons_for_screen(&ui, ScreenKind::Game)
    {
        eprintln!("selected {}", addon.toc_path.display());
        wow_ui_sim::loader::load_startup_addon(
            &env.loader_env(),
            &addon.toc_path,
            addon.kind,
            None,
        )
        .expect("original startup addon load");
    }
    env.exec(
        r#"
        assert(WorldMapFrame and WorldMapFrame.pinPools, 'original XML WorldMap initialized')
        for provider in pairs(WorldMapFrame.dataProviders) do
            if provider.OnMapChanged == GroupMembersDataProviderMixin.OnMapChanged then
                GroupProviderUnderTest = provider
            end
        end
        assert(GroupProviderUnderTest and GroupProviderUnderTest.pin, 'original OnLoad group pin')
        local pin = GroupProviderUnderTest.pin
        assert(pin:GetMap() == WorldMapFrame, 'original WorldMap owns group pin')
        assert(pin.OnMapChanged == GroupMembersPinMixin.OnMapChanged, 'original callback before pass')
        local info = debug.getinfo(pin.OnMapChanged, 'S')
        local suffix = 'Blizzard_SharedMapDataProviders/GroupMembersDataProvider.lua'
        local source = info.source:gsub('\\', '/')
        assert(info.what == 'Lua' and source:sub(-#suffix) == suffix, 'original cached callback')
        print('group callback', source, info.linedefined, info.lastlinedefined)
        OriginalGroupMapChanged = pin.OnMapChanged
        "#,
    )
    .expect("original WorldMap consumer selection");
    env
}

#[test]
fn original_world_map_keeps_non_fog_pin_callback_after_each_pass() {
    let env = load_original_world_map();
    for _ in 0..2 {
        env.apply_post_load_workarounds();
        env.exec(
            r#"
            local pin = GroupProviderUnderTest.pin
            assert(pin.OnMapChanged == OriginalGroupMapChanged,
                'post-load fog workaround replaced original group OnMapChanged')
            assert(pin.TryFindingBestFogOfWarID == nil, 'group pin acquired fog refresh')
            "#,
        )
        .expect("non-fog callback must survive each stock post-load pass");
    }
    // Mists lacks GetMapDisplayInfo at this base; that separate producer lane
    // owns the missing query. Do not mask it or install a test replacement.
    if !cfg!(feature = "client-mists") {
        for (map, hidden) in [(84, true), (85, false), (84, false)] {
            env.state()
                .borrow_mut()
                .map_display_hide_icons
                .insert(map, hidden);
            env.exec(&format!(
                "WorldMapFrame.mapID = {map}; GroupProviderUnderTest:OnMapChanged(); \
                 assert(GroupProviderUnderTest.pin:IsShown() == {}, 'original visibility');",
                !hidden
            ))
            .expect("original group provider still consumes supplied visibility");
            if !hidden {
                let actual: i32 = env
                    .eval("return GroupProviderUnderTest.pin:GetUiMapID()")
                    .unwrap();
                assert_eq!(actual, map);
            }
        }
    }
}

#[test]
fn original_world_map_keeps_highlight_pin_callback_after_each_pass() {
    let env = load_original_world_map();
    env.exec(
        r#"
        HighlightPinUnderTest = WorldMapFrame:EnumeratePinsByTemplate('MapHighlightPinTemplate')()
        assert(HighlightPinUnderTest and HighlightPinUnderTest:GetMap() == WorldMapFrame,
            'original WorldMap highlight pin')
        assert(HighlightPinUnderTest.OnLoad == MapHighlightPinMixin.OnLoad, 'original highlight mixin')
        OriginalHighlightMapChanged = HighlightPinUnderTest.OnMapChanged
        assert(type(OriginalHighlightMapChanged) == 'function', 'inherited map callback')
        "#,
    )
    .unwrap();
    for _ in 0..2 {
        env.apply_post_load_workarounds();
        env.exec(
            r#"
            assert(HighlightPinUnderTest.OnMapChanged == OriginalHighlightMapChanged,
                'post-load fog workaround replaced original highlight OnMapChanged')
            assert(HighlightPinUnderTest.TryFindingBestFogOfWarID == nil, 'highlight acquired fog refresh')
            HighlightPinUnderTest:OnMapChanged()
            "#,
        )
        .expect("original highlight callback survives and remains callable");
    }
}

#[test]
fn original_world_map_fog_pin_still_refreshes_and_clears_missing_fog() {
    let env = load_original_world_map();
    env.apply_post_load_workarounds();
    env.exec(
        r#"
        local pin = WorldMapFrame:EnumeratePinsByTemplate('FogOfWarPinTemplate')()
        assert(pin and pin:GetMap() == WorldMapFrame, 'original fog pin')
        for _, mapID in ipairs({84, 85, 0}) do
            WorldMapFrame.mapID = mapID
            pin:SetFogOfWarBackgroundAtlas('stale-background')
            pin:SetFogOfWarMaskAtlas('stale-mask')
            pin:Show()
            pin:OnMapChanged()
            local expected = mapID == 0 and C_Map.GetCurrentMapID() or mapID
            assert(pin:GetUiMapID() == expected, 'fog pin follows canvas/current map')
            assert(pin:GetFogOfWarBackgroundAtlas() == nil, 'stale fog background cleared')
            assert(pin:GetFogOfWarMaskAtlas() == nil, 'stale fog mask cleared')
            assert(not pin:IsShown(), 'missing fog data stays hidden')
        end
        "#,
    )
    .expect("fog-only repair remains active on original pin");
}
