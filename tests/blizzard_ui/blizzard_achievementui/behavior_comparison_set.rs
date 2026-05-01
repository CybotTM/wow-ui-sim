//! Behavior pin: `AchievementFrame_DisplayComparison(unit)` (lua:225)
//! reaches `SetAchievementComparisonUnit(unit)` TRANSITIVELY (through
//! `AchievementFrameComparison_SetUnit`, which first calls
//! `ClearAchievementComparisonUnit()`), and shows
//! `AchievementFrameComparison` DIRECTLY via `AchievementFrame_ShowSubFrame`
//! at lua:232 — *before* anything related to `INSPECT_ACHIEVEMENT_READY`.
//! PLAN's "shows AchievementFrameComparison after INSPECT_ACHIEVEMENT_READY
//! fires" reverses the actual order; the depends-on tag is also stale.
//!
//! Source map (the actual contract at
//! `Mainline/Blizzard_AchievementUI.lua:225-235`):
//!
//! ```lua
//! function AchievementFrame_DisplayComparison (unit)
//!     ClearSelectedCategories();
//!
//!     AchievementFrameTab_OnClick = AchievementFrameComparisonTab_OnClick;
//!     AchievementFrameTab_OnClick(1);
//!     AchievementFrame_SetComparisonTabs();
//!     ShowUIPanel(AchievementFrame);
//!     AchievementFrame_ShowSubFrame(AchievementFrameComparison, AchievementFrameComparison.AchievementContainer);  -- line 232: SHOW happens here
//!     AchievementFrameComparison_SetUnit(unit);                                                                    -- line 233: SetUnit chain follows
//!     AchievementFrameComparison_ForceUpdate();
//! end
//! ```
//!
//! ```lua
//! -- lua:2834-2844 (the proxy that wraps SetAchievementComparisonUnit)
//! function AchievementFrameComparison_SetUnit (unit)
//!     ClearAchievementComparisonUnit();              -- line 2835: clear first
//!     SetAchievementComparisonUnit(unit);            -- line 2836: then set
//!     AchievementFrameComparisonHeader.Points:SetText(GetComparisonAchievementPoints());
//!     AchievementFrameComparisonHeaderName:SetText(GetUnitName(unit));
//!     ...
//! end
//! ```
//!
//! ```lua
//! -- lua:2814-2832 (what INSPECT_ACHIEVEMENT_READY actually does)
//! function AchievementFrameComparison_OnEvent (self, event, ...)
//!     if event == "INSPECT_ACHIEVEMENT_READY" then
//!         ClearSelectedCategories();
//!         local category = AchievementFrame_GetOrSelectCurrentCategory();
//!         AchievementFrameComparison_UpdateStatusBars(category);                  -- updates STATUS BARS
//!         AchievementFrameComparisonHeader.Points:SetText(GetComparisonAchievementPoints());  -- updates POINTS TEXT
//!     elseif ...
//!     end
//!     AchievementFrameComparison_ForceUpdate();
//! end
//! ```
//!
//! XML widget chain (the comparison frame's declaration):
//!
//! - `AchievementFrameComparison` is declared at xml:2080 as
//!   `<Frame name="$parentComparison" hidden="true">` inside
//!   `AchievementFrame`, so the global resolves to
//!   `AchievementFrameComparison` and starts hidden.
//! - The frame's `OnLoad` (lua:2764) calls
//!   `self:RegisterEvent("INSPECT_ACHIEVEMENT_READY")` at lua:2781 — the
//!   event is wired to the comparison frame, but only delivers to
//!   `_OnEvent` to update status bars + points text, NOT to toggle
//!   visibility.
//!
//! **Spec/source mismatch on TWO axes:**
//!
//! 1. **Order is reversed.** `_DisplayComparison` calls
//!    `AchievementFrame_ShowSubFrame(AchievementFrameComparison, ...)` at
//!    lua:232 BEFORE calling `_SetUnit(unit)` at lua:233. The simulator
//!    impl at `src/lua_api/globals/missing_surface/achievement_info.rs:689-704`
//!    fires `INSPECT_ACHIEVEMENT_READY` from inside
//!    `SetAchievementComparisonUnit`, so the event fires *after* the show.
//!    PLAN's "shows after INSPECT_ACHIEVEMENT_READY fires" reverses the
//!    actual temporal order. The event handler updates STATUS BARS and
//!    POINTS TEXT (lua:2818-2819), not visibility.
//! 2. **`SetAchievementComparisonUnit` is reached transitively, not
//!    directly.** PLAN's "calls `SetAchievementComparisonUnit(unit)`" is
//!    half-true: `_DisplayComparison` calls
//!    `AchievementFrameComparison_SetUnit(unit)` (lua:233), which calls
//!    `ClearAchievementComparisonUnit()` first (lua:2835) and only then
//!    `SetAchievementComparisonUnit(unit)` (lua:2836). The clear-then-set
//!    sequence resets `state.achievement_comparison_unit` between
//!    inspect targets so stale data from the previous unit doesn't bleed
//!    into the new one.
//! 3. **Depends-on tag is stale.** The C API
//!    `SetAchievementComparisonUnit` is implemented at
//!    `src/lua_api/globals/missing_surface/achievement_info.rs:327`
//!    (registration) and `:689` (impl, sets
//!    `state.achievement_comparison_unit = Some(unit)` and queues the
//!    `INSPECT_ACHIEVEMENT_READY` event); the companion
//!    `ClearAchievementComparisonUnit` is at `:333` (registration) and
//!    `:707` (impl, sets `state.achievement_comparison_unit = None`).
//!
//! Seven assertions split presence/behavior:
//!
//! - **Presence half** (5): `_G.AchievementFrame_DisplayComparison` is a
//!   function (lua:225); `_G.SetAchievementComparisonUnit` is a function
//!   (depends-on stale); `_G.ClearAchievementComparisonUnit` is a
//!   function (the actual prefix call inside the proxy at lua:2835);
//!   `_G.AchievementFrameComparison_SetUnit` is a function (the actual
//!   proxy at lua:2834 that wraps the clear-then-set sequence);
//!   `AchievementFrameComparison:GetObjectType() == "Frame"` (the live
//!   frame at xml:2080 that `_ShowSubFrame` toggles).
//! - **Behavior half** (2): `pcall(AchievementFrame_DisplayComparison, "player")`
//!   succeeds — the call chain doesn't crash on a real unit token; AND
//!   `AchievementFrameComparison:IsShown()` is true *immediately* after
//!   the call returns — proving the show happened in
//!   `AchievementFrame_ShowSubFrame` at lua:232, NOT gated on
//!   `INSPECT_ACHIEVEMENT_READY` dispatch (which would require an event
//!   tick).

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AchievementUI";
const PLAN_REFERENCED_DISPLAY_COMPARISON: &str = "AchievementFrame_DisplayComparison";
const PLAN_REFERENCED_SET_UNIT_API: &str = "SetAchievementComparisonUnit";
const COMPARISON_UNIT: &str = "player";

type ComparisonSetProbe = (String, String, String, String, String, bool, bool);

#[test]
fn display_comparison_shows_frame_directly_and_reaches_set_unit_via_proxy_wrapper() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let observations: ComparisonSetProbe = env
            .eval(
                r#"
                local display_comparison_type = type(_G.AchievementFrame_DisplayComparison)
                local set_unit_api_type = type(_G.SetAchievementComparisonUnit)
                local clear_unit_api_type = type(_G.ClearAchievementComparisonUnit)
                local proxy_set_unit_type = type(_G.AchievementFrameComparison_SetUnit)

                local comparison_frame_object_type = "no-comparison-frame"
                if type(_G.AchievementFrameComparison) == "table" then
                    comparison_frame_object_type = AchievementFrameComparison:GetObjectType()
                end

                local display_call_ok = false
                if type(_G.AchievementFrame_DisplayComparison) == "function" then
                    display_call_ok = pcall(AchievementFrame_DisplayComparison, "player")
                end

                local comparison_shown_after_display = false
                if type(_G.AchievementFrameComparison) == "table" then
                    comparison_shown_after_display = AchievementFrameComparison:IsShown() and true or false
                end

                return display_comparison_type,
                       set_unit_api_type,
                       clear_unit_api_type,
                       proxy_set_unit_type,
                       comparison_frame_object_type,
                       display_call_ok,
                       comparison_shown_after_display
                "#,
            )
            .expect("AchievementFrame_DisplayComparison probe must run cleanly");

        let (
            display_comparison_type,
            set_unit_api_type,
            clear_unit_api_type,
            proxy_set_unit_type,
            comparison_frame_object_type,
            display_call_ok,
            comparison_shown_after_display,
        ) = observations;

        assert_eq!(
            display_comparison_type, "function",
            "Expected `_G.{PLAN_REFERENCED_DISPLAY_COMPARISON}` to be a function — declared at \
             `Mainline/Blizzard_AchievementUI.lua:225` and `Cata/Blizzard_AchievementUI.lua` \
             (mirrored). Got `{display_comparison_type}`. A `nil` reading would mean the \
             addon's chunk failed to register the global; the inspect-from-context-menu flow \
             (`InspectUnit` → eventually `AchievementFrame_DisplayComparison`) would then \
             have no entry point."
        );

        assert_eq!(
            set_unit_api_type, "function",
            "Expected `_G.{PLAN_REFERENCED_SET_UNIT_API}` to be a function (PLAN tags this as \
             a gap, but it's implemented at \
             `src/lua_api/globals/missing_surface/achievement_info.rs:327` (registration) and \
             `:689` (impl: sets `state.achievement_comparison_unit = Some(unit)` and queues \
             the `INSPECT_ACHIEVEMENT_READY` event)). Got `{set_unit_api_type}`. The \
             depends-on tag is stale; if this assertion fails the call chain \
             `_DisplayComparison` → `_SetUnit` → `SetAchievementComparisonUnit` at lua:2836 \
             would crash with `attempt to call a nil value` and the comparison frame would \
             never receive its inspect target."
        );

        assert_eq!(
            clear_unit_api_type, "function",
            "Expected `_G.ClearAchievementComparisonUnit` to be a function — implemented at \
             `src/lua_api/globals/missing_surface/achievement_info.rs:333` (registration) \
             and `:707` (impl: sets `state.achievement_comparison_unit = None`). This is the \
             *prefix* call inside `AchievementFrameComparison_SetUnit` at lua:2835, so \
             every set is preceded by a clear (the order documented in the PLAN's \
             same-aspect comparison-clear test). Got `{clear_unit_api_type}`. A `nil` \
             reading means the proxy at lua:2834 would crash on its first line, before \
             reaching the set."
        );

        assert_eq!(
            proxy_set_unit_type, "function",
            "Expected `_G.AchievementFrameComparison_SetUnit` to be a function — declared at \
             `Mainline/Blizzard_AchievementUI.lua:2834`. This is the actual proxy that \
             `_DisplayComparison` calls at lua:233, NOT a direct call to \
             `SetAchievementComparisonUnit` as PLAN's wording suggests. The proxy clears \
             first (lua:2835), sets (lua:2836), then writes header text + portrait (lua:2838-2843). \
             Got `{proxy_set_unit_type}`. A `nil` reading means the addon's chunk failed to \
             register the proxy and the call at lua:233 would crash."
        );

        assert_eq!(
            comparison_frame_object_type, "Frame",
            "Expected `AchievementFrameComparison:GetObjectType()` to return `\"Frame\"` — \
             declared at `Mainline/Blizzard_AchievementUI.xml:2080` as \
             `<Frame name=\"$parentComparison\" hidden=\"true\">` inside `AchievementFrame`, \
             with scripts `OnLoad`/`OnEvent`/`OnShow`/`OnHide` (xml:2303-2306) bound to \
             `AchievementFrameComparison_*`. Got `{comparison_frame_object_type}`. A \
             `no-comparison-frame` reading means the frame failed to instantiate."
        );

        assert!(
            display_call_ok,
            "Expected `pcall(AchievementFrame_DisplayComparison, \"{COMPARISON_UNIT}\")` to \
             return `true` — the call chain reaches `ClearSelectedCategories` (lua:226), \
             `AchievementFrameComparisonTab_OnClick(1)` (lua:228-229), \
             `AchievementFrame_SetComparisonTabs()` (lua:230), \
             `ShowUIPanel(AchievementFrame)` (lua:231), \
             `AchievementFrame_ShowSubFrame(AchievementFrameComparison, ...)` (lua:232), \
             `AchievementFrameComparison_SetUnit(\"{COMPARISON_UNIT}\")` → \
             `ClearAchievementComparisonUnit()` + \
             `SetAchievementComparisonUnit(\"{COMPARISON_UNIT}\")` (lua:2835-2836), \
             `AchievementFrameComparison_ForceUpdate()` (lua:234). A `false` reading means \
             one of these intermediate calls is missing or errored (e.g. `GetUnitName`, \
             `UnitRace`, `UnitSex`, `C_AchievementInfo.SetPortraitTexture`, \
             `GetComparisonAchievementPoints` on the proxy's tail-end at lua:2838-2843)."
        );

        assert!(
            comparison_shown_after_display,
            "Expected `AchievementFrameComparison:IsShown()` to be `true` IMMEDIATELY after \
             `AchievementFrame_DisplayComparison(\"{COMPARISON_UNIT}\")` returns — proving \
             the show happened in `AchievementFrame_ShowSubFrame` at lua:232, NOT gated on \
             `INSPECT_ACHIEVEMENT_READY` dispatch (which would require an event tick to \
             dispatch the queued event from inside `SetAchievementComparisonUnit` to \
             `AchievementFrameComparison_OnEvent`). The frame starts `hidden=\"true\"` per \
             xml:2080, so a `false` reading means the `_ShowSubFrame` call didn't flip \
             visibility — either the subframes list (`GetOrCreateAchievementSubFramesList` \
             at lua:467-479) was missing the entry, or `subFrame:SetShown(true)` at lua:490 \
             failed to take effect. PLAN's wording \"shows AchievementFrameComparison \
             after INSPECT_ACHIEVEMENT_READY fires\" inverts the actual order: the show \
             happens FIRST (lua:232), then the SetUnit-chain queues the event (lua:233 → \
             lua:2836 → `state.events.push`), and the event handler at lua:2814-2832 only \
             updates STATUS BARS + POINTS TEXT, never visibility."
        );
    });
}
