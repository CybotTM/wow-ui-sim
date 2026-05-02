//! Behavior probes for APIDocumentation API-kind table routing.

use crate::common::blizzard_addon_harness::{
    load_blizzard_addon_closure_into_env, with_blizzard_addon_startup_shape,
};
use crate::common::panel_fixtures::{
    blizzard_ui_dir, clear_recorded_lua_errors, recorded_lua_errors,
};

const ROOT: &str = "Blizzard_APIDocumentation";

#[test]
fn get_api_table_by_type_name_returns_matching_registry_tables() {
    with_blizzard_addon_startup_shape(&[], &[], |env, _loaded| {
        load_api_documentation(env);

        let (
            table_matches,
            function_matches,
            event_matches,
            system_matches,
            unknown_is_nil,
        ): (bool, bool, bool, bool, bool) = env
            .eval(
                r#"
                return APIDocumentation:GetAPITableByTypeName("table") == APIDocumentation.tables,
                       APIDocumentation:GetAPITableByTypeName("function") == APIDocumentation.functions,
                       APIDocumentation:GetAPITableByTypeName("event") == APIDocumentation.events,
                       APIDocumentation:GetAPITableByTypeName("system") == APIDocumentation.systems,
                       APIDocumentation:GetAPITableByTypeName("unknown") == nil
                "#,
            )
            .expect("APIDocumentation API-kind table routing probe must run cleanly");

        assert!(
            table_matches,
            "`table` kind must return APIDocumentation.tables by identity"
        );
        assert!(
            function_matches,
            "`function` kind must return APIDocumentation.functions by identity"
        );
        assert!(
            event_matches,
            "`event` kind must return APIDocumentation.events by identity"
        );
        assert!(
            system_matches,
            "`system` kind must return APIDocumentation.systems by identity"
        );
        assert!(unknown_is_nil, "unknown API kind must return nil");
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
