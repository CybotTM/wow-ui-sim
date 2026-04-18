mod common;
#[path = "perf/base_game.rs"]
mod perf_base_game;
#[path = "perf/game_ui.rs"]
mod perf_game_ui;
#[path = "perf/template_create.rs"]
mod perf_template_create;

use std::time::Duration;

use perf_game_ui::load_timed_game_ui;
use perf_template_create::{
    ACTION_BUTTON_SPELLFX_BENCH, ACTION_BUTTON_TEMPLATE_BENCH, MINIMAL_SCROLLBAR_BENCH,
    TemplateBenchResult, measure_profiled_startup_hot_paths,
};

const FULL_GAME_STARTUP_BUDGET: Duration = Duration::from_secs(30);

// Per-template budgets for creating N instances from a loaded game UI.
// Time budgets: ~5-8x measured baseline to absorb CI variance + debug builds.
// Frame-count budgets: expected total frames (parents + children) created by
// N template instances. Changes here indicate template structure changed.
const ACTION_BUTTON_SPELLFX_BUDGET: Duration = Duration::from_millis(400);
const ACTION_BUTTON_SPELLFX_EXPECTED_FRAMES: usize = 430;

const ACTION_BUTTON_TEMPLATE_BUDGET: Duration = Duration::from_millis(500);
const ACTION_BUTTON_TEMPLATE_EXPECTED_FRAMES: usize = 660;

const MINIMAL_SCROLLBAR_BUDGET: Duration = Duration::from_millis(400);
const MINIMAL_SCROLLBAR_EXPECTED_FRAMES: usize = 130;

const ACTION_BAR_BUTTON_BUDGET: Duration = Duration::from_millis(1500);
const ACTION_BAR_BUTTON_COUNT: usize = 12;
const ACTION_BAR_BUTTON_EXPECTED_FRAMES: usize = 805;

fn expected_hot_path_metrics(template: &str) -> (usize, usize, Duration) {
    match template {
        "ActionButtonSpellFXTemplate" => (
            ACTION_BUTTON_SPELLFX_BENCH.count,
            ACTION_BUTTON_SPELLFX_EXPECTED_FRAMES,
            ACTION_BUTTON_SPELLFX_BUDGET,
        ),
        "ActionButtonTemplate" => (
            ACTION_BUTTON_TEMPLATE_BENCH.count,
            ACTION_BUTTON_TEMPLATE_EXPECTED_FRAMES,
            ACTION_BUTTON_TEMPLATE_BUDGET,
        ),
        "MinimalScrollBar" => (
            MINIMAL_SCROLLBAR_BENCH.count,
            MINIMAL_SCROLLBAR_EXPECTED_FRAMES,
            MINIMAL_SCROLLBAR_BUDGET,
        ),
        "ActionBarButtonTemplate" => (
            ACTION_BAR_BUTTON_COUNT,
            ACTION_BAR_BUTTON_EXPECTED_FRAMES,
            ACTION_BAR_BUTTON_BUDGET,
        ),
        other => panic!("unexpected startup hot path result: {other}"),
    }
}

fn assert_startup_hot_path_result(result: &TemplateBenchResult) {
    let (expected_count, expected_frames, budget) = expected_hot_path_metrics(result.template);
    eprintln!(
        "{} x{}: {:.2?}, {} frames (budget {:.2?})",
        result.template, result.count, result.elapsed, result.frames_created, budget,
    );

    assert_eq!(
        result.count, expected_count,
        "{} measured {} instances, expected {}",
        result.template, result.count, expected_count,
    );
    assert_eq!(
        result.frames_created, expected_frames,
        "{} x{} created {} frames, expected {}",
        result.template, result.count, result.frames_created, expected_frames,
    );
    assert!(
        result.elapsed < budget,
        "{} x{} took {:.2?}, exceeding budget {:.2?}",
        result.template,
        result.count,
        result.elapsed,
        budget,
    );
}

#[test]
fn full_game_startup_stays_under_budget() {
    test_timeout! {
        common::with_perf_lock(|| {
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
        });
    }
}

#[test]
fn profiled_startup_hot_paths_stay_under_budgets() {
    test_timeout! {
        common::with_perf_lock(|| {
            let loaded = load_timed_game_ui();
            let results = measure_profiled_startup_hot_paths(&loaded.env, ACTION_BAR_BUTTON_COUNT);
            let hot_path_names = results
                .iter()
                .map(|result| result.template)
                .collect::<Vec<_>>();

            assert_eq!(
                hot_path_names,
                vec![
                    "ActionButtonSpellFXTemplate",
                    "ActionButtonTemplate",
                    "MinimalScrollBar",
                    "ActionBarButtonTemplate",
                ],
                "startup hot path harness should cover every profiled template family",
            );
            for result in &results {
                assert_startup_hot_path_result(result);
            }
        });
    }
}
