use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn startup_closes_open_windows_before_first_frame() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.exec(
        r#"
        StartupSpecialFrameProbe = CreateFrame("Frame", "StartupSpecialFrameProbe", UIParent)
        StartupSpecialFrameProbe:Show()

        startupCloseCalled = false
        startupFirstFrameSawHidden = false

        function CloseAllWindows(ignoreCenter)
            if ignoreCenter == 1 then
                startupCloseCalled = true
                StartupSpecialFrameProbe:Hide()
            end
            return true
        end

        local watcher = CreateFrame("Frame")
        watcher:RegisterEvent("FIRST_FRAME_RENDERED")
        watcher:SetScript("OnEvent", function()
            startupFirstFrameSawHidden = not StartupSpecialFrameProbe:IsShown()
        end)
        "#,
    )
    .unwrap();

    wow_ui_sim::startup::fire_startup_events(&env);

    let close_called: bool = env.eval("return startupCloseCalled").unwrap();
    let first_frame_saw_hidden: bool = env.eval("return startupFirstFrameSawHidden").unwrap();
    let still_shown: bool = env
        .eval("return StartupSpecialFrameProbe:IsShown()")
        .unwrap();

    assert!(close_called, "startup should close pre-open windows");
    assert!(
        first_frame_saw_hidden,
        "windows should be hidden before FIRST_FRAME_RENDERED"
    );
    assert!(!still_shown, "startup special frame should end hidden");
}
