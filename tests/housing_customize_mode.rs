use wow_ui_sim::lua_api::WowLuaEnv;

const HOUSING_CUSTOMIZE_MODE_SCRIPT: &str = r#"
    if C_HousingCustomizeMode.IsDecorSelected() ~= true then
        return "selected_decor_not_reported"
    end

    local info = C_HousingCustomizeMode.GetSelectedDecorInfo()
    if not info then
        return "missing_selected_decor_info"
    end

    if info.decorGUID ~= "Decor-Selection-1001" then
        return "wrong_decor_guid:" .. tostring(info.decorGUID)
    end

    if info.name ~= "Sunspire Chair" then
        return "wrong_decor_name:" .. tostring(info.name)
    end

    if info.canBeCustomized ~= true then
        return "wrong_customizable_state"
    end

    if info.canBeRemoved ~= true then
        return "wrong_removable_state"
    end

    if info.isLocked ~= false then
        return "wrong_locked_state"
    end

    return "ok"
"#;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn housing_customize_mode_selected_decor_methods_use_seeded_state() {
    let env = env();
    let result: String = env
        .eval(HOUSING_CUSTOMIZE_MODE_SCRIPT)
        .expect("seeded C_HousingCustomizeMode methods should be queryable");
    assert_eq!(result, "ok");
}
