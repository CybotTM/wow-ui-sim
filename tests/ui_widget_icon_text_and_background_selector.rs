//! Public selector regression; no supplied-record scaffold or cached UI load required.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

use wow_ui_sim::lua_api::WowLuaEnv;

fn assert_icon_background_selector_rejected(arguments: &str) {
    let env = WowLuaEnv::new().expect("create fresh selector-test environment");
    env.exec(&format!(
        r#"
        local sharedOK, sharedError = pcall(
            C_UIWidgetManager.GetWidgetSetInfo{arguments})
        assert(not sharedOK, 'shared required-number control must reject')
        assert(type(sharedError) == 'string')
        assert(string.find(sharedError, 'must be a number', 1, true))

        local ok, message = pcall(
            C_UIWidgetManager.GetIconTextAndBackgroundWidgetVisualizationInfo{arguments})
        assert(not ok, 'IconTextAndBackground invalid widgetID must reject')
        assert(type(message) == 'string')
        assert(string.find(message, 'must be a number', 1, true),
            'target must reject at the required-number boundary')
        "#,
    ))
    .expect("IconTextAndBackground requires a numeric widgetID");
}

#[test]
fn icon_text_and_background_missing_selector_rejects_like_shared_required_number() {
    assert_icon_background_selector_rejected("");
}

#[test]
fn icon_text_and_background_non_number_selectors_reject_like_shared_required_number() {
    for arguments in [", nil", ", '42'", ", true", ", {}", ", function() end"] {
        assert_icon_background_selector_rejected(arguments);
    }
}
