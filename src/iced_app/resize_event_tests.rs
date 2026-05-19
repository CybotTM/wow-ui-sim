use super::*;
use crate::iced_app::app::AppInit;
use crate::screen::ScreenKind;
use crate::texture::TextureManager;
use iced::Size;
use std::cell::RefCell;
use std::rc::Rc;
use tokio::sync::mpsc;

fn build_test_app() -> App {
    let env = Rc::new(RefCell::new(
        WowLuaEnv::new().expect("Failed to create Lua environment"),
    ));
    env.borrow().set_screen_mode(ScreenKind::Game);

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
fn resizing_window_fires_display_size_changed_without_ui_scale_changed() {
    let app = build_test_app();
    app.env.borrow().set_screen_size(800.0, 600.0);
    app.env
        .borrow()
        .exec(
            r#"
            __display_size_changed = 0
            __ui_scale_changed = 0
            local frame = CreateFrame("Frame")
            frame:RegisterEvent("DISPLAY_SIZE_CHANGED")
            frame:RegisterEvent("UI_SCALE_CHANGED")
            frame:SetScript("OnEvent", function(_, event)
                if event == "DISPLAY_SIZE_CHANGED" then
                    __display_size_changed = __display_size_changed + 1
                elseif event == "UI_SCALE_CHANGED" then
                    __ui_scale_changed = __ui_scale_changed + 1
                end
            end)
            "#,
        )
        .expect("event counter setup should succeed");

    app.sync_screen_size_to_state(Size::new(1024.0, 768.0));

    let (display_count, scale_count): (f64, f64) = app
        .env
        .borrow()
        .eval("return __display_size_changed, __ui_scale_changed")
        .expect("event counters should be readable");
    assert_eq!(
        display_count, 1.0,
        "window resize should fire display event"
    );
    assert_eq!(
        scale_count, 0.0,
        "window resize should not imply scale event"
    );
}
