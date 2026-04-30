//! Behavior pin: `UpdateAccountState` collapses to the Disabled branch
//! when account save is unavailable.
//!
//! Source (`Interface/BlizzardUI/Blizzard_AccountSaveUI/
//! Blizzard_AccountSaveUI.lua`, lines 49-54):
//!
//! ```lua
//! function AccountSaveFrameMixin:UpdateAccountState()
//!     if not C_AccountServices.IsAccountSaveEnabled() then
//!         self.activeState = AccountSaveFrameMixin.VisualState.Disabled;
//!         self:Hide();
//!         return;
//!     end
//!     ...
//! end
//! ```
//!
//! The disabled branch is the early-return guard. It must do exactly
//! two observable things: stamp `self.activeState` with the Disabled
//! sentinel (1) and hide the frame. Every later branch in the function
//! reads `oldState = self.activeState` to decide whether to call
//! `UpdateSizing`, so a regression that forgets the activeState write
//! would silently make the next state transition skip the resize.
//! Pinning both observables keeps the early-return contract honest.
//!
//! Default `SimState.account_save_enabled` is `false` (set in
//! `state.rs:3153`), so `IsAccountSaveEnabled()` returns false without
//! any pre-arrangement. The test explicitly `Show()`s the frame first
//! to make the hide-on-disabled outcome unambiguous — without that, a
//! frame that was already hidden would still report `IsShown() ==
//! false` and the test would falsely pass even if `Hide()` was deleted.

use crate::common::blizzard_addon_harness::with_blizzard_addon_glue_smoke_shape;

const ROOT: &str = "Blizzard_AccountSaveUI";

#[test]
fn update_account_state_hides_frame_and_marks_disabled_when_save_unavailable() {
    with_blizzard_addon_glue_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let (precondition_save_enabled, active_state, is_shown) = env
            .eval::<(bool, i64, bool)>(
                r#"
                assert(AccountSaveFrame, "AccountSaveFrame global must exist after Blizzard_AccountSaveUI load")
                assert(C_AccountServices and C_AccountServices.IsAccountSaveEnabled,
                       "C_AccountServices.IsAccountSaveEnabled must be registered before this fixture runs")
                local before = C_AccountServices.IsAccountSaveEnabled()
                AccountSaveFrame:Show()
                AccountSaveFrame:UpdateAccountState()
                return before, AccountSaveFrame.activeState, AccountSaveFrame:IsShown()
                "#,
            )
            .expect("UpdateAccountState probe must run cleanly under the disabled precondition");

        assert!(
            !precondition_save_enabled,
            "Precondition: `C_AccountServices.IsAccountSaveEnabled()` must return false before \
             this test runs UpdateAccountState. The simulator's SimState defaults \
             `account_save_enabled` to false (state.rs:3153); if this precondition flips, the \
             test must explicitly drive the flag back to false (no Lua-side setter exists yet — \
             would require a SimState mutation). Got: {precondition_save_enabled}."
        );

        let expected_disabled: i64 = 1;
        assert_eq!(
            active_state, expected_disabled,
            "AccountSaveFrame.activeState must be stamped with `VisualState.Disabled` (= 1) \
             when UpdateAccountState runs with IsAccountSaveEnabled() == false. The disabled \
             branch is the early-return guard at Blizzard_AccountSaveUI.lua:51 — every later \
             branch reads `oldState = self.activeState` to decide whether to UpdateSizing, so a \
             regression that forgets this write would silently skip the next state transition's \
             resize. Got activeState = {active_state}."
        );

        assert!(
            !is_shown,
            "AccountSaveFrame must be hidden after UpdateAccountState runs with \
             IsAccountSaveEnabled() == false (Blizzard_AccountSaveUI.lua:52). The test \
             explicitly Show()s the frame first, so a `false` here proves the disabled branch \
             actually called Hide() — not just that the frame was already hidden by some \
             prior step. If this regresses, the early-return guard probably dropped the \
             self:Hide() call. Got IsShown = {is_shown}."
        );
    });
}
