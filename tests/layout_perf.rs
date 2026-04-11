mod common;
#[path = "perf/base_game.rs"]
mod perf_base_game;
#[path = "perf/game_ui.rs"]
mod perf_game_ui;
#[path = "perf/layout.rs"]
mod perf_layout;

use std::time::Duration;

use perf_game_ui::load_timed_game_ui;
use perf_layout::measure_full_root_layout_pass;

const FULL_ROOT_LAYOUT_BUDGET: Duration = Duration::from_millis(500);

#[test]
fn full_root_layout_pass_stays_under_budget() {
    test_timeout! {
        let loaded_ui = load_timed_game_ui();
        let env = &loaded_ui.env;
        assert!(
            loaded_ui.startup_elapsed.as_nanos() > 0,
            "shared perf UI startup timing should be captured"
        );

        let layout_elapsed = measure_full_root_layout_pass(env);
        eprintln!(
            "full root layout baseline: {:.2?} (budget {:.2?})",
            layout_elapsed,
            FULL_ROOT_LAYOUT_BUDGET
        );

        assert!(
            layout_elapsed < FULL_ROOT_LAYOUT_BUDGET,
            "full root layout took {:.2?}, exceeding budget {:.2?}",
            layout_elapsed,
            FULL_ROOT_LAYOUT_BUDGET
        );
    }
}
