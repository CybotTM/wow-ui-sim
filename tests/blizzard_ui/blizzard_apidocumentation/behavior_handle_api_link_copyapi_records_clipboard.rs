//! Behavior probes for APIDocumentation copy-API link output.

use crate::common::blizzard_addon_harness::{
    load_blizzard_addon_closure_into_env, with_blizzard_addon_startup_shape,
};
use crate::common::panel_fixtures::{
    blizzard_ui_dir, clear_recorded_lua_errors, recorded_lua_errors,
};

const ROOT: &str = "Blizzard_APIDocumentation";

#[test]
fn copyapi_link_records_function_clipboard_string() {
    with_blizzard_addon_startup_shape(&[], &[], |env, _loaded| {
        load_api_documentation(env);

        let (expected_clipboard_text, generated_link_uses_api_payload): (String, bool) = env
            .eval(
                r#"
                APIDocumentation:AddDocumentationTable({
                    Name = "ClipboardSystem",
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

                APIDocumentation:HandleAPILink(
                    generatedPayload,
                    APIDocumentation.Commands.CopyAPI
                )

                return apiInfo:GetClipboardString(), generatedLinkUsesApiPayload
                "#,
            )
            .expect("APIDocumentation copy-API link probe must run cleanly");

        let state = env.state();
        let sim = state.borrow();
        assert!(
            generated_link_uses_api_payload,
            "generated APIDocumentation links must use the `api:function:GetTime:` payload"
        );
        assert_eq!(
            Some(expected_clipboard_text.as_str()),
            sim.clipboard.last_text.as_deref(),
            "CopyAPI link must copy GetTime's GetClipboardString output"
        );
        assert!(
            !sim.clipboard.last_remove_markup,
            "APIDocumentation calls CopyToClipboard without removeMarkup"
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
