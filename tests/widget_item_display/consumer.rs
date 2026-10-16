//! Actual cached type27 + XML BaseItem, existing synchronous item model only.
use super::fixtures::{self, ID, SET_ID};
use super::helpers::{assert_lua, flush_counts};
use crate::common;
use crate::common::blizzard_addon_harness::{
    load_blizzard_addon_closure_into_env, new_blizzard_addon_env,
};
use wow_ui_sim::c_api::ui_widget_manager::WidgetSetRecord;
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
            "Blizzard_Colors",
            "Blizzard_ObjectAPI",
            "Blizzard_UIWidgets",
        ],
        &[],
    );
    assert!(loaded.iter().any(|name| name == "Blizzard_ObjectAPI"));
    assert!(loaded.iter().any(|name| name == "Blizzard_UIWidgets"));
    assert_lua(
        env,
        "existing item metadata and synchronous completed load",
        r#"
        local name, _, quality, _, _, _, _, _, _, texture = C_Item.GetItemInfo(6948)
        assert(name == 'Hearthstone' and quality == 1 and texture == 134414)
        assert(C_Item.IsItemDataCachedByID(6948))
        local completed = false
        Item:CreateFromItemID(6948):ContinueOnItemLoad(function() completed = true end)
        assert(completed, 'existing cached Item load completes synchronously')
    "#,
    );
    assert_lua(
        env,
        "observational child/lifecycle assertions",
        include_str!("consumer_assertions.lua"),
    );
    assert_lua(
        env,
        "actual cached registration",
        include_str!("register.lua"),
    );
    flush_counts(env, 0);
}

fn activate_and_update(env: &WowLuaEnv) {
    fixtures::supply(env, fixtures::supplied());
    // Check completion before any OnUpdate layout flush, not a deferred provider.
    assert_lua(
        env,
        "synchronous cached Setup completes inside feed",
        r#"
        FixtureFrame = assert(FixtureContainer.widgetFrames[934227])
        FixtureChild = AssertCompletedItem(FixtureFrame, ItemExpected)
        FixtureIcon = FixtureChild.Icon
        AssertLiveItemColorCallback(FixtureChild)
        AssertItemAt(934227, ItemExpected)
    "#,
    );
    flush_counts(env, 1);
    fixtures::supply(env, fixtures::updated());
    assert_lua(
        env,
        "actual in-place update preserves XML child/texture identity",
        r#"
        assert(FixtureContainer.widgetFrames[934227] == FixtureFrame)
        assert(AssertCompletedItem(FixtureFrame, ItemUpdated) == FixtureChild)
        assert(FixtureChild.Icon == FixtureIcon)
        AssertItemAt(934227, ItemUpdated)
        AssertSuppliedSnapshot(SavedItemSnapshot)
    "#,
    );
    flush_counts(env, 1);
}

fn nullable_update(env: &WowLuaEnv) {
    fixtures::supply(env, fixtures::no_optionals());
    assert_lua(
        env,
        "None uses existing item name and hides absent info text",
        r#"
        assert(FixtureContainer.widgetFrames[934227] == FixtureFrame)
        assert(AssertCompletedItem(FixtureFrame, ItemNoOptionals) == FixtureChild)
        assert(FixtureChild.Icon == FixtureIcon)
        AssertItemAt(934227, ItemNoOptionals)
    "#,
    );
    flush_counts(env, 1);
    fixtures::supply(env, fixtures::updated());
    assert_lua(
        env,
        "restore synchronous supplied text",
        "AssertCompletedItem(FixtureFrame, ItemUpdated)",
    );
    flush_counts(env, 1);
}

fn hide_and_reacquire(env: &WowLuaEnv) {
    let mut hidden = fixtures::updated();
    hidden.shown_state = 0;
    fixtures::supply(env, hidden);
    flush_counts(env, 0);
    assert_lua(
        env,
        "Hidden releases live widget and child color callback",
        r#"
        assert(FixtureContainer.widgetFrames[934227] == nil)
        AssertReleasedItem(FixtureFrame)
        assert(#C_UIWidgetManager.GetAllWidgetsBySetID(467) == 1)
        local hidden = CopyItemExpected(ItemUpdated)
        hidden.shownState = 0
        AssertItemAt(934227, hidden)
    "#,
    );
    fixtures::supply(env, fixtures::updated());
    flush_counts(env, 1);
    assert_lua(
        env,
        "Shown reacquires completed actual ItemDisplay",
        r#"
        RestoredFrame = assert(FixtureContainer.widgetFrames[934227])
        AssertLiveItemColorCallback(AssertCompletedItem(RestoredFrame, ItemUpdated))
        AssertItemAt(934227, ItemUpdated)
    "#,
    );
}

fn remove_membership_and_replace_set(env: &WowLuaEnv) {
    assert!(env.remove_ui_widget(SET_ID, ID).unwrap());
    flush_counts(env, 0);
    assert_lua(
        env,
        "membership removal resets child but retains getter state",
        r#"
        assert(next(FixtureContainer.widgetFrames) == nil)
        assert(#C_UIWidgetManager.GetAllWidgetsBySetID(467) == 0)
        AssertReleasedItem(RestoredFrame)
        AssertItemAt(934227, ItemUpdated)
    "#,
    );
    fixtures::supply(env, fixtures::updated());
    flush_counts(env, 1);
    assert_lua(
        env,
        "retain live frame before full set replacement",
        r#"
        FullSetFrame = assert(FixtureContainer.widgetFrames[934227])
        AssertCompletedItem(FullSetFrame, ItemUpdated)
    "#,
    );
    env.replace_ui_widget_set(
        SET_ID,
        Some(WidgetSetRecord {
            layout_direction: 0,
            vertical_padding: 7.0,
            widgets: Vec::new(),
        }),
    )
    .expect("full-set replacement drops live membership");
    flush_counts(env, 0);
    assert_lua(
        env,
        "full set replacement releases actual consumer and retains record",
        r#"
        assert(next(FixtureContainer.widgetFrames) == nil)
        assert(#C_UIWidgetManager.GetAllWidgetsBySetID(467) == 0)
        AssertReleasedItem(FullSetFrame)
        AssertItemAt(934227, ItemUpdated)
        AssertSuppliedSnapshot(SavedItemSnapshot)
    "#,
    );
}

fn unregister_live_consumer(env: &WowLuaEnv) {
    fixtures::supply(env, fixtures::updated());
    flush_counts(env, 1);
    assert_lua(
        env,
        "live unregister resets child and subscriptions",
        r#"
        CleanupFrame = assert(FixtureContainer.widgetFrames[934227])
        AssertCompletedItem(CleanupFrame, ItemUpdated)
        FixtureContainer:UnregisterForWidgetSet()
        assert(not FixtureContainer:IsRegisteredForWidgetSet(467))
        assert(not UIWidgetManager.registeredWidgetContainers[FixtureContainer])
        assert(not FixtureContainer:IsEventRegistered('UPDATE_UI_WIDGET'))
        assert(not FixtureContainer:IsEventRegistered('UPDATE_ALL_UI_WIDGETS'))
        assert(next(FixtureContainer.widgetFrames) == nil)
        assert(FixtureContainer.widgetPools:GetNumActive() == 0)
        AssertReleasedItem(CleanupFrame)
        assert(#C_UIWidgetManager.GetAllWidgetsBySetID(467) == 1)
        AssertItemAt(934227, ItemUpdated)
    "#,
    );
    fixtures::supply(env, fixtures::supplied());
    flush_counts(env, 0);
    assert_lua(
        env,
        "later feed cannot reactivate unregistered container",
        r#"
        assert(next(FixtureContainer.widgetFrames) == nil)
        AssertReleasedItem(CleanupFrame)
        AssertItemAt(934227, ItemExpected)
        AssertSuppliedSnapshot(SavedItemSnapshot)
    "#,
    );
    assert!(env.remove_ui_widget(SET_ID, ID).unwrap());
    flush_counts(env, 0);
}

#[test]
fn item_display_cached_type27_synchronous_completed_lifecycle_and_child_cleanup() {
    common::with_timeout(88, || {
        let cache = wow_ui_sim::paths::default_blizzard_ui_addons_path()
            .expect("active Retail/PTR profile cache");
        let env = new_blizzard_addon_env(&cache);
        fixtures::seed(&env, ID, fixtures::supplied());
        fixtures::install_assertions(&env);
        // Missing getter must fail here, BEFORE cached setup/load dependencies.
        assert_lua(
            &env,
            "populated getter precondition before cached setup",
            r#"
            SavedItemSnapshot = AssertItemAt(934227, ItemExpected)
        "#,
        );
        fixtures::supply_empty_set(&env);
        load_cached_consumer(&env, &cache);
        activate_and_update(&env);
        nullable_update(&env);
        hide_and_reacquire(&env);
        remove_membership_and_replace_set(&env);
        unregister_live_consumer(&env);
    });
}
