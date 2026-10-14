//! Real cached type22 provider/template driven only by normal host feeds.
use super::fixtures::{self, GETTER, ID, SET_ID};
use super::helpers::{assert_lua, install_snapshot_assertion};
use crate::common;
use crate::common::blizzard_addon_harness::{
    load_blizzard_addon_closure_into_env, new_blizzard_addon_env,
};
use wow_ui_sim::lua_api::WowLuaEnv;

fn assert_phase_counts(env: &WowLuaEnv, phase: &str, active: usize, showing: usize) {
    env.fire_on_update(0.0)
        .unwrap_or_else(|error| panic!("spacer [{phase}] actual OnUpdate: {error}"));
    assert_lua(
        env,
        phase,
        &format!(
            r#"
            local phase = {phase:?}
            local actualActive = FixtureContainer.widgetPools:GetNumActive()
            local actualShowing = FixtureContainer:GetNumWidgetsShowing()
            assert(actualActive == {active}, phase .. ': active expected {active}, actual ' .. tostring(actualActive))
            assert(actualShowing == {showing}, phase .. ': showing expected {showing}, actual ' .. tostring(actualShowing))
            assert(FixtureContainer.numTimers == 0,
                phase .. ': timers expected 0, actual ' .. tostring(FixtureContainer.numTimers))
            assert(FixtureContainer.ticker == nil,
                phase .. ': ticker expected nil, actual ' .. tostring(FixtureContainer.ticker))
            "#
        ),
    );
}

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
        "observational assertions",
        include_str!("consumer_assertions.lua"),
    );
    assert_lua(
        env,
        "actual cached registration",
        include_str!("register.lua"),
    );
    assert_phase_counts(env, "cached empty registration", 0, 0);
}

fn activate_and_update(env: &WowLuaEnv) {
    fixtures::supply(env, fixtures::supplied());
    assert_phase_counts(env, "normal typed activation", 1, 1);
    assert_lua(
        env,
        "normal activation",
        r#"
        FixtureFrame = assert(FixtureContainer.widgetFrames[762201])
        AssertSpacerFrame(FixtureFrame, 173.25, 41.5, 'spacer-contract-731')
        SavedSpacerSnapshot = AssertSuppliedSnapshot(FixtureGetter(762201))
    "#,
    );
    fixtures::supply(env, fixtures::updated());
    assert_phase_counts(env, "normal dimension/tag update", 1, 1);
    assert_lua(
        env,
        "normal width/height update retains acquisition tag",
        r#"
        assert(FixtureContainer.widgetFrames[762201] == FixtureFrame)
        AssertSpacerFrame(FixtureFrame, 206.25, 73.75, 'spacer-contract-731')
        AssertSpacerTags(FixtureFrame, 'spacer-contract-731', 'updated-spacer-tag')
        AssertUpdatedSpacerSnapshot()
    "#,
    );
}

fn assert_dimension_boundaries(env: &WowLuaEnv) {
    // Both Retail Clamp(value, 1, value) and PTR math.max(value, 1) agree here.
    // Crossed pairs independently exercise both axes below one; no NaN/inf policy.
    for (width, height, expected_width, expected_height) in [
        (0.0, -11.5, 1.0, 1.0),
        (-7.25, 0.0, 1.0, 1.0),
        (0.25, 1.0, 1.0, 1.0),
        (1.0, 0.5, 1.0, 1.0),
        (206.25, 73.75, 206.25, 73.75),
    ] {
        assert_dimension_boundary(env, (width, height), (expected_width, expected_height));
    }
}

fn assert_dimension_boundary(env: &WowLuaEnv, supplied: (f64, f64), expected: (f64, f64)) {
    let (width, height) = supplied;
    let (expected_width, expected_height) = expected;
    let mut record = fixtures::updated();
    record.widget_width = width;
    record.widget_height = height;
    fixtures::supply(env, record);
    assert_phase_counts(
        env,
        &format!("dimension boundary width={width}, height={height}"),
        1,
        1,
    );
    assert_lua(
        env,
        "supplied dimensions versus actual consumer bounds",
        &format!(
            r#"
            assert(FixtureContainer.widgetFrames[762201] == FixtureFrame)
            AssertSpacerFrame(FixtureFrame, {expected_width}, {expected_height}, 'spacer-contract-731')
            local current = FixtureGetter(762201)
            assert(current.widgetWidth == {width} and current.widgetHeight == {height})
            assert(current.widgetTag == 'updated-spacer-tag')
            AssertSuppliedSnapshot(SavedSpacerSnapshot)
        "#
        ),
    );
}

fn hide_and_reacquire(env: &WowLuaEnv) {
    let mut record = fixtures::updated();
    record.shown_state = 0;
    fixtures::supply(env, record);
    assert_phase_counts(env, "hidden releases acquisition", 0, 0);
    assert_lua(
        env,
        "hidden releases frame but retains membership/record",
        r#"
        assert(FixtureContainer.widgetFrames[762201] == nil and not FixtureFrame:IsShown())
        assert(#C_UIWidgetManager.GetAllWidgetsBySetID(422) == 1)
        assert(FixtureGetter(762201).shownState == 0)
        AssertSuppliedSnapshot(SavedSpacerSnapshot)
    "#,
    );
    fixtures::supply(env, fixtures::updated());
    assert_phase_counts(env, "shown reacquires current tag", 1, 1);
    assert_lua(
        env,
        "shown reacquires current tag",
        r#"
        RestoredFrame = assert(FixtureContainer.widgetFrames[762201])
        AssertSpacerFrame(RestoredFrame, 206.25, 73.75, 'updated-spacer-tag')
        AssertSpacerTags(RestoredFrame, 'updated-spacer-tag', 'spacer-contract-731')
        assert(FixtureGetter(762201).shownState == 1)
        AssertUpdatedSpacerSnapshot()
    "#,
    );
}

fn remove_membership(env: &WowLuaEnv) {
    assert!(env.remove_ui_widget(SET_ID, ID).unwrap());
    assert_phase_counts(env, "membership removal", 0, 0);
    assert_lua(
        env,
        "membership removal retains visualization",
        r#"
        assert(#C_UIWidgetManager.GetAllWidgetsBySetID(422) == 0)
        assert(next(FixtureContainer.widgetFrames) == nil and not RestoredFrame:IsShown())
        AssertUpdatedSpacerSnapshot()
    "#,
    );
}

fn unregister_live_container(env: &WowLuaEnv) {
    fixtures::supply(env, fixtures::updated());
    assert_phase_counts(env, "reactivated before live unregister", 1, 1);
    assert_lua(
        env,
        "live unregister retains host record and membership",
        r#"
        CleanupFrame = assert(FixtureContainer.widgetFrames[762201])
        AssertSpacerFrame(CleanupFrame, 206.25, 73.75, 'updated-spacer-tag')
        FixtureContainer:UnregisterForWidgetSet()
        assert(not FixtureContainer:IsRegisteredForWidgetSet(422))
        assert(not UIWidgetManager.registeredWidgetContainers[FixtureContainer])
        assert(not FixtureContainer:IsEventRegistered('UPDATE_UI_WIDGET'))
        assert(not FixtureContainer:IsEventRegistered('UPDATE_ALL_UI_WIDGETS'))
        assert(next(FixtureContainer.widgetFrames) == nil and not CleanupFrame:IsShown())
        assert(#C_UIWidgetManager.GetAllWidgetsBySetID(422) == 1)
        AssertUpdatedSpacerSnapshot()
    "#,
    );
    assert_phase_counts(env, "live unregister cleanup", 0, 0);
}

#[test]
fn spacer_cached_normal_activation_dimensions_hidden_reacquire_remove_live_unregister() {
    common::with_timeout(88, || {
        let cache = wow_ui_sim::paths::default_blizzard_ui_addons_path()
            .expect("active profile cache, never legacy vendor tree");
        let env = new_blizzard_addon_env(&cache);
        fixtures::supply_empty_set(&env);
        install_snapshot_assertion(&env, GETTER, fixtures::EXPECTED);
        // Cached registration precedes the first populated feed/query, as in production.
        load_cached_consumer(&env, &cache);
        activate_and_update(&env);
        assert_dimension_boundaries(&env);
        hide_and_reacquire(&env);
        remove_membership(&env);
        unregister_live_container(&env);
    });
}
