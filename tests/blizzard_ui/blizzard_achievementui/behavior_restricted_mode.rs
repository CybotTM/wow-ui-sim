//! Behavior pin: PLAN-named "AchievementFrame_SetRestrictedMode hides
//! Stats/Summary tabs and forces the active category" collapses FIVE
//! structural facts about the global at
//! `Mainline/Blizzard_AchievementUI.lua:290-303`.
//!
//! 1. **Writes `self.restrictedCategoryID`** — the FIRST line at
//!    lua:291 records the parameter onto the frame. This is the same
//!    field read by `AchievementFrame_TryShowFilterDropdown` at
//!    lua:311 (pinned in `behavior_filter_dropdown.rs`); the two
//!    behaviors share state through this single field. PLAN's
//!    wording elides the storage step — a regression that dropped
//!    the assignment would silently break the dropdown gate without
//!    failing this function's other side effects.
//! 2. **`isRestricted = restrictedCategoryID ~= nil` is the gate
//!    expression.** lua:293 computes a boolean from a presence
//!    check, then drives every subsequent toggle with `not
//!    isRestricted`. The integer `0` is non-nil, so passing `0`
//!    enters restricted mode (matches the truthy-`0` semantics
//!    pinned in `behavior_filter_dropdown.rs`). PLAN's wording
//!    elides the gate computation; a regression that swapped to
//!    `restrictedCategoryID and restrictedCategoryID > 0` would
//!    let `0` through.
//! 3. **Symmetric two-state on Header + SearchBox + tabs.** lua:294
//!    `self.Header:SetShown(not isRestricted)`, lua:295
//!    `self.SearchBox:SetShown(not isRestricted)`, lua:300
//!    `PanelTemplates_SetAllTabsShown(self, not isRestricted)`.
//!    PLAN says "Stats/Summary tabs" — the actual API hides ALL
//!    tabs (the generic helper takes the frame; it does not
//!    distinguish Stats/Summary from any other tab). The Header
//!    surface is the WHOLE header (Points, Title, Shield, Emblems)
//!    — not just the dropdown row. A regression that narrowed the
//!    PanelTemplates call to a per-tab list would leave non-
//!    Stats/Summary tabs visible in restricted mode.
//! 4. **Asymmetric `searchProgressBar:Hide` fires ONLY in restricted
//!    branch.** lua:296-298 wraps `self.searchProgressBar:Hide()`
//!    in `if isRestricted then ... end`. The non-restricted branch
//!    does NOT call `:Show()` on the progress bar — its visibility
//!    is owned by the search lifecycle (lua:3262/3310), not by this
//!    function. PLAN's wording would lead a reader to expect a
//!    symmetric pair; the test pins one-way Hide so a regression
//!    that mirrored it would change `progress_bar_hide_calls=0` on
//!    the non-restricted drive.
//! 5. **Forwards `restrictedCategoryID` verbatim to
//!    `AchievementFrame_UpdateAndSelectCategory`.** lua:302 passes
//!    the parameter (which may be `nil`) to the helper. PLAN's
//!    "forces the active category" wording elides the forward —
//!    the helper at lua:2600-2622 is what does the actual work
//!    (short-circuits when `GetSelectedCategory() == category`,
//!    otherwise `ExpandToCategory` + `UpdateDataProvider` + scroll-
//!    to-selection). A test installs a spy on the helper and pins
//!    the argument round-trip for both `nil` (non-restricted teardown
//!    still calls through, helper is responsible for the no-op) and
//!    a concrete id (restricted entry).
//!
//! Source map of the contract:
//!
//! ```lua
//! function AchievementFrame_SetRestrictedMode (self, restrictedCategoryID)  -- lua:290
//!     self.restrictedCategoryID = restrictedCategoryID                      -- lua:291
//!
//!     local isRestricted = restrictedCategoryID ~= nil                      -- lua:293
//!     self.Header:SetShown(not isRestricted)                                -- lua:294
//!     self.SearchBox:SetShown(not isRestricted)                             -- lua:295
//!     if isRestricted then                                                  -- lua:296
//!         self.searchProgressBar:Hide()                                     -- lua:297
//!     end
//!
//!     PanelTemplates_SetAllTabsShown(self, not isRestricted)                -- lua:300
//!
//!     AchievementFrame_UpdateAndSelectCategory(restrictedCategoryID)        -- lua:302
//! end
//! ```
//!
//! One test drives both states (entry + teardown) and pins all five
//! axes in a single combined signature; the body fits within the
//! readability budget because the fake-frame surface is small.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AchievementUI";
const RESTRICTED_CATEGORY_ID: i64 = 7;

const FAKE_FRAME_BUILDER: &str = r#"
    local function counter(captures, key)
        return function() captures[key] = (captures[key] or 0) + 1 end
    end
    local function shown_setter(captures, key)
        return function(self, shown)
            captures[key] = (captures[key] or 0) + 1
            captures[key .. "_arg"] = tostring(shown)
        end
    end
    local function build_frame(captures)
        local frame = {}
        frame.Header = {SetShown = shown_setter(captures, "header_set_shown")}
        frame.SearchBox = {SetShown = shown_setter(captures, "search_box_set_shown")}
        frame.searchProgressBar = {Hide = counter(captures, "progress_bar_hide_calls")}
        return frame
    end
    local function install_panel_templates_spy(captures)
        _G.PanelTemplates_SetAllTabsShown = function(frame, shown)
            captures.panel_templates_set_all_tabs_shown_calls =
                (captures.panel_templates_set_all_tabs_shown_calls or 0) + 1
            captures.panel_templates_arg = tostring(shown)
        end
    end
    local function install_update_and_select_category_spy(captures)
        _G.AchievementFrame_UpdateAndSelectCategory = function(category)
            captures.update_and_select_category_calls =
                (captures.update_and_select_category_calls or 0) + 1
            captures.update_and_select_category_arg = tostring(category)
        end
    end
    local function signature(captures)
        return string.format(
            "restricted_category_id=%s header_set_shown=%d header_set_shown_arg=%s " ..
            "search_box_set_shown=%d search_box_set_shown_arg=%s " ..
            "progress_bar_hide_calls=%d " ..
            "panel_templates_calls=%d panel_templates_arg=%s " ..
            "update_and_select_category_calls=%d update_and_select_category_arg=%s",
            tostring(captures.restricted_category_id_after),
            captures.header_set_shown or 0,
            tostring(captures.header_set_shown_arg),
            captures.search_box_set_shown or 0,
            tostring(captures.search_box_set_shown_arg),
            captures.progress_bar_hide_calls or 0,
            captures.panel_templates_set_all_tabs_shown_calls or 0,
            tostring(captures.panel_templates_arg),
            captures.update_and_select_category_calls or 0,
            tostring(captures.update_and_select_category_arg))
    end
"#;

#[test]
fn set_restricted_mode_writes_restricted_category_id_and_two_state_toggles_header_search_and_tabs_and_forwards_to_update_category()
 {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        let observations: String = env
            .eval(&format!(
                r#"
                assert(type(_G.AchievementFrame_SetRestrictedMode) == "function",
                    "AchievementFrame_SetRestrictedMode must be a global function (lua:290)")
                assert(type(_G.PanelTemplates_SetAllTabsShown) == "function",
                    "PanelTemplates_SetAllTabsShown must exist as a global (lua:300)")
                assert(type(_G.AchievementFrame_UpdateAndSelectCategory) == "function",
                    "AchievementFrame_UpdateAndSelectCategory must exist as a global (lua:302/2600)")

                {fake_frame_builder}

                local restricted_captures = {{}}
                local restricted_frame = build_frame(restricted_captures)
                install_panel_templates_spy(restricted_captures)
                install_update_and_select_category_spy(restricted_captures)
                AchievementFrame_SetRestrictedMode(restricted_frame, {restricted_id})
                restricted_captures.restricted_category_id_after = restricted_frame.restrictedCategoryID
                local restricted_signature = signature(restricted_captures)

                local nonrestricted_captures = {{}}
                local nonrestricted_frame = build_frame(nonrestricted_captures)
                nonrestricted_frame.restrictedCategoryID = {restricted_id}
                install_panel_templates_spy(nonrestricted_captures)
                install_update_and_select_category_spy(nonrestricted_captures)
                AchievementFrame_SetRestrictedMode(nonrestricted_frame, nil)
                nonrestricted_captures.restricted_category_id_after =
                    nonrestricted_frame.restrictedCategoryID
                local nonrestricted_signature = signature(nonrestricted_captures)

                return string.format(
                    "restricted[%s] nonrestricted[%s]",
                    restricted_signature, nonrestricted_signature)
                "#,
                fake_frame_builder = FAKE_FRAME_BUILDER,
                restricted_id = RESTRICTED_CATEGORY_ID,
            ))
            .expect("AchievementFrame_SetRestrictedMode must be callable on a fake frame");

        let restricted_expected = format!(
            "restricted_category_id={id} header_set_shown=1 header_set_shown_arg=false \
             search_box_set_shown=1 search_box_set_shown_arg=false \
             progress_bar_hide_calls=1 \
             panel_templates_calls=1 panel_templates_arg=false \
             update_and_select_category_calls=1 update_and_select_category_arg={id}",
            id = RESTRICTED_CATEGORY_ID
        );
        let nonrestricted_expected = "restricted_category_id=nil header_set_shown=1 header_set_shown_arg=true \
             search_box_set_shown=1 search_box_set_shown_arg=true \
             progress_bar_hide_calls=0 \
             panel_templates_calls=1 panel_templates_arg=true \
             update_and_select_category_calls=1 update_and_select_category_arg=nil";
        let expected =
            format!("restricted[{restricted_expected}] nonrestricted[{nonrestricted_expected}]");

        assert_eq!(
            observations, expected,
            "SetRestrictedMode contract pinned at lua:290-303: \
             restricted entry writes restrictedCategoryID, hides Header + SearchBox + ALL tabs \
             (PanelTemplates_SetAllTabsShown is generic, NOT Stats/Summary-specific), and \
             additionally hides searchProgressBar (asymmetric — fires ONLY in restricted branch). \
             Non-restricted teardown clears restrictedCategoryID to nil, shows Header + SearchBox + \
             tabs, does NOT touch searchProgressBar (no symmetric Show), and still calls through to \
             UpdateAndSelectCategory(nil) — the helper short-circuits on no-change. \
             Regression candidates: missing storage write, asymmetric branches mirrored, generic \
             tabs API narrowed to per-tab list, or the UpdateAndSelectCategory forward dropped."
        );
    });
}
