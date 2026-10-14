use super::fixtures::{self, GETTER, ID, SET_ID};
use super::helpers::{assert_lua, install_snapshot_assertion};
use crate::common;
use wow_ui_sim::c_api::ui_widget_manager::WidgetVisualization;
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn text_with_subtext_normal_feed_publishes_complete_state_before_synchronous_events() {
    common::with_timeout(88, || {
        let env = WowLuaEnv::new().unwrap();
        install_snapshot_assertion(&env, GETTER, fixtures::EXPECTED);
        // Ordinary subscriber, not a replacement/wrapper of any cached event handler.
        assert_lua(
            &env,
            "subscribe to actual public frame events",
            r#"
            FeedAll, FeedUpdates = 0, 0
            FeedListener = CreateFrame('Frame')
            FeedListener:RegisterEvent('UPDATE_ALL_UI_WIDGETS')
            FeedListener:RegisterEvent('UPDATE_UI_WIDGET')
            FeedListener:SetScript('OnEvent', function(_, event, info)
                if event == 'UPDATE_ALL_UI_WIDGETS' then
                    FeedAll = FeedAll + 1
                    assert(C_UIWidgetManager.GetWidgetSetInfo(435).verticalPadding == 7)
                    assert(#C_UIWidgetManager.GetAllWidgetsBySetID(435) == 0)
                else
                    FeedUpdates = FeedUpdates + 1
                    assert(info.widgetSetID == 435 and info.widgetID == 762501)
                    assert(info.widgetType == 25 and info.unitToken == nil)
                    local members = C_UIWidgetManager.GetAllWidgetsBySetID(435)
                    assert(#members == 1 and members[1].widgetID == 762501)
                    assert(members[1].widgetType == 25 and members[1].widgetSetID == 435)
                    AssertSuppliedSnapshot(FixtureGetter(762501))
                end
            end)
        "#,
        );
        fixtures::supply_empty_set(&env);
        fixtures::supply(&env, fixtures::supplied());
        install_snapshot_assertion(&env, GETTER, fixtures::REPLACEMENT_EXPECTED);
        fixtures::supply(&env, fixtures::replacement());
        assert_lua(
            &env,
            "both typed updates dispatched after state publication",
            r#"
            assert(FeedAll == 1 and FeedUpdates == 2)
            AssertSuppliedSnapshot(FixtureGetter(762501))
        "#,
        );
        assert!(env.remove_ui_widget(SET_ID, ID).unwrap());
        assert_lua(
            &env,
            "removal publishes membership but retains record",
            r#"
            assert(FeedAll == 2 and FeedUpdates == 2)
            AssertSuppliedSnapshot(FixtureGetter(762501))
        "#,
        );
    });
}

#[test]
fn text_with_subtext_wrong_feed_type_or_missing_set_changes_neither_state_nor_events() {
    common::with_timeout(88, || {
        let env = WowLuaEnv::new().unwrap();
        fixtures::supply_empty_set(&env);
        fixtures::supply(&env, fixtures::supplied());
        install_snapshot_assertion(&env, GETTER, fixtures::EXPECTED);
        assert_lua(
            &env,
            "ordinary subscriber to rejected updates",
            r#"
            RejectedEvents = 0
            RejectionListener = CreateFrame('Frame')
            RejectionListener:RegisterEvent('UPDATE_ALL_UI_WIDGETS')
            RejectionListener:RegisterEvent('UPDATE_UI_WIDGET')
            RejectionListener:SetScript('OnEvent', function() RejectedEvents = RejectedEvents + 1 end)
        "#,
        );
        let (sets, records) = {
            let state = env.state();
            let state = state.borrow();
            (
                state.ui_widget_inputs.sets.clone(),
                state.ui_widget_inputs.text_with_subtext.clone(),
            )
        };
        let mut wrong_type = fixtures::entry();
        wrong_type.widget_type = 8; // TextWithState is not TextWithSubtext.
        assert!(
            env.update_ui_widget(
                SET_ID,
                wrong_type,
                WidgetVisualization::TextWithSubtext(fixtures::replacement())
            )
            .is_err()
        );
        assert!(
            env.update_ui_widget(
                SET_ID + 1.0,
                fixtures::entry(),
                WidgetVisualization::TextWithSubtext(fixtures::replacement())
            )
            .is_err()
        );
        assert!(!env.remove_ui_widget(SET_ID, ID + 1.0).unwrap());
        assert_lua(
            &env,
            "rejection is atomic and silent to subscribers",
            r#"
            assert(RejectedEvents == 0)
            AssertSuppliedSnapshot(FixtureGetter(762501))
            assert(C_UIWidgetManager.GetWidgetSetInfo(436) == nil)
            assert(#C_UIWidgetManager.GetAllWidgetsBySetID(435) == 1)
        "#,
        );
        let state = env.state();
        let state = state.borrow();
        assert_eq!(state.ui_widget_inputs.sets, sets);
        assert_eq!(state.ui_widget_inputs.text_with_subtext, records);
    });
}
