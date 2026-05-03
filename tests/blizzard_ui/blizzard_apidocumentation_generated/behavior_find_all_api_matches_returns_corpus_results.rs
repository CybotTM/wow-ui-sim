//! Behavior probes for generated `APIDocumentation:FindAllAPIMatches`.

use crate::common::blizzard_addon_harness::load_blizzard_addon_closure_into_env;
use crate::common::blizzard_addon_harness::new_blizzard_addon_env;
use crate::common::panel_fixtures::{
    blizzard_ui_dir, clear_recorded_lua_errors, load_panel_addons, recorded_lua_errors,
};

const ROOT: &str = "Blizzard_APIDocumentationGenerated";

#[test]
fn find_all_api_matches_returns_full_corpus_results() {
    let env = load_generated_api_documentation();

    let (has_get_time_match, matched_function_count, empty_search_is_nil): (bool, i64, bool) = env
        .eval(
            r#"
            local matches = APIDocumentation:FindAllAPIMatches("GetTime")
            local hasGetTime = false
            local functionCount = matches and #matches.functions or 0

            if matches then
                for _, functionInfo in ipairs(matches.functions) do
                    if functionInfo:GetName() == "GetTime"
                        and functionInfo:GetType() == "function" then
                        hasGetTime = true
                        break
                    end
                end
            end

            local emptyMatches = APIDocumentation:FindAllAPIMatches("ZZZZ_NEVER_EXISTS")

            return hasGetTime,
                   functionCount,
                   emptyMatches == nil
            "#,
        )
        .expect("generated FindAllAPIMatches corpus probe must run cleanly");

    assert!(
        has_get_time_match,
        "FindAllAPIMatches(\"GetTime\") must include the global GetTime function entry"
    );
    assert!(
        matched_function_count > 0,
        "FindAllAPIMatches(\"GetTime\") must return at least one function match"
    );
    assert!(
        empty_search_is_nil,
        "FindAllAPIMatches must return nil when no generated API entries match"
    );
}

fn load_generated_api_documentation() -> wow_ui_sim::lua_api::WowLuaEnv {
    let ui_dir = blizzard_ui_dir();
    let env = new_blizzard_addon_env(&ui_dir);
    load_panel_addons(&env);
    clear_recorded_lua_errors(&env);

    let loaded = load_blizzard_addon_closure_into_env(&env, &ui_dir, &[ROOT], &[]);
    assert!(
        loaded.iter().any(|addon| addon == ROOT),
        "{ROOT} must be included in the loaded addon closure; loaded={loaded:?}"
    );

    let errors = recorded_lua_errors(&env);
    assert!(
        errors.is_empty(),
        "{ROOT} must load without recorded Lua errors:\n  {}",
        errors.join("\n  ")
    );

    env
}
