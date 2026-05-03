//! Behavior probes for generated APIDocumentation system-list output.

use crate::common::blizzard_addon_harness::load_blizzard_addon_closure_into_env;
use crate::common::blizzard_addon_harness::new_blizzard_addon_env;
use crate::common::panel_fixtures::{
    blizzard_ui_dir, clear_recorded_lua_errors, load_panel_addons, recorded_lua_errors,
};

const ROOT: &str = "Blizzard_APIDocumentationGenerated";

#[test]
fn system_list_slash_command_lists_every_generated_system() {
    let env = load_generated_api_documentation();

    let (system_count, message_count, failure): (i64, i64, String) = env
        .eval(
            r#"
            DEFAULT_CHAT_FRAME:Clear()
            APIDocumentation:HandleSlashCommand("system list")

            local systemCount = #APIDocumentation.systems
            local messageCount = DEFAULT_CHAT_FRAME:GetNumMessages()

            local expectedHeader = string.format("All systems (%d):", systemCount)
            local header = DEFAULT_CHAT_FRAME:GetMessageInfo(1)
            if header ~= expectedHeader then
                return systemCount,
                       messageCount,
                       string.format("expected header %q, got %q", expectedHeader, tostring(header))
            end

            for index, systemInfo in ipairs(APIDocumentation.systems) do
                local line = DEFAULT_CHAT_FRAME:GetMessageInfo(index + 1)
                local systemName = systemInfo:GetName()
                if type(line) ~= "string" or not line:find(systemName, 1, true) then
                    return systemCount,
                           messageCount,
                           string.format(
                               "line %d expected system %q, got %q",
                               index,
                               systemName,
                               tostring(line)
                           )
                end
            end

            return systemCount, messageCount, ""
            "#,
        )
        .expect("generated APIDocumentation system-list probe must run cleanly");

    assert_eq!(
        system_count + 1,
        message_count,
        "system list must write one header plus one line per generated system"
    );
    assert_eq!(
        "", failure,
        "system list output must contain every generated system name"
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
