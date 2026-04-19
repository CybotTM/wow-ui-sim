mod common;
#[path = "perf/base_game.rs"]
mod perf_base_game;
#[path = "perf/game_ui.rs"]
mod perf_game_ui;

use std::time::Duration;

use perf_game_ui::load_timed_game_ui;

const FULL_GAME_STARTUP_BUDGET: Duration = Duration::from_secs(20);

#[test]
fn full_game_startup_stays_under_budget() {
    test_timeout! {
        common::with_perf_lock(|| {
            let loaded_ui = load_timed_game_ui();
            let env = &loaded_ui.env;
            let phase_timings = loaded_ui.phase_timings();
            let startup_elapsed = loaded_ui.startup_elapsed;

            let startup_ready: bool = env
                .eval("return UIParent ~= nil and PlayerFrame ~= nil and IsLoggedIn()")
                .unwrap();
            assert!(
                startup_ready,
                "timed startup should produce a settled logged-in game UI"
            );

            eprintln!(
                "full game startup baseline: {:.2?} (addons {:.2?}, post-load {:.2?}, startup events {:.2?}; budget {:.2?})",
                startup_elapsed,
                phase_timings.addon_load_elapsed(),
                phase_timings.post_load_workarounds_elapsed(),
                phase_timings.startup_events_elapsed(),
                FULL_GAME_STARTUP_BUDGET
            );

            assert!(
                startup_elapsed < FULL_GAME_STARTUP_BUDGET,
                "full game startup took {:.2?}, exceeding budget {:.2?}",
                startup_elapsed,
                FULL_GAME_STARTUP_BUDGET
            );
        });
    }
}
