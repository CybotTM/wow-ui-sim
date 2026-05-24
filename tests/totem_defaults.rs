use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn get_totem_info_defaults_to_no_active_totem() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    let result: String = env
        .eval(
            r#"
            local haveTotem, name, startTime, duration, icon = GetTotemInfo(1)
            if haveTotem ~= false then return "active" end
            if name ~= nil then return "name" end
            if startTime ~= 0 then return "start" end
            if duration ~= 0 then return "duration" end
            if icon ~= nil then return "icon" end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok");
}
