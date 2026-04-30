//! Behavior pin: `UpdateAccountState` disables both inputs and shows
//! the in-progress popup when `IsAccountSaveInProgress()` is true.
//!
//! Source (`Interface/BlizzardUI/Blizzard_AccountSaveUI/
//! Blizzard_AccountSaveUI.lua`, lines 77-84):
//!
//! ```lua
//! if C_AccountServices.IsAccountSaveInProgress() then
//!     self.LockEditBox:SetEnabled(false);
//!     self.SaveButton:SetEnabled(false);
//!     StaticPopup_Show("ACCOUNT_SAVE_IN_PROGRESS");
//! else
//!     self.LockEditBox:SetEnabled(true);
//!     StaticPopup_Hide("ACCOUNT_SAVE_IN_PROGRESS");
//! end
//! ```
//!
//! This block runs after the locked/unlocked branch settles the
//! editbox visibility, so it needs `account_save_enabled = true` to
//! avoid the early-return guard. The test uses the unlocked
//! combination (locked=false) so `LockEditBox` is `Show()`n by the
//! preceding branch — that way `IsEnabled() == false` after the
//! in-progress block proves the `SetEnabled(false)` call actually
//! landed (a hidden editbox would also report enabled=false on some
//! widget toolkits, masking the regression).
//!
//! `StaticPopup_Show` is a no-op stub in the simulator
//! (`runtime_surface_bootstrap.lua:259-262`), so visibility cannot be
//! observed via a sim-side query API. Instead the test wraps the
//! global `StaticPopup_Show` in a Lua closure that records the dialog
//! name into a probe table, then asserts `"ACCOUNT_SAVE_IN_PROGRESS"`
//! appears in the captured calls. The harness builds a fresh env per
//! test so the global override does not leak across fixtures.

use crate::common::blizzard_addon_harness::with_blizzard_addon_glue_smoke_shape;

const ROOT: &str = "Blizzard_AccountSaveUI";
const POPUP_NAME: &str = "ACCOUNT_SAVE_IN_PROGRESS";

#[test]
fn update_account_state_disables_inputs_and_shows_popup_when_save_in_progress() {
    with_blizzard_addon_glue_smoke_shape(&[ROOT], &[], |env, _loaded| {
        {
            let mut state = env.state().borrow_mut();
            state.account_save_enabled = true;
            state.account_locked_post_save = false;
            state.account_save_in_progress = true;
        }

        let (lock_edit_box_enabled, save_button_enabled, popup_shown) = env
            .eval::<(bool, bool, bool)>(
                r#"
                assert(AccountSaveFrame, "AccountSaveFrame global must exist after Blizzard_AccountSaveUI load")

                local shown_dialogs = {}
                local original_show = StaticPopup_Show
                StaticPopup_Show = function(which, ...)
                    shown_dialogs[#shown_dialogs + 1] = which
                    return original_show(which, ...)
                end

                AccountSaveFrame.LockEditBox:SetEnabled(true)
                AccountSaveFrame.SaveButton:SetEnabled(true)

                AccountSaveFrame:UpdateAccountState()

                local popup_seen = false
                for _, name in ipairs(shown_dialogs) do
                    if name == "ACCOUNT_SAVE_IN_PROGRESS" then
                        popup_seen = true
                        break
                    end
                end

                return AccountSaveFrame.LockEditBox:IsEnabled(),
                       AccountSaveFrame.SaveButton:IsEnabled(),
                       popup_seen
                "#,
            )
            .expect("UpdateAccountState probe must run cleanly under the in-progress precondition");

        assert!(
            !lock_edit_box_enabled,
            "AccountSaveFrame.LockEditBox must be disabled when IsAccountSaveInProgress() is \
             true (Blizzard_AccountSaveUI.lua:78). The probe explicitly SetEnabled(true)s the \
             editbox before calling UpdateAccountState — a `false` here proves the in-progress \
             branch actually called SetEnabled(false). If this regresses while a save is in \
             flight, the player can keep typing into the lock editbox while the backend save is \
             racing — the SaveButton is disabled but stale text could be re-confirmed once \
             the in-progress flag flips off. Got IsEnabled = {lock_edit_box_enabled}."
        );

        assert!(
            !save_button_enabled,
            "AccountSaveFrame.SaveButton must be disabled when IsAccountSaveInProgress() is \
             true (Blizzard_AccountSaveUI.lua:79). The probe explicitly SetEnabled(true)s the \
             button before calling UpdateAccountState — a `false` here proves the in-progress \
             branch actually called SetEnabled(false). Without this gate, double-clicking the \
             button during a save would issue a second SaveAccountData call (which the C API \
             rejects with ALREADY_IN_PROGRESS, but the UI must not even allow the attempt). \
             Got IsEnabled = {save_button_enabled}."
        );

        assert!(
            popup_shown,
            "StaticPopup_Show(\"{POPUP_NAME}\") must be called when UpdateAccountState runs \
             with IsAccountSaveInProgress() == true (Blizzard_AccountSaveUI.lua:80). The probe \
             wraps StaticPopup_Show in a tracking closure (the simulator's stub is a no-op so \
             popup state can't be queried directly); a `false` here means the in-progress \
             branch never called the popup helper, leaving the player without the visual \
             confirmation that a save is in flight. The corresponding StaticPopup definition \
             (StaticPopupDialogs[\"{POPUP_NAME}\"]) is pinned by the surface_globals fixture \
             with `explicitAcknowledge = true`."
        );
    });
}
