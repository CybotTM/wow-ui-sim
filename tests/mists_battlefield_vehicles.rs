//! Mists-only original consumer. Retail does not publish this supplied-state API.
#![cfg(feature = "client-mists")]

use std::path::{Path, PathBuf};
use wow_ui_sim::c_api::c_pvp::battlefield_vehicles::BattlefieldVehicle;
use wow_ui_sim::client_profile::{ACTIVE, ClientProfile};
use wow_ui_sim::loader::discover_blizzard_addon_closure_for_screen_with_overrides;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;

fn selected_mists_ui() -> PathBuf {
    assert_eq!(ACTIVE, ClientProfile::Mists);
    let ui = crate::common::blizzard_addon_harness::blizzard_ui_dir();
    let roots = &["Blizzard_WorldMap"];
    let closure = discover_blizzard_addon_closure_for_screen_with_overrides(
        &ui,
        ScreenKind::Game,
        roots,
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

fn load_stock_ui(ui: &Path) -> WowLuaEnv {
    // WorldMap's TOC omits implicit eager dependencies. Use stock discovery order,
    // not hand-loaded Lua or replacement provider methods.
    let env = WowLuaEnv::new().expect("vehicle consumer environment");
    env.set_screen_size(1024.0, 768.0);
    env.state().borrow_mut().addon_base_paths = vec![ui.to_path_buf()];
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
    env
}

fn load_original_vehicle_provider() -> WowLuaEnv {
    let env = load_stock_ui(&selected_mists_ui());
    eprintln!(
        "initial load diagnostics: {:?}",
        env.state().borrow().lua_errors
    );
    env.exec(
        r#"
        assert(WorldMapFrame and WorldMapFrame.pinPools, 'original XML map initialized')
        for provider in pairs(WorldMapFrame.dataProviders) do
            if provider.RefreshAllData == VehicleDataProviderMixin.RefreshAllData then
                VehicleProviderUnderTest = provider
            end
        end
        assert(VehicleProviderUnderTest, 'original OnLoad selected the vehicle provider')
        local info = debug.getinfo(VehicleProviderUnderTest.RefreshAllData, 'S')
        local suffix = 'Blizzard_SharedMapDataProviders/VehicleDataProvider.lua'
        local source = info.source:gsub('\\', '/')
        assert(info.what == 'Lua' and source:sub(-#suffix) == suffix, 'original cached provider')
        print('vehicle consumer provenance', source, info.linedefined, info.lastlinedefined)
        WorldMapFrame.mapID = 84
        assert(C_Map.GetMapInfo(84).mapType >= Enum.UIMapType.Zone, 'zone gate must reach API')
    "#,
    )
    .expect("selected original WorldMap vehicle provider");
    env
}

fn supplied_vehicle(name: &str, x: f64) -> BattlefieldVehicle {
    BattlefieldVehicle {
        x,
        y: 0.75,
        name: name.into(),
        is_occupied: true,
        atlas: "Vehicle-SiegeEngine".into(),
        texture_width: 37.0,
        texture_height: 29.0,
        facing: 0.375,
        is_player: false,
        is_alive: true,
        should_draw_below_player_blips: false,
    }
}

fn refresh_and_count(env: &WowLuaEnv, expected: i32) {
    let before = env.state().borrow().lua_errors.len();
    env.exec("VehicleProviderUnderTest:RefreshAllData()")
        .expect("original VehicleDataProvider.lua:31 must read supplied battlefield vehicles");
    let count: i32 = env.eval(r#"
        local count = 0
        for pin in WorldMapFrame:EnumeratePinsByTemplate('VehiclePinTemplate') do count = count + 1 end
        return count
    "#).unwrap();
    assert_eq!(count, expected);
    assert_eq!(
        env.state().borrow().lua_errors.len(),
        before,
        "pin acquisition must not hide a sibling API failure"
    );
}

#[test]
fn original_world_map_vehicle_provider_projects_supplied_state_and_removal() {
    let env = load_original_vehicle_provider();
    refresh_and_count(&env, 0);
    {
        let mut sim = env.state().borrow_mut();
        let mut dead = supplied_vehicle("dead fixture", 0.2);
        dead.is_alive = false;
        let mut player = supplied_vehicle("player fixture", 0.3);
        player.is_player = true;
        sim.is_active_battlefield = true;
        sim.battlefield_vehicles.insert(
            84,
            vec![dead, supplied_vehicle("supplied siege", 0.625), player],
        );
    }
    refresh_and_count(&env, 1);
    env.exec(r#"
        local pin = WorldMapFrame:EnumeratePinsByTemplate('VehiclePinTemplate')()
        assert(pin:GetVehicleIndex() == 2, 'preserve supplied list order through filtering')
        assert(pin.name == 'supplied siege', 'original OnAcquired/Refresh reads supplied name')
        assert(pin:GetWidth() == 37 and pin:GetHeight() == 29, 'supplied dimensions reach original pin')
        assert(pin.Texture:GetAtlas() == 'Vehicle-SiegeEngine', 'supplied atlas reaches texture')
        local x, y = pin:GetPosition()
        assert(x == 0.625 and y == 0.75, 'supplied coordinates reach original pin')
        local v = C_PvP.GetBattlefieldVehicles(84)[2]
        assert(v.x == x and v.y == y and v.name == pin.name)
        assert(v.isOccupied and v.isAlive and not v.isPlayer)
        assert(v.facing == 0.375 and v.textureWidth == 37 and v.textureHeight == 29)
        assert(v.atlas == 'Vehicle-SiegeEngine' and v.shouldDrawBelowPlayerBlips == false)
        assert(select('#', C_PvP.GetBattlefieldVehicleInfo(0, 84)) == 0)
        assert(select('#', C_PvP.GetBattlefieldVehicleInfo(4, 84)) == 0)
    "#).expect("original XML pin faithfully consumes the supplied record");
    env.exec("WorldMapFrame.mapID = 85").unwrap();
    refresh_and_count(&env, 0);
    env.exec("WorldMapFrame.mapID = 84").unwrap();
    refresh_and_count(&env, 1);
    env.state().borrow_mut().is_active_battlefield = false;
    refresh_and_count(&env, 0);
    env.exec("assert(select('#', C_PvP.GetBattlefieldVehicles(84)) == 0)")
        .expect("outside a battlefield returns no values even with old observations");
    env.state().borrow_mut().is_active_battlefield = true;
    env.state()
        .borrow_mut()
        .battlefield_vehicles
        .insert(84, vec![]);
    refresh_and_count(&env, 0);
    env.exec("assert(type(C_PvP.GetBattlefieldVehicles(84)) == 'table' and #C_PvP.GetBattlefieldVehicles(84) == 0)")
        .expect("explicit empty observation returns an empty array");
}
