//! Behavior probes for APIDocumentation open-dump link output.

use crate::common::blizzard_addon_harness::{
    load_blizzard_addon_closure_into_env, with_blizzard_addon_startup_shape,
};
use crate::common::panel_fixtures::{
    blizzard_ui_dir, clear_recorded_lua_errors, recorded_lua_errors,
};

const ROOT: &str = "Blizzard_APIDocumentation";
const DUMP_PREFIX: &str = "/dump ";

#[test]
fn opendump_link_records_chat_edit_dump_command() {
    with_blizzard_addon_startup_shape(&[], &[], |env, _loaded| {
        load_api_documentation(env);

        let generated_link_uses_api_payload: bool = env
            .eval(
                r#"
                APIDocumentation:AddDocumentationTable({
                    Name = "DumpSystem",
                    Type = "System",
                    Namespace = "",
                    Tables = {},
                    Functions = {
                        {
                            Name = "GetTime",
                            Type = "Function",
                            Arguments = {},
                            Returns = {},
                        },
                    },
                    Events = {},
                })

                local apiInfo = APIDocumentation.functions[1]
                local generatedLink = apiInfo:GenerateAPILink()
                local generatedPayload = generatedLink:match("|H([^|]+)|h")

                APIDocumentation:HandleAPILink(
                    generatedPayload,
                    APIDocumentation.Commands.OpenDump
                )

                return generatedLink:find("|Hapi:function:GetTime:", 1, true) ~= nil
                "#,
            )
            .expect("APIDocumentation open-dump link probe must run cleanly");

        let state = env.state();
        let sim = state.borrow();
        let chat_edit_state = sim
            .chat_edit_open_state
            .as_ref()
            .expect("OpenDump link must seed chat edit state");

        assert!(
            generated_link_uses_api_payload,
            "generated APIDocumentation links must use the `api:function:GetTime:` payload"
        );
        assert!(
            chat_edit_state.text.starts_with(DUMP_PREFIX),
            "OpenDump link must seed a /dump chat command; got {:?}",
            chat_edit_state.text
        );
        assert_eq!(
            "/dump GetTime()", chat_edit_state.text,
            "OpenDump link must include the target function call"
        );
        assert!(
            chat_edit_state.chat_type.is_none(),
            "HandleOpenDump passes nil chat type to ChatFrameUtil.OpenChat"
        );
        assert_eq!(
            Some((chat_edit_state.text.len() - 1) as i64),
            chat_edit_state.cursor_position,
            "HandleOpenDump parks the cursor just before the closing parenthesis"
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
