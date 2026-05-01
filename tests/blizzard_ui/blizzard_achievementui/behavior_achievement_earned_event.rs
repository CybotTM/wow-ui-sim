//! Behavior pin: `AchievementFrameAchievements_OnEvent` routes the
//! `ACHIEVEMENT_EARNED` event into `AchievementFrameAchievements_OnAchievementEarned(id)`,
//! AND `_OnAchievementEarned` updates `AchievementFrame.Header.Points`
//! via `BreakUpLargeNumbers(GetTotalAchievementPoints(InGuildView()))`.
//!
//! Source map:
//!
//! ```lua
//! -- lua:925-953 (the dispatch table)
//! function AchievementFrameAchievements_OnEvent (self, event, ...)
//!     ...
//!     elseif ( event == "ACHIEVEMENT_EARNED" ) then
//!         if not AchievementFrameCategories.ScrollBox:HasDataProvider() then  -- line 932
//!             AchievementFrameCategories_UpdateDataProvider();
//!         end
//!         local achievementID = ...;                                          -- line 936
//!         AchievementFrameAchievements_OnAchievementEarned(achievementID);    -- line 937
//!     elseif ( event == "CRITERIA_UPDATE" ) then ...
//! end
//! ```
//!
//! ```lua
//! -- lua:895-905 (the actual update fan-out)
//! function AchievementFrameAchievements_OnAchievementEarned(achievementId)
//!     AchievementFrameAchievements_UpdateDataProvider();                                          -- line 896
//!     if AchievementFrameAchievements_GetSelectedAchievementId() == achievementId then            -- line 898
//!         AchievementFrame_SelectAndScrollToAchievementId(AchievementFrameAchievements.ScrollBox,
//!             achievementId);
//!     end
//!     AchievementFrameCategories_UpdateTooltip();                                                 -- line 902
//!     AchievementFrame.Header.Points:SetText(BreakUpLargeNumbers(GetTotalAchievementPoints(InGuildView())));  -- line 904
//! end
//! ```
//!
//! XML binding (xml:1795): `<OnEvent function="AchievementFrameAchievements_OnEvent"/>`.
//! `ACHIEVEMENT_EARNED` is registered for the frame inside `_OnShow` at lua:805
//! (`FrameUtil.RegisterFrameForEvents(self, AchievementFrameShownEvents)` — the
//! shown-events table at lua:796-802 includes `ACHIEVEMENT_EARNED`,
//! `CRITERIA_UPDATE`, `RECEIVED_ACHIEVEMENT_MEMBER_LIST`,
//! `ACHIEVEMENT_SEARCH_UPDATED`). `_OnHide` at lua:815 unregisters them. The
//! dispatch routing logic itself lives inside the body of `_OnEvent` and is
//! independent of whether `RegisterFrameForEvents` has run; calling
//! `_OnEvent(self, "ACHIEVEMENT_EARNED", id)` directly exercises the routing
//! without requiring the frame to be shown first.
//!
//! Rust-side state plumbing:
//!
//! - `state.world.earned_achievements: HashSet<i32>`
//!   (`src/lua_api/state_types/character_world.rs:399`) is the authoritative
//!   "earned" store. The `ACHIEVEMENT_EARNED` event handler does NOT itself
//!   write to that set — the handler just refreshes UI state that READS
//!   from it. To test the points-text update, the test pre-seeds the set
//!   from Rust before driving the event.
//! - `GetTotalAchievementPoints(isGuild)` at
//!   `src/lua_api/globals/missing_surface/achievement_info.rs:917-932`
//!   sums `info.points` for every achievement id present in
//!   `world.earned_achievements` for the requested view. Id 6 ("Level 10",
//!   `state.rs` `achievement_level_ten`) has `points: 10`.
//! - `BreakUpLargeNumbers(10)` returns the string `"10"` (no thousands
//!   separator below 1000), so a clean `Header.Points` text after the event
//!   is exactly `"10"`.
//!
//! **Spec/source mismatch on TWO axes — PLAN's "marks the matching
//! button row complete" wording is wrong on both the scope and the
//! mechanism axes:**
//!
//! 1. **Scope: the handler refreshes ALL rows, not just the matching
//!    one.** `_UpdateDataProvider()` at lua:896 rebuilds the entire
//!    achievement list's data provider. The matching-id branch at
//!    lua:898-900 only triggers a SCROLL (not a "complete" mark) and
//!    only when the earned id happens to be the currently-selected one.
//!    Most calls do not enter this branch.
//! 2. **Mechanism: the "complete" state is not WRITTEN by the handler;
//!    it is READ from `world.earned_achievements`.** PLAN's wording
//!    suggests the handler imperatively flips a row to "complete". In
//!    reality, the row's `Init(elementData)` reads completion via
//!    `GetAchievementInfo(id)` at next render time, which queries
//!    `world.earned_achievements.contains(id)`. The handler's
//!    contribution is the `_UpdateDataProvider()` call that triggers the
//!    re-render; the underlying state mutation must happen separately
//!    (in real WoW, the server message that delivers `ACHIEVEMENT_EARNED`
//!    also marks the achievement complete client-side; in the simulator,
//!    test code or admin globals seed `world.earned_achievements`).
//!
//! Additionally PLAN omits the side effects: `_OnAchievementEarned` also
//! refreshes `AchievementFrameCategories_UpdateTooltip()` (lua:902) and
//! updates `AchievementFrame.Header.Points` (lua:904), the latter being
//! the user-visible total-points readout that this test pins.
//!
//! Eight assertions split presence/behavior:
//!
//! - **Presence half** (4): `_G.AchievementFrameAchievements_OnEvent`,
//!   `_G.AchievementFrameAchievements_OnAchievementEarned`, and
//!   `_G.GetTotalAchievementPoints` are all functions; the
//!   `AchievementFrame.Header.Points` widget exists as a `FontString`.
//! - **Behavior half** (4):
//!   - The spy on `_OnAchievementEarned` fires exactly once when
//!     `_OnEvent(self, "ACHIEVEMENT_EARNED", 6)` is called (proves the
//!     routing at lua:931 → 937).
//!   - The spy captures the id arg as `6` (proves the vararg unpack at
//!     lua:936 forwards the first event arg correctly).
//!   - `AchievementFrame.Header.Points:GetText() == "10"` after the
//!     event (proves the points readout was rebuilt — the sentinel set
//!     pre-event was clobbered AND the new text matches
//!     `BreakUpLargeNumbers(GetTotalAchievementPoints(false))` for an
//!     `earned_achievements = {6}` state).
//!   - `GetAchievementInfo(6)`'s 4th return (`completed`) is `true` —
//!     pins the underlying state read that any row's `Init` would use to
//!     mark the row visually complete on the next render. PLAN's "marks
//!     the matching button row complete" semantic resolves to this read
//!     edge, not a write performed by the event handler itself.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AchievementUI";
const SEEDED_ACHIEVEMENT_ID: i32 = 6;
const SEEDED_ACHIEVEMENT_POINTS_TEXT: &str = "10";
const HEADER_POINTS_SENTINEL: &str = "SENTINEL_HEADER_POINTS_PRE_EARN";

type EarnedEventProbe = (String, String, String, String, i64, i64, String, bool);

#[test]
fn achievement_earned_event_routes_on_event_into_on_achievement_earned_and_updates_header_points() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let setup_ok: bool = env
            .eval(
                r#"
                assert(AchievementFrame, "AchievementFrame must exist after addon load")
                assert(AchievementFrame.Header, "AchievementFrame.Header must exist")
                assert(AchievementFrame.Header.Points, "AchievementFrame.Header.Points must exist")
                assert(AchievementFrameAchievements, "AchievementFrameAchievements frame must exist")

                _G.__test_on_earned_count = 0
                _G.__test_on_earned_captured_id = -1
                _G.__test_original_on_earned = _G.AchievementFrameAchievements_OnAchievementEarned
                _G.AchievementFrameAchievements_OnAchievementEarned = function(achievementId)
                    _G.__test_on_earned_count = _G.__test_on_earned_count + 1
                    _G.__test_on_earned_captured_id = achievementId or -1
                    return _G.__test_original_on_earned(achievementId)
                end

                AchievementFrame.Header.Points:SetText("SENTINEL_HEADER_POINTS_PRE_EARN")
                return true
                "#,
            )
            .expect("setup phase must run cleanly (sentinel + spy install)");
        assert!(setup_ok, "setup eval must return true");

        {
            let mut state = env.state().borrow_mut();
            state
                .world
                .earned_achievements
                .insert(SEEDED_ACHIEVEMENT_ID);
        }

        let drive_ok: bool = env
            .eval(
                r#"
                AchievementFrameAchievements_OnEvent(AchievementFrameAchievements,
                    "ACHIEVEMENT_EARNED", 6)
                return true
                "#,
            )
            .expect("driving _OnEvent('ACHIEVEMENT_EARNED', 6) must run cleanly");
        assert!(drive_ok, "drive eval must return true");

        let observations: EarnedEventProbe = env
            .eval(
                r#"
                local on_event_type = type(_G.AchievementFrameAchievements_OnEvent)
                local on_earned_type = type(_G.__test_original_on_earned)
                local total_points_api_type = type(_G.GetTotalAchievementPoints)
                local header_points_object_type =
                    AchievementFrame.Header.Points:GetObjectType()

                local count = _G.__test_on_earned_count or -1
                local captured_id = _G.__test_on_earned_captured_id or -1
                local header_points_text = AchievementFrame.Header.Points:GetText() or ""
                local _, _, _, completed = C_AchievementInfo.GetAchievementInfo(6)
                local completed_flag = completed and true or false

                _G.AchievementFrameAchievements_OnAchievementEarned = _G.__test_original_on_earned
                _G.__test_original_on_earned = nil
                _G.__test_on_earned_count = nil
                _G.__test_on_earned_captured_id = nil

                return on_event_type,
                       on_earned_type,
                       total_points_api_type,
                       header_points_object_type,
                       count,
                       captured_id,
                       header_points_text,
                       completed_flag
                "#,
            )
            .expect("post-event probe must run cleanly");

        let (
            on_event_type,
            on_earned_type,
            total_points_api_type,
            header_points_object_type,
            count,
            captured_id,
            header_points_text,
            completed_flag,
        ) = observations;

        assert_eq!(
            on_event_type, "function",
            "Expected `_G.AchievementFrameAchievements_OnEvent` to be a function (declared at \
             `Mainline/Blizzard_AchievementUI.lua:925` and bound via xml:1795 \
             `<OnEvent function=\"AchievementFrameAchievements_OnEvent\"/>`). Got \
             `{on_event_type}`. A `nil` reading means the addon's Lua chunk never registered the \
             global — the entire shown-events dispatch table at lua:931-953 (ACHIEVEMENT_EARNED, \
             CRITERIA_UPDATE, RECEIVED_ACHIEVEMENT_MEMBER_LIST, ACHIEVEMENT_SEARCH_UPDATED) would \
             be unreachable."
        );

        assert_eq!(
            on_earned_type, "function",
            "Expected the original `_G.AchievementFrameAchievements_OnAchievementEarned` to be a \
             function (declared at `Mainline/Blizzard_AchievementUI.lua:895`). Got \
             `{on_earned_type}`. A `nil` reading means the global was never registered, in which \
             case the dispatch at lua:937 \
             (`AchievementFrameAchievements_OnAchievementEarned(achievementID)`) would crash with \
             `attempt to call a nil value`."
        );

        assert_eq!(
            total_points_api_type, "function",
            "Expected `_G.GetTotalAchievementPoints` to be a function (impl at \
             `src/lua_api/globals/missing_surface/achievement_info.rs:917-932`, sums \
             `info.points` for every id in `world.earned_achievements` filtered by \
             `categories_for_view(is_guild_view)`). Got `{total_points_api_type}`. A `nil` reading \
             means the inline call at lua:904 \
             (`SetText(BreakUpLargeNumbers(GetTotalAchievementPoints(InGuildView())))`) would crash."
        );

        assert_eq!(
            header_points_object_type, "FontString",
            "Expected `AchievementFrame.Header.Points` to be a `FontString` (declared at \
             `Mainline/Blizzard_AchievementUI.xml:1713` as \
             `<FontString parentKey=\"Points\" inherits=\"GameFontHighlight\">` inside the Header \
             frame). Got `{header_points_object_type}`. A different object type means the XML \
             parentKey resolution dropped or the FontString was redeclared as another widget type \
             — either way, `:SetText`/`:GetText` semantics break and the points readout would no \
             longer reflect the earned total."
        );

        assert_eq!(
            count, 1,
            "Expected the spy on `_G.AchievementFrameAchievements_OnAchievementEarned` to fire \
             exactly once when driving `_OnEvent(self, \"ACHIEVEMENT_EARNED\", 6)`. Got \
             `{count}`. A count of 0 means the elseif branch at lua:931 did not match (event \
             string mismatch — most likely a refactor that changed the event name or split the \
             dispatch into a sub-handler); a count > 1 means the handler was called multiple \
             times (re-entrant dispatch — unexpected from a single direct invocation)."
        );

        assert_eq!(
            captured_id, SEEDED_ACHIEVEMENT_ID as i64,
            "Expected the spy to capture `achievementId == {SEEDED_ACHIEVEMENT_ID}` from the \
             vararg forward at lua:936 (`local achievementID = ...`) into the call at lua:937 \
             (`AchievementFrameAchievements_OnAchievementEarned(achievementID)`). Got \
             `{captured_id}`. A `-1` reading means the spy never received an arg (nil → -1 default); \
             any other id means the vararg unpack regressed (e.g. someone added an extra leading \
             arg before the id, shifting the slot)."
        );

        assert_eq!(
            header_points_text, SEEDED_ACHIEVEMENT_POINTS_TEXT,
            "Expected `AchievementFrame.Header.Points:GetText() == {SEEDED_ACHIEVEMENT_POINTS_TEXT:?}` \
             after the event — the pre-event sentinel `{HEADER_POINTS_SENTINEL:?}` should have \
             been clobbered by the inline write at lua:904 \
             (`SetText(BreakUpLargeNumbers(GetTotalAchievementPoints(InGuildView())))`). Got \
             `{header_points_text:?}`. If still equal to the sentinel: the routing at lua:931 → \
             937 → 904 broke (most likely the event dispatch never reached `_OnAchievementEarned`); \
             if `\"0\"`: `world.earned_achievements` was not seeded with id 6 before the call, OR \
             `GetTotalAchievementPoints(false)` returned 0 because id 6's category is not in \
             `ACHIEVEMENT_CATEGORIES`; if any other number: the seeded points value at \
             `state.rs::achievement_level_ten` (`points: 10`) was changed."
        );

        assert!(
            completed_flag,
            "Expected `C_AchievementInfo.GetAchievementInfo(6)`'s 4th return (`completed`) to be \
             `true` after `world.earned_achievements.insert(6)` — the impl at \
             `achievement_info.rs:961` reads \
             `let completed = sim.world.earned_achievements.contains(&achievement_id);` and \
             returns it as the 4th tuple slot. Got `false`. This is the read edge that PLAN's \
             \"marks the matching button row complete\" wording resolves to: the row's `Init` at \
             lua:862 reads completion via `GetAchievementInfo(elementData.id)` on next render \
             after `_UpdateDataProvider()` rebuilds. A `false` reading means either the seed was \
             dropped (state borrow released too early before insert committed) or the impl no \
             longer queries `world.earned_achievements`."
        );
    });
}
