//! Behavior pin: `UpdateAccountState` enters the locked-post-save
//! branch when account save is enabled and the account is already
//! locked.
//!
//! Source (`Interface/BlizzardUI/Blizzard_AccountSaveUI/
//! Blizzard_AccountSaveUI.lua`, lines 58-66):
//!
//! ```lua
//! if C_AccountServices.IsAccountLockedPostSave() then
//!     self.activeState = AccountSaveFrameMixin.VisualState.EnabledLocked;
//!
//!     self.Text:SetText(HTML_START .. ACCOUNT_SAVE_DESCRIPTION_LOCKED .. HTML_END);
//!     self.Text:SetWidth(self.ContentInsets:GetWidth());
//!     self.LockEditBox:SetText("");
//!     self.LockEditBox:Hide();
//!     self.SaveButton:SetPoint("TOPLEFT", self.Text, "BOTTOMLEFT", 0, -5);
//!     self.SaveButton:SetEnabled(true);
//! ```
//!
//! The locked branch represents the "save already happened, can't
//! re-enter the lock string" state. Three observables matter:
//!   1. `activeState == VisualState.EnabledLocked` (= 2) — drives
//!      `UpdateSizing` on transition and is read by external code that
//!      probes this frame's state.
//!   2. `LockEditBox:IsShown() == false` — the lock-string input is
//!      hidden because there's no lock to confirm anymore.
//!   3. `SaveButton:IsEnabled() == true` — this branch enables the
//!      Save button unconditionally; the unlocked branch instead gates
//!      it on `DoesLockStringMatch`.
//!
//! The simulator's `account_save_enabled` and `account_locked_post_save`
//! flags default to `false`, so the test mutates `SimState` directly
//! before calling `UpdateAccountState` to drive the locked path
//! (mirrors the pattern used in `tests/account_services.rs`, the only
//! way to flip these flags — no Lua-side setter exists).

use crate::common::blizzard_addon_harness::with_blizzard_addon_glue_smoke_shape;

const ROOT: &str = "Blizzard_AccountSaveUI";

#[test]
fn update_account_state_enters_locked_branch_when_save_enabled_and_locked() {
    with_blizzard_addon_glue_smoke_shape(&[ROOT], &[], |env, _loaded| {
        {
            let mut state = env.state().borrow_mut();
            state.account_save_enabled = true;
            state.account_locked_post_save = true;
            state.account_save_in_progress = false;
        }

        let (active_state, lock_edit_box_shown, save_button_enabled) = env
            .eval::<(i64, bool, bool)>(
                r#"
                assert(AccountSaveFrame, "AccountSaveFrame global must exist after Blizzard_AccountSaveUI load")
                AccountSaveFrame.LockEditBox:Show()
                AccountSaveFrame.SaveButton:SetEnabled(false)
                AccountSaveFrame:UpdateAccountState()
                return AccountSaveFrame.activeState,
                       AccountSaveFrame.LockEditBox:IsShown(),
                       AccountSaveFrame.SaveButton:IsEnabled()
                "#,
            )
            .expect("UpdateAccountState probe must run cleanly under the locked precondition");

        let expected_enabled_locked: i64 = 2;
        assert_eq!(
            active_state, expected_enabled_locked,
            "AccountSaveFrame.activeState must be stamped with `VisualState.EnabledLocked` (= 2) \
             when UpdateAccountState runs with IsAccountSaveEnabled() == true and \
             IsAccountLockedPostSave() == true (Blizzard_AccountSaveUI.lua:59). External code \
             reads activeState to decide whether to call UpdateSizing on transition; if this \
             write regresses, the next state change skips the resize. Got activeState = \
             {active_state} (expected {expected_enabled_locked} = EnabledLocked)."
        );

        assert!(
            !lock_edit_box_shown,
            "AccountSaveFrame.LockEditBox must be hidden in the locked branch \
             (Blizzard_AccountSaveUI.lua:64). The probe explicitly Show()s the editbox before \
             calling UpdateAccountState — a `false` here proves the locked branch actually \
             called Hide() rather than the editbox being incidentally hidden by some prior \
             step. Locked state means the save already happened, so there's no lock string \
             left to confirm. Got IsShown = {lock_edit_box_shown}."
        );

        assert!(
            save_button_enabled,
            "AccountSaveFrame.SaveButton must be enabled in the locked branch \
             (Blizzard_AccountSaveUI.lua:66 — `SetEnabled(true)` unconditional). The probe \
             explicitly SetEnabled(false)s the button before calling UpdateAccountState — a \
             `true` here proves the locked branch flipped it back on rather than the button \
             being incidentally enabled. The unlocked branch instead gates this on \
             `DoesLockStringMatch`, so a regression that conflates the two branches would \
             flip this assertion. Got IsEnabled = {save_button_enabled}."
        );
    });
}
