use super::fixtures::{self, ID, SET_ID};
use super::helpers::assert_lua;
use crate::common;
use wow_ui_sim::c_api::ui_widget_manager::WidgetVisualization;
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn icon_currencies_normal_feed_publishes_complete_state_before_synchronous_events() {
    common::with_timeout(88, || {
        let env = WowLuaEnv::new().unwrap();
        fixtures::install_assertions(&env);
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
                        assert(C_UIWidgetManager.GetWidgetSetInfo(447).verticalPadding == 7)
                        assert(#C_UIWidgetManager.GetAllWidgetsBySetID(447) == 0)
                    else
                        FeedUpdates = FeedUpdates + 1
                        assert(info.widgetSetID == 447 and info.widgetID == 762507)
                        assert(info.widgetType == 7 and info.unitToken == nil)
                        local members = C_UIWidgetManager.GetAllWidgetsBySetID(447)
                        assert(#members == 1 and members[1].widgetID == 762507)
                        assert(members[1].widgetType == 7 and members[1].widgetSetID == 447)
                        local snapshot = AssertSuppliedSnapshot(FixtureGetter(762507))
                        if FeedUpdates == 1 then
                            FeedFirstSnapshot = snapshot
                        else
                            FeedSecondSnapshot = snapshot
                        end
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
        fixtures::expect(&env, "CurrencyReplacement");
        fixtures::supply(&env, fixtures::replacement());
        assert_lua(
            &env,
            "both typed updates dispatched after state publication",
            r#"
            assert(FeedCallbackFailure == nil, FeedCallbackFailure)
            assert(FeedAll == 1 and FeedUpdates == 2)
            AssertSuppliedSnapshot(FixtureGetter(762507))
            AssertSuppliedSnapshot(FeedSecondSnapshot)
            assert(FeedFirstSnapshot ~= FeedSecondSnapshot)
            assert(FeedFirstSnapshot.currencies ~= FeedSecondSnapshot.currencies)
            FixtureExpected = CurrencyExpected
            AssertSuppliedSnapshot(FeedFirstSnapshot)
            FixtureExpected = CurrencyReplacement
        "#,
        );
        assert!(env.remove_ui_widget(SET_ID, ID).unwrap());
        assert_lua(
            &env,
            "removal publishes membership but retains record",
            r#"
            assert(FeedCallbackFailure == nil, FeedCallbackFailure)
            assert(FeedAll == 2 and FeedUpdates == 2)
            AssertSuppliedSnapshot(FixtureGetter(762507))
        "#,
        );
    });
}

#[test]
fn icon_currencies_wrong_feed_type_or_missing_set_changes_neither_state_nor_events() {
    common::with_timeout(88, || {
        let env = WowLuaEnv::new().unwrap();
        fixtures::supply_empty_set(&env);
        fixtures::supply(&env, fixtures::supplied());
        fixtures::install_assertions(&env);
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
                state.ui_widget_inputs.icon_text_and_currencies.clone(),
            )
        };
        let mut wrong_type = fixtures::entry();
        wrong_type.widget_type = 8; // TextWithState is not IconTextAndCurrencies.
        assert!(
            env.update_ui_widget(
                SET_ID,
                wrong_type,
                WidgetVisualization::IconTextAndCurrencies(fixtures::replacement())
            )
            .is_err()
        );
        assert!(
            env.update_ui_widget(
                SET_ID + 1.0,
                fixtures::entry(),
                WidgetVisualization::IconTextAndCurrencies(fixtures::replacement())
            )
            .is_err()
        );
        let mut absent_entry = fixtures::entry();
        absent_entry.widget_id = ID + 0.5;
        assert!(
            env.update_ui_widget(
                SET_ID + 1.0,
                absent_entry,
                WidgetVisualization::IconTextAndCurrencies(fixtures::replacement()),
            )
            .is_err()
        );
        assert!(!env.remove_ui_widget(SET_ID, ID + 1.0).unwrap());
        assert_lua(
            &env,
            "rejection is atomic and silent to subscribers",
            r#"
            assert(RejectedEvents == 0)
            AssertSuppliedSnapshot(FixtureGetter(762507))
            assert(C_UIWidgetManager.GetWidgetSetInfo(448) == nil)
            local function oneNil(...) assert(select('#', ...) == 1 and (...) == nil) end
            oneNil(FixtureGetter(762507.5))
            assert(#C_UIWidgetManager.GetAllWidgetsBySetID(447) == 1)
        "#,
        );
        let state = env.state();
        let state = state.borrow();
        assert_eq!(state.ui_widget_inputs.sets, sets);
        assert_eq!(state.ui_widget_inputs.icon_text_and_currencies, records);
    });
}
