//! Real synchronous host events, not mock callbacks or private consumer handlers.
use super::fixtures::{self, ID, SET_ID};
use wow_ui_sim::c_api::ui_widget_manager::{WidgetId, WidgetVisualization};
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn text_column_feed_publishes_complete_state_before_normal_event_and_removes_membership() {
    let env = WowLuaEnv::new().unwrap();
    fixtures::seed(&env, ID, fixtures::supplied());
    fixtures::populated_boundary(&env);
    // Remove the query seed so the first event must observe a newly supplied record.
    env.state()
        .borrow_mut()
        .ui_widget_inputs
        .text_column_rows
        .remove(&WidgetId::from_number(ID));
    fixtures::supply_empty_set(&env);
    fixtures::assert_lua(
        &env,
        "normal event observer",
        r#"
        ColumnEvents = 0; ColumnAllEvents = 0; ColumnEventError = nil
        ColumnEventExpected = ColumnExpected
        local observer = CreateFrame('Frame')
        observer:RegisterEvent('UPDATE_UI_WIDGET')
        observer:RegisterEvent('UPDATE_ALL_UI_WIDGETS')
        observer:SetScript('OnEvent', function(_, event, info)
            local ok, message = pcall(function()
                if event == 'UPDATE_ALL_UI_WIDGETS' then
                    ColumnAllEvents = ColumnAllEvents + 1
                    assert(#C_UIWidgetManager.GetAllWidgetsBySetID(436) == 0)
                    ColumnOneSnapshot(ColumnReplacement, ColumnGet(763621))
                    return
                end
                assert(info.widgetID == 763621 and info.widgetSetID == 436 and info.widgetType == 21)
                ColumnEvents = ColumnEvents + 1
                ColumnOneSnapshot(ColumnEventExpected, ColumnGet(info.widgetID))
                local members = C_UIWidgetManager.GetAllWidgetsBySetID(436)
                assert(#members == 1 and members[1].widgetID == 763621 and members[1].widgetType == 21)
            end)
            if not ok then ColumnEventError = tostring(message) end
        end)
        ColumnObserver = observer
    "#,
    );
    fixtures::publish(&env, fixtures::supplied());
    fixtures::assert_lua(
        &env,
        "committed full input observed at first event",
        r#"
        assert(ColumnEvents == 1 and ColumnEventError == nil, tostring(ColumnEventError))
        ColumnEventExpected = ColumnReplacement
    "#,
    );
    fixtures::publish(&env, fixtures::replacement());
    fixtures::assert_lua(
        &env,
        "replacement observed during second event",
        r#"
        assert(ColumnEvents == 2 and ColumnEventError == nil, tostring(ColumnEventError))
        ColumnSnapshot(ColumnRetained, ColumnExpected)
    "#,
    );
    assert!(env.remove_ui_widget(SET_ID, ID).unwrap());
    assert!(!env.remove_ui_widget(SET_ID, ID).unwrap());
    fixtures::assert_lua(
        &env,
        "membership removal does not delete independent visualization",
        r#"
        assert(ColumnAllEvents == 1 and ColumnEvents == 2)
        assert(ColumnEventError == nil, tostring(ColumnEventError))
        ColumnOneSnapshot(ColumnReplacement, ColumnGet(763621))
    "#,
    );
}

#[test]
fn text_column_rejected_type_or_unknown_set_keeps_records_membership_and_events_unchanged() {
    let env = WowLuaEnv::new().unwrap();
    fixtures::seed(&env, ID, fixtures::supplied());
    fixtures::populated_boundary(&env);
    fixtures::supply_empty_set(&env);
    fixtures::assert_lua(
        &env,
        "count actual events only",
        r#"
        ColumnRejectedEvents = 0
        ColumnObserver = CreateFrame('Frame')
        ColumnObserver:RegisterEvent('UPDATE_UI_WIDGET')
        ColumnObserver:RegisterEvent('UPDATE_ALL_UI_WIDGETS')
        ColumnObserver:SetScript('OnEvent', function() ColumnRejectedEvents = ColumnRejectedEvents + 1 end)
    "#,
    );
    let mut wrong_type = fixtures::membership();
    wrong_type.widget_type = 25;
    assert!(
        env.update_ui_widget(
            SET_ID,
            wrong_type,
            WidgetVisualization::TextColumnRow(fixtures::replacement())
        )
        .is_err()
    );
    assert!(
        env.update_ui_widget(
            SET_ID + 1.0,
            fixtures::membership(),
            WidgetVisualization::TextColumnRow(fixtures::replacement())
        )
        .is_err()
    );
    assert_eq!(
        env.state()
            .borrow()
            .ui_widget_inputs
            .text_column_rows
            .get(&WidgetId::from_number(ID)),
        Some(&fixtures::supplied())
    );
    fixtures::assert_lua(
        &env,
        "no partial mutation or notification",
        r#"
        assert(ColumnRejectedEvents == 0)
        assert(#C_UIWidgetManager.GetAllWidgetsBySetID(436) == 0)
        assert(C_UIWidgetManager.GetWidgetSetInfo(437) == nil)
        ColumnOneSnapshot(ColumnExpected, ColumnGet(763621))
    "#,
    );
}
