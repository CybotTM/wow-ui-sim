use wow_ui_sim::lua_api::WowLuaEnv;

fn seed_bitfield_state(env: &WowLuaEnv) {
    env.exec(
        r#"
        TEST_INITIAL_BIT3 = GetCVarBitfield("closedInfoFramesAccountWide", 3)
        SetCVarBitfield("closedInfoFramesAccountWide", 3, true)
        TEST_AFTER_SET = GetCVarBitfield("closedInfoFramesAccountWide", 3)
        SetCVarBitfield("closedInfoFramesAccountWide", 3, false)
        TEST_AFTER_CLEAR = GetCVarBitfield("closedInfoFramesAccountWide", 3)
        "#,
    )
    .unwrap();
}

#[test]
fn cvar_bitfields_can_be_set_and_cleared() {
    let env = WowLuaEnv::new().unwrap();
    seed_bitfield_state(&env);

    let initial: bool = env.eval("return TEST_INITIAL_BIT3").unwrap();
    let after_set: bool = env.eval("return TEST_AFTER_SET").unwrap();
    let after_clear: bool = env.eval("return TEST_AFTER_CLEAR").unwrap();

    assert!(!initial, "bit 3 should start unset");
    assert!(after_set, "bit 3 should be set after SetCVarBitfield");
    assert!(!after_clear, "bit 3 should be clear after clearing");
}

#[test]
fn c_cvar_bitfields_share_the_same_storage() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        C_CVar.SetCVarBitfield("closedInfoFramesAccountWide", 3, true)
        TEST_GLOBAL_READ = GetCVarBitfield("closedInfoFramesAccountWide", 3)
        SetCVarBitfield("closedInfoFramesAccountWide", 3, false)
        TEST_NAMESPACE_READ = C_CVar.GetCVarBitfield("closedInfoFramesAccountWide", 3)
        "#,
    )
    .unwrap();

    let global_read: bool = env.eval("return TEST_GLOBAL_READ").unwrap();
    let namespace_read: bool = env.eval("return TEST_NAMESPACE_READ").unwrap();

    assert!(global_read);
    assert!(!namespace_read);
}
