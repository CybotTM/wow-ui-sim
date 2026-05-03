//! Behavior probes for generated APIDocumentation open-dump links.

use crate::common::blizzard_addon_harness::load_blizzard_addon_closure_into_env;
use crate::common::blizzard_addon_harness::new_blizzard_addon_env;
use crate::common::panel_fixtures::{
    blizzard_ui_dir, clear_recorded_lua_errors, load_panel_addons, recorded_lua_errors,
};

const ROOT: &str = "Blizzard_APIDocumentationGenerated";
const DUMP_TEXT: &str = "/dump GetTime()";

#[test]
fn opendump_link_records_generated_get_time_chat_edit_state() {
    let env = load_generated_api_documentation();

    let generated_link_uses_get_time_payload: bool = env
        .eval(
            r#"
            local apiInfo = APIDocumentation:FindAPIByName("function", "GetTime", "SystemTime")
            local generatedLink = apiInfo:GenerateAPILink()
            local generatedPayload = generatedLink:match("|H([^|]+)|h")

            APIDocumentation:HandleAPILink(
                generatedPayload,
                APIDocumentation.Commands.OpenDump
            )

            return generatedLink:find("|Hapi:function:GetTime:SystemTime", 1, true) ~= nil
            "#,
        )
        .expect("generated APIDocumentation open-dump link probe must run cleanly");

    let state = env.state();
    let sim = state.borrow();
    let chat_edit_state = sim
        .chat_edit_open_state
        .as_ref()
        .expect("OpenDump link must seed chat edit state");

    assert!(
        generated_link_uses_get_time_payload,
        "generated API links must include the real GetTime SystemTime payload"
    );
    assert_eq!(
        DUMP_TEXT, chat_edit_state.text,
        "OpenDump link must include the generated target function call"
    );
    assert!(
        chat_edit_state.chat_type.is_none(),
        "HandleOpenDump passes nil chat type to ChatFrameUtil.OpenChat"
    );
    assert_eq!(
        Some((DUMP_TEXT.len() - 1) as i64),
        chat_edit_state.cursor_position,
        "HandleOpenDump parks the cursor just before the closing parenthesis"
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
