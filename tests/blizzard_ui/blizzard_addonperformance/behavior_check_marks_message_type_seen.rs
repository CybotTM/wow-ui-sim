//! AddOnPerformance successful-message behavior for `Blizzard_AddOnPerformance`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AddOnPerformance";

#[test]
fn check_marks_message_type_seen_and_records_profiler_message_once() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let probe: MarkSeenProbe = env
            .eval(
                r#"
                local originalInCombatLockdown = InCombatLockdown
                local originalCheckForPerformanceMessage = C_AddOnProfiler.CheckForPerformanceMessage
                local originalAddPerformanceMessageShown = C_AddOnProfiler.AddPerformanceMessageShown
                local originalStaticPopupShow = StaticPopup_Show

                local checkCalls = 0
                local markedSeenCalls = 0
                local markedMessageIsOriginal = false
                local popupCalls = 0
                local message = {
                    type = Enum.AddOnPerformanceMessageType.OverallAddOnErrorDialog,
                }

                InCombatLockdown = function() return false end
                C_AddOnProfiler.CheckForPerformanceMessage = function()
                    checkCalls = checkCalls + 1
                    return message
                end
                C_AddOnProfiler.AddPerformanceMessageShown = function(markedMessage)
                    markedSeenCalls = markedSeenCalls + 1
                    markedMessageIsOriginal = markedMessage == message
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
                       markedMessageIsOriginal,
                       popupCalls,
                       AddOnPerformance.shownPerformanceMessages[message.type] == true
                "#,
            )
            .expect("AddOnPerformance mark-seen probe must run cleanly");

        assert_mark_seen_probe(probe);
    });
}

type MarkSeenProbe = (i64, i64, bool, i64, bool);

fn assert_mark_seen_probe(probe: MarkSeenProbe) {
    let (check_calls, marked_seen_calls, marked_message_is_original, popup_calls, message_marked) =
        probe;

    assert_eq!(
        check_calls, 1,
        "`CheckAndDisplayPerformanceMessage` must poll one profiler message"
    );
    assert_eq!(
        marked_seen_calls, 1,
        "`CheckAndDisplayPerformanceMessage` must record the shown message exactly once"
    );
    assert!(
        marked_message_is_original,
        "`AddPerformanceMessageShown` must receive the profiler message object"
    );
    assert_eq!(
        popup_calls, 1,
        "successful overall-error messages must continue through `DisplayMessage`"
    );
    assert!(
        message_marked,
        "successful tick must mark the message type in `shownPerformanceMessages`"
    );
}
