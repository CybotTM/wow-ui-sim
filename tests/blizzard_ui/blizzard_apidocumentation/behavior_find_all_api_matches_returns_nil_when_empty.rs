//! Behavior probes for `APIDocumentation:FindAllAPIMatches`.

use crate::common::blizzard_addon_harness::{
    load_blizzard_addon_closure_into_env, with_blizzard_addon_startup_shape,
};
use crate::common::panel_fixtures::{
    blizzard_ui_dir, clear_recorded_lua_errors, recorded_lua_errors,
};

const ROOT: &str = "Blizzard_APIDocumentation";

#[test]
fn find_all_api_matches_returns_nil_until_a_function_matches() {
    with_blizzard_addon_startup_shape(&[], &[], |env, _loaded| {
        load_api_documentation(env);

        let (
            empty_search_is_nil,
            function_match_count,
            matched_function_name,
            table_match_count,
            event_match_count,
            system_match_count,
        ): (bool, i64, String, i64, i64, i64) = env
            .eval(
                r#"
                local emptyMatches = APIDocumentation:FindAllAPIMatches("Anything")

                APIDocumentation:AddDocumentationTable({
                    Name = "SearchSystem",
                    Type = "System",
                    Namespace = "",
                    Tables = {},
                    Functions = {
                        { Name = "NeedleFunction", Type = "Function", Arguments = {}, Returns = {} },
                    },
                    Events = {},
                })

                local matches = APIDocumentation:FindAllAPIMatches("needlefunction")

                return emptyMatches == nil,
                       #matches.functions,
                       matches.functions[1]:GetName(),
                       #matches.tables,
                       #matches.events,
                       #matches.systems
                "#,
            )
            .expect("FindAllAPIMatches probe must run cleanly");

        assert!(
            empty_search_is_nil,
            "FindAllAPIMatches must return nil when every match bucket is empty"
        );
        assert_eq!(
            1, function_match_count,
            "one seeded function must match by its case-insensitive name"
        );
        assert_eq!(
            "NeedleFunction", matched_function_name,
            "the returned function match must be the seeded payload"
        );
        assert_eq!(0, table_match_count, "table matches must stay empty");
        assert_eq!(0, event_match_count, "event matches must stay empty");
        assert_eq!(0, system_match_count, "system matches must stay empty");
    });
}

fn load_api_documentation(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    clear_recorded_lua_errors(env);
    let ui_dir = blizzard_ui_dir();
    let loaded = load_blizzard_addon_closure_into_env(env, &ui_dir, &[ROOT], &[]);

    assert!(
        loaded.iter().any(|addon| addon == ROOT),
        "{ROOT} must be included in the loaded addon closure; loaded={loaded:?}"
    );

    let errors = recorded_lua_errors(env);
    assert!(
        errors.is_empty(),
        "{ROOT} must settle without recorded Lua errors:\n  {}",
        errors.join("\n  ")
    );
}
