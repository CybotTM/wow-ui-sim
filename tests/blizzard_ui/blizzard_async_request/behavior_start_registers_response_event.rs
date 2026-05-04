//! StartRequest event registration probes for `Blizzard_AsyncRequest`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

// `RegisterEvent` validates against real WoW event names, so this uses a
// registerable event while probing AsyncRequest's response-event field.
const RESPONSE_EVENT: &str = "PLAYER_LOGIN";
const ROOT: &str = "Blizzard_AsyncRequest";

#[test]
fn start_request_registers_and_stop_request_unregisters_response_event() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let probe: ResponseEventRegistrationProbe = env
            .eval(&format!(
                r#"
                local request = AsyncRequests:CreateAsyncRequest({{
                    requestFunction = function() end,
                    responseEventName = {response_event:?},
                    responseEventCallback = function() end,
                }})

                request:StartRequest()
                local registeredAfterStart = request:IsEventRegistered({response_event:?})

                request:StopRequest()
                local registeredAfterStop = request:IsEventRegistered({response_event:?})

                return registeredAfterStart,
                       registeredAfterStop
                "#,
                response_event = RESPONSE_EVENT,
            ))
            .expect("StartRequest event registration probe must run cleanly");

        assert_response_event_registration(probe);
    });
}

type ResponseEventRegistrationProbe = (bool, bool);

fn assert_response_event_registration(probe: ResponseEventRegistrationProbe) {
    let (registered_after_start, registered_after_stop) = probe;

    assert!(
        registered_after_start,
        "StartRequest must register the response event"
    );
    assert!(
        !registered_after_stop,
        "StopRequest must unregister the response event"
    );
}
