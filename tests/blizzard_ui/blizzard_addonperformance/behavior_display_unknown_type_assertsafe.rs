//! AddOnPerformance invalid-message behavior for `Blizzard_AddOnPerformance`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AddOnPerformance";

#[test]
fn display_unknown_message_type_reports_assertsafe_without_throwing() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let probe: UnknownTypeAssertSafeProbe = env
            .eval(
                r#"
                local originalAssertSafe = assertsafe
                local originalStaticPopupShow = StaticPopup_Show
                local originalAddSystemMessage = ChatFrameUtil.AddSystemMessage
                local assertsafeCalls = 0
                local capturedCondition = nil
                local capturedMessage = nil
                local popupCalls = 0
                local chatCalls = 0

                assertsafe = function(condition, message)
                    assertsafeCalls = assertsafeCalls + 1
                    capturedCondition = condition
                    capturedMessage = message
                    return condition
                end
                StaticPopup_Show = function()
                    popupCalls = popupCalls + 1
                end
                ChatFrameUtil.AddSystemMessage = function()
                    chatCalls = chatCalls + 1
                end

                AddOnPerformance:DisplayMessage({
                    type = -1,
                    addOnName = "InvalidPerformanceProbe",
                })

                assertsafe = originalAssertSafe
                StaticPopup_Show = originalStaticPopupShow
                ChatFrameUtil.AddSystemMessage = originalAddSystemMessage

                return assertsafeCalls,
                       capturedCondition == false,
                       capturedMessage,
                       popupCalls,
                       chatCalls
                "#,
            )
            .expect("invalid AddOnPerformance message must not throw");

        assert_unknown_type_assertsafe_probe(probe);
    });
}

type UnknownTypeAssertSafeProbe = (i64, bool, String, i64, i64);

fn assert_unknown_type_assertsafe_probe(probe: UnknownTypeAssertSafeProbe) {
    let (assertsafe_calls, captured_condition_is_false, captured_message, popup_calls, chat_calls) =
        probe;

    assert_eq!(
        assertsafe_calls, 1,
        "invalid message type must invoke `assertsafe` exactly once"
    );
    assert!(
        captured_condition_is_false,
        "invalid message type must call `assertsafe(false, ...)`"
    );
    assert_eq!(
        captured_message, "Invalid addon performance msg.",
        "invalid message type must report the Blizzard guard message"
    );
    assert_eq!(
        popup_calls, 0,
        "invalid message type must not route through popup display paths"
    );
    assert_eq!(
        chat_calls, 0,
        "invalid message type must not route through chat display paths"
    );
}
