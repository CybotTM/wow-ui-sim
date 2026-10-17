use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn host_error_unwind_preserves_captured_helper() {
    let env = WowLuaEnv::new().unwrap();
    let error = env
        .exec(
            r#"
            local function helper() return { "CharacterHeadSlot" } end
            saved = function() return helper() end
            error("deliberate")
            "#,
        )
        .unwrap_err();
    assert!(error.to_string().contains("deliberate"), "{error}");

    env.exec(
        r#"
        local function provider() return nil end
        other = function() return provider() end
        "#,
    )
    .unwrap();
    env.exec(
        r#"
        local value = saved()
        assert(type(value) == "table", "saved helper must return its original table")
        assert(value[1] == "CharacterHeadSlot", "saved helper must retain its original value")
        "#,
    )
    .unwrap();
}

#[test]
fn missing_global_before_loop_reports_global_name() {
    let env = WowLuaEnv::new().unwrap();
    let error = env
        .exec(
            r#"
            MissingForGeneratorRepro.field()
            for index, value in ipairs({}) do end
            "#,
        )
        .unwrap_err();
    let message = error.to_string();
    assert!(
        message.contains("attempt to index global 'MissingForGeneratorRepro' (a nil value)"),
        "{message}"
    );
    assert!(!message.contains("(for generator)"), "{message}");
}
