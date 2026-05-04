//! StartRequest behavior probes for `Blizzard_AsyncRequest`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AsyncRequest";

#[test]
fn start_request_forwards_varargs_and_sets_running_state() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let probe: StartRequestProbe = env
            .eval(
                r#"
                local recordedArgs = nil
                local request = AsyncRequests:CreateAsyncRequest({
                    requestFunction = function(...)
                        recordedArgs = {...}
                    end,
                    responseEventName = "PLAYER_LOGIN",
                    responseEventCallback = function() end,
                })

                request:StartRequest("a", 42, true)

                return recordedArgs[1],
                       recordedArgs[2],
                       recordedArgs[3],
                       request.isRunning
                "#,
            )
            .expect("StartRequest vararg probe must run cleanly");

        assert_start_request_probe(probe);
    });
}

type StartRequestProbe = (String, i32, bool, bool);

fn assert_start_request_probe(probe: StartRequestProbe) {
    let (first_arg, second_arg, third_arg, is_running) = probe;

    assert_eq!(first_arg, "a", "first vararg must be forwarded");
    assert_eq!(second_arg, 42, "second vararg must be forwarded");
    assert!(third_arg, "third vararg must be forwarded");
    assert!(is_running, "StartRequest must flip isRunning to true");
}
