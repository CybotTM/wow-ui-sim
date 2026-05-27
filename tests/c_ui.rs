use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn display_safe_area_defaults_return_no_notch() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");

    let result: String = env
        .eval(
            r#"
            if C_UI.ShouldUIParentAvoidNotch() ~= false then return "avoid" end
            if C_UI.DoesAnyDisplayHaveNotch() ~= false then return "display" end

            local left, top, right, bottom = C_UI.GetTopLeftNotchSafeRegion()
            if left ~= 0 or top ~= 0 or right ~= 0 or bottom ~= 0 then
                return "region"
            end

            return "ok"
            "#,
        )
        .expect("C_UI display-safe-area defaults should be callable");

    assert_eq!(result, "ok");
}
