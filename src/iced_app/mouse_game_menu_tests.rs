use super::test_support::*;
use super::*;
use crate::iced_app::{CanvasMessage, Message};
use crate::screen::ScreenKind;
use iced_runtime::Action;
use iced_runtime::futures::futures::StreamExt;

#[cfg(feature = "client-mists")]
#[test]
fn mists_game_menu_exit_button_requests_runtime_exit() {
    let mut app = build_test_app(ScreenKind::Game);
    load_blizzard_game_ui_for_mouse_test(&app);
    click_frame_by_name(&mut app, "MainMenuMicroButton");
    rebuild_hittable_cache(&app);

    let click_pos = button_center_by_text(&app, "Exit Game");
    let mouse_down_task = app.update(Message::CanvasEvent(CanvasMessage::MouseDown(click_pos)));
    let mouse_up_task = app.update(Message::CanvasEvent(CanvasMessage::MouseUp(click_pos)));
    assert_task_has_no_exit(mouse_down_task, "mouse down should not quit");
    assert_task_exits(
        mouse_up_task,
        "Exit Game mouse up should request runtime exit",
    );
}

#[cfg(feature = "client-mists")]
fn load_blizzard_game_ui_for_mouse_test(app: &App) {
    use crate::loader::{discover_blizzard_addons_for_screen, load_addon};

    let env = app.env.borrow();
    env.set_screen_mode(ScreenKind::Game);

    if let Some(framexml_toc) = crate::client_profile::blizzard_ui_framexml_toc() {
        load_addon(&env.loader_env(), &framexml_toc).expect("FrameXML should load");
    }

    let addons_dir = crate::client_profile::blizzard_ui_addons_dir();
    let addons = discover_blizzard_addons_for_screen(&addons_dir, ScreenKind::Game);
    for (name, toc_path) in addons {
        load_addon(&env.loader_env(), &toc_path)
            .unwrap_or_else(|err| panic!("{name} should load: {err}"));
        if name == "Blizzard_EnvironmentCleanup" {
            env.restore_post_cleanup_globals();
        }
    }

    env.apply_post_load_workarounds();
    crate::startup::settle_headless_startup(&env);
}

#[cfg(feature = "client-mists")]
fn click_frame_by_name(app: &mut App, name: &str) {
    rebuild_hittable_cache(app);
    let click_pos = {
        let env = app.env.borrow();
        let mut state = env.state().borrow_mut();
        state.ensure_layout_rects();
        frame_center(
            state
                .widgets
                .get_by_name(name)
                .unwrap_or_else(|| panic!("{name} should exist")),
        )
    };
    app.handle_mouse_down(click_pos);
    app.handle_mouse_up(click_pos);
}

#[cfg(feature = "client-mists")]
fn button_center_by_text(app: &App, text: &str) -> Point {
    let env = app.env.borrow();
    let mut state = env.state().borrow_mut();
    state.ensure_layout_rects();
    let menu_id = state
        .widgets
        .get_id_by_name("GameMenuFrame")
        .expect("GameMenuFrame should exist");

    state
        .widgets
        .iter_ids()
        .filter(|&id| frame_has_ancestor(&state, id, menu_id))
        .filter_map(|id| state.widgets.get(id))
        .find(|frame| {
            frame.widget_type == crate::widget::WidgetType::Button
                && frame.layout_rect.is_some()
                && frame.children.iter().any(|&child_id| {
                    state
                        .widgets
                        .get(child_id)
                        .is_some_and(|child| child.text.as_deref() == Some(text))
                })
        })
        .map(frame_center)
        .unwrap_or_else(|| panic!("{text:?} button should have a layout rect"))
}

#[cfg(feature = "client-mists")]
fn frame_has_ancestor(
    state: &crate::lua_api::state::SimState,
    mut frame_id: u64,
    ancestor_id: u64,
) -> bool {
    while let Some(frame) = state.widgets.get(frame_id) {
        let Some(parent_id) = frame.parent_id else {
            return false;
        };
        if parent_id == ancestor_id {
            return true;
        }
        frame_id = parent_id;
    }
    false
}

#[cfg(feature = "client-mists")]
fn frame_center(frame: &crate::widget::Frame) -> Point {
    let rect = frame.layout_rect.expect("frame should have a layout rect");
    Point::new(rect.x + rect.width / 2.0, rect.y + rect.height / 2.0)
}

#[cfg(feature = "client-mists")]
fn assert_task_exits(task: iced::Task<Message>, context: &str) {
    let action = pollster::block_on(async {
        iced_runtime::task::into_stream(task)
            .expect("task should create task actions")
            .next()
            .await
            .expect("task should emit a runtime action")
    });
    assert!(matches!(action, Action::Exit), "{context}; got {action:?}");
}

#[cfg(feature = "client-mists")]
fn assert_task_has_no_exit(task: iced::Task<Message>, context: &str) {
    let Some(mut stream) = iced_runtime::task::into_stream(task) else {
        return;
    };
    let action = pollster::block_on(async { stream.next().await });
    assert!(
        !matches!(action, Some(Action::Exit)),
        "{context}; got {action:?}"
    );
}
