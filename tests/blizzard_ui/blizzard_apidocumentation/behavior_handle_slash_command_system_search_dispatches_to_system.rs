//! Behavior probes for APIDocumentation system-scoped slash search.

use crate::common::blizzard_addon_harness::{
    load_blizzard_addon_closure_into_env, with_blizzard_addon_startup_shape,
};
use crate::common::panel_fixtures::{
    blizzard_ui_dir, clear_recorded_lua_errors, recorded_lua_errors,
};

const ROOT: &str = "Blizzard_APIDocumentation";

#[test]
fn system_search_slash_command_dispatches_to_system_scope() {
    with_blizzard_addon_startup_shape(&[], &[], |env, _loaded| {
        load_api_documentation(env);

        let (
            found_match_header,
            found_function_line,
            found_global_fallthrough,
            bare_system_search_outputs_usage,
        ): (bool, bool, bool, bool) = env
            .eval(
                r#"
                APIDocumentation:AddDocumentationTable({
                    Name = "Friends",
                    Type = "System",
                    Namespace = "C_FriendList",
                    Tables = {},
                    Functions = {
                        { Name = "GetFriendInfo", Type = "Function", Arguments = {}, Returns = {} },
                    },
                    Events = {},
                })

                DEFAULT_CHAT_FRAME:Clear()
                APIDocumentation:HandleSlashCommand("Friends search GetFriendInfo")

                local foundMatchHeader = false
                local foundFunctionLine = false
                local foundGlobalFallthrough = false
                for index = 1, DEFAULT_CHAT_FRAME:GetNumMessages() do
                    local text = DEFAULT_CHAT_FRAME:GetMessageInfo(index)
                    foundMatchHeader = foundMatchHeader
                        or text == 'Found 1 API that matches "GetFriendInfo"'
                    foundFunctionLine = foundFunctionLine
                        or text:find("GetFriendInfo", 1, true) ~= nil
                    foundGlobalFallthrough = foundGlobalFallthrough
                        or text:find("No system found", 1, true) ~= nil
                        or text:find("No API found that matches", 1, true) ~= nil
                end

                DEFAULT_CHAT_FRAME:Clear()
                APIDocumentation:HandleSlashCommand("Friends GetFriendInfo")
                local bareCommandFirstLine = DEFAULT_CHAT_FRAME:GetMessageInfo(1)

                return foundMatchHeader,
                       foundFunctionLine,
                       foundGlobalFallthrough,
                       bareCommandFirstLine == "Usage:"
                "#,
            )
            .expect("APIDocumentation system search slash-command probe must run cleanly");

        assert!(
            found_match_header,
            "system-scoped search must write the scoped match-count header"
        );
        assert!(
            found_function_line,
            "system-scoped search must write the matching function output line"
        );
        assert!(
            !found_global_fallthrough,
            "system-scoped search must not fall through to global/no-system search output"
        );
        assert!(
            bare_system_search_outputs_usage,
            "the pinned Blizzard command grammar requires `search`/`s` before the API name"
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
