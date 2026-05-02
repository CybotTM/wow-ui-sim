//! Behavior probes for APIDocumentation slash-command stats output.

use crate::common::blizzard_addon_harness::{
    load_blizzard_addon_closure_into_env, with_blizzard_addon_startup_shape,
};
use crate::common::panel_fixtures::{
    blizzard_ui_dir, clear_recorded_lua_errors, recorded_lua_errors,
};

const ROOT: &str = "Blizzard_APIDocumentation";

#[test]
fn stats_slash_command_writes_seeded_counts() {
    with_blizzard_addon_startup_shape(&[], &[], |env, _loaded| {
        load_api_documentation(env);

        let (
            banner,
            system_count,
            function_count,
            event_count,
            table_count,
        ): (String, i64, i64, i64, i64) = env
            .eval(
                r#"
                APIDocumentation:AddDocumentationTable({
                    Name = "FirstSystem",
                    Type = "System",
                    Namespace = "First",
                    Tables = {
                        { Name = "FirstTable", Type = "Structure", Fields = {} },
                    },
                    Functions = {
                        { Name = "FirstFunction", Type = "Function", Arguments = {}, Returns = {} },
                        { Name = "SecondFunction", Type = "Function", Arguments = {}, Returns = {} },
                    },
                    Events = {
                        { Name = "FirstEvent", LiteralName = "FIRST_EVENT", Type = "Event", Payload = {} },
                    },
                })

                APIDocumentation:AddDocumentationTable({
                    Name = "SecondSystem",
                    Type = "System",
                    Namespace = "Second",
                    Tables = {},
                    Functions = {
                        { Name = "ThirdFunction", Type = "Function", Arguments = {}, Returns = {} },
                    },
                    Events = {},
                })

                DEFAULT_CHAT_FRAME:Clear()
                APIDocumentation:HandleSlashCommand("stats")

                local function countFromLine(index)
                    local text = DEFAULT_CHAT_FRAME:GetMessageInfo(index)
                    return tonumber(text:match("(%d+)$"))
                end

                local banner = DEFAULT_CHAT_FRAME:GetMessageInfo(1)
                return banner,
                       countFromLine(2),
                       countFromLine(3),
                       countFromLine(4),
                       countFromLine(5)
                "#,
            )
            .expect("APIDocumentation stats slash-command probe must run cleanly");

        assert_eq!("Stats:", banner, "stats output must start with a banner");
        assert_eq!(2, system_count, "stats must count seeded systems");
        assert_eq!(
            3, function_count,
            "stats must count seeded system functions"
        );
        assert_eq!(1, event_count, "stats must count seeded system events");
        assert_eq!(1, table_count, "stats must count seeded system tables");
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
