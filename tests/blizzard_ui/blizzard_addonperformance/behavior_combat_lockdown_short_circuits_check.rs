//! AddOnPerformance combat-lockdown behavior for `Blizzard_AddOnPerformance`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AddOnPerformance";

#[test]
fn combat_lockdown_short_circuits_performance_message_check() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let probe: CombatLockdownProbe = env
            .eval(
                r#"
                local originalInCombatLockdown = InCombatLockdown
                local originalCheckForPerformanceMessage = C_AddOnProfiler.CheckForPerformanceMessage
                local originalAddPerformanceMessageShown = C_AddOnProfiler.AddPerformanceMessageShown
                local originalStaticPopupShow = StaticPopup_Show

                local checkCalls = 0
                local markedSeenCalls = 0
                local popupCalls = 0
                local message = {
                    type = Enum.AddOnPerformanceMessageType.SpecificAddOnErrorDialog,
                    addOnName = "CombatLockedPerformanceProbe",
                }

                InCombatLockdown = function() return true end
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

                AddOnPerformance:CheckAndDisplayPerformanceMessage()

                InCombatLockdown = originalInCombatLockdown
                C_AddOnProfiler.CheckForPerformanceMessage = originalCheckForPerformanceMessage
                C_AddOnProfiler.AddPerformanceMessageShown = originalAddPerformanceMessageShown
                StaticPopup_Show = originalStaticPopupShow

                return checkCalls,
                       markedSeenCalls,
                       popupCalls,
                       AddOnPerformance.shownPerformanceMessages[message.type] == nil,
                       AddOnPerformance.addOnHasPerformanceWarning[message.addOnName] == nil
                "#,
            )
            .expect("AddOnPerformance combat-lockdown probe must run cleanly");

        assert_combat_lockdown_probe(probe);
    });
}

type CombatLockdownProbe = (i64, i64, i64, bool, bool);

fn assert_combat_lockdown_probe(probe: CombatLockdownProbe) {
    let (
        check_calls,
        marked_seen_calls,
        popup_calls,
        message_not_marked_seen,
        addon_warning_not_marked,
    ) = probe;

    assert_eq!(
        check_calls, 0,
        "`CheckAndDisplayPerformanceMessage` must not poll profiler messages during combat"
    );
    assert_eq!(
        marked_seen_calls, 0,
        "`CheckAndDisplayPerformanceMessage` must not mark messages seen during combat"
    );
    assert_eq!(
        popup_calls, 0,
        "`CheckAndDisplayPerformanceMessage` must not show popups during combat"
    );
    assert!(
        message_not_marked_seen,
        "combat short-circuit must leave `shownPerformanceMessages` untouched"
    );
    assert!(
        addon_warning_not_marked,
        "combat short-circuit must leave addon warning flags untouched"
    );
}
