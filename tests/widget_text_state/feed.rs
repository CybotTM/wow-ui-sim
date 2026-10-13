use super::discrete_fixtures::assert_lua;
use super::fixtures::{ID, SET_ID, entry, supplied_record};
use wow_ui_sim::c_api::ui_widget_manager::{WidgetId, WidgetSetRecord, WidgetVisualization};
use wow_ui_sim::lua_api::WowLuaEnv;

pub(super) fn supply_empty_set(env: &WowLuaEnv) {
    env.replace_ui_widget_set(
        SET_ID,
        Some(WidgetSetRecord {
            layout_direction: 0, // UIWidgetSetLayoutDirection.Vertical in both cached docs
            vertical_padding: 7.0,
            widgets: Vec::new(),
        }),
    )
    .expect("supplied empty text-state set");
}

fn observe_feed(env: &WowLuaEnv) {
    assert_lua(
        env,
        "TextWithState real frame event observer",
        r#"
        TextStateFeedUpdates = {}
        TextStateFullUpdates = 0
        TextStateObserver = CreateFrame('Frame')
        TextStateObserver:RegisterEvent('UPDATE_UI_WIDGET')
        TextStateObserver:RegisterEvent('UPDATE_ALL_UI_WIDGETS')
        TextStateObserver:SetScript('OnEvent', function(_, event, info)
            if event == 'UPDATE_ALL_UI_WIDGETS' then
                TextStateFullUpdates = TextStateFullUpdates + 1
                assert(#C_UIWidgetManager.GetAllWidgetsBySetID(428) == 0)
                return
            end
            assert(info.widgetID == 760801 and info.widgetSetID == 428 and info.widgetType == 8)
            local members = C_UIWidgetManager.GetAllWidgetsBySetID(428)
            assert(#members == 1 and members[1].widgetID == 760801 and members[1].widgetType == 8)
            local row = C_UIWidgetManager.GetTextWithStateWidgetVisualizationInfo(760801)
            assert(type(row) == 'table', 'missing supplied TextWithState record in update event')
            TextStateFeedUpdates[#TextStateFeedUpdates + 1] = row
        end)
    "#,
    );
}

#[test]
fn text_state_feed_commits_before_events_retains_visualization_on_membership_removal() {
    let env = WowLuaEnv::new().expect("text-state feed environment");
    supply_empty_set(&env);
    observe_feed(&env);
    env.update_ui_widget(
        SET_ID,
        entry(),
        WidgetVisualization::TextWithState(supplied_record()),
    )
    .expect("initial typed text-state feed");
    // Check immediately: initial event's getter result is the intended pre-producer RED.
    assert_lua(
        &env,
        "TextWithState first event result boundary",
        "assert(#TextStateFeedUpdates == 1, 'initial event must observe supplied record')",
    );
    let mut record = supplied_record();
    record.text = "Updated state text".into();
    record.tooltip = "Updated text tooltip".into();
    env.update_ui_widget(
        SET_ID,
        entry(),
        WidgetVisualization::TextWithState(record.clone()),
    )
    .expect("replace visualization without duplicate membership");
    assert_eq!(
        env.state().borrow().ui_widget_inputs.text_with_state[&WidgetId::from_number(ID)],
        record
    );
    assert_lua(
        &env,
        "TextWithState immutable event snapshots",
        r#"
        assert(#TextStateFeedUpdates == 2 and TextStateFullUpdates == 0)
        assert(TextStateFeedUpdates[1].text == 'Supplied state text')
        assert(TextStateFeedUpdates[2].text == 'Updated state text')
        assert(TextStateFeedUpdates[1] ~= TextStateFeedUpdates[2])
    "#,
    );
    assert!(
        env.remove_ui_widget(SET_ID, ID)
            .expect("membership removal")
    );
    assert!(!env.remove_ui_widget(SET_ID, ID).expect("absent membership"));
    assert_lua(
        &env,
        "TextWithState independent visualization survives membership removal",
        r#"
        assert(TextStateFullUpdates == 1 and #TextStateFeedUpdates == 2)
        assert(#C_UIWidgetManager.GetAllWidgetsBySetID(428) == 0)
        assert(C_UIWidgetManager.GetTextWithStateWidgetVisualizationInfo(760801).text == 'Updated state text')
        assert(TextStateFeedUpdates[1].text == 'Supplied state text')
    "#,
    );
}

#[test]
fn text_state_feed_rejects_type_mismatch_unknown_set_without_mutation_or_notification() {
    let env = WowLuaEnv::new().expect("text-state rejected feed environment");
    supply_empty_set(&env);
    observe_feed(&env);
    assert!(
        env.update_ui_widget(
            SET_ID + 1.0,
            entry(),
            WidgetVisualization::TextWithState(supplied_record()),
        )
        .is_err()
    );
    let mut wrong_type = entry();
    wrong_type.widget_type = 19;
    assert!(
        env.update_ui_widget(
            SET_ID,
            wrong_type,
            WidgetVisualization::TextWithState(supplied_record()),
        )
        .is_err()
    );
    assert!(
        env.state()
            .borrow()
            .ui_widget_inputs
            .text_with_state
            .is_empty()
    );
    assert_lua(
        &env,
        "TextWithState rejected feed has no side effects",
        r#"
        assert(#TextStateFeedUpdates == 0 and TextStateFullUpdates == 0)
        assert(#C_UIWidgetManager.GetAllWidgetsBySetID(428) == 0)
        assert(C_UIWidgetManager.GetWidgetSetInfo(429) == nil)
    "#,
    );
}
