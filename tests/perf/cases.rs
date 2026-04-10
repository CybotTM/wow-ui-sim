use std::time::{Duration, Instant};

use wow_ui_sim::lua_api::WowLuaEnv;

use crate::perf_game_ui::load_timed_game_ui;

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
    let loaded_ui = load_timed_game_ui();
    assert!(
        loaded_ui.startup_elapsed.as_nanos() > 0,
        "shared perf UI startup timing should be captured"
    );
    let env = &loaded_ui.env;
    let mut results = Vec::with_capacity(cases.len());

    for case in &mut cases {
        let started = Instant::now();
        let output = (case.run)(env);

        results.push(PerfCaseResult {
            name: case.name.to_string(),
            elapsed: started.elapsed(),
            output,
        });
    }

    results
}
