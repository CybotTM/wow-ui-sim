use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use tokio::sync::mpsc;

use crate::iced_app::app::{App, AppInit};
use crate::lua_api::WowLuaEnv;
use crate::render::{GlyphAtlas, WowFontSystem};
use crate::screen::ScreenKind;
use crate::texture::TextureManager;

pub(super) fn build_test_app() -> App {
    build_test_app_with_addons(None)
}

pub(super) fn build_test_app_with_addons(addons_path: Option<&Path>) -> App {
    let env = Rc::new(RefCell::new(
        WowLuaEnv::new().expect("Failed to create Lua environment"),
    ));
    env.borrow().set_screen_mode(ScreenKind::Game);

    let mut texture_manager = TextureManager::new();
    if let Some(path) = addons_path {
        texture_manager = texture_manager.with_addons_path(path);
    }

    let texture_manager = Rc::new(RefCell::new(texture_manager));
    let font_system = Rc::new(RefCell::new(WowFontSystem::new()));
    let glyph_atlas = Rc::new(RefCell::new(GlyphAtlas::new()));
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
