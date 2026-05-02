//! Load probes for `Blizzard_APIDocumentation`.

use crate::common::blizzard_addon_harness::{
    load_blizzard_addon_closure_into_env, with_blizzard_addon_startup_shape,
};
use crate::common::panel_fixtures::{
    blizzard_ui_dir, clear_recorded_lua_errors, recorded_lua_errors,
};

const ROOT: &str = "Blizzard_APIDocumentation";

#[test]
fn api_documentation_loads_cleanly_and_expands_default_chat_history() {
    with_blizzard_addon_startup_shape(&[], &[], |env, _loaded| {
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

        let (api_documentation_type, object_type, default_chat_max_lines): (String, String, i64) =
            env.eval(
                r#"
                assert(DEFAULT_CHAT_FRAME ~= nil, "DEFAULT_CHAT_FRAME must exist")
                return type(APIDocumentation),
                       DEFAULT_CHAT_FRAME:GetObjectType(),
                       DEFAULT_CHAT_FRAME:GetMaxLines()
                "#,
            )
            .expect("APIDocumentation load probe must run cleanly");

        assert_eq!(
            api_documentation_type, "table",
            "APIDocumentation singleton must be created by addon load"
        );
        assert!(
            default_chat_max_lines >= 2000,
            "APIDocumentation:OnLoad must raise DEFAULT_CHAT_FRAME ({object_type}) max lines to at least 2000; got {default_chat_max_lines}"
        );
    });
}
