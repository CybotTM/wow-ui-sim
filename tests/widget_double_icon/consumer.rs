//! Actual cached type5 provider/template through normal synchronous host feeds.
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
        FixtureFrame = assert(FixtureContainer.widgetFrames[762505])
        assert(FixtureFrame.widgetID == 762505 and FixtureFrame.widgetSetID == 439)
        assert(FixtureFrame.widgetType == 5)
        AssertDoubleIconInitial(FixtureFrame)
        AssertSuppliedSnapshot(FixtureGetter(762505))
    "#,
    );
    fixtures::supply(env, fixtures::updated());
    flush_counts(env, 1);
    assert_updated_frame_and_snapshot(env);
}

fn assert_updated_frame_and_snapshot(env: &WowLuaEnv) {
    assert_lua(
        env,
        "in-place update of label and both sides",
        r#"
        assert(FixtureContainer.widgetFrames[762505] == FixtureFrame, 'updated frame identity')
        AssertDoubleIconUpdated(FixtureFrame, 'doubleicon-contract-731')
        AssertSuppliedSnapshot(SavedDoubleIconSnapshot)
        local current = FixtureGetter(762505)
        assert(current.label == 'Updated label 947' and current.leftText == 'Updated left 947'
        and current.rightText == 'Updated right 947')
        assert(current.leftTooltip == 'Updated left tooltip 947')
        assert(current.rightTooltip == 'Updated right tooltip 947')
        assert(current.widgetTag == 'updated-doubleicon-tag', 'getter projects the replacement tag')
        local previousTagFrames, replacementTagFrames = {}, {}
        FixtureContainer:GatherWidgetsByWidgetTag(previousTagFrames, 'doubleicon-contract-731')
        FixtureContainer:GatherWidgetsByWidgetTag(replacementTagFrames, 'updated-doubleicon-tag')
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
        assert(FixtureContainer.widgetFrames[762505] == nil and not FixtureFrame:IsShown())
        assert(#C_UIWidgetManager.GetAllWidgetsBySetID(439) == 1)
        assert(FixtureGetter(762505).shownState == 0)
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
        RestoredFrame = assert(FixtureContainer.widgetFrames[762505])
        AssertDoubleIconUpdated(RestoredFrame, 'updated-doubleicon-tag')
        local replacementTagFrames = {}
        FixtureContainer:GatherWidgetsByWidgetTag(replacementTagFrames, 'updated-doubleicon-tag')
        assert(#replacementTagFrames == 1 and replacementTagFrames[1] == RestoredFrame)
        assert(FixtureGetter(762505).shownState == 1)
        AssertSuppliedSnapshot(SavedDoubleIconSnapshot)
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
        local current = FixtureGetter(762505)
        assert(current.label == 'Updated label 947' and current.leftText == 'Updated left 947'
        and current.rightText == 'Updated right 947')
        AssertSuppliedSnapshot(SavedDoubleIconSnapshot)
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
        CleanupFrame = assert(FixtureContainer.widgetFrames[762505])
        AssertDoubleIconUpdated(CleanupFrame, 'updated-doubleicon-tag')
        FixtureContainer:UnregisterForWidgetSet()
        assert(not FixtureContainer:IsRegisteredForWidgetSet(439))
        assert(not UIWidgetManager.registeredWidgetContainers[FixtureContainer])
        assert(not FixtureContainer:IsEventRegistered('UPDATE_UI_WIDGET'))
        assert(not FixtureContainer:IsEventRegistered('UPDATE_ALL_UI_WIDGETS'))
        assert(next(FixtureContainer.widgetFrames) == nil and not CleanupFrame:IsShown())
        assert(FixtureContainer.widgetPools:GetNumActive() == 0)
        assert(FixtureContainer.numTimers == 0 and FixtureContainer.ticker == nil)
        assert(#C_UIWidgetManager.GetAllWidgetsBySetID(439) == 1)
        assert(FixtureGetter(762505).rightText == 'Updated right 947')
    "#,
    );
}

fn assert_unregistered_container_stays_inactive(env: &WowLuaEnv) {
    fixtures::supply(env, fixtures::supplied());
    flush_counts(env, 0);
    assert_lua(
        env,
        "later normal feed does not reactivate unregistered consumer",
        r#"
        assert(next(FixtureContainer.widgetFrames) == nil and not CleanupFrame:IsShown())
        assert(not UIWidgetManager.registeredWidgetContainers[FixtureContainer])
        assert(FixtureGetter(762505).label == 'Double label 731')
        "#,
    );
    assert!(env.remove_ui_widget(SET_ID, ID).unwrap());
    flush_counts(env, 0);
}

#[test]
fn double_icon_and_text_cached_type5_normal_activation_update_hidden_shown_remove_unregister() {
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
            SavedDoubleIconSnapshot = AssertSuppliedSnapshot(FixtureGetter(762505))
        "#,
        );
        fixtures::supply_empty_set(&env);
        load_cached_consumer(&env, &cache);
        activate_and_update(&env);
        hide_widget(&env);
        reacquire_widget(&env);
        remove_membership(&env);
        unregister_live_container(&env);
        assert_unregistered_container_stays_inactive(&env);
    });
}
