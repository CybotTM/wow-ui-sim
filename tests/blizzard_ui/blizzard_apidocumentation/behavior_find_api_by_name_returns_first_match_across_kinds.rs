//! Behavior probes for APIDocumentation exact-name lookup.

use crate::common::blizzard_addon_harness::{
    load_blizzard_addon_closure_into_env, with_blizzard_addon_startup_shape,
};
use crate::common::panel_fixtures::{
    blizzard_ui_dir, clear_recorded_lua_errors, recorded_lua_errors,
};

const ROOT: &str = "Blizzard_APIDocumentation";

#[test]
fn find_api_by_name_uses_requested_kind_when_names_overlap() {
    with_blizzard_addon_startup_shape(&[], &[], |env, _loaded| {
        load_api_documentation(env);

        let (table_type, function_type, untyped_lookup_is_nil): (String, String, bool) = env
            .eval(
                r#"
                APIDocumentation:AddDocumentationTable({
                    Name = "LookupSystem",
                    Type = "System",
                    Namespace = "",
                    Tables = {
                        { Name = "Foo", Type = "Structure", Fields = {} },
                    },
                    Functions = {
                        { Name = "Foo", Type = "Function", Arguments = {}, Returns = {} },
                    },
                    Events = {},
                })

                local tableInfo = APIDocumentation:FindAPIByName("table", "Foo")
                local functionInfo = APIDocumentation:FindAPIByName("function", "Foo")
                local untypedLookup = APIDocumentation:FindAPIByName("Foo")

                return tableInfo:GetType(),
                       functionInfo:GetType(),
                       untypedLookup == nil
                "#,
            )
            .expect("APIDocumentation exact-name lookup probe must run cleanly");

        assert_eq!(
            "table", table_type,
            "table lookup must return the Foo table despite a same-named function"
        );
        assert_eq!(
            "function", function_type,
            "function lookup must return the Foo function despite a same-named table"
        );
        assert!(
            untyped_lookup_is_nil,
            "FindAPIByName requires apiType as its first argument in Blizzard's API"
        );
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
