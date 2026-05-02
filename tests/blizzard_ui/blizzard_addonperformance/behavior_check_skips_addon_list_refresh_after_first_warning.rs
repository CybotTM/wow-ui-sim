//! AddOnPerformance repeated addon warning refresh behavior for `Blizzard_AddOnPerformance`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AddOnPerformance";

#[test]
fn check_skips_addon_list_refresh_after_first_warning_for_addon() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let probe: RepeatAddonWarningProbe = env
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
                local addOnName = "RepeatAddonWarningRefreshProbe"
                local messages = {
                    {
                        type = Enum.AddOnPerformanceMessageType.SpecificAddOnErrorDialog,
                        addOnName = addOnName,
                    },
                    {
                        type = Enum.AddOnPerformanceMessageType.SpecificAddOnChatWarning,
                        addOnName = addOnName,
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
                        return true
                    end,
                }
                AddonList_Update = function()
                    updateCalls = updateCalls + 1
                end

                AddOnPerformance:CheckAndDisplayPerformanceMessage()
                local updateCallsAfterFirstWarning = updateCalls

                AddOnPerformance:CheckAndDisplayPerformanceMessage()
                local updateCallsAfterSecondWarning = updateCalls

                local warningCached = AddOnPerformance:AddOnHasPerformanceWarning(addOnName) == true
                local firstTypeMarked =
                    AddOnPerformance.shownPerformanceMessages[messages[1].type] == true
                local secondTypeMarked =
                    AddOnPerformance.shownPerformanceMessages[messages[2].type] == true

                InCombatLockdown = originalInCombatLockdown
                C_AddOnProfiler.CheckForPerformanceMessage = originalCheckForPerformanceMessage
                C_AddOnProfiler.AddPerformanceMessageShown = originalAddPerformanceMessageShown
                StaticPopup_Show = originalStaticPopupShow
                AddonList = originalAddonList
                AddonList_Update = originalAddonListUpdate

                return checkCalls,
                       updateCallsAfterFirstWarning,
                       updateCallsAfterSecondWarning,
                       warningCached,
                       firstTypeMarked,
                       secondTypeMarked
                "#,
            )
            .expect("AddOnPerformance repeated-addon warning probe must run cleanly");

        assert_repeat_addon_warning_probe(probe);
    });
}

type RepeatAddonWarningProbe = (i64, i64, i64, bool, bool, bool);

fn assert_repeat_addon_warning_probe(probe: RepeatAddonWarningProbe) {
    let (
        check_calls,
        update_calls_after_first_warning,
        update_calls_after_second_warning,
        warning_cached,
        first_type_marked,
        second_type_marked,
    ) = probe;

    assert_eq!(
        check_calls, 2,
        "`CheckAndDisplayPerformanceMessage` must poll both warning messages"
    );
    assert_eq!(
        update_calls_after_first_warning, 1,
        "first warning for an addon must refresh visible AddonList once"
    );
    assert_eq!(
        update_calls_after_second_warning, 1,
        "second warning for the same addon must not refresh AddonList again"
    );
    assert!(
        warning_cached,
        "first warning must cache the addon performance warning flag"
    );
    assert!(
        first_type_marked,
        "first warning message type must still be marked shown"
    );
    assert!(
        second_type_marked,
        "second warning message type must still be marked shown"
    );
}
