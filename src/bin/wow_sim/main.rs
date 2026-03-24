mod addon_loading;

use clap::{Parser, Subcommand};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use tracing_subscriber::EnvFilter;
use wow_ui_sim::logging;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::render::WowFontSystem;
use wow_ui_sim::saved_variables::SavedVariablesManager;
use wow_ui_sim::screen::ScreenKind;

#[derive(Parser)]
#[command(name = "wow-sim", about = "WoW UI Simulator")]
struct Args {
    /// Skip loading WTF SavedVariables (faster startup)
    #[arg(long)]
    no_saved_vars: bool,

    /// Skip loading third-party addons
    #[arg(long)]
    no_addons: bool,

    /// Show debug borders and anchor points on all elements
    #[arg(long)]
    debug_elements: bool,

    /// Show red debug borders around all elements
    #[arg(long)]
    debug_borders: bool,

    /// Show green anchor points on all elements
    #[arg(long)]
    debug_anchors: bool,

    /// Delay in milliseconds after firing startup events (for dump-tree/screenshot)
    #[arg(long, value_name = "MS")]
    delay: Option<u64>,

    /// Execute Lua code after startup (runs after first frame in GUI, after events in screenshot/dump-tree).
    /// Prefix with @ to load from file (e.g., --exec-lua @/tmp/debug.lua).
    #[arg(long, value_name = "CODE")]
    exec_lua: Option<String>,

    /// Which top-level WoW screen to load.
    #[arg(long, value_enum, default_value_t = ScreenKind::Game, value_name = "SCREEN")]
    screen: ScreenKind,

    /// Compatibility alias for `--screen character-select`.
    #[arg(long, hide = true)]
    character_select: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Load UI and dump frame tree (no GUI needed)
    DumpTree {
        #[arg(short, long)]
        filter: Option<String>,
        #[arg(long)]
        filter_key: Option<String>,
        #[arg(long)]
        visible_only: bool,
        #[arg(long, default_value_t = 1600)]
        width: u32,
        #[arg(long, default_value_t = 1200)]
        height: u32,
    },

    /// Render UI to an image file (no GUI needed)
    #[cfg(feature = "gui")]
    Screenshot {
        #[arg(short, long, default_value = "screenshot.webp")]
        output: PathBuf,
        #[arg(long, default_value_t = 1600)]
        width: u32,
        #[arg(long, default_value_t = 1200)]
        height: u32,
        #[arg(short, long)]
        filter: Option<String>,
        #[arg(long, value_name = "WxH+X+Y")]
        crop: Option<String>,
        #[arg(long, value_name = "FILTER")]
        dump_tree: Option<Option<String>>,
    },

    /// Show unique Lua errors as JSON (suppresses other output)
    LuaErrors,

    /// Run simulator self-tests (Wowless test suite)
    SelfTest {
        #[arg(long, default_value_t = 10000)]
        max_ticks: u32,
        #[arg(long)]
        categories: Option<String>,
    },

    /// Run test Lua files from Interface/AddOns/<name>/tests/
    RunTests { addon_name: String },

    /// Dump textures used by frames to disk (for debugging atlas crops)
    #[cfg(feature = "gui")]
    DumpTexture {
        #[arg(short, long, default_value = "/tmp/claude/textures")]
        output: PathBuf,
        #[arg(short, long)]
        filter: Option<String>,
        #[arg(long)]
        frame_filter: Option<String>,
    },
}

impl Args {
    fn effective_screen(&self) -> ScreenKind {
        if self.character_select {
            ScreenKind::CharacterSelect
        } else {
            self.screen
        }
    }

    fn is_test_command(&self) -> bool {
        matches!(
            self.command,
            Some(Commands::SelfTest { .. }) | Some(Commands::RunTests { .. })
        )
    }

    fn skip_addons(&self) -> bool {
        self.no_addons
            || std::env::var("WOW_SIM_NO_ADDONS")
                .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
                .unwrap_or(false)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    wow_ui_sim::stack::ensure_large_stack();
    run_main()
}

fn run_main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let screen = args.effective_screen();
    let saved_stdout = redirect_if_quiet(&args);
    let (env, font_system, saved_vars) = init_and_load(&args, screen);

    dispatch_command(
        args.command,
        env,
        font_system,
        args.delay,
        resolve_exec_lua(&args.exec_lua),
        saved_stdout,
        saved_vars,
        args.debug_borders,
        args.debug_anchors,
        args.debug_elements,
    )
}

fn redirect_if_quiet(args: &Args) -> Option<i32> {
    let quiet = matches!(
        args.command,
        Some(Commands::LuaErrors) | Some(Commands::SelfTest { .. })
    );
    if quiet {
        wow_ui_sim::lua_errors::redirect_stdout_to_stderr()
    } else {
        None
    }
}

fn init_and_load(
    args: &Args,
    screen: ScreenKind,
) -> (
    WowLuaEnv,
    Rc<RefCell<WowFontSystem>>,
    Option<SavedVariablesManager>,
) {
    let env = WowLuaEnv::new().expect("failed to create Lua env");
    let font_system = Rc::new(RefCell::new(WowFontSystem::new(&PathBuf::from("./fonts"))));
    init_environment(args, &env, &font_system);
    env.set_screen_mode(screen);

    let mut saved_vars = configure_saved_vars(args);
    addon_loading::load_blizzard_addons(&env, screen);
    addon_loading::load_third_party_addons(
        args.skip_addons(),
        args.is_test_command(),
        &env,
        &mut saved_vars,
        screen,
    );
    env.sync_addon_names_to_lua();
    env.apply_post_load_workarounds();
    (env, font_system, saved_vars)
}

fn resolve_exec_lua(arg: &Option<String>) -> Option<String> {
    arg.as_ref().map(|s| {
        if let Some(path) = s.strip_prefix('@') {
            std::fs::read_to_string(path).unwrap_or_else(|e| {
                eprintln!("[exec-lua] Failed to read {path}: {e}");
                String::new()
            })
        } else {
            s.clone()
        }
    })
}

fn configure_saved_vars(args: &Args) -> Option<SavedVariablesManager> {
    use wow_ui_sim::saved_variables::WtfConfig;
    let skip = args.no_saved_vars
        || std::env::var("WOW_SIM_NO_SAVED_VARS")
            .map(|v| v == "1")
            .unwrap_or(false);
    if skip {
        logging::println_elapsed("SavedVariables loading disabled");
        return None;
    }
    let mut saved_vars = SavedVariablesManager::new();
    let wtf_path = PathBuf::from("/syncthing/Sync/Projects/wow/WTF");
    if wtf_path.exists() {
        let wtf = WtfConfig::new(wtf_path, "50868465#2", "Burning Blade", "Haky");
        logging::println_elapsed(&format!(
            "WTF config: {} @ {}/{}",
            wtf.account, wtf.realm, wtf.character
        ));
        saved_vars.set_wtf_config(wtf);
    }
    Some(saved_vars)
}

fn init_sound(env: &WowLuaEnv) {
    let skip = std::env::var("WOW_SIM_NO_SOUND")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    if skip {
        logging::println_elapsed("Sound disabled");
        return;
    }
    let sound_dir = PathBuf::from("./sounds");
    match wow_ui_sim::sound::SoundManager::new(sound_dir) {
        Some(mgr) => {
            logging::println_elapsed("Sound initialized");
            env.state().borrow_mut().sound_manager = Some(mgr);
        }
        None => logging::println_elapsed("Sound: no audio device available"),
    }
}

fn apply_resource_limits() {
    let max_mem_gb: u64 = std::env::var("WOW_SIM_MAX_MEM_GB")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(10);
    let max_mem_bytes = max_mem_gb * 1024 * 1024 * 1024;
    let mem_limit = libc::rlimit {
        rlim_cur: max_mem_bytes,
        rlim_max: max_mem_bytes,
    };
    unsafe {
        libc::setrlimit(libc::RLIMIT_AS, &mem_limit);
    }
    let max_cores: usize = std::env::var("WOW_SIM_MAX_CORES")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);
    unsafe {
        let mut cpuset: libc::cpu_set_t = std::mem::zeroed();
        for i in 0..max_cores {
            libc::CPU_SET(i, &mut cpuset);
        }
        libc::sched_setaffinity(0, std::mem::size_of::<libc::cpu_set_t>(), &cpuset);
    }
    logging::println_elapsed(&format!(
        "Resource limits: {max_mem_gb}GB memory, {max_cores} CPU core(s)"
    ));
}

fn init_environment(_args: &Args, env: &WowLuaEnv, font_system: &Rc<RefCell<WowFontSystem>>) {
    logging::init_process_start_time(env.state().borrow().start_time);
    apply_resource_limits();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    env.set_font_system(Rc::clone(font_system));
    init_sound(env);
    {
        let mut state = env.state().borrow_mut();
        state.addon_base_paths = vec![
            PathBuf::from("./Interface/BlizzardUI"),
            PathBuf::from("./Interface/AddOns"),
        ];
    }
    wow_ui_sim::xml::register_intrinsic_templates();
}

use wow_ui_sim::startup::{apply_delay, run_extra_update_ticks, settle_headless_startup};

#[allow(clippy::too_many_arguments)]
fn dispatch_command(
    command: Option<Commands>,
    env: WowLuaEnv,
    font_system: Rc<RefCell<WowFontSystem>>,
    delay: Option<u64>,
    exec_lua: Option<String>,
    saved_stdout: Option<i32>,
    saved_vars: Option<SavedVariablesManager>,
    debug_borders: bool,
    debug_anchors: bool,
    debug_elements: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        Some(Commands::DumpTree {
            filter,
            filter_key,
            visible_only,
            width,
            height,
        }) => {
            run_dump_tree(
                &env,
                filter,
                filter_key,
                visible_only,
                width,
                height,
                delay,
                exec_lua.as_deref(),
            );
        }
        #[cfg(feature = "gui")]
        Some(Commands::Screenshot {
            output,
            width,
            height,
            filter,
            crop,
            dump_tree,
        }) => {
            run_screenshot(
                &env,
                &font_system,
                output,
                width,
                height,
                filter,
                crop,
                delay,
                exec_lua.as_deref(),
                dump_tree,
            );
        }
        Some(Commands::LuaErrors) => {
            wow_ui_sim::lua_errors::run_lua_errors(&env, saved_stdout, exec_lua.as_deref());
        }
        Some(Commands::SelfTest {
            max_ticks,
            categories,
        }) => {
            if let Some(c) = &categories {
                wow_ui_sim::self_test::inject_category_filter(&env, c);
            }
            wow_ui_sim::self_test::run_startup(&env);
            wow_ui_sim::self_test::run_test(&env, max_ticks, exec_lua.as_deref(), saved_stdout);
        }
        Some(Commands::RunTests { addon_name }) => {
            settle_headless_startup(&env);
            wow_ui_sim::addon_tests::run_addon_tests(&env, &addon_name, exec_lua.as_deref());
        }
        #[cfg(feature = "gui")]
        Some(Commands::DumpTexture {
            output,
            filter,
            frame_filter,
        }) => {
            run_dump_texture(&env, &font_system, output, filter, frame_filter);
        }
        #[cfg(feature = "gui")]
        None => {
            let debug = wow_ui_sim::DebugOptions {
                borders: debug_borders || debug_elements,
                anchors: debug_anchors || debug_elements,
            };
            wow_ui_sim::run_iced_ui(env, debug, saved_vars, exec_lua)?;
        }
        #[cfg(not(feature = "gui"))]
        None => {
            eprintln!("GUI not available (compiled without 'gui' feature).");
            std::process::exit(1);
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn run_dump_tree(
    env: &WowLuaEnv,
    filter: Option<String>,
    filter_key: Option<String>,
    visible_only: bool,
    width: u32,
    height: u32,
    delay: Option<u64>,
    exec_lua: Option<&str>,
) {
    settle_headless_startup(env);
    if let Some(code) = exec_lua
        && let Err(e) = env.exec(code)
    {
        eprintln!("[exec-lua] error: {e}");
    }
    run_extra_update_ticks(env, 3);
    apply_delay(delay);
    let state = env.state().borrow();
    let addon_names: Vec<String> = state.addons.iter().map(|a| a.folder_name.clone()).collect();
    wow_ui_sim::dump::print_frame_tree(
        &state.widgets,
        &addon_names,
        filter.as_deref(),
        filter_key.as_deref(),
        visible_only,
        width as f32,
        height as f32,
    );
}

#[cfg(feature = "gui")]
fn parse_crop(s: &str) -> Option<(u32, u32, u32, u32)> {
    let (dims, rest) = s.split_once('+')?;
    let (x_str, y_str) = rest.split_once('+')?;
    let (w_str, h_str) = dims.split_once('x')?;
    Some((
        w_str.parse().ok()?,
        h_str.parse().ok()?,
        x_str.parse().ok()?,
        y_str.parse().ok()?,
    ))
}

#[cfg(feature = "gui")]
fn apply_crop(img: image::RgbaImage, crop_str: &str) -> image::RgbaImage {
    use image::GenericImageView;
    let (cw, ch, cx, cy) = parse_crop(crop_str).unwrap_or_else(|| {
        eprintln!("Invalid crop format '{}', expected WxH+X+Y", crop_str);
        std::process::exit(1);
    });
    if cx + cw > img.width() || cy + ch > img.height() {
        eprintln!("Crop region exceeds image bounds");
        std::process::exit(1);
    }
    img.view(cx, cy, cw, ch).to_image()
}

#[cfg(feature = "gui")]
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
    use wow_ui_sim::iced_app::build_quad_batch_for_registry;
    use wow_ui_sim::render::GlyphAtlas;
    let mut glyph_atlas = GlyphAtlas::new();
    let batch = {
        let mut fs = font_system.borrow_mut();
        let buckets = {
            let mut state = env.state().borrow_mut();
            state.ensure_layout_rects();
            wow_ui_sim::iced_app::tooltip::update_tooltip_sizes(&mut state, &mut fs);
            let _ = state.get_strata_buckets();
            state.strata_buckets.as_ref().unwrap().clone()
        };
        let state = env.state().borrow();
        let tooltip_data = wow_ui_sim::iced_app::tooltip::collect_tooltip_data(&state);
        build_quad_batch_for_registry(
            &state.widgets,
            (width as f32, height as f32),
            filter,
            None,
            None,
            Some((&mut fs, &mut glyph_atlas)),
            Some(&state.message_frames),
            Some(&tooltip_data),
            &buckets,
        )
    };
    (batch, glyph_atlas)
}

#[cfg(feature = "gui")]
#[allow(clippy::too_many_arguments)]
fn run_screenshot(
    env: &WowLuaEnv,
    font_system: &Rc<RefCell<WowFontSystem>>,
    output: PathBuf,
    width: u32,
    height: u32,
    filter: Option<String>,
    crop: Option<String>,
    delay: Option<u64>,
    exec_lua: Option<&str>,
    dump_tree: Option<Option<String>>,
) {
    use wow_ui_sim::render::headless::render_to_image;
    settle_headless_startup(env);
    env.set_screen_size(width as f32, height as f32);
    wow_ui_sim::debug_helpers::debug_show_game_menu(env);
    if let Some(code) = exec_lua
        && let Err(e) = env.exec(code)
    {
        eprintln!("[exec-lua] error: {e}");
    }
    run_extra_update_ticks(env, 3);
    apply_delay(delay);
    let (batch, glyph_atlas) =
        build_screenshot_batch(env, font_system, width, height, filter.as_deref());
    if let Some(dump_filter) = &dump_tree {
        dump_screenshot_tree(env, dump_filter.as_deref(), width, height);
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
    let img = render_to_image(&batch, &mut tex_mgr, width, height, glyph_data);
    let img = match crop.as_deref() {
        Some(crop_str) => apply_crop(img, crop_str),
        None => img,
    };
    let output = output.with_extension("webp");
    save_screenshot(&img, &output);
    eprintln!(
        "Saved {}x{} screenshot to {}",
        img.width(),
        img.height(),
        output.display()
    );
}

#[cfg(feature = "gui")]
fn dump_screenshot_tree(env: &WowLuaEnv, filter_key: Option<&str>, w: u32, h: u32) {
    let state = env.state().borrow();
    let addon_names: Vec<String> = state.addons.iter().map(|a| a.folder_name.clone()).collect();
    wow_ui_sim::dump::print_frame_tree(
        &state.widgets,
        &addon_names,
        None,
        filter_key,
        false,
        w as f32,
        h as f32,
    );
}

#[cfg(feature = "gui")]
fn save_screenshot(img: &image::RgbaImage, output: &std::path::Path) {
    let output = output.with_extension("webp");
    let encoder = webp::Encoder::from_rgba(img.as_raw(), img.width(), img.height());
    let mem = encoder.encode(15.0);
    if let Err(e) = std::fs::write(&output, &*mem) {
        eprintln!("Failed to save WebP: {}", e);
        std::process::exit(1);
    }
}

#[cfg(feature = "gui")]
fn run_dump_texture(
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

#[cfg(feature = "gui")]
fn create_texture_manager() -> wow_ui_sim::texture::TextureManager {
    use wow_ui_sim::texture::TextureManager;
    let config = wow_ui_sim::config::SimConfig::load();
    let home = dirs::home_dir().unwrap_or_default();
    let local_textures = PathBuf::from("./textures");
    let textures_path = if local_textures.exists() {
        local_textures
    } else {
        home.join("Repos/wow-ui-textures")
    };
    let mut mgr = TextureManager::new(textures_path)
        .with_interface_path(home.join("Projects/wow/Interface"))
        .with_addons_path(PathBuf::from("./Interface/AddOns"))
        .with_disk_cache("./cache/textures");
    mgr.preload_talent_textures(790);
    mgr.preload_talent_panel_textures(&config.player_class);
    mgr
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn legacy_character_select_flag_maps_to_character_select_screen() {
        let args = Args::try_parse_from(["wow-sim", "--character-select"])
            .expect("legacy character-select flag should parse");
        assert_eq!(args.effective_screen(), ScreenKind::CharacterSelect);
    }

    #[test]
    fn explicit_screen_still_parses_character_select() {
        let args = Args::try_parse_from(["wow-sim", "--screen", "character-select"])
            .expect("screen option should parse character-select");
        assert_eq!(args.effective_screen(), ScreenKind::CharacterSelect);
    }

    #[test]
    fn explicit_screen_parses_character_create() {
        let args = Args::try_parse_from(["wow-sim", "--screen", "character-create"])
            .expect("screen option should parse character-create");
        assert_eq!(args.effective_screen(), ScreenKind::CharacterCreate);
    }
}
