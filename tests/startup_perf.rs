mod common;
#[path = "perf/base_game.rs"]
mod perf_base_game;
#[path = "perf/game_ui.rs"]
mod perf_game_ui;
#[path = "perf/template_create.rs"]
mod perf_template_create;

use std::time::Duration;

use perf_game_ui::load_timed_game_ui;
use perf_template_create::{TemplateBench, measure_action_bar_button_family, measure_template_create};

const FULL_GAME_STARTUP_BUDGET: Duration = Duration::from_secs(30);

// Per-template budgets for creating N instances from a loaded game UI.
// Time budgets: ~5-8x measured baseline to absorb CI variance + debug builds.
// Frame-count budgets: expected total frames (parents + children) created by
// N template instances. Changes here indicate template structure changed.
const ACTION_BUTTON_SPELLFX_BUDGET: Duration = Duration::from_millis(400);
const ACTION_BUTTON_SPELLFX_COUNT: usize = 10;
const ACTION_BUTTON_SPELLFX_EXPECTED_FRAMES: usize = 350;

const MINIMAL_SCROLLBAR_BUDGET: Duration = Duration::from_millis(400);
const MINIMAL_SCROLLBAR_COUNT: usize = 10;
const MINIMAL_SCROLLBAR_EXPECTED_FRAMES: usize = 130;

const ACTION_BAR_BUTTON_BUDGET: Duration = Duration::from_millis(1500);
const ACTION_BAR_BUTTON_COUNT: usize = 12;
const ACTION_BAR_BUTTON_EXPECTED_FRAMES: usize = 805;

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

#[test]
fn action_button_spellfx_template_stays_under_budget() {
    test_timeout! {
        let loaded = load_timed_game_ui();
        let result = measure_template_create(&loaded.env, &TemplateBench {
            template: "ActionButtonSpellFXTemplate",
            widget_type: "CheckButton",
            count: ACTION_BUTTON_SPELLFX_COUNT,
        });

        eprintln!(
            "{} x{}: {:.2?}, {} frames (budget {:.2?})",
            result.template, result.count, result.elapsed,
            result.frames_created, ACTION_BUTTON_SPELLFX_BUDGET,
        );

        assert_eq!(
            result.frames_created, ACTION_BUTTON_SPELLFX_EXPECTED_FRAMES,
            "{} x{} created {} frames, expected {}",
            result.template, result.count, result.frames_created,
            ACTION_BUTTON_SPELLFX_EXPECTED_FRAMES,
        );
        assert!(
            result.elapsed < ACTION_BUTTON_SPELLFX_BUDGET,
            "{} x{} took {:.2?}, exceeding budget {:.2?}",
            result.template, result.count, result.elapsed, ACTION_BUTTON_SPELLFX_BUDGET,
        );
    }
}

#[test]
fn minimal_scrollbar_template_stays_under_budget() {
    test_timeout! {
        let loaded = load_timed_game_ui();
        let result = measure_template_create(&loaded.env, &TemplateBench {
            template: "MinimalScrollBar",
            widget_type: "EventFrame",
            count: MINIMAL_SCROLLBAR_COUNT,
        });

        eprintln!(
            "{} x{}: {:.2?}, {} frames (budget {:.2?})",
            result.template, result.count, result.elapsed,
            result.frames_created, MINIMAL_SCROLLBAR_BUDGET,
        );

        assert_eq!(
            result.frames_created, MINIMAL_SCROLLBAR_EXPECTED_FRAMES,
            "{} x{} created {} frames, expected {}",
            result.template, result.count, result.frames_created,
            MINIMAL_SCROLLBAR_EXPECTED_FRAMES,
        );
        assert!(
            result.elapsed < MINIMAL_SCROLLBAR_BUDGET,
            "{} x{} took {:.2?}, exceeding budget {:.2?}",
            result.template, result.count, result.elapsed, MINIMAL_SCROLLBAR_BUDGET,
        );
    }
}

#[test]
fn action_bar_button_family_stays_under_budget() {
    test_timeout! {
        let loaded = load_timed_game_ui();
        let result = measure_action_bar_button_family(&loaded.env, ACTION_BAR_BUTTON_COUNT);

        eprintln!(
            "action-bar button family x{}: {:.2?}, {} frames (budget {:.2?})",
            result.count, result.elapsed,
            result.frames_created, ACTION_BAR_BUTTON_BUDGET,
        );

        assert_eq!(
            result.frames_created, ACTION_BAR_BUTTON_EXPECTED_FRAMES,
            "action-bar button family x{} created {} frames, expected {}",
            result.count, result.frames_created, ACTION_BAR_BUTTON_EXPECTED_FRAMES,
        );
        assert!(
            result.elapsed < ACTION_BAR_BUTTON_BUDGET,
            "action-bar button family x{} took {:.2?}, exceeding budget {:.2?}",
            result.count, result.elapsed, ACTION_BAR_BUTTON_BUDGET,
        );
    }
}
