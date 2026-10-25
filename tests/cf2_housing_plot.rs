//! Static staging only: expected RED/GREEN, no execution evidence yet.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

use crate::common::blizzard_addon_harness::{
    blizzard_ui_dir, load_blizzard_addon_closure_into_env, new_blizzard_addon_env,
};
use wow_ui_sim::c_api::c_housing::HousingArea;
use wow_ui_sim::loader::find_toc_file;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::toc::TocFile;

fn load_original_world_handler() -> WowLuaEnv {
    let ui = blizzard_ui_dir();
    let toc_path = find_toc_file(&ui.join("Blizzard_Game")).expect("selected Game TOC");
    let toc = TocFile::from_file(&toc_path).expect("parse selected Game TOC");
    assert!(toc.allows_screen(ScreenKind::Game));
    assert!(!toc.is_game_type_restricted());
    assert!(
        toc.files
            .iter()
            .any(|path| path.ends_with("Mainline/EventImplementation.lua"))
    );
    assert!(
        toc.files
            .iter()
            .any(|path| path.ends_with("Shared/EventRouting.lua"))
    );
    eprintln!("CF2 housing plot selected TOC: {}", toc_path.display());
    let env = new_blizzard_addon_env(&ui);
    crate::common::panel_fixtures::load_panel_addons(&env);
    crate::common::load_blizzard_addon_bootstrap(
        &env,
        &ui,
        "Blizzard_BoostTutorial",
        "Blizzard_BoostTutorial_Bootstrap.lua",
    );
    let loaded = load_blizzard_addon_closure_into_env(&env, &ui, &["Blizzard_Game"], &[]);
    assert!(loaded.iter().any(|name| name == "Blizzard_Game"));
    let controls_path =
        find_toc_file(&ui.join("Blizzard_HousingControls")).expect("selected housing controls TOC");
    let controls = TocFile::from_file(&controls_path).expect("parse housing controls TOC");
    assert!(controls.allows_screen(ScreenKind::Game));
    assert!(!controls.is_game_type_restricted());
    assert!(
        controls
            .files
            .iter()
            .any(|path| { path.ends_with("Blizzard_HousingControls_Bootstrap.lua") })
    );
    wow_ui_sim::loader::load_addon_bootstrap_from_toc(&env.loader_env(), &controls)
        .expect("load original housing bootstrap through the real startup phase");
    for file in [
        "Blizzard_HousingTutorialsData.lua",
        "Blizzard_HousingTutorialsUtil.lua",
    ] {
        crate::common::load_blizzard_addon_bootstrap(&env, &ui, "Blizzard_HousingTutorials", file);
    }
    install_branch_observers(&env);
    env
}

fn install_branch_observers(env: &WowLuaEnv) {
    env.exec(
        r#"
        local handler = debug.getinfo(GameEvent.HandlePlayerEnteringWorld, 'S')
        local source = handler.source:gsub('\\', '/')
        local mainline = 'Blizzard_Game/Mainline/EventImplementation.lua'
        assert(handler.what == 'Lua' and source:sub(-#mainline) == mainline,
            'must retain original selected world handler')
        local loader = debug.getinfo(HousingControls_LoadUI, 'S')
        local bootstrap = 'Blizzard_HousingControls/Blizzard_HousingControls_Bootstrap.lua'
        local loaderSource = loader.source:gsub('\\', '/')
        assert(loader.what == 'Lua' and loaderSource:sub(-#bootstrap) == bootstrap,
            'must retain original housing loader')
        print('CF2 housing consumer', source, handler.linedefined, handler.lastlinedefined)
        __housingPlotLoads = 0
        __housingWorldCompletions = 0
        hooksecurefunc('HousingControls_LoadUI', function()
            __housingPlotLoads = __housingPlotLoads + 1
        end)
        hooksecurefunc(GameEvent, 'HandlePlayerEnteringWorld', function()
            __housingWorldCompletions = __housingWorldCompletions + 1
        end)
        "#,
    )
    .expect("observe original handler and loader without replacing their behavior");
}

fn assert_world_branch(env: &WowLuaEnv, expected: bool) {
    env.exec("__housingPlotLoads = 0; __housingWorldCompletions = 0")
        .expect("reset observation counters only");
    let before = env.state().borrow().lua_errors.len();
    env.fire_event_with_args(
        "PLAYER_ENTERING_WORLD",
        &[rilua::Val::Bool(false), rilua::Val::Bool(false)],
    )
    .expect("fire registered original world event");
    {
        let sim = env.state().borrow();
        let errors = &sim.lua_errors[before..];
        eprintln!("CF2 housing world-event diagnostics: {errors:?}");
        assert!(
            !errors.iter().any(|error| {
                error.contains("Blizzard_Game/")
                    || error.contains("IsInsideHouseOrPlot")
                    || error.contains("Blizzard_Game\\")
            }),
            "original event must reach and finish its housing branch: {errors:?}"
        );
    }
    let completions: i32 = env.eval("return __housingWorldCompletions").unwrap();
    assert_eq!(completions, 1, "original registered event must finish once");
    let loads: i32 = env.eval("return __housingPlotLoads").unwrap();
    assert_eq!(
        loads,
        i32::from(expected),
        "original loader branch decision"
    );
    // Check the signature only AFTER the consumer, so missing-producer RED is
    // required at the original event boundary, not at a standalone API probe.
    env.exec(&format!(
        "assert(select('#', C_Housing.IsInsideHouseOrPlot()) == 1); \
         assert(type(C_Housing.IsInsideHouseOrPlot()) == 'boolean'); \
         assert(C_Housing.IsInsideHouseOrPlot() == {expected})"
    ))
    .expect("one non-nil boolean from the supplied spatial area");
}

#[test]
fn original_world_event_branches_on_house_plot_absence_and_reentry() {
    use HousingArea::{HouseInterior, PlotExterior};

    let env = load_original_world_handler();
    assert_eq!(env.state().borrow().housing.current_housing_area, None);
    for (area, owned, expected) in [
        (None, false, false),
        (Some(HouseInterior), false, true),
        (Some(PlotExterior), false, true),
        (None, false, false),
        (Some(PlotExterior), true, true),
        (Some(HouseInterior), true, true),
        (None, false, false),
        (Some(PlotExterior), false, true),
    ] {
        {
            let mut sim = env.state().borrow_mut();
            sim.housing.current_housing_area = area;
            sim.housing.inside_owned_house = owned && area == Some(HouseInterior);
            sim.housing.inside_owned_plot = owned && area == Some(PlotExterior);
        }
        assert_world_branch(&env, expected);
    }
}
