//! AddOnPerformance per-addon warning behavior for `Blizzard_AddOnPerformance`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AddOnPerformance";

#[test]
fn check_records_per_addon_warning_and_query_reports_it() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let probe: AddonWarningProbe = env
            .eval(
                r#"
                local originalInCombatLockdown = InCombatLockdown
                local originalCheckForPerformanceMessage = C_AddOnProfiler.CheckForPerformanceMessage
                local originalAddPerformanceMessageShown = C_AddOnProfiler.AddPerformanceMessageShown
                local originalStaticPopupShow = StaticPopup_Show
                local originalAddonList = AddonList

                local checkCalls = 0
                local markedSeenCalls = 0
                local popupCalls = 0
                local addonName = "PerAddonPerformanceProbe"
                local message = {
                    type = Enum.AddOnPerformanceMessageType.SpecificAddOnErrorDialog,
                    addOnName = addonName,
                }

                InCombatLockdown = function() return false end
                C_AddOnProfiler.CheckForPerformanceMessage = function()
                    checkCalls = checkCalls + 1
                    return message
                end
                C_AddOnProfiler.AddPerformanceMessageShown = function()
                    markedSeenCalls = markedSeenCalls + 1
                end
                StaticPopup_Show = function()
                    popupCalls = popupCalls + 1
                end
                AddonList = {
                    IsVisible = function()
                        return false
                    end,
                }

                AddOnPerformance:CheckAndDisplayPerformanceMessage()

                local tableFlag = AddOnPerformance.addOnHasPerformanceWarning[addonName] == true
                local queryFlag = AddOnPerformance:AddOnHasPerformanceWarning(addonName) == true

                InCombatLockdown = originalInCombatLockdown
                C_AddOnProfiler.CheckForPerformanceMessage = originalCheckForPerformanceMessage
                C_AddOnProfiler.AddPerformanceMessageShown = originalAddPerformanceMessageShown
                StaticPopup_Show = originalStaticPopupShow
                AddonList = originalAddonList

                return checkCalls,
                       markedSeenCalls,
                       popupCalls,
                       tableFlag,
                       queryFlag
                "#,
            )
            .expect("AddOnPerformance per-addon warning probe must run cleanly");

        assert_addon_warning_probe(probe);
    });
}

type AddonWarningProbe = (i64, i64, i64, bool, bool);

fn assert_addon_warning_probe(probe: AddonWarningProbe) {
    let (check_calls, marked_seen_calls, popup_calls, table_flag, query_flag) = probe;

    assert_eq!(
        check_calls, 1,
        "`CheckAndDisplayPerformanceMessage` must poll one profiler message"
    );
    assert_eq!(
        marked_seen_calls, 1,
        "messages with an addon warning must still be recorded as shown"
    );
    assert_eq!(
        popup_calls, 1,
        "specific addon error messages must continue through `DisplayMessage`"
    );
    assert!(
        table_flag,
        "successful tick must flag `addOnHasPerformanceWarning[addOnName]`"
    );
    assert!(
        query_flag,
        "`AddOnHasPerformanceWarning(addOnName)` must report the cached warning"
    );
}
