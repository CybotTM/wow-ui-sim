//! Non-matching response event probes for `Blizzard_AsyncRequest`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AsyncRequest";

#[test]
fn non_matching_response_event_is_ignored() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let probe: NonMatchingResponseEventProbe = env
            .eval(
                r#"
                local callbackCount = 0
                local request = AsyncRequests:CreateAsyncRequest({
                    requestFunction = function() end,
                    responseEventName = "PLAYER_LOGIN",
                    responseEventCallback = function()
                        callbackCount = callbackCount + 1
                    end,
                })

                request:StartRequest()
                FireEvent("PLAYER_LOGOUT")
                local callbackCountAfterEvent = callbackCount

                request:GetScript("OnEvent")(request, "PLAYER_LOGOUT")

                return callbackCountAfterEvent,
                       callbackCount,
                       request.isRunning,
                       request:IsEventRegistered("PLAYER_LOGIN")
                "#,
            )
            .expect("non-matching response event probe must run cleanly");

        assert_non_matching_response_event_probe(probe);
    });
}

type NonMatchingResponseEventProbe = (i32, i32, bool, bool);

fn assert_non_matching_response_event_probe(probe: NonMatchingResponseEventProbe) {
    let (
        callback_count_after_event,
        callback_count_after_direct_dispatch,
        is_running,
        is_response_event_registered,
    ) = probe;

    assert_eq!(
        callback_count_after_event, 0,
        "unregistered non-response event must not reach the request handler"
    );
    assert_eq!(
        callback_count_after_direct_dispatch, 0,
        "direct non-response OnEvent dispatch must not invoke the callback"
    );
    assert!(
        is_running,
        "non-response event must leave the async request running"
    );
    assert!(
        is_response_event_registered,
        "non-response event must leave the response event registered"
    );
}
