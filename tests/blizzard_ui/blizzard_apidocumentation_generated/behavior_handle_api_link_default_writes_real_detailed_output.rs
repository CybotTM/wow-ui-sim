//! Behavior probes for generated APIDocumentation default API-link output.

use crate::common::blizzard_addon_harness::load_blizzard_addon_closure_into_env;
use crate::common::blizzard_addon_harness::new_blizzard_addon_env;
use crate::common::panel_fixtures::{
    blizzard_ui_dir, clear_recorded_lua_errors, load_panel_addons, recorded_lua_errors,
};

const ROOT: &str = "Blizzard_APIDocumentationGenerated";

#[test]
fn default_api_link_writes_generated_get_time_details() {
    let env = load_generated_api_documentation();

    let (
        generated_link_uses_get_time_payload,
        found_function_line,
        found_return_header,
        found_return_detail,
    ): (bool, bool, bool, bool) = env
        .eval(
            r#"
            local apiInfo = APIDocumentation:FindAPIByName("function", "GetTime", "SystemTime")
            local generatedLink = apiInfo:GenerateAPILink()
            local generatedPayload = generatedLink:match("|H([^|]+)|h")
            local generatedLinkUsesGetTimePayload =
                generatedLink:find("|Hapi:function:GetTime:SystemTime", 1, true) ~= nil

            DEFAULT_CHAT_FRAME:Clear()
            APIDocumentation:HandleAPILink(
                generatedPayload,
                APIDocumentation.Commands.Default
            )

            local foundFunctionLine = false
            local foundReturnHeader = false
            local foundReturnDetail = false
            for index = 1, DEFAULT_CHAT_FRAME:GetNumMessages() do
                local text = DEFAULT_CHAT_FRAME:GetMessageInfo(index)
                foundFunctionLine = foundFunctionLine
                    or text:find("GetTime", 1, true) ~= nil
                foundReturnHeader = foundReturnHeader
                    or text == "   Returns"
                foundReturnDetail = foundReturnDetail
                    or text:find("time", 1, true) ~= nil
                    and text:find("number", 1, true) ~= nil
            end

            return generatedLinkUsesGetTimePayload,
                   foundFunctionLine,
                   foundReturnHeader,
                   foundReturnDetail
            "#,
        )
        .expect("generated APIDocumentation API-link probe must run cleanly");

    assert!(
        generated_link_uses_get_time_payload,
        "generated API links must include the real GetTime SystemTime payload"
    );
    assert!(
        found_function_line,
        "default API-link output must include GetTime's detailed function line"
    );
    assert!(
        found_return_header,
        "default API-link output must include GetTime's Returns section"
    );
    assert!(
        found_return_detail,
        "default API-link output must include GetTime's generated return field"
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
