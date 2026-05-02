//! Behavior probes for APIDocumentation default API-link output.

use crate::common::blizzard_addon_harness::{
    load_blizzard_addon_closure_into_env, with_blizzard_addon_startup_shape,
};
use crate::common::panel_fixtures::{
    blizzard_ui_dir, clear_recorded_lua_errors, recorded_lua_errors,
};

const ROOT: &str = "Blizzard_APIDocumentation";

#[test]
fn default_api_link_writes_function_detailed_output() {
    with_blizzard_addon_startup_shape(&[], &[], |env, _loaded| {
        load_api_documentation(env);

        let (
            generated_link_uses_api_payload,
            found_function_line,
            found_return_header,
            found_return_detail,
        ): (bool, bool, bool, bool) = env
            .eval(
                r#"
                APIDocumentation:AddDocumentationTable({
                    Name = "DefaultSystem",
                    Type = "System",
                    Namespace = "",
                    Tables = {},
                    Functions = {
                        {
                            Name = "GetTime",
                            Type = "Function",
                            Arguments = {},
                            Returns = {
                                { Name = "time", Type = "number" },
                            },
                        },
                    },
                    Events = {},
                })

                local apiInfo = APIDocumentation.functions[1]
                local generatedLink = apiInfo:GenerateAPILink()
                local generatedPayload = generatedLink:match("|H([^|]+)|h")
                local generatedLinkUsesApiPayload =
                    generatedLink:find("|Hapi:function:GetTime:", 1, true) ~= nil

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
                end

                return generatedLinkUsesApiPayload,
                       foundFunctionLine,
                       foundReturnHeader,
                       foundReturnDetail
                "#,
            )
            .expect("APIDocumentation API-link probe must run cleanly");

        assert!(
            generated_link_uses_api_payload,
            "generated APIDocumentation links must use the `api:function:GetTime:` payload"
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
            "default API-link output must include GetTime's return field"
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
