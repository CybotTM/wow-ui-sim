//! Actual cached type25 provider/template through normal synchronous host feeds.
use super::fixtures::{self, GETTER, ID, SET_ID};
use super::helpers::{assert_lua, flush_counts, install_snapshot_assertion};
use crate::common;
use crate::common::blizzard_addon_harness::{
    load_blizzard_addon_closure_into_env, new_blizzard_addon_env,
};
use wow_ui_sim::lua_api::WowLuaEnv;

fn load_cached_consumer(env: &WowLuaEnv, cache: &std::path::Path) {
    wow_ui_sim::xml::register_intrinsic_templates();
    let loaded = load_blizzard_addon_closure_into_env(
        env,
        cache,
        &[
            "Blizzard_SharedXMLGame",
            "Blizzard_Fonts_Shared",
            "Blizzard_FrameEffects",
            "Blizzard_UIWidgets",
        ],
        &[],
    );
    assert!(loaded.iter().any(|name| name == "Blizzard_UIWidgets"));
    assert_lua(
        env,
        "install observational assertions",
        include_str!("consumer_assertions.lua"),
    );
    assert_lua(
        env,
        "register actual cached container",
        include_str!("register.lua"),
    );
    flush_counts(env, 0);
}

fn activate_and_update(env: &WowLuaEnv) {
    fixtures::supply(env, fixtures::supplied());
    flush_counts(env, 1);
    assert_lua(
        env,
        "activation from normal feed",
        r#"
        FixtureFrame = assert(FixtureContainer.widgetFrames[762501])
        assert(FixtureFrame.widgetID == 762501 and FixtureFrame.widgetSetID == 435)
        assert(FixtureFrame.widgetType == 25)
        AssertSubtextInitial(FixtureFrame)
        AssertSuppliedSnapshot(FixtureGetter(762501))
    "#,
    );
    fixtures::supply(env, fixtures::updated());
    flush_counts(env, 1);
    assert_lua(
        env,
        "in-place update of both visible lines",
        r#"
        assert(FixtureContainer.widgetFrames[762501] == FixtureFrame)
        AssertSubtextUpdated(FixtureFrame)
        AssertSuppliedSnapshot(SavedSubtextSnapshot)
        local current = FixtureGetter(762501)
        assert(current.text == 'Updated heading 947' and current.subText == 'Updated secondary 947')
        assert(current.spacing == 7.25 and current.widgetWidth == 913.25)
    "#,
    );
}

fn hide_and_reacquire(env: &WowLuaEnv) {
    let mut record = fixtures::updated();
    record.shown_state = 0;
    fixtures::supply(env, record.clone());
    flush_counts(env, 0);
    assert_lua(
        env,
        "Hidden releases actual cached frame",
        r#"
        assert(FixtureContainer.widgetFrames[762501] == nil and not FixtureFrame:IsShown())
        assert(#C_UIWidgetManager.GetAllWidgetsBySetID(435) == 1)
        assert(FixtureGetter(762501).shownState == 0)
    "#,
    );
    record.shown_state = 1;
    fixtures::supply(env, record);
    flush_counts(env, 1);
    assert_lua(
        env,
        "Shown reacquires actual cached template",
        r#"
        RestoredFrame = assert(FixtureContainer.widgetFrames[762501])
        AssertSubtextUpdated(RestoredFrame)
        assert(FixtureGetter(762501).shownState == 1)
        AssertSuppliedSnapshot(SavedSubtextSnapshot)
    "#,
    );
}

fn remove_membership(env: &WowLuaEnv) {
    assert!(env.remove_ui_widget(SET_ID, ID).unwrap());
    flush_counts(env, 0);
    assert_lua(
        env,
        "membership removal retains record",
        r#"
        assert(#C_UIWidgetManager.GetAllWidgetsBySetID(435) == 0)
        assert(next(FixtureContainer.widgetFrames) == nil and not RestoredFrame:IsShown())
        local current = FixtureGetter(762501)
        assert(current.text == 'Updated heading 947' and current.subText == 'Updated secondary 947')
        AssertSuppliedSnapshot(SavedSubtextSnapshot)
    "#,
    );
}

fn unregister_live_container(env: &WowLuaEnv) {
    // Cleanup must release live frames, not merely an already empty pool.
    fixtures::supply(env, fixtures::updated());
    flush_counts(env, 1);
    assert_lua(
        env,
        "unregister cleans live container subscriptions and pool",
        r#"
        CleanupFrame = assert(FixtureContainer.widgetFrames[762501])
        AssertSubtextUpdated(CleanupFrame)
        FixtureContainer:UnregisterForWidgetSet()
        assert(not FixtureContainer:IsRegisteredForWidgetSet(435))
        assert(not UIWidgetManager.registeredWidgetContainers[FixtureContainer])
        assert(not FixtureContainer:IsEventRegistered('UPDATE_UI_WIDGET'))
        assert(not FixtureContainer:IsEventRegistered('UPDATE_ALL_UI_WIDGETS'))
        assert(next(FixtureContainer.widgetFrames) == nil and not CleanupFrame:IsShown())
        assert(FixtureContainer.widgetPools:GetNumActive() == 0)
        assert(FixtureContainer.numTimers == 0 and FixtureContainer.ticker == nil)
        assert(#C_UIWidgetManager.GetAllWidgetsBySetID(435) == 1)
        assert(FixtureGetter(762501).subText == 'Updated secondary 947')
    "#,
    );
}

#[test]
fn text_with_subtext_cached_type25_normal_activation_update_hidden_shown_remove_unregister() {
    common::with_timeout(88, || {
        let cache = wow_ui_sim::paths::default_blizzard_ui_addons_path()
            .expect("active profile cache, never unprofiled legacy tree");
        let env = new_blizzard_addon_env(&cache);
        fixtures::seed(&env, ID, fixtures::supplied());
        install_snapshot_assertion(&env, GETTER, fixtures::EXPECTED);
        // Establish populated getter boundary before cached load/setup dependencies.
        assert_lua(
            &env,
            "populated contract before consumer setup",
            r#"
            SavedSubtextSnapshot = AssertSuppliedSnapshot(FixtureGetter(762501))
        "#,
        );
        fixtures::supply_empty_set(&env);
        load_cached_consumer(&env, &cache);
        activate_and_update(&env);
        hide_and_reacquire(&env);
        remove_membership(&env);
        unregister_live_container(&env);
    });
}
