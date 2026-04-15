use std::time::{Duration, Instant};

use wow_ui_sim::loader::{
    LoadTiming, discover_blizzard_addons_for_screen, load_addon, load_addon_with_saved_vars,
};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::saved_variables::SavedVariablesManager;
use wow_ui_sim::screen::ScreenKind;

use crate::perf_base_game::{blizzard_ui_dir, new_game_env};

#[derive(Clone, Debug)]
pub struct PerAddonLoadTiming {
    pub name: String,
    pub total_time: Duration,
}

pub struct LoadedAddonPhases {
    pub env: WowLuaEnv,
    pub addon_count: usize,
    pub addon_elapsed: Duration,
    pub addon_timing: LoadTiming,
    pub per_addon_timings: Vec<PerAddonLoadTiming>,
}

pub fn load_timed_game_addons_without_saved_vars() -> LoadedAddonPhases {
    load_timed_game_addons(false)
}

pub fn load_timed_game_addons_with_saved_vars() -> LoadedAddonPhases {
    load_timed_game_addons(true)
}

fn load_timed_game_addons(with_saved_vars: bool) -> LoadedAddonPhases {
    let started = Instant::now();
    let env = new_game_env();
    let addons = discover_blizzard_addons_for_screen(&blizzard_ui_dir(), ScreenKind::Game);
    let mut saved_vars = with_saved_vars.then(SavedVariablesManager::new);
    let mut addon_timing = LoadTiming::default();
    let mut per_addon_timings = Vec::with_capacity(addons.len());

    for (name, toc_path) in &addons {
        let result = if with_saved_vars {
            load_addon_with_saved_vars(
                &env.loader_env(),
                toc_path,
                saved_vars
                    .as_mut()
                    .expect("saved vars manager should exist when enabled"),
            )
        } else {
            load_addon(&env.loader_env(), toc_path)
        }
        .unwrap_or_else(|err| panic!("Failed to load Blizzard addon {name}: {err}"));
        addon_timing.accumulate(&result.timing);
        per_addon_timings.push(PerAddonLoadTiming {
            name: name.clone(),
            total_time: result.timing.total(),
        });
    }

    LoadedAddonPhases {
        env,
        addon_count: addons.len(),
        addon_elapsed: started.elapsed(),
        addon_timing,
        per_addon_timings,
    }
}
