use super::fixtures::{self, ID, SET_ID};
use super::helpers::assert_lua;
use crate::common;
use wow_ui_sim::c_api::ui_widget_manager::WidgetVisualization;
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn item_display_normal_feed_publishes_complete_state_before_synchronous_events() {
    common::with_timeout(88, || {
        let env = WowLuaEnv::new().unwrap();
        fixtures::install_assertions(&env);
        assert_lua(
            &env,
            "ordinary public event subscriber",
            r#"
            FeedAll, FeedUpdates = 0, 0
            FeedFailure = nil
            FeedListener = CreateFrame('Frame')
            FeedListener:RegisterEvent('UPDATE_ALL_UI_WIDGETS')
            FeedListener:RegisterEvent('UPDATE_UI_WIDGET')
            FeedListener:SetScript('OnEvent', function(_, event, info)
                local ok, message = pcall(function()
                    if event == 'UPDATE_ALL_UI_WIDGETS' then
                        FeedAll = FeedAll + 1
                        assert(C_UIWidgetManager.GetWidgetSetInfo(467).verticalPadding == 7)
                        assert(#C_UIWidgetManager.GetAllWidgetsBySetID(467) == 0)
                        if FeedAll == 2 then AssertItemAt(934227, ItemReplacement) end
                    else
                        FeedUpdates = FeedUpdates + 1
                        assert(info.widgetSetID == 467 and info.widgetID == 934227)
                        assert(info.widgetType == 27 and info.unitToken == nil)
                        local members = C_UIWidgetManager.GetAllWidgetsBySetID(467)
                        assert(#members == 1 and members[1].widgetID == 934227)
                        assert(members[1].widgetType == 27 and members[1].widgetSetID == 467)
                        local expected = FeedUpdates == 1 and ItemExpected or ItemReplacement
                        local snapshot = AssertItemAt(934227, expected)
                        if FeedUpdates == 1 then FeedFirst = snapshot else FeedSecond = snapshot end
                    end
                end)
                if not ok and FeedFailure == nil then FeedFailure = event .. ': ' .. tostring(message) end
            end)
        "#,
        );
        fixtures::supply_empty_set(&env);
        fixtures::supply(&env, fixtures::supplied());
        fixtures::supply(&env, fixtures::replacement());
        assert_lua(
            &env,
            "complete records observable inside both update events",
            r#"
            assert(FeedFailure == nil, FeedFailure)
            assert(FeedAll == 1 and FeedUpdates == 2)
            AssertItemAt(934227, ItemReplacement)
            FixtureExpected = ItemReplacement
            AssertSuppliedSnapshot(FeedSecond)
            FixtureExpected = ItemExpected
            AssertSuppliedSnapshot(FeedFirst)
            assert(FeedFirst ~= FeedSecond and FeedFirst.itemInfo ~= FeedSecond.itemInfo)
        "#,
        );
        assert!(env.remove_ui_widget(SET_ID, ID).unwrap());
        assert_lua(
            &env,
            "membership removal published before event, record retained",
            r#"
            assert(FeedFailure == nil, FeedFailure)
            assert(FeedAll == 2 and FeedUpdates == 2)
            AssertItemAt(934227, ItemReplacement)
        "#,
        );
    });
}

#[test]
fn item_display_wrong_type_and_missing_set_reject_atomically_without_events() {
    common::with_timeout(88, || {
        let env = WowLuaEnv::new().unwrap();
        fixtures::supply_empty_set(&env);
        fixtures::supply(&env, fixtures::supplied());
        fixtures::install_assertions(&env);
        assert_lua(
            &env,
            "populated boundary before rejection checks",
            r#"
            AssertItemAt(934227, ItemExpected)
            RejectedEvents = 0
            RejectionListener = CreateFrame('Frame')
            RejectionListener:RegisterEvent('UPDATE_UI_WIDGET')
            RejectionListener:RegisterEvent('UPDATE_ALL_UI_WIDGETS')
            RejectionListener:SetScript('OnEvent', function() RejectedEvents = RejectedEvents + 1 end)
        "#,
        );
        let (sets, records) = {
            let state = env.state();
            let state = state.borrow();
            (
                state.ui_widget_inputs.sets.clone(),
                state.ui_widget_inputs.item_display.clone(),
            )
        };
        let mut wrong_type = fixtures::entry();
        wrong_type.widget_type = 8;
        assert!(
            env.update_ui_widget(
                SET_ID,
                wrong_type,
                WidgetVisualization::ItemDisplay(fixtures::replacement())
            )
            .is_err()
        );
        assert!(
            env.update_ui_widget(
                SET_ID + 1.0,
                fixtures::entry(),
                WidgetVisualization::ItemDisplay(fixtures::replacement())
            )
            .is_err()
        );
        let mut absent = fixtures::entry();
        absent.widget_id = ID + 0.5;
        assert!(
            env.update_ui_widget(
                SET_ID + 1.0,
                absent,
                WidgetVisualization::ItemDisplay(fixtures::replacement())
            )
            .is_err()
        );
        assert!(!env.remove_ui_widget(SET_ID, ID + 1.0).unwrap());
        assert_lua(
            &env,
            "rejection preserves complete public state",
            r#"
            assert(RejectedEvents == 0)
            AssertItemAt(934227, ItemExpected)
            assert(C_UIWidgetManager.GetWidgetSetInfo(468) == nil)
            local function oneNil(...) assert(select('#', ...) == 1 and (...) == nil) end
            oneNil(FixtureGetter(934227.5))
            local members = C_UIWidgetManager.GetAllWidgetsBySetID(467)
            assert(#members == 1 and members[1].widgetID == 934227)
        "#,
        );
        let state = env.state();
        let state = state.borrow();
        assert_eq!(state.ui_widget_inputs.sets, sets);
        assert_eq!(state.ui_widget_inputs.item_display, records);
    });
}
