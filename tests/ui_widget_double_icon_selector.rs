//! Public missing-selector counterexample; no passive input scaffold required.
#![cfg(feature = "client-retail")]

use crate::common;
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn double_icon_and_text_missing_selector_rejects_like_shared_required_number() {
    common::with_timeout(88, || {
        let env = WowLuaEnv::new().unwrap();
        env.exec(
            r#"
            local sharedOK, sharedError = pcall(C_UIWidgetManager.GetWidgetSetInfo)
            assert(not sharedOK and type(sharedError) == 'string')
            assert(string.find(sharedError, 'must be a number', 1, true))
            local ok, message = pcall(
                C_UIWidgetManager.GetDoubleIconAndTextWidgetVisualizationInfo)
            assert(not ok, 'missing widgetID must reject, not return nil')
            assert(type(message) == 'string')
            assert(string.find(message, 'must be a number', 1, true))
            "#,
        )
        .expect("DoubleIconAndText requires a numeric widget selector");
    });
}
