use std::time::{Duration, Instant};

use wow_ui_sim::iced_app::compute_frame_rect;
use wow_ui_sim::lua_api::WowLuaEnv;

const PERF_SCREEN_SIZE: (f32, f32) = (1024.0, 768.0);
const ANCHOR_FRAME_COUNT: usize = 1000;
const ANCHOR_FRAME_COLUMNS: usize = 40;
const ANCHOR_FRAME_SPACING: f32 = 8.0;
const ANCHOR_FRAME_SIZE: f32 = 12.0;
const LAST_ANCHOR_FRAME_NAME: &str = "PerfAnchorFrame1000";

pub fn measure_anchor_update_throughput() -> Duration {
    let env = WowLuaEnv::new().expect("failed to create Lua env for anchor perf test");
    install_anchor_perf_helpers(&env);
    create_anchor_perf_frames(&env);

    let ui_parent_id = find_frame_id(&env, "UIParent");

    let started = Instant::now();
    env.exec(&format!(
        "__perf_reanchor_frames({ANCHOR_FRAME_COLUMNS}, {ANCHOR_FRAME_SPACING}, {ANCHOR_FRAME_SIZE})"
    ))
    .expect("anchor perf reanchor helper should run");
    {
        let mut state = env.state().borrow_mut();
        state.widgets.mark_rect_dirty(ui_parent_id);
        state.invalidate_layout(ui_parent_id);
    }
    let elapsed = started.elapsed();

    assert_last_frame_anchor_resolved(&env);

    elapsed
}

fn install_anchor_perf_helpers(env: &WowLuaEnv) {
    env.exec_named(
        r#"
        local perf_anchor_frames = {}

        function __perf_create_anchor_frames(count, frameSize)
            for i = 1, count do
                local frame = CreateFrame("Frame", "PerfAnchorFrame" .. i, UIParent)
                frame:SetSize(frameSize, frameSize)
                perf_anchor_frames[i] = frame
            end
        end

        function __perf_reanchor_frames(columns, spacing, frameSize)
            for i, frame in ipairs(perf_anchor_frames) do
                local zeroBased = i - 1
                local column = math.fmod(zeroBased, columns)
                local row = math.floor(zeroBased / columns)
                frame:ClearAllPoints()
                frame:SetSize(frameSize, frameSize)
                frame:SetPoint(
                    "TOPLEFT",
                    UIParent,
                    "TOPLEFT",
                    column * spacing,
                    -(row * spacing)
                )
            end
        end
    "#,
        "=[perf/anchor_helper]",
    )
    .expect("failed to install anchor perf helpers");
}

fn create_anchor_perf_frames(env: &WowLuaEnv) {
    env.exec(&format!(
        "__perf_create_anchor_frames({ANCHOR_FRAME_COUNT}, {ANCHOR_FRAME_SIZE})"
    ))
    .expect("anchor perf frame creation helper should run");
}

fn assert_last_frame_anchor_resolved(env: &WowLuaEnv) {
    let last_frame_id = find_frame_id(env, LAST_ANCHOR_FRAME_NAME);
    let state = env.state().borrow();
    let frame = state
        .widgets
        .get(last_frame_id)
        .expect("last anchor perf frame should exist");
    assert_eq!(
        frame.anchors.len(),
        1,
        "last anchor perf frame should end with a single TOPLEFT anchor"
    );

    let rect = compute_frame_rect(
        &state.widgets,
        last_frame_id,
        PERF_SCREEN_SIZE.0,
        PERF_SCREEN_SIZE.1,
    );
    let expected_column = (ANCHOR_FRAME_COUNT - 1) % ANCHOR_FRAME_COLUMNS;
    let expected_row = (ANCHOR_FRAME_COUNT - 1) / ANCHOR_FRAME_COLUMNS;
    let expected_x = expected_column as f32 * ANCHOR_FRAME_SPACING;
    let expected_y = expected_row as f32 * ANCHOR_FRAME_SPACING;

    assert!(
        (rect.x - expected_x).abs() < 0.01,
        "last anchor perf frame should resolve x={expected_x}, got {}",
        rect.x
    );
    assert!(
        (rect.y - expected_y).abs() < 0.01,
        "last anchor perf frame should resolve y={expected_y}, got {}",
        rect.y
    );
}

fn find_frame_id(env: &WowLuaEnv, name: &str) -> u64 {
    let state = env.state().borrow();
    state
        .widgets
        .get_id_by_name(name)
        .unwrap_or_else(|| panic!("{name} should exist in the anchor perf env"))
}
