//! Behavior pin: pressing Enter in `LockEditBox` triggers
//! `SaveAccountData` only when `DoesLockStringMatch` is true; pressing
//! Escape clears focus on the editbox.
//!
//! Source (`Interface/BlizzardUI/Blizzard_AccountSaveUI/
//! Blizzard_AccountSaveUI.lua`, lines 35-37 and 95-103):
//!
//! ```lua
//! function AccountSaveFrameMixin:OnLoad()
//!     self.LockEditBox:SetScript("OnTextChanged", GenerateClosure(self.OnLockEditBoxTextChanged, self));
//!     self.LockEditBox:SetScript("OnEnterPressed", GenerateClosure(self.OnLockEditBoxEnterPressed, self));
//!     self.LockEditBox:SetScript("OnEscapePressed", GenerateClosure(self.OnLockEditBoxEscapePressed, self));
//!     ...
//! end
//!
//! function AccountSaveFrameMixin:OnLockEditBoxEnterPressed()
//!     if self:DoesLockStringMatch() then
//!         self:SaveAccountData();
//!     end
//! end
//!
//! function AccountSaveFrameMixin:OnLockEditBoxEscapePressed()
//!     self.LockEditBox:ClearFocus();
//! end
//! ```
//!
//! Why pinning the Enter gate matters: this is the second path into
//! `SaveAccountData` (the first is the SaveButton click, already pinned
//! by `behavior_save_button_click.rs`). Without the gate, the user
//! could trigger an account save by pressing Enter on an empty or
//! mistyped editbox — bypassing the deliberate friction the lock
//! string is meant to provide. The gate uses the same matcher the
//! button-enabled state uses (`DoesLockStringMatch`), so this fixture
//! also acts as a redundant pin on the matcher's branch behavior.
//!
//! Why pinning the Escape behavior matters: the editbox holds focus
//! during typing; without `ClearFocus()` on Escape the user couldn't
//! return focus to the rest of the UI without clicking elsewhere.
//!
//! Both tests drive through `GetScript("OnEnterPressed")` /
//! `GetScript("OnEscapePressed")` rather than calling the mixin
//! methods directly. This pins the full chain — `OnLoad` set the
//! script via `GenerateClosure(self.OnLockEditBoxEnterPressed, self)`,
//! and the closure must forward `self` correctly so the gate has the
//! right `self.LockEditBox` to read text from. Calling the mixin
//! method directly would skip the closure binding.

use crate::common::blizzard_addon_harness::with_blizzard_addon_glue_smoke_shape;

const ROOT: &str = "Blizzard_AccountSaveUI";

#[test]
fn enter_pressed_triggers_save_account_data_only_when_lock_string_matches() {
    with_blizzard_addon_glue_smoke_shape(&[ROOT], &[], |env, _loaded| {
        {
            let mut state = env.state().borrow_mut();
            state.account_save_enabled = true;
            state.account_locked_post_save = false;
            state.account_save_in_progress = false;
        }

        let (count_no_match, count_match) = env
            .eval::<(i64, i64)>(
                r#"
                assert(AccountSaveFrame, "AccountSaveFrame global must exist after Blizzard_AccountSaveUI load")
                assert(ACCOUNT_SAVE_CONFIRM_STRING == "LOCK ACCOUNT",
                       "ACCOUNT_SAVE_CONFIRM_STRING must equal `LOCK ACCOUNT` (data/global_strings.rs)")

                AccountSaveFrame:UpdateAccountState()

                local save_call_count = 0
                local original_save = C_AccountServices.SaveAccountData
                C_AccountServices.SaveAccountData = function(...)
                    save_call_count = save_call_count + 1
                    return original_save(...)
                end

                local enter_handler = AccountSaveFrame.LockEditBox:GetScript("OnEnterPressed")
                assert(type(enter_handler) == "function",
                       "OnEnterPressed must be a function set by OnLoad via SetScript " ..
                       "(Blizzard_AccountSaveUI.lua:36)")

                AccountSaveFrame.LockEditBox:SetText("not the right phrase")
                enter_handler(AccountSaveFrame.LockEditBox)
                local count_after_no_match = save_call_count

                AccountSaveFrame.LockEditBox:SetText("LOCK ACCOUNT")
                enter_handler(AccountSaveFrame.LockEditBox)
                local count_after_match = save_call_count

                return count_after_no_match, count_after_match
                "#,
            )
            .expect("Enter-pressed save-gating probe must run cleanly");

        assert_eq!(
            count_no_match, 0,
            "C_AccountServices.SaveAccountData must NOT be called when the OnEnterPressed \
             script fires with non-matching editbox text \
             (Blizzard_AccountSaveUI.lua:96-98 — `if self:DoesLockStringMatch() then ... end`). \
             A non-zero count here means the gate was lost: pressing Enter on an empty or \
             wrong editbox would trigger an account save, bypassing the lock-string friction. \
             The probe wraps SaveAccountData in a counting closure and asserts the count stays \
             at zero after the OnEnterPressed handler runs against `\"not the right phrase\"`. \
             Got count_no_match = {count_no_match}."
        );

        assert_eq!(
            count_match, 1,
            "C_AccountServices.SaveAccountData must be called exactly once when the \
             OnEnterPressed script fires with matching editbox text \
             (Blizzard_AccountSaveUI.lua:96-98). The probe sets `\"LOCK ACCOUNT\"` (the \
             canonical confirm string) and asserts the counter incremented from 0 to 1. \
             A count of 0 means the gate is too strict (e.g., comparison flipped to `not \
             self:DoesLockStringMatch()`); a count > 1 means SaveAccountData was called \
             multiple times in one Enter press. Got count_match = {count_match}."
        );
    });
}

#[test]
fn escape_pressed_clears_focus_on_lock_edit_box() {
    with_blizzard_addon_glue_smoke_shape(&[ROOT], &[], |env, _loaded| {
        {
            let mut state = env.state().borrow_mut();
            state.account_save_enabled = true;
            state.account_locked_post_save = false;
            state.account_save_in_progress = false;
        }

        let clear_focus_count = env
            .eval::<i64>(
                r#"
                assert(AccountSaveFrame, "AccountSaveFrame global must exist after Blizzard_AccountSaveUI load")

                AccountSaveFrame:UpdateAccountState()

                local clear_focus_call_count = 0
                local original_clear_focus = AccountSaveFrame.LockEditBox.ClearFocus
                AccountSaveFrame.LockEditBox.ClearFocus = function(self)
                    clear_focus_call_count = clear_focus_call_count + 1
                    return original_clear_focus(self)
                end

                local escape_handler = AccountSaveFrame.LockEditBox:GetScript("OnEscapePressed")
                assert(type(escape_handler) == "function",
                       "OnEscapePressed must be a function set by OnLoad via SetScript " ..
                       "(Blizzard_AccountSaveUI.lua:37)")

                escape_handler(AccountSaveFrame.LockEditBox)

                return clear_focus_call_count
                "#,
            )
            .expect("Escape-pressed clear-focus probe must run cleanly");

        assert_eq!(
            clear_focus_count, 1,
            "LockEditBox:ClearFocus must be called exactly once when the OnEscapePressed \
             script fires (Blizzard_AccountSaveUI.lua:101-103 — \
             `self.LockEditBox:ClearFocus()`). The probe wraps ClearFocus in a counting \
             closure and dispatches the OnEscapePressed handler installed by OnLoad. A count \
             of 0 means OnLockEditBoxEscapePressed never reached ClearFocus — the body was \
             likely emptied or the wrong frame's ClearFocus was called (e.g., \
             `self:ClearFocus()` instead of `self.LockEditBox:ClearFocus()`, which would \
             route the call to AccountSaveFrame and miss our LockEditBox-scoped wrapper). A \
             count > 1 means ClearFocus was called multiple times in one Escape press. \
             Got clear_focus_count = {clear_focus_count}."
        );
    });
}
