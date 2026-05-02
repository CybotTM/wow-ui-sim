//! Behavior probes for APIDocumentation chat output helpers.

use crate::common::blizzard_addon_harness::{
    load_blizzard_addon_closure_into_env, with_blizzard_addon_startup_shape,
};
use crate::common::panel_fixtures::{
    blizzard_ui_dir, clear_recorded_lua_errors, recorded_lua_errors,
};

const ROOT: &str = "Blizzard_APIDocumentation";
const COLOR_TOLERANCE: f64 = 0.001;

#[test]
fn write_line_uses_system_chat_color_and_write_all_lines_preserves_order() {
    with_blizzard_addon_startup_shape(&[], &[], |env, _loaded| {
        load_api_documentation(env);

        let (
            write_line_count,
            message_text,
            red_delta,
            green_delta,
            blue_delta,
            first_line,
            second_line,
            write_all_count,
        ): (i32, String, f64, f64, f64, String, String, i32) = env
            .eval(
                r#"
                DEFAULT_CHAT_FRAME:Clear()
                APIDocumentation:WriteLine("hello")

                local text, r, g, b = DEFAULT_CHAT_FRAME:GetMessageInfo(1)
                local writeLineCount = DEFAULT_CHAT_FRAME:GetNumMessages()
                local systemColor = ChatTypeInfo["SYSTEM"]

                DEFAULT_CHAT_FRAME:Clear()
                APIDocumentation:WriteAllLines({ "a", "b" })

                return writeLineCount,
                       text,
                       math.abs(r - systemColor.r),
                       math.abs(g - systemColor.g),
                       math.abs(b - systemColor.b),
                       DEFAULT_CHAT_FRAME:GetMessageInfo(1),
                       DEFAULT_CHAT_FRAME:GetMessageInfo(2),
                       DEFAULT_CHAT_FRAME:GetNumMessages()
                "#,
            )
            .expect("APIDocumentation chat output probe must run cleanly");

        assert_eq!(
            1, write_line_count,
            "WriteLine must append exactly one chat message"
        );
        assert_eq!("hello", message_text, "WriteLine must preserve text");
        assert!(
            red_delta <= COLOR_TOLERANCE,
            "WriteLine red channel must match ChatTypeInfo.SYSTEM"
        );
        assert!(
            green_delta <= COLOR_TOLERANCE,
            "WriteLine green channel must match ChatTypeInfo.SYSTEM"
        );
        assert!(
            blue_delta <= COLOR_TOLERANCE,
            "WriteLine blue channel must match ChatTypeInfo.SYSTEM"
        );
        assert_eq!(
            "a", first_line,
            "WriteAllLines must append first line first"
        );
        assert_eq!(
            "b", second_line,
            "WriteAllLines must append second line second"
        );
        assert_eq!(
            2, write_all_count,
            "WriteAllLines must append one message per input line"
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
