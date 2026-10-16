//! Actual cached type4 provider/template through normal synchronous host feeds.
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
        FixtureFrame = assert(FixtureContainer.widgetFrames[762504])
        assert(FixtureFrame.widgetID == 762504 and FixtureFrame.widgetSetID == 439)
        assert(FixtureFrame.widgetType == 4)
        AssertIconBackgroundInitial(FixtureFrame)
        AssertSuppliedSnapshot(FixtureGetter(762504))
    "#,
    );
    fixtures::supply(env, fixtures::updated());
    flush_counts(env, 1);
    install_snapshot_assertion(env, GETTER, fixtures::UPDATED_EXPECTED);
    assert_updated_frame_and_snapshot(env);
}

fn assert_updated_frame_and_snapshot(env: &WowLuaEnv) {
    assert_lua(
        env,
        "in-place text/kit update through cached Setup",
        r#"
        assert(FixtureContainer.widgetFrames[762504] == FixtureFrame, 'updated frame identity')
        AssertIconBackgroundUpdated(FixtureFrame, 'iconbackground-contract-731')
        local currentExpected = FixtureExpected
        FixtureExpected = SavedIconBackgroundExpected
        AssertSuppliedSnapshot(SavedIconBackgroundSnapshot)
        FixtureExpected = currentExpected
        local current = FixtureGetter(762504)
        AssertSuppliedSnapshot(current)
        assert(current.widgetTag == 'updated-iconbackground-tag', 'getter projects the replacement tag')
        local previousTagFrames, replacementTagFrames = {}, {}
        FixtureContainer:GatherWidgetsByWidgetTag(previousTagFrames, 'iconbackground-contract-731')
        FixtureContainer:GatherWidgetsByWidgetTag(replacementTagFrames, 'updated-iconbackground-tag')
        assert(#previousTagFrames == 1 and previousTagFrames[1] == FixtureFrame)
        assert(#replacementTagFrames == 0, 'in-place update retains acquisition tag')
    "#,
    );
}

fn hide_widget(env: &WowLuaEnv) {
    let mut record = fixtures::updated();
    record.shown_state = 0;
    fixtures::supply(env, record);
    flush_counts(env, 0);
    assert_lua(
        env,
        "Hidden releases actual cached frame",
        r#"
        assert(FixtureContainer.widgetFrames[762504] == nil and not FixtureFrame:IsShown())
        assert(#C_UIWidgetManager.GetAllWidgetsBySetID(439) == 1)
        assert(FixtureGetter(762504).shownState == 0)
    "#,
    );
}

fn reacquire_widget(env: &WowLuaEnv) {
    fixtures::supply(env, fixtures::updated());
    flush_counts(env, 1);
    assert_lua(
        env,
        "Shown reacquires actual cached template",
        r#"
        RestoredFrame = assert(FixtureContainer.widgetFrames[762504])
        AssertIconBackgroundUpdated(RestoredFrame, 'updated-iconbackground-tag')
        local replacementTagFrames = {}
        FixtureContainer:GatherWidgetsByWidgetTag(replacementTagFrames, 'updated-iconbackground-tag')
        assert(#replacementTagFrames == 1 and replacementTagFrames[1] == RestoredFrame)
        assert(FixtureGetter(762504).shownState == 1)
        local currentExpected = FixtureExpected
        FixtureExpected = SavedIconBackgroundExpected
        AssertSuppliedSnapshot(SavedIconBackgroundSnapshot)
        FixtureExpected = currentExpected
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
        assert(#C_UIWidgetManager.GetAllWidgetsBySetID(439) == 0)
        assert(next(FixtureContainer.widgetFrames) == nil and not RestoredFrame:IsShown())
        local current = FixtureGetter(762504)
        AssertSuppliedSnapshot(current)
        local currentExpected = FixtureExpected
        FixtureExpected = SavedIconBackgroundExpected
        AssertSuppliedSnapshot(SavedIconBackgroundSnapshot)
        FixtureExpected = currentExpected
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
        CleanupFrame = assert(FixtureContainer.widgetFrames[762504])
        AssertIconBackgroundUpdated(CleanupFrame, 'updated-iconbackground-tag')
        FixtureContainer:UnregisterForWidgetSet()
        assert(not FixtureContainer:IsRegisteredForWidgetSet(439))
        assert(not UIWidgetManager.registeredWidgetContainers[FixtureContainer])
        assert(not FixtureContainer:IsEventRegistered('UPDATE_UI_WIDGET'))
        assert(not FixtureContainer:IsEventRegistered('UPDATE_ALL_UI_WIDGETS'))
        assert(next(FixtureContainer.widgetFrames) == nil and not CleanupFrame:IsShown())
        assert(FixtureContainer.widgetPools:GetNumActive() == 0)
        assert(FixtureContainer.numTimers == 0 and FixtureContainer.ticker == nil)
        assert(#C_UIWidgetManager.GetAllWidgetsBySetID(439) == 1)
        AssertSuppliedSnapshot(FixtureGetter(762504))
    "#,
    );
}

fn assert_unregistered_container_stays_inactive(env: &WowLuaEnv) {
    install_snapshot_assertion(env, GETTER, fixtures::EXPECTED);
    fixtures::supply(env, fixtures::supplied());
    flush_counts(env, 0);
    assert_lua(
        env,
        "later normal feed does not reactivate unregistered consumer",
        r#"
        assert(next(FixtureContainer.widgetFrames) == nil and not CleanupFrame:IsShown())
        assert(not UIWidgetManager.registeredWidgetContainers[FixtureContainer])
        AssertSuppliedSnapshot(FixtureGetter(762504))
        "#,
    );
    assert!(env.remove_ui_widget(SET_ID, ID).unwrap());
    flush_counts(env, 0);
}

#[test]
fn icon_text_and_background_cached_type4_normal_activation_update_hidden_shown_remove_unregister() {
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
            SavedIconBackgroundSnapshot = AssertSuppliedSnapshot(FixtureGetter(762504))
            SavedIconBackgroundExpected = FixtureExpected
        "#,
        );
        fixtures::supply_empty_set(&env);
        load_cached_consumer(&env, &cache);
        activate_and_update(&env);
        super::ordering::assert_ordering_and_set_replacement(&env);
        hide_widget(&env);
        reacquire_widget(&env);
        remove_membership(&env);
        unregister_live_container(&env);
        assert_unregistered_container_stays_inactive(&env);
    });
}
