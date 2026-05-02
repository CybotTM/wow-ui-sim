//! Behavior probes for APIDocumentation system-list slash output.

use crate::common::blizzard_addon_harness::{
    load_blizzard_addon_closure_into_env, with_blizzard_addon_startup_shape,
};
use crate::common::panel_fixtures::{
    blizzard_ui_dir, clear_recorded_lua_errors, recorded_lua_errors,
};

const ROOT: &str = "Blizzard_APIDocumentation";

#[test]
fn system_list_slash_command_writes_all_system_names() {
    with_blizzard_addon_startup_shape(&[], &[], |env, _loaded| {
        load_api_documentation(env);

        let (header, first_line, second_line, third_line): (String, String, String, String) = env
            .eval(
                r#"
                local systems = { "AlphaSystem", "BetaSystem", "GammaSystem" }
                for _, systemName in ipairs(systems) do
                    APIDocumentation:AddDocumentationTable({
                        Name = systemName,
                        Type = "System",
                        Namespace = systemName,
                        Tables = {},
                        Functions = {},
                        Events = {},
                    })
                end

                DEFAULT_CHAT_FRAME:Clear()
                APIDocumentation:HandleSlashCommand("system list")

                local header = DEFAULT_CHAT_FRAME:GetMessageInfo(1)
                local firstLine = DEFAULT_CHAT_FRAME:GetMessageInfo(2)
                local secondLine = DEFAULT_CHAT_FRAME:GetMessageInfo(3)
                local thirdLine = DEFAULT_CHAT_FRAME:GetMessageInfo(4)

                return header, firstLine, secondLine, thirdLine
                "#,
            )
            .expect("APIDocumentation system-list slash-command probe must run cleanly");

        assert_eq!(
            "All systems (3):", header,
            "system list output must start with the total system count"
        );
        assert!(
            first_line.contains("AlphaSystem"),
            "first system line must contain the first seeded system; got {first_line:?}"
        );
        assert!(
            second_line.contains("BetaSystem"),
            "second system line must contain the second seeded system; got {second_line:?}"
        );
        assert!(
            third_line.contains("GammaSystem"),
            "third system line must contain the third seeded system; got {third_line:?}"
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
