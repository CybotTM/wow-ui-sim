//! Original Mists WorldMap death provider and its selected Wrath XML pin.
#![cfg(feature = "client-mists")]

use std::path::PathBuf;
use wow_ui_sim::client_profile::{ACTIVE, ClientProfile};
use wow_ui_sim::loader::discover_blizzard_addon_closure_for_screen_with_overrides;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;

fn selected_mists_ui() -> PathBuf {
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
        let (_, toc) = closure
            .iter()
            .find(|(addon, _)| addon == name)
            .expect("selected dependency");
        assert_eq!(toc.file_name().unwrap().to_str().unwrap(), expected);
        eprintln!("selected {name}: {}", toc.display());
    }
    ui
}

fn load_original_death_provider() -> WowLuaEnv {
    let ui = selected_mists_ui();
    let env = WowLuaEnv::new().expect("corpse consumer environment");
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
    env.exec(
        r#"
        assert(WorldMapFrame and WorldMapFrame.pinPools, 'original XML map initialized')
        for provider in pairs(WorldMapFrame.dataProviders) do
            if provider.RefreshAllData == DeathMapDataProviderMixin.RefreshAllData then
                DeathProviderUnderTest = provider
            end
        end
        assert(DeathProviderUnderTest, 'original OnLoad installed death provider')
        local info = debug.getinfo(DeathProviderUnderTest.RefreshAllData, 'S')
        local suffix = 'Blizzard_SharedMapDataProviders/DeathMapDataProvider.lua'
        local source = info.source:gsub('\\', '/')
        assert(info.what == 'Lua' and source:sub(-#suffix) == suffix, 'original cached provider')
        print('death consumer provenance', source, info.linedefined, info.lastlinedefined)
        WorldMapFrame.mapID = 84
    "#,
    )
    .expect("selected original death provider");
    env
}

fn refresh_and_count(env: &WowLuaEnv, expected: i32) {
    let before = env.state().borrow().lua_errors.len();
    env.exec("DeathProviderUnderTest:RefreshAllData()")
        .expect("original DeathMapDataProvider.lua:26 must query corpse position");
    let count: i32 = env.eval(r#"
        local count = 0
        for pin in WorldMapFrame:EnumeratePinsByTemplate('CorpsePinTemplate') do count = count + 1 end
        return count
    "#).unwrap();
    assert_eq!(count, expected);
    assert_eq!(
        env.state().borrow().lua_errors.len(),
        before,
        "pin must not hide sibling errors"
    );
}

#[test]
fn original_death_provider_projects_supplied_corpse_and_removal() {
    let env = load_original_death_provider();
    refresh_and_count(&env, 0);
    env.exec("assert(select('#', C_DeathInfo.GetCorpseMapPosition(84)) == 1 and C_DeathInfo.GetCorpseMapPosition(84) == nil)")
        .expect("documented nil, not zero results");
    {
        let mut sim = env.state().borrow_mut();
        sim.player.health = 0;
        sim.corpse_available = true;
    }
    refresh_and_count(&env, 0);
    env.state()
        .borrow_mut()
        .corpse_map_positions
        .insert(84, (0.625, 0.375));
    refresh_and_count(&env, 1);
    env.exec(
        r#"
        local pin = WorldMapFrame:EnumeratePinsByTemplate('CorpsePinTemplate')()
        local x, y = pin:GetPosition()
        assert(x == 0.625 and y == 0.375, 'supplied coordinates reach original XML pin')
        assert(pin:IsShown(), 'original provider shows corpse pin')
        local position = C_DeathInfo.GetCorpseMapPosition(84)
        local px, py = position:GetXY()
        assert(px == x and py == y and position.x == x and position.y == y)
    "#,
    )
    .expect("supplied corpse projected faithfully");
    env.exec("WorldMapFrame.mapID = 85").unwrap();
    refresh_and_count(&env, 0);
    env.state()
        .borrow_mut()
        .corpse_map_positions
        .insert(85, (0.25, 0.75));
    refresh_and_count(&env, 1);
    env.exec(
        r#"
        local pin = WorldMapFrame:EnumeratePinsByTemplate('CorpsePinTemplate')()
        local x, y = pin:GetPosition()
        assert(x == 0.25 and y == 0.75, 'map-specific supplied projection')
    "#,
    )
    .unwrap();
    env.state().borrow_mut().corpse_available = false;
    refresh_and_count(&env, 0);
    env.state().borrow_mut().corpse_available = true;
    refresh_and_count(&env, 1);
    {
        let mut sim = env.state().borrow_mut();
        sim.player.health = sim.player.health_max;
    }
    refresh_and_count(&env, 0);
    env.state().borrow_mut().player.health = 0;
    refresh_and_count(&env, 1);
    env.exec("RetrieveCorpse()").unwrap();
    refresh_and_count(&env, 0);
}
