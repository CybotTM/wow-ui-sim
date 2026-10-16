//! Grounding (Retail/PTR AddOns/Blizzard_UIWidgets/):
//! Blizzard_UIWidgetTemplateHorizontalCurrencies.lua:1-63;
//! Blizzard_UIWidgetTemplateHorizontalCurrencies.xml:3-5;
//! Blizzard_UIWidgetManager.lua:238-322,390-570.
//! Actual cached type9 provider/template through normal synchronous host feeds.
use super::fixtures::{self, ID, SET_ID};
use super::helpers::{assert_lua, flush_counts};
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
        FixtureFrame = assert(FixtureContainer.widgetFrames[934209])
        assert(FixtureFrame.widgetID == 934209 and FixtureFrame.widgetSetID == 449)
        assert(FixtureFrame.widgetType == 9)
        AssertHorizontalInitial(FixtureFrame)
        AssertSuppliedSnapshot(FixtureGetter(934209))
    "#,
    );
    fixtures::supply(env, fixtures::updated());
    flush_counts(env, 1);
    fixtures::expect(env, "CurrencyUpdated");
    assert_updated_frame_and_snapshot(env);
}

fn assert_updated_frame_and_snapshot(env: &WowLuaEnv) {
    assert_lua(
        env,
        "in-place text/kit update through cached Setup",
        r#"
        assert(FixtureContainer.widgetFrames[934209] == FixtureFrame, 'updated frame identity')
        AssertHorizontalUpdated(FixtureFrame, 'horizontalcurrencies-contract-731')
        local currentExpected = FixtureExpected
        FixtureExpected = SavedHorizontalExpected
        AssertSuppliedSnapshot(SavedHorizontalSnapshot)
        FixtureExpected = currentExpected
        local current = FixtureGetter(934209)
        AssertSuppliedSnapshot(current)
        assert(current.widgetTag == 'updated-horizontalcurrencies-tag', 'getter projects the replacement tag')
        local previousTagFrames, replacementTagFrames = {}, {}
        FixtureContainer:GatherWidgetsByWidgetTag(previousTagFrames, 'horizontalcurrencies-contract-731')
        FixtureContainer:GatherWidgetsByWidgetTag(replacementTagFrames, 'updated-horizontalcurrencies-tag')
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
        assert(FixtureContainer.widgetFrames[934209] == nil and not FixtureFrame:IsShown())
        assert(FixtureFrame.currencyPool:GetNumActive() == 0, "hidden releases nested rows")
        assert(#C_UIWidgetManager.GetAllWidgetsBySetID(449) == 1)
        assert(FixtureGetter(934209).shownState == 0)
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
        RestoredFrame = assert(FixtureContainer.widgetFrames[934209])
        AssertHorizontalUpdated(RestoredFrame, 'updated-horizontalcurrencies-tag')
        local replacementTagFrames = {}
        FixtureContainer:GatherWidgetsByWidgetTag(replacementTagFrames, 'updated-horizontalcurrencies-tag')
        assert(#replacementTagFrames == 1 and replacementTagFrames[1] == RestoredFrame)
        assert(FixtureGetter(934209).shownState == 1)
        local currentExpected = FixtureExpected
        FixtureExpected = SavedHorizontalExpected
        AssertSuppliedSnapshot(SavedHorizontalSnapshot)
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
        assert(#C_UIWidgetManager.GetAllWidgetsBySetID(449) == 0)
        assert(next(FixtureContainer.widgetFrames) == nil and not RestoredFrame:IsShown())
        assert(RestoredFrame.currencyPool:GetNumActive() == 0, "membership removal releases rows")
        local current = FixtureGetter(934209)
        AssertSuppliedSnapshot(current)
        local currentExpected = FixtureExpected
        FixtureExpected = SavedHorizontalExpected
        AssertSuppliedSnapshot(SavedHorizontalSnapshot)
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
        CleanupFrame = assert(FixtureContainer.widgetFrames[934209])
        AssertHorizontalUpdated(CleanupFrame, 'updated-horizontalcurrencies-tag')
        FixtureContainer:UnregisterForWidgetSet()
        assert(not FixtureContainer:IsRegisteredForWidgetSet(449))
        assert(not UIWidgetManager.registeredWidgetContainers[FixtureContainer])
        assert(not FixtureContainer:IsEventRegistered('UPDATE_UI_WIDGET'))
        assert(not FixtureContainer:IsEventRegistered('UPDATE_ALL_UI_WIDGETS'))
        assert(next(FixtureContainer.widgetFrames) == nil and not CleanupFrame:IsShown())
        assert(CleanupFrame.currencyPool:GetNumActive() == 0, "unregistered nested pool empty")
        assert(FixtureContainer.widgetPools:GetNumActive() == 0)
        assert(FixtureContainer.numTimers == 0 and FixtureContainer.ticker == nil)
        assert(#C_UIWidgetManager.GetAllWidgetsBySetID(449) == 1)
        AssertSuppliedSnapshot(FixtureGetter(934209))
    "#,
    );
}

fn assert_unregistered_container_stays_inactive(env: &WowLuaEnv) {
    fixtures::expect(env, "CurrencyExpected");
    fixtures::supply(env, fixtures::supplied());
    flush_counts(env, 0);
    assert_lua(
        env,
        "later normal feed does not reactivate unregistered consumer",
        r#"
        assert(next(FixtureContainer.widgetFrames) == nil and not CleanupFrame:IsShown())
        assert(CleanupFrame.currencyPool:GetNumActive() == 0, "unregistered nested pool empty")
        assert(not UIWidgetManager.registeredWidgetContainers[FixtureContainer])
        AssertSuppliedSnapshot(FixtureGetter(934209))
        "#,
    );
    assert!(env.remove_ui_widget(SET_ID, ID).unwrap());
    flush_counts(env, 0);
}

fn empty_and_width_updates(env: &WowLuaEnv) {
    assert_lua(
        env,
        "retain rows before empty update",
        "OldCurrencyRows = AssertHorizontalUpdated(FixtureFrame, 'horizontalcurrencies-contract-731')",
    );
    let mut empty = fixtures::updated();
    empty.currencies.clear();
    fixtures::supply(env, empty);
    flush_counts(env, 1);
    assert_lua(
        env,
        "empty update releases rows and computes supplied-size branch",
        r#"
        EmptyExpected = CopyCurrencyExpected(CurrencyUpdated)
        EmptyExpected.currencies = {}
        assert(FixtureContainer.widgetFrames[934209] == FixtureFrame)
        AssertHorizontalConsumer(FixtureFrame, EmptyExpected, 'horizontalcurrencies-contract-731')
        AssertCurrencyAt(934209, EmptyExpected)
        for _, row in ipairs(OldCurrencyRows) do assert(not row:IsShown()) end
        assert(math.abs(FixtureFrame:GetWidth() - 90.25) < .0001)
        assert(FixtureFrame:GetHeight() == 0)
        "#,
    );
    assert_populated_width_branches(env);
    fixtures::supply(env, fixtures::updated());
    flush_counts(env, 1);
    assert_lua(
        env,
        "restore normal supplied record",
        "AssertHorizontalUpdated(FixtureFrame, 'horizontalcurrencies-contract-731')",
    );
}

fn assert_populated_width_branches(env: &WowLuaEnv) {
    // Two concrete size settings exercise content and supplied-size branches.
    for size in [0.0, 10000.0] {
        let mut record = fixtures::updated();
        record.widget_size_setting = size;
        fixtures::supply(env, record);
        flush_counts(env, 1);
        assert_lua(
            env,
            "populated rows compute cached width branch",
            &format!(
                r#"
                local expected = CopyCurrencyExpected(CurrencyUpdated)
                expected.widgetSizeSetting = {size}
                AssertHorizontalConsumer(FixtureFrame, expected, 'horizontalcurrencies-contract-731')
                AssertCurrencyAt(934209, expected)
                "#,
            ),
        );
    }
}

#[test]
fn horizontal_currencies_cached_type9_normal_activation_update_hidden_shown_remove_unregister() {
    common::with_timeout(88, || {
        let cache = wow_ui_sim::paths::default_blizzard_ui_addons_path()
            .expect("active profile cache, never unprofiled legacy tree");
        let env = new_blizzard_addon_env(&cache);
        fixtures::seed(&env, ID, fixtures::supplied());
        fixtures::install_assertions(&env);
        // Establish populated getter boundary before cached load/setup dependencies.
        assert_lua(
            &env,
            "populated contract before consumer setup",
            r#"
            SavedHorizontalSnapshot = AssertSuppliedSnapshot(FixtureGetter(934209))
            SavedHorizontalExpected = FixtureExpected
        "#,
        );
        fixtures::supply_empty_set(&env);
        load_cached_consumer(&env, &cache);
        activate_and_update(&env);
        empty_and_width_updates(&env);
        super::ordering::assert_ordering_and_set_replacement(&env);
        hide_widget(&env);
        reacquire_widget(&env);
        remove_membership(&env);
        unregister_live_container(&env);
        assert_unregistered_container_stays_inactive(&env);
    });
}
