use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn account_services_flags_are_state_backed() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.account_save_enabled = true;
        state.account_save_in_progress = true;
        state.account_locked_post_save = true;
    }

    let (enabled, in_progress, locked): (bool, bool, bool) = env
        .eval(
            r#"
            return C_AccountServices.IsAccountSaveEnabled(),
                   C_AccountServices.IsAccountSaveInProgress(),
                   C_AccountServices.IsAccountLockedPostSave()
            "#,
        )
        .unwrap();

    assert!(enabled, "enabled flag should come from simulator state");
    assert!(
        in_progress,
        "in-progress flag should come from simulator state"
    );
    assert!(
        locked,
        "locked-post-save flag should come from simulator state"
    );
}

#[test]
fn save_account_data_returns_unavailable_when_disabled() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.account_save_enabled = false;
        state.account_save_in_progress = false;
        state.account_locked_post_save = false;
    }

    let (started, result_code, in_progress, locked): (bool, i32, bool, bool) = env
        .eval(
            r#"
            local started, resultCode = C_AccountServices.SaveAccountData()
            return started, resultCode,
                   C_AccountServices.IsAccountSaveInProgress(),
                   C_AccountServices.IsAccountLockedPostSave()
            "#,
        )
        .unwrap();

    assert!(!started, "disabled account save should fail to start");
    assert_eq!(result_code, 10, "disabled save should return Unavailable");
    assert!(
        !in_progress,
        "disabled save should not flip in-progress state on"
    );
    assert!(!locked, "disabled save should not lock the account");
}

#[test]
fn save_account_data_returns_already_in_progress_when_busy() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.account_save_enabled = true;
        state.account_save_in_progress = true;
        state.account_locked_post_save = false;
    }

    let (started, result_code, in_progress, locked): (bool, i32, bool, bool) = env
        .eval(
            r#"
            local started, resultCode = C_AccountServices.SaveAccountData()
            return started, resultCode,
                   C_AccountServices.IsAccountSaveInProgress(),
                   C_AccountServices.IsAccountLockedPostSave()
            "#,
        )
        .unwrap();

    assert!(
        !started,
        "save should fail while one is already in progress"
    );
    assert_eq!(result_code, 11, "busy save should return AlreadyInProgress");
    assert!(in_progress, "in-progress state should remain true");
    assert!(!locked, "busy save should not lock the account");
}

#[test]
fn save_account_data_starts_and_locks_on_success() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.account_save_enabled = true;
        state.account_save_in_progress = false;
        state.account_locked_post_save = false;
    }

    let (started, result_code, in_progress, locked): (bool, i32, bool, bool) = env
        .eval(
            r#"
            local started, resultCode = C_AccountServices.SaveAccountData()
            return started, resultCode,
                   C_AccountServices.IsAccountSaveInProgress(),
                   C_AccountServices.IsAccountLockedPostSave()
            "#,
        )
        .unwrap();

    assert!(started, "enabled save should start");
    assert_eq!(result_code, 0, "successful save should return Success");
    assert!(
        !in_progress,
        "stubbed save completes immediately and should not remain in-progress"
    );
    assert!(locked, "successful save should lock the account");
}

#[test]
fn account_save_result_event_is_registerable() {
    // Blizzard_AccountSaveUI.lua line 42: AccountSaveFrame:RegisterEvent("ACCOUNT_SAVE_RESULT").
    // Without this in the valid-events list, RegisterEvent rejects it with
    // "Attempt to register unknown event" and the addon fails to wire its handler.
    let env = env();
    let (registered, error): (bool, Option<String>) = env
        .eval(
            r#"
            local f = CreateFrame("Frame")
            local ok, err = pcall(function() f:RegisterEvent("ACCOUNT_SAVE_RESULT") end)
            return ok, ok and nil or tostring(err)
            "#,
        )
        .unwrap();
    assert!(
        registered,
        "ACCOUNT_SAVE_RESULT should be registerable; error: {error:?}"
    );
}

#[test]
fn account_save_result_event_dispatches_payload_to_handler() {
    // Per Blizzard_AccountSaveUI.lua line 128, the event payload is
    // (result: Enum.AccountExportResult, outputFolderPath: string, outputFilePath: string).
    let env = env();
    let (received, result, folder, file): (bool, i32, String, String) = env
        .eval(
            r#"
            local got_result, got_folder, got_file
            local received = false
            local f = CreateFrame("Frame")
            f:RegisterEvent("ACCOUNT_SAVE_RESULT")
            f:SetScript("OnEvent", function(self, event, result, folder, file)
                if event == "ACCOUNT_SAVE_RESULT" then
                    received = true
                    got_result, got_folder, got_file = result, folder, file
                end
            end)
            FireEvent("ACCOUNT_SAVE_RESULT", Enum.AccountExportResult.Success, "/tmp/saves", "save.zip")
            return received, got_result, got_folder, got_file
            "#,
        )
        .unwrap();
    assert!(received, "OnEvent should fire for ACCOUNT_SAVE_RESULT");
    assert_eq!(result, 0, "first payload arg should be Enum.AccountExportResult.Success (0)");
    assert_eq!(folder, "/tmp/saves");
    assert_eq!(file, "save.zip");
}
