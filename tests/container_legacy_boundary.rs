use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn legacy_container_globals_are_not_c_container_registration() {
    let c_container = include_str!("../src/c_api/item_spell/c_container.rs");
    let utility_registration = include_str!("../src/lua_api/globals/utility_system_spell/mod.rs");

    assert!(
        !c_container.contains("register_legacy_container_globals")
            && !c_container.contains("state.global"),
        "legacy container globals should not be registered from c_api::item_spell::c_container"
    );
    assert!(
        utility_registration.contains("real::container_legacy::register_legacy_container_globals"),
        "legacy container globals should be registered from the Lua globals layer"
    );
}

#[test]
fn legacy_container_globals_remain_registered() {
    let env = WowLuaEnv::new().expect("failed to create Lua environment");
    let (slots_global, slots_namespaced, item_id_global, item_id_namespaced): (
        String,
        String,
        String,
        String,
    ) = env
        .eval(
            r#"
            return type(GetContainerNumSlots),
                   type(C_Container.GetContainerNumSlots),
                   type(GetContainerItemID),
                   type(C_Container.GetContainerItemID)
            "#,
        )
        .expect("legacy container globals should be registered");

    assert_eq!(slots_global, "function");
    assert_eq!(slots_namespaced, "function");
    assert_eq!(item_id_global, "function");
    assert_eq!(item_id_namespaced, "function");
}
