use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn addon_tainted_function_is_not_secret() {
    let env = env();
    let (secret, inserted): (bool, bool) = env
        .eval(
            r#"
            local f = function() end
            debug.setobjecttaint(f, "TestAddon")
            local array = SecureTypes.CreateSecureArray()
            local ok = pcall(function() array:Insert(f) end)
            return issecretvalue(f), ok and array[1] == f
            "#,
        )
        .unwrap();

    assert!(!secret, "addon-tainted closures should not be secret values");
    assert!(inserted, "SecureArray should accept addon-tainted closures");
}

#[test]
fn secure_loadstring_function_is_not_secret() {
    let env = env();
    let (secret, inserted): (bool, bool) = env
        .eval(
            r#"
            local f = loadstring("return 1")
            local array = SecureTypes.CreateSecureArray()
            local ok = pcall(function() array:Insert(f) end)
            return issecretvalue(f), ok and array[1] == f
            "#,
        )
        .unwrap();

    assert!(!secret, "secure loadstring closures should not be secret values");
    assert!(inserted, "SecureArray should accept secure generated closures");
}
