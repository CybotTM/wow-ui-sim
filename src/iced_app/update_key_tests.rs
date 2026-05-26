use super::*;
use crate::iced_app::app::AppInit;
use crate::screen::ScreenKind;
use crate::texture::TextureManager;
use iced::Size;
use iced_runtime::Action;
use iced_runtime::futures::futures::StreamExt;
use iced_runtime::window::Action as WindowAction;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;
use tokio::sync::mpsc;

fn build_test_app(screen_kind: ScreenKind) -> App {
    let env = Rc::new(RefCell::new(
        WowLuaEnv::new().expect("Failed to create Lua environment"),
    ));
    env.borrow().set_screen_mode(screen_kind);

    let texture_manager = Rc::new(RefCell::new(TextureManager::new()));
    let font_system = Rc::new(RefCell::new(crate::render::WowFontSystem::new()));
    let glyph_atlas = Rc::new(RefCell::new(crate::render::GlyphAtlas::new()));
    let (_cmd_tx, cmd_rx) = mpsc::channel(1);
    let (_lua_tx, lua_rx) = std::sync::mpsc::channel();

    App::build_app(AppInit {
        env,
        log_messages: Vec::new(),
        texture_manager,
        font_system,
        glyph_atlas,
        cmd_rx,
        lua_rx,
        debug_borders: false,
        debug_anchors: false,
        saved_vars: None,
        config: crate::config::SimConfig::default(),
    })
}

#[test]
fn key_press_requests_redraw_after_lua_visual_mutation() {
    let mut app = build_test_app(ScreenKind::Game);
    app.screen_size.set(Size::new(1024.0, 768.0));
    app.strata_dirty.set(0);
    app.textures_pending.set(false);

    {
        let env = app.env.borrow();
        env.exec(
            r#"
            local editBox = CreateFrame("EditBox", "KeyPressRedrawEditBox", UIParent)
            editBox:SetSize(120, 20)
            editBox:SetPoint("CENTER")
            editBox:Show()
            editBox:SetFocus()
        "#,
        )
        .expect("create focused editbox");
        let _ = env.state().borrow().widgets.take_render_dirty_with_ids();
    }

    let task = app.update(Message::KeyPress(
        "A".to_string(),
        Some("a".to_string()),
        Instant::now(),
    ));
    let action = pollster::block_on(async {
        iced_runtime::task::into_stream(task)
            .expect("key visual mutation should request redraw")
            .next()
            .await
            .expect("task should emit a redraw action")
    });

    let text: String = app
        .env
        .borrow()
        .eval("return KeyPressRedrawEditBox:GetText()")
        .expect("read editbox text");
    assert_eq!(text, "a", "keypress should still reach Lua editbox input");
    assert!(
        matches!(action, Action::Window(WindowAction::RedrawAll)),
        "keypress visual mutation should request an immediate redraw"
    );
}

#[test]
fn ctrl_q_key_press_requests_window_close() {
    let mut app = build_test_app(ScreenKind::Game);

    let task = app.update(Message::KeyPress(
        "CTRL-Q".to_string(),
        None,
        Instant::now(),
    ));
    let close_action = pollster::block_on(async {
        iced_runtime::task::into_stream(task)
            .expect("ctrl-q should create task actions")
            .next()
            .await
            .expect("ctrl-q should emit a runtime-exit action")
    });

    assert!(
        !app.env.borrow().state().borrow().simulator_exit_requested,
        "update should consume the simulator exit request after creating the close task"
    );
    assert!(
        matches!(close_action, Action::Exit),
        "CTRL-Q should request runtime exit; got {close_action:?}"
    );

    app.env
        .borrow()
        .state()
        .borrow_mut()
        .simulator_exit_requested = true;
    let close_task = app.take_simulator_exit_task();
    let action = pollster::block_on(async {
        iced_runtime::task::into_stream(close_task)
            .expect("exit request should create a runtime operation")
            .next()
            .await
            .expect("close task should emit runtime exit")
    });

    assert!(
        matches!(action, Action::Exit),
        "exit request should emit runtime exit"
    );
}
