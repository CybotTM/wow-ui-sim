use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn bare_frames_expose_runtime_panel_helper_methods() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");

    let methods_available: bool = env
        .eval(
            r#"
            local button = CreateFrame("Button")
            return type(button.SetSelectionText) == "function"
                and type(button.SetIsDefaultCallback) == "function"
                and type(button.SetUpdateCallback) == "function"
                and type(button.EnableRegenerateOnResponse) == "function"
                and type(button.SetOnClickHandler) == "function"
                and type(button.SetOnEnterHandler) == "function"
                and type(button.SetFixedSize) == "function"
            "#,
        )
        .expect("helper method probe should evaluate");

    assert!(
        methods_available,
        "bare frame userdata should expose the runtime panel helper methods"
    );
}
