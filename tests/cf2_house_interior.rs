//! CF-2 remaining housing consumer. Edit-only: expected RED, not observed RED.
#![cfg(any(
    feature = "client-retail",
    feature = "client-ptr",
    feature = "client-wowforever",
))]

use wow_ui_sim::c_api::c_housing::HousingArea;
use wow_ui_sim::lua_api::WowLuaEnv;

fn load_selected_minimap() -> WowLuaEnv {
    use crate::common::blizzard_addon_harness::{
        blizzard_ui_dir, build_blizzard_addon_closure_env,
    };
    use wow_ui_sim::loader::find_toc_file;
    use wow_ui_sim::screen::ScreenKind;
    use wow_ui_sim::toc::TocFile;

    let ui = blizzard_ui_dir();
    let addon = ui.join("Blizzard_Minimap");
    let toc_path = find_toc_file(&addon).expect("profile-selected Minimap TOC");
    let toc = TocFile::from_file(&toc_path).expect("parse selected TOC");
    assert!(toc.allows_screen(ScreenKind::Game));
    assert!(!toc.is_game_type_restricted());
    assert!(
        toc.files
            .iter()
            .any(|file| file.ends_with("Mainline/Minimap.lua"))
    );
    assert!(
        toc.files
            .iter()
            .any(|file| file.ends_with("Mainline/Minimap.xml"))
    );
    eprintln!("CF-2 selected consumer TOC: {}", toc_path.display());
    let (env, loaded) = build_blizzard_addon_closure_env(&ui, &["Blizzard_Minimap"], &[]);
    assert!(loaded.iter().any(|name| name == "Blizzard_Minimap"));
    env.exec(
        r#"
        for _, callback in ipairs({MinimapMixin.UpdateStaticOverlayTexture, MinimapMixin.OnEvent}) do
            local info = debug.getinfo(callback, 'S')
            local suffix = 'Blizzard_Minimap/Mainline/Minimap.lua'
            local source = info.source:gsub('\\', '/')
            assert(info.what == 'Lua' and source:sub(-#suffix) == suffix,
                'must use the selected original Blizzard consumer')
            print('CF-2 original consumer', source, info.linedefined, info.lastlinedefined)
        end
        assert(Minimap:IsEventRegistered('PLAYER_ENTERING_WORLD'))
        "#,
    )
    .expect("original loaded consumer and XML event registration");
    env
}

fn assert_overlay(env: &WowLuaEnv, inside: bool) {
    let shown: bool = env
        .eval("return MinimapBackdrop.StaticOverlayTexture:IsShown()")
        .expect("original overlay visibility");
    assert_eq!(
        shown, inside,
        "original Minimap consumer must follow live housing area"
    );
    if inside {
        let atlas: String = env
            .eval("return MinimapBackdrop.StaticOverlayTexture:GetAtlas()")
            .expect("indoor overlay atlas");
        assert_eq!(atlas, "ui-hud-minimap-housing-indoor-static-bg");
    }
}

fn exercise_housing_transitions(env: &WowLuaEnv, refresh: impl Fn(&WowLuaEnv)) {
    use HousingArea::{HouseInterior, PlotExterior};

    assert_eq!(env.state().borrow().housing.current_housing_area, None);
    // Both selectors independently include absent, visiting, plot and owned states.
    // Constant false/nil fails visiting; constant true fails absent/plot; ownership-only
    // fails visiting. Reentry also rejects a producer that retains its first result.
    for (area, owned, inside) in [
        (None, false, false),
        (Some(HouseInterior), false, true),
        (Some(PlotExterior), false, false),
        (Some(HouseInterior), true, true),
        (None, false, false),
        (Some(HouseInterior), false, true),
    ] {
        {
            let mut sim = env.state().borrow_mut();
            sim.housing.current_housing_area = area;
            sim.housing.inside_owned_house = owned;
            sim.housing.inside_owned_plot = area == Some(PlotExterior) && owned;
        }
        refresh(env);
        assert_overlay(env, inside);
    }
}

#[test]
fn original_minimap_overlay_tracks_house_plot_absence_and_ownership() {
    let env = load_selected_minimap();
    exercise_housing_transitions(&env, |env| {
        env.exec("MinimapMixin.UpdateStaticOverlayTexture(Minimap)")
            .expect("original overlay update must reach the housing producer");
    });
}

#[test]
fn original_minimap_world_event_tracks_house_plot_absence_and_ownership() {
    let env = load_selected_minimap();
    exercise_housing_transitions(&env, |env| {
        let before = env.state().borrow().lua_errors.len();
        env.fire_event("PLAYER_ENTERING_WORLD")
            .expect("dispatch registered original Minimap event");
        let sim = env.state().borrow();
        let new_errors = &sim.lua_errors[before..];
        eprintln!("CF-2 event diagnostics: {new_errors:?}");
        assert!(
            !new_errors
                .iter()
                .any(|error| error.contains("Blizzard_Minimap")),
            "original Minimap event must finish: {new_errors:?}"
        );
    });
}
