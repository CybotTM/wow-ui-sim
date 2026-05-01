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
use crate::common::panel_fixtures::blizzard_ui_dir;
use wow_ui_sim::toc::TocFile;

const ROOT: &str = "Blizzard_ActionBar";
const ROOT_TOC_FILE: &str = "Blizzard_ActionBar_Mainline.toc";
const PLAN_DECLARED_DEPS: &[&str] = &[
    "Blizzard_StoreUI",
    "Blizzard_QuickKeybind",
    "Blizzard_UIParent",
    "Blizzard_EditMode",
    "Blizzard_UIPanels_Game",
    "Blizzard_TextStatusBar",
    "Blizzard_Flyout",
    "Blizzard_Colors",
    "Blizzard_HelpPlate",
    "Blizzard_MicroMenu",
];
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

/// Pin the TOC-declared dependency contract via TWO independent grounds:
///
/// 1. **Parser-grounded.** Parse `Blizzard_ActionBar_Mainline.toc` via
///    `TocFile::from_file` (the same parser the closure walker uses at
///    `src/loader/mod.rs:454` via `toc.dependencies().chain(toc.optional_deps())`)
///    and assert the parser-extracted dep set matches the PLAN-stated list
///    BIT-FOR-BIT. A regression where Blizzard adds, removes, or reorders a
///    `## Dependencies:` entry would trip this — surfacing the contract drift
///    before the runtime-load round-trip matters.
/// 2. **Runtime-loaded round-trip.** After the startup-shape harness runs,
///    assert `C_AddOns.IsAddOnLoaded(dep) == true` for every parser-extracted
///    dep. The closure-walked `loaded` set does NOT contain these deps —
///    they're pre-loaded by the panel-addons baseline at
///    `tests/common/panel_fixtures.rs:104-115` BEFORE the closure walker
///    runs, and the walker skips already-loaded entries when adding to the
///    new closure's `loaded` list. So `IsAddOnLoaded` (which reads the
///    addon registry's loaded-flag, set by `mark_addon_loaded` at
///    `src/c_api/c_addons.rs:661`) is the correct ground for the
///    "dep is satisfied at runtime" contract.
///
/// PLAN's wording "every TOC dependency present in loaded set after harness
/// runs" is interpreted as **runtime-loaded** rather than **closure-walked**:
/// the closure-walked `loaded` is a build-time concept (what the walker
/// just pulled), while the registry's loaded-flag is the runtime-state
/// concept (what is currently usable from Lua). For an addon whose deps
/// are pre-loaded by the baseline, only the runtime-state reading carries
/// information.
#[test]
fn action_bar_every_declared_toc_dependency_is_loaded_after_harness_runs() {
    let toc_path = blizzard_ui_dir().join(ROOT).join(ROOT_TOC_FILE);
    let toc = TocFile::from_file(&toc_path).unwrap_or_else(|e| {
        panic!(
            "TOC at `{}` MUST parse cleanly. The closure walker reads this same file via \
             `TocFile::from_file` (src/c_api/c_addons.rs:639) when servicing LoadAddOn — a \
             parser failure here would prove the simulator's runtime LoadAddOn dispatch \
             cannot resolve this addon either. Got: {e}",
            toc_path.display()
        )
    });
    let mut declared_deps: Vec<String> = toc.dependencies();
    declared_deps.extend(toc.optional_deps());

    let plan_deps_owned: Vec<String> = PLAN_DECLARED_DEPS.iter().map(|s| s.to_string()).collect();
    assert_eq!(
        declared_deps, plan_deps_owned,
        "Parser-extracted dep list from `{ROOT_TOC_FILE}` MUST match the PLAN-stated dep list \
         BIT-FOR-BIT (same entries in the same order). The TOC currently declares \
         `## Dependencies: Blizzard_StoreUI, Blizzard_QuickKeybind, Blizzard_UIParent, \
         Blizzard_EditMode, Blizzard_UIPanels_Game, Blizzard_TextStatusBar, Blizzard_Flyout, \
         Blizzard_Colors, Blizzard_HelpPlate, Blizzard_MicroMenu` (line 4) and NO \
         `## OptionalDeps:` line. A regression that adds, removes, or reorders an entry would \
         trip here BEFORE the runtime-loaded round-trip below; if the PLAN line at PLAN.md:1116 \
         is updated to match Blizzard's new TOC, this `PLAN_DECLARED_DEPS` const must be \
         updated alongside it."
    );

    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        for dep in &declared_deps {
            let is_loaded = env
                .eval::<bool>(&format!(
                    r#"return C_AddOns.IsAddOnLoaded("{dep}") == true"#
                ))
                .expect("C_AddOns.IsAddOnLoaded probe must run cleanly");

            assert!(
                is_loaded,
                "Declared TOC dependency `{dep}` (parsed from `{ROOT_TOC_FILE}` via \
                 `TocFile::from_file`) MUST be runtime-loaded after the startup-shape harness \
                 finishes — `C_AddOns.IsAddOnLoaded(\"{dep}\")` returned false. Each declared \
                 dep is pre-loaded by the panel-addons baseline (cross-references in this \
                 file's module doc) BEFORE the closure walker runs; the walker then sees the \
                 dep as already satisfied and skips re-loading. A false reading here means \
                 either (a) the panel baseline regressed (entry missing or load failure \
                 silently swallowed at `panel_fixtures.rs:111-113`), OR (b) `mark_addon_loaded` \
                 (src/c_api/c_addons.rs:661) didn't run for this dep, OR (c) the TOC's \
                 declared name doesn't match the addon folder name discovered by \
                 `discover_blizzard_addons_for_screen`. Downstream addons inheriting templates \
                 from `{dep}` would fail to resolve in production."
            );
        }
    });
}
