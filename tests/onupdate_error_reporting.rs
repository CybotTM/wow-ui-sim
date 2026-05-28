use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn anonymous_onupdate_errors_include_frame_owner_and_source() {
    let env = env();

    env.exec(
        r#"
        _G.__onupdate_errors = {}
        seterrorhandler(function(msg)
            table.insert(_G.__onupdate_errors, tostring(msg))
        end)

        local frame = CreateFrame("Frame")
        frame:SetScript("OnUpdate", function()
            error("onupdate boom")
        end)
        frame:Show()
        "#,
    )
    .expect("OnUpdate error fixture should load");

    env.fire_on_update(0.016)
        .expect("OnUpdate dispatch should report handler failures without failing the tick");

    let message: String = env
        .eval("return table.concat(_G.__onupdate_errors, '\\n')")
        .expect("captured error should be readable");

    assert!(
        message.contains("[OnUpdate] frame=#"),
        "anonymous frame errors should include a stable frame id, got: {message}"
    );
    assert!(
        message.contains("addon=__BuiltIn"),
        "built-in owner fallback should be explicit, got: {message}"
    );
    assert!(
        message.contains("source="),
        "handler source should be included, got: {message}"
    );
    assert!(
        message.contains("onupdate boom"),
        "original error should be preserved, got: {message}"
    );
}

#[test]
fn onupdate_handler_survives_full_gc_after_setscript() {
    let env = env();

    env.exec(
        r#"
        _G.__onupdate_ticks = 0
        _G.__onupdate_errors = {}
        seterrorhandler(function(msg)
            table.insert(_G.__onupdate_errors, tostring(msg))
        end)

        local frame = CreateFrame("Frame")
        frame:SetScript("OnUpdate", function()
            _G.__onupdate_ticks = _G.__onupdate_ticks + 1
        end)
        frame:Show()

        for _ = 1, 5 do
            collectgarbage("collect")
        end
        "#,
    )
    .expect("OnUpdate GC fixture should load");

    env.fire_on_update(0.016)
        .expect("OnUpdate dispatch should survive full GC");

    let (ticks, errors): (i32, String) = env
        .eval("return _G.__onupdate_ticks, table.concat(_G.__onupdate_errors, '\\n')")
        .expect("OnUpdate GC result should be readable");

    assert_eq!(
        1, ticks,
        "OnUpdate handler should remain callable after full GC; errors: {errors}"
    );
    assert!(
        errors.is_empty(),
        "full GC should not invalidate OnUpdate handler references, got: {errors}"
    );
}
