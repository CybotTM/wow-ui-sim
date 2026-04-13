use super::*;
use crate::lua_api::WowLuaEnv;
use crate::render::{GlyphAtlas, TextureRequest, WowFontSystem};
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
fn budgeted_preload_keeps_textures_pending_until_gpu_uploads_cached_batch_requests() {
    let temp_dir = tempdir().unwrap();
    let texture_path = temp_dir.path().join("world-map-tile.png");
    let image = image::RgbaImage::from_pixel(4, 4, image::Rgba([0x44, 0x88, 0xcc, 0xff]));
    image.save(&texture_path).unwrap();

    let app = build_test_app_with_textures(temp_dir.path());
    let mut batch = QuadBatch::new();
    batch.texture_requests.push(TextureRequest {
        path: "world-map-tile".to_string(),
        vertex_start: 0,
        vertex_count: 4,
    });
    app.cached_strata_quads.borrow_mut()[0] = Some(Arc::new(batch));
    app.strata_dirty.set(0);

    app.preload_current_render_requests(Some(std::time::Duration::from_millis(50)));

    assert!(
        app.texture_manager.borrow().get("world-map-tile").is_some(),
        "budgeted preload should decode uncached sources instead of deferring them to draw"
    );
    assert!(
        app.gpu_uploaded_textures.borrow().is_empty(),
        "preload should not mark textures as GPU-uploaded before draw runs"
    );
    assert!(
        app.textures_pending.get(),
        "preload should keep the fast tick alive until draw uploads cached-batch textures"
    );
}

#[test]
fn budgeted_preload_clears_pending_after_cached_batch_texture_reaches_gpu() {
    let temp_dir = tempdir().unwrap();
    let texture_path = temp_dir.path().join("world-map-tile.png");
    let image = image::RgbaImage::from_pixel(4, 4, image::Rgba([0x44, 0x88, 0xcc, 0xff]));
    image.save(&texture_path).unwrap();

    let app = build_test_app_with_textures(temp_dir.path());
    let mut batch = QuadBatch::new();
    batch.texture_requests.push(TextureRequest {
        path: "world-map-tile".to_string(),
        vertex_start: 0,
        vertex_count: 4,
    });
    app.cached_strata_quads.borrow_mut()[0] = Some(Arc::new(batch));
    app.strata_dirty.set(0);

    app.preload_current_render_requests(Some(std::time::Duration::from_millis(50)));
    app.gpu_uploaded_textures
        .borrow_mut()
        .insert("world-map-tile".to_string());

    app.preload_current_render_requests(Some(std::time::Duration::from_millis(50)));

    assert!(
        !app.textures_pending.get(),
        "pending should clear once the requested texture is already in the GPU atlas"
    );
}
