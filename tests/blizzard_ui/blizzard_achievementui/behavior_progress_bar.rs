//! Behavior pin: `AchievementFrame_UpdateProgressBar` (PLAN-named) is
//! NOT defined anywhere in the source. PLAN claim is spec-imagined; this
//! test pins the absence as a tripwire and pins the actual call sites
//! that the imagined function would have wrapped.
//!
//! **Spec/source mismatch — the PLAN-named function does not exist.** A
//! grep across both `Mainline/Blizzard_AchievementUI.lua` and
//! `Cata/Blizzard_AchievementUI.lua` finds zero references to
//! `AchievementFrame_UpdateProgressBar` (or any `UpdateProgressBar` symbol
//! at all). The closest matches are:
//!
//! - `AchievementsObjectivesMixin:GetProgressBar(index)` at lua:1720 —
//!   per-row criterion progress bar (one per progress-quest criterion
//!   inside an achievement's objective list, NOT a header-level bar).
//! - `AchievementFrameSummary_UpdateSummaryProgressBars(categories)` at
//!   lua:2303 — per-category progress bars on the Summary tab.
//! - `AchievementFrame.searchProgressBar` — animated bar for the
//!   search-progress poll loop (not a points/completion bar).
//!
//! There is NO header-level progress bar in the Mainline AchievementFrame
//! at all. The header has only a `Points` FontString showing the total
//! points and `Achievements` (a category name field, not a bar). PLAN's
//! "header progress bar tooltip and label" is doubly imagined.
//!
//! **The two PLAN-named C APIs exist and are not gaps.** PLAN tags this
//! item as `(depends-on: GetNumCompletedAchievements gap,
//! GetTotalAchievementPoints gap)` but both are implemented at
//! `src/lua_api/globals/missing_surface/achievement_info.rs:651`
//! (`get_num_completed_achievements`) and `:917`
//! (`get_total_achievement_points`). The depends-on tags are stale.
//!
//! **Where the points label actually gets updated.** The header's
//! `Points` FontString is set inline at four sites in the source:
//! `AchievementFrame_OnShow` at lua:272 (`SetText(BreakUpLargeNumbers(GetTotalAchievementPoints()))`),
//! `AchievementFrameTab_OnClick` at lua:378 (with `InGuildView()` arg),
//! `AchievementFrameAchievements_OnEvent` at lua:904 (also with
//! `InGuildView()` arg), and an inline-color path at lua:352
//! (`SetVertexColor(0, 1, 0)` for guild view) / lua:370
//! (`SetVertexColor(1, 1, 1)` for character view). None of these are
//! factored into a function called `AchievementFrame_UpdateProgressBar`.
//!
//! Five assertions split presence/absence:
//!
//! - **Absence half** (1): `_G.AchievementFrame_UpdateProgressBar` is
//!   nil. A non-nil reading would prove Blizzard extracted the inline
//!   header-points logic into a function (in which case PLAN's spec
//!   becomes accurate retroactively and the absence half should be
//!   replaced by a presence-half method probe).
//! - **Presence half** (4): the two C APIs that PLAN's claim references
//!   exist as functions; `AchievementFrame.Header.Points` is a
//!   FontString; after calling `AchievementFrame_OnShow(AchievementFrame)`,
//!   the FontString's text equals `BreakUpLargeNumbers(GetTotalAchievementPoints())`
//!   (the actual update site at lua:272).

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AchievementUI";
const PLAN_NAMED_BUT_ABSENT_FUNCTION: &str = "AchievementFrame_UpdateProgressBar";

#[test]
fn update_progress_bar_function_is_absent_but_underlying_apis_and_label_path_work() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let observations: (String, String, String, String, String) = env
            .eval(
                r#"
                assert(AchievementFrame, "AchievementFrame global must exist after Blizzard_AchievementUI load")
                assert(AchievementFrame.Header, "AchievementFrame.Header must exist after load")

                local plan_named_function_type = type(_G.AchievementFrame_UpdateProgressBar)
                local total_points_api_type = type(_G.GetTotalAchievementPoints)
                local num_completed_api_type = type(_G.GetNumCompletedAchievements)
                local header_points_type = AchievementFrame.Header.Points
                    and AchievementFrame.Header.Points:GetObjectType()
                    or "nil"

                AchievementFrame_OnShow(AchievementFrame)
                local label_text_after_onshow = AchievementFrame.Header.Points:GetText() or ""

                return plan_named_function_type,
                       total_points_api_type,
                       num_completed_api_type,
                       header_points_type,
                       label_text_after_onshow
                "#,
            )
            .expect("AchievementFrame progress-bar absence/presence probe must run cleanly");

        let (
            plan_named_function_type,
            total_points_api_type,
            num_completed_api_type,
            header_points_type,
            label_text_after_onshow,
        ) = observations;

        assert_eq!(
            plan_named_function_type, "nil",
            "Expected `_G.{PLAN_NAMED_BUT_ABSENT_FUNCTION}` to be nil — the function does not \
             exist in either Mainline or Cata source. Got `{plan_named_function_type}`. A \
             non-nil reading would prove Blizzard extracted the inline header-points logic \
             at lua:272/378/904 into a named function, in which case the PLAN spec becomes \
             accurate retroactively and this absence-half assertion should be replaced by a \
             presence-half method probe (`type(_G.{PLAN_NAMED_BUT_ABSENT_FUNCTION}) == \"function\"`) \
             plus a behavior probe that calls it and asserts the points label updates."
        );

        assert_eq!(
            total_points_api_type, "function",
            "Expected `_G.GetTotalAchievementPoints` to be a function (PLAN tags this as a \
             gap, but it's implemented at \
             `src/lua_api/globals/missing_surface/achievement_info.rs:917`). Got \
             `{total_points_api_type}`. The depends-on tag is stale; if this assertion fails \
             it means the C API was removed or renamed, in which case the inline `:SetText(BreakUpLargeNumbers(GetTotalAchievementPoints()))` \
             call sites at lua:272/378/904 would also crash."
        );

        assert_eq!(
            num_completed_api_type, "function",
            "Expected `_G.GetNumCompletedAchievements` to be a function (PLAN tags this as a \
             gap, but it's implemented at \
             `src/lua_api/globals/missing_surface/achievement_info.rs:651`). Got \
             `{num_completed_api_type}`. The depends-on tag is stale; if this assertion \
             fails it means the C API was removed or renamed, in which case the per-category \
             completion-ratio site at lua:531 (`numAchievements, numCompleted = GetNumCompletedAchievements(InGuildView())`) \
             and the meta-completion site at lua:2472 would also crash."
        );

        assert_eq!(
            header_points_type, "FontString",
            "Expected `AchievementFrame.Header.Points` to be a FontString (the only \
             header-level points/completion display in the Mainline UI — there is no \
             header-level *progress bar* despite PLAN's wording). Got `{header_points_type}`. \
             A `nil` reading means either the parentKey routing dropped (the FontString is \
             declared in the XML header at xml:65+ as a child of AchievementFrame.Header) or \
             the entire Header subtree failed to instantiate."
        );

        assert!(
            !label_text_after_onshow.is_empty(),
            "Expected `AchievementFrame.Header.Points:GetText()` to be non-empty after \
             `AchievementFrame_OnShow(AchievementFrame)` runs. Got `{label_text_after_onshow:?}`. \
             The OnShow handler at lua:272 calls \
             `AchievementFrame.Header.Points:SetText(BreakUpLargeNumbers(GetTotalAchievementPoints()))`. \
             An empty reading means either OnShow never ran (probe error), \
             `GetTotalAchievementPoints()` returned nil/0 with `BreakUpLargeNumbers` producing \
             an empty string, or the SetText call was removed. The simulator's default \
             SimState seeds achievement points so a 0-return is itself a regression worth \
             flagging."
        );
    });
}
