//! Constructor guard probes for `Blizzard_AsyncRequest`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AsyncRequest";

#[test]
fn create_async_request_requires_all_required_callbacks() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let probe: RequiredCallbackGuardProbe = env
            .eval(
                r#"
                local originalAssertSafe = assertsafe
                local guardCalls = 0

                assertsafe = function(condition)
                    guardCalls = guardCalls + 1
                    if not condition then
                        error("assertsafe guard failed", 2)
                    end
                end

                local function probeGuard(input)
                    local callsBefore = guardCalls
                    local ok = pcall(function()
                        AsyncRequests:CreateAsyncRequest(input)
                    end)
                    return ok == false, guardCalls - callsBefore
                end

                local nilInputFailed, nilInputCalls = probeGuard(nil)
                local emptyInputFailed, emptyInputCalls = probeGuard({})
                local noEventNameFailed, noEventNameCalls = probeGuard({
                    requestFunction = function() end,
                })
                local noCallbackFailed, noCallbackCalls = probeGuard({
                    requestFunction = function() end,
                    responseEventName = "ASYNC_REQUEST_TEST_RESPONSE",
                })

                assertsafe = originalAssertSafe

                return nilInputFailed,
                       nilInputCalls,
                       emptyInputFailed,
                       emptyInputCalls,
                       noEventNameFailed,
                       noEventNameCalls,
                       noCallbackFailed,
                       noCallbackCalls
                "#,
            )
            .expect("CreateAsyncRequest required callback guard probe must run cleanly");

        assert_required_callback_guards(probe);
    });
}

type RequiredCallbackGuardProbe = (bool, i32, bool, i32, bool, i32, bool, i32);

fn assert_required_callback_guards(probe: RequiredCallbackGuardProbe) {
    let (
        nil_input_failed,
        nil_input_calls,
        empty_input_failed,
        empty_input_calls,
        no_event_name_failed,
        no_event_name_calls,
        no_callback_failed,
        no_callback_calls,
    ) = probe;

    assert!(nil_input_failed, "`nil` input must fail the first guard");
    assert_eq!(nil_input_calls, 1, "`nil` input must stop at lua:86");

    assert!(empty_input_failed, "missing requestFunction must fail");
    assert_eq!(
        empty_input_calls, 2,
        "missing requestFunction must stop at lua:89"
    );

    assert!(no_event_name_failed, "missing responseEventName must fail");
    assert_eq!(
        no_event_name_calls, 3,
        "missing responseEventName must stop at lua:90"
    );

    assert!(
        no_callback_failed,
        "missing responseEventCallback must fail"
    );
    assert_eq!(
        no_callback_calls, 4,
        "missing responseEventCallback must stop at lua:91"
    );
}
