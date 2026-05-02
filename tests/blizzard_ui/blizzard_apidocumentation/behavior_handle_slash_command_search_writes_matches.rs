//! Behavior probes for APIDocumentation slash-command search output.

use crate::common::blizzard_addon_harness::{
    load_blizzard_addon_closure_into_env, with_blizzard_addon_startup_shape,
};
use crate::common::panel_fixtures::{
    blizzard_ui_dir, clear_recorded_lua_errors, recorded_lua_errors,
};

const ROOT: &str = "Blizzard_APIDocumentation";

#[test]
fn search_slash_command_writes_matches_and_no_match_notice() {
    with_blizzard_addon_startup_shape(&[], &[], |env, _loaded| {
        load_api_documentation(env);

        let (foo_match_count, found_foo_bar, found_foo_baz, found_quux, no_match_notice): (
            i64,
            bool,
            bool,
            bool,
            String,
        ) = env
            .eval(
                r#"
                APIDocumentation:AddDocumentationTable({
                    Name = "SearchSystem",
                    Type = "System",
                    Namespace = "",
                    Tables = {},
                    Functions = {
                        { Name = "FooBar", Type = "Function", Arguments = {}, Returns = {} },
                        { Name = "FooBaz", Type = "Function", Arguments = {}, Returns = {} },
                        { Name = "Quux", Type = "Function", Arguments = {}, Returns = {} },
                    },
                    Events = {},
                })

                DEFAULT_CHAT_FRAME:Clear()
                APIDocumentation:HandleSlashCommand("search foo")

                local foundFooBar = false
                local foundFooBaz = false
                local foundQuux = false
                for index = 1, DEFAULT_CHAT_FRAME:GetNumMessages() do
                    local text = DEFAULT_CHAT_FRAME:GetMessageInfo(index)
                    foundFooBar = foundFooBar or text:find("FooBar", 1, true) ~= nil
                    foundFooBaz = foundFooBaz or text:find("FooBaz", 1, true) ~= nil
                    foundQuux = foundQuux or text:find("Quux", 1, true) ~= nil
                end

                local matchHeader = DEFAULT_CHAT_FRAME:GetMessageInfo(2)
                local fooMatchCount = tonumber(matchHeader:match("Found (%d+) API"))

                DEFAULT_CHAT_FRAME:Clear()
                APIDocumentation:HandleSlashCommand("search nothing")

                local noMatchNotice = DEFAULT_CHAT_FRAME:GetMessageInfo(2)
                return fooMatchCount, foundFooBar, foundFooBaz, foundQuux, noMatchNotice
                "#,
            )
            .expect("APIDocumentation search slash-command probe must run cleanly");

        assert_eq!(2, foo_match_count, "search foo must report two API matches");
        assert!(found_foo_bar, "search foo must write a FooBar match line");
        assert!(found_foo_baz, "search foo must write a FooBaz match line");
        assert!(
            !found_quux,
            "search foo must not write the non-matching Quux function"
        );
        assert_eq!(
            "No API found that matches \"nothing\"", no_match_notice,
            "search nothing must write the no-match notice"
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
