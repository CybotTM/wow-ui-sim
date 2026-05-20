use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn test_tfilter_skips_nil_holes() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    let result: String = env
        .eval(
            r#"
            local t = { 10, false, 30 }
            t[2] = nil
            local sawNil = false
            tFilter(t, function(v)
                if v == nil then
                    sawNil = true
                    return false
                end
                return true
            end, true)
            return tostring(sawNil) .. ":" .. tostring(#t) .. ":" .. tostring(t[1]) .. ":" .. tostring(t[2])
            "#,
        )
        .unwrap();
    assert_eq!(result, "false:2:10:30");
}
