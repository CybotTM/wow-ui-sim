//! StopRequest-before-StartRequest probes for `Blizzard_AsyncRequest`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AsyncRequest";

#[test]
fn stop_request_without_start_is_safe() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        env.exec(
            r#"
            _G.AsyncRequestStopWithoutStartProbe = {
                request = AsyncRequests:CreateAsyncRequest({
                    requestFunction = function() end,
                    responseEventName = "PLAYER_LOGIN",
                    responseEventCallback = function() end,
                    timeoutSeconds = 10,
                    timeoutCallback = function() end,
                }),
            }
            "#,
        )
        .expect("stop-without-start setup must run cleanly");

        let timer_count_before_stop = timer_count(env);

        env.exec("_G.AsyncRequestStopWithoutStartProbe.request:StopRequest()")
            .expect("StopRequest before StartRequest must not raise a Lua error");

        let probe: StopWithoutStartProbe = env
            .eval(
                r#"
                local request = _G.AsyncRequestStopWithoutStartProbe.request

                return request.isRunning,
                       request.timeoutTimer == nil,
                       request:IsEventRegistered("PLAYER_LOGIN")
                "#,
            )
            .expect("stop-without-start state probe must run cleanly");

        assert_stop_without_start_probe(probe);
        assert_eq!(
            timer_count(env),
            timer_count_before_stop,
            "StopRequest without StartRequest must not create or cancel a timer"
        );
    });
}

fn timer_count(env: &wow_ui_sim::lua_api::WowLuaEnv) -> usize {
    env.state().borrow().rilua_timers.len()
}

type StopWithoutStartProbe = (bool, bool, bool);

fn assert_stop_without_start_probe(probe: StopWithoutStartProbe) {
    let (is_running, has_nil_timeout_timer, is_event_registered) = probe;

    assert!(
        !is_running,
        "StopRequest before StartRequest must leave isRunning false"
    );
    assert!(
        has_nil_timeout_timer,
        "StopRequest before StartRequest must leave timeoutTimer unset"
    );
    assert!(
        !is_event_registered,
        "StopRequest before StartRequest must not register the response event"
    );
}
