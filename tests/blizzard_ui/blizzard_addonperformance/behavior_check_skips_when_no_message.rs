//! AddOnPerformance no-message behavior for `Blizzard_AddOnPerformance`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AddOnPerformance";

#[test]
fn check_exits_without_mutation_when_profiler_returns_no_message() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let probe: NoMessageProbe = env
            .eval(
                r#"
                local originalInCombatLockdown = InCombatLockdown
                local originalCheckForPerformanceMessage = C_AddOnProfiler.CheckForPerformanceMessage
                local originalAddPerformanceMessageShown = C_AddOnProfiler.AddPerformanceMessageShown
                local originalStaticPopupShow = StaticPopup_Show

                local checkCalls = 0
                local markedSeenCalls = 0
                local popupCalls = 0

                InCombatLockdown = function() return false end
                C_AddOnProfiler.CheckForPerformanceMessage = function()
                    checkCalls = checkCalls + 1
                    return nil
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
                       next(AddOnPerformance.shownPerformanceMessages) == nil,
                       next(AddOnPerformance.addOnHasPerformanceWarning) == nil
                "#,
            )
            .expect("AddOnPerformance no-message probe must run cleanly");

        assert_no_message_probe(probe);
    });
}

type NoMessageProbe = (i64, i64, i64, bool, bool);

fn assert_no_message_probe(probe: NoMessageProbe) {
    let (check_calls, marked_seen_calls, popup_calls, shown_messages_empty, addon_warnings_empty) =
        probe;

    assert_eq!(
        check_calls, 1,
        "`CheckAndDisplayPerformanceMessage` must poll once when not in combat"
    );
    assert_eq!(
        marked_seen_calls, 0,
        "`CheckAndDisplayPerformanceMessage` must not mark a nil message as seen"
    );
    assert_eq!(
        popup_calls, 0,
        "`CheckAndDisplayPerformanceMessage` must not show a popup when no message is pending"
    );
    assert!(
        shown_messages_empty,
        "nil-message short-circuit must leave `shownPerformanceMessages` empty"
    );
    assert!(
        addon_warnings_empty,
        "nil-message short-circuit must leave addon warning flags empty"
    );
}
