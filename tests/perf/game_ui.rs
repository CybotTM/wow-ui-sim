use std::time::{Duration, Instant};

use wow_ui_sim::loader::{discover_blizzard_addons_for_screen, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::startup::fire_startup_events_for_screen;

use crate::perf_base_game::{blizzard_ui_dir, new_game_env};

pub struct LoadedGameUi {
    pub env: WowLuaEnv,
    pub startup_elapsed: Duration,
}

pub fn load_timed_game_ui() -> LoadedGameUi {
    let started = Instant::now();
    let env = new_game_env();
    load_game_ui_addons(&env);
    apply_post_load_workarounds(&env);
    fire_startup_events(&env);

    LoadedGameUi {
        env,
        startup_elapsed: started.elapsed(),
    }
}

pub fn fire_startup_events(env: &WowLuaEnv) -> Duration {
    let started = Instant::now();
    fire_startup_events_for_screen(&env, ScreenKind::Game);
    started.elapsed()
}

pub fn apply_post_load_workarounds(env: &WowLuaEnv) -> Duration {
    let started = Instant::now();
    env.apply_post_load_workarounds();
    started.elapsed()
}

pub fn load_game_ui_addons(env: &WowLuaEnv) -> Duration {
    let started = Instant::now();
    let addons = discover_blizzard_addons_for_screen(&blizzard_ui_dir(), ScreenKind::Game);
    for (name, toc_path) in &addons {
        load_addon(&env.loader_env(), toc_path)
            .unwrap_or_else(|err| panic!("Failed to load Blizzard addon {name}: {err}"));
    }
    started.elapsed()
}
