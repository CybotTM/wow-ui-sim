//! Behavior pin: PLAN-named `AchievementButton_OnDoubleClick` does NOT
//! exist in either Mainline or Cata source. PLAN's "calls
//! `SetFocusedAchievement(id)` exactly once and toggles tracking" claim
//! is also imagined as a single bundled action — the actual UI splits
//! these across two distinct user-facing handlers (button click vs.
//! check-icon click). This test pins the absence of the PLAN-named
//! function and the presence of the actual call sites.
//!
//! Source map (the real Mainline contract):
//!
//! - `AchievementTemplateMixin:SetSelected(selected)` at lua:1141-1145
//!   calls `SetFocusedAchievement(self.id)` exactly once after re-running
//!   `:Init(elementData)`. This is the focus side of PLAN's claim — but
//!   it does NOT toggle tracking.
//! - `AchievementTemplateMixin:ToggleTracking()` at lua:1580-1607 is the
//!   tracking-toggle side. It checks `trackedAchievements[id]`, calls
//!   `C_ContentTracking.StopTracking` / `StartTracking`, and bails with
//!   a UIErrors message when the tracked-count cap is hit or the
//!   achievement is already completed-by-guild / earned-by-me.
//! - `AchievementTemplateMixin:OnCheckClicked(o, buttonName, down)` at
//!   lua:1621-1623 is the actual user-facing dispatch — clicking the
//!   `Check` icon (NOT the button itself) fires `:ToggleTracking()`.
//!   `:ProcessClick(buttonName, down)` at lua:1060 also reaches
//!   `:ToggleTracking()` via the `IsModifiedClick("QUESTWATCHTOGGLE")`
//!   branch.
//!
//! Cata mirrors the split with bare globals:
//! `AchievementFrameAchievements_SelectButton(button)` at
//! `Cata/Blizzard_AchievementUI.lua:1086` calls `SetFocusedAchievement(button.id)`,
//! and `AchievementButton_ToggleTracking(id)` at lua:926 toggles tracking.
//! Neither flavor wires both into a single double-click handler.
//!
//! **Spec/source mismatch on THREE axes:**
//!
//! 1. `AchievementButton_OnDoubleClick` exists nowhere in either
//!    `Mainline/Blizzard_AchievementUI.lua` or
//!    `Cata/Blizzard_AchievementUI.lua` — no `OnDoubleClick`,
//!    `DoubleClick`, or `doubleclick` symbols at all.
//! 2. The "calls `SetFocusedAchievement(id)` exactly once AND toggles
//!    tracking" pairing is not a real Blizzard handler — focus and
//!    tracking are dispatched from different user actions (button click
//!    selects → focuses; check-icon click toggles tracking; modified
//!    click `QUESTWATCHTOGGLE` does both via `:ProcessClick`).
//! 3. The depends-on tag `SetFocusedAchievement gap` is stale — the C
//!    API is implemented at
//!    `src/lua_api/globals/missing_surface/achievement_info.rs:245`
//!    (registration) and `:911` (impl, writes
//!    `state.focused_achievement`).
//!
//! Six assertions split presence/absence:
//!
//! - **Absence half** (1): `_G.AchievementButton_OnDoubleClick` is nil.
//! - **Presence half** (5): `_G.SetFocusedAchievement` is a function;
//!   `AchievementTemplateMixin.SetSelected` is a function (the focus
//!   side of PLAN's claim, lua:1141);
//!   `AchievementTemplateMixin.ToggleTracking` is a function (the
//!   tracking side, lua:1580); `AchievementTemplateMixin.OnCheckClicked`
//!   is a function (the user-facing tracking-toggle dispatch, lua:1621);
//!   `SetFocusedAchievement(<seeded id>)` returns without error
//!   (writes `state.focused_achievement`).

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AchievementUI";
const PLAN_NAMED_BUT_ABSENT_FUNCTION: &str = "AchievementButton_OnDoubleClick";
const SEEDED_ACHIEVEMENT_ID: i64 = 6;

type DoubleClickProbe = (String, String, String, String, String, bool);

#[test]
fn double_click_function_is_absent_but_focus_and_tracking_call_sites_work() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let observations: DoubleClickProbe = env
            .eval(
                r#"
                local plan_named_function_type = type(_G.AchievementButton_OnDoubleClick)
                local set_focused_api_type = type(_G.SetFocusedAchievement)
                local mixin_set_selected_type =
                    (type(_G.AchievementTemplateMixin) == "table"
                        and type(_G.AchievementTemplateMixin.SetSelected))
                    or "no-mixin-table"
                local mixin_toggle_tracking_type =
                    (type(_G.AchievementTemplateMixin) == "table"
                        and type(_G.AchievementTemplateMixin.ToggleTracking))
                    or "no-mixin-table"
                local mixin_on_check_clicked_type =
                    (type(_G.AchievementTemplateMixin) == "table"
                        and type(_G.AchievementTemplateMixin.OnCheckClicked))
                    or "no-mixin-table"

                local set_focused_call_ok = false
                if type(_G.SetFocusedAchievement) == "function" then
                    set_focused_call_ok = pcall(SetFocusedAchievement, 6)
                end

                return plan_named_function_type,
                       set_focused_api_type,
                       mixin_set_selected_type,
                       mixin_toggle_tracking_type,
                       mixin_on_check_clicked_type,
                       set_focused_call_ok
                "#,
            )
            .expect("AchievementButton double-click probe must run cleanly");

        let (
            plan_named_function_type,
            set_focused_api_type,
            mixin_set_selected_type,
            mixin_toggle_tracking_type,
            mixin_on_check_clicked_type,
            set_focused_call_ok,
        ) = observations;

        assert_eq!(
            plan_named_function_type, "nil",
            "Expected `_G.{PLAN_NAMED_BUT_ABSENT_FUNCTION}` to be nil — a grep across both \
             `Mainline/Blizzard_AchievementUI.lua` and `Cata/Blizzard_AchievementUI.lua` finds \
             zero `OnDoubleClick`, `DoubleClick`, or `doubleclick` symbols. Got \
             `{plan_named_function_type}`. A non-nil reading would prove Blizzard added a \
             double-click handler (the absence half should then be replaced by a behavior probe \
             that drives the double-click and asserts `SetFocusedAchievement` was called \
             exactly once and `:ToggleTracking` was reached). The actual focus + toggle-tracking \
             logic is split across `:SetSelected` (focus, lua:1141), `:ToggleTracking` (toggle, \
             lua:1580), `:OnCheckClicked` (toggle dispatch from check-icon click, lua:1621), \
             and `:ProcessClick` (modified-click toggle dispatch via QUESTWATCHTOGGLE, lua:1060)."
        );

        assert_eq!(
            set_focused_api_type, "function",
            "Expected `_G.SetFocusedAchievement` to be a function (PLAN tags this as a gap, but \
             it's implemented at \
             `src/lua_api/globals/missing_surface/achievement_info.rs:245` (registration) and \
             `:911` (impl, writes `state.focused_achievement`)). Got `{set_focused_api_type}`. \
             The depends-on tag is stale; if this assertion fails the Mainline \
             `:SetSelected` site at lua:1144 and the Cata \
             `AchievementFrameAchievements_SelectButton` site at lua:1093 would crash."
        );

        assert_eq!(
            mixin_set_selected_type, "function",
            "Expected `AchievementTemplateMixin.SetSelected` to be a function — declared at \
             `Mainline/Blizzard_AchievementUI.lua:1141`, this is the actual call site for \
             `SetFocusedAchievement(self.id)` (the focus-side half of PLAN's imagined \
             double-click contract). Got `{mixin_set_selected_type}`. A `nil` reading means the \
             addon's mixin definition never executed or the method was renamed."
        );

        assert_eq!(
            mixin_toggle_tracking_type, "function",
            "Expected `AchievementTemplateMixin.ToggleTracking` to be a function — declared at \
             `Mainline/Blizzard_AchievementUI.lua:1580`, this is the actual tracking-toggle \
             site (the tracking-side half of PLAN's imagined double-click contract). It calls \
             `C_ContentTracking.StopTracking` / `StartTracking` and bails with a UIErrors \
             message on the cap or completed-by-guild / earned-by-me guards. Got \
             `{mixin_toggle_tracking_type}`. A `nil` reading means the mixin definition never \
             executed or the method was renamed; the user would still be able to click the \
             button but the check-icon click and the QUESTWATCHTOGGLE-modifier click would both \
             crash."
        );

        assert_eq!(
            mixin_on_check_clicked_type, "function",
            "Expected `AchievementTemplateMixin.OnCheckClicked` to be a function — declared at \
             `Mainline/Blizzard_AchievementUI.lua:1621`, this is the actual user-facing dispatch \
             that fires `:ToggleTracking()` when the user clicks the `Check` icon (NOT the \
             button itself). Got `{mixin_on_check_clicked_type}`. A `nil` reading means the \
             check-icon handler is unreachable; the only remaining path to `:ToggleTracking` \
             would be `:ProcessClick` via `IsModifiedClick(\"QUESTWATCHTOGGLE\")`."
        );

        assert!(
            set_focused_call_ok,
            "Expected `SetFocusedAchievement({SEEDED_ACHIEVEMENT_ID})` to call without error — \
             the impl at `achievement_info.rs:911` accepts an integer arg and writes to \
             `state.focused_achievement`. A `false` reading means the C API errored out (likely \
             on stack-arg type coercion or a borrow conflict). The seeded `Level 10` achievement \
             at `src/lua_api/state.rs:2178-2191` has id 6 so the call is well-formed."
        );
    });
}
