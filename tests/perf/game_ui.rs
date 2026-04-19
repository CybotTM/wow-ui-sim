use std::time::{Duration, Instant};

use wow_ui_sim::loader::{discover_blizzard_addons_for_screen, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::startup::fire_startup_events_for_screen;

use crate::perf_base_game::{blizzard_ui_dir, new_game_env};

pub struct StartupPhaseTimings {
    addon_load_elapsed: Duration,
    post_load_workarounds_elapsed: Duration,
    startup_events_elapsed: Duration,
}

impl StartupPhaseTimings {
    pub fn addon_load_elapsed(&self) -> Duration {
        self.addon_load_elapsed
    }

    pub fn post_load_workarounds_elapsed(&self) -> Duration {
        self.post_load_workarounds_elapsed
    }

    pub fn startup_events_elapsed(&self) -> Duration {
        self.startup_events_elapsed
    }
}

pub struct LoadedGameUi {
    pub env: WowLuaEnv,
    phase_timings: StartupPhaseTimings,
    pub startup_elapsed: Duration,
}

impl LoadedGameUi {
    pub fn phase_timings(&self) -> &StartupPhaseTimings {
        &self.phase_timings
    }
}

pub fn load_timed_game_ui() -> LoadedGameUi {
    let started = Instant::now();
    let env = new_game_env();
    let addon_load_elapsed = load_game_ui_addons(&env);
    let post_load_workarounds_elapsed = apply_post_load_workarounds(&env);
    let startup_events_elapsed = fire_startup_events(&env);

    LoadedGameUi {
        env,
        phase_timings: StartupPhaseTimings {
            addon_load_elapsed,
            post_load_workarounds_elapsed,
            startup_events_elapsed,
        },
        startup_elapsed: started.elapsed(),
    }
}

fn fire_startup_events(env: &WowLuaEnv) -> Duration {
    let started = Instant::now();
    fire_startup_events_for_screen(&env, ScreenKind::Game);
    started.elapsed()
}

fn apply_post_load_workarounds(env: &WowLuaEnv) -> Duration {
    let started = Instant::now();
    env.apply_post_load_workarounds();
    started.elapsed()
}

fn load_game_ui_addons(env: &WowLuaEnv) -> Duration {
    let started = Instant::now();
    let addons = discover_blizzard_addons_for_screen(&blizzard_ui_dir(), ScreenKind::Game);
    for (name, toc_path) in &addons {
        load_addon(&env.loader_env(), toc_path)
            .unwrap_or_else(|err| panic!("Failed to load Blizzard addon {name}: {err}"));
    }
    started.elapsed()
}
