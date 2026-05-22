use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn toybox_and_heirloom_filter_defaults_are_registered() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            if C_ToyBoxInfo.IsUsingDefaultFilters() ~= true then return "toybox" end
            if C_HeirloomInfo.IsUsingDefaultFilters() ~= true then return "heirloom" end
            return "ok"
            "#,
        )
        .expect("collection filter defaults should be callable");

    assert_eq!(result, "ok");
}
