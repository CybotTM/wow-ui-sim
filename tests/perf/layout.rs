use std::time::{Duration, Instant};

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::widget::Anchor;

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
