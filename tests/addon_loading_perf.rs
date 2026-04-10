mod common;
#[path = "perf/addon_loading.rs"]
mod perf_addon_loading;
#[path = "perf/base_game.rs"]
mod perf_base_game;

use std::time::Duration;

use perf_addon_loading::load_timed_game_addons_with_saved_vars;

const ADDON_LOADING_BUDGET: Duration = Duration::from_secs(25);

#[test]
fn blizzard_addon_loading_reports_phase_breakdown_under_budget() {
    test_timeout! {
        let loaded = load_timed_game_addons_with_saved_vars();
        let env = &loaded.env;
        let timing = &loaded.addon_timing;

        let addon_surface_ready: bool = env
            .eval("return UIParent ~= nil and PlayerFrame ~= nil and type(IsLoggedIn) == 'function'")
            .unwrap();
        assert!(
            addon_surface_ready,
            "timed addon loading should produce a real Blizzard game UI surface"
        );
        assert!(loaded.addon_count > 0, "expected Blizzard addons to be discovered");
        assert!(
            timing.xml_parse_time > Duration::ZERO,
            "xml parse timing should be non-zero across Blizzard addons"
        );
        assert!(
            timing.lua_compile_time > Duration::ZERO,
            "lua compile timing should be non-zero across Blizzard addons"
        );
        assert!(
            timing.lua_call_time > Duration::ZERO,
            "lua call timing should be non-zero across Blizzard addons"
        );
        assert!(
            timing.saved_vars_time > Duration::ZERO,
            "saved variables timing should be non-zero when loading through SavedVariablesManager"
        );
        assert_eq!(
            timing.lua_exec_time,
            timing.lua_compile_time + timing.lua_call_time,
            "lua exec timing should equal the sum of compile and call phases"
        );

        eprintln!(
            "blizzard addon loading baseline: {:.2?} total across {} addons (xml parse {:.2?}, lua compile {:.2?}, lua call {:.2?}, saved vars {:.2?}; budget {:.2?})",
            loaded.addon_elapsed,
            loaded.addon_count,
            timing.xml_parse_time,
            timing.lua_compile_time,
            timing.lua_call_time,
            timing.saved_vars_time,
            ADDON_LOADING_BUDGET
        );

        assert!(
            loaded.addon_elapsed < ADDON_LOADING_BUDGET,
            "blizzard addon loading took {:.2?}, exceeding budget {:.2?}",
            loaded.addon_elapsed,
            ADDON_LOADING_BUDGET
        );
    }
}
