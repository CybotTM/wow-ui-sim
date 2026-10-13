//! Genuine cached closure/container. First data assertion runs in real event callback.
use super::fixtures::{
    ID, SET_ID, assert_lua, consumer_record, install_snapshot_assertions, supply_empty_set, update,
};
use crate::common;
use common::blizzard_addon_harness::load_blizzard_addon_closure_into_env;
use wow_ui_sim::lua_api::WowLuaEnv;

fn load_cached_consumer(env: &WowLuaEnv) {
    let cache = wow_ui_sim::paths::default_blizzard_ui_addons_path()
        .expect("active Retail/PTR cache; no vendor override");
    eprintln!(
        "[bullet-cache] profile={} path={}",
        wow_ui_sim::client_profile::ACTIVE.cache_subdir(),
        cache.display()
    );
    env.set_screen_size(1024.0, 768.0);
    env.set_screen_mode(wow_ui_sim::screen::ScreenKind::Game);
    env.state().borrow_mut().addon_base_paths = vec![cache.clone()];
    wow_ui_sim::xml::register_intrinsic_templates();
    let loaded = load_blizzard_addon_closure_into_env(
        env,
        &cache,
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

fn register_empty_container_and_observer(env: &WowLuaEnv) {
    install_snapshot_assertions(env);
    assert_lua(
        env,
        "install consumer observations",
        include_str!("consumer_assertions.lua"),
    );
    assert_lua(
        env,
        "real empty cached container and callback",
        r#"
        assert(Enum.UIWidgetVisualizationType.BulletTextList == 10)
        assert(Enum.WidgetShownState.Hidden == 0 and Enum.WidgetShownState.Shown == 1)
        assert(Enum.WidgetEnabledState.Yellow == 1 and Enum.WidgetEnabledState.Disabled == 0)
        assert(Enum.WidgetAnimationType.None == 0 and Enum.UIWidgetScale.OneHundred == 0)
        assert(Enum.UIWidgetLayoutDirection.Vertical == 1 and Enum.UIWidgetModelSceneLayer.None == 0)
        assert(UIWidgetManager:GetWidgetTypeInfo(10))
        expectedBullet.widgetSizeSetting = 237
        expectedBullet.textureKit = ''
        expectedBullet.frameTextureKit = ''
        expectedBullet.hasTimer = false
        expectedBullet.inAnimType = 0
        expectedBullet.outAnimType = 0
        expectedBullet.widgetScale = 0
        expectedBullet.modelSceneLayer = 0
        expectedBullet.scriptedAnimationEffectID = 0
        expectedBullet.shownState = 0
        bulletConsumerEvents = 0
        bulletConsumerEventError = nil
        bulletExpectedEvents = 1
        BulletConsumerObserver = CreateFrame('Frame')
        BulletConsumerObserver:RegisterEvent('UPDATE_UI_WIDGET')
        BulletConsumerObserver:SetScript('OnEvent', function(_, event, info)
            if info.widgetSetID ~= 431 then return end
            bulletConsumerEvents = bulletConsumerEvents + 1
            local ok, message = pcall(function()
                assert(event == 'UPDATE_UI_WIDGET' and info.widgetID == 760401 and info.widgetType == 10)
                local row = C_UIWidgetManager.GetBulletTextListWidgetVisualizationInfo(info.widgetID)
                assert(type(row) == 'table',
                    'missing supplied BulletTextList record in real cached-consumer callback')
                AssertBulletSnapshot(row, expectedBullet)
                local entries = C_UIWidgetManager.GetAllWidgetsBySetID(431)
                assert(#entries == 1 and entries[1].widgetID == 760401 and entries[1].widgetType == 10)
                bulletConsumerSnapshot = row
            end)
            if not ok then bulletConsumerEventError = tostring(message) end
        end)
        BulletContainer = CreateFrame('Frame', nil, UIParent, 'UIWidgetContainerTemplate')
        BulletContainer:SetPoint('CENTER', UIParent, 'CENTER')
        BulletContainer:RegisterForWidgetSet(431)
        assert(BulletContainer:IsRegisteredForWidgetSet(431))
        assert(UIWidgetManager.registeredWidgetContainers[BulletContainer])
        assert(BulletContainer:IsEventRegistered('UPDATE_UI_WIDGET'))
        assert(BulletContainer:IsEventRegistered('UPDATE_ALL_UI_WIDGETS'))
        assert(next(BulletContainer.widgetFrames) == nil)
    "#,
    );
}

fn flush_and_assert_counts(env: &WowLuaEnv, count: usize) {
    env.fire_on_update(0.0)
        .expect("flush real cached layout OnUpdate");
    assert_lua(
        env,
        "cached pool/layout counts after OnUpdate",
        &format!(
            r#"
        local c = BulletContainer
        assert(not c.dirtyLayout)
        assert(c:GetNumWidgetsShowing() == {count} and c.widgetPools:GetNumActive() == {count})
        assert(c.numTimers == 0 and c.ticker == nil)
    "#
        ),
    );
}

fn activate_and_replace_lines(env: &WowLuaEnv) {
    let mut record = consumer_record();
    record.shown_state = 0;
    update(env, record);
    assert_lua(
        env,
        "first positive-data callback RED before pooled consumer assertions",
        r#"
        AssertBulletCallback()
        assert(BulletContainer.widgetFrames[760401] == nil, 'hidden adapter suppresses acquisition')
        expectedBullet.shownState = 1
        bulletExpectedEvents = 2
    "#,
    );
    flush_and_assert_counts(env, 0);
    update(env, consumer_record());
    assert_lua(
        env,
        "real shown pooled-line template",
        r#"
        AssertBulletCallback()
        BulletFrame = assert(BulletContainer.widgetFrames[760401], 'real cached template acquired')
        oldBulletLines = AssertBulletLines(BulletFrame, {'first', '', 'third'}, 1)
        oldBulletSnapshot = bulletConsumerSnapshot
        expectedBullet.lines = {'replacement A', 'replacement B'}
        expectedBullet.enabledState = 0
        bulletExpectedEvents = 3
    "#,
    );
    flush_and_assert_counts(env, 1);
    replace_lines(env);
}

fn replace_lines(env: &WowLuaEnv) {
    let mut replacement = consumer_record();
    replacement.lines = vec!["replacement A".into(), "replacement B".into()];
    replacement.enabled_state = 0;
    update(env, replacement);
    assert_lua(
        env,
        "incremental Setup releases and replaces line pool",
        r#"
        AssertBulletCallback()
        assert(BulletContainer.widgetFrames[760401] == BulletFrame)
        local active = AssertBulletLines(BulletFrame, {'replacement A', 'replacement B'}, 0)
        local members = {}
        for _, line in ipairs(active) do members[line] = true end
        local inactiveCount = 0
        for _, line in ipairs(oldBulletLines) do
            if not members[line] then
                assert(not line:IsShown(), 'unreacquired old line hidden')
                inactiveCount = inactiveCount + 1
            end
        end
        assert(inactiveCount >= 1, 'three to two lines releases an old frame')
        assert(oldBulletSnapshot.enabledState == 1 and #oldBulletSnapshot.lines == 3)
        assert(oldBulletSnapshot.lines[1] == 'first' and oldBulletSnapshot.lines[2] == ''
            and oldBulletSnapshot.lines[3] == 'third', 'retained snapshot isolation')
    "#,
    );
    flush_and_assert_counts(env, 1);
}

fn hide_restore_and_remove(env: &WowLuaEnv) {
    assert_lua(
        env,
        "expect hide callback",
        r#"
        expectedBullet.shownState = 0
        expectedBullet.lines = {'first', '', 'third'}
        expectedBullet.enabledState = 1
        bulletExpectedEvents = 4
    "#,
    );
    let mut hidden = consumer_record();
    hidden.shown_state = 0;
    update(env, hidden);
    assert_lua(
        env,
        "cached hidden adapter resets widget and lines",
        r#"
        AssertBulletCallback()
        assert(BulletContainer.widgetFrames[760401] == nil)
        assert(BulletFrame.linePool:GetNumActive() == 0)
        for _, line in ipairs(oldBulletLines) do assert(not line:IsShown()) end
        expectedBullet.shownState = 1
        bulletExpectedEvents = 5
    "#,
    );
    flush_and_assert_counts(env, 0);
    update(env, consumer_record());
    assert_lua(
        env,
        "restore shown template",
        r#"
        AssertBulletCallback()
        BulletRestored = assert(BulletContainer.widgetFrames[760401])
        restoredBulletLines = AssertBulletLines(BulletRestored, {'first', '', 'third'}, 1)
    "#,
    );
    flush_and_assert_counts(env, 1);
    remove_and_unregister(env);
}

fn remove_and_unregister(env: &WowLuaEnv) {
    assert!(
        env.remove_ui_widget(SET_ID, ID)
            .expect("real full membership reconciliation")
    );
    assert_lua(
        env,
        "membership removal resets lines but retains owned record",
        r#"
        assert(next(BulletContainer.widgetFrames) == nil)
        assert(BulletRestored.linePool:GetNumActive() == 0)
        for _, line in ipairs(restoredBulletLines) do assert(not line:IsShown()) end
        AssertBulletSnapshot(C_UIWidgetManager.GetBulletTextListWidgetVisualizationInfo(760401), expectedBullet)
        assert(bulletConsumerEvents == 5, 'membership removal uses full event, not incremental')
        bulletExpectedEvents = 6
    "#,
    );
    flush_and_assert_counts(env, 0);
    update(env, consumer_record());
    assert_lua(
        env,
        "actual unregister releases active pool and subscriptions",
        r#"
        AssertBulletCallback()
        BulletBeforeUnregister = assert(BulletContainer.widgetFrames[760401])
        BulletContainer:UnregisterForWidgetSet()
        assert(not BulletContainer:IsRegisteredForWidgetSet(431))
        assert(not UIWidgetManager.registeredWidgetContainers[BulletContainer])
        assert(not BulletContainer:IsEventRegistered('UPDATE_UI_WIDGET'))
        assert(not BulletContainer:IsEventRegistered('UPDATE_ALL_UI_WIDGETS'))
        assert(next(BulletContainer.widgetFrames) == nil)
        assert(BulletBeforeUnregister.linePool:GetNumActive() == 0)
        bulletExpectedEvents = 7
    "#,
    );
    update(env, consumer_record());
    env.fire_event("UPDATE_ALL_UI_WIDGETS")
        .expect("unregistered full event");
    flush_and_assert_counts(env, 0);
    assert_lua(
        env,
        "post-unregister no consumer reacquisition",
        r#"
        AssertBulletCallback()
        assert(next(BulletContainer.widgetFrames) == nil)
        assert(BulletContainer.horizontalRowContainerPool:GetNumActive() == 0)
    "#,
    );
}

#[test]
fn bullet_real_cached_consumer_callbacks_order_lines_replace_hide_restore_remove_unregister() {
    common::with_timeout(88, || {
        let env = WowLuaEnv::new().expect("isolated real cached bullet environment");
        supply_empty_set(&env);
        load_cached_consumer(&env);
        register_empty_container_and_observer(&env);
        activate_and_replace_lines(&env);
        hide_restore_and_remove(&env);
    });
}
