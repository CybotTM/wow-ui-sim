use super::fixtures::{self, GETTER, ID, SET_ID};
use super::helpers::{assert_lua, install_snapshot_assertion};
use crate::common;
use wow_ui_sim::c_api::ui_widget_manager::WidgetVisualization;
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn spacer_normal_feed_before_event_and_atomic_rejection() {
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
                    assert(C_UIWidgetManager.GetWidgetSetInfo(422).verticalPadding == 7)
                    assert(#C_UIWidgetManager.GetAllWidgetsBySetID(422) == 0)
                else
                    FeedUpdates = FeedUpdates + 1
                    assert(info.widgetSetID == 422 and info.widgetID == 762201)
                    assert(info.widgetType == 22 and info.unitToken == nil)
                    local members = C_UIWidgetManager.GetAllWidgetsBySetID(422)
                    assert(#members == 1 and members[1].widgetID == 762201)
                    assert(members[1].widgetType == 22 and members[1].widgetSetID == 422)
                    AssertSuppliedSnapshot(FixtureGetter(762201))
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
            AssertSuppliedSnapshot(FixtureGetter(762201))
        "#,
        );
        assert!(env.remove_ui_widget(SET_ID, ID).unwrap());
        assert_lua(
            &env,
            "removal publishes membership but retains record",
            r#"
            assert(FeedAll == 2 and FeedUpdates == 2)
            AssertSuppliedSnapshot(FixtureGetter(762201))
        "#,
        );
        assert_lua(
            &env,
            "detach original feed observer",
            "FeedListener:UnregisterAllEvents()",
        );
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
                state.ui_widget_inputs.spacers.clone(),
            )
        };
        let mut wrong_type = fixtures::entry();
        wrong_type.widget_type = 8; // TextWithState is not Spacer.
        assert!(
            env.update_ui_widget(
                SET_ID,
                wrong_type,
                WidgetVisualization::Spacer(fixtures::replacement())
            )
            .is_err()
        );
        assert!(
            env.update_ui_widget(
                SET_ID + 1.0,
                fixtures::entry(),
                WidgetVisualization::Spacer(fixtures::replacement())
            )
            .is_err()
        );
        assert!(!env.remove_ui_widget(SET_ID, ID + 1.0).unwrap());
        assert_lua(
            &env,
            "rejection is atomic and silent to subscribers",
            r#"
            assert(RejectedEvents == 0)
            AssertSuppliedSnapshot(FixtureGetter(762201))
            assert(C_UIWidgetManager.GetWidgetSetInfo(423) == nil)
            assert(#C_UIWidgetManager.GetAllWidgetsBySetID(422) == 1)
        "#,
        );
        let state = env.state();
        let state = state.borrow();
        assert_eq!(state.ui_widget_inputs.sets, sets);
        assert_eq!(state.ui_widget_inputs.spacers, records);
    });
}
