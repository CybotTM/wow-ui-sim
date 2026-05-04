//! Concurrent request isolation probes for `Blizzard_AsyncRequest`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AsyncRequest";

#[test]
fn two_concurrent_requests_keep_response_state_isolated() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let probe: ConcurrentRequestsProbe = env
            .eval(
                r#"
                local requestPayloads = {}
                local responseCounts = { first = 0, second = 0 }

                local first = AsyncRequests:CreateAsyncRequest({
                    requestFunction = function(...)
                        requestPayloads.first = {...}
                    end,
                    responseEventName = "PLAYER_LOGIN",
                    responseEventCallback = function(...)
                        responseCounts.first = responseCounts.first + 1
                    end,
                })

                local second = AsyncRequests:CreateAsyncRequest({
                    requestFunction = function(...)
                        requestPayloads.second = {...}
                    end,
                    responseEventName = "PLAYER_LOGOUT",
                    responseEventCallback = function(...)
                        responseCounts.second = responseCounts.second + 1
                    end,
                })

                first:StartRequest("first-payload")
                second:StartRequest("second-payload")

                FireEvent("PLAYER_LOGIN")

                return requestPayloads.first[1],
                       requestPayloads.second[1],
                       responseCounts.first,
                       responseCounts.second,
                       first.isRunning,
                       second.isRunning,
                       first:IsEventRegistered("PLAYER_LOGIN"),
                       second:IsEventRegistered("PLAYER_LOGOUT"),
                       first == second
                "#,
            )
            .expect("concurrent request probe must run cleanly");

        assert_concurrent_requests_probe(probe);
    });
}

type ConcurrentRequestsProbe = (String, String, i32, i32, bool, bool, bool, bool, bool);

fn assert_concurrent_requests_probe(probe: ConcurrentRequestsProbe) {
    let (
        first_payload,
        second_payload,
        first_response_count,
        second_response_count,
        first_is_running,
        second_is_running,
        first_is_registered,
        second_is_registered,
        requests_are_same_frame,
    ) = probe;

    assert_request_payloads(first_payload, second_payload);
    assert_response_counts(first_response_count, second_response_count);
    assert_request_running_states(first_is_running, second_is_running);
    assert_event_registration_states(first_is_registered, second_is_registered);
    assert!(
        !requests_are_same_frame,
        "CreateAsyncRequest must allocate an independent frame per request"
    );
}

fn assert_request_payloads(first_payload: String, second_payload: String) {
    assert_eq!(first_payload, "first-payload");
    assert_eq!(second_payload, "second-payload");
}

fn assert_response_counts(first_response_count: i32, second_response_count: i32) {
    assert_eq!(
        first_response_count, 1,
        "first response event must invoke the first callback"
    );
    assert_eq!(
        second_response_count, 0,
        "first response event must not invoke the second callback"
    );
}

fn assert_request_running_states(first_is_running: bool, second_is_running: bool) {
    assert!(
        !first_is_running,
        "first response event must stop first request"
    );
    assert!(second_is_running, "second request must keep running");
}

fn assert_event_registration_states(first_is_registered: bool, second_is_registered: bool) {
    assert!(
        !first_is_registered,
        "first response event must unregister first request event"
    );
    assert!(
        second_is_registered,
        "second request must keep its response event registered"
    );
}
