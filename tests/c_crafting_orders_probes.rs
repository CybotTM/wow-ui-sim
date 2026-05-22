use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn personal_orders_info_defaults_to_empty_table() {
    let env = env();
    let (value_type, count): (String, i64) = env
        .eval(
            r#"
            local personalOrders = C_CraftingOrders.GetPersonalOrdersInfo()
            return type(personalOrders), #personalOrders
            "#,
        )
        .unwrap();

    assert_eq!(value_type, "table");
    assert_eq!(count, 0);
}
