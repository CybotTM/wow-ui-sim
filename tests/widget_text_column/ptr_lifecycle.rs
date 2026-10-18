//! PTR-only original cached lifecycle; shared assertions and concrete inputs stay unchanged.
use super::fixtures::{self, ID, SET_ID};
use crate::common;
use crate::common::blizzard_addon_harness::{
    load_blizzard_addon_closure_into_env, new_blizzard_addon_env,
};
use wow_ui_sim::lua_api::WowLuaEnv;

fn load_cached_widget_addons(env: &WowLuaEnv, cache: &std::path::Path) {
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
}

fn register_cached_container(env: &WowLuaEnv) {
    fixtures::assert_lua(
        env,
        "cached observation helpers",
        include_str!("consumer.lua"),
    );
    fixtures::assert_lua(
        env,
        "normal registration for supplied empty set",
        r#"
        assert(Enum.UIWidgetVisualizationType.TextColumnRow == 21)
        assert(Enum.WidgetShownState.Shown == 1 and Enum.WidgetShownState.Hidden == 0)
        assert(Enum.UIWidgetTextSizeType.Medium16Pt == 1 and Enum.UIWidgetTextSizeType.Standard14Pt == 4)
        assert(Enum.UIWidgetFontType.Normal == 0 and Enum.UIWidgetFontType.Shadow == 1)
        assert(UIWidgetManager:GetWidgetTypeInfo(21), 'cached provider registered')
        ColumnContainer = CreateFrame('Frame', nil, UIParent, 'UIWidgetContainerTemplate')
        ColumnContainer:SetPoint('CENTER', UIParent, 'CENTER')
        ColumnContainer:RegisterForWidgetSet(436)
        assert(ColumnContainer:IsRegisteredForWidgetSet(436))
        assert(UIWidgetManager.registeredWidgetContainers[ColumnContainer])
        assert(ColumnContainer:IsEventRegistered('UPDATE_UI_WIDGET'))
        assert(ColumnContainer:IsEventRegistered('UPDATE_ALL_UI_WIDGETS'))
        assert(next(ColumnContainer.widgetFrames) == nil)
    "#,
    );
}

fn flush_counts(env: &WowLuaEnv, count: usize) {
    env.fire_on_update(0.0)
        .expect("actual cached deferred layout OnUpdate");
    fixtures::assert_lua(
        env,
        "post-OnUpdate actual pool and shown counts",
        &format!(
            r#"
        assert(not ColumnContainer.dirtyLayout)
        assert(ColumnContainer.widgetPools:GetNumActive() == {count})
        assert(ColumnContainer:GetNumWidgetsShowing() == {count})
        assert(ColumnContainer.numTimers == 0 and ColumnContainer.ticker == nil)
    "#
        ),
    );
}

fn activate_cached_row(env: &WowLuaEnv) {
    fixtures::publish(env, fixtures::supplied());
    flush_counts(env, 1);
    fixtures::assert_lua(
        env,
        "normal activation of nested pooled entries",
        r#"
        ColumnFrame = assert(ColumnContainer.widgetFrames[763621], 'cached template acquired')
        ColumnOldEntries = ColumnAssertFrame(ColumnFrame, ColumnExpected, SystemFont_Large, 'column-731')
        ColumnOneSnapshot(ColumnExpected, ColumnGet(763621))
    "#,
    );
}

fn update_cached_row(env: &WowLuaEnv) {
    fixtures::publish(env, fixtures::consumer_update());
    flush_counts(env, 1);
    fixtures::assert_lua(
        env,
        "incremental replacement shrinks entry pool from three to two",
        r#"
        assert(ColumnContainer.widgetFrames[763621] == ColumnFrame, 'same active row frame')
        local active = ColumnAssertFrame(ColumnFrame, ColumnConsumerExpected, SystemFont_Shadow_Med3, 'column-731')
        local members = {}
        for _, entry in ipairs(active) do members[entry] = true end
        local released = 0
        for _, old in ipairs(ColumnOldEntries) do
            if not members[old] then
                assert(not old:IsShown(), 'unreacquired old child hidden')
                released = released + 1
            end
        end
        assert(released >= 1, 'at least one old child released')
        ColumnOneSnapshot(ColumnConsumerExpected, ColumnGet(763621))
        ColumnSnapshot(ColumnRetained, ColumnExpected)
    "#,
    );
}

fn hide_and_restore(env: &WowLuaEnv) {
    let mut record = fixtures::consumer_update();
    record.shown_state = 0;
    fixtures::publish(env, record);
    flush_counts(env, 0);
    fixtures::assert_lua(
        env,
        "Hidden adapter removes actual frame and nested active children",
        r#"
        assert(ColumnContainer.widgetFrames[763621] == nil and not ColumnFrame:IsShown())
        assert(ColumnFrame.entryPool:GetNumActive() == 0)
        assert(#C_UIWidgetManager.GetAllWidgetsBySetID(436) == 1)
        local expected = {}
        for key, value in pairs(ColumnConsumerExpected) do expected[key] = value end
        expected.shownState = 0
        ColumnOneSnapshot(expected, ColumnGet(763621))
    "#,
    );
    fixtures::publish(env, fixtures::consumer_update());
    flush_counts(env, 1);
    fixtures::assert_lua(
        env,
        "Shown reacquires unchanged cached nested template",
        r#"
        ColumnRestored = assert(ColumnContainer.widgetFrames[763621])
        ColumnAssertFrame(ColumnRestored, ColumnConsumerExpected, SystemFont_Shadow_Med3, 'column-947')
        ColumnSnapshot(ColumnRetained, ColumnExpected)
    "#,
    );
}

fn remove_widget_membership(env: &WowLuaEnv) {
    assert!(env.remove_ui_widget(SET_ID, ID).unwrap());
    flush_counts(env, 0);
    fixtures::assert_lua(
        env,
        "full feed removal releases row and nested pool but retains record",
        r#"
        assert(next(ColumnContainer.widgetFrames) == nil and not ColumnRestored:IsShown())
        assert(ColumnRestored.entryPool:GetNumActive() == 0)
        assert(#C_UIWidgetManager.GetAllWidgetsBySetID(436) == 0)
        ColumnOneSnapshot(ColumnConsumerExpected, ColumnGet(763621))
    "#,
    );
}

fn unregister_live_container(env: &WowLuaEnv) {
    fixtures::publish(env, fixtures::consumer_update());
    flush_counts(env, 1);
    fixtures::assert_lua(
        env,
        "unregister a live container, not an already empty one",
        r#"
        ColumnCleanup = assert(ColumnContainer.widgetFrames[763621])
        ColumnAssertFrame(ColumnCleanup, ColumnConsumerExpected, SystemFont_Shadow_Med3, 'column-947')
        ColumnContainer:UnregisterForWidgetSet()
        assert(not ColumnContainer:IsRegisteredForWidgetSet(436))
        assert(not UIWidgetManager.registeredWidgetContainers[ColumnContainer])
        assert(not ColumnContainer:IsEventRegistered('UPDATE_UI_WIDGET'))
        assert(not ColumnContainer:IsEventRegistered('UPDATE_ALL_UI_WIDGETS'))
        assert(next(ColumnContainer.widgetFrames) == nil and not ColumnCleanup:IsShown())
        assert(ColumnCleanup.entryPool:GetNumActive() == 0)
        assert(ColumnContainer.widgetPools:GetNumActive() == 0)
        assert(not ColumnContainer:IsShown(), 'unregistered container hidden')
        assert(ColumnContainer:GetNumWidgetsShowing() == 0, 'unregistered showing count reset')
    "#,
    );
    fixtures::publish(env, fixtures::supplied());
    env.fire_on_update(0.0)
        .expect("post-unregister normal OnUpdate");
    fixtures::assert_lua(
        env,
        "post-unregister feed cannot reacquire consumer",
        r#"
        assert(next(ColumnContainer.widgetFrames) == nil)
        assert(ColumnContainer.widgetPools:GetNumActive() == 0)
        assert(not ColumnContainer:IsShown())
        assert(ColumnContainer:GetNumWidgetsShowing() == 0)
        assert(ColumnCleanup.entryPool:GetNumActive() == 0)
        ColumnOneSnapshot(ColumnExpected, ColumnGet(763621))
    "#,
    );
}

/// Exercise an already-loaded original consumer, including an ordinary-startup env.
pub(super) fn observe_preloaded(env: &WowLuaEnv) {
    fixtures::seed(env, ID, fixtures::supplied());
    fixtures::populated_boundary(env);
    fixtures::supply_empty_set(env);
    register_cached_container(env);
    flush_counts(env, 0);
    activate_cached_row(env);
    update_cached_row(env);
    hide_and_restore(env);
    remove_widget_membership(env);
    unregister_live_container(env);
}

#[test]
fn ptr_cached_register_update_remove_and_live_unregister() {
    common::with_timeout(88, || {
        assert_eq!(
            wow_ui_sim::client_profile::ACTIVE,
            wow_ui_sim::client_profile::ClientProfile::Ptr
        );
        let cache =
            wow_ui_sim::paths::default_blizzard_ui_addons_path().expect("resolve active PTR cache");
        assert!(
            cache.ends_with("ptr/AddOns"),
            "wrong cache: {}",
            cache.display()
        );
        let env = new_blizzard_addon_env(&cache);
        load_cached_widget_addons(&env, &cache);
        observe_preloaded(&env);
    });
}
