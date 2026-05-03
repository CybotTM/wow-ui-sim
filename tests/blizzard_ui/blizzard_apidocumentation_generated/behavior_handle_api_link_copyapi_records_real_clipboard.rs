//! Behavior probes for generated APIDocumentation copy-API links.

use crate::common::blizzard_addon_harness::load_blizzard_addon_closure_into_env;
use crate::common::blizzard_addon_harness::new_blizzard_addon_env;
use crate::common::panel_fixtures::{
    blizzard_ui_dir, clear_recorded_lua_errors, load_panel_addons, recorded_lua_errors,
};

const ROOT: &str = "Blizzard_APIDocumentationGenerated";

#[test]
fn copyapi_link_records_generated_get_time_clipboard_string() {
    let env = load_generated_api_documentation();

    let (expected_clipboard_text, generated_link_uses_get_time_payload): (String, bool) = env
        .eval(
            r#"
            local apiInfo = APIDocumentation:FindAPIByName("function", "GetTime", "SystemTime")
            local generatedLink = apiInfo:GenerateAPILink()
            local generatedPayload = generatedLink:match("|H([^|]+)|h")
            local generatedLinkUsesGetTimePayload =
                generatedLink:find("|Hapi:function:GetTime:SystemTime", 1, true) ~= nil

            APIDocumentation:HandleAPILink(
                generatedPayload,
                APIDocumentation.Commands.CopyAPI
            )

            return apiInfo:GetClipboardString(), generatedLinkUsesGetTimePayload
            "#,
        )
        .expect("generated APIDocumentation copy-API link probe must run cleanly");

    let state = env.state();
    let sim = state.borrow();
    assert!(
        generated_link_uses_get_time_payload,
        "generated API links must include the real GetTime SystemTime payload"
    );
    assert_eq!(
        Some(expected_clipboard_text.as_str()),
        sim.clipboard.last_text.as_deref(),
        "CopyAPI link must copy generated GetTime's GetClipboardString output"
    );
    assert!(
        !sim.clipboard.last_remove_markup,
        "APIDocumentation calls CopyToClipboard without removeMarkup"
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
