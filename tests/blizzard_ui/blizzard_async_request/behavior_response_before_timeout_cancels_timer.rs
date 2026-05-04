//! Response-before-timeout probes for `Blizzard_AsyncRequest`.

use std::time::Instant;

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AsyncRequest";

#[test]
fn response_event_before_timeout_cancels_timeout_timer() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        env.exec(
            r#"
            _G.AsyncRequestResponseBeforeTimeoutProbe = {
                responseCount = 0,
                timeoutCount = 0,
            }

            _G.AsyncRequestResponseBeforeTimeoutProbe.request =
                AsyncRequests:CreateAsyncRequest({
                    requestFunction = function() end,
                    responseEventName = "PLAYER_LOGIN",
                    responseEventCallback = function()
                        _G.AsyncRequestResponseBeforeTimeoutProbe.responseCount =
                            _G.AsyncRequestResponseBeforeTimeoutProbe.responseCount + 1
                    end,
                    timeoutSeconds = 10,
                    timeoutCallback = function()
                        _G.AsyncRequestResponseBeforeTimeoutProbe.timeoutCount =
                            _G.AsyncRequestResponseBeforeTimeoutProbe.timeoutCount + 1
                    end,
                })

            _G.AsyncRequestResponseBeforeTimeoutProbe.request:StartRequest()
            _G.AsyncRequestResponseBeforeTimeoutProbe.timeoutTimerID =
                _G.AsyncRequestResponseBeforeTimeoutProbe.request.timeoutTimer.__id

            FireEvent("PLAYER_LOGIN")
            "#,
        )
        .expect("response-before-timeout setup must run cleanly");

        let timer_id = timeout_timer_id(env);
        make_pending_timer_ready(env, timer_id);
        env.process_timers()
            .expect("processing canceled timeout timer must not error");

        let probe: ResponseBeforeTimeoutProbe = env
            .eval(
                r#"
                local request = _G.AsyncRequestResponseBeforeTimeoutProbe.request

                return _G.AsyncRequestResponseBeforeTimeoutProbe.responseCount,
                       _G.AsyncRequestResponseBeforeTimeoutProbe.timeoutCount,
                       request.isRunning,
                       request:IsEventRegistered("PLAYER_LOGIN"),
                       request.timeoutTimer == nil
                "#,
            )
            .expect("response-before-timeout state probe must run cleanly");

        assert_response_before_timeout_probe(probe);
    });
}

fn timeout_timer_id(env: &wow_ui_sim::lua_api::WowLuaEnv) -> u64 {
    let timer_id: i64 = env
        .eval("return _G.AsyncRequestResponseBeforeTimeoutProbe.timeoutTimerID")
        .expect("timeout timer id must be captured before response");
    timer_id as u64
}

fn make_pending_timer_ready(env: &wow_ui_sim::lua_api::WowLuaEnv, timer_id: u64) {
    let now = Instant::now();
    let state = env.state();
    let mut state = state.borrow_mut();
    let mut found_timer = false;

    for timer in &mut state.rilua_timers {
        if timer.id == timer_id {
            found_timer = true;
            assert!(
                timer.cancelled,
                "response event must cancel the timeout timer"
            );
            timer.fire_at = now;
        }
    }

    assert!(
        found_timer,
        "timeout timer must remain queued until processed"
    );
}

type ResponseBeforeTimeoutProbe = (i32, i32, bool, bool, bool);

fn assert_response_before_timeout_probe(probe: ResponseBeforeTimeoutProbe) {
    let (response_count, timeout_count, is_running, is_event_registered, has_nil_timeout_timer) =
        probe;

    assert_eq!(response_count, 1, "response callback must run exactly once");
    assert_eq!(
        timeout_count, 0,
        "timeout callback must not run after the response event"
    );
    assert!(!is_running, "response event must stop the async request");
    assert!(
        !is_event_registered,
        "response event must unregister the response event"
    );
    assert!(
        has_nil_timeout_timer,
        "response event must clear the timeout timer handle"
    );
}
