use super::*;
use std::time::Duration;

#[test]
fn empty_runtime_state_new_seeds_expected_runtime_defaults() {
    let state = EmptyRuntimeState::new();

    assert_eq!(state.next_report_token, INITIAL_REPORT_TOKEN);
    assert_eq!(state.next_anim_group_id, INITIAL_ANIM_GROUP_ID);
    assert_eq!(state.next_cast_id, INITIAL_CAST_ID);
    assert_eq!(state.screen_width, DEFAULT_SCREEN_WIDTH);
    assert_eq!(state.screen_height, DEFAULT_SCREEN_HEIGHT);
    assert_eq!(state.screen_kind, ScreenKind::Game);
    assert!(!state.is_logged_in);
    assert!(!state.screen_first_displayed);
    assert!(state.focused_frame_id.is_none());
    assert!(state.hovered_frame.is_none());
    assert!(state.saved_account_name.is_empty());
    assert!(state.saved_account_list.is_empty());
    assert!(state.start_time.elapsed() < Duration::from_secs(1));
}

#[test]
fn post_event_workaround_marker_is_one_shot() {
    let mut state = SimState::default();

    assert!(!state.post_event_workarounds_applied);
    assert!(state.mark_post_event_workarounds_applied());
    assert!(state.post_event_workarounds_applied);
    assert!(!state.mark_post_event_workarounds_applied());
}
