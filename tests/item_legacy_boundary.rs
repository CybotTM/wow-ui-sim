use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn legacy_get_item_id_global_is_not_c_item_registration() {
    let c_item = include_str!("../src/c_api/item_spell/c_item.rs");
    let utility_registration = include_str!("../src/lua_api/globals/utility_system_spell/mod.rs");

    assert!(
        !c_item.contains("state.global, \"GetItemID\""),
        "legacy GetItemID global should not be registered from c_api::item_spell::c_item"
    );
    assert!(
        utility_registration.contains("item_legacy::register_legacy_item_globals"),
        "legacy GetItemID global should be registered from the Lua globals layer"
    );
}

#[test]
fn legacy_get_item_id_global_remains_registered() {
    let env = WowLuaEnv::new().expect("failed to create Lua environment");
    let (global_type, namespaced_type, global_id, namespaced_id): (String, String, i64, i64) = env
        .eval(
            r#"
            local link = "|cffffffff|Hitem:777::::::::80:::::|h[X]|h|r"
            return type(GetItemID), type(C_Item.GetItemID), GetItemID(link), C_Item.GetItemID(link)
            "#,
        )
        .expect("legacy item global should be registered");

    assert_eq!(global_type, "function");
    assert_eq!(namespaced_type, "function");
    assert_eq!(global_id, 777);
    assert_eq!(namespaced_id, 777);
}
