//! Optional timeout pair guard probes for `Blizzard_AsyncRequest`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AsyncRequest";

#[test]
fn create_async_request_requires_timeout_seconds_and_callback_together() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let probe: TimeoutPairGuardProbe = env
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

                local function baseInput()
                    return {
                        requestFunction = function() end,
                        responseEventName = "ASYNC_REQUEST_TEST_RESPONSE",
                        responseEventCallback = function() end,
                    }
                end

                local function probeGuard(input)
                    local callsBefore = guardCalls
                    local ok = pcall(function()
                        AsyncRequests:CreateAsyncRequest(input)
                    end)
                    return ok, guardCalls - callsBefore
                end

                local onlySeconds = baseInput()
                onlySeconds.timeoutSeconds = 5

                local onlyCallback = baseInput()
                onlyCallback.timeoutCallback = function() end

                local bothNilOk, bothNilCalls = probeGuard(baseInput())
                local bothSet = baseInput()
                bothSet.timeoutSeconds = 5
                bothSet.timeoutCallback = function() end
                local bothSetOk, bothSetCalls = probeGuard(bothSet)
                local onlySecondsOk, onlySecondsCalls = probeGuard(onlySeconds)
                local onlyCallbackOk, onlyCallbackCalls = probeGuard(onlyCallback)

                assertsafe = originalAssertSafe

                return bothNilOk,
                       bothNilCalls,
                       bothSetOk,
                       bothSetCalls,
                       onlySecondsOk == false,
                       onlySecondsCalls,
                       onlyCallbackOk == false,
                       onlyCallbackCalls
                "#,
            )
            .expect("CreateAsyncRequest timeout pair guard probe must run cleanly");

        assert_timeout_pair_guards(probe);
    });
}

type TimeoutPairGuardProbe = (bool, i32, bool, i32, bool, i32, bool, i32);

fn assert_timeout_pair_guards(probe: TimeoutPairGuardProbe) {
    let (
        both_nil_ok,
        both_nil_calls,
        both_set_ok,
        both_set_calls,
        only_seconds_failed,
        only_seconds_calls,
        only_callback_failed,
        only_callback_calls,
    ) = probe;

    assert!(both_nil_ok, "timeout fields may both be nil");
    assert_eq!(
        both_nil_calls, 5,
        "valid nil timeout pair must pass all constructor guards"
    );

    assert!(both_set_ok, "timeout fields may both be provided");
    assert_eq!(
        both_set_calls, 5,
        "valid timeout pair must pass all constructor guards"
    );

    assert!(
        only_seconds_failed,
        "timeoutSeconds without timeoutCallback must fail"
    );
    assert_eq!(
        only_seconds_calls, 5,
        "timeoutSeconds without timeoutCallback must stop at lua:94-95"
    );

    assert!(
        only_callback_failed,
        "timeoutCallback without timeoutSeconds must fail"
    );
    assert_eq!(
        only_callback_calls, 5,
        "timeoutCallback without timeoutSeconds must stop at lua:94-95"
    );
}
