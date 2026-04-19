use std::path::PathBuf;
use std::time::{Duration, Instant};

use iced::mouse;
use iced::widget::shader::Program;
use iced::{Rectangle, Size};

use crate::lua_api::WowLuaEnv;

use super::app::{
    App, FALLBACK_TEXTURES_PATH, INIT_DEBUG, INIT_ENV, INIT_SAVED_VARS, INIT_TEXTURES,
    LOCAL_TEXTURES_PATH,
};
use super::{DebugOptions, Message};

const BENCHMARK_SIZE: Size = Size {
    width: 1024.0,
    height: 768.0,
};
const MAX_SETTLE_FRAMES: usize = 256;
const FORCED_TICK_INTERVAL: Duration = Duration::from_millis(17);

#[derive(Debug, Clone)]
pub struct SpellbookBenchmarkReport {
    pub startup_idle: BenchmarkPhase,
    pub first_open: BenchmarkPhase,
    pub first_close: BenchmarkPhase,
    pub second_open: BenchmarkPhase,
}

#[derive(Debug, Clone)]
pub struct BenchmarkPhase {
    pub name: &'static str,
    pub keypress_elapsed: Duration,
    pub settle_elapsed: Duration,
    pub tick_elapsed: Duration,
    pub draw_elapsed: Duration,
    pub frames: usize,
    pub textures_loaded: usize,
    pub bc_textures_loaded: usize,
    pub max_pending_dirty_ids: usize,
    pub spellbook_shown: bool,
}

#[derive(Debug, Clone, Copy)]
struct PhaseOptions {
    name: &'static str,
    keypress: Option<&'static str>,
    expect_visible: bool,
}

#[derive(Debug, Clone, Copy)]
struct FrameTelemetry {
    textures_loaded: usize,
    bc_textures_loaded: usize,
}

pub fn benchmark_spellbook_open_in_gui(env: WowLuaEnv) -> crate::Result<SpellbookBenchmarkReport> {
    let mut app = boot_benchmark_app(env);
    let startup_idle = benchmark_phase(
        &mut app,
        PhaseOptions {
            name: "startup_idle",
            keypress: None,
            expect_visible: false,
        },
    )?;
    let first_open = benchmark_phase(
        &mut app,
        PhaseOptions {
            name: "first_open",
            keypress: Some("S"),
            expect_visible: true,
        },
    )?;
    let first_close = benchmark_phase(
        &mut app,
        PhaseOptions {
            name: "first_close",
            keypress: Some("S"),
            expect_visible: false,
        },
    )?;
    let second_open = benchmark_phase(
        &mut app,
        PhaseOptions {
            name: "second_open",
            keypress: Some("S"),
            expect_visible: true,
        },
    )?;
    Ok(SpellbookBenchmarkReport {
        startup_idle,
        first_open,
        first_close,
        second_open,
    })
}

fn boot_benchmark_app(env: WowLuaEnv) -> App {
    INIT_ENV.with(|cell| *cell.borrow_mut() = Some(env));
    INIT_TEXTURES.with(|cell| *cell.borrow_mut() = Some(default_textures_path()));
    INIT_DEBUG.with(|cell| *cell.borrow_mut() = Some(DebugOptions::default()));
    INIT_SAVED_VARS.with(|cell| *cell.borrow_mut() = None);

    let (app, _task) = App::boot();
    app
}

fn default_textures_path() -> PathBuf {
    if PathBuf::from(LOCAL_TEXTURES_PATH).exists() {
        PathBuf::from(LOCAL_TEXTURES_PATH)
    } else {
        PathBuf::from(FALLBACK_TEXTURES_PATH)
    }
}

fn benchmark_phase(app: &mut App, options: PhaseOptions) -> crate::Result<BenchmarkPhase> {
    let keypress_elapsed = dispatch_keypress(app, options.keypress);
    let settle_started = Instant::now();
    let mut tick_elapsed = Duration::ZERO;
    let mut draw_elapsed = Duration::ZERO;
    let mut frames = 0_usize;
    let mut textures_loaded = 0_usize;
    let mut bc_textures_loaded = 0_usize;
    let mut max_pending_dirty_ids = pending_dirty_count(app);

    while !is_quiescent(app) {
        if frames >= MAX_SETTLE_FRAMES {
            return Err(crate::Error::Other(format!(
                "{} did not settle after {} frames",
                options.name, MAX_SETTLE_FRAMES
            )));
        }
        let (tick_dur, draw_dur, frame_telemetry) = run_benchmark_frame(app);
        tick_elapsed += tick_dur;
        draw_elapsed += draw_dur;
        textures_loaded += frame_telemetry.textures_loaded;
        bc_textures_loaded += frame_telemetry.bc_textures_loaded;
        max_pending_dirty_ids = max_pending_dirty_ids.max(pending_dirty_count(app));
        frames += 1;
    }

    let spellbook_shown = is_spellbook_shown(app)?;
    if spellbook_shown != options.expect_visible {
        return Err(crate::Error::Other(format!(
            "{} ended with spellbook_shown={} expected={}",
            options.name, spellbook_shown, options.expect_visible
        )));
    }

    Ok(BenchmarkPhase {
        name: options.name,
        keypress_elapsed,
        settle_elapsed: settle_started.elapsed(),
        tick_elapsed,
        draw_elapsed,
        frames,
        textures_loaded,
        bc_textures_loaded,
        max_pending_dirty_ids,
        spellbook_shown,
    })
}

fn dispatch_keypress(app: &mut App, keypress: Option<&'static str>) -> Duration {
    let Some(keypress) = keypress else {
        return Duration::ZERO;
    };
    let started = Instant::now();
    let _ = app.update(Message::KeyPress(
        keypress.to_string(),
        None,
        Instant::now(),
    ));
    started.elapsed()
}

fn run_benchmark_frame(app: &mut App) -> (Duration, Duration, FrameTelemetry) {
    app.last_on_update_time = Instant::now() - FORCED_TICK_INTERVAL;

    let tick_started = Instant::now();
    let _ = app.update(Message::ProcessTimers);
    let tick_elapsed = tick_started.elapsed();

    let draw_started = Instant::now();
    let primitive = <&App as Program<Message>>::draw(
        &&*app,
        &(),
        mouse::Cursor::Unavailable,
        Rectangle::with_size(BENCHMARK_SIZE),
    );
    let draw_elapsed = draw_started.elapsed();

    (
        tick_elapsed,
        draw_elapsed,
        FrameTelemetry {
            textures_loaded: primitive.textures.len(),
            bc_textures_loaded: primitive.bc_textures.len(),
        },
    )
}

fn is_quiescent(app: &App) -> bool {
    app.strata_dirty.get() == 0
        && !app.textures_pending.get()
        && app
            .pending_dirty_ids
            .borrow()
            .as_ref()
            .is_none_or(|ids| ids.is_empty())
}

fn pending_dirty_count(app: &App) -> usize {
    app.pending_dirty_ids
        .borrow()
        .as_ref()
        .map_or(0, rustc_hash::FxHashSet::len)
}

fn is_spellbook_shown(app: &App) -> crate::Result<bool> {
    let env = app.env.borrow();
    env.eval::<bool>(
        "return PlayerSpellsFrame ~= nil and PlayerSpellsFrame:IsShown() and PlayerSpellsUtil.GetCurrentTabID() == PlayerSpellsUtil.FrameTabs.SpellBook",
    )
}
