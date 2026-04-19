use super::*;
use crate::lua_api::WowLuaEnv;
use crate::render::{GlyphAtlas, WowFontSystem};
use crate::screen::ScreenKind;
use crate::texture::TextureManager;
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use tempfile::tempdir;
use tokio::sync::mpsc;

fn build_test_app_with_textures(textures_path: &Path) -> App {
    let env = Rc::new(RefCell::new(
        WowLuaEnv::new().expect("Failed to create Lua environment"),
    ));
    env.borrow().set_screen_mode(ScreenKind::Game);

    let texture_manager = Rc::new(RefCell::new(TextureManager::new(textures_path)));
    let font_system = Rc::new(RefCell::new(WowFontSystem::new(&std::path::PathBuf::from(
        crate::iced_app::app::DEFAULT_FONTS_PATH,
    ))));
    let glyph_atlas = Rc::new(RefCell::new(GlyphAtlas::new()));
    let (_cmd_tx, cmd_rx) = mpsc::channel(1);
    let (_lua_tx, lua_rx) = std::sync::mpsc::channel();

    App::build_app(
        env,
        Vec::new(),
        texture_manager,
        font_system,
        glyph_atlas,
        cmd_rx,
        lua_rx,
        false,
        false,
        None,
        crate::config::SimConfig::default(),
    )
}

#[test]
fn budgeted_preload_loads_explicitly_queued_texture_requests() {
    let temp_dir = tempdir().unwrap();
    let texture_path = temp_dir.path().join("world-map-tile.png");
    let image = image::RgbaImage::from_pixel(4, 4, image::Rgba([0x44, 0x88, 0xcc, 0xff]));
    image.save(&texture_path).unwrap();

    let app = build_test_app_with_textures(temp_dir.path());
    app.env
        .borrow()
        .state()
        .borrow_mut()
        .enqueue_texture_preloads(["world-map-tile".to_string()]);

    app.preload_current_render_requests(Some(std::time::Duration::from_millis(50)));

    assert!(
        app.texture_manager.borrow().get("world-map-tile").is_some(),
        "queued preload should decode the requested texture source"
    );
    assert!(
        !app.textures_pending.get(),
        "queue-driven preload should clear pending state once the queue drains"
    );
}

#[test]
fn budgeted_preload_requeues_tail_when_budget_hits() {
    let temp_dir = tempdir().unwrap();
    for name in ["alpha", "beta"] {
        let texture_path = temp_dir.path().join(format!("{name}.png"));
        let image = image::RgbaImage::from_pixel(4, 4, image::Rgba([0x44, 0x88, 0xcc, 0xff]));
        image.save(&texture_path).unwrap();
    }

    let app = build_test_app_with_textures(temp_dir.path());
    app.env
        .borrow()
        .state()
        .borrow_mut()
        .enqueue_texture_preloads(["alpha".to_string(), "beta".to_string()]);

    app.preload_current_render_requests(Some(std::time::Duration::ZERO));

    let queued_after = app
        .env
        .borrow()
        .state()
        .borrow()
        .pending_texture_preloads
        .len();

    assert!(
        app.textures_pending.get(),
        "budget hit should keep queue-driven preload pending"
    );
    assert_ne!(queued_after, 0, "unprocessed paths should be requeued");
}

#[test]
fn empty_queue_preload_preserves_existing_pending_state() {
    let temp_dir = tempdir().unwrap();
    let app = build_test_app_with_textures(temp_dir.path());
    app.textures_pending.set(true);

    app.preload_current_render_requests(Some(std::time::Duration::from_millis(50)));

    assert!(
        app.textures_pending.get(),
        "an empty preload queue should not clear draw-owned pending state"
    );
}
