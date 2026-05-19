//! App struct definition and core initialization.

use iced::{Point, Size, Task};
use rilua::LuaApiMut;
use rustc_hash::{FxHashMap, FxHashSet};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

#[cfg(not(target_os = "linux"))]
use crate::inspector_server_stub as debug_server;
use crate::lua_api::WowLuaEnv;
use crate::lua_server;
use crate::render::{GlyphAtlas, WowFontSystem};
use crate::saved_variables::SavedVariablesManager;
use crate::texture::TextureManager;
use debug_server::ScreenshotData;
#[cfg(target_os = "linux")]
use iced_layout_inspector::server as debug_server;

use super::Message;
use super::state::InspectorState;

type RenderResources = (
    Rc<RefCell<TextureManager>>,
    Rc<RefCell<WowFontSystem>>,
    Rc<RefCell<GlyphAtlas>>,
);

struct AppInitialSelections {
    xp_level: String,
    party_size: String,
    class: String,
    race: String,
    rot_level: String,
    movement: crate::config::MovementConfig,
}

const DEFAULT_FAST_TICK_MS: u64 = 16;

/// Debug visualization options.
#[derive(Default, Clone)]
pub struct DebugOptions {
    pub borders: bool,
    pub anchors: bool,
}

// Thread-local storage for init params
thread_local! {
    pub static INIT_ENV: RefCell<Option<WowLuaEnv>> = const { RefCell::new(None) };
    pub static INIT_DEBUG: RefCell<Option<DebugOptions>> = const { RefCell::new(None) };
    pub static INIT_SAVED_VARS: RefCell<Option<SavedVariablesManager>> = const { RefCell::new(None) };
    pub static INIT_EXEC_LUA: RefCell<Option<(String, bool)>> = const { RefCell::new(None) };
}

/// Fire the standard WoW startup events.
pub fn fire_startup_events(env: &Rc<RefCell<WowLuaEnv>>) {
    let env = env.borrow();
    let screen = env.state().borrow().screen_kind;
    crate::logging::println_elapsed("[Startup] beginning startup event dispatch");
    crate::startup::fire_startup_events_for_screen(&env, screen);
    crate::logging::println_elapsed("[Startup] startup event dispatch complete");
}

/// Application state.
pub struct App {
    pub(crate) env: Rc<RefCell<WowLuaEnv>>,
    pub(crate) log_messages: Vec<String>,
    pub(crate) command_input: String,
    pub(crate) texture_manager: Rc<RefCell<TextureManager>>,
    pub(crate) font_system: Rc<RefCell<WowFontSystem>>,
    pub(crate) glyph_atlas: Rc<RefCell<GlyphAtlas>>,
    pub(crate) hovered_frame: Option<u64>,
    pub(crate) pressed_frame: Option<u64>,
    pub(crate) mouse_down_frame: Option<u64>,
    pub(crate) right_mouse_down_frame: Option<u64>,
    /// Position where mouse button was pressed (for drag threshold detection).
    pub(crate) mouse_down_pos: Option<Point>,
    /// True once drag threshold is exceeded for the current press.
    pub(crate) dragging: bool,
    pub(crate) scroll_offset: f32,
    /// Current canvas size (updated each frame for layout calculations).
    pub(crate) screen_size: std::cell::Cell<Size>,
    pub(crate) debug_rx: Option<mpsc::Receiver<debug_server::Command>>,
    pub(crate) pending_screenshot: Option<oneshot::Sender<Result<ScreenshotData, String>>>,
    pub(crate) lua_rx: Option<std::sync::mpsc::Receiver<lua_server::LuaCommand>>,
    /// Draw red debug borders around all frames when true.
    pub(crate) debug_borders: bool,
    /// Draw green anchor points on all frames when true.
    pub(crate) debug_anchors: bool,
    /// Cached merged quad batch (all strata combined), used by draw().
    pub(crate) cached_quads: RefCell<Option<(Size, std::sync::Arc<crate::render::QuadBatch>)>>,
    /// Per-strata cached quad batches. Index = FrameStrata::as_index().
    pub(crate) cached_strata_quads: RefCell<
        [Option<std::sync::Arc<crate::render::QuadBatch>>; crate::widget::FrameStrata::COUNT],
    >,
    /// Per-strata per-frame quad snapshots for incremental strata rebuild.
    pub(crate) cached_frame_snapshots: RefCell<
        [Option<std::collections::HashMap<u64, crate::render::FrameQuadSnapshot>>;
            crate::widget::FrameStrata::COUNT],
    >,
    /// Dirty frame IDs from the last timer tick. `None` means full rebuild needed.
    pub(crate) pending_dirty_ids: RefCell<Option<rustc_hash::FxHashSet<u64>>>,
    /// Spatial grid for fast hit testing (rebuilt when layout changes).
    pub(crate) cached_hittable: RefCell<Option<super::hit_grid::HitGrid>>,
    /// Per-strata dirty bitmask — bit `i` means strata index `i` needs re-emit.
    pub(crate) strata_dirty: std::cell::Cell<u16>,
    /// True when texture loading was capped and more textures are pending.
    pub(crate) textures_pending: std::cell::Cell<bool>,
    /// Persistent queue of normalized pending texture paths (draw/tick shared).
    pub(crate) pending_texture_path_queue: RefCell<VecDeque<String>>,
    /// Membership set for `pending_texture_path_queue`.
    pub(crate) pending_texture_path_set: RefCell<FxHashSet<String>>,
    /// Per-strata pending requests keyed by normalized texture path.
    pub(crate) strata_pending_texture_requests: RefCell<
        [FxHashMap<String, Vec<crate::render::TextureRequest>>; crate::widget::FrameStrata::COUNT],
    >,
    /// Aggregated pending requests keyed by normalized texture path.
    pub(crate) pending_texture_requests_by_path:
        RefCell<FxHashMap<String, Vec<crate::render::TextureRequest>>>,
    /// Paths known-ready in GPU atlas from prior successful request handles.
    pub(crate) ready_texture_path_cache: RefCell<FxHashSet<String>>,
    /// Most recent main-thread phase that can block event handling.
    pub(crate) main_thread_phase: RefCell<(&'static str, std::time::Instant)>,
    /// Count of stale timer ticks dropped since the last key log.
    pub(crate) dropped_stale_timer_ticks: std::cell::Cell<u32>,
    /// Oldest age among dropped stale timer ticks since the last key log.
    pub(crate) oldest_dropped_timer_tick_age: std::cell::Cell<std::time::Duration>,
    /// FPS counter: frame count since last update (interior mutability for draw()).
    pub(crate) frame_count: std::cell::Cell<u32>,
    /// Total draw time accumulated since the last metrics refresh.
    pub(crate) draw_time_accum_ms: std::cell::Cell<f32>,
    /// Total timer tick time accumulated since the last metrics refresh.
    pub(crate) tick_time_accum_ms: std::cell::Cell<f32>,
    /// Count of timer ticks accumulated since the last metrics refresh.
    pub(crate) tick_count: std::cell::Cell<u32>,
    /// FPS counter: last FPS calculation time.
    pub(crate) fps_last_time: std::time::Instant,
    /// Current FPS value.
    pub(crate) fps: f32,
    /// Timer tick time for display, amortized per draw over the sample window.
    pub(crate) tick_time_display: f32,
    /// Draw time for display, averaged per draw over the sample window.
    pub(crate) draw_time_display: f32,
    /// Remaining wall time for display after subtracting tick + draw.
    pub(crate) other_time_display: f32,
    /// Current mouse position in canvas coordinates.
    pub(crate) mouse_position: Option<Point>,
    /// Currently inspected frame ID.
    pub(crate) inspected_frame: Option<u64>,
    /// Whether the inspector panel is visible.
    pub(crate) inspector_visible: bool,
    /// Position of the inspector panel.
    pub(crate) inspector_position: Point,
    /// Inspector panel state (editable fields).
    pub(crate) inspector_state: InspectorState,
    /// Whether the frames panel is collapsed.
    pub(crate) frames_panel_collapsed: bool,
    /// Last time OnUpdate handlers were fired (for elapsed calculation).
    pub(crate) last_on_update_time: std::time::Instant,
    /// SavedVariables manager for persisting addon data on exit.
    pub(crate) saved_vars: Option<SavedVariablesManager>,
    /// Lua code to execute after first frame (from --exec-lua). Bool is
    /// `true` when `--exec-lua-secure` was passed (run under secureenv).
    pub(crate) pending_exec_lua: Option<(String, bool)>,
    /// Currently selected XP bar level label.
    pub(crate) selected_xp_level: String,
    /// Currently selected party size label.
    pub(crate) selected_party_size: String,
    /// Last time party health was ticked (random walk every 2 seconds).
    pub(crate) last_party_health_tick: std::time::Instant,
    /// Currently selected player class name (for picker display).
    pub(crate) selected_class: String,
    /// Currently selected player race name (for picker display).
    pub(crate) selected_race: String,
    /// Currently selected rot damage level label.
    pub(crate) selected_rot_level: String,
    /// Whether the options modal is visible.
    pub(crate) options_modal_visible: bool,
    /// Movement state toggles (mirrors SimState.movement for UI display).
    pub(crate) movement: crate::config::MovementConfig,
}

pub(crate) struct AppInit {
    pub(crate) env: Rc<RefCell<WowLuaEnv>>,
    pub(crate) log_messages: Vec<String>,
    pub(crate) texture_manager: Rc<RefCell<TextureManager>>,
    pub(crate) font_system: Rc<RefCell<WowFontSystem>>,
    pub(crate) glyph_atlas: Rc<RefCell<GlyphAtlas>>,
    pub(crate) cmd_rx: mpsc::Receiver<debug_server::Command>,
    pub(crate) lua_rx: std::sync::mpsc::Receiver<lua_server::LuaCommand>,
    pub(crate) debug_borders: bool,
    pub(crate) debug_anchors: bool,
    pub(crate) saved_vars: Option<SavedVariablesManager>,
    pub(crate) config: crate::config::SimConfig,
}

impl AppInitialSelections {
    fn from_config(config: &crate::config::SimConfig) -> Self {
        Self {
            xp_level: config.xp_level.clone(),
            party_size: config.party_size.to_string(),
            class: config.player_class.clone(),
            race: config.player_race.clone(),
            rot_level: config.rot_damage_level.clone(),
            movement: config.movement.clone(),
        }
    }
}

macro_rules! app_from_initial_state {
    ($init:ident, $selections:ident, $now:ident, $initial_screen_size:ident) => {
        App {
            env: $init.env,
            log_messages: $init.log_messages,
            command_input: String::new(),
            texture_manager: $init.texture_manager,
            font_system: $init.font_system,
            glyph_atlas: $init.glyph_atlas,
            hovered_frame: None,
            pressed_frame: None,
            mouse_down_frame: None,
            right_mouse_down_frame: None,
            mouse_down_pos: None,
            dragging: false,
            scroll_offset: 0.0,
            screen_size: std::cell::Cell::new($initial_screen_size),
            debug_rx: Some($init.cmd_rx),
            pending_screenshot: None,
            lua_rx: Some($init.lua_rx),
            debug_borders: $init.debug_borders,
            debug_anchors: $init.debug_anchors,
            cached_quads: RefCell::new(None),
            cached_strata_quads: RefCell::new(std::array::from_fn(|_| None)),
            cached_frame_snapshots: RefCell::new(std::array::from_fn(|_| None)),
            pending_dirty_ids: RefCell::new(None),
            cached_hittable: RefCell::new(None),
            strata_dirty: std::cell::Cell::new((1u16 << crate::widget::FrameStrata::COUNT) - 1),
            textures_pending: std::cell::Cell::new(false),
            pending_texture_path_queue: RefCell::new(VecDeque::new()),
            pending_texture_path_set: RefCell::new(FxHashSet::default()),
            strata_pending_texture_requests: RefCell::new(std::array::from_fn(|_| {
                FxHashMap::default()
            })),
            pending_texture_requests_by_path: RefCell::new(FxHashMap::default()),
            ready_texture_path_cache: RefCell::new(FxHashSet::default()),
            main_thread_phase: RefCell::new(("boot", $now)),
            dropped_stale_timer_ticks: std::cell::Cell::new(0),
            oldest_dropped_timer_tick_age: std::cell::Cell::new(std::time::Duration::ZERO),
            frame_count: std::cell::Cell::new(0),
            draw_time_accum_ms: std::cell::Cell::new(0.0),
            tick_time_accum_ms: std::cell::Cell::new(0.0),
            tick_count: std::cell::Cell::new(0),
            fps_last_time: $now,
            fps: 0.0,
            tick_time_display: 0.0,
            draw_time_display: 0.0,
            other_time_display: 0.0,
            mouse_position: None,
            inspected_frame: None,
            inspector_visible: false,
            inspector_position: Point::new(100.0, 100.0),
            inspector_state: InspectorState::default(),
            frames_panel_collapsed: true,
            last_on_update_time: $now,
            saved_vars: $init.saved_vars,
            pending_exec_lua: INIT_EXEC_LUA.with(|cell| cell.borrow_mut().take()),
            selected_xp_level: $selections.xp_level,
            selected_party_size: $selections.party_size,
            last_party_health_tick: $now,
            selected_class: $selections.class,
            selected_race: $selections.race,
            selected_rot_level: $selections.rot_level,
            options_modal_visible: false,
            movement: $selections.movement,
        }
    };
}

impl App {
    pub fn title(_state: &Self) -> String {
        "WoW UI Simulator".to_string()
    }

    pub fn boot() -> (Self, Task<Message>) {
        let (env_rc, saved_vars) = Self::take_init_params();
        let config = crate::config::SimConfig::load();
        Self::apply_config_to_state(&env_rc, &config);
        Self::set_initial_gui_screen_size(&env_rc);

        Self::run_startup_sequence(&env_rc);
        env_rc
            .borrow()
            .state()
            .borrow_mut()
            .initialize_render_state();
        // Eagerly resolve all pending layouts so the first render doesn't pay the cost.
        env_rc.borrow().state().borrow_mut().ensure_layout_rects();
        let log_messages = Self::collect_startup_logs(&env_rc);

        let (texture_manager, font_system, glyph_atlas) = Self::init_rendering(&env_rc);
        let (cmd_rx, lua_rx) = Self::init_servers();
        let (debug_borders, debug_anchors) = Self::resolve_debug_flags();

        let app = Self::build_app(AppInit {
            env: env_rc,
            log_messages,
            texture_manager,
            font_system,
            glyph_atlas,
            cmd_rx,
            lua_rx,
            debug_borders,
            debug_anchors,
            saved_vars,
            config,
        });

        app.preload_initial_texture_requests();

        (app, Task::none())
    }

    /// Construct the App struct from initialized components.
    pub(crate) fn build_app(init: AppInit) -> Self {
        let now = std::time::Instant::now();
        let selections = AppInitialSelections::from_config(&init.config);
        let initial_screen_size = current_env_screen_size(&init.env);
        app_from_initial_state!(init, selections, now, initial_screen_size)
    }

    fn set_initial_gui_screen_size(env_rc: &Rc<RefCell<WowLuaEnv>>) {
        let initial_size = super::app_icon::initial_window_size();
        env_rc
            .borrow()
            .set_screen_size(initial_size.width, initial_size.height);
    }

    /// Apply saved config to SimState before startup events fire.
    fn apply_config_to_state(env_rc: &Rc<RefCell<WowLuaEnv>>, config: &crate::config::SimConfig) {
        use crate::lua_api::state::{CLASS_LABELS, RACE_DATA, ROT_DAMAGE_LEVELS};
        let env = env_rc.borrow();
        let mut state = env.state().borrow_mut();
        state.player.class_index = CLASS_LABELS
            .iter()
            .position(|&n| n == config.player_class)
            .map(|i| (i + 1) as i32)
            .unwrap_or(1);
        state.player.race_index = RACE_DATA
            .iter()
            .position(|(n, _, _)| *n == config.player_race)
            .unwrap_or(0);
        state.rot_damage_level = ROT_DAMAGE_LEVELS
            .iter()
            .position(|(l, _)| *l == config.rot_damage_level)
            .unwrap_or(0);
        state.player.movement = crate::lua_api::state::MovementState {
            moving: config.movement.moving,
            mounted: config.movement.mounted,
            flying: config.movement.flying,
            falling: config.movement.falling,
            swimming: config.movement.swimming,
        };
        crate::startup::resize_party_state(&mut state, usize::from(config.party_size.min(4)));
    }

    /// Extract init params from thread-local storage.
    /// Fire startup events, apply post-event workarounds, and hide default-hidden frames.
    fn run_startup_sequence(env_rc: &Rc<RefCell<WowLuaEnv>>) {
        fire_startup_events(env_rc);
        let env_ref = env_rc.borrow();
        env_ref.apply_post_event_workarounds();
        crate::startup::settle_startup_animation_groups(&env_ref);
        crate::startup::process_pending_timers(&env_ref);
        crate::startup::fire_gui_startup_on_update_tick(&env_ref);
        crate::startup::process_pending_timers(&env_ref);
        let _ = crate::lua_api::hide_runtime_hidden_frames(env_ref.lua());
        env_ref.state().borrow_mut().widgets.rebuild_anchor_index();
    }

    fn take_init_params() -> (Rc<RefCell<WowLuaEnv>>, Option<SavedVariablesManager>) {
        let env = INIT_ENV
            .with(|cell| cell.borrow_mut().take())
            .expect("WowLuaEnv not initialized");
        let saved_vars = INIT_SAVED_VARS.with(|cell| cell.borrow_mut().take());
        (Rc::new(RefCell::new(env)), saved_vars)
    }

    /// Drain console output collected during startup.
    fn collect_startup_logs(env_rc: &Rc<RefCell<WowLuaEnv>>) -> Vec<String> {
        let mut log_messages = vec!["UI loaded. Press Ctrl+R to reload.".to_string()];
        let env = env_rc.borrow();
        let mut state = env.state().borrow_mut();
        log_messages.append(&mut state.console_output);
        log_messages
    }

    /// Create texture manager, font system, and glyph atlas.
    fn init_rendering(env_rc: &Rc<RefCell<WowLuaEnv>>) -> RenderResources {
        let mut tex_mgr =
            TextureManager::new().with_addons_paths(crate::paths::default_addons_paths());
        if Self::eager_startup_texture_preloads_enabled() {
            let class_name = {
                let env = env_rc.borrow();
                let state = env.state().borrow();
                crate::lua_api::state::CLASS_LABELS
                    .get((state.player.class_index - 1).max(0) as usize)
                    .copied()
                    .unwrap_or("Warrior")
                    .to_string()
            };
            let is_glue_screen = {
                let env = env_rc.borrow();
                env.state().borrow().screen_kind.is_glue()
            };
            tex_mgr.preload_talent_textures(790);
            tex_mgr.preload_talent_panel_textures(&class_name);
            if !is_glue_screen {
                Self::preload_non_glue_textures(&mut tex_mgr, &class_name);
            }
        }
        let texture_manager = Rc::new(RefCell::new(tex_mgr));
        let font_system = Rc::new(RefCell::new(WowFontSystem::new()));
        env_rc.borrow().set_font_system(Rc::clone(&font_system));
        let glyph_atlas = Rc::new(RefCell::new(GlyphAtlas::new()));
        (texture_manager, font_system, glyph_atlas)
    }

    fn preload_non_glue_textures(tex_mgr: &mut TextureManager, class_name: &str) {
        if let Some(skill_line) = crate::lua_api::globals::spellbook_data::get_skill_line(2)
            && skill_line.name != class_name
        {
            tex_mgr.preload_talent_panel_textures(skill_line.name);
        }
        tex_mgr.preload_game_hud_textures();
        tex_mgr.preload_playerspells_runtime_textures();
        tex_mgr.preload_spellbook_icons();
    }

    fn eager_startup_texture_preloads_enabled() -> bool {
        std::env::var_os("WOW_SIM_EAGER_STARTUP_TEXTURES").is_some()
    }

    /// Start debug server and Lua REPL server.
    fn init_servers() -> (
        mpsc::Receiver<debug_server::Command>,
        std::sync::mpsc::Receiver<lua_server::LuaCommand>,
    ) {
        let (cmd_rx, _guard) = debug_server::init();
        #[cfg(target_os = "linux")]
        std::mem::forget(_guard);
        #[cfg(not(target_os = "linux"))]
        let _ = _guard;

        let lua_rx = lua_server::init();
        (cmd_rx, lua_rx)
    }

    /// Resolve debug border/anchor flags from CLI and env vars.
    fn resolve_debug_flags() -> (bool, bool) {
        let init_debug = INIT_DEBUG
            .with(|cell| cell.borrow_mut().take())
            .unwrap_or_default();
        let debug_elements = std::env::var("WOW_SIM_DEBUG_ELEMENTS").is_ok();
        let debug_borders =
            init_debug.borders || debug_elements || std::env::var("WOW_SIM_DEBUG_BORDERS").is_ok();
        let debug_anchors =
            init_debug.anchors || debug_elements || std::env::var("WOW_SIM_DEBUG_ANCHORS").is_ok();

        if debug_borders || debug_anchors {
            eprintln!(
                "[wow-sim] Debug mode: borders={} anchors={}",
                debug_borders, debug_anchors
            );
        }
        (debug_borders, debug_anchors)
    }

    pub(crate) fn set_main_thread_phase(&self, phase: &'static str) {
        crate::logging::set_blocking_phase(phase);
        *self.main_thread_phase.borrow_mut() = (phase, std::time::Instant::now());
    }
}

fn current_env_screen_size(env: &Rc<RefCell<WowLuaEnv>>) -> Size {
    let env = env.borrow();
    let state = env.state().borrow();
    Size::new(state.screen_width, state.screen_height)
}

impl App {
    /// Mark specific strata as dirty (need quad re-emit + GPU re-upload).
    pub(crate) fn mark_strata_dirty(&self, mask: u16) {
        self.strata_dirty.set(self.strata_dirty.get() | mask);
    }

    /// Mark ALL strata as dirty (full rebuild).
    /// Also clears per-frame snapshot caches so the next rebuild is non-incremental.
    pub(crate) fn mark_all_strata_dirty(&self) {
        self.strata_dirty
            .set((1u16 << crate::widget::FrameStrata::COUNT) - 1);
        *self.cached_frame_snapshots.borrow_mut() = std::array::from_fn(|_| None);
        *self.pending_dirty_ids.borrow_mut() = None;
    }

    /// Determine how often the timer subscription should tick.
    ///
    /// Returns `Some(interval)` when periodic work is needed (animations,
    /// casting, pending C_Timers, rot damage), or `None` when fully idle
    /// (only user input triggers redraws).
    ///
    /// OnUpdate handlers cause a 1-second tick when any frames have them
    /// registered (e.g., buff duration countdown text).
    pub(crate) fn compute_tick_interval(&self) -> Option<std::time::Duration> {
        let env = self.env.borrow();
        let state = env.state().borrow();

        // Fast tick: playing visual animations, active cast, or dirty quads.
        let has_animations = state.animation_groups.values().any(|g| {
            g.playing
                && !g.paused
                && g.has_visual_effects()
                && state.widgets.is_ancestor_visible(g.owner_frame_id)
        });
        let has_casting = state.casting.is_some();
        let has_cooldowns = has_active_cooldowns(&state);
        let is_glue_screen = state.screen_kind.is_glue();
        if has_animations
            || has_casting
            || has_cooldowns
            || self.strata_dirty.get() != 0
            || self.textures_pending.get()
        {
            return Some(fast_tick_interval());
        }
        drop(state);

        // Timer tick: wake up when next C_Timer fires (min 16ms)
        if let Some(delay) = env.next_timer_delay() {
            return Some(delay.max(std::time::Duration::from_millis(16)));
        }
        drop(env);

        // Slow heartbeat: rot damage needs 2s ticks
        if self.selected_rot_level != "Off" {
            return Some(std::time::Duration::from_secs(2));
        }

        // Glue screens rely on interactive OnUpdate handlers for character
        // rotation and similar motion even when nothing else is animating.
        if is_glue_screen {
            return Some(std::time::Duration::from_millis(33));
        }

        // Idle heartbeat: fire OnUpdate handlers at 1s intervals even when
        // nothing else is active (e.g., buff duration countdown text).
        Some(std::time::Duration::from_secs(1))
    }
}

fn fast_tick_interval() -> std::time::Duration {
    let fast_tick_ms = std::env::var("WOW_SIM_TICK_MS")
        .ok()
        .as_deref()
        .and_then(parse_fast_tick_ms)
        .unwrap_or(DEFAULT_FAST_TICK_MS);
    std::time::Duration::from_millis(fast_tick_ms)
}

fn parse_fast_tick_ms(value: &str) -> Option<u64> {
    let tick_ms = value.trim().parse::<u64>().ok()?;
    (tick_ms > 0).then_some(tick_ms)
}

/// Check if any GCD or spell cooldowns are still active.
fn has_active_cooldowns(state: &crate::lua_api::SimState) -> bool {
    let now = state.start_time.elapsed().as_secs_f64();
    if let Some((start, dur)) = state.gcd {
        if now < start + dur {
            return true;
        }
    }
    state
        .spell_cooldowns
        .values()
        .any(|cd| now < cd.start + cd.duration)
}

impl Drop for App {
    fn drop(&mut self) {
        if let Some(ref saved_vars) = self.saved_vars {
            let env = self.env.borrow();
            let mut lua = env.rilua_mut();
            match saved_vars.save_all(lua.state_mut()) {
                Ok(()) => crate::logging::eprintln_elapsed(
                    "[wow-sim] SavedVariables saved to simulator storage",
                ),
                Err(e) => crate::logging::eprintln_elapsed(&format!(
                    "[wow-sim] SavedVariables save error: {e}"
                )),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::screen::ScreenKind;
    use crate::texture::TextureManager;
    use iced::Size;
    use tokio::sync::mpsc;

    fn build_env() -> Rc<RefCell<WowLuaEnv>> {
        Rc::new(RefCell::new(
            WowLuaEnv::new().expect("Failed to create Lua environment"),
        ))
    }

    fn build_test_app(screen_kind: ScreenKind) -> App {
        let env = build_env();
        env.borrow().set_screen_mode(screen_kind);
        env.borrow().set_screen_size(800.0, 600.0);

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

    #[test]
    fn glue_screens_tick_on_update_at_interactive_rate() {
        let app = build_test_app(ScreenKind::CharacterSelect);
        app.strata_dirty.set(0);

        assert_eq!(app.screen_size.get(), Size::new(800.0, 600.0));
        assert_eq!(
            app.compute_tick_interval(),
            Some(std::time::Duration::from_millis(33)),
        );
    }

    #[test]
    fn game_screen_keeps_idle_on_update_heartbeat() {
        let app = build_test_app(ScreenKind::Game);
        app.strata_dirty.set(0);

        assert_eq!(
            app.compute_tick_interval(),
            Some(std::time::Duration::from_secs(1)),
        );
    }

    #[test]
    fn gui_boot_seeds_sim_state_with_initial_window_size() {
        let env = build_env();

        App::set_initial_gui_screen_size(&env);

        assert_eq!(
            current_env_screen_size(&env),
            super::super::app_icon::initial_window_size()
        );
    }

    #[test]
    fn app_screen_size_starts_from_sim_state_size() {
        let app = build_test_app(ScreenKind::Game);

        assert_eq!(app.screen_size.get(), current_env_screen_size(&app.env));
    }

    #[test]
    fn gui_startup_drains_ready_timers_before_interactive_ticks() {
        let env = build_env();
        env.borrow()
            .exec(
                r#"
                __gui_startup_timer_fired = 0
                C_Timer.After(0, function()
                    __gui_startup_timer_fired = __gui_startup_timer_fired + 1
                end)
                "#,
            )
            .expect("startup timer setup should succeed");

        App::run_startup_sequence(&env);

        let fired: f64 = env
            .borrow()
            .eval("return __gui_startup_timer_fired")
            .expect("startup timer result should be readable");
        assert_eq!(fired, 1.0, "ready startup timers should be settled");
    }

    #[test]
    fn gui_startup_runs_one_on_update_before_interactive_ticks() {
        let env = build_env();
        env.borrow()
            .exec(
                r#"
                __gui_startup_on_update_fired = 0
                local frame = CreateFrame("Frame")
                frame:SetScript("OnUpdate", function(self)
                    __gui_startup_on_update_fired = __gui_startup_on_update_fired + 1
                    self:SetScript("OnUpdate", nil)
                end)
                "#,
            )
            .expect("startup OnUpdate setup should succeed");

        App::run_startup_sequence(&env);

        let fired: f64 = env
            .borrow()
            .eval("return __gui_startup_on_update_fired")
            .expect("startup OnUpdate result should be readable");
        assert_eq!(fired, 1.0, "one-shot startup OnUpdate should be settled");
    }

    #[test]
    fn parse_fast_tick_ms_accepts_positive_integers() {
        assert_eq!(parse_fast_tick_ms("1"), Some(1));
        assert_eq!(parse_fast_tick_ms(" 8 "), Some(8));
    }

    #[test]
    fn parse_fast_tick_ms_rejects_zero_and_invalid_values() {
        assert_eq!(parse_fast_tick_ms("0"), None);
        assert_eq!(parse_fast_tick_ms("abc"), None);
        assert_eq!(parse_fast_tick_ms(""), None);
    }
}
