//! Behavior pin: `AchievementFrameSummary_Update()` (lua:2297) does NOT
//! populate `AchievementFrame.Header.Points` and does NOT call
//! `GetTotalAchievementPoints`. PLAN's "populates Header.Points and
//! total/completed counters using the same two getters" wording is doubly
//! wrong; this test pins the actual contract and the absence of the PLAN
//! claim.
//!
//! Source (`Mainline/Blizzard_AchievementUI.lua:2297-2301`):
//!
//! ```lua
//! function AchievementFrameSummary_Update()
//!     AchievementFrameSummary_Refresh();
//!     AchievementFrameSummaryCategoriesStatusBar_Update();
//!     AchievementFrameSummary_UpdateAchievements(GetLatestCompletedAchievements(InGuildView()));
//! end
//! ```
//!
//! The function delegates to three sub-calls, none of which touch
//! `AchievementFrame.Header.Points`:
//!
//! - `AchievementFrameSummary_Refresh()` (lua:2318) sets per-button icon
//!   textures and dispatches `AchievementFrameSummary_UpdateSummaryProgressBars`
//!   for the per-category bars on the Summary tab.
//! - `AchievementFrameSummaryCategoriesStatusBar_Update()` (lua:2471-2476)
//!   is the **only** site in this call chain that uses
//!   `GetNumCompletedAchievements(InGuildView())`. It writes to
//!   `AchievementFrameSummaryCategoriesStatusBar:SetMinMaxValues(0, total)`,
//!   `:SetValue(completed)`, and
//!   `AchievementFrameSummaryCategoriesStatusBarText:SetText(BreakUpLargeNumbers(completed).."/"..BreakUpLargeNumbers(total))`.
//!   Header.Points is never referenced.
//! - `AchievementFrameSummary_UpdateAchievements(...)` (lua:2342) uses
//!   `GetLatestCompletedAchievements`, **not** the two getters PLAN names.
//!
//! `AchievementFrame.Header.Points` is updated by entirely different code
//! paths: `AchievementFrame_OnShow` at lua:272, `AchievementFrameTab_OnClick`
//! at lua:378, and `AchievementFrameAchievements_OnEvent` at lua:904 — all
//! direct inline `:SetText(BreakUpLargeNumbers(GetTotalAchievementPoints(...)))`
//! calls. None of those go through `AchievementFrameSummary_Update`.
//!
//! **Depends-on tags are stale.** PLAN tags this with `(depends-on:
//! GetNumCompletedAchievements gap, GetTotalAchievementPoints gap)` but
//! both C APIs are implemented at
//! `src/lua_api/globals/missing_surface/achievement_info.rs:651` and `:917`.
//!
//! Seven assertions split presence/absence:
//!
//! - **Presence half** (6): `_G.AchievementFrameSummary_Update` is a
//!   function; both C APIs are functions; the StatusBar and Text widgets
//!   the actual contract writes to
//!   (`AchievementFrameSummaryCategoriesStatusBar` /
//!   `AchievementFrameSummaryCategoriesStatusBarText`) exist with the
//!   right object types; the StatusBarText contains a `/` separator after
//!   the call (the format string
//!   `BreakUpLargeNumbers(completed).."/"..BreakUpLargeNumbers(total)`
//!   from lua:2475 always produces a `<n>/<n>` string when both APIs
//!   return numbers).
//! - **Absence half** (1): a sentinel string written to
//!   `AchievementFrame.Header.Points` BEFORE the call must be unchanged
//!   AFTER the call — proves `AchievementFrameSummary_Update` does NOT
//!   write to Header.Points (PLAN's claim is wrong). A future change that
//!   re-routes header points through the Summary update would clobber the
//!   sentinel and trip this assertion.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AchievementUI";
const HEADER_POINTS_SENTINEL: &str = "SENTINEL_HEADER_POINTS_UNCHANGED";

type SummaryProbe = (String, String, String, String, String, String, String);

#[test]
fn summary_update_writes_categories_status_bar_text_but_does_not_touch_header_points() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let observations: SummaryProbe = env
            .eval(
                r#"
                assert(AchievementFrame, "AchievementFrame global must exist after Blizzard_AchievementUI load")
                assert(AchievementFrame.Header, "AchievementFrame.Header must exist after load")
                assert(AchievementFrame.Header.Points, "AchievementFrame.Header.Points must exist after load")

                local update_function_type = type(_G.AchievementFrameSummary_Update)
                local total_points_api_type = type(_G.GetTotalAchievementPoints)
                local num_completed_api_type = type(_G.GetNumCompletedAchievements)

                local categories_status_bar = _G.AchievementFrameSummaryCategoriesStatusBar
                local categories_status_bar_text = _G.AchievementFrameSummaryCategoriesStatusBarText
                local status_bar_object_type = categories_status_bar
                    and categories_status_bar:GetObjectType()
                    or "nil"
                local status_bar_text_object_type = categories_status_bar_text
                    and categories_status_bar_text:GetObjectType()
                    or "nil"

                AchievementFrame.Header.Points:SetText("SENTINEL_HEADER_POINTS_UNCHANGED")
                AchievementFrameSummary_Update()
                local header_points_after_update = AchievementFrame.Header.Points:GetText() or ""
                local categories_status_bar_text_after_update = categories_status_bar_text
                    and (categories_status_bar_text:GetText() or "")
                    or ""

                return update_function_type,
                       total_points_api_type,
                       num_completed_api_type,
                       status_bar_object_type,
                       status_bar_text_object_type,
                       header_points_after_update,
                       categories_status_bar_text_after_update
                "#,
            )
            .expect("AchievementFrameSummary_Update behavior probe must run cleanly");

        let (
            update_function_type,
            total_points_api_type,
            num_completed_api_type,
            status_bar_object_type,
            status_bar_text_object_type,
            header_points_after_update,
            categories_status_bar_text_after_update,
        ) = observations;

        assert_eq!(
            update_function_type, "function",
            "Expected `_G.AchievementFrameSummary_Update` to be a function (declared at \
             Mainline/Blizzard_AchievementUI.lua:2297). Got `{update_function_type}`. A `nil` \
             reading means the addon's Lua never ran or the function was renamed."
        );

        assert_eq!(
            total_points_api_type, "function",
            "Expected `_G.GetTotalAchievementPoints` to be a function (PLAN tags it as a gap, \
             but it's implemented at \
             `src/lua_api/globals/missing_surface/achievement_info.rs:917`). Got \
             `{total_points_api_type}`. The depends-on tag is stale. Note: this getter is NOT \
             actually called from inside `AchievementFrameSummary_Update`'s call chain — PLAN's \
             \"using the same two getters\" claim is wrong; only `GetNumCompletedAchievements` \
             is reached (transitively via `AchievementFrameSummaryCategoriesStatusBar_Update` \
             at lua:2472)."
        );

        assert_eq!(
            num_completed_api_type, "function",
            "Expected `_G.GetNumCompletedAchievements` to be a function (PLAN tags it as a gap, \
             but it's implemented at \
             `src/lua_api/globals/missing_surface/achievement_info.rs:651`). Got \
             `{num_completed_api_type}`. The depends-on tag is stale; if this assertion fails \
             the inline `total, completed = GetNumCompletedAchievements(InGuildView())` site at \
             lua:2472 (the only getter actually reached from \
             `AchievementFrameSummary_Update`'s call chain) would crash."
        );

        assert_eq!(
            status_bar_object_type, "StatusBar",
            "Expected `AchievementFrameSummaryCategoriesStatusBar` to be a StatusBar (declared \
             in XML at Mainline/Blizzard_AchievementUI.xml:1958 as `$parentStatusBar` inside \
             `$parentCategories` inside `$parentSummary` inside AchievementFrame). Got \
             `{status_bar_object_type}`. A `nil` reading means the XML name-token resolution \
             failed for the nested-`$parent` chain and the StatusBar is unreachable as a global \
             — the actual update site at lua:2473-2474 (`SetMinMaxValues(0, total)` / \
             `SetValue(completed)`) would crash."
        );

        assert_eq!(
            status_bar_text_object_type, "FontString",
            "Expected `AchievementFrameSummaryCategoriesStatusBarText` to be a FontString \
             (declared in XML at Mainline/Blizzard_AchievementUI.xml:1970 as `$parentText` \
             inside the StatusBar). Got `{status_bar_text_object_type}`. A `nil` reading means \
             the FontString never instantiated or the parent-name expansion dropped — the \
             actual update site at lua:2475 \
             (`SetText(BreakUpLargeNumbers(completed)..\"/\"..BreakUpLargeNumbers(total))`) \
             would crash."
        );

        assert_eq!(
            header_points_after_update, HEADER_POINTS_SENTINEL,
            "Expected `AchievementFrame.Header.Points:GetText()` to STILL equal the sentinel \
             `{HEADER_POINTS_SENTINEL}` after `AchievementFrameSummary_Update()` runs — proving \
             the function does NOT write to Header.Points (PLAN's \"populates Header.Points\" \
             claim is wrong). Got `{header_points_after_update:?}`. Header.Points is updated \
             only by `AchievementFrame_OnShow` at lua:272, `AchievementFrameTab_OnClick` at \
             lua:378, and `AchievementFrameAchievements_OnEvent` at lua:904 — none of which are \
             reached from `AchievementFrameSummary_Update`'s three-sub-call body. A different \
             reading proves a future change re-routed header-points updates through the Summary \
             path; if that's intentional, the absence-half assertion should flip to a \
             presence-half pin (`GetText() == BreakUpLargeNumbers(GetTotalAchievementPoints())`) \
             and the PLAN entry can drop the \"populates Header.Points\" qualifier."
        );

        assert!(
            categories_status_bar_text_after_update.contains('/'),
            "Expected `AchievementFrameSummaryCategoriesStatusBarText:GetText()` to contain a \
             `/` separator after `AchievementFrameSummary_Update()` runs. Got \
             `{categories_status_bar_text_after_update:?}`. The actual update site at lua:2475 \
             writes \
             `BreakUpLargeNumbers(completed)..\"/\"..BreakUpLargeNumbers(total)` so any pair of \
             numbers from `GetNumCompletedAchievements(InGuildView())` produces a `<n>/<n>` \
             string. An empty or no-slash reading means either \
             `AchievementFrameSummaryCategoriesStatusBar_Update` never ran (the inner sub-call \
             was skipped or errored), `BreakUpLargeNumbers` returned a non-string, or the \
             `..\"/\"..` concatenation was refactored away."
        );
    });
}
