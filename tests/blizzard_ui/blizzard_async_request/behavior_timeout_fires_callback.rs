//! Timeout callback probes for `Blizzard_AsyncRequest`.

use std::time::Instant;

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AsyncRequest";

#[test]
fn timeout_timer_fires_callback_and_stops_request() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        env.exec(
            r#"
            _G.AsyncRequestTimeoutProbe = {
                timeoutCount = 0,
            }

            _G.AsyncRequestTimeoutProbe.request = AsyncRequests:CreateAsyncRequest({
                requestFunction = function() end,
                responseEventName = "PLAYER_LOGIN",
                responseEventCallback = function() end,
                timeoutSeconds = 2.5,
                timeoutCallback = function()
                    _G.AsyncRequestTimeoutProbe.timeoutCount =
                        _G.AsyncRequestTimeoutProbe.timeoutCount + 1
                end,
            })

            _G.AsyncRequestTimeoutProbe.request:StartRequest()
            "#,
        )
        .expect("timeout request setup must run cleanly");

        let timeout_timer_id = timeout_timer_id(env);
        make_pending_timer_ready(env, timeout_timer_id);
        let fired = env
            .process_timers()
            .expect("ready timeout timer must process");
        assert!(fired >= 1, "the timeout timer must be ready to fire");

        let probe: TimeoutCallbackProbe = env
            .eval(
                r#"
                local request = _G.AsyncRequestTimeoutProbe.request

                return _G.AsyncRequestTimeoutProbe.timeoutCount,
                       request.isRunning,
                       request:IsEventRegistered("PLAYER_LOGIN"),
                       request.timeoutTimer == nil
                "#,
            )
            .expect("timeout callback state probe must run cleanly");

        assert_timeout_callback_probe(probe);
    });
}

fn timeout_timer_id(env: &wow_ui_sim::lua_api::WowLuaEnv) -> u64 {
    let timer_id: i64 = env
        .eval("return _G.AsyncRequestTimeoutProbe.request.timeoutTimer.__id")
        .expect("timeout timer handle must expose an id");
    timer_id as u64
}

fn make_pending_timer_ready(env: &wow_ui_sim::lua_api::WowLuaEnv, timer_id: u64) {
    let now = Instant::now();
    let state = env.state();
    let mut state = state.borrow_mut();

    for timer in &mut state.rilua_timers {
        if timer.id == timer_id {
            timer.fire_at = now;
        }
    }
}

type TimeoutCallbackProbe = (i32, bool, bool, bool);

fn assert_timeout_callback_probe(probe: TimeoutCallbackProbe) {
    let (timeout_count, is_running, is_event_registered, has_nil_timeout_timer) = probe;

    assert_eq!(timeout_count, 1, "timeout callback must run exactly once");
    assert!(!is_running, "timeout callback must stop the async request");
    assert!(
        !is_event_registered,
        "timeout callback must unregister the response event"
    );
    assert!(
        has_nil_timeout_timer,
        "timeout callback must clear the timer handle"
    );
}
