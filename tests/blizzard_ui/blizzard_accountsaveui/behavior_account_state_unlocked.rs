//! Behavior pin: `UpdateAccountState` enters the unlocked branch when
//! account save is enabled and the account is not yet locked.
//!
//! Source (`Interface/BlizzardUI/Blizzard_AccountSaveUI/
//! Blizzard_AccountSaveUI.lua`, lines 67-75):
//!
//! ```lua
//! else
//!     self.activeState = AccountSaveFrameMixin.VisualState.EnabledUnlocked;
//!
//!     self.Text:SetText(HTML_START .. ACCOUNT_SAVE_DESCRIPTION_UNLOCKED .. HTML_END);
//!     self.Text:SetWidth(self.ContentInsets:GetWidth());
//!     self.LockEditBox:Show();
//!     self.SaveButton:SetPoint("TOPLEFT", self.LockEditBox, "BOTTOMLEFT", -10, 0);
//!     self.SaveButton:SetEnabled(self:DoesLockStringMatch());
//! end
//! ```
//!
//! `DoesLockStringMatch` (line 111-113):
//!
//! ```lua
//! function AccountSaveFrameMixin:DoesLockStringMatch()
//!     return ConfirmationEditBoxMatches(self.LockEditBox, ACCOUNT_SAVE_CONFIRM_STRING);
//! end
//! ```
//!
//! `ConfirmationEditBoxMatches` (Blizzard_SharedXML/StringUtil.lua:5-7)
//! does a case-insensitive comparison via `strupper`. The simulator
//! loads `ACCOUNT_SAVE_CONFIRM_STRING = "LOCK ACCOUNT"` from
//! `data/global_strings.rs` via `register_all_ui_strings`, so the
//! match check is fully exercised by setting the editbox text.
//!
//! Three observables are pinned for the unlocked branch:
//!   1. `activeState == VisualState.EnabledUnlocked` (= 3) — drives
//!      `UpdateSizing` on transition.
//!   2. `LockEditBox:IsShown() == true` — the lock-string input has to
//!      be visible so the player can type the confirmation phrase.
//!   3. `SaveButton:IsEnabled()` follows `DoesLockStringMatch()`. Both
//!      directions are pinned within the same test:
//!        - empty editbox text → no match → button disabled
//!        - text == "LOCK ACCOUNT" → matches → button enabled
//!
//! Capturing both directions in one test avoids loading the addon
//! twice and makes the pairing explicit: a regression that always
//! returns true (or always false) from `DoesLockStringMatch` would
//! flip exactly one of the two button assertions.

use crate::common::blizzard_addon_harness::with_blizzard_addon_glue_smoke_shape;

const ROOT: &str = "Blizzard_AccountSaveUI";

#[test]
fn update_account_state_unlocked_branch_pins_state_and_button_follows_match() {
    with_blizzard_addon_glue_smoke_shape(&[ROOT], &[], |env, _loaded| {
        {
            let mut state = env.state().borrow_mut();
            state.account_save_enabled = true;
            state.account_locked_post_save = false;
            state.account_save_in_progress = false;
        }

        let (active_state, edit_box_shown, button_enabled_no_match, button_enabled_match) = env
            .eval::<(i64, bool, bool, bool)>(
                r#"
                assert(AccountSaveFrame, "AccountSaveFrame global must exist after Blizzard_AccountSaveUI load")
                assert(ACCOUNT_SAVE_CONFIRM_STRING == "LOCK ACCOUNT",
                       "ACCOUNT_SAVE_CONFIRM_STRING must equal `LOCK ACCOUNT` (loaded from data/global_strings.rs)")

                AccountSaveFrame.LockEditBox:Hide()
                AccountSaveFrame.LockEditBox:SetText("")
                AccountSaveFrame:UpdateAccountState()
                local active = AccountSaveFrame.activeState
                local shown = AccountSaveFrame.LockEditBox:IsShown()
                local btn_no_match = AccountSaveFrame.SaveButton:IsEnabled()

                AccountSaveFrame.LockEditBox:SetText(ACCOUNT_SAVE_CONFIRM_STRING)
                AccountSaveFrame:UpdateAccountState()
                local btn_match = AccountSaveFrame.SaveButton:IsEnabled()

                return active, shown, btn_no_match, btn_match
                "#,
            )
            .expect("UpdateAccountState probe must run cleanly under the unlocked precondition");

        let expected_enabled_unlocked: i64 = 3;
        assert_eq!(
            active_state, expected_enabled_unlocked,
            "AccountSaveFrame.activeState must be stamped with `VisualState.EnabledUnlocked` \
             (= 3) when UpdateAccountState runs with IsAccountSaveEnabled() == true and \
             IsAccountLockedPostSave() == false (Blizzard_AccountSaveUI.lua:68). External code \
             reads activeState to decide whether to call UpdateSizing on transition; if this \
             write regresses, the next state change skips the resize. Got activeState = \
             {active_state} (expected {expected_enabled_unlocked} = EnabledUnlocked)."
        );

        assert!(
            edit_box_shown,
            "AccountSaveFrame.LockEditBox must be shown in the unlocked branch \
             (Blizzard_AccountSaveUI.lua:72). The probe explicitly Hide()s the editbox before \
             calling UpdateAccountState — a `true` here proves the unlocked branch actually \
             called Show() rather than the editbox being incidentally visible. Without the \
             editbox visible, the player has no way to type the confirmation phrase. Got \
             IsShown = {edit_box_shown}."
        );

        assert!(
            !button_enabled_no_match,
            "AccountSaveFrame.SaveButton must be DISABLED when LockEditBox text does not \
             match `ACCOUNT_SAVE_CONFIRM_STRING` (Blizzard_AccountSaveUI.lua:74 — \
             `SetEnabled(self:DoesLockStringMatch())`). With empty editbox text, \
             `ConfirmationEditBoxMatches` returns false (`strupper(\"\") ~= strupper(\"LOCK ACCOUNT\")`), \
             so the button must be disabled. Got IsEnabled = {button_enabled_no_match} — a \
             regression that always returns true from DoesLockStringMatch (or hard-codes \
             SetEnabled(true)) would flip this assertion."
        );

        assert!(
            button_enabled_match,
            "AccountSaveFrame.SaveButton must be ENABLED when LockEditBox text matches \
             `ACCOUNT_SAVE_CONFIRM_STRING` (Blizzard_AccountSaveUI.lua:74). After setting the \
             editbox text to `\"LOCK ACCOUNT\"`, `ConfirmationEditBoxMatches` returns true and \
             the button must be enabled. Got IsEnabled = {button_enabled_match} — a regression \
             that always returns false from DoesLockStringMatch (or hard-codes \
             SetEnabled(false)) would flip this assertion."
        );
    });
}
