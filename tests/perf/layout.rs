use std::path::Path;
use std::time::{Duration, Instant};

use wow_ui_sim::iced_app::build_quad_batch_for_registry;
use wow_ui_sim::iced_app::tooltip::collect_tooltip_data;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::render::{GlyphAtlas, WowFontSystem};
use wow_ui_sim::widget::Anchor;

const PERF_SCREEN_SIZE: (f32, f32) = (1024.0, 768.0);
const PERF_FONTS_PATH: &str = "./fonts";

pub fn measure_full_root_layout_pass(env: &WowLuaEnv) -> Duration {
    let ui_parent_id = {
        let state = env.state().borrow();
        state
            .widgets
            .get_id_by_name("UIParent")
            .expect("UIParent should exist in the settled game UI")
    };

    let started = Instant::now();
    {
        let mut state = env.state().borrow_mut();
        state.widgets.mark_rect_dirty(ui_parent_id);
        state.invalidate_layout(ui_parent_id);
    }
    started.elapsed()
}

pub fn measure_incremental_anchor_change_layout_pass(env: &WowLuaEnv) -> Duration {
    let player_frame_id = find_player_frame_id(env);
    let original_anchors = snapshot_frame_anchors(env, player_frame_id);

    let started = Instant::now();
    {
        let mut state = env.state().borrow_mut();
        shift_first_anchor_and_relayout(&mut state, player_frame_id);
        restore_anchors_and_relayout(&mut state, player_frame_id, original_anchors);
    }
    started.elapsed()
}

pub fn measure_strata_bucket_rebuild(env: &WowLuaEnv) -> Duration {
    let started = Instant::now();
    {
        let mut state = env.state().borrow_mut();
        state.strata_buckets = None;
        let _ = state
            .get_strata_buckets()
            .expect("strata buckets should rebuild for the settled game UI");
    }
    started.elapsed()
}

pub fn measure_full_quad_batch_build(env: &WowLuaEnv) -> Duration {
    let mut font_system = WowFontSystem::new(Path::new(PERF_FONTS_PATH));
    let mut glyph_atlas = GlyphAtlas::new();

    let started = Instant::now();
    {
        let mut state = env.state().borrow_mut();
        let strata_buckets = state
            .get_strata_buckets()
            .expect("strata buckets should exist for quad batch measurement")
            .clone();
        let tooltip_data = collect_tooltip_data(&state);
        let text_ctx = Some((&mut font_system, &mut glyph_atlas));

        let _batch = build_quad_batch_for_registry(
            &state.widgets,
            PERF_SCREEN_SIZE,
            None,
            None,
            None,
            text_ctx,
            Some(&state.message_frames),
            Some(&tooltip_data),
            &strata_buckets,
        );
    }
    started.elapsed()
}

fn find_player_frame_id(env: &WowLuaEnv) -> u64 {
    let state = env.state().borrow();
    state
        .widgets
        .get_id_by_name("PlayerFrame")
        .expect("PlayerFrame should exist in the settled game UI")
}

fn snapshot_frame_anchors(env: &WowLuaEnv, frame_id: u64) -> Vec<Anchor> {
    let state = env.state().borrow();
    let anchors = state
        .widgets
        .get(frame_id)
        .expect("frame should resolve from its widget id")
        .anchors
        .clone();
    assert!(
        !anchors.is_empty(),
        "frame should have at least one anchor for incremental layout measurement"
    );
    anchors
}

fn shift_first_anchor_and_relayout(state: &mut wow_ui_sim::lua_api::SimState, frame_id: u64) {
    state
        .widgets
        .get_mut(frame_id)
        .expect("frame should resolve mutably from its widget id")
        .anchors[0]
        .x_offset += 1.0;
    state.widgets.mark_rect_dirty(frame_id);
    state.invalidate_layout(frame_id);
}

fn restore_anchors_and_relayout(
    state: &mut wow_ui_sim::lua_api::SimState,
    frame_id: u64,
    original_anchors: Vec<Anchor>,
) {
    state
        .widgets
        .get_mut(frame_id)
        .expect("frame should still exist after incremental layout")
        .anchors = original_anchors;
    state.widgets.mark_rect_dirty(frame_id);
    state.invalidate_layout(frame_id);
}
