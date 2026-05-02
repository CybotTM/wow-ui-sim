//! AddonList memory usage throttling behavior for `Blizzard_AddOnList`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_AddOnList";

#[test]
fn update_addon_memory_usage_is_throttled_to_more_than_fifteen_seconds() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        let (calls_after_first, calls_after_early, calls_after_late): MemoryThrottleProbe = env
            .eval(
                r#"
                local originalGetTime = GetTime
                local originalUpdateAddOnMemoryUsage = UpdateAddOnMemoryUsage
                local now = 100
                local calls = 0

                GetTime = function()
                    return now
                end
                UpdateAddOnMemoryUsage = function()
                    calls = calls + 1
                end

                AddonList.lastMemoryUpdate = nil
                AddonList:UpdateAddOnMemoryUsage()
                local callsAfterFirst = calls

                now = 114
                AddonList:UpdateAddOnMemoryUsage()
                local callsAfterEarly = calls

                now = 116
                AddonList:UpdateAddOnMemoryUsage()
                local callsAfterLate = calls

                GetTime = originalGetTime
                UpdateAddOnMemoryUsage = originalUpdateAddOnMemoryUsage

                return callsAfterFirst, callsAfterEarly, callsAfterLate
                "#,
            )
            .expect("AddonList memory usage throttle probe must run cleanly");

        assert_memory_throttle_probe((calls_after_first, calls_after_early, calls_after_late));
    });
}

type MemoryThrottleProbe = (i64, i64, i64);

fn assert_memory_throttle_probe(probe: MemoryThrottleProbe) {
    let (calls_after_first, calls_after_early, calls_after_late) = probe;

    assert_eq!(
        calls_after_first, 1,
        "`UpdateAddOnMemoryUsage` must run immediately when no previous update is recorded"
    );
    assert_eq!(
        calls_after_early, 1,
        "`UpdateAddOnMemoryUsage` must not run again before more than 15 seconds elapsed"
    );
    assert_eq!(
        calls_after_late, 2,
        "`UpdateAddOnMemoryUsage` must run once more after more than 15 seconds elapsed"
    );
}
