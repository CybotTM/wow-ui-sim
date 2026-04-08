use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn prototype_dialog_select_option_creates_dialog_state() {
    let env = env();
    let (ok, selected_option, selection_count, removed): (bool, i32, i32, bool) = env
        .eval(
            r#"
            local ok = C_PrototypeDialog.SelectOption(10, 2)
            local state = C_PrototypeDialog._activeDialogs[10]
            return ok, state.selectedOptionID, state.selectionCount, C_PrototypeDialog._removedDialogs[10] == true
            "#,
        )
        .unwrap();

    assert!(ok, "SelectOption should succeed for valid inputs");
    assert_eq!(selected_option, 2, "selected option should be tracked");
    assert_eq!(
        selection_count, 1,
        "first selection should start at count 1"
    );
    assert!(!removed, "active dialogs should not be marked removed");
}

#[test]
fn prototype_dialog_ensure_removed_transitions_state() {
    let env = env();
    let (had_active_dialog, active_cleared, removed_marked, last_transition_removed): (
        bool,
        bool,
        bool,
        bool,
    ) = env
        .eval(
            r#"
            C_PrototypeDialog.SelectOption(20, 1)
            local hadActiveDialog = C_PrototypeDialog.EnsureRemoved(20)
            local activeCleared = C_PrototypeDialog._activeDialogs[20] == nil
            local removedMarked = C_PrototypeDialog._removedDialogs[20] == true
            local last = C_PrototypeDialog._transitionHistory[#C_PrototypeDialog._transitionHistory]
            local lastTransitionRemoved = last and last.transition == "removed" and last.dialogID == 20
            return hadActiveDialog, activeCleared, removedMarked, lastTransitionRemoved
            "#,
        )
        .unwrap();

    assert!(
        had_active_dialog,
        "EnsureRemoved should report true when active dialog existed"
    );
    assert!(active_cleared, "EnsureRemoved should clear active state");
    assert!(
        removed_marked,
        "EnsureRemoved should mark dialog as removed"
    );
    assert!(
        last_transition_removed,
        "transition history should record removal transition"
    );
}

#[test]
fn prototype_dialog_reselect_after_removal_reopens_dialog() {
    let env = env();
    let (ok, selected_option, selection_count, removed_marked): (bool, i32, i32, bool) = env
        .eval(
            r#"
            C_PrototypeDialog.SelectOption(30, 4)
            C_PrototypeDialog.EnsureRemoved(30)
            local ok = C_PrototypeDialog.SelectOption(30, 7)
            local state = C_PrototypeDialog._activeDialogs[30]
            local removedMarked = C_PrototypeDialog._removedDialogs[30] == true
            return ok, state.selectedOptionID, state.selectionCount, removedMarked
            "#,
        )
        .unwrap();

    assert!(ok, "SelectOption should reopen a previously removed dialog");
    assert_eq!(
        selected_option, 7,
        "new selection should replace prior option"
    );
    assert_eq!(
        selection_count, 1,
        "reopened dialog should start fresh selection count"
    );
    assert!(
        !removed_marked,
        "reselected dialog should clear removed marker"
    );
}

#[test]
fn prototype_dialog_rejects_invalid_inputs() {
    let env = env();
    let (select_bad_dialog, select_bad_option, remove_bad_dialog): (bool, bool, bool) = env
        .eval(
            r#"
            local selectBadDialog = C_PrototypeDialog.SelectOption({}, 1)
            local selectBadOption = C_PrototypeDialog.SelectOption(40, {})
            local removeBadDialog = C_PrototypeDialog.EnsureRemoved({})
            return selectBadDialog, selectBadOption, removeBadDialog
            "#,
        )
        .unwrap();

    assert!(!select_bad_dialog, "invalid dialog IDs should be rejected");
    assert!(!select_bad_option, "invalid option IDs should be rejected");
    assert!(
        !remove_bad_dialog,
        "EnsureRemoved should reject invalid dialog IDs"
    );
}
