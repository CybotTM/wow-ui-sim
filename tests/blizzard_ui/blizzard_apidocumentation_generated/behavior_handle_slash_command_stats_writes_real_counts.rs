//! Behavior probes for generated APIDocumentation stats output.

use crate::common::blizzard_addon_harness::load_blizzard_addon_closure_into_env;
use crate::common::blizzard_addon_harness::new_blizzard_addon_env;
use crate::common::panel_fixtures::{
    blizzard_ui_dir, clear_recorded_lua_errors, load_panel_addons, recorded_lua_errors,
};

const ROOT: &str = "Blizzard_APIDocumentationGenerated";

#[test]
fn stats_slash_command_writes_generated_corpus_counts() {
    let env = load_generated_api_documentation();

    let (
        banner,
        actual_system_count,
        written_system_count,
        actual_function_count,
        written_function_count,
        actual_event_count,
        written_event_count,
        actual_table_count,
        written_table_count,
    ): (String, i64, i64, i64, i64, i64, i64, i64, i64) = env
        .eval(
            r#"
            DEFAULT_CHAT_FRAME:Clear()
            APIDocumentation:HandleSlashCommand("stats")

            local function sumSystemMethod(methodName)
                local total = 0
                for _, systemInfo in ipairs(APIDocumentation.systems) do
                    total = total + systemInfo[methodName](systemInfo)
                end
                return total
            end

            local function countFromLine(index)
                local text = DEFAULT_CHAT_FRAME:GetMessageInfo(index)
                return tonumber(text:match("(%d+)$"))
            end

            return DEFAULT_CHAT_FRAME:GetMessageInfo(1),
                   #APIDocumentation.systems,
                   countFromLine(2),
                   sumSystemMethod("GetNumFunctions"),
                   countFromLine(3),
                   sumSystemMethod("GetNumEvents"),
                   countFromLine(4),
                   sumSystemMethod("GetNumTables"),
                   countFromLine(5)
            "#,
        )
        .expect("generated APIDocumentation stats slash-command probe must run cleanly");

    assert_eq!("Stats:", banner, "stats output must start with a banner");
    assert_eq!(
        actual_system_count, written_system_count,
        "stats output must report the live generated system count"
    );
    assert_eq!(
        actual_function_count, written_function_count,
        "stats output must report the live generated function count"
    );
    assert_eq!(
        actual_event_count, written_event_count,
        "stats output must report the live generated event count"
    );
    assert_eq!(
        actual_table_count, written_table_count,
        "stats output must report the live generated table count"
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
