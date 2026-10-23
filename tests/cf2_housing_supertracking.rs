//! Original-cache CF-2 slice 5 fixtures. Authored only; RED not observed.
//! Populated supertracking inputs remain PENDING-NATIVE-ORACLE.
#![cfg(any(
    feature = "client-retail",
    feature = "client-ptr",
    feature = "client-wowforever",
))]

use wow_ui_sim::lua_api::WowLuaEnv;

fn load_original_consumer(root: &str) -> WowLuaEnv {
    let ui = crate::common::blizzard_addon_harness::blizzard_ui_dir();
    let (env, loaded) =
        crate::common::blizzard_addon_harness::build_blizzard_addon_closure_env(&ui, &[root], &[]);
    assert!(loaded.iter().any(|name| name == root));
    eprintln!("CF-2 original consumer cache: {}", ui.display());
    env
}

fn assert_original_source(env: &WowLuaEnv, function: &str, suffix: &str) {
    env.exec(&format!(
        r#"
        local info = debug.getinfo({function}, 'S')
        local source = info.source:gsub('\\', '/')
        local suffix = '{suffix}'
        assert(info.what == 'Lua' and source:sub(-#suffix) == suffix,
            'must execute original cached Blizzard consumer')
        print('CF-2 consumer provenance', source, info.linedefined, info.lastlinedefined)
        "#,
    ))
    .expect("original consumer provenance");
}

#[test]
fn original_minimap_overlay_follows_owned_house_interior_transitions() {
    let env = load_original_consumer("Blizzard_Minimap");
    assert_original_source(
        &env,
        "MinimapMixin.UpdateStaticOverlayTexture",
        "Blizzard_Minimap/Mainline/Minimap.lua",
    );
    {
        let mut sim = env.state().borrow_mut();
        sim.housing.inside_owned_house = true;
        sim.housing.inside_owned_plot = false;
    }
    let errors_before = env.state().borrow().lua_errors.len();
    env.exec(
        r#"
        MinimapBackdrop.StaticOverlayTexture:Hide()
        MinimapMixin.UpdateStaticOverlayTexture(Minimap)
        assert(MinimapBackdrop.StaticOverlayTexture:IsShown(),
            'owned house interior shows the original minimap overlay')
        assert(MinimapBackdrop.StaticOverlayTexture:GetAtlas() ==
            'ui-hud-minimap-housing-indoor-static-bg', 'indoor housing atlas')
        "#,
    )
    .expect("original minimap consumer inside owned house");
    {
        let mut sim = env.state().borrow_mut();
        sim.housing.inside_owned_house = false;
        sim.housing.inside_owned_plot = true;
    }
    env.exec(
        r#"
        MinimapMixin.UpdateStaticOverlayTexture(Minimap)
        assert(not MinimapBackdrop.StaticOverlayTexture:IsShown(),
            'owned exterior plot is not a house interior')
        "#,
    )
    .expect("original minimap consumer after leaving house for plot");
    env.state().borrow_mut().housing.inside_owned_house = true;
    env.state().borrow_mut().housing.inside_owned_plot = false;
    env.exec(
        r#"
        MinimapMixin.UpdateStaticOverlayTexture(Minimap)
        assert(MinimapBackdrop.StaticOverlayTexture:IsShown(), 'reentry restores overlay')
        "#,
    )
    .expect("original minimap consumer after reentry");
    assert_eq!(env.state().borrow().lua_errors.len(), errors_before);
}

#[test]
fn original_supertracking_refresh_publishes_its_cache_to_listeners_preserves_boundary() {
    let env = load_original_consumer("Blizzard_FrameXMLUtil");
    assert_original_source(
        &env,
        "QuestSuperTracking_GetSuperTrackedContent",
        "Blizzard_FrameXMLUtil/Mainline/Blizzard_QuestSuperTracking.lua",
    );
    env.exec(
        r#"
        CF2SupertrackingObserver = { notifications = 0 }
        EventRegistry:RegisterCallback('Supertracking.OnChanged', function(owner, tracker)
            owner.notifications = owner.notifications + 1
            owner.tracker = tracker
        end, CF2SupertrackingObserver)
        "#,
    )
    .expect("register external observer, not a replacement API or consumer");
    let errors_before = env.state().borrow().lua_errors.len();
    env.fire_event("SUPER_TRACKING_CHANGED")
        .expect("dispatch original supertracking event");
    assert_eq!(
        env.state().borrow().lua_errors.len(),
        errors_before,
        "original CacheCurrentSuperTrackInfo must reach its notification after getter line 52"
    );
    env.exec(
        r#"
        assert(CF2SupertrackingObserver.notifications == 1,
            'original refresh publishes exactly one notification to the observer')
        local tracker = CF2SupertrackingObserver.tracker
        assert(type(tracker) == 'table', 'notification exposes the original frame cache')
        local info = debug.getinfo(tracker.CacheCurrentSuperTrackInfo, 'S')
        local suffix = 'Blizzard_FrameXMLUtil/Mainline/Blizzard_QuestSuperTracking.lua'
        local source = info.source:gsub('\\', '/')
        assert(info.what == 'Lua' and source:sub(-#suffix) == suffix)
        "#,
    )
    .expect("original supertracking refresh notification");
    // Boundary only: no live content model/setter exists to seed a populated pair.
    // No fabricated empty tuple, copied consumer, private-table seed or API override.
}
