use std::time::{Duration, Instant};

use wow_ui_sim::loader::{discover_blizzard_addons_for_screen, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::startup::settle_headless_startup;

use crate::perf_base_game::{blizzard_ui_dir, new_game_env};

pub struct LoadedGameUi {
    pub env: WowLuaEnv,
    pub startup_elapsed: Duration,
}

pub fn load_timed_game_ui() -> LoadedGameUi {
    let started = Instant::now();
    let env = load_settled_game_ui();

    LoadedGameUi {
        env,
        startup_elapsed: started.elapsed(),
    }
}

fn load_settled_game_ui() -> WowLuaEnv {
    let env = new_game_env();
    let addons = discover_blizzard_addons_for_screen(&blizzard_ui_dir(), ScreenKind::Game);
    for (name, toc_path) in &addons {
        load_addon(&env.loader_env(), toc_path)
            .unwrap_or_else(|err| panic!("Failed to load Blizzard addon {name}: {err}"));
    }

    env.apply_post_load_workarounds();
    settle_headless_startup(&env);
    env
}
