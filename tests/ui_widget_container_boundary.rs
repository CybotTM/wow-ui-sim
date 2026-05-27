#[test]
fn ui_widget_container_mixin_is_not_c_api_surface() {
    let c_spec = include_str!("../src/c_api/c_spec.rs");
    let utility_registration = include_str!("../src/lua_api/globals/utility_system_spell/mod.rs");

    assert!(
        !c_spec.contains("UIWidgetContainerMixin"),
        "UIWidgetContainerMixin is a Lua mixin and should not live in c_api::c_spec"
    );
    assert!(
        utility_registration.contains("ui_widget_container::register_widget_container_mixin"),
        "UIWidgetContainerMixin should be registered from the Lua globals layer"
    );
}
