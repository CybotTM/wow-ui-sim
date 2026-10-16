//! Private, unexecuted public selector draft; no supplied records or cached UI load.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

use wow_ui_sim::lua_api::WowLuaEnv;

fn assert_horizontal_currencies_selector_rejected(arguments: &str) {
    let env = WowLuaEnv::new().expect("create fresh selector-test environment");
    env.exec(&format!(
        r#"
        local sharedOK, sharedError = pcall(
            C_UIWidgetManager.GetWidgetSetInfo{arguments})
        assert(not sharedOK, 'shared required-number control must reject')
        assert(type(sharedError) == 'string')
        assert(string.find(sharedError, 'must be a number', 1, true))

        local getter = C_UIWidgetManager.GetHorizontalCurrenciesWidgetVisualizationInfo
        assert(type(getter) == 'function', 'public HorizontalCurrencies getter must exist')
        local function assertAbsent(...)
            assert(select('#', ...) == 1, 'absent numeric widgetID must return exactly one result')
            assert(select(1, ...) == nil, 'absent numeric widgetID must return nil')
        end
        assertAbsent(getter(934208.5))

        local ok, message = pcall(getter{arguments})
        assert(not ok, 'HorizontalCurrencies invalid widgetID must reject')
        assert(type(message) == 'string')
        assert(string.find(message, 'must be a number', 1, true),
            'target must reject at the required-number boundary')
        assert(string.find(message, 'GetHorizontalCurrenciesWidgetVisualizationInfo', 1, true),
            'required-number diagnostic must identify the public getter')
        "#,
    ))
    .expect("HorizontalCurrencies requires a numeric widgetID");
}

#[test]
fn horizontal_currencies_missing_selector_rejects_like_shared_required_number() {
    assert_horizontal_currencies_selector_rejected("");
}

#[test]
fn horizontal_currencies_non_number_selectors_reject_like_shared_required_number() {
    for arguments in [
        ", nil",
        ", 'not-a-number'",
        ", '42'",
        ", true",
        ", {}",
        ", function() end",
    ] {
        assert_horizontal_currencies_selector_rejected(arguments);
    }
}
