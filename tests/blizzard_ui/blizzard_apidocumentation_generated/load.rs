//! Load smoke for `Blizzard_APIDocumentationGenerated`.

use crate::common::blizzard_addon_harness::new_blizzard_addon_env;
use crate::common::panel_fixtures::{
    blizzard_ui_dir, clear_recorded_lua_errors, load_panel_addons, recorded_lua_errors,
};
use wow_ui_sim::loader::{discover_blizzard_addon_closure_for_screen_with_overrides, load_addon};
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::startup::settle_headless_startup;

const ROOT: &str = "Blizzard_APIDocumentationGenerated";
const DEPENDENCY: &str = "Blizzard_APIDocumentation";

#[test]
fn generated_api_documentation_loads_after_core_api_documentation() {
    let ui_dir = blizzard_ui_dir();
    let env = new_blizzard_addon_env(&ui_dir);
    load_panel_addons(&env);
    clear_recorded_lua_errors(&env);

    let loaded = load_generated_documentation_closure(&env, &ui_dir);
    ensure_player_frame_stub(&env);
    env.apply_post_load_workarounds();
    settle_headless_startup(&env);

    assert_dependency_loaded_before_root(&loaded);
    assert_first_documentation_file_saw_loaded_dependency(&env);

    let errors = recorded_lua_errors(&env);
    assert!(
        errors.is_empty(),
        "{ROOT} must settle without recorded Lua errors:\n  {}",
        errors.join("\n  ")
    );
}

fn ensure_player_frame_stub(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    env.exec(
        r#"
        if not PlayerFrame then
            PlayerFrame = CreateFrame("Frame", "PlayerFrame", UIParent)
        end
        PlayerFrame.unit = "player"
        "#,
    )
    .expect("PlayerFrame startup stub must install");
}

fn load_generated_documentation_closure(
    env: &wow_ui_sim::lua_api::WowLuaEnv,
    ui_dir: &std::path::Path,
) -> Vec<String> {
    let closure = discover_blizzard_addon_closure_for_screen_with_overrides(
        ui_dir,
        ScreenKind::Game,
        &[ROOT],
        &[],
    );
    let mut loaded = Vec::new();
    for (name, toc_path) in closure {
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
        _G.__apiDocumentationGeneratedTrace = {
            firstDataFileSawDependencyLoaded = nil,
            firstDataFileName = nil,
            addDocumentationTableCalls = 0,
        }

        local originalAddDocumentationTable = APIDocumentation.AddDocumentationTable
        APIDocumentation.AddDocumentationTable = function(self, documentationInfo)
            local trace = _G.__apiDocumentationGeneratedTrace
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
    .expect("APIDocumentation AddDocumentationTable trace probe must install");
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

fn assert_first_documentation_file_saw_loaded_dependency(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let (call_count, first_name, dependency_loaded): (i32, String, bool) = env
        .eval(
            r#"
            local trace = _G.__apiDocumentationGeneratedTrace
            return trace.addDocumentationTableCalls,
                   trace.firstDataFileName or "",
                   trace.firstDataFileSawDependencyLoaded == true
            "#,
        )
        .expect("generated documentation trace must be readable");

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
        "{DEPENDENCY} must already be marked loaded when the first data file calls \
         APIDocumentation:AddDocumentationTable"
    );
}
