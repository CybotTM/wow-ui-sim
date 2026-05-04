//! Response event callback probes for `Blizzard_AsyncRequest`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AsyncRequest";

#[test]
fn response_event_forwards_payload_and_stops_request() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let probe: ResponseCallbackProbe = env
            .eval(
                r#"
                local callbackArgs = nil
                local request = AsyncRequests:CreateAsyncRequest({
                    requestFunction = function() end,
                    responseEventName = "PLAYER_LOGIN",
                    responseEventCallback = function(...)
                        callbackArgs = {...}
                    end,
                })

                request:StartRequest()
                FireEvent("PLAYER_LOGIN", 1, "two", false)

                return callbackArgs[1],
                       callbackArgs[2],
                       callbackArgs[3],
                       request.isRunning,
                       request:IsEventRegistered("PLAYER_LOGIN")
                "#,
            )
            .expect("response event callback probe must run cleanly");

        assert_response_callback_probe(probe);
    });
}

type ResponseCallbackProbe = (i32, String, bool, bool, bool);

fn assert_response_callback_probe(probe: ResponseCallbackProbe) {
    let (first_arg, second_arg, third_arg, is_running, is_event_registered) = probe;

    assert_eq!(first_arg, 1, "first response payload arg must be forwarded");
    assert_eq!(
        second_arg, "two",
        "second response payload arg must be forwarded"
    );
    assert!(
        !third_arg,
        "third response payload arg must be forwarded as false"
    );
    assert!(
        !is_running,
        "response event handler must stop the async request"
    );
    assert!(
        !is_event_registered,
        "response event handler must unregister the response event"
    );
}
