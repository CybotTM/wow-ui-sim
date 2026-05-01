//! Surface-level globals pinned by the `Blizzard_AchievementUI` lane.
//!
//! PLAN.md task: `AchievementFrame_ToggleAchievementFrame`,
//! `AchievementFrame_DisplayComparison`, `AchievementFrame_ForceUpdate`,
//! `AchievementFrame_SetTabs`, `AchievementFrame_SetComparisonTabs`,
//! `AchievementFrame_UpdateTabs` are functions.
//!
//! All six PLAN-named globals match the actual source verbatim — no
//! spec/source mismatches in this batch. They are declared via the
//! `function Name(args)` syntax (not `Name = function(args)`), which under
//! Lua 5.1 semantics binds the function to the global table:
//!
//! - `AchievementFrame_ToggleAchievementFrame` at line 195: the entry-point
//!   for opening the achievement panel — handles the
//!   `Enum.GameRule.AchievementsPanelDisabled` early-return guard, drives
//!   the tab-state swap (`AchievementFrameTab_OnClick =
//!   AchievementFrameBaseTab_OnClick`), and invokes `ShowUIPanel` /
//!   `HideUIPanel` against the `AchievementFrame` global. This is the
//!   surface bound to the `TOGGLEACHIEVEMENT` keybinding.
//!
//! - `AchievementFrame_DisplayComparison` at line 225: drives the
//!   side-by-side comparison view; takes a `unit` argument and sets
//!   `AchievementFrameComparison.unit = unit` before showing the panel.
//!   Note the source has a stylistic space before the open paren (`function
//!   AchievementFrame_DisplayComparison (unit)`); the Lua parser doesn't
//!   care.
//!
//! - `AchievementFrame_ForceUpdate` at line 317: dispatches to one of
//!   `AchievementFrameAchievements_ForceUpdate` /
//!   `AchievementFrameStats_UpdateDataProvider` /
//!   `AchievementFrameComparison_ForceUpdate` depending on which sub-frame
//!   is currently shown. Zero-arg.
//!
//! - `AchievementFrame_SetTabs` at line 327: configures the tab strip for
//!   the player-only mode (shows tab 2, anchors tab 3 to right of tab 2).
//!   Zero-arg.
//!
//! - `AchievementFrame_SetComparisonTabs` at line 332: configures the tab
//!   strip for comparison mode (hides tab 2, re-anchors tab 3 to right of
//!   tab 1). Zero-arg.
//!
//! - `AchievementFrame_UpdateTabs` at line 337: drives the active-tab
//!   visual state — calls `PanelTemplates_Tab_OnClick`, hides
//!   `AchievementFrame.SearchResults`, and per-tab adjusts the Text
//!   position offset (clicked tab gets y=-5, others get y=-3). Takes the
//!   index of the newly-clicked tab.
//!
//! The test pins each name's `_G` entry as a function under the LoadOnDemand
//! root harness (smoke-shape with `[ROOT]`). A non-function reading would
//! prove either (a) the file chunk failed before reaching the function
//! declaration, OR (b) Blizzard re-shaped the global into a method on a
//! mixin / removed it / renamed it. Each is a meaningful API contract
//! change.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AchievementUI";
const PLAN_NAMED_FRAME_GLOBALS: &[&str] = &[
    "AchievementFrame_ToggleAchievementFrame",
    "AchievementFrame_DisplayComparison",
    "AchievementFrame_ForceUpdate",
    "AchievementFrame_SetTabs",
    "AchievementFrame_SetComparisonTabs",
    "AchievementFrame_UpdateTabs",
];
const PLAN_NAMED_CATEGORIES_GLOBALS: &[&str] = &[
    "AchievementFrameCategories_OnLoad",
    "AchievementFrameCategories_ExpandToCategory",
    "AchievementFrameCategories_SelectDefaultElementData",
    "AchievementFrameCategories_UpdateDataProvider",
];
const PLAN_NAMED_ACHIEVEMENTS_GLOBALS: &[&str] = &[
    "AchievementFrameAchievements_GetSelectedAchievementId",
    "AchievementFrameAchievements_OnAchievementEarned",
    "AchievementFrameAchievements_OnCriteriaUpdate",
    "AchievementFrameAchievements_UpdateDataProvider",
];

#[test]
fn achievement_frame_plan_named_globals_are_functions() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        for name in PLAN_NAMED_FRAME_GLOBALS {
            let is_function = env
                .eval::<bool>(&format!(r#"return type({name}) == "function""#))
                .expect("file-scope global type probe must run cleanly");

            assert!(
                is_function,
                "Global `{name}` MUST be a function after the smoke-shape harness loads \
                 `Blizzard_AchievementUI`. Each entry in `PLAN_NAMED_FRAME_GLOBALS` is declared \
                 via `function <name>(...)` syntax in `Mainline/Blizzard_AchievementUI.lua` \
                 (lines 195/225/317/327/332/337). A non-function reading here proves either \
                 (a) the file chunk failed before reaching the declaration (load.rs's \
                 `achievement_ui_load_emits_no_lane_specific_lua_errors` would also fail in that \
                 case — the failure mode is multi-test), OR (b) Blizzard re-shaped the global \
                 into a method on a mixin / removed it / renamed it (this lane-specific test \
                 surfaces the API contract drift even when the load smoke remains green). Got \
                 `type({name}) == \"function\"` returned false."
            );
        }
    });
}

/// Pin the `AchievementFrameCategories_*` ScrollBox-driver globals.
///
/// All four PLAN-named globals match the actual source verbatim — no
/// spec/source mismatches in this batch. Each is declared via the
/// `function <name>(...)` syntax in `Mainline/Blizzard_AchievementUI.lua`:
///
/// - `AchievementFrameCategories_OnLoad` at line 584 — wires the
///   ScrollBox/ScrollBar via `ScrollUtil.InitScrollBoxListWithScrollBar`,
///   sets `AchievementCategoryTemplate` as the element initializer (each
///   element's `Init(elementData)` is called per-button), and registers
///   the `ADDON_LOADED` event. Takes the categories frame as `self` —
///   bound from the XML `<OnLoad>` script.
///
/// - `AchievementFrameCategories_ExpandToCategory` at line 603 — drives
///   the cross-tab navigation when a deep-link target lives on the
///   inactive tab. Takes a category id, calls
///   `AchievementFrameCategories_FindCategoryElement` against the active
///   tab's categories first, then falls back to the alternate tab
///   (guild/personal swap via `InGuildView()`) and dispatches
///   `AchievementFrameBaseTab_OnClick(alternateTabIndex)` to switch tabs.
///   When the target is a child category, hides sibling-of-different-
///   parent children to expand only the target's parent block.
///
/// - `AchievementFrameCategories_SelectDefaultElementData` at line 707 —
///   guards against missing data provider (lazy-builds via
///   `AchievementFrameCategories_UpdateDataProvider` when
///   `ScrollBox:HasDataProvider()` is false), then scrolls to index 1
///   with `AlignCenter` and selects the resulting elementData via
///   `AchievementFrameCategories_SelectElementData`. Zero-arg.
///
/// - `AchievementFrameCategories_UpdateDataProvider` at line 718 — builds
///   a fresh `CreateDataProvider()` from `achievementFunctions.categories`
///   filtered by `category.hidden` and the active
///   `AchievementFrame.restrictedCategoryID` (when set, only categories
///   matching the restricted ID or its parent get inserted). Calls
///   `ScrollBox:SetDataProvider(newDataProvider)` at the end. Note the
///   stylistic space `function AchievementFrameCategories_UpdateDataProvider ()`
///   matches the OnLoad declaration's formatting; the parser doesn't care.
#[test]
fn achievement_frame_categories_plan_named_globals_are_functions() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        for name in PLAN_NAMED_CATEGORIES_GLOBALS {
            let is_function = env
                .eval::<bool>(&format!(r#"return type({name}) == "function""#))
                .expect("file-scope global type probe must run cleanly");

            assert!(
                is_function,
                "Global `{name}` MUST be a function after the smoke-shape harness loads \
                 `Blizzard_AchievementUI`. Each entry in `PLAN_NAMED_CATEGORIES_GLOBALS` is \
                 declared via `function <name>(...)` syntax in \
                 `Mainline/Blizzard_AchievementUI.lua` (lines 584/603/707/718). A non-function \
                 reading here proves either (a) the file chunk failed before reaching the \
                 declaration (load.rs's `achievement_ui_load_emits_no_lane_specific_lua_errors` \
                 would also fail in that case — the failure mode is multi-test), OR (b) the \
                 ScrollBox-driver dispatch was re-shaped into a method on \
                 `AchievementFrameCategories` (a frame mixin) / removed / renamed (this \
                 lane-specific test surfaces the API contract drift even when the load smoke \
                 stays green). The four functions form one cohesive surface: OnLoad wires the \
                 ScrollBox view, UpdateDataProvider rebuilds its data provider, \
                 SelectDefaultElementData scrolls to + selects the first entry, and \
                 ExpandToCategory drives cross-tab navigation — a regression on any one of them \
                 would silently break either the panel's first-open population or the deep-link \
                 navigation flow. Got `type({name}) == \"function\"` returned false."
            );
        }
    });
}

/// Pin the `AchievementFrameAchievements_*` selection-and-data-driver globals.
///
/// All four PLAN-named globals match the actual source verbatim — no
/// spec/source mismatches in this batch. Each is declared via the
/// `function <name>(...)` syntax in `Mainline/Blizzard_AchievementUI.lua`:
///
/// - `AchievementFrameAchievements_GetSelectedAchievementId` at line 890 —
///   trampolines through `AchievementFrameAchievements_GetSelectedElementData`
///   (line 886, which itself returns
///   `g_achievementSelectionBehavior:GetFirstSelectedElementData()`) and
///   returns `elementData and elementData.id or 0` — i.e. zero is the
///   sentinel for "nothing currently selected", not a real achievement id.
///   Zero-arg.
///
/// - `AchievementFrameAchievements_OnAchievementEarned` at line 895 — the
///   ACHIEVEMENT_EARNED event handler. Calls `UpdateDataProvider` to rebuild
///   the list, then conditionally invokes
///   `AchievementFrame_SelectAndScrollToAchievementId(AchievementFrameAchievements.ScrollBox, achievementId)`
///   only when the just-earned id matches the currently-selected one (so the
///   list stays anchored on the player's focus), refreshes the category
///   tooltip via `AchievementFrameCategories_UpdateTooltip()`, and
///   re-renders the points header via
///   `AchievementFrame.Header.Points:SetText(BreakUpLargeNumbers(GetTotalAchievementPoints(InGuildView())))`.
///   Takes the freshly-earned achievement id.
///
/// - `AchievementFrameAchievements_OnCriteriaUpdate` at line 907 — the
///   CRITERIA_UPDATE handler. Looks up the currently-selected element data,
///   resolves the rendered button via `ScrollBox:FindFrame(selectedElementData)`,
///   and re-runs `button:Init(selectedElementData)` to refresh the visible
///   progress bars in place (no data provider rebuild — surgical update of
///   the one frame whose criteria changed). Zero-arg.
///
/// - `AchievementFrameAchievements_UpdateDataProvider` at line 960 — the
///   data-provider rebuild driver. Early-returns when the active category
///   is "summary" (which uses a different rendering path), then resolves
///   `numAchievements`/`numCompleted`/`completedOffset` via
///   `ACHIEVEMENTUI_SELECTEDFILTER(category)`, toggles the
///   `AchievementFrameAchievementsFeatOfStrengthText` visibility/text
///   (choosing `GUILD_FEAT_OF_STRENGTH_DESCRIPTION` vs
///   `FEAT_OF_STRENGTH_DESCRIPTION` based on `AchievementFrame.selectedTab == 2`),
///   builds a fresh `CreateDataProvider()` by iterating
///   `GetAchievementInfo(category, filteredIndex)` and inserting
///   `{category, index, id}` records, and dispatches
///   `ScrollBox:SetDataProvider(newDataProvider)` at the end. Zero-arg.
#[test]
fn achievement_frame_achievements_plan_named_globals_are_functions() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, _loaded| {
        for name in PLAN_NAMED_ACHIEVEMENTS_GLOBALS {
            let is_function = env
                .eval::<bool>(&format!(r#"return type({name}) == "function""#))
                .expect("file-scope global type probe must run cleanly");

            assert!(
                is_function,
                "Global `{name}` MUST be a function after the smoke-shape harness loads \
                 `Blizzard_AchievementUI`. Each entry in `PLAN_NAMED_ACHIEVEMENTS_GLOBALS` is \
                 declared via `function <name>(...)` syntax in \
                 `Mainline/Blizzard_AchievementUI.lua` (lines 890/895/907/960). A non-function \
                 reading here proves either (a) the file chunk failed before reaching the \
                 declaration (load.rs's `achievement_ui_load_emits_no_lane_specific_lua_errors` \
                 would also fail in that case — the failure mode is multi-test), OR (b) the \
                 selection/data-driver dispatch was re-shaped into a method on \
                 `AchievementFrameAchievements` (a frame mixin) / removed / renamed (this \
                 lane-specific test surfaces the API contract drift even when the load smoke \
                 stays green). The four functions form one cohesive surface: \
                 GetSelectedAchievementId reads the current selection's id (zero-default), \
                 OnAchievementEarned is the ACHIEVEMENT_EARNED event entry-point that rebuilds \
                 the data provider + conditionally scrolls + refreshes the points header, \
                 OnCriteriaUpdate surgically refreshes the selected button's progress bars on \
                 CRITERIA_UPDATE, and UpdateDataProvider rebuilds the ScrollBox's data \
                 provider for the current category — a regression on any one of them would \
                 silently break either the live progress display or the post-earn list refresh. \
                 Got `type({name}) == \"function\"` returned false."
            );
        }
    });
}
