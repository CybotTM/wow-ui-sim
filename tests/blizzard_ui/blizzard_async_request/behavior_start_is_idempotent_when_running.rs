//! StartRequest re-entry guard probes for `Blizzard_AsyncRequest`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AsyncRequest";

#[test]
fn start_request_is_idempotent_while_running_and_runs_again_after_stop() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let probe: StartRequestIdempotenceProbe = env
            .eval(
                r#"
                local calls = {}
                local request = AsyncRequests:CreateAsyncRequest({
                    requestFunction = function(value)
                        table.insert(calls, value)
                    end,
                    responseEventName = "PLAYER_LOGIN",
                    responseEventCallback = function() end,
                })

                request:StartRequest("first")
                request:StartRequest("second-while-running")
                local runningAfterDuplicateStart = request.isRunning

                request:StopRequest()
                local runningAfterStop = request.isRunning

                request:StartRequest("second")

                return #calls,
                       calls[1],
                       calls[2],
                       runningAfterDuplicateStart,
                       runningAfterStop,
                       request.isRunning
                "#,
            )
            .expect("StartRequest idempotence probe must run cleanly");

        assert_start_request_idempotence(probe);
    });
}

type StartRequestIdempotenceProbe = (i32, String, String, bool, bool, bool);

fn assert_start_request_idempotence(probe: StartRequestIdempotenceProbe) {
    let (
        call_count,
        first_call_arg,
        second_call_arg,
        running_after_duplicate_start,
        running_after_stop,
        running_after_restart,
    ) = probe;

    assert_eq!(
        call_count, 2,
        "StartRequest must run once per stopped cycle"
    );
    assert_eq!(
        first_call_arg, "first",
        "running request must keep first args"
    );
    assert_eq!(
        second_call_arg, "second",
        "StartRequest must run again after StopRequest"
    );
    assert!(
        running_after_duplicate_start,
        "duplicate StartRequest must leave the request running"
    );
    assert!(!running_after_stop, "StopRequest must clear isRunning");
    assert!(
        running_after_restart,
        "StartRequest after StopRequest must set isRunning again"
    );
}
