//! Load smoke for `Blizzard_ActionBar`.
//!
//! TOC reference (`Interface/BlizzardUI/Blizzard_ActionBar/
//! Blizzard_ActionBar_Mainline.toc`):
//!
//! ```text
//! ## Title: Blizzard_ActionBar
//! ## Author: Blizzard Entertainment
//! ## DefaultState: enabled
//! ## Dependencies: Blizzard_StoreUI, Blizzard_QuickKeybind, Blizzard_UIParent,
//!                  Blizzard_EditMode, Blizzard_UIPanels_Game, Blizzard_TextStatusBar,
//!                  Blizzard_Flyout, Blizzard_Colors, Blizzard_HelpPlate, Blizzard_MicroMenu
//! ## AllowLoad: Game
//! ```
//!
//! Why this lane uses the `with_blizzard_addon_startup_shape` harness rather
//! than the smoke-shape counterpart used by AchievementUI: action bars
//! register many events at OnLoad (`PLAYER_ENTERING_WORLD`,
//! `UPDATE_BINDINGS`, `ACTIONBAR_SLOT_CHANGED`, etc., across
//! `ActionBarMixin:OnLoad` at `Shared/ActionBar.lua`,
//! `ActionBarActionButtonMixin:OnLoad` at `Shared/ActionButton.lua:442`,
//! `MainActionBarMixin:OnLoad` at `Shared/MainActionBar.lua:3`) and rely on
//! `PLAYER_ENTERING_WORLD` to populate their first visual state — startup
//! settling is required before any Lua-error pinning is meaningful. The
//! startup-shape harness invokes `settle_headless_startup` after the closure
//! load, which fires the headless startup-event sequence and lets the OnLoad
//! handlers run to completion.
//!
//! Why all 10 declared TOC deps are satisfied without appearing in the
//! closure-walked `loaded` set: every entry in the `## Dependencies:` line
//! is ALSO in the panel-addons baseline preloaded by
//! `tests/common/panel_fixtures.rs:22-80` BEFORE the closure walker runs.
//! Cross-reference (panel_fixtures.rs line ↔ ActionBar dep):
//! - `Blizzard_StoreUI`        → panel_fixtures.rs:48
//! - `Blizzard_QuickKeybind`   → panel_fixtures.rs:75
//! - `Blizzard_UIParent`       → panel_fixtures.rs:43
//! - `Blizzard_EditMode`       → panel_fixtures.rs:50
//! - `Blizzard_UIPanels_Game`  → panel_fixtures.rs:77-79
//! - `Blizzard_TextStatusBar`  → panel_fixtures.rs:44
//! - `Blizzard_Flyout`         → panel_fixtures.rs:47
//! - `Blizzard_Colors`         → panel_fixtures.rs:23
//! - `Blizzard_HelpPlate`      → panel_fixtures.rs:37
//! - `Blizzard_MicroMenu`      → panel_fixtures.rs:49
//!
//! When the closure walker reaches a dep that is already loaded, it does
//! NOT add it to the new closure's `loaded` list. So this lane's `loaded`
//! set contains only `[ROOT]`, while every dep is satisfied by the
//! panel-baseline preload. The `every_declared_dep_is_loaded_by_is_addon_loaded`
//! companion test at PLAN line 1117 (separate task) will pin the
//! `IsAddOnLoaded` round-trip for each dep; this load smoke pins the
//! root-loaded + zero-error contract.
//!
//! Assertion pinned: loading the startup-shape closure rooted at
//! `Blizzard_ActionBar` completes cleanly with zero lane-specific Lua errors
//! recorded. The lane spans 22 Lua files plus 14 XML siblings; any
//! template-resolution failure (XML inheritance from `ActionButtonTemplate`
//! / `MainActionBarTemplate` / `MultiActionBarTemplate`), nil-call (e.g.
//! `Mixin(...)`) at file scope, or missing global from a panel-baseline
//! gap would surface in `state.lua_errors` and fall through the
//! lane-specific filter below.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_ActionBar";
const LANE_FILE_SCOPE_MIXINS: &[&str] = &[
    "ActionBarMixin",
    "EditModeActionBarMixin",
    "MainActionBarMixin",
    "ActionBarActionButtonMixin",
    "BaseActionButtonMixin",
    "ActionBarButtonMixin",
    "SmallActionButtonMixin",
];
const LANE_FILE_SCOPE_TABLES: &[&str] = &["ActionButtonUtil", "AssistedCombatManager"];

#[test]
fn action_bar_load_emits_no_lane_specific_lua_errors() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, loaded| {
        assert!(
            loaded.iter().any(|name| name == ROOT),
            "Startup-shape harness MUST end up loading `{ROOT}` itself when it is the closure \
             root. The TOC carries `## AllowLoad: Game`, which the closure walker accepts via \
             `allows_screen(Game)`. A regression that filtered the root by AllowLoad would land \
             here. Loaded set: {loaded:?}"
        );

        let lane_lua_errors: Vec<String> = env
            .state()
            .borrow()
            .lua_errors
            .iter()
            .filter(|message| {
                message.contains("ActionBar")
                    || message.contains("ActionButton")
                    || message.contains("MainActionBar")
                    || message.contains("StanceBar")
                    || message.contains("ExtraActionBar")
                    || message.contains("PossessActionBar")
                    || message.contains("PetActionBar")
                    || message.contains("VehicleLeaveButton")
                    || message.contains("StatusTrackingBar")
                    || message.contains("StatusTrackingManager")
                    || message.contains("ExpBar")
                    || message.contains("ReputationBar")
                    || message.contains("AzeriteBar")
                    || message.contains("ArtifactBar")
                    || message.contains("HonorBar")
                    || message.contains("HouseFavorBar")
                    || message.contains("AssistedCombatManager")
                    || message.contains("SpellFlyout")
            })
            .cloned()
            .collect();

        assert!(
            lane_lua_errors.is_empty(),
            "Blizzard_ActionBar emitted lane-specific Lua errors during the startup-shape closure \
             load. The lane spans 22 Lua files (`ActionButtonUtil`, `ActionButtonSpellAlerts`, \
             `AssistedCombatManager`, `StatusTrackingBar`, `ExpBar`, `ReputationBar`, \
             `AzeriteBar`, `ArtifactBar`, `HonorBar`, `HouseFavorBar`, `ActionButton`, \
             `ActionBar`, `MultiActionBars`, `MainActionBar`, `VehicleLeaveButton`, \
             `StatusTrackingManager`, `StanceBar`, `ExtraActionBar`, `PossessActionBar`, \
             `PetActionBar`, `SpellFlyout`, `Localization`) plus 14 XML siblings; any \
             template-resolution failure, nil-call at file scope, or missing global from a \
             panel-baseline gap would surface here. The filter substring-matches the file/global \
             names of every Lua chunk in the lane. Got:\n  {}",
            lane_lua_errors.join("\n  ")
        );
    });
}

#[test]
fn action_bar_load_executes_file_scope_mixin_declarations() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        for mixin_name in LANE_FILE_SCOPE_MIXINS {
            let is_table = env
                .eval::<bool>(&format!(r#"return type({mixin_name}) == "table""#))
                .expect("file-scope mixin type probe must run cleanly");

            assert!(
                is_table,
                "Mixin `{mixin_name}` MUST be a table after the startup-shape harness loads \
                 `Blizzard_ActionBar`. Each entry in `LANE_FILE_SCOPE_MIXINS` is declared via \
                 `Mixin = {{}}` at file scope across the lane's Lua files \
                 (ActionBar.lua:1/254, MainActionBar.lua:3, ActionButton.lua:442/1500/1603/1625). \
                 If the closure walker silently skipped this addon's load — e.g. because a panel \
                 baseline pre-load shadowed the dep without the closure walker noticing — the \
                 file chunks would never run, leaving these globals as nil. A nil reading here \
                 means the load step regressed: the addon was discovered but its file chunks \
                 didn't execute. Got `type({mixin_name}) == \"table\"` returned false."
            );
        }

        for table_name in LANE_FILE_SCOPE_TABLES {
            let is_table = env
                .eval::<bool>(&format!(r#"return type({table_name}) == "table""#))
                .expect("file-scope table type probe must run cleanly");

            assert!(
                is_table,
                "Table `{table_name}` MUST be a table after the startup-shape harness loads \
                 `Blizzard_ActionBar`. `ActionButtonUtil` is declared at \
                 ActionButtonUtil.lua:10 and `AssistedCombatManager` at \
                 AssistedCombatManager.lua:3 — both as `Name = {{}}` at file scope. A nil \
                 reading here means the file chunk failed before reaching the declaration. Got \
                 `type({table_name}) == \"table\"` returned false."
            );
        }
    });
}
