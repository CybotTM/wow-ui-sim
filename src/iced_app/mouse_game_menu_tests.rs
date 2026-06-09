use super::test_support::*;
use super::*;
use crate::iced_app::Message;
use crate::screen::ScreenKind;
use iced_runtime::Action;
use iced_runtime::futures::futures::StreamExt;

#[cfg(feature = "client-mists")]
#[test]
fn mists_game_menu_exit_button_click_requests_runtime_exit() {
    let mut app = build_test_app(ScreenKind::Game);
    load_blizzard_game_ui_for_mouse_test(&app);
    app.env
        .borrow()
        .exec("ToggleGameMenu()")
        .expect("GameMenuFrame should open");
    app.env
        .borrow()
        .exec(
            r#"
            for button in GameMenuFrame.buttonPool:EnumerateActive() do
                if button:GetText() == EXIT_GAME then
                    button:Click()
                    return
                end
            end
            error("Exit Game button missing")
            "#,
        )
        .expect("Exit Game button click should run");

    let exit_task = app.update(Message::FpsTick);
    assert_task_exits(exit_task, "Exit Game click should request runtime exit");
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
fn assert_task_exits(task: iced::Task<Message>, context: &str) {
    let mut stream =
        iced_runtime::task::into_stream(task).expect("task should create task actions");
    let exits = pollster::block_on(async {
        while let Some(action) = stream.next().await {
            if matches!(action, Action::Exit) {
                return true;
            }
        }
        false
    });
    assert!(exits, "{context}; task did not emit Action::Exit");
}
