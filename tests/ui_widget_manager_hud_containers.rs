//! Actual cached Retail HUD OnLoad consumers; no vendor overrides or fixture stubs.
#![cfg(feature = "client-retail")]

use crate::common;
use common::blizzard_addon_harness::{blizzard_ui_dir, load_blizzard_addon_closure_into_env};
use wow_ui_sim::c_api::ui_widget_manager::{
    IconAndTextRecord, WidgetId, WidgetSetEntry, WidgetSetRecord,
};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;

fn icon_record(text: &str) -> IconAndTextRecord {
    IconAndTextRecord {
        state: 1, // Shown
        text: text.into(),
        tooltip: "Supplied HUD tooltip".into(),
        dynamic_tooltip: String::new(),
        tooltip_loc: 3,
        shift_text_type: 0,
        widget_size_setting: 160.0,
        texture_kit: String::new(),
        frame_texture_kit: String::new(),
        has_timer: false,
        order_index: 10.0,
        widget_tag: "supplied-hud".into(),
        in_anim_type: 0,
        out_anim_type: 0,
        widget_scale: 0,
        layout_direction: 1,
        model_scene_layer: 0,
        scripted_animation_effect_id: 0.0,
    }
}

fn seed_before_loading(env: &WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    let inputs = &mut state.ui_widget_inputs;
    inputs.top_center_widget_set_id = 417.0;
    inputs.below_minimap_widget_set_id = 529.0;
    inputs.power_bar_widget_set_id = 641.0;
    inputs.objective_tracker_widget_set_id = 753.0;
    // Synthetic supplied identities, not native HUD fixtures.
    for (set_id, widget_id, text) in [
        (417.0, 760201.0, "Top supplied"),
        (529.0, 760202.0, "Minimap supplied"),
        (641.0, 760203.0, "Power supplied"),
        (753.0, 760204.0, "Objective supplied"),
    ] {
        inputs.sets.insert(
            WidgetId::from_number(set_id),
            WidgetSetRecord {
                layout_direction: 0,
                vertical_padding: 7.0,
                widgets: vec![WidgetSetEntry {
                    widget_id,
                    widget_type: 0, // IconAndText
                    unit_token: None,
                }],
            },
        );
        inputs
            .icon_and_text
            .insert(WidgetId::from_number(widget_id), icon_record(text));
    }
}

fn assert_selection_queries(env: &WowLuaEnv, expected: [i32; 4]) {
    let actual: (i32, i32, i32, i32) = env
        .eval(
            r#"
            return C_UIWidgetManager.GetTopCenterWidgetSetID(),
                C_UIWidgetManager.GetBelowMinimapWidgetSetID(),
                C_UIWidgetManager.GetPowerBarWidgetSetID(),
                C_UIWidgetManager.GetObjectiveTrackerWidgetSetID()
            "#,
        )
        .expect("HUD selection queries must return numbers before cached addon loading");
    assert_eq!(actual, (expected[0], expected[1], expected[2], expected[3]));
}

fn load_standard_containers(env: &WowLuaEnv) {
    // Query assertions precede even cache discovery: cache failures are prerequisites,
    // not evidence of a failing selector or OnLoad consumer.
    let ui_dir = blizzard_ui_dir();
    for name in ["Blizzard_UIWidgets", "Blizzard_ObjectiveTracker"] {
        assert!(
            ui_dir.join(name).is_dir(),
            "cached-addon prerequisite missing: {}",
            ui_dir.join(name).display()
        );
    }
    // Same environment setup as the dependency-closure harness, after query proof.
    env.set_screen_size(1024.0, 768.0);
    env.set_screen_mode(ScreenKind::Game);
    env.state().borrow_mut().addon_base_paths = vec![ui_dir.clone()];
    // Empty overrides: no patched manager, providers, templates, or OnLoad.
    let loaded = load_blizzard_addon_closure_into_env(
        env,
        &ui_dir,
        &[
            "Blizzard_SharedXMLGame",
            "Blizzard_Fonts_Shared",
            "Blizzard_FrameEffects",
            "Blizzard_UIWidgets",
            "Blizzard_ObjectiveTracker",
        ],
        &[],
    );
    for name in ["Blizzard_UIWidgets", "Blizzard_ObjectiveTracker"] {
        assert!(
            loaded.iter().any(|loaded_name| loaded_name == name),
            "cached dependency closure did not load {name}"
        );
    }
    assert!(
        env.state().borrow().lua_errors.is_empty(),
        "cached-load prerequisite errors (not selector RED): {:?}",
        env.state().borrow().lua_errors
    );
}

#[test]
fn cached_standard_hud_onload_consumes_preseeded_selection_and_records() {
    common::with_timeout(88, || {
        wow_ui_sim::xml::register_intrinsic_templates();
        let env = WowLuaEnv::new().expect("failed to create Lua environment");
        seed_before_loading(&env);
        assert_selection_queries(&env, [417, 529, 641, 753]);
        load_standard_containers(&env);
        env.exec(r#"
            local cases = {
                {'UIWidgetTopCenterContainerFrame', 417, 760201, 'Top supplied'},
                {'UIWidgetBelowMinimapContainerFrame', 529, 760202, 'Minimap supplied'},
                {'UIWidgetPowerBarContainerFrame', 641, 760203, 'Power supplied'},
                {'ObjectiveTrackerUIWidgetContainer', 753, 760204, 'Objective supplied'},
            }
            for _, case in ipairs(cases) do
                local container = assert(_G[case[1]], case[1] .. ' missing')
                assert(container:IsRegisteredForWidgetSet(case[2]), case[1] .. ' selected wrong set')
                local widget = assert(container.widgetFrames[case[3]], case[1] .. ' record not consumed')
                assert(widget.Text:GetText() == case[4], case[1] .. ' supplied text not consumed')
                for _, other in ipairs(cases) do
                    if other[3] ~= case[3] then
                        assert(container.widgetFrames[other[3]] == nil, 'cross-set consumer leakage')
                    end
                end
            end
        "#).unwrap();
        assert!(env.state().borrow().lua_errors.is_empty());
        // No manual RegisterForWidgetSet, OnLoad replay, update event or show.
        // No IsVisible assertion: power starts hidden; objective has alpha0.
    });
}

#[test]
fn cached_standard_hud_default_zero_leaves_no_active_widget_set() {
    common::with_timeout(88, || {
        wow_ui_sim::xml::register_intrinsic_templates();
        let env = WowLuaEnv::new().expect("failed to create Lua environment");
        // Preserve initialized state. No set0 or HUD records supplied.
        assert_selection_queries(&env, [0, 0, 0, 0]);
        load_standard_containers(&env);
        env.exec(
            r#"
            local names = {
                'UIWidgetTopCenterContainerFrame',
                'UIWidgetBelowMinimapContainerFrame',
                'UIWidgetPowerBarContainerFrame',
                'ObjectiveTrackerUIWidgetContainer',
            }
            for _, name in ipairs(names) do
                local container = assert(_G[name], name .. ' missing')
                assert(not container:IsRegisteredForWidgetSet(), name .. ' activated default HUD')
                assert(next(container.widgetFrames) == nil, name .. ' consumed unsupplied records')
            end
        "#,
        )
        .unwrap();
        assert!(env.state().borrow().lua_errors.is_empty());
    });
}
