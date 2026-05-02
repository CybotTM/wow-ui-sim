//! Behavior probes for APIDocumentation indentation strings.

use crate::common::blizzard_addon_harness::{
    load_blizzard_addon_closure_into_env, with_blizzard_addon_startup_shape,
};
use crate::common::panel_fixtures::{
    blizzard_ui_dir, clear_recorded_lua_errors, recorded_lua_errors,
};

const ROOT: &str = "Blizzard_APIDocumentation";

#[test]
fn get_indent_string_stacks_three_space_blocks_linearly() {
    with_blizzard_addon_startup_shape(&[], &[], |env, _loaded| {
        load_api_documentation(env);

        let (zero_indent, one_indent, two_indent, four_indent): (i32, i32, i32, i32) = env
            .eval(
                r#"
                return #APIDocumentation:GetIndentString(0),
                       #APIDocumentation:GetIndentString(1),
                       #APIDocumentation:GetIndentString(2),
                       #APIDocumentation:GetIndentString(4)
                "#,
            )
            .expect("APIDocumentation indent probe must run cleanly");

        assert_eq!(0, zero_indent, "zero indent must be empty");
        assert_eq!(
            3, one_indent,
            "Blizzard APIDocumentation uses three spaces per indent unit"
        );
        assert_eq!(6, two_indent, "two indent units must stack linearly");
        assert_eq!(12, four_indent, "deeper indent units must stack linearly");
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
