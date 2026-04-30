//! Behavior pin: `UpdateSizing` runs only on `activeState` transitions
//! and computes a panel height that sums AlertIcon + Text + SaveButton
//! plus content-inset padding, with the LockEditBox padding+height
//! folded in when (and only when) the editbox is shown.
//!
//! Source (`Interface/BlizzardUI/Blizzard_AccountSaveUI/
//! Blizzard_AccountSaveUI.lua`, lines 49-89 and 173-196):
//!
//! ```lua
//! function AccountSaveFrameMixin:UpdateAccountState()
//!     ...
//!     local oldState = self.activeState;
//!     ... -- branch on locked vs unlocked, mutate self.activeState
//!
//!     if self.activeState ~= oldState then
//!         self:UpdateSizing();
//!     end
//! end
//!
//! local function GetYPadding(frame, pointNum)
//!     local point, _, _, _, offsetY = frame:GetPoint(pointNum);
//!     if string.find(point, "TOP") then
//!         return -(offsetY);
//!     else
//!         return offsetY;
//!     end
//! end
//!
//! function AccountSaveFrameMixin:UpdateSizing()
//!     local verticalPadding =
//!         GetYPadding(self.ContentInsets, 1) +
//!         GetYPadding(self.ContentInsets, 2) +
//!         GetYPadding(self.Text, 1) +
//!         GetYPadding(self.SaveButton, 1);
//!
//!     local panelHeight = verticalPadding + self.AlertIcon:GetHeight()
//!         + self.Text:GetHeight() + self.SaveButton:GetHeight();
//!     if self.LockEditBox:IsShown() then
//!         panelHeight = panelHeight + GetYPadding(self.LockEditBox, 1)
//!             + self.LockEditBox:GetHeight();
//!     end
//!     self:SetHeight(math.floor(panelHeight + 0.5));
//! end
//! ```
//!
//! The transition gate matters because `UpdateAccountState` runs on
//! every event and on every keystroke through `OnLockEditBoxTextChanged`
//! → re-running `UpdateSizing` per keystroke would be wasted work AND
//! would re-issue `SetHeight` calls that propagate dirty bits through
//! the layout system. Pinning "no transition → no UpdateSizing" guards
//! against a regression that drops the gate (e.g. an else-branch added
//! that calls UpdateSizing unconditionally, or the `~=` swapped to `==`).
//!
//! The height formula matters because the panel uses no layout-frame
//! template — heights are computed manually because the available
//! layout templates are inconsistent across classic/mainline (per the
//! comment in the upstream source). A regression that dropped one of
//! the four constituent heights, or that summed widths instead of
//! heights, would leave the frame visibly clipped or oversized. The
//! LockEditBox conditional is the one branch in the formula and the
//! one that flips between the locked/unlocked variants — pinning both
//! states catches a regression that inverted the `IsShown()` guard or
//! that always added the LockEditBox term.

use crate::common::blizzard_addon_harness::with_blizzard_addon_glue_smoke_shape;

const ROOT: &str = "Blizzard_AccountSaveUI";

#[test]
fn update_sizing_runs_only_on_active_state_transitions() {
    with_blizzard_addon_glue_smoke_shape(&[ROOT], &[], |env, _loaded| {
        {
            let mut state = env.state().borrow_mut();
            state.account_save_enabled = false;
            state.account_locked_post_save = false;
            state.account_save_in_progress = false;
        }

        env.eval::<()>(
            r#"
            assert(AccountSaveFrame, "AccountSaveFrame global must exist after Blizzard_AccountSaveUI load")
            update_sizing_calls = 0
            local original = AccountSaveFrame.UpdateSizing
            AccountSaveFrame.UpdateSizing = function(self)
                update_sizing_calls = update_sizing_calls + 1
                return original(self)
            end
            "#,
        )
        .expect("UpdateSizing wrapper install probe must run cleanly");

        let count_after_disabled = env
            .eval::<i64>(
                r#"
                AccountSaveFrame:UpdateAccountState()
                return update_sizing_calls
                "#,
            )
            .expect("Disabled-state probe must run cleanly");

        {
            let mut state = env.state().borrow_mut();
            state.account_save_enabled = true;
            state.account_locked_post_save = true;
        }

        let count_after_locked_first = env
            .eval::<i64>(
                r#"
                AccountSaveFrame:UpdateAccountState()
                return update_sizing_calls
                "#,
            )
            .expect("Locked-first-call probe must run cleanly");

        let count_after_locked_repeat = env
            .eval::<i64>(
                r#"
                AccountSaveFrame:UpdateAccountState()
                return update_sizing_calls
                "#,
            )
            .expect("Locked-repeat-call probe must run cleanly");

        {
            let mut state = env.state().borrow_mut();
            state.account_locked_post_save = false;
        }

        let count_after_unlocked = env
            .eval::<i64>(
                r#"
                AccountSaveFrame:UpdateAccountState()
                return update_sizing_calls
                "#,
            )
            .expect("Unlocked-transition probe must run cleanly");

        assert_eq!(
            count_after_disabled, 0,
            "UpdateSizing must NOT run when UpdateAccountState takes the Disabled early-return \
             branch (Blizzard_AccountSaveUI.lua:50-54). The Disabled branch sets activeState and \
             calls Hide(), then returns BEFORE the `if self.activeState ~= oldState` transition \
             check. A non-zero count here means either the early return was lost or UpdateSizing \
             was hoisted above the early return. Got: {count_after_disabled}."
        );

        assert_eq!(
            count_after_locked_first, 1,
            "UpdateSizing must run once when activeState transitions from Disabled to \
             EnabledLocked (Blizzard_AccountSaveUI.lua:86-88). The previous Disabled call set \
             activeState = Disabled (1); this call sets activeState = EnabledLocked (2), so the \
             `~= oldState` check fires. A count of 0 means the transition gate is too strict; a \
             count > 1 means UpdateSizing was called multiple times in one UpdateAccountState \
             pass. Got: {count_after_locked_first}."
        );

        assert_eq!(
            count_after_locked_repeat, 1,
            "UpdateSizing must NOT run a second time when UpdateAccountState is re-called with \
             the same activeState (still EnabledLocked). This is the load-bearing assertion for \
             the transition gate — `OnLockEditBoxTextChanged` calls UpdateAccountState on every \
             keystroke, so a missing gate would re-issue SetHeight per keystroke and dirty the \
             layout. A count of 2 here means the gate was lost or the comparison flipped. \
             Got: {count_after_locked_repeat}."
        );

        assert_eq!(
            count_after_unlocked, 2,
            "UpdateSizing must run again when activeState transitions from EnabledLocked to \
             EnabledUnlocked. The locked→unlocked transition flips the LockEditBox visibility, \
             which changes the panel height — without this call the panel would be sized for \
             the wrong variant. A count of 1 here means the transition was missed; a count > 2 \
             means UpdateSizing fired more times than expected. Got: {count_after_unlocked}."
        );
    });
}

#[test]
fn update_sizing_panel_height_includes_lock_edit_box_only_when_shown() {
    with_blizzard_addon_glue_smoke_shape(&[ROOT], &[], |env, _loaded| {
        {
            let mut state = env.state().borrow_mut();
            state.account_save_enabled = true;
            state.account_locked_post_save = false;
            state.account_save_in_progress = false;
        }

        env.eval::<()>(
            r#"
            assert(AccountSaveFrame, "AccountSaveFrame global must exist after Blizzard_AccountSaveUI load")

            function compute_panel_metrics()
                local function getYPadding(frame, pointNum)
                    local point, _, _, _, offsetY = frame:GetPoint(pointNum)
                    if string.find(point, "TOP") then
                        return -(offsetY)
                    else
                        return offsetY
                    end
                end

                local f = AccountSaveFrame
                local verticalPadding = getYPadding(f.ContentInsets, 1)
                                      + getYPadding(f.ContentInsets, 2)
                                      + getYPadding(f.Text, 1)
                                      + getYPadding(f.SaveButton, 1)
                local panelHeight = verticalPadding
                                  + f.AlertIcon:GetHeight()
                                  + f.Text:GetHeight()
                                  + f.SaveButton:GetHeight()
                local lockShown = f.LockEditBox:IsShown()
                if lockShown then
                    panelHeight = panelHeight + getYPadding(f.LockEditBox, 1)
                                              + f.LockEditBox:GetHeight()
                end
                return f:GetHeight(), math.floor(panelHeight + 0.5), lockShown
            end
            "#,
        )
        .expect("compute_panel_metrics install probe must run cleanly");

        let (unlocked_actual, unlocked_expected, unlocked_lock_shown) = env
            .eval::<(f64, f64, bool)>(
                r#"
                AccountSaveFrame:UpdateAccountState()
                return compute_panel_metrics()
                "#,
            )
            .expect("Unlocked-state metrics probe must run cleanly");

        {
            let mut state = env.state().borrow_mut();
            state.account_locked_post_save = true;
        }

        let (locked_actual, locked_expected, locked_lock_shown) = env
            .eval::<(f64, f64, bool)>(
                r#"
                AccountSaveFrame:UpdateAccountState()
                return compute_panel_metrics()
                "#,
            )
            .expect("Locked-state metrics probe must run cleanly");

        assert!(
            unlocked_lock_shown,
            "LockEditBox must be shown in EnabledUnlocked state \
             (Blizzard_AccountSaveUI.lua:72 — `self.LockEditBox:Show()`). Without LockEditBox \
             shown, the height-formula assertion below cannot exercise the if-shown branch."
        );
        assert_eq!(
            unlocked_actual, unlocked_expected,
            "AccountSaveFrame:GetHeight() must equal the panel-height formula in unlocked \
             state, INCLUDING the LockEditBox padding+height contribution \
             (Blizzard_AccountSaveUI.lua:191-194). The probe duplicates the addon's formula \
             verbatim; a mismatch means UpdateSizing's formula drifted from this test or \
             SetHeight was passed a different value (e.g. unrounded, or with a constituent \
             dropped). Got actual = {unlocked_actual}, expected = {unlocked_expected}."
        );

        assert!(
            !locked_lock_shown,
            "LockEditBox must be hidden in EnabledLocked state \
             (Blizzard_AccountSaveUI.lua:64 — `self.LockEditBox:Hide()`). Without LockEditBox \
             hidden, the locked-state assertion can't exercise the !if-shown branch."
        );
        assert_eq!(
            locked_actual, locked_expected,
            "AccountSaveFrame:GetHeight() must equal the panel-height formula in locked state, \
             EXCLUDING the LockEditBox contribution because IsShown() is false. A regression \
             that inverted the IsShown() guard, or that always added the LockEditBox term, \
             would flip this assertion while leaving the unlocked-state assertion passing. Got \
             actual = {locked_actual}, expected = {locked_expected}."
        );

        assert!(
            locked_actual < unlocked_actual,
            "Locked-state panel height must be smaller than unlocked-state panel height \
             because the locked formula omits the LockEditBox padding+height term \
             (Blizzard_AccountSaveUI.lua:192-194). This cross-state inequality catches the \
             regression where both formulas matched their own self-derived expected values \
             (test would otherwise pass tautologically) but the LockEditBox term was \
             unconditionally included or excluded in both. Got locked = {locked_actual}, \
             unlocked = {unlocked_actual}."
        );
    });
}
