//! Behavior probes for APIDocumentation slash-command help output.

use crate::common::blizzard_addon_harness::{
    load_blizzard_addon_closure_into_env, with_blizzard_addon_startup_shape,
};
use crate::common::panel_fixtures::{
    blizzard_ui_dir, clear_recorded_lua_errors, recorded_lua_errors,
};

const ROOT: &str = "Blizzard_APIDocumentation";

#[test]
fn help_slash_commands_write_usage_to_default_chat_frame() {
    with_blizzard_addon_startup_shape(&[], &[], |env, _loaded| {
        load_api_documentation(env);

        let (ok, error): (bool, Option<String>) = env
            .eval(
                r#"
                local commands = { "?", "", "help" }
                local systemInfo = ChatTypeInfo["SYSTEM"]

                for _, command in ipairs(commands) do
                    DEFAULT_CHAT_FRAME:Clear()
                    local ok, err = pcall(APIDocumentation.HandleSlashCommand, APIDocumentation, command)
                    if not ok then
                        return false, tostring(err)
                    end

                    local lineCount = DEFAULT_CHAT_FRAME:GetNumMessages()
                    local text, r, g, b = DEFAULT_CHAT_FRAME:GetMessageInfo(1)
                    local hasUsageBanner = text == "Usage:"
                    local hasUsageBody = lineCount > 1
                    local hasSystemColor = r == systemInfo.r
                        and g == systemInfo.g
                        and b == systemInfo.b

                    if not (hasUsageBanner and hasUsageBody and hasSystemColor) then
                        return false, nil
                    end
                end

                return true, nil
                "#,
            )
            .expect("APIDocumentation help slash-command probe must run cleanly");

        assert!(
            ok,
            "?, empty, and help commands must write the usage banner with SYSTEM chat color; error={error:?}"
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
