//! Public-API test stage only; no BulletTextList host producer or supplied records.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

use wow_ui_sim::lua_api::WowLuaEnv;

fn assert_lua(env: &WowLuaEnv, phase: &str, code: &str) {
    env.eval::<()>(code)
        .unwrap_or_else(|error| panic!("bullet-text-list [{phase}]: {error}"));
    let errors = env.state().borrow().lua_errors.clone();
    assert!(
        errors.is_empty(),
        "bullet-text-list [{phase}] Lua errors: {errors:?}"
    );
}

#[test]
fn bullet_text_list_public_api_rejects_non_numeric_widget_id_with_context() {
    let env = WowLuaEnv::new().expect("bullet-text-list validation environment");
    assert_lua(
        &env,
        "first observable public widgetID validation boundary",
        r#"
        local function reject(value, label)
            local ok, message = pcall(function()
                return C_UIWidgetManager.GetBulletTextListWidgetVisualizationInfo(value)
            end)
            assert(not ok, label .. ': expected widgetID validation error')
            assert(type(message) == 'string', label .. ': expected contextual error text')
            assert(string.find(message,
                'C_UIWidgetManager.GetBulletTextListWidgetVisualizationInfo', 1, true),
                label .. ': missing API context: ' .. tostring(message))
            assert(string.find(message, 'widgetID must be a number', 1, true),
                label .. ': wrong validation boundary: ' .. tostring(message))
        end
        reject(nil, 'nil')
        reject('7601001', 'string')
        reject(true, 'bool')
        reject({}, 'table')
    "#,
    );
}

#[test]
fn bullet_text_list_empty_default_unknown_numeric_id_returns_one_nil_simulator_policy() {
    let env = WowLuaEnv::new().expect("bullet-text-list empty-default environment");
    assert_lua(
        &env,
        "empty default and unknown IDs: simulator arity policy, not native proof",
        r#"
        for _, id in ipairs({7601001, -2.5, 0, 617.25, 4294967299}) do
            local function query()
                return C_UIWidgetManager.GetBulletTextListWidgetVisualizationInfo(id)
            end
            assert(select('#', query()) == 1,
                'simulator arity policy: exactly one return for unknown ID ' .. tostring(id))
            assert(query() == nil,
                'empty default: no supplied BulletTextList record for ID ' .. tostring(id))
        end
    "#,
    );
}
