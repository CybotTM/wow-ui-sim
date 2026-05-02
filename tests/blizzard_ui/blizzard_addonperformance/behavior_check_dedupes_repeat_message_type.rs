//! AddOnPerformance repeat-message behavior for `Blizzard_AddOnPerformance`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AddOnPerformance";

#[test]
fn check_dedupes_repeat_message_type_before_displaying_again() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let probe: RepeatMessageProbe = env
            .eval(
                r#"
                local originalInCombatLockdown = InCombatLockdown
                local originalCheckForPerformanceMessage = C_AddOnProfiler.CheckForPerformanceMessage
                local originalAddPerformanceMessageShown = C_AddOnProfiler.AddPerformanceMessageShown
                local originalStaticPopupShow = StaticPopup_Show
                local originalDisplayMessage = AddOnPerformance.DisplayMessage

                local checkCalls = 0
                local markedSeenCalls = 0
                local displayCalls = 0
                local popupCalls = 0
                local message = {
                    type = Enum.AddOnPerformanceMessageType.OverallAddOnErrorDialog,
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
                AddOnPerformance.DisplayMessage = function(self, msg)
                    displayCalls = displayCalls + 1
                    return originalDisplayMessage(self, msg)
                end

                AddOnPerformance:CheckAndDisplayPerformanceMessage()
                AddOnPerformance:CheckAndDisplayPerformanceMessage()

                InCombatLockdown = originalInCombatLockdown
                C_AddOnProfiler.CheckForPerformanceMessage = originalCheckForPerformanceMessage
                C_AddOnProfiler.AddPerformanceMessageShown = originalAddPerformanceMessageShown
                StaticPopup_Show = originalStaticPopupShow
                AddOnPerformance.DisplayMessage = originalDisplayMessage

                return checkCalls,
                       markedSeenCalls,
                       displayCalls,
                       popupCalls,
                       AddOnPerformance.shownPerformanceMessages[message.type] == true
                "#,
            )
            .expect("AddOnPerformance repeat-message probe must run cleanly");

        assert_repeat_message_probe(probe);
    });
}

type RepeatMessageProbe = (i64, i64, i64, i64, bool);

fn assert_repeat_message_probe(probe: RepeatMessageProbe) {
    let (check_calls, marked_seen_calls, display_calls, popup_calls, message_marked) = probe;

    assert_eq!(
        check_calls, 2,
        "`CheckAndDisplayPerformanceMessage` must poll on both checks"
    );
    assert_eq!(
        marked_seen_calls, 1,
        "repeat message types must be recorded as shown only once"
    );
    assert_eq!(
        display_calls, 1,
        "repeat message types must short-circuit before `DisplayMessage`"
    );
    assert_eq!(
        popup_calls, 1,
        "repeat message types must not show a second popup"
    );
    assert!(
        message_marked,
        "first successful tick must cache the message type as shown"
    );
}
