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
