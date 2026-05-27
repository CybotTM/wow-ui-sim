use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn legacy_spell_flyout_globals_are_not_c_api_registration() {
    let c_spell_book = include_str!("../src/c_api/c_spell_book.rs");
    let utility_registration = include_str!("../src/lua_api/globals/utility_system_spell/mod.rs");

    assert!(
        !c_spell_book.contains("state.global, \"GetFlyoutInfo\"")
            && !c_spell_book.contains("state.global, \"GetFlyoutSlotInfo\""),
        "legacy flyout globals should not be registered from c_api::c_spell_book"
    );
    assert!(
        utility_registration
            .contains("spell_flyout_legacy::register_legacy_spell_flyout_globals"),
        "legacy flyout globals should be registered from the Lua globals layer"
    );
}

#[test]
fn legacy_spell_flyout_globals_remain_registered() {
    let env = WowLuaEnv::new().expect("failed to create Lua environment");
    let (global_type, namespaced_type): (String, String) = env
        .eval(
            r#"
            return type(GetFlyoutInfo), type(C_SpellBook.GetFlyoutInfo)
            "#,
        )
        .expect("legacy flyout globals should be registered");

    assert_eq!(global_type, "function");
    assert_eq!(namespaced_type, "function");
}
