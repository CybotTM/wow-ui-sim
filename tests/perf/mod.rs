use std::path::PathBuf;
use std::time::{Duration, Instant};

use wow_ui_sim::loader::{discover_blizzard_addons_for_screen, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::startup::settle_headless_startup;

pub struct PerfCase<'a, T> {
    name: &'a str,
    run: Box<dyn FnMut(&WowLuaEnv) -> T + 'a>,
}

impl<'a, T> PerfCase<'a, T> {
    pub fn new(name: &'a str, run: impl FnMut(&WowLuaEnv) -> T + 'a) -> Self {
        Self {
            name,
            run: Box::new(run),
        }
    }
}

pub struct PerfCaseResult<T> {
    pub name: String,
    pub elapsed: Duration,
    pub output: T,
}

pub fn run_game_ui_cases<T>(mut cases: Vec<PerfCase<'_, T>>) -> Vec<PerfCaseResult<T>> {
    let env = load_settled_game_ui();
    let mut results = Vec::with_capacity(cases.len());

    for case in &mut cases {
        let started = Instant::now();
        let output = (case.run)(&env);

        results.push(PerfCaseResult {
            name: case.name.to_string(),
            elapsed: started.elapsed(),
            output,
        });
    }

    results
}

fn load_settled_game_ui() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);
    env.set_screen_mode(ScreenKind::Game);

    let ui = blizzard_ui_dir();
    {
        let mut state = env.state().borrow_mut();
        state.addon_base_paths = vec![ui.clone()];
    }

    let addons = discover_blizzard_addons_for_screen(&ui, ScreenKind::Game);
    for (name, toc_path) in &addons {
        load_addon(&env.loader_env(), toc_path)
            .unwrap_or_else(|err| panic!("Failed to load Blizzard addon {name}: {err}"));
    }

    env.apply_post_load_workarounds();
    settle_headless_startup(&env);
    env
}

fn blizzard_ui_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI")
}
