use std::time::{Duration, Instant};

use wow_ui_sim::lua_api::WowLuaEnv;

const EVENT_DISPATCH_FRAME_COUNT: usize = 128;
const PERF_EVENT_NAME: &str = "PLAYER_LOGIN";

pub fn measure_event_dispatch_throughput() -> Duration {
    let env = WowLuaEnv::new().expect("failed to create Lua env for event dispatch perf test");
    install_event_dispatch_perf_helpers(&env);
    register_event_dispatch_frames(&env);
    let frame_count: i64 = env
        .eval("return __perf_event_frames and #__perf_event_frames or -1")
        .expect("event dispatch perf frame count should be readable");
    assert_eq!(
        frame_count, EVENT_DISPATCH_FRAME_COUNT as i64,
        "event dispatch perf helper should build the expected frame set"
    );
    let registered_count: i64 = env
        .eval("return __perf_event_registered or -1")
        .expect("event dispatch perf registration count should be readable");
    assert_eq!(
        registered_count, EVENT_DISPATCH_FRAME_COUNT as i64,
        "event dispatch perf helper should register every frame"
    );

    let started = Instant::now();
    env.exec(&format!("__perf_dispatch_event(\"{PERF_EVENT_NAME}\")"))
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
    install_event_dispatch_registration_helper(env);
    install_event_dispatch_dispatch_helper(env);
}

fn install_event_dispatch_registration_helper(env: &WowLuaEnv) {
    env.exec_named(
        r#"
        __perf_event_hits = 0
        local perf_event_frames = {}

        function __perf_register_event_frames(count, eventName)
            local registered = 0
            for i = 1, count do
                local frame = CreateFrame("Frame", "PerfEventFrame" .. i, UIParent)
                frame:RegisterEvent(eventName)
                if frame:IsEventRegistered(eventName) then
                    registered = registered + 1
                end
                frame:SetScript("OnEvent", function()
                    __perf_event_hits = __perf_event_hits + 1
                end)
                perf_event_frames[i] = frame
            end
            __perf_event_frames = perf_event_frames
            __perf_event_registered = registered
        end

    "#,
        "=[perf/event_dispatch_helper/register]",
    )
    .expect("failed to install event dispatch perf registration helper");
}

fn install_event_dispatch_dispatch_helper(env: &WowLuaEnv) {
    env.exec_named(
        r#"
        function __perf_dispatch_event(eventName)
            for i = 1, #__perf_event_frames do
                local frame = __perf_event_frames[i]
                local handler = frame:GetScript("OnEvent")
                if handler then
                    handler(frame, eventName)
                end
            end
        end
    "#,
        "=[perf/event_dispatch_helper/dispatch]",
    )
    .expect("failed to install event dispatch perf dispatch helper");
}

fn register_event_dispatch_frames(env: &WowLuaEnv) {
    env.exec(&format!(
        "__perf_register_event_frames({EVENT_DISPATCH_FRAME_COUNT}, \"{PERF_EVENT_NAME}\")"
    ))
    .expect("event dispatch perf frame registration helper should run");
}
