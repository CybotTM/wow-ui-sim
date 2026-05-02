//! AddOnPerformance missing-addon-name behavior for `Blizzard_AddOnPerformance`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AddOnPerformance";

#[test]
fn specific_messages_without_addon_name_report_assertsafe() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let probe: MissingAddonNameProbe = env
            .eval(
                r#"
                local originalAssertSafe = assertsafe
                local originalStaticPopupShow = StaticPopup_Show
                local originalAddSystemMessage = ChatFrameUtil.AddSystemMessage
                local assertsafeCalls = 0
                local falseConditionCalls = 0
                local guardMessages = 0
                local popupCalls = 0
                local chatCalls = 0

                assertsafe = function(condition, message)
                    assertsafeCalls = assertsafeCalls + 1
                    if condition == false then
                        falseConditionCalls = falseConditionCalls + 1
                    end
                    if message == "Invalid addon performance msg." then
                        guardMessages = guardMessages + 1
                    end
                    return condition
                end
                StaticPopup_Show = function()
                    popupCalls = popupCalls + 1
                end
                ChatFrameUtil.AddSystemMessage = function()
                    chatCalls = chatCalls + 1
                end

                AddOnPerformance:DisplayMessage({
                    type = Enum.AddOnPerformanceMessageType.SpecificAddOnChatWarning,
                })
                AddOnPerformance:DisplayMessage({
                    type = Enum.AddOnPerformanceMessageType.SpecificAddOnErrorDialog,
                })

                assertsafe = originalAssertSafe
                StaticPopup_Show = originalStaticPopupShow
                ChatFrameUtil.AddSystemMessage = originalAddSystemMessage

                return assertsafeCalls,
                       falseConditionCalls,
                       guardMessages,
                       popupCalls,
                       chatCalls
                "#,
            )
            .expect("missing-addon-name AddOnPerformance probe must not throw");

        assert_missing_addon_name_probe(probe);
    });
}

type MissingAddonNameProbe = (i64, i64, i64, i64, i64);

fn assert_missing_addon_name_probe(probe: MissingAddonNameProbe) {
    let (assertsafe_calls, false_condition_calls, guard_messages, popup_calls, chat_calls) = probe;

    assert_eq!(
        assertsafe_calls, 2,
        "both specific message types without addon names must invoke `assertsafe`"
    );
    assert_eq!(
        false_condition_calls, 2,
        "missing addon names must call `assertsafe(false, ...)`"
    );
    assert_eq!(
        guard_messages, 2,
        "missing addon names must report the Blizzard invalid-message guard"
    );
    assert_eq!(
        popup_calls, 0,
        "missing addon names must not route through popup display paths"
    );
    assert_eq!(
        chat_calls, 0,
        "missing addon names must not route through chat display paths"
    );
}
