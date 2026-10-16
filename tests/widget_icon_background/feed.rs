use super::fixtures::{self, GETTER, ID, SET_ID};
use super::helpers::{assert_lua, install_snapshot_assertion};
use crate::common;
use wow_ui_sim::c_api::ui_widget_manager::WidgetVisualization;
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn icon_text_and_background_normal_feed_publishes_complete_state_before_synchronous_events() {
    common::with_timeout(88, || {
        let env = WowLuaEnv::new().unwrap();
        install_snapshot_assertion(&env, GETTER, fixtures::EXPECTED);
        // Ordinary subscriber, not a replacement/wrapper of any cached event handler.
        assert_lua(
            &env,
            "subscribe to actual public frame events",
            r#"
            FeedAll, FeedUpdates = 0, 0
            FeedCallbackFailure = nil
            FeedListener = CreateFrame('Frame')
            FeedListener:RegisterEvent('UPDATE_ALL_UI_WIDGETS')
            FeedListener:RegisterEvent('UPDATE_UI_WIDGET')
            FeedListener:SetScript('OnEvent', function(_, event, info)
                local ok, message = pcall(function()
                    if event == 'UPDATE_ALL_UI_WIDGETS' then
                        FeedAll = FeedAll + 1
                        assert(C_UIWidgetManager.GetWidgetSetInfo(439).verticalPadding == 7)
                        assert(#C_UIWidgetManager.GetAllWidgetsBySetID(439) == 0)
                    else
                        FeedUpdates = FeedUpdates + 1
                        assert(info.widgetSetID == 439 and info.widgetID == 762504)
                        assert(info.widgetType == 4 and info.unitToken == nil)
                        local members = C_UIWidgetManager.GetAllWidgetsBySetID(439)
                        assert(#members == 1 and members[1].widgetID == 762504)
                        assert(members[1].widgetType == 4 and members[1].widgetSetID == 439)
                        AssertSuppliedSnapshot(FixtureGetter(762504))
                    end
                end)
                if not ok and FeedCallbackFailure == nil then
                    FeedCallbackFailure = event .. ': ' .. tostring(message)
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
            assert(FeedCallbackFailure == nil, FeedCallbackFailure)
            assert(FeedAll == 1 and FeedUpdates == 2)
            AssertSuppliedSnapshot(FixtureGetter(762504))
        "#,
        );
        assert!(env.remove_ui_widget(SET_ID, ID).unwrap());
        assert_lua(
            &env,
            "removal publishes membership but retains record",
            r#"
            assert(FeedCallbackFailure == nil, FeedCallbackFailure)
            assert(FeedAll == 2 and FeedUpdates == 2)
            AssertSuppliedSnapshot(FixtureGetter(762504))
        "#,
        );
    });
}

#[test]
fn icon_text_and_background_wrong_feed_type_or_missing_set_changes_neither_state_nor_events() {
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
                state.ui_widget_inputs.icon_text_and_background.clone(),
            )
        };
        let mut wrong_type = fixtures::entry();
        wrong_type.widget_type = 8; // TextWithState is not IconTextAndBackground.
        assert!(
            env.update_ui_widget(
                SET_ID,
                wrong_type,
                WidgetVisualization::IconTextAndBackground(fixtures::replacement())
            )
            .is_err()
        );
        assert!(
            env.update_ui_widget(
                SET_ID + 1.0,
                fixtures::entry(),
                WidgetVisualization::IconTextAndBackground(fixtures::replacement())
            )
            .is_err()
        );
        assert!(!env.remove_ui_widget(SET_ID, ID + 1.0).unwrap());
        assert_lua(
            &env,
            "rejection is atomic and silent to subscribers",
            r#"
            assert(RejectedEvents == 0)
            AssertSuppliedSnapshot(FixtureGetter(762504))
            assert(C_UIWidgetManager.GetWidgetSetInfo(440) == nil)
            assert(#C_UIWidgetManager.GetAllWidgetsBySetID(439) == 1)
        "#,
        );
        let state = env.state();
        let state = state.borrow();
        assert_eq!(state.ui_widget_inputs.sets, sets);
        assert_eq!(state.ui_widget_inputs.icon_text_and_background, records);
    });
}
