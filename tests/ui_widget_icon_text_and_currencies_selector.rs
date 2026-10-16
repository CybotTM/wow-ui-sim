//! Public selector regression; no supplied-record scaffold or cached UI load required.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

use wow_ui_sim::lua_api::WowLuaEnv;

fn assert_icon_currencies_selector_rejected(arguments: &str) {
    let env = WowLuaEnv::new().expect("create fresh selector-test environment");
    env.exec(&format!(
        r#"
        local sharedOK, sharedError = pcall(
            C_UIWidgetManager.GetWidgetSetInfo{arguments})
        assert(not sharedOK, 'shared required-number control must reject')
        assert(type(sharedError) == 'string')
        assert(string.find(sharedError, 'must be a number', 1, true))

        local getter = C_UIWidgetManager.GetIconTextAndCurrenciesWidgetVisualizationInfo
        assert(type(getter) == 'function', 'public IconTextAndCurrencies getter must exist')
        local function assertAbsent(...)
            assert(select('#', ...) == 1, 'absent numeric widgetID must return exactly one result')
            assert(select(1, ...) == nil, 'absent numeric widgetID must return nil')
        end
        assertAbsent(getter(934207.5))

        local ok, message = pcall(getter{arguments})
        assert(not ok, 'IconTextAndCurrencies invalid widgetID must reject')
        assert(type(message) == 'string')
        assert(string.find(message, 'must be a number', 1, true),
            'target must reject at the required-number boundary')
        assert(string.find(message, 'GetIconTextAndCurrenciesWidgetVisualizationInfo', 1, true),
            'required-number diagnostic must identify the public getter')
        "#,
    ))
    .expect("IconTextAndCurrencies requires a numeric widgetID");
}

#[test]
fn icon_text_and_currencies_missing_selector_rejects_like_shared_required_number() {
    assert_icon_currencies_selector_rejected("");
}

#[test]
fn icon_text_and_currencies_non_number_selectors_reject_like_shared_required_number() {
    for arguments in [", nil", ", '42'", ", true", ", {}", ", function() end"] {
        assert_icon_currencies_selector_rejected(arguments);
    }
}
