use super::fixtures::{
    ID, SET_ID, assert_lua, entry, install_snapshot_assertions, supplied_record, supply_empty_set,
    update,
};
use wow_ui_sim::c_api::ui_widget_manager::{UiWidgetInputs, WidgetId, WidgetVisualization};
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn bullet_feed_commits_complete_record_and_membership_before_synchronous_event() {
    let env = WowLuaEnv::new().expect("bullet feed environment");
    assert!(UiWidgetInputs::default().bullet_text_lists.is_empty());
    assert!(
        env.state()
            .borrow()
            .ui_widget_inputs
            .bullet_text_lists
            .is_empty()
    );
    supply_empty_set(&env);
    install_snapshot_assertions(&env);
    assert_lua(
        &env,
        "real event observer before host supply",
        r#"
        bulletEvents = 0
        bulletEventError = nil
        BulletObserver = CreateFrame('Frame')
        BulletObserver:RegisterEvent('UPDATE_UI_WIDGET')
        BulletObserver:SetScript('OnEvent', function(_, event, info)
            bulletEvents = bulletEvents + 1
            local ok, message = pcall(function()
                assert(event == 'UPDATE_UI_WIDGET')
                assert(info.widgetSetID == 431 and info.widgetID == 760401
                    and info.widgetType == 10)
                local row = C_UIWidgetManager.GetBulletTextListWidgetVisualizationInfo(info.widgetID)
                assert(type(row) == 'table',
                    'missing supplied BulletTextList record in real UPDATE_UI_WIDGET callback')
                AssertBulletSnapshot(row, expectedBullet)
                local entries = C_UIWidgetManager.GetAllWidgetsBySetID(431)
                assert(#entries == 1 and entries[1].widgetID == 760401
                    and entries[1].widgetType == 10)
                eventRetainedBullet = row
            end)
            if not ok then bulletEventError = tostring(message) end
        end)
    "#,
    );
    update(&env, supplied_record());
    assert_lua(
        &env,
        "first real-callback positive-data RED",
        r#"
        assert(bulletEvents == 1, 'synchronous event observed')
        assert(bulletEventError == nil, bulletEventError)
        AssertBulletSnapshot(eventRetainedBullet, expectedBullet)
        firstFeedBullet = eventRetainedBullet
        expectedBullet.lines = {'replacement A', 'replacement B'}
        expectedBullet.enabledState = 0
    "#,
    );
    let mut replacement = supplied_record();
    replacement.lines = vec!["replacement A".into(), "replacement B".into()];
    replacement.enabled_state = 0;
    update(&env, replacement);
    assert_lua(
        &env,
        "replacement callback reads new committed data",
        r#"
        assert(bulletEvents == 2 and bulletEventError == nil, bulletEventError)
        AssertBulletSnapshot(eventRetainedBullet, expectedBullet)
        assert(#C_UIWidgetManager.GetAllWidgetsBySetID(431) == 1)
        assert(eventRetainedBullet ~= firstFeedBullet and eventRetainedBullet.lines ~= firstFeedBullet.lines)
        assert(firstFeedBullet.enabledState == 1 and #firstFeedBullet.lines == 3)
        assert(firstFeedBullet.lines[1] == 'first' and firstFeedBullet.lines[2] == ''
            and firstFeedBullet.lines[3] == 'third')
    "#,
    );
}

#[test]
fn bullet_feed_rejects_missing_set_and_type_mismatch_without_state_or_events() {
    let env = WowLuaEnv::new().expect("bullet rejected feed environment");
    assert_lua(
        &env,
        "real rejection event observers",
        r#"
        bulletRejectedEvents = 0
        BulletRejectionObserver = CreateFrame('Frame')
        BulletRejectionObserver:RegisterEvent('UPDATE_UI_WIDGET')
        BulletRejectionObserver:RegisterEvent('UPDATE_ALL_UI_WIDGETS')
        BulletRejectionObserver:SetScript('OnEvent', function()
            bulletRejectedEvents = bulletRejectedEvents + 1
        end)
    "#,
    );
    let error = env
        .update_ui_widget(
            SET_ID,
            entry(),
            WidgetVisualization::BulletTextList(supplied_record()),
        )
        .unwrap_err();
    assert!(error.to_string().contains("requires a supplied set"));
    assert!(
        env.state()
            .borrow()
            .ui_widget_inputs
            .bullet_text_lists
            .is_empty()
    );
    assert!(env.state().borrow().ui_widget_inputs.sets.is_empty());
    assert_lua(
        &env,
        "missing set has no notification",
        "assert(bulletRejectedEvents == 0)",
    );
    supply_empty_set(&env);
    assert_lua(
        &env,
        "reset observation after authorized set event",
        "bulletRejectedEvents = 0",
    );
    let mut wrong_entry = entry();
    wrong_entry.widget_type = 8;
    let error = env
        .update_ui_widget(
            SET_ID,
            wrong_entry,
            WidgetVisualization::BulletTextList(supplied_record()),
        )
        .unwrap_err();
    assert!(error.to_string().contains("does not match"));
    let state = env.state().borrow();
    assert!(state.ui_widget_inputs.bullet_text_lists.is_empty());
    assert!(
        state
            .ui_widget_inputs
            .sets
            .get(&WidgetId::from_number(SET_ID))
            .expect("supplied set survives rejection")
            .widgets
            .is_empty()
    );
    drop(state);
    assert_lua(
        &env,
        "type mismatch has no notification",
        "assert(bulletRejectedEvents == 0)",
    );
    assert!(!env.remove_ui_widget(SET_ID, ID).expect("absent membership"));
}
