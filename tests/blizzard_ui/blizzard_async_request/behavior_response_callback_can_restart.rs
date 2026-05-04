//! Response callback restart-order probes for `Blizzard_AsyncRequest`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AsyncRequest";

#[test]
fn response_callback_restart_attempt_noops_until_stop_request_runs() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let probe: ResponseCallbackRestartProbe = env
            .eval(
                r#"
                local request
                local requestCount = 0
                local callbackCount = 0
                local payloads = {}

                request = AsyncRequests:CreateAsyncRequest({
                    requestFunction = function(payload)
                        requestCount = requestCount + 1
                        payloads[requestCount] = payload
                    end,
                    responseEventName = "PLAYER_LOGIN",
                    responseEventCallback = function()
                        callbackCount = callbackCount + 1
                        request:StartRequest("restart-payload")
                    end,
                })

                request:StartRequest("initial-payload")
                request:GetScript("OnEvent")(request, "PLAYER_LOGIN")

                return requestCount,
                       callbackCount,
                       payloads[1],
                       payloads[2],
                       request.isRunning,
                       request:IsEventRegistered("PLAYER_LOGIN")
                "#,
            )
            .expect("response callback restart probe must run cleanly");

        assert_response_callback_restart_probe(probe);
    });
}

type ResponseCallbackRestartProbe = (i32, i32, String, Option<String>, bool, bool);

fn assert_response_callback_restart_probe(probe: ResponseCallbackRestartProbe) {
    let (
        request_count,
        callback_count,
        first_payload,
        second_payload,
        running_after_event,
        event_registered_after_event,
    ) = probe;

    assert_restart_attempt_did_not_run_request_again(request_count, second_payload);
    assert_callback_and_initial_payload(callback_count, first_payload);
    assert_request_stopped_after_callback(running_after_event, event_registered_after_event);
}

fn assert_restart_attempt_did_not_run_request_again(
    request_count: i32,
    second_payload: Option<String>,
) {
    assert_eq!(
        request_count, 1,
        "restart attempt inside response callback must no-op while request is still running"
    );
    assert_eq!(
        second_payload, None,
        "restart payload must not reach requestFunction"
    );
}

fn assert_callback_and_initial_payload(callback_count: i32, first_payload: String) {
    assert_eq!(callback_count, 1, "response callback must run exactly once");
    assert_eq!(first_payload, "initial-payload");
}

fn assert_request_stopped_after_callback(
    running_after_event: bool,
    event_registered_after_event: bool,
) {
    assert!(
        !running_after_event,
        "StopRequest must run after the response callback returns"
    );
    assert!(
        !event_registered_after_event,
        "StopRequest must unregister the response event after callback"
    );
}
