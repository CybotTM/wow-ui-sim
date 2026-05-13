//! Behavior pin: SETTINGS_LOADED wires action bar setting callbacks.

use crate::common;
use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_ActionBarController";

#[test]
fn settings_loaded_registers_action_bar_callbacks_and_update_event() {
    test_timeout! {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        env.exec(
            r#"
            _G.actionBarSettingCallbacks = {}
            _G.actionBarSettingValues = {}
            _G.multiActionBarUpdateCalls = 0
            _G.actionBarShownSettingUpdatedEvents = 0

            function Settings.SetOnValueChangedCallback(variable, callback)
                _G.actionBarSettingCallbacks[variable] = callback
            end

            function Settings.SetValue(variable, value)
                _G.actionBarSettingValues[variable] = value
                local callback = _G.actionBarSettingCallbacks[variable]
                if callback then
                    callback()
                end
            end

            function MultiActionBar_Update()
                _G.multiActionBarUpdateCalls = _G.multiActionBarUpdateCalls + 1
            end

            StatusTrackingBarManager.UpdateBarTicks = function() end

            function EventRegistry:TriggerEvent(eventName)
                if eventName == "ActionBarShownSettingUpdated" then
                    _G.actionBarShownSettingUpdatedEvents =
                        _G.actionBarShownSettingUpdatedEvents + 1
                end
            end

            ActionBarController:GetScript("OnEvent")(
                ActionBarController,
                "SETTINGS_LOADED"
            )
            "#,
        )
        .expect("SETTINGS_LOADED dispatch must run cleanly");

        let (registered_count, initial_updates, initial_events): (i32, i32, i32) = env
            .eval(
                r#"
                local variables = {
                    "PROXY_SHOW_ACTIONBAR_2",
                    "PROXY_SHOW_ACTIONBAR_3",
                    "PROXY_SHOW_ACTIONBAR_4",
                    "PROXY_SHOW_ACTIONBAR_5",
                    "PROXY_SHOW_ACTIONBAR_6",
                    "PROXY_SHOW_ACTIONBAR_7",
                    "PROXY_SHOW_ACTIONBAR_8",
                }
                local count = 0
                for index, variable in ipairs(variables) do
                    if type(_G.actionBarSettingCallbacks[variable]) == "function" then
                        count = count + 1
                    end
                end
                return count,
                    _G.multiActionBarUpdateCalls,
                    _G.actionBarShownSettingUpdatedEvents
                "#,
            )
            .expect("SETTINGS_LOADED callback registration probe must run cleanly");

        assert_eq!(
            registered_count, 7,
            "SETTINGS_LOADED must register callbacks for PROXY_SHOW_ACTIONBAR_2 through _8"
        );
        assert_eq!(
            initial_updates, 1,
            "SETTINGS_LOADED must run the action bar update once immediately"
        );
        assert_eq!(
            initial_events, 1,
            "SETTINGS_LOADED must fire ActionBarShownSettingUpdated once immediately"
        );

        env.exec(
            r#"
            _G.multiActionBarUpdateCalls = 0
            _G.actionBarShownSettingUpdatedEvents = 0

            Settings.SetValue("PROXY_SHOW_ACTIONBAR_4", true)
            "#,
        )
        .expect("action bar setting change must run cleanly");

        let (change_updates, change_events, changed_value): (i32, i32, bool) = env
            .eval(
                r#"
                return _G.multiActionBarUpdateCalls,
                    _G.actionBarShownSettingUpdatedEvents,
                    _G.actionBarSettingValues.PROXY_SHOW_ACTIONBAR_4
                "#,
            )
            .expect("post setting change probe must run cleanly");

        assert_eq!(
            change_updates, 1,
            "changing a registered action bar setting must run MultiActionBar_Update"
        );
        assert_eq!(
            change_events, 1,
            "changing a registered action bar setting must fire ActionBarShownSettingUpdated"
        );
        assert!(
            changed_value,
            "the setting change probe must update the selected setting value"
        );
    });
    }
}
