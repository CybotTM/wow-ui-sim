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
    let env = load_game_ui_until_startup_events();

    LoadedGameUi {
        env,
        startup_elapsed: started.elapsed(),
    }
}

fn load_game_ui_until_startup_events() -> WowLuaEnv {
    let env = load_game_ui_addons();
    fire_startup_events_for_screen(&env, ScreenKind::Game);
    env
}

fn load_game_ui_addons() -> WowLuaEnv {
    let env = new_game_env();
    let addons = discover_blizzard_addons_for_screen(&blizzard_ui_dir(), ScreenKind::Game);
    for (name, toc_path) in &addons {
        load_addon(&env.loader_env(), toc_path)
            .unwrap_or_else(|err| panic!("Failed to load Blizzard addon {name}: {err}"));
    }

    env
}
