use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn ui_parent_manage_frame_positions_is_callable_noop() {
    let env = WowLuaEnv::new().unwrap();
    let result: String = env
        .eval(
            r#"
            UIParent_ManageFramePositions()
            return type(UIParent_ManageFramePositions)
            "#,
        )
        .unwrap();

    assert_eq!(result, "function");
}
