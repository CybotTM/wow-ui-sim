use std::time::{Duration, Instant};

use wow_ui_sim::lua_api::WowLuaEnv;

const EVENT_DISPATCH_FRAME_COUNT: usize = 128;
const PERF_EVENT_NAME: &str = "PLAYER_LOGIN";

pub fn measure_event_dispatch_throughput() -> Duration {
    let env = WowLuaEnv::new().expect("failed to create Lua env for event dispatch perf test");
    install_event_dispatch_perf_helpers(&env);
    register_event_dispatch_frames(&env);

    let started = Instant::now();
    env.fire_event(PERF_EVENT_NAME)
        .expect("event dispatch perf event should fire");
    let elapsed = started.elapsed();

    let handled_count: i64 = env
        .eval("return __perf_event_hits")
        .expect("event dispatch perf hit counter should be readable");
    assert_eq!(
        handled_count, EVENT_DISPATCH_FRAME_COUNT as i64,
        "event dispatch perf event should reach every registered frame"
    );

    elapsed
}

fn install_event_dispatch_perf_helpers(env: &WowLuaEnv) {
    env.exec_named(
        r#"
        __perf_event_hits = 0
        local perf_event_frames = {}

        function __perf_register_event_frames(count, eventName)
            for i = 1, count do
                local frame = CreateFrame("Frame", "PerfEventFrame" .. i, UIParent)
                frame:RegisterEvent(eventName)
                frame:SetScript("OnEvent", function()
                    __perf_event_hits = __perf_event_hits + 1
                end)
                perf_event_frames[i] = frame
            end
        end
    "#,
        "=[perf/event_dispatch_helper]",
    )
    .expect("failed to install event dispatch perf helpers");
}

fn register_event_dispatch_frames(env: &WowLuaEnv) {
    env.exec(&format!(
        "__perf_register_event_frames({EVENT_DISPATCH_FRAME_COUNT}, \"{PERF_EVENT_NAME}\")"
    ))
    .expect("event dispatch perf frame registration helper should run");
}
