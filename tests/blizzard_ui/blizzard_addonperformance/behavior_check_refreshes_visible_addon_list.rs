//! AddOnPerformance AddonList refresh behavior for `Blizzard_AddOnPerformance`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AddOnPerformance";

#[test]
fn check_refreshes_visible_addon_list_only_when_warning_is_recorded() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let probe: AddonListRefreshProbe = env
            .eval(
                r#"
                local originalInCombatLockdown = InCombatLockdown
                local originalCheckForPerformanceMessage = C_AddOnProfiler.CheckForPerformanceMessage
                local originalAddPerformanceMessageShown = C_AddOnProfiler.AddPerformanceMessageShown
                local originalStaticPopupShow = StaticPopup_Show
                local originalAddonList = AddonList
                local originalAddonListUpdate = AddonList_Update

                local checkCalls = 0
                local updateCalls = 0
                local visible = true
                local messages = {
                    {
                        type = Enum.AddOnPerformanceMessageType.SpecificAddOnErrorDialog,
                        addOnName = "VisibleAddonListRefreshProbe",
                    },
                    {
                        type = Enum.AddOnPerformanceMessageType.SpecificAddOnErrorDialog,
                        addOnName = "HiddenAddonListRefreshProbe",
                    },
                }

                InCombatLockdown = function() return false end
                C_AddOnProfiler.CheckForPerformanceMessage = function()
                    checkCalls = checkCalls + 1
                    return messages[checkCalls]
                end
                C_AddOnProfiler.AddPerformanceMessageShown = function() end
                StaticPopup_Show = function() end
                AddonList = {
                    IsVisible = function()
                        return visible
                    end,
                }
                AddonList_Update = function()
                    updateCalls = updateCalls + 1
                end

                AddOnPerformance:CheckAndDisplayPerformanceMessage()
                local updateCallsAfterVisible = updateCalls

                visible = false
                AddOnPerformance:CheckAndDisplayPerformanceMessage()
                local updateCallsAfterHidden = updateCalls

                local visibleWarning =
                    AddOnPerformance:AddOnHasPerformanceWarning(messages[1].addOnName) == true
                local hiddenWarning =
                    AddOnPerformance:AddOnHasPerformanceWarning(messages[2].addOnName) == true

                InCombatLockdown = originalInCombatLockdown
                C_AddOnProfiler.CheckForPerformanceMessage = originalCheckForPerformanceMessage
                C_AddOnProfiler.AddPerformanceMessageShown = originalAddPerformanceMessageShown
                StaticPopup_Show = originalStaticPopupShow
                AddonList = originalAddonList
                AddonList_Update = originalAddonListUpdate

                return checkCalls,
                       updateCallsAfterVisible,
                       updateCallsAfterHidden,
                       visibleWarning,
                       hiddenWarning
                "#,
            )
            .expect("AddOnPerformance AddonList refresh probe must run cleanly");

        assert_addon_list_refresh_probe(probe);
    });
}

type AddonListRefreshProbe = (i64, i64, i64, bool, bool);

fn assert_addon_list_refresh_probe(probe: AddonListRefreshProbe) {
    let (
        check_calls,
        update_calls_after_visible,
        update_calls_after_hidden,
        visible_warning,
        hidden_warning,
    ) = probe;

    assert_eq!(
        check_calls, 2,
        "`CheckAndDisplayPerformanceMessage` must poll both warning messages"
    );
    assert_eq!(
        update_calls_after_visible, 1,
        "visible AddonList must refresh exactly once when the warning is recorded"
    );
    assert_eq!(
        update_calls_after_hidden, 1,
        "hidden AddonList must not refresh when the warning is recorded"
    );
    assert!(
        visible_warning,
        "visible AddonList scenario must still record the addon warning"
    );
    assert!(
        hidden_warning,
        "hidden AddonList scenario must still record the addon warning"
    );
}
