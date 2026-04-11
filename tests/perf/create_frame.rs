use std::time::{Duration, Instant};

use wow_ui_sim::lua_api::WowLuaEnv;

const CREATE_FRAME_COUNT: usize = 1000;

pub fn measure_create_frame_throughput() -> Duration {
    let env = WowLuaEnv::new().expect("failed to create Lua env for CreateFrame perf test");
    install_create_frame_perf_helper(&env);

    let frames_before = env.state().borrow().widgets.iter_ids().count();

    let started = Instant::now();
    env.exec(&format!("__perf_create_frames({CREATE_FRAME_COUNT})"))
        .expect("CreateFrame perf helper should run");
    let elapsed = started.elapsed();

    let frames_after = env.state().borrow().widgets.iter_ids().count();
    assert_eq!(
        frames_after - frames_before,
        CREATE_FRAME_COUNT,
        "CreateFrame perf helper should register exactly {CREATE_FRAME_COUNT} new frames"
    );

    elapsed
}

fn install_create_frame_perf_helper(env: &WowLuaEnv) {
    env.exec_named(
        r#"
        function __perf_create_frames(count)
            for i = 1, count do
                CreateFrame("Frame", nil, UIParent)
            end
        end
    "#,
        "=[perf/create_frame_helper]",
    )
    .expect("failed to install CreateFrame perf helper");
}
