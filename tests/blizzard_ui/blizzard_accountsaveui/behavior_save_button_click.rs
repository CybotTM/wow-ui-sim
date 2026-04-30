//! Behavior pin: `OnSaveButtonClicked` gates on the button's enabled
//! state, and `SaveAccountData` routes failures to
//! `ProcessAccountSaveError`.
//!
//! Source (`Interface/BlizzardUI/Blizzard_AccountSaveUI/
//! Blizzard_AccountSaveUI.lua`, lines 105-123):
//!
//! ```lua
//! function AccountSaveFrameMixin:OnSaveButtonClicked()
//!     if self.SaveButton:IsEnabled() then
//!         self:SaveAccountData();
//!     end
//! end
//!
//! function AccountSaveFrameMixin:SaveAccountData()
//!     local startedSuccessfully, errorCode = C_AccountServices.SaveAccountData();
//!     if not startedSuccessfully then
//!         self:ProcessAccountSaveError(errorCode);
//!     end
//!     self:UpdateAccountState();
//! end
//! ```
//!
//! Two contracts are pinned:
//!   1. The click handler is gated on `SaveButton:IsEnabled()`. A
//!      disabled button must never reach `SaveAccountData`. Without
//!      this gate, the player could trigger a save by sending a
//!      synthetic click to a button the UI clearly shows as inactive.
//!   2. When `C_AccountServices.SaveAccountData()` returns `false`,
//!      the addon must call `ProcessAccountSaveError(errorCode)`. The
//!      simulator's C API returns `(false, AlreadyInProgress=11)` when
//!      `account_save_in_progress` is true on SimState; the test
//!      drives this exact path so the routing can be verified
//!      end-to-end.
//!
//! Both observations require Lua-side wrapping. `C_AccountServices` is
//! a regular Lua table, so reassigning `C_AccountServices.SaveAccountData`
//! cleanly intercepts. `AccountSaveFrame.ProcessAccountSaveError` is a
//! method copied onto the userdata's per-instance fenv table by the
//! `Mixin` helper (shared_bootstrap.lua:1-13 — `object[k] = v`),
//! reachable through `__newindex`; reassigning the same key shadows
//! the inherited method for subsequent dispatch.

use crate::common::blizzard_addon_harness::with_blizzard_addon_glue_smoke_shape;

const ROOT: &str = "Blizzard_AccountSaveUI";
const ALREADY_IN_PROGRESS: i64 = 11;

#[test]
fn on_save_button_clicked_gates_save_call_on_button_enabled_state() {
    with_blizzard_addon_glue_smoke_shape(&[ROOT], &[], |env, _loaded| {
        {
            let mut state = env.state().borrow_mut();
            state.account_save_enabled = true;
            state.account_locked_post_save = false;
            state.account_save_in_progress = false;
        }

        let (save_called_when_disabled, save_called_when_enabled) = env
            .eval::<(bool, bool)>(
                r#"
                assert(AccountSaveFrame, "AccountSaveFrame global must exist after Blizzard_AccountSaveUI load")

                local save_call_count = 0
                local original_save = C_AccountServices.SaveAccountData
                C_AccountServices.SaveAccountData = function(...)
                    save_call_count = save_call_count + 1
                    return original_save(...)
                end

                AccountSaveFrame.SaveButton:SetEnabled(false)
                AccountSaveFrame:OnSaveButtonClicked()
                local called_when_disabled = save_call_count > 0

                save_call_count = 0
                AccountSaveFrame.SaveButton:SetEnabled(true)
                AccountSaveFrame:OnSaveButtonClicked()
                local called_when_enabled = save_call_count > 0

                return called_when_disabled, called_when_enabled
                "#,
            )
            .expect("OnSaveButtonClicked gating probe must run cleanly");

        assert!(
            !save_called_when_disabled,
            "C_AccountServices.SaveAccountData must NOT be called when SaveButton is disabled \
             (Blizzard_AccountSaveUI.lua:106 — `if self.SaveButton:IsEnabled() then`). Without \
             this gate, a synthetic click on a disabled button would still trigger a save \
             attempt. The probe wraps SaveAccountData in a counting closure and asserts the \
             count stays at zero after OnSaveButtonClicked runs against a disabled button. \
             Got called_when_disabled = {save_called_when_disabled}."
        );

        assert!(
            save_called_when_enabled,
            "C_AccountServices.SaveAccountData must be called when SaveButton is enabled \
             (Blizzard_AccountSaveUI.lua:107 — `self:SaveAccountData()`). The probe explicitly \
             SetEnabled(true)s the button before calling OnSaveButtonClicked; a `false` here \
             would mean the click handler stopped delegating to SaveAccountData entirely. Got \
             called_when_enabled = {save_called_when_enabled}."
        );
    });
}

#[test]
fn save_account_data_invokes_process_error_on_c_api_failure() {
    with_blizzard_addon_glue_smoke_shape(&[ROOT], &[], |env, _loaded| {
        {
            let mut state = env.state().borrow_mut();
            state.account_save_enabled = true;
            state.account_locked_post_save = false;
            state.account_save_in_progress = true;
        }

        let (save_called, process_called, process_error_code) = env
            .eval::<(bool, bool, i64)>(
                r#"
                assert(AccountSaveFrame, "AccountSaveFrame global must exist after Blizzard_AccountSaveUI load")

                local save_call_count = 0
                local original_save = C_AccountServices.SaveAccountData
                C_AccountServices.SaveAccountData = function(...)
                    save_call_count = save_call_count + 1
                    return original_save(...)
                end

                local process_call_count = 0
                local last_process_code = -1
                local original_process = AccountSaveFrame.ProcessAccountSaveError
                AccountSaveFrame.ProcessAccountSaveError = function(self, errorCode)
                    process_call_count = process_call_count + 1
                    last_process_code = errorCode
                    return original_process(self, errorCode)
                end

                AccountSaveFrame.SaveButton:SetEnabled(true)
                AccountSaveFrame:OnSaveButtonClicked()

                return save_call_count > 0,
                       process_call_count > 0,
                       last_process_code
                "#,
            )
            .expect("Save+ProcessError routing probe must run cleanly");

        assert!(
            save_called,
            "C_AccountServices.SaveAccountData must be called by SaveAccountData \
             (Blizzard_AccountSaveUI.lua:116). Without this, the failure-routing assertion \
             below would also fail but for the wrong reason; checking save_called first \
             isolates the C API call from the error routing."
        );

        assert!(
            process_called,
            "AccountSaveFrame:ProcessAccountSaveError must be called when \
             C_AccountServices.SaveAccountData() returns `false` \
             (Blizzard_AccountSaveUI.lua:118-120). The simulator returns \
             `(false, AlreadyInProgress=11)` when `account_save_in_progress` is true on \
             SimState; the test drives this precondition to force the failure path. A `false` \
             here means the `if not startedSuccessfully` guard either dropped the call or got \
             inverted. Got process_called = {process_called}."
        );

        assert_eq!(
            process_error_code, ALREADY_IN_PROGRESS,
            "ProcessAccountSaveError must receive the errorCode returned by \
             C_AccountServices.SaveAccountData (Blizzard_AccountSaveUI.lua:119 — \
             `self:ProcessAccountSaveError(errorCode)`). The simulator returns \
             `Enum.AccountExportResult.AlreadyInProgress = 11` when \
             `account_save_in_progress` is true. A different value here means the addon \
             dropped the errorCode argument or swapped it for something else \
             (e.g. the truthy-failure flag). Got errorCode = {process_error_code}, expected \
             {ALREADY_IN_PROGRESS}."
        );
    });
}
