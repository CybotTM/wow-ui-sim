//! Behavior probes for generated APIDocumentation dependency ordering.

use crate::common::blizzard_addon_harness::new_blizzard_addon_env;
use crate::common::panel_fixtures::{
    blizzard_ui_dir, clear_recorded_lua_errors, load_panel_addons, recorded_lua_errors,
};
use wow_ui_sim::loader::{discover_blizzard_addon_closure_for_screen_with_overrides, load_addon};
use wow_ui_sim::screen::ScreenKind;

const ROOT: &str = "Blizzard_APIDocumentationGenerated";
const DEPENDENCY: &str = "Blizzard_APIDocumentation";

#[test]
fn generated_data_files_run_after_api_documentation_dependency_is_loaded() {
    let ui_dir = blizzard_ui_dir();
    let env = new_blizzard_addon_env(&ui_dir);
    load_panel_addons(&env);
    clear_recorded_lua_errors(&env);
    assert_addons_start_unloaded(&env);

    let loaded = load_generated_documentation_closure_with_probe(&env, &ui_dir);

    assert_dependency_loaded_before_root(&loaded);
    assert_first_data_file_saw_loaded_dependency(&env);

    let errors = recorded_lua_errors(&env);
    assert!(
        errors.is_empty(),
        "{ROOT} must load without recorded Lua errors:\n  {}",
        errors.join("\n  ")
    );
}

fn assert_addons_start_unloaded(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let (dependency_loaded, root_loaded): (bool, bool) = env
        .eval(
            r#"
            return C_AddOns.IsAddOnLoaded("Blizzard_APIDocumentation") == true,
                   C_AddOns.IsAddOnLoaded("Blizzard_APIDocumentationGenerated") == true
            "#,
        )
        .expect("initial generated APIDocumentation loaded-state probe must run cleanly");

    assert!(
        !dependency_loaded,
        "{DEPENDENCY} must start unloaded in the fresh generated documentation harness"
    );
    assert!(
        !root_loaded,
        "{ROOT} must start unloaded in the fresh generated documentation harness"
    );
}

fn load_generated_documentation_closure_with_probe(
    env: &wow_ui_sim::lua_api::WowLuaEnv,
    ui_dir: &std::path::Path,
) -> Vec<String> {
    let mut loaded = Vec::new();
    for (name, toc_path) in discover_blizzard_addon_closure_for_screen_with_overrides(
        ui_dir,
        ScreenKind::Game,
        &[ROOT],
        &[],
    ) {
        if name == ROOT {
            install_documentation_trace_probe(env);
        }
        load_addon(&env.loader_env(), &toc_path)
            .unwrap_or_else(|error| panic!("{name} should load cleanly: {error}"));
        loaded.push(name);
    }
    loaded
}

fn install_documentation_trace_probe(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    env.exec(
        r#"
        _G.__apiDocumentationGeneratedDependencyTrace = {
            firstDataFileSawDependencyLoaded = nil,
            firstDataFileName = nil,
            addDocumentationTableCalls = 0,
        }

        local originalAddDocumentationTable = APIDocumentation.AddDocumentationTable
        APIDocumentation.AddDocumentationTable = function(self, documentationInfo)
            local trace = _G.__apiDocumentationGeneratedDependencyTrace
            trace.addDocumentationTableCalls = trace.addDocumentationTableCalls + 1
            if trace.firstDataFileSawDependencyLoaded == nil then
                trace.firstDataFileSawDependencyLoaded =
                    C_AddOns.IsAddOnLoaded("Blizzard_APIDocumentation") == true
                trace.firstDataFileName =
                    type(documentationInfo) == "table" and documentationInfo.Name or nil
            end
            return originalAddDocumentationTable(self, documentationInfo)
        end
        "#,
    )
    .expect("APIDocumentation AddDocumentationTable dependency trace probe must install");
}

fn assert_dependency_loaded_before_root(loaded: &[String]) {
    let dependency_index = loaded
        .iter()
        .position(|name| name == DEPENDENCY)
        .expect("generated documentation closure must include Blizzard_APIDocumentation");
    let root_index = loaded
        .iter()
        .position(|name| name == ROOT)
        .expect("generated documentation closure must include its root addon");

    assert!(
        dependency_index < root_index,
        "{DEPENDENCY} must load before {ROOT}; loaded={loaded:?}"
    );
}

fn assert_first_data_file_saw_loaded_dependency(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let (call_count, first_name, dependency_loaded): (i64, String, bool) = env
        .eval(
            r#"
            local trace = _G.__apiDocumentationGeneratedDependencyTrace
            return trace.addDocumentationTableCalls,
                   trace.firstDataFileName or "",
                   trace.firstDataFileSawDependencyLoaded == true
            "#,
        )
        .expect("generated documentation dependency trace must be readable");

    assert!(
        call_count > 0,
        "{ROOT} must execute at least one generated *Documentation.lua data file"
    );
    assert_eq!(
        "AbbreviateConfigAPI", first_name,
        "first generated documentation file should be AbbreviateConfigAPIDocumentation.lua"
    );
    assert!(
        dependency_loaded,
        "{DEPENDENCY} must already be marked loaded before the first generated data file registers"
    );
}
