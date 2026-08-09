use wow_ui_sim::lua_api::WowLuaEnv;

const HOUSE_EXTERIOR_TYPE_SCRIPT: &str = r#"
    local getCurrentHouseExteriorType = C_HouseExterior.GetCurrentHouseExteriorType
    if type(getCurrentHouseExteriorType) ~= "function" then
        return "not_callable", 0, 0, ""
    end

    local returnCount = select('#', getCurrentHouseExteriorType())
    local exteriorType, exteriorTypeName = getCurrentHouseExteriorType()
    return type(exteriorType), returnCount, exteriorType, exteriorTypeName
"#;

const HOUSE_EXTERIOR_SCRIPT: &str = r#"
    if C_HouseExterior.IsExteriorDecorHidden() ~= false then
        return "wrong_initial_hidden_state"
    end

    if C_HouseExterior.IsAnyDecorAttachedToHouseExterior() ~= true then
        return "wrong_house_exterior_attachment"
    end

    if C_HouseExterior.IsAnyDecorAttachedToDoor() ~= true then
        return "wrong_door_attachment"
    end

    if C_HouseExterior.IsAnyDecorAttachedToSelectedFixturePoint() ~= true then
        return "wrong_selected_point_attachment"
    end

    if C_HouseExterior.IsAnyDecorAttachedToCoreFixture(Enum.HousingFixtureType.Base) ~= true then
        return "wrong_base_fixture_attachment"
    end

    if C_HouseExterior.IsAnyDecorAttachedToCoreFixture(Enum.HousingFixtureType.Roof) ~= false then
        return "wrong_roof_fixture_attachment"
    end

    if C_HouseExterior.IsAnyDecorAttachedToCoreFixture(99999) ~= false then
        return "wrong_unknown_fixture_attachment"
    end

    C_HouseExterior.SetExteriorDecorHidden(true)
    if C_HouseExterior.IsExteriorDecorHidden() ~= true then
        return "hide_toggle_not_applied"
    end

    C_HouseExterior.SetExteriorDecorHidden(false)
    if C_HouseExterior.IsExteriorDecorHidden() ~= false then
        return "hide_toggle_not_cleared"
    end

    return "ok"
"#;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn current_house_exterior_type_returns_seeded_type_and_name() {
    let env = env();
    let (value_type, return_count, exterior_type, exterior_type_name): (String, i64, i64, String) =
        env.eval(HOUSE_EXTERIOR_TYPE_SCRIPT)
            .expect("seeded C_HouseExterior type method should be queryable");

    assert_eq!(value_type, "number");
    assert_eq!(return_count, 2);
    assert_eq!(exterior_type, 1);
    assert_eq!(exterior_type_name, "Sunspire Cottage");
}

#[test]
fn house_exterior_attachment_queries_and_hidden_toggle_use_seeded_state() {
    let env = env();
    let result: String = env
        .eval(HOUSE_EXTERIOR_SCRIPT)
        .expect("seeded C_HouseExterior methods should be queryable");
    assert_eq!(result, "ok");
}
