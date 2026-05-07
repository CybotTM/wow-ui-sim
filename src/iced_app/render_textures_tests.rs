use super::{
    TextureLoadBatchTelemetry, process_budgeted_texture_request,
    should_pause_texture_loading_state, texture_request_base_path,
    unresolved_texture_request_paths,
};
use crate::iced_app::App;
use crate::iced_app::app::AppInit;
use crate::render::shader::primitive::TextureRequestTracker;
use crate::render::{GlyphAtlas, GpuTextureData, QuadBatch, TextureRequest, WowFontSystem};
use crate::screen::ScreenKind;
use crate::texture::TextureManager;
use crate::widget::{AnchorPoint, Frame, WidgetType};
use crate::{LayoutRect, lua_api::WowLuaEnv};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use tokio::sync::mpsc;

fn request(path: &str) -> TextureRequest {
    TextureRequest::new(path, 0, 4)
}

#[test]
fn unresolved_texture_request_paths_filters_non_loadable_requests() {
    let mut batch = QuadBatch::new();
    batch
        .texture_requests
        .push(request(r"Interface\WorldMap\IsleofDorn\IsleOfDorn1"));
    batch
        .texture_requests
        .push(request(r"Interface\Minimap\UI-Minimap-Background"));
    batch.mask_texture_requests.push(request("uploaded-mask"));
    batch.texture_requests.push(request("failed-path"));

    batch.mask_texture_requests[0].handle.mark_staged();
    batch.texture_requests[2].handle.mark_failed();

    let paths = unresolved_texture_request_paths(&batch);
    assert_eq!(
        paths,
        vec![
            r"Interface\WorldMap\IsleofDorn\IsleOfDorn1",
            r"Interface\Minimap\UI-Minimap-Background",
        ]
    );
}

#[test]
fn unresolved_texture_request_paths_deduplicates_before_sorting() {
    let mut batch = QuadBatch::new();
    batch
        .texture_requests
        .push(request(r"Interface\WorldMap\IsleofDorn\IsleOfDorn1"));
    batch
        .texture_requests
        .push(request(r"Interface\WorldMap\IsleofDorn\IsleOfDorn1"));
    batch.texture_requests.push(request(
        r"Interface\questframe\questmaplogatlas@crop:0.1,0.2,0.3,0.4",
    ));

    let paths = unresolved_texture_request_paths(&batch);
    assert_eq!(
        paths,
        vec![
            r"Interface\WorldMap\IsleofDorn\IsleOfDorn1",
            r"Interface\questframe\questmaplogatlas@crop:0.1,0.2,0.3,0.4",
        ]
    );
}

#[test]
fn texture_request_base_path_strips_crop_suffix() {
    assert_eq!(
        texture_request_base_path(r"Interface\Foo\Bar@crop:0.1,0.2,0.3,0.4"),
        r"Interface\Foo\Bar"
    );
    assert_eq!(
        texture_request_base_path(r"Interface\Foo\Bar"),
        r"Interface\Foo\Bar"
    );
}

#[test]
fn texture_loading_only_pauses_after_budget_hit_for_uncached_base_path() {
    assert!(!should_pause_texture_loading_state(false, true, false));
    assert!(!should_pause_texture_loading_state(true, false, false));
    assert!(!should_pause_texture_loading_state(true, true, true));
    assert!(should_pause_texture_loading_state(true, true, false));
}

#[test]
fn process_budgeted_texture_request_returns_true_before_loading_uncached_work() {
    let mut tex_mgr = TextureManager::new();
    let mut textures = vec![GpuTextureData {
        path: "already-loaded".to_string(),
        width: 1,
        height: 1,
        rgba: Arc::<[u8]>::from(vec![0xff; 4]),
    }];
    let mut bc_textures = Vec::new();
    let mut telemetry = TextureLoadBatchTelemetry::default();
    let mut texture_requests = TextureRequestTracker::default();

    let paused = process_budgeted_texture_request(
        std::time::Instant::now(),
        r"Interface\Foo\Bar",
        &mut tex_mgr,
        &mut textures,
        &mut bc_textures,
        &mut telemetry,
        &mut texture_requests,
    );

    assert!(paused);
    assert_eq!(texture_requests.ready_count(), 0);
    assert_eq!(texture_requests.staged_count(), 0);
    assert_eq!(textures.len(), 1);
    assert!(bc_textures.is_empty());
}

fn build_test_app(debug_borders: bool, debug_anchors: bool) -> App {
    let env = Rc::new(RefCell::new(
        WowLuaEnv::new().expect("Failed to create Lua environment"),
    ));
    env.borrow().set_screen_mode(ScreenKind::Game);

    let texture_manager = Rc::new(RefCell::new(TextureManager::new()));
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
        debug_borders,
        debug_anchors,
        saved_vars: None,
        config: crate::config::SimConfig::default(),
    })
}

fn build_texture_load_test_app() -> App {
    let env = Rc::new(RefCell::new(
        WowLuaEnv::new().expect("Failed to create Lua environment"),
    ));
    env.borrow().set_screen_mode(ScreenKind::Game);

    let texture_manager = Rc::new(RefCell::new(TextureManager::new()));
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

fn register_debug_frame(app: &App) {
    let env = app.env.borrow();
    let mut state = env.state().borrow_mut();
    let widgets = &mut state.widgets;
    let mut frame = Frame::new(WidgetType::Frame, Some("DebugFrame".to_string()), None);
    frame.layout_rect = Some(LayoutRect {
        x: 10.0,
        y: 20.0,
        width: 40.0,
        height: 30.0,
    });
    frame.set_point(AnchorPoint::TopLeft, None, AnchorPoint::TopLeft, 0.0, 0.0);
    widgets.register(frame);
}

#[test]
fn build_overlay_emits_debug_quads_from_startup_flags() {
    let app = build_test_app(true, true);
    register_debug_frame(&app);

    let overlay = app.build_overlay();
    assert_eq!(overlay.quad_count(), 5);
    assert!(overlay.texture_requests.is_empty());
    assert!(overlay.mask_texture_requests.is_empty());
}

#[test]
fn build_overlay_emits_debug_quads_from_runtime_toggles() {
    let app = build_test_app(false, false);
    register_debug_frame(&app);
    {
        let env = app.env.borrow();
        let mut state = env.state().borrow_mut();
        state.debug_borders = true;
        state.debug_anchors = true;
    }

    let overlay = app.build_overlay();
    assert_eq!(overlay.quad_count(), 5);
    assert!(overlay.texture_requests.is_empty());
    assert!(overlay.mask_texture_requests.is_empty());
}

#[test]
fn load_new_textures_budgeted_loads_spellbook_mask_via_bc_path() {
    let app = build_texture_load_test_app();
    let mut batch = QuadBatch::new();
    batch
        .mask_texture_requests
        .push(request(r"Interface\spellbook\spellbookelementsiconmask"));

    let prev_bc_supported = crate::render::shader::atlas::set_bc_supported_for_tests(true);
    let mut texture_requests = TextureRequestTracker::default();
    let (rgba, bc, _scan_elapsed, _load_elapsed, _telemetry, hit_deadline) = app
        .load_new_textures_budgeted(
            &batch,
            std::time::Instant::now() + std::time::Duration::from_secs(1),
            &mut texture_requests,
        );
    crate::render::shader::atlas::set_bc_supported_for_tests(prev_bc_supported);

    assert!(!hit_deadline, "single mask request should not hit deadline");
    assert!(rgba.is_empty(), "mask should not fall back to RGBA path");
    assert_eq!(bc.len(), 1, "expected one BC texture upload");
    assert_eq!(bc[0].path, r"Interface\spellbook\spellbookelementsiconmask");
}
