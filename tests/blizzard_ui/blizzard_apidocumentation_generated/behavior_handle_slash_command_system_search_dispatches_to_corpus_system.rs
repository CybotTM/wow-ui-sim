//! Behavior probes for generated APIDocumentation system-scoped search.

use crate::common::blizzard_addon_harness::load_blizzard_addon_closure_into_env;
use crate::common::blizzard_addon_harness::new_blizzard_addon_env;
use crate::common::panel_fixtures::{
    blizzard_ui_dir, clear_recorded_lua_errors, load_panel_addons, recorded_lua_errors,
};

const ROOT: &str = "Blizzard_APIDocumentationGenerated";

#[test]
fn system_search_slash_command_dispatches_to_generated_map_system() {
    let env = load_generated_api_documentation();

    let (
        found_match_header,
        found_function_line,
        found_global_fallthrough,
        function_details_have_returns,
    ): (bool, bool, bool, bool) = env
        .eval(
            r#"
            DEFAULT_CHAT_FRAME:Clear()
            APIDocumentation:HandleSlashCommand("C_Map search GetMapInfo")

            local foundMatchHeader = false
            local foundFunctionLine = false
            local foundGlobalFallthrough = false
            for index = 1, DEFAULT_CHAT_FRAME:GetNumMessages() do
                local text = DEFAULT_CHAT_FRAME:GetMessageInfo(index)
                foundMatchHeader = foundMatchHeader
                    or text:match('^Found %d+ API that matches "GetMapInfo"$') ~= nil
                foundFunctionLine = foundFunctionLine
                    or text:find("C_Map.GetMapInfo", 1, true) ~= nil
                foundGlobalFallthrough = foundGlobalFallthrough
                    or text:find("No system found", 1, true) ~= nil
                    or text:find("No API found that matches", 1, true) ~= nil
            end

            local functionInfo =
                APIDocumentation:FindAPIByName("function", "GetMapInfo", "MapUI")
            local detailsHaveReturns = false
            for _, line in ipairs(functionInfo:GetDetailedOutputLines()) do
                detailsHaveReturns = detailsHaveReturns or line:find("Returns", 1, true) ~= nil
            end

            return foundMatchHeader,
                   foundFunctionLine,
                   foundGlobalFallthrough,
                   detailsHaveReturns
            "#,
        )
        .expect("generated APIDocumentation system search probe must run cleanly");

    assert!(
        found_match_header,
        "system-scoped search must write the scoped match-count header"
    );
    assert!(
        found_function_line,
        "system-scoped search must write the generated C_Map.GetMapInfo function line"
    );
    assert!(
        !found_global_fallthrough,
        "system-scoped search must not fall through to global/no-system output"
    );
    assert!(
        function_details_have_returns,
        "C_Map.GetMapInfo detailed output must include its generated Returns section"
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
