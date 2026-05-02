//! Mixin surface probes for `Blizzard_APIDocumentation`.

use crate::common::blizzard_addon_harness::{
    load_blizzard_addon_closure_into_env, with_blizzard_addon_startup_shape,
};
use crate::common::panel_fixtures::{
    blizzard_ui_dir, clear_recorded_lua_errors, recorded_lua_errors,
};

const ROOT: &str = "Blizzard_APIDocumentation";

const API_DOCUMENTATION_MIXIN_METHODS: &[&str] = &[
    "OnLoad",
    "HandleSlashCommand",
    "HandleAPILink",
    "HandleDefaultCommand",
    "HandleCopyAPI",
    "HandleOpenDump",
    "FindAPIByName",
    "GetAPITableByTypeName",
    "OutputUsage",
    "OutputStats",
    "OutputAllSystems",
    "TryHandlingSystemSearchCommand",
    "OutputAPIMatches",
    "OutputAllAPIMatches",
    "OutputAllSystemAPIMatches",
    "OutputAllSystemAPI",
    "AddAllMatches",
    "FindAllAPIMatches",
    "FindSystemByName",
    "AddDocumentationTable",
    "WriteLine",
    "WriteLineF",
    "WriteAllLines",
    "GetIndentString",
    "AddTable",
    "AddFunction",
    "AddEvent",
    "AddField",
    "AddSystem",
];

const BASE_API_MIXIN_METHODS: &[&str] = &[
    "GetType",
    "GetPrettyType",
    "GetLinkHexColor",
    "GetName",
    "GetFullName",
    "GetParentName",
    "GetLoweredParentName",
    "GetLoweredName",
    "GetClipboardString",
    "GenerateAPILink",
    "GetSingleOutputLine",
    "GetDetailedOutputLines",
    "MatchesSearchString",
    "MatchesName",
    "MatchesNameCaseInsenstive",
    "MatchesAnyAPI",
    "MatchesAnyDocumentation",
    "AddDocumentationTags",
    "AddSystemTag",
];

#[test]
fn api_documentation_mixin_exposes_expected_methods() {
    with_blizzard_addon_startup_shape(&[], &[], |env, _loaded| {
        load_api_documentation(env);

        for method_name in API_DOCUMENTATION_MIXIN_METHODS {
            let method_type: String = env
                .eval(&format!(
                    r#"return type(APIDocumentationMixin["{method_name}"])"#
                ))
                .expect("APIDocumentationMixin method type probe must run cleanly");

            assert_eq!(
                "function", method_type,
                "APIDocumentationMixin.{method_name} must be a function"
            );
        }
    });
}

#[test]
fn base_api_mixin_exposes_expected_methods() {
    with_blizzard_addon_startup_shape(&[], &[], |env, _loaded| {
        load_api_documentation(env);

        for method_name in BASE_API_MIXIN_METHODS {
            let method_type: String = env
                .eval(&format!(r#"return type(BaseAPIMixin["{method_name}"])"#))
                .expect("BaseAPIMixin method type probe must run cleanly");

            assert_eq!(
                "function", method_type,
                "BaseAPIMixin.{method_name} must be a function"
            );
        }
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
