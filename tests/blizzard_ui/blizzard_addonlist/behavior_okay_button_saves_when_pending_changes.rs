//! Okay-button save behavior for `Blizzard_AddOnList`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_AddOnList";

#[test]
fn okay_button_saves_pending_changes_and_reloads_when_needed() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        let (
            okay_text_is_reload,
            should_reload,
            disabled_before_okay,
            reload_call_count,
            reset_restored_saved_disabled_state,
        ): (bool, bool, bool, i64, bool) = env
            .eval(
                r#"
                local function FindToggleableAddonIndex()
                    for index = 1, C_AddOns.GetNumAddOns() do
                        if C_AddOns.GetAddOnName(index) ~= "__BuiltIn" then
                            return index
                        end
                    end
                end

                local addonIndex = FindToggleableAddonIndex()
                C_AddOns.EnableAddOn(addonIndex)
                AddonList.startStatus[addonIndex] = true

                AddonList:Show()
                AddonList_Enable(addonIndex, false)

                local okayTextIsReload = AddonList.OkayButton:GetText() == RELOADUI
                local shouldReload = AddonList.shouldReload == true
                local disabledBeforeOkay =
                    C_AddOns.GetAddOnEnableState(addonIndex) <= Enum.AddOnEnableState.None

                local reloadCallCount = 0
                ReloadUI = function()
                    reloadCallCount = reloadCallCount + 1
                end

                AddonList.OkayButton:Click()

                C_AddOns.EnableAddOn(addonIndex)
                C_AddOns.ResetAddOns()
                local resetRestoredSavedDisabledState =
                    C_AddOns.GetAddOnEnableState(addonIndex) <= Enum.AddOnEnableState.None

                return okayTextIsReload,
                       shouldReload,
                       disabledBeforeOkay,
                       reloadCallCount,
                       resetRestoredSavedDisabledState
                "#,
            )
            .expect("AddonList Okay-button save probe must run cleanly");

        assert!(
            okay_text_is_reload,
            "pending addon changes must change `AddonList.OkayButton` text to `RELOADUI`"
        );
        assert!(
            should_reload,
            "pending addon changes must set `AddonList.shouldReload`"
        );
        assert!(
            disabled_before_okay,
            "test setup must create a pending disabled-addon change before clicking Okay"
        );
        assert_eq!(
            reload_call_count, 1,
            "`AddonList.OkayButton:Click()` must route through `AddonList_OnOkay` and call `ReloadUI`"
        );
        assert!(
            reset_restored_saved_disabled_state,
            "`AddonList.OkayButton:Click()` must hide the panel and trigger `C_AddOns.SaveAddOns()`"
        );
    });
}
