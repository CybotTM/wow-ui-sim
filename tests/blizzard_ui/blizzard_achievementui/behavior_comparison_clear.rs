//! Behavior pin: closing the comparison pane runs
//! `AchievementFrameComparison:Hide()` (or the equivalent
//! `_ShowSubFrame` flip), which fires the `OnHide` script handler bound
//! to `AchievementFrameComparison_OnHide` (xml:2306, lua:2804). The
//! handler at lua:2810 then calls `ClearAchievementComparisonUnit()`.
//! PLAN's claim is correct as a *behavior summary*; the depends-on tag
//! is the stale half — both `SetAchievementComparisonUnit` and
//! `ClearAchievementComparisonUnit` are implemented on the Rust side.
//!
//! Source map (the actual close-path contract):
//!
//! ```lua
//! -- lua:195-212 (one of the close entry points: toggle the panel)
//! function AchievementFrame_ToggleAchievementFrame(toggleStatFrame, toggleGuildView)
//!     AchievementFrameComparison:Hide();    -- line 196: forces OnHide
//!     ...
//! end
//! ```
//!
//! ```lua
//! -- lua:2804-2812 (the OnHide handler bound via xml:2306)
//! function AchievementFrameComparison_OnHide(self)
//!     AchievementFrame.selectedTab = nil;
//!     AchievementFrame:SetWidth(768);
//!     SetUIPanelAttribute(AchievementFrame, "xOffset", 80);
//!     UpdateUIPanelPositions(AchievementFrame);
//!     AchievementFrame.isComparison = false;
//!     ClearAchievementComparisonUnit();                                   -- line 2810: the clear call
//!     FrameUtil.UnregisterFrameForEvents(self, AchievementFrameComparisonShownEvents);
//! end
//! ```
//!
//! ```lua
//! -- lua:2834-2844 (the OPEN-side proxy; `ClearAchievementComparisonUnit`
//! -- is also called as a *prefix* before each set, lua:2835)
//! function AchievementFrameComparison_SetUnit (unit)
//!     ClearAchievementComparisonUnit();
//!     SetAchievementComparisonUnit(unit);
//!     ...
//! end
//! ```
//!
//! XML script binding for the comparison frame:
//!
//! ```xml
//! <!-- xml:2303-2306 -->
//! <Scripts>
//!     <OnLoad function="AchievementFrameComparison_OnLoad"/>
//!     <OnEvent function="AchievementFrameComparison_OnEvent"/>
//!     <OnShow function="AchievementFrameComparison_OnShow"/>
//!     <OnHide function="AchievementFrameComparison_OnHide"/>   <!-- triggers the clear -->
//! </Scripts>
//! ```
//!
//! Cata mirrors the OnHide contract at
//! `Cata/Blizzard_AchievementUI.lua` — the same handler runs
//! `ClearAchievementComparisonUnit()` on hide.
//!
//! **Spec/source agreement on the behavior axis; depends-on tag is the
//! stale half:**
//!
//! 1. `AchievementFrameComparison:Hide()` does NOT directly call
//!    `ClearAchievementComparisonUnit()` — the clear is bound through
//!    the `OnHide` *script handler* (xml:2306 → lua:2804), so any path
//!    that flips the frame from shown to hidden (`Hide()`, `SetShown(false)`,
//!    `_ShowSubFrame` switching to a non-comparison container) reaches
//!    the clear.
//! 2. The depends-on tag `SetAchievementComparisonUnit gap` is stale.
//!    `ClearAchievementComparisonUnit` is implemented at
//!    `src/lua_api/globals/missing_surface/achievement_info.rs:333`
//!    (registration) and `:707` (impl: sets
//!    `state.achievement_comparison_unit = None`); the companion
//!    `SetAchievementComparisonUnit` is at `:327`/`:689`.
//!
//! Eight assertions split presence/behavior:
//!
//! - **Presence half** (5): `_G.AchievementFrame_DisplayComparison` is a
//!   function (the setup entry point); `_G.ClearAchievementComparisonUnit`
//!   is a function (depends-on stale); `_G.SetAchievementComparisonUnit`
//!   is a function (used by the setup chain); `_G.AchievementFrameComparison_OnHide`
//!   is a function (lua:2804 — the actual close-side handler that calls
//!   the clear); `AchievementFrameComparison:GetObjectType() == "Frame"`.
//! - **Behavior half** (3): after
//!   `AchievementFrame_DisplayComparison("player")` runs, the frame is
//!   shown; after a subsequent `AchievementFrameComparison:Hide()`, the
//!   frame is hidden AND the spy on `_G.ClearAchievementComparisonUnit`
//!   counted at least one call between Hide-start and Hide-finish —
//!   proving the OnHide-bound `ClearAchievementComparisonUnit()` at
//!   lua:2810 fires on the close edge (a count of 0 means the OnHide
//!   handler dispatch was lost).

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AchievementUI";
const PLAN_REFERENCED_CLEAR_API: &str = "ClearAchievementComparisonUnit";
const PLAN_REFERENCED_ON_HIDE: &str = "AchievementFrameComparison_OnHide";
const COMPARISON_UNIT: &str = "player";

type ComparisonClearProbe = (String, String, String, String, String, bool, bool, i64);

#[test]
fn closing_comparison_pane_fires_on_hide_handler_that_calls_clear_achievement_comparison_unit() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let observations: ComparisonClearProbe = env
            .eval(
                r#"
                local display_comparison_type = type(_G.AchievementFrame_DisplayComparison)
                local clear_unit_api_type = type(_G.ClearAchievementComparisonUnit)
                local set_unit_api_type = type(_G.SetAchievementComparisonUnit)
                local on_hide_handler_type = type(_G.AchievementFrameComparison_OnHide)

                local comparison_frame_object_type = "no-comparison-frame"
                if type(_G.AchievementFrameComparison) == "table" then
                    comparison_frame_object_type = AchievementFrameComparison:GetObjectType()
                end

                local shown_before_close = false
                local shown_after_close = true
                local close_clear_count = 0

                local can_drive = type(_G.AchievementFrame_DisplayComparison) == "function"
                    and type(_G.ClearAchievementComparisonUnit) == "function"
                    and type(_G.AchievementFrameComparison) == "table"

                if can_drive then
                    pcall(AchievementFrame_DisplayComparison, "player")
                    shown_before_close = AchievementFrameComparison:IsShown() and true or false

                    local original_clear = _G.ClearAchievementComparisonUnit
                    _G.ClearAchievementComparisonUnit = function(...)
                        close_clear_count = close_clear_count + 1
                        return original_clear(...)
                    end

                    pcall(function() AchievementFrameComparison:Hide() end)
                    shown_after_close = AchievementFrameComparison:IsShown() and true or false

                    _G.ClearAchievementComparisonUnit = original_clear
                end

                return display_comparison_type,
                       clear_unit_api_type,
                       set_unit_api_type,
                       on_hide_handler_type,
                       comparison_frame_object_type,
                       shown_before_close,
                       shown_after_close,
                       close_clear_count
                "#,
            )
            .expect("AchievementFrameComparison close-path probe must run cleanly");

        let (
            display_comparison_type,
            clear_unit_api_type,
            set_unit_api_type,
            on_hide_handler_type,
            comparison_frame_object_type,
            shown_before_close,
            shown_after_close,
            close_clear_count,
        ) = observations;

        assert_eq!(
            display_comparison_type, "function",
            "Expected `_G.AchievementFrame_DisplayComparison` to be a function — declared at \
             `Mainline/Blizzard_AchievementUI.lua:225`. The test uses it as the *setup* entry \
             point to put the comparison pane in the shown state before exercising the close \
             path. Got `{display_comparison_type}`. A `nil` reading means the addon's chunk \
             failed to register the global; without it the test cannot create the \
             shown-then-hide transition that triggers `OnHide`."
        );

        assert_eq!(
            clear_unit_api_type, "function",
            "Expected `_G.{PLAN_REFERENCED_CLEAR_API}` to be a function (PLAN tags this as a \
             gap, but it's implemented at \
             `src/lua_api/globals/missing_surface/achievement_info.rs:333` (registration) \
             and `:707` (impl: sets `state.achievement_comparison_unit = None`)). Got \
             `{clear_unit_api_type}`. The depends-on tag is stale; if this assertion fails \
             both close-side call sites at lua:2810 (`_OnHide`) AND lua:2835 (`_SetUnit` \
             prefix) would crash with `attempt to call a nil value`."
        );

        assert_eq!(
            set_unit_api_type, "function",
            "Expected `_G.SetAchievementComparisonUnit` to be a function — used during the \
             test's setup pass (`AchievementFrame_DisplayComparison(\"player\")` reaches it \
             transitively at lua:2836). Implemented at \
             `src/lua_api/globals/missing_surface/achievement_info.rs:327` (registration) \
             and `:689` (impl: sets `state.achievement_comparison_unit = Some(unit)` and \
             queues `INSPECT_ACHIEVEMENT_READY`). Got `{set_unit_api_type}`. A `nil` reading \
             means the setup pass would fail before the close path can be exercised."
        );

        assert_eq!(
            on_hide_handler_type, "function",
            "Expected `_G.{PLAN_REFERENCED_ON_HIDE}` to be a function — declared at \
             `Mainline/Blizzard_AchievementUI.lua:2804` and bound via the XML `<OnHide \
             function=\"AchievementFrameComparison_OnHide\"/>` at xml:2306. This is the \
             ONLY place `ClearAchievementComparisonUnit()` is called on the close path \
             (lua:2810). Got `{on_hide_handler_type}`. A `nil` reading means the OnHide \
             dispatch is unwired and the close path would leave \
             `state.achievement_comparison_unit` pinned at `Some(prevUnit)` — the next \
             `_DisplayComparison` would then re-fire `INSPECT_ACHIEVEMENT_READY` carrying \
             stale state."
        );

        assert_eq!(
            comparison_frame_object_type, "Frame",
            "Expected `AchievementFrameComparison:GetObjectType()` to return `\"Frame\"` — \
             declared at `Mainline/Blizzard_AchievementUI.xml:2080` as \
             `<Frame name=\"$parentComparison\" hidden=\"true\">` with the close-bound \
             `<OnHide>` script at xml:2306. Got `{comparison_frame_object_type}`. A \
             `no-comparison-frame` reading means the frame failed to instantiate."
        );

        assert!(
            shown_before_close,
            "Expected `AchievementFrameComparison:IsShown()` to be `true` after the setup \
             call `AchievementFrame_DisplayComparison(\"{COMPARISON_UNIT}\")` — `_DisplayComparison` \
             flips visibility via `AchievementFrame_ShowSubFrame` at lua:232 (which calls \
             `subFrame:SetShown(true)` at lua:490). Got `false`. Without the shown state, \
             the subsequent `Hide()` call would be a no-op and `OnHide` would never fire — \
             the close-side `ClearAchievementComparisonUnit()` would not be exercised, \
             making the count assertion below meaningless. A `false` reading means the \
             setup chain failed (`_DisplayComparison` errored partway through, or \
             `_ShowSubFrame` didn't include `AchievementFrameComparison` in its subframes \
             list at lua:467-479)."
        );

        assert!(
            !shown_after_close,
            "Expected `AchievementFrameComparison:IsShown()` to be `false` after \
             `AchievementFrameComparison:Hide()` — the frame transition shown→hidden is \
             what fires `OnHide` and reaches `ClearAchievementComparisonUnit()` at lua:2810. \
             Got `true`. A `true` reading means `Hide()` did not flip visibility (e.g. the \
             frame was already hidden, or a parent's `IsShown()` short-circuit made the \
             explicit hide a no-op). Without the visibility flip the OnHide dispatch \
             doesn't fire, making the close-path clear unreachable."
        );

        assert!(
            close_clear_count >= 1,
            "Expected the spy on `_G.{PLAN_REFERENCED_CLEAR_API}` to record at least one \
             call between `AchievementFrameComparison:Hide()` start and finish — proving \
             the OnHide-bound `ClearAchievementComparisonUnit()` at lua:2810 fires on the \
             close edge. Got `close_clear_count == {close_clear_count}`. A count of 0 means \
             one of: (a) the OnHide handler dispatch was lost (the simulator failed to fire \
             the script when `SetShown(false)` flipped visibility), (b) the XML \
             `<OnHide>` binding never resolved `AchievementFrameComparison_OnHide` to a \
             real function, (c) the handler crashed before reaching lua:2810 \
             (e.g. `SetUIPanelAttribute` or `UpdateUIPanelPositions` on lines 2807-2808 \
             errored — both are panel-system primitives that should exist in the smoke \
             harness). The exact-count expectation is 1 (OnHide runs once per Hide), but \
             `>= 1` is accepted to keep the assertion robust against incidental dispatch \
             chains; a count > 1 is itself worth investigating but is not the regression \
             this assertion is designed to catch."
        );
    });
}
