use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn frame_environment_does_not_occupy_numeric_array_slot() {
    let env = WowLuaEnv::new().expect("lua env");

    env.exec(
        r#"
        local element = CreateFrame("Frame", nil, UIParent)
        assert(element[1] == nil, "fresh frame numeric slot 1 should be available to addons")

        local frameEnv = debug.getfenv(element)
        assert(type(frameEnv) == "table", "frame env proxy should be a table")
        assert(frameEnv[1] == element, "frame env slot 1 should point at the frame fields")

        rawset(frameEnv[1], "CustomField", true)
        assert(element.CustomField == true, "env writes should resolve as frame fields")
        assert(element[1] == nil, "env fields must not consume numeric array slot 1")

        local button = CreateFrame("Button", nil, element)
        table.insert(element, button)
        assert(element[1] == button, "addons should be able to store array entries on frames")
        assert(type(element[1].SetSize) == "function", "array entry should remain the inserted button")
        "#,
    )
    .unwrap();
}
