//! Behavior pin: `AchievementFrameAchievements_OnEvent` routes the
//! `CRITERIA_UPDATE` event into `AchievementFrameAchievements_OnCriteriaUpdate()`,
//! AND `_OnCriteriaUpdate` is a NO-OP unless a row is currently
//! selected (the handler re-Inits ONLY the selected row, not "the
//! visible objectives strip" PLAN's wording suggests).
//!
//! Source map:
//!
//! ```lua
//! -- lua:925-953 (the dispatcher)
//! function AchievementFrameAchievements_OnEvent (self, event, ...)
//!     ...
//!     elseif ( event == "CRITERIA_UPDATE" ) then
//!         AchievementFrameAchievements_OnCriteriaUpdate();   -- line 939, no args
//!     ...
//! end
//! ```
//!
//! ```lua
//! -- lua:907-915 (the per-event handler)
//! function AchievementFrameAchievements_OnCriteriaUpdate()
//!     local selectedElementData = AchievementFrameAchievements_GetSelectedElementData();   -- line 908
//!     if selectedElementData then                                                          -- line 909
//!         local button = AchievementFrameAchievements.ScrollBox:FindFrame(selectedElementData);  -- line 910
//!         if button then                                                                   -- line 911
//!             button:Init(selectedElementData);                                            -- line 912
//!         end
//!     end
//! end
//! ```
//!
//! ```lua
//! -- lua:886-893 (the selection accessor)
//! function AchievementFrameAchievements_GetSelectedElementData()
//!     return g_achievementSelectionBehavior:GetFirstSelectedElementData();
//! end
//! ```
//!
//! XML binding (xml:1795): `<OnEvent function="AchievementFrameAchievements_OnEvent"/>`.
//! `CRITERIA_UPDATE` is registered for the frame inside `_OnShow` at
//! lua:805 (`FrameUtil.RegisterFrameForEvents(self, AchievementFrameShownEvents)`,
//! the table at lua:796-802 also lists `ACHIEVEMENT_EARNED`,
//! `RECEIVED_ACHIEVEMENT_MEMBER_LIST`, `ACHIEVEMENT_SEARCH_UPDATED`).
//! `_OnHide` at lua:815 unregisters them. The dispatch routing logic
//! itself lives inside the body of `_OnEvent`, independent of whether
//! `RegisterFrameForEvents` has run; calling
//! `_OnEvent(self, "CRITERIA_UPDATE")` directly exercises the routing
//! without requiring the frame to be shown first.
//!
//! **Spec/source mismatch on THREE axes around "refreshes the visible
//! objectives strip":**
//!
//! 1. **Scope: the handler refreshes ONE row (the selected one),
//!    not "the visible objectives strip".** Only the row whose
//!    elementData matches the selection behavior's first selection is
//!    re-Inited at lua:912. Any other visible row keeps its previous
//!    state until its own next refresh edge. PLAN's "the visible
//!    objectives strip" wording suggests an iterate-all pattern that
//!    does not exist in the source.
//! 2. **Mechanism: the row's Init re-reads ALL row data, not just
//!    objectives.** `button:Init(selectedElementData)` at lua:912
//!    enters `AchievementTemplateMixin:Init(elementData)` (the
//!    template's full row constructor), which rebuilds the header,
//!    points, icon, status bar, criteria rows, and the rep-criteria
//!    strip — not a narrow "objectives" subset.
//! 3. **No-selection gate: the handler is a complete no-op when
//!    nothing is selected.** PLAN's wording "refreshes the visible
//!    objectives strip" implies unconditional refresh. In reality, the
//!    `if selectedElementData then` guard at lua:909 short-circuits the
//!    entire body when `_GetSelectedElementData()` returns nil — which
//!    is the default in the smoke-shape harness because nothing has
//!    been selected via `g_achievementSelectionBehavior`. The
//!    `findFrame_called` flag below pins this no-op path: when no row
//!    is selected, `ScrollBox:FindFrame` is never reached, so a class
//!    of regressions (e.g. an unconditional `:FindFrame` call before
//!    the guard) would trip the assertion.
//!
//! Eight assertions split presence/behavior:
//!
//! - **Presence half** (4): `_G.AchievementFrameAchievements_OnEvent`,
//!   `_G.AchievementFrameAchievements_OnCriteriaUpdate`, and
//!   `_G.AchievementFrameAchievements_GetSelectedElementData` are all
//!   functions; `AchievementFrameAchievements.ScrollBox` exists and
//!   `ScrollBox.FindFrame` is a callable method (the access point at
//!   lua:910).
//! - **Behavior half** (4):
//!   - The spy on `_OnCriteriaUpdate` fires exactly once when
//!     `_OnEvent(self, "CRITERIA_UPDATE")` is called (proves the
//!     routing at lua:938 → 939).
//!   - `_GetSelectedElementData()` returns `nil` in the default smoke
//!     state (no row selected — the precondition that makes the
//!     `if selectedElementData` guard at lua:909 short-circuit).
//!   - `pcall(_OnCriteriaUpdate)` succeeds AND the inner ScrollBox
//!     `findFrame_called` spy flag stays `false` after the call (proves
//!     the no-op gate: the handler returns without entering the
//!     if-branch and without calling `:FindFrame`).
//!   - The full event drive (`_OnEvent(self, "CRITERIA_UPDATE")`)
//!     completes via `pcall` returning `true` (no error from the
//!     dispatcher → handler → guard → return chain).

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AchievementUI";

type CriteriaUpdateProbe = (String, String, String, String, i64, bool, bool, bool);

#[test]
fn criteria_update_event_routes_on_event_into_on_criteria_update_which_no_ops_without_selection() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let setup_ok: bool = env
            .eval(
                r#"
                assert(AchievementFrameAchievements, "AchievementFrameAchievements must exist")
                assert(AchievementFrameAchievements.ScrollBox, "ScrollBox must exist")

                _G.__test_on_criteria_count = 0
                _G.__test_original_on_criteria = _G.AchievementFrameAchievements_OnCriteriaUpdate
                _G.AchievementFrameAchievements_OnCriteriaUpdate = function(...)
                    _G.__test_on_criteria_count = _G.__test_on_criteria_count + 1
                    return _G.__test_original_on_criteria(...)
                end

                _G.__test_findframe_called = false
                _G.__test_original_findframe = AchievementFrameAchievements.ScrollBox.FindFrame
                AchievementFrameAchievements.ScrollBox.FindFrame = function(scrollBox, elementData)
                    _G.__test_findframe_called = true
                    return _G.__test_original_findframe(scrollBox, elementData)
                end
                return true
                "#,
            )
            .expect("setup phase must run cleanly (spies install)");
        assert!(setup_ok, "setup eval must return true");

        let drive_ok: bool = env
            .eval(
                r#"
                local ok, _ = pcall(AchievementFrameAchievements_OnEvent,
                    AchievementFrameAchievements, "CRITERIA_UPDATE")
                _G.__test_full_drive_ok = ok and true or false
                return ok and true or false
                "#,
            )
            .expect("event drive must run cleanly");
        assert!(drive_ok, "drive eval must return true");

        let observations: CriteriaUpdateProbe = env
            .eval(
                r#"
                local on_event_type = type(_G.AchievementFrameAchievements_OnEvent)
                local on_criteria_type = type(_G.__test_original_on_criteria)
                local get_selected_type =
                    type(_G.AchievementFrameAchievements_GetSelectedElementData)
                local findframe_type = type(_G.__test_original_findframe)

                local count = _G.__test_on_criteria_count or -1
                local selected_is_nil =
                    AchievementFrameAchievements_GetSelectedElementData() == nil
                local findframe_called = _G.__test_findframe_called and true or false
                local full_drive_ok = _G.__test_full_drive_ok and true or false

                AchievementFrameAchievements.ScrollBox.FindFrame = _G.__test_original_findframe
                _G.AchievementFrameAchievements_OnCriteriaUpdate =
                    _G.__test_original_on_criteria
                _G.__test_original_findframe = nil
                _G.__test_original_on_criteria = nil
                _G.__test_on_criteria_count = nil
                _G.__test_findframe_called = nil
                _G.__test_full_drive_ok = nil

                return on_event_type,
                       on_criteria_type,
                       get_selected_type,
                       findframe_type,
                       count,
                       selected_is_nil,
                       findframe_called,
                       full_drive_ok
                "#,
            )
            .expect("post-event probe must run cleanly");

        let (
            on_event_type,
            on_criteria_type,
            get_selected_type,
            findframe_type,
            count,
            selected_is_nil,
            findframe_called,
            full_drive_ok,
        ) = observations;

        assert_eq!(
            on_event_type, "function",
            "Expected `_G.AchievementFrameAchievements_OnEvent` to be a function (declared at \
             `Mainline/Blizzard_AchievementUI.lua:925`, bound via xml:1795 \
             `<OnEvent function=\"AchievementFrameAchievements_OnEvent\"/>`). Got \
             `{on_event_type}`. A `nil` reading means the dispatcher global never registered, in \
             which case no event in the shown-events table at lua:796-802 (ACHIEVEMENT_EARNED, \
             CRITERIA_UPDATE, RECEIVED_ACHIEVEMENT_MEMBER_LIST, ACHIEVEMENT_SEARCH_UPDATED) \
             could route into a per-event handler."
        );

        assert_eq!(
            on_criteria_type, "function",
            "Expected the original `_G.AchievementFrameAchievements_OnCriteriaUpdate` to be a \
             function (declared at `Mainline/Blizzard_AchievementUI.lua:907`). Got \
             `{on_criteria_type}`. A `nil` reading means the dispatch at lua:939 \
             (`AchievementFrameAchievements_OnCriteriaUpdate()`) would crash, breaking criteria \
             refresh on every server criteria-update tick."
        );

        assert_eq!(
            get_selected_type, "function",
            "Expected `_G.AchievementFrameAchievements_GetSelectedElementData` to be a function \
             (declared at `Mainline/Blizzard_AchievementUI.lua:886`, returns \
             `g_achievementSelectionBehavior:GetFirstSelectedElementData()`). Got \
             `{get_selected_type}`. A `nil` reading means the gate at lua:908 \
             (`local selectedElementData = AchievementFrameAchievements_GetSelectedElementData()`) \
             would crash before the guard could short-circuit, surfacing as a `nil value` error \
             on every CRITERIA_UPDATE while the achievement frame is shown."
        );

        assert_eq!(
            findframe_type, "function",
            "Expected `AchievementFrameAchievements.ScrollBox.FindFrame` to be a function (the \
             ScrollBoxList method that maps elementData → button widget; used at lua:910 \
             `local button = AchievementFrameAchievements.ScrollBox:FindFrame(selectedElementData)`). \
             Got `{findframe_type}`. A `nil` reading means the ScrollBox view didn't initialize \
             (e.g. `AchievementFrameAchievements_OnLoad` at lua:843 didn't run), which would \
             also break selection-driven scrolling at lua:899 \
             (`AchievementFrame_SelectAndScrollToAchievementId(...)`)."
        );

        assert_eq!(
            count, 1,
            "Expected the spy on `_G.AchievementFrameAchievements_OnCriteriaUpdate` to fire \
             exactly once when driving `_OnEvent(self, \"CRITERIA_UPDATE\")`. Got `{count}`. A \
             count of 0 means the elseif branch at lua:938 did not match (event-string \
             mismatch — most likely a refactor that changed the event name or moved the \
             dispatch into a sub-handler); a count > 1 means the handler was called multiple \
             times (re-entrant dispatch — unexpected from a single direct invocation)."
        );

        assert!(
            selected_is_nil,
            "Expected `_GetSelectedElementData()` to return `nil` in the default smoke-shape \
             state (no row has been selected via `g_achievementSelectionBehavior`). Got \
             non-nil. This is the precondition that makes the `if selectedElementData` guard at \
             lua:909 short-circuit; without it, the test's no-op assertion below would not be \
             well-formed (the handler would proceed into the `:FindFrame`/`:Init` branch and the \
             findframe spy below would record a call). A non-nil reading here means either the \
             smoke harness now seeds a default selection OR an addon load-edge auto-selected the \
             first row — adjust the test to assert the selected-branch behavior instead."
        );

        assert!(
            !findframe_called,
            "Expected `ScrollBox.FindFrame` to NOT be called during the no-selection \
             CRITERIA_UPDATE drive — the guard at lua:909 (`if selectedElementData then`) should \
             short-circuit BEFORE the `:FindFrame` call at lua:910. Got `findframe_called == \
             true`. A `true` reading means the handler reached the inner branch despite the \
             guard, which would happen if the guard was inverted, removed, or if the spy got \
             called during setup (the spy installation path itself must not invoke FindFrame). \
             This is the central pin against PLAN's \"refreshes the visible objectives strip\" \
             wording — the handler does NOT iterate visible rows; it gates entirely on the \
             selection."
        );

        assert!(
            full_drive_ok,
            "Expected `pcall(AchievementFrameAchievements_OnEvent, self, \"CRITERIA_UPDATE\")` \
             to succeed. Got `false`. A `false` reading means the dispatcher → handler → guard \
             → return chain raised an error somewhere — likely either the dispatcher's \
             `Kiosk.IsEnabled()` branch at lua:926 hit a missing API, or the handler's \
             selection accessor at lua:908 raised because `g_achievementSelectionBehavior` was \
             not initialized (the upvalue is set in `AchievementFrameAchievements_OnLoad` at \
             lua:875-880; if OnLoad didn't fire, the accessor crashes before the guard can \
             short-circuit)."
        );
    });
}
