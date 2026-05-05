use super::CommandDispatch;
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use wow_ui_sim::font::WowFontSystem;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::startup::{apply_delay, run_extra_update_ticks, settle_headless_startup};

pub(super) fn run_gui(dispatch: CommandDispatch) -> Result<(), Box<dyn std::error::Error>> {
    let debug_options = dispatch.debug_options();
    wow_ui_sim::run_iced_ui(
        dispatch.env,
        debug_options,
        dispatch.saved_vars,
        dispatch.exec_lua,
        dispatch.exec_lua_secure,
    )
}

pub(super) struct ScreenshotCommand<'a> {
    pub(super) output: PathBuf,
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) filter: Option<String>,
    pub(super) crop: Option<String>,
    pub(super) delay: Option<u64>,
    pub(super) exec_lua: Option<&'a str>,
    pub(super) exec_lua_secure: bool,
    pub(super) dump_tree: Option<Option<String>>,
}

pub(super) fn run_screenshot(
    env: &WowLuaEnv,
    font_system: &Rc<RefCell<WowFontSystem>>,
    command: ScreenshotCommand<'_>,
) {
    use wow_ui_sim::render::headless::render_to_image;

    settle_headless_startup(env);
    env.set_screen_size(command.width as f32, command.height as f32);
    wow_ui_sim::debug_helpers::debug_show_game_menu(env);
    if let Some(code) = command.exec_lua
        && let Err(e) = env.exec_maybe_secure(code, command.exec_lua_secure)
    {
        eprintln!("[exec-lua] error: {e}");
    }
    run_extra_update_ticks(env, 3);
    apply_delay(command.delay);
    let (batch, glyph_atlas) = build_screenshot_batch(
        env,
        font_system,
        command.width,
        command.height,
        command.filter.as_deref(),
    );
    if let Some(dump_filter) = &command.dump_tree {
        dump_screenshot_tree(env, dump_filter.as_deref(), command.width, command.height);
    }
    eprintln!(
        "QuadBatch: {} quads, {} texture requests",
        batch.quad_count(),
        batch.texture_requests.len()
    );

    let mut tex_mgr = create_texture_manager();
    let glyph_data = glyph_atlas.is_dirty().then(|| {
        let (data, size, _) = glyph_atlas.texture_data();
        (data, size)
    });
    let img = render_to_image(
        &batch,
        &mut tex_mgr,
        command.width,
        command.height,
        glyph_data,
    );
    let img = match command.crop.as_deref() {
        Some(crop_str) => apply_crop(img, crop_str),
        None => img,
    };
    let output = command.output.with_extension("webp");
    save_screenshot(&img, &output);
    eprintln!(
        "Saved {}x{} screenshot to {}",
        img.width(),
        img.height(),
        output.display()
    );
}

pub(super) fn run_dump_texture(
    env: &WowLuaEnv,
    font_system: &Rc<RefCell<WowFontSystem>>,
    output: PathBuf,
    filter: Option<String>,
    frame_filter: Option<String>,
) {
    env.set_screen_size(1600.0, 1200.0);
    settle_headless_startup(env);
    let (batch, _) = build_screenshot_batch(env, font_system, 1600, 1200, frame_filter.as_deref());
    eprintln!(
        "QuadBatch: {} quads, {} tex requests",
        batch.quad_count(),
        batch.texture_requests.len()
    );
    let mut tex_mgr = create_texture_manager();
    wow_ui_sim::dump_texture::dump_batch_textures(&batch, &mut tex_mgr, &output, filter.as_deref());
}

fn build_screenshot_batch(
    env: &WowLuaEnv,
    font_system: &Rc<RefCell<WowFontSystem>>,
    width: u32,
    height: u32,
    filter: Option<&str>,
) -> (
    wow_ui_sim::render::QuadBatch,
    wow_ui_sim::render::GlyphAtlas,
) {
    use wow_ui_sim::iced_app::{
        RegistryQuadBatchParams, build_quad_batch_for_registry_with_quest_blobs,
    };
    use wow_ui_sim::render::GlyphAtlas;

    let mut glyph_atlas = GlyphAtlas::new();
    let batch = {
        let mut fs = font_system.borrow_mut();
        let buckets = {
            let mut state = env.state().borrow_mut();
            wow_ui_sim::iced_app::tooltip::update_tooltip_sizes(&mut state, &mut fs);
            state.ensure_layout_rects();
            let _ = state.get_strata_buckets();
            state.strata_buckets.as_ref().unwrap().clone()
        };
        let state = env.state().borrow();
        let tooltip_data = wow_ui_sim::iced_app::tooltip::collect_tooltip_data(&state);
        build_quad_batch_for_registry_with_quest_blobs(
            RegistryQuadBatchParams::new(&state.widgets, (width as f32, height as f32), &buckets)
                .root_name(filter)
                .text_ctx(Some((&mut fs, &mut glyph_atlas)))
                .message_frames(Some(&state.message_frames))
                .tooltip_data(Some(&tooltip_data))
                .quest_blobs(Some(&state.quest_blobs)),
        )
    };
    (batch, glyph_atlas)
}

fn dump_screenshot_tree(env: &WowLuaEnv, filter_key: Option<&str>, width: u32, height: u32) {
    let state = env.state().borrow();
    let addon_names: Vec<String> = state.addons.iter().map(|a| a.folder_name.clone()).collect();
    wow_ui_sim::dump::print_frame_tree(
        &state.widgets,
        &addon_names,
        None,
        filter_key,
        false,
        false,
        width as f32,
        height as f32,
    );
}

fn parse_crop(spec: &str) -> Option<(u32, u32, u32, u32)> {
    let (dims, rest) = spec.split_once('+')?;
    let (x_str, y_str) = rest.split_once('+')?;
    let (w_str, h_str) = dims.split_once('x')?;
    Some((
        w_str.parse().ok()?,
        h_str.parse().ok()?,
        x_str.parse().ok()?,
        y_str.parse().ok()?,
    ))
}

fn apply_crop(img: image::RgbaImage, crop_str: &str) -> image::RgbaImage {
    use image::GenericImageView;

    let (crop_width, crop_height, crop_x, crop_y) = parse_crop(crop_str).unwrap_or_else(|| {
        eprintln!("Invalid crop format '{}', expected WxH+X+Y", crop_str);
        std::process::exit(1);
    });
    if crop_x + crop_width > img.width() || crop_y + crop_height > img.height() {
        eprintln!("Crop region exceeds image bounds");
        std::process::exit(1);
    }
    img.view(crop_x, crop_y, crop_width, crop_height).to_image()
}

fn save_screenshot(img: &image::RgbaImage, output: &Path) {
    let output = output.with_extension("webp");
    let encoder = webp::Encoder::from_rgba(img.as_raw(), img.width(), img.height());
    let mem = encoder.encode(15.0);
    if let Err(e) = std::fs::write(&output, &*mem) {
        eprintln!("Failed to save WebP: {}", e);
        std::process::exit(1);
    }
}

fn create_texture_manager() -> wow_ui_sim::texture::TextureManager {
    use wow_ui_sim::texture::TextureManager;

    let config = wow_ui_sim::config::SimConfig::load();
    let mut mgr = TextureManager::new().with_addons_path(wow_ui_sim::paths::default_addons_path());
    mgr.preload_talent_textures(790);
    mgr.preload_talent_panel_textures(&config.player_class);
    mgr
}
