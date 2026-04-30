//! Behavior pin: `DoesLockStringMatch` is the gate that drives the
//! SaveButton enabled state through `OnLockEditBoxTextChanged`.
//!
//! Source (`Interface/BlizzardUI/Blizzard_AccountSaveUI/
//! Blizzard_AccountSaveUI.lua`, lines 91-93 and 111-113):
//!
//! ```lua
//! function AccountSaveFrameMixin:OnLockEditBoxTextChanged()
//!     self.SaveButton:SetEnabled(self:DoesLockStringMatch());
//! end
//!
//! function AccountSaveFrameMixin:DoesLockStringMatch()
//!     return ConfirmationEditBoxMatches(self.LockEditBox, ACCOUNT_SAVE_CONFIRM_STRING);
//! end
//! ```
//!
//! `ConfirmationEditBoxMatches` (Blizzard_SharedXML/StringUtil.lua:5-7)
//! delegates to `ConfirmationStringMatches`, which compares via
//! `strupper(...)` — case-insensitive equality. The gate therefore has
//! four interesting cases:
//!   1. empty editbox → mismatch → button disabled
//!   2. wrong text   → mismatch → button disabled
//!   3. exact match  → match    → button enabled
//!   4. case-flipped → match    → button enabled (proves `strupper`
//!      normalisation is exercised, not a literal `==` comparison)
//!
//! The unlocked-branch fixture (`behavior_account_state_unlocked`)
//! already pins cases (1) and (3) via `UpdateAccountState`. This file
//! pins the gate via `OnLockEditBoxTextChanged` directly — the path
//! the EditBox runs every keystroke — and adds case-flipping (4) plus
//! a deliberately-wrong string (2) to lock down the case-insensitive
//! comparator. Each scenario captures both `DoesLockStringMatch()`'s
//! return and the resulting `SaveButton:IsEnabled()` so a regression
//! that fixes the matcher but breaks the propagation (or vice versa)
//! reports independently.

use crate::common::blizzard_addon_harness::with_blizzard_addon_glue_smoke_shape;

const ROOT: &str = "Blizzard_AccountSaveUI";

#[test]
fn lock_string_match_drives_save_button_through_on_text_changed() {
    with_blizzard_addon_glue_smoke_shape(&[ROOT], &[], |env, _loaded| {
        {
            let mut state = env.state().borrow_mut();
            state.account_save_enabled = true;
            state.account_locked_post_save = false;
            state.account_save_in_progress = false;
        }

        let (
            match_empty,
            button_empty,
            match_wrong,
            button_wrong,
            match_exact,
            button_exact,
            match_lower,
            button_lower,
        ) = env
            .eval::<(bool, bool, bool, bool, bool, bool, bool, bool)>(
                r#"
                assert(AccountSaveFrame, "AccountSaveFrame global must exist after Blizzard_AccountSaveUI load")
                assert(ACCOUNT_SAVE_CONFIRM_STRING == "LOCK ACCOUNT",
                       "ACCOUNT_SAVE_CONFIRM_STRING must equal `LOCK ACCOUNT` (data/global_strings.rs)")

                AccountSaveFrame:UpdateAccountState()

                local function probe(text)
                    AccountSaveFrame.LockEditBox:SetText(text)
                    AccountSaveFrame:OnLockEditBoxTextChanged()
                    return AccountSaveFrame:DoesLockStringMatch(),
                           AccountSaveFrame.SaveButton:IsEnabled()
                end

                local m_empty, b_empty = probe("")
                local m_wrong, b_wrong = probe("not the right phrase")
                local m_exact, b_exact = probe("LOCK ACCOUNT")
                local m_lower, b_lower = probe("lock account")

                return m_empty, b_empty,
                       m_wrong, b_wrong,
                       m_exact, b_exact,
                       m_lower, b_lower
                "#,
            )
            .expect("DoesLockStringMatch / OnLockEditBoxTextChanged probe must run cleanly");

        assert!(
            !match_empty,
            "DoesLockStringMatch() must return false on an empty editbox \
             (Blizzard_AccountSaveUI.lua:111-113 → Blizzard_SharedXML/StringUtil.lua:1-3 → \
             `strupper(\"\") ~= strupper(\"LOCK ACCOUNT\")`). Got match = {match_empty}."
        );
        assert!(
            !button_empty,
            "SaveButton must be disabled after OnLockEditBoxTextChanged with empty text \
             (Blizzard_AccountSaveUI.lua:91-93 → SetEnabled(false)). Got IsEnabled = {button_empty}."
        );

        assert!(
            !match_wrong,
            "DoesLockStringMatch() must return false on a non-matching phrase. The case-\
             insensitive comparator must still reject text that doesn't normalise to \
             `\"LOCK ACCOUNT\"`. Got match = {match_wrong} for input = `\"not the right phrase\"`."
        );
        assert!(
            !button_wrong,
            "SaveButton must be disabled after OnLockEditBoxTextChanged with non-matching text. \
             Got IsEnabled = {button_wrong}."
        );

        assert!(
            match_exact,
            "DoesLockStringMatch() must return true when editbox text is exactly \
             `\"LOCK ACCOUNT\"`. Got match = {match_exact}."
        );
        assert!(
            button_exact,
            "SaveButton must be enabled after OnLockEditBoxTextChanged with exact-match text. \
             Got IsEnabled = {button_exact}."
        );

        assert!(
            match_lower,
            "DoesLockStringMatch() must return true when editbox text is `\"lock account\"` \
             (case-flipped). The matcher delegates through `ConfirmationEditBoxMatches` to \
             `strupper` (Blizzard_SharedXML/StringUtil.lua:1-3), so case differences must be \
             erased — a regression that swapped the comparator for a literal `==` would flip \
             this assertion while leaving the exact-match case passing. Got match = {match_lower}."
        );
        assert!(
            button_lower,
            "SaveButton must be enabled after OnLockEditBoxTextChanged with case-flipped \
             matching text. Got IsEnabled = {button_lower}."
        );
    });
}
