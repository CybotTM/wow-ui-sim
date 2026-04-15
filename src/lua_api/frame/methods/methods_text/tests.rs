use super::*;
use crate::lua_api::SimState;
use crate::widget::{Frame, WidgetType};

#[test]
fn collect_fontstring_measure_ids_skips_unchanged_text_targets() {
    let mut state = SimState::default();
    let mut frame = Frame::new(WidgetType::FontString, Some("Duration".to_string()), None);
    frame.text = Some("1m".to_string());
    frame.font = Some("Fonts\\FRIZQT__.TTF".to_string());
    frame.font_size = 12.0;
    frame.width = 12.0;
    frame.width_is_text_auto = true;
    frame.height = 15.0;
    let id = frame.id;
    state.widgets.register(frame);

    let ids = collect_fontstring_measure_ids(&state, &[(id, false)]);
    assert!(ids.is_empty(), "unchanged text should not be remeasured");
}

#[test]
fn collect_fontstring_measure_ids_keeps_changed_fontstrings() {
    let mut state = SimState::default();
    let mut frame = Frame::new(WidgetType::FontString, Some("Duration".to_string()), None);
    frame.text = Some("59s".to_string());
    frame.font = Some("Fonts\\FRIZQT__.TTF".to_string());
    frame.font_size = 12.0;
    frame.width = 18.0;
    frame.width_is_text_auto = true;
    frame.height = 15.0;
    let id = frame.id;
    state.widgets.register(frame);

    let ids = collect_fontstring_measure_ids(&state, &[(id, true)]);
    assert_eq!(ids.len(), 1, "changed text should still be measured");
    assert_eq!(ids[0].id, id);
    assert_eq!(ids[0].text, "59s");
}

fn make_button_with_text_child(state: &mut SimState, text: &str) -> (u64, u64) {
    let mut button = Frame::new(WidgetType::Button, Some("Btn".to_string()), None);
    button.text = Some(text.to_string());
    let button_id = button.id;
    state.widgets.register(button);

    let mut child = Frame::new(WidgetType::FontString, None, Some(button_id));
    child.text = Some(text.to_string());
    child.width = 100.0;
    child.height = 15.0;
    let child_id = child.id;
    state.widgets.register(child);
    state.widgets.add_child(button_id, child_id);
    if let Some(btn) = state.widgets.get_mut_visual(button_id) {
        btn.children_keys.insert("Text".to_string(), child_id);
    }
    (button_id, child_id)
}

#[test]
fn set_text_noop_detects_matching_text_on_button_with_text_child() {
    let mut state = SimState::default();
    let (id, _child) = make_button_with_text_child(&mut state, "Leave Party");
    assert!(is_set_text_noop(
        &state,
        id,
        &Some("Leave Party".to_string())
    ));
}

#[test]
fn set_text_noop_rejects_changed_text_on_button() {
    let mut state = SimState::default();
    let (id, _child) = make_button_with_text_child(&mut state, "Leave Party");
    assert!(!is_set_text_noop(
        &state,
        id,
        &Some("Leave Instance Group".to_string())
    ));
}

#[test]
fn set_text_noop_rejects_button_without_text_child_when_setting_non_nil() {
    let mut state = SimState::default();
    let button = Frame::new(WidgetType::Button, Some("Btn".to_string()), None);
    let id = button.id;
    state.widgets.register(button);
    // First SetText must take the slow path to create the lazy Text child.
    assert!(!is_set_text_noop(
        &state,
        id,
        &Some("Leave Party".to_string())
    ));
}

#[test]
fn set_text_noop_detects_matching_text_on_fontstring() {
    let mut state = SimState::default();
    let mut frame = Frame::new(WidgetType::FontString, Some("Label".to_string()), None);
    frame.text = Some("Hello".to_string());
    frame.width = 35.0;
    frame.width_is_text_auto = true;
    frame.height = 15.0;
    let id = frame.id;
    state.widgets.register(frame);
    assert!(is_set_text_noop(&state, id, &Some("Hello".to_string())));
}

#[test]
fn set_text_noop_rejects_fontstring_that_needs_remeasurement() {
    let mut state = SimState::default();
    let mut frame = Frame::new(WidgetType::FontString, Some("Label".to_string()), None);
    frame.text = Some("Hello".to_string());
    frame.width = 35.0;
    frame.width_is_text_auto = true;
    frame.height = 0.0;
    let id = frame.id;
    state.widgets.register(frame);
    assert!(!is_set_text_noop(&state, id, &Some("Hello".to_string())));
}

#[test]
fn set_text_noop_rejects_stale_child_text() {
    let mut state = SimState::default();
    let (id, child_id) = make_button_with_text_child(&mut state, "Leave Party");
    // Corrupt the child so it no longer matches self — slow path must run.
    if let Some(child) = state.widgets.get_mut_visual(child_id) {
        child.text = Some("stale".to_string());
    }
    assert!(!is_set_text_noop(
        &state,
        id,
        &Some("Leave Party".to_string())
    ));
}

#[test]
fn set_text_noop_rejects_tooltip_frames() {
    let mut state = SimState::default();
    let mut frame = Frame::new(WidgetType::FontString, Some("Tip".to_string()), None);
    frame.text = Some("Hello".to_string());
    let id = frame.id;
    state.widgets.register(frame);
    state
        .tooltips
        .insert(id, crate::lua_api::tooltip::TooltipData::default());
    // Tooltip frames must always re-run to pick up color/wrap args.
    assert!(!is_set_text_noop(&state, id, &Some("Hello".to_string())));
}

#[test]
fn set_text_noop_rejects_simple_html_frames() {
    let mut state = SimState::default();
    let mut frame = Frame::new(WidgetType::SimpleHTML, Some("Html".to_string()), None);
    frame.text = Some("Hello".to_string());
    let id = frame.id;
    state.widgets.register(frame);
    state
        .simple_htmls
        .insert(id, crate::lua_api::simple_html::SimpleHtmlData::default());
    assert!(!is_set_text_noop(&state, id, &Some("Hello".to_string())));
}

#[test]
fn set_text_noop_detects_both_nil() {
    let mut state = SimState::default();
    let frame = Frame::new(WidgetType::FontString, Some("Label".to_string()), None);
    let id = frame.id;
    state.widgets.register(frame);
    assert!(is_set_text_noop(&state, id, &None));
}
