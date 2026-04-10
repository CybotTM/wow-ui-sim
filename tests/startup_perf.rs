mod common;
#[path = "perf/base_game.rs"]
mod perf_base_game;
#[path = "perf/game_ui.rs"]
mod perf_game_ui;

use std::time::Duration;

use perf_game_ui::load_timed_game_ui;

const FULL_GAME_STARTUP_BUDGET: Duration = Duration::from_secs(30);

#[test]
fn full_game_startup_stays_under_budget() {
    test_timeout! {
        let loaded_ui = load_timed_game_ui();
        let env = &loaded_ui.env;
        let startup_elapsed = loaded_ui.startup_elapsed;

        let startup_ready: bool = env
            .eval("return UIParent ~= nil and PlayerFrame ~= nil and IsLoggedIn()")
            .unwrap();
        assert!(
            startup_ready,
            "timed startup should produce a settled logged-in game UI"
        );

        eprintln!(
            "full game startup baseline: {:.2?} (budget {:.2?})",
            startup_elapsed,
            FULL_GAME_STARTUP_BUDGET
        );

        assert!(
            startup_elapsed < FULL_GAME_STARTUP_BUDGET,
            "full game startup took {:.2?}, exceeding budget {:.2?}",
            startup_elapsed,
            FULL_GAME_STARTUP_BUDGET
        );
    }
}
