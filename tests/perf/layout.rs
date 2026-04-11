use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, Instant};

use iced::Size;
use wow_ui_sim::iced_app::tooltip::collect_tooltip_data;
use wow_ui_sim::iced_app::{
    build_quad_batch_for_registry, rebuild_dirty_strata_batches_for_registry,
};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::render::{FrameQuadSnapshot, GlyphAtlas, QuadBatch, WowFontSystem};
use wow_ui_sim::widget::{Anchor, FrameStrata};

const PERF_SCREEN_SIZE: (f32, f32) = (1024.0, 768.0);
const PERF_FONTS_PATH: &str = "./fonts";
const PERF_DIRTY_TEXTURE_NAME: &str = "PerfDirtyTexture";

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

pub fn measure_dirty_tree_quad_rebuild(env: &WowLuaEnv) -> Duration {
    ensure_perf_dirty_texture_exists(env);

    let mut font_system = WowFontSystem::new(Path::new(PERF_FONTS_PATH));
    let mut glyph_atlas = GlyphAtlas::new();
    let mut strata_cache = empty_strata_cache();
    let mut snapshot_cache = empty_snapshot_cache();

    let mut state = env.state().borrow_mut();
    state.ensure_layout_rects();

    let strata_buckets = state
        .get_strata_buckets()
        .expect("strata buckets should exist for dirty-tree quad measurement")
        .clone();
    let mut tooltip_data = collect_tooltip_data(&state);
    let elapsed_secs = state.start_time.elapsed().as_secs_f64();
    let mut text_ctx = Some((&mut font_system, &mut glyph_atlas));

    rebuild_dirty_strata_batches_for_registry(
        &mut strata_cache,
        &mut snapshot_cache,
        &mut text_ctx,
        full_dirty_mask(),
        None,
        perf_screen_size(),
        &strata_buckets,
        &state.widgets,
        None,
        &state.message_frames,
        &tooltip_data,
        &state.quest_blobs,
        elapsed_secs,
    );

    let _ = state.widgets.take_render_dirty_with_ids();

    let dirty_texture_id = state
        .widgets
        .get_id_by_name(PERF_DIRTY_TEXTURE_NAME)
        .expect("perf dirty texture should resolve by name");
    let original_alpha = state
        .widgets
        .get(dirty_texture_id)
        .expect("perf dirty texture should exist")
        .alpha;
    let updated_alpha = (original_alpha * 0.5).max(0.1);
    state
        .widgets
        .get_mut_visual(dirty_texture_id)
        .expect("perf dirty texture should be mutable")
        .alpha = updated_alpha;

    let (dirty_mask, dirty_ids) = state.widgets.take_render_dirty_with_ids();
    assert_ne!(
        dirty_mask, 0,
        "single-frame visual change should dirty a strata"
    );
    let dirty_ids = dirty_ids.expect("single-frame visual change should stay incremental");
    assert!(
        dirty_ids.contains(&dirty_texture_id),
        "dirty IDs should contain the changed texture frame"
    );

    tooltip_data = collect_tooltip_data(&state);
    let elapsed_secs = state.start_time.elapsed().as_secs_f64();
    let started = Instant::now();
    rebuild_dirty_strata_batches_for_registry(
        &mut strata_cache,
        &mut snapshot_cache,
        &mut text_ctx,
        dirty_mask,
        Some(&dirty_ids),
        perf_screen_size(),
        &strata_buckets,
        &state.widgets,
        None,
        &state.message_frames,
        &tooltip_data,
        &state.quest_blobs,
        elapsed_secs,
    );
    let elapsed = started.elapsed();

    state
        .widgets
        .get_mut_visual(dirty_texture_id)
        .expect("perf dirty texture should still be mutable")
        .alpha = original_alpha;
    let _ = state.widgets.take_render_dirty_with_ids();

    elapsed
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

fn ensure_perf_dirty_texture_exists(env: &WowLuaEnv) {
    env.eval::<()>(
        r#"
        if not _G.PerfDirtyFrame then
            local frame = CreateFrame("Frame", "PerfDirtyFrame", UIParent)
            frame:SetSize(32, 32)
            frame:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 12, -12)
            frame:Show()

            local texture = frame:CreateTexture("PerfDirtyTexture", "ARTWORK")
            texture:SetAllPoints()
            texture:SetColorTexture(1, 0.25, 0.25, 1)
            texture:Show()
        end
    "#,
    )
    .expect("perf dirty texture setup should succeed");
}

fn full_dirty_mask() -> u16 {
    (1u16 << FrameStrata::COUNT) - 1
}

fn perf_screen_size() -> Size {
    Size::new(PERF_SCREEN_SIZE.0, PERF_SCREEN_SIZE.1)
}

fn empty_strata_cache() -> [Option<Arc<QuadBatch>>; FrameStrata::COUNT] {
    std::array::from_fn(|_| None)
}

fn empty_snapshot_cache()
-> [Option<std::collections::HashMap<u64, FrameQuadSnapshot>>; FrameStrata::COUNT] {
    std::array::from_fn(|_| None)
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
