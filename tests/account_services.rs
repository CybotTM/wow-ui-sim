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
