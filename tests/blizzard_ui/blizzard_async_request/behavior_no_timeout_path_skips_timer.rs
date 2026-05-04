//! No-timeout StartRequest probes for `Blizzard_AsyncRequest`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AsyncRequest";

#[test]
fn start_request_without_timeout_skips_timer_creation() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let timer_count_before_start = timer_count(env);

        let probe: NoTimeoutPathProbe = env
            .eval(
                r#"
                local responseCount = 0
                local request = AsyncRequests:CreateAsyncRequest({
                    requestFunction = function() end,
                    responseEventName = "PLAYER_LOGIN",
                    responseEventCallback = function()
                        responseCount = responseCount + 1
                    end,
                })

                request:StartRequest()
                local timeoutTimerAfterStartIsNil = request.timeoutTimer == nil
                local runningAfterStart = request.isRunning

                FireEvent("PLAYER_LOGIN")

                return responseCount,
                       timeoutTimerAfterStartIsNil,
                       runningAfterStart,
                       request.isRunning,
                       request:IsEventRegistered("PLAYER_LOGIN"),
                       request.timeoutTimer == nil
                "#,
            )
            .expect("no-timeout path probe must run cleanly");

        assert_no_timeout_path_probe(probe);
        assert_eq!(
            timer_count(env),
            timer_count_before_start,
            "StartRequest without timeoutSeconds must not create a timer"
        );
    });
}

fn timer_count(env: &wow_ui_sim::lua_api::WowLuaEnv) -> usize {
    env.state().borrow().rilua_timers.len()
}

type NoTimeoutPathProbe = (i32, bool, bool, bool, bool, bool);

fn assert_no_timeout_path_probe(probe: NoTimeoutPathProbe) {
    let (
        response_count,
        timeout_timer_after_start_is_nil,
        running_after_start,
        is_running,
        is_event_registered,
        timeout_timer_after_response_is_nil,
    ) = probe;

    assert_eq!(response_count, 1, "response callback must run exactly once");
    assert!(
        timeout_timer_after_start_is_nil,
        "StartRequest without timeoutSeconds must leave timeoutTimer nil"
    );
    assert!(running_after_start, "StartRequest must set isRunning");
    assert!(!is_running, "response event must stop the async request");
    assert!(
        !is_event_registered,
        "response event must unregister the response event"
    );
    assert!(
        timeout_timer_after_response_is_nil,
        "response event must leave timeoutTimer nil"
    );
}
