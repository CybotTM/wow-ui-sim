//! Load smoke for `Blizzard_AchievementUI`.
//!
//! TOC reference (`Interface/BlizzardUI/Blizzard_AchievementUI/
//! Blizzard_AchievementUI_Mainline.toc`):
//!
//! ```text
//! ## Title: Blizzard_AchievementUI
//! ## Secure: 1
//! ## Author: Blizzard Entertainment
//! ## Version: 1.0
//! ## LoadOnDemand: 1
//! ## AllowLoadGameType: mainline
//! Mainline\Blizzard_AchievementUI.lua
//! Mainline\Blizzard_AchievementUI.xml
//! Mainline\Localization.lua
//! ```
//!
//! Why this lane uses the game-shape `with_blizzard_addon_smoke_shape`
//! harness rather than the glue counterpart: the TOC carries no
//! `## AllowLoad:` directive, which defaults to game-screen visibility
//! (the closure walker's `allows_screen(Game)` returns true). The lane
//! also relies on the panel-addons baseline that the smoke-shape harness
//! pre-loads via `load_panel_addons` (`tests/common/panel_fixtures.rs:53-56`):
//! `Blizzard_AchievementUI.lua:2` writes
//! `UIPanelWindows["AchievementFrame"] = { area = "doublewide", ... }` at
//! file scope, which requires `UIPanelWindows` to already exist as a
//! table — provided by `Blizzard_UIParentPanelManager` in the panel
//! baseline. Without that baseline this assignment would either build a
//! stray global table or trip the panel manager's missing-frame
//! invariants when the achievement frame is later opened.
//!
//! Why the closure-walked `loaded` set is expected to contain ONLY the
//! root: the TOC declares neither `## Dependencies:` nor any flavour of
//! `## OptionalDep[s]:`. The closure walker therefore pulls nothing
//! transitively. Every parent template the addon's XML inherits from
//! (`TooltipBackdropTemplate`, `GameFontNormal*`, `_SearchBarLg`,
//! `TooltipBorderBackdropTemplate`, etc., per
//! `Mainline/Blizzard_AchievementUI.xml`) lives in `Blizzard_SharedXML` /
//! `Blizzard_FrameXML`, which the panel-addons baseline preloads. A
//! future change that DOES add a `## Dependencies:` line would land in
//! `dependency_closure_includes_only_the_root` as a new entry in
//! `loaded`, surfacing the new dep without silently changing the
//! contract.
//!
//! Why `## AllowLoadGameType: mainline` matters here: the simulator
//! discovers the TOC with the suffix matching the active game type
//! (`Blizzard_AchievementUI_Mainline.toc`); a Mists-suffixed sibling
//! exists at `Blizzard_AchievementUI_Mists.toc`. The closure walker
//! resolves the right TOC via the screen+gameType filters; this load
//! smoke pins the mainline lane only.
//!
//! Assertion pinned: loading the smoke-shape closure rooted at
//! `Blizzard_AchievementUI` completes cleanly with zero lane-specific
//! Lua errors recorded. The lane's single Lua chunk
//! (`Mainline/Blizzard_AchievementUI.lua`) plus its XML sibling registers
//! ten `Achievement*Mixin` globals, the `AchievementFrameFilters` table,
//! and the `ACHIEVEMENT_FUNCTIONS` / `GUILD_ACHIEVEMENT_FUNCTIONS` /
//! `STAT_FUNCTIONS` / `COMPARISON_*_FUNCTIONS` dispatch tables at file
//! scope; any nil-call, missing global, or template-resolution failure
//! would be recorded into `state.lua_errors` and surface in the filtered
//! list below.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use crate::common::panel_fixtures::blizzard_ui_dir;
use wow_ui_sim::toc::TocFile;

const ROOT: &str = "Blizzard_AchievementUI";
const ROOT_TOC_FILE: &str = "Blizzard_AchievementUI_Mainline.toc";
const LANE_FILE_SCOPE_MIXINS: &[&str] = &[
    "AchievementCategoryTemplateMixin",
    "AchievementCategoryTemplateButtonMixin",
    "AchievementTemplateMixin",
    "AchivementButtonCheckMixin",
    "AchievementsObjectivesMixin",
    "AchievementStatTemplateMixin",
    "AchievementMetaCriteriaMixin",
    "AchievementComparisonTemplateMixin",
    "AchivementComparisonStatMixin",
    "AchievementFullSearchResultsButtonMixin",
];
const LANE_FILE_SCOPE_DISPATCH_TABLES: &[&str] = &[
    "ACHIEVEMENT_FUNCTIONS",
    "GUILD_ACHIEVEMENT_FUNCTIONS",
    "STAT_FUNCTIONS",
    "COMPARISON_ACHIEVEMENT_FUNCTIONS",
    "COMPARISON_STAT_FUNCTIONS",
    "AchievementFrameFilterStrings",
    "AchievementFrameFilters",
];
const LANE_FILE_SCOPE_PANEL_REGISTRATION_KEY: &str = "AchievementFrame";

#[test]
fn achievement_ui_load_emits_no_lane_specific_lua_errors() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
        assert!(
            loaded.iter().any(|name| name == ROOT),
            "Smoke-shape harness MUST end up loading `{ROOT}` itself when it is the closure root \
             — even though the TOC carries `## LoadOnDemand: 1`, the closure walker chains the \
             LoD pool into the main pool when an LoD addon is requested as a root \
             (src/loader/mod.rs:410). A regression that routed LoD roots away from the closure \
             walker would land here. Loaded set: {loaded:?}"
        );

        let lane_lua_errors: Vec<String> = env
            .state()
            .borrow()
            .lua_errors
            .iter()
            .filter(|message| message.contains("Achievement") || message.contains("Achivement"))
            .cloned()
            .collect();

        assert!(
            lane_lua_errors.is_empty(),
            "Blizzard_AchievementUI emitted lane-specific Lua errors during the smoke-shape \
             closure load. The addon defines AchievementCategoryTemplateMixin / \
             AchievementCategoryTemplateButtonMixin / AchievementTemplateMixin / \
             AchivementButtonCheckMixin / AchievementsObjectivesMixin / \
             AchievementStatTemplateMixin / AchievementMetaCriteriaMixin / \
             AchievementComparisonTemplateMixin / AchivementComparisonStatMixin / \
             AchievementFullSearchResultsButtonMixin at file scope across one Lua file plus its \
             XML sibling — any nil-call, missing global, or template-resolution failure would \
             surface here. The filter matches any error message containing the substring \
             `Achievement` or `Achivement` (the source has typo'd mixin names \
             `AchivementButtonCheckMixin` / `AchivementComparisonStatMixin` at \
             Blizzard_AchievementUI.lua:1629,2908 — preserved verbatim because Blizzard's API \
             contract uses the misspelling). The disjunction covers both file paths \
             (`Interface/BlizzardUI/Blizzard_AchievementUI/...`) and global identifiers \
             (`Achievement*Mixin`, `Achivement*Mixin`). Got:\n  {}",
            lane_lua_errors.join("\n  ")
        );
    });
}

#[test]
fn achievement_ui_dependency_closure_includes_only_the_root() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |_env, loaded| {
        assert_eq!(
            loaded,
            &[ROOT.to_string()],
            "The closure-walked `loaded` set MUST contain ONLY `{ROOT}` because the TOC \
             (`Blizzard_AchievementUI_Mainline.toc`) declares neither `## Dependencies:` nor any \
             flavour of `## OptionalDep[s]:`. Every XML parent template the addon inherits from \
             (`TooltipBackdropTemplate`, `GameFontNormal*`, `_SearchBarLg`, \
             `TooltipBorderBackdropTemplate`) lives in `Blizzard_SharedXML` / `Blizzard_FrameXML`, \
             which the panel-addons baseline (`tests/common/panel_fixtures.rs:53-56`) preloads \
             OUTSIDE the closure walker — so they don't appear here. A regression that adds a \
             `## Dependencies:` line, OR that pollutes the closure with panel-addon entries, \
             would change this set and surface the contract drift. Got: {loaded:?}"
        );
    });
}

#[test]
fn achievement_ui_load_on_demand_root_executes_file_scope_code() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
        assert!(
            loaded.iter().any(|name| name == ROOT),
            "`{ROOT}` MUST appear in the closure-walked `loaded` set despite carrying \
             `## LoadOnDemand: 1`. The closure walker chains the LoD pool into the main pool \
             when an LoD addon is requested as a root (src/loader/mod.rs:410). A regression \
             that excluded LoD addons from being closure roots would prevent any of this lane's \
             file-scope code from executing — the global-existence assertions below would all \
             fail, but this top-level check pins the root cause. Loaded set: {loaded:?}"
        );

        for mixin_name in LANE_FILE_SCOPE_MIXINS {
            let is_table = env
                .eval::<bool>(&format!(r#"return type({mixin_name}) == "table""#))
                .expect("file-scope mixin type probe must run cleanly");

            assert!(
                is_table,
                "Mixin `{mixin_name}` MUST be a table after the smoke-shape harness loads \
                 `Blizzard_AchievementUI`. Each entry in `LANE_FILE_SCOPE_MIXINS` is declared \
                 via `Mixin = {{}}` at file scope across the lane's single Lua file \
                 (Blizzard_AchievementUI.lua:496,571,1039,1629,1674,2125,2580,2682,2908,3392). \
                 If the LoadOnDemand flag silently skipped the addon's load, the closure walker \
                 would still list it in `loaded` (since LoD routing happens upstream) but the \
                 file chunks would never run — leaving these globals as nil. A nil reading here \
                 means the LoadOnDemand handling in `load_addon` regressed: the addon was \
                 discovered but its file chunks didn't execute. Got \
                 `type({mixin_name}) == \"table\"` returned false."
            );
        }

        for table_name in LANE_FILE_SCOPE_DISPATCH_TABLES {
            let is_table = env
                .eval::<bool>(&format!(r#"return type({table_name}) == "table""#))
                .expect("file-scope dispatch table type probe must run cleanly");

            assert!(
                is_table,
                "Dispatch table `{table_name}` MUST be a table after the smoke-shape harness \
                 loads `Blizzard_AchievementUI`. These tables (declared at \
                 Blizzard_AchievementUI.lua:56,144,149,154,159,164,1846) carry the per-mode \
                 dispatch handlers used by the achievement / guild-achievement / stat / \
                 comparison panels. A nil reading here means the file chunk failed before \
                 reaching the dispatch-table block. Got `type({table_name}) == \"table\"` \
                 returned false."
            );
        }

        let panel_registration_present = env
            .eval::<bool>(&format!(
                r#"return type(UIPanelWindows) == "table"
                    and type(UIPanelWindows["{LANE_FILE_SCOPE_PANEL_REGISTRATION_KEY}"]) == "table""#
            ))
            .expect("UIPanelWindows registration probe must run cleanly");

        assert!(
            panel_registration_present,
            "After loading `Blizzard_AchievementUI`, \
             `UIPanelWindows[\"{LANE_FILE_SCOPE_PANEL_REGISTRATION_KEY}\"]` MUST be a table — \
             populated at Blizzard_AchievementUI.lua:2 via \
             `UIPanelWindows[\"AchievementFrame\"] = {{ area = \"doublewide\", pushable = 0, \
             xoffset = 80, whileDead = 1 }}`. This is the addon's registration with the panel \
             manager (driven by `Blizzard_UIParentPanelManager`, preloaded by the \
             panel-addons baseline). A nil reading would prove either (a) the file chunk \
             aborted at line 2 because `UIPanelWindows` was nil at that moment (panel-addons \
             baseline regressed or load order changed), OR (b) the assignment was moved \
             behind a deferred path (e.g. `OnLoad`) — both meaningful contract changes."
        );
    });
}

/// Pin the TOC-driven dependency contract via direct parser dispatch.
///
/// Earlier `achievement_ui_dependency_closure_includes_only_the_root` asserts
/// the literal `loaded == [ROOT]` set, which is correct but reads the contract
/// as a constant. THIS test grounds the contract in the TOC file via
/// `TocFile::from_file`: parses `Blizzard_AchievementUI_Mainline.toc`, extracts
/// `dependencies()` and `optional_deps()`, and asserts every entry appears in
/// `loaded`. For this addon both lists are currently empty, so the forward
/// inclusion is vacuous — but the auxiliary "no extras beyond ROOT" check on
/// the reverse side AND the "TOC has zero deps" parser-grounded assertion
/// together hold the contract under change: a future PR adding a `##
/// Dependencies:` line either (a) gets pulled by the closure walker and
/// satisfies the forward inclusion, OR (b) gets dropped and trips the
/// inclusion. Either way the empty-list assertion forces the PR to also
/// update this test, surfacing the contract drift.
#[test]
fn achievement_ui_loaded_set_contains_every_declared_toc_dependency() {
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

    assert!(
        declared_deps.is_empty(),
        "`{ROOT_TOC_FILE}` currently declares NO `## Dependencies:` and NO `## OptionalDeps:` \
         entries — the parser-extracted set is `{declared_deps:?}`. If a future PR adds either \
         line, this assertion will trip; that PR must (a) update this expected list AND (b) \
         either confirm the closure walker pulls the new dep (the forward inclusion below \
         passes) or fix the closure walker. The plural `## OptionalDeps:` form is the only \
         one the parser recognises (src/toc.rs:229-234) — singular `## OptionalDep:` is silently \
         ignored, so a future singular-form addition would NOT change `declared_deps` and would \
         NOT trip this guard but WOULD silently fail to pull the dep."
    );

    with_blizzard_addon_smoke_shape(&[ROOT], &[], |_env, loaded| {
        for dep in &declared_deps {
            assert!(
                loaded.iter().any(|name| name == dep),
                "Declared TOC dependency `{dep}` (parsed from \
                 `{ROOT_TOC_FILE}` via `TocFile::from_file`) MUST appear in the closure-walked \
                 `loaded` set. The walker calls `toc.dependencies().chain(toc.optional_deps())` \
                 (src/loader/mod.rs:454) to pull deps transitively. A missing entry here means \
                 the walker dropped the dep — downstream addons inheriting templates from this \
                 dep would fail to resolve. Loaded set: {loaded:?}"
            );
        }

        for entry in loaded {
            if entry == ROOT {
                continue;
            }
            assert!(
                declared_deps.iter().any(|dep| dep == entry),
                "Closure-walked `loaded` entry `{entry}` is NOT declared as a TOC dependency in \
                 `{ROOT_TOC_FILE}` — the parser extracted `{declared_deps:?}` from the file's \
                 `## Dependencies:` and `## OptionalDeps:` lines. An extra entry here means \
                 either (a) the closure walker pulled an addon that wasn't requested (e.g. via \
                 a panel-baseline leak into the closure pool), OR (b) the TOC declares a dep \
                 in a form the parser doesn't recognise (e.g. singular `## OptionalDep:`). \
                 Loaded set: {loaded:?}"
            );
        }
    });
}

/// Pin the LoD-trigger contract: `C_AddOns.LoadAddOn` from Lua resolves and
/// loads the addon when called against a fresh env that has NOT pre-loaded it.
///
/// Earlier `achievement_ui_load_on_demand_root_executes_file_scope_code` runs
/// the smoke-shape harness with `&[ROOT]` as roots, which loads the addon via
/// the closure walker — that test pins "LoD as a closure root works", not "LoD
/// as a Lua-driven runtime dispatch works". THIS test exercises the second
/// path: passing `&[]` to the harness loads only the panel-addons baseline
/// (`tests/common/panel_fixtures.rs:53-56`), leaving the AchievementUI TOC
/// discoverable via `addon_base_paths` but unloaded. The Lua call
/// `C_AddOns.LoadAddOn("Blizzard_AchievementUI")` then exercises
/// `c_addons_load_addon` (src/c_api/c_addons.rs:570), which finds the TOC via
/// `find_runtime_addon_toc`, parses it, and runs the file chunks.
#[test]
fn achievement_ui_load_on_demand_triggers_via_lua_load_addon_api() {
    with_blizzard_addon_smoke_shape(&[], &[], |env, loaded| {
        assert!(
            !loaded.iter().any(|name| name == ROOT),
            "Pre-condition violated: `{ROOT}` MUST NOT be in the closure-walked `loaded` set \
             when the harness is invoked with empty roots. The smoke-shape harness only loads \
             the closure of `roots` plus the panel-addons baseline — and the panel baseline does \
             not include `{ROOT}`. A non-empty reading here means a baseline pre-load regressed \
             into pulling AchievementUI, which would invalidate this test's LoD-trigger \
             assertion (the pre-loaded addon would short-circuit the LoadAddOn call). Loaded \
             set: {loaded:?}"
        );

        let pre_loaded = env
            .eval::<bool>(&format!(
                r#"return C_AddOns.IsAddOnLoaded("{ROOT}") == true"#
            ))
            .expect("IsAddOnLoaded probe must run cleanly");

        assert!(
            !pre_loaded,
            "Pre-condition violated: `C_AddOns.IsAddOnLoaded(\"{ROOT}\")` returned true BEFORE \
             the LoadAddOn dispatch ran. This means a panel-baseline workaround or runtime \
             preload (e.g. `apply_for_runtime_addon_preload` at src/c_api/c_addons.rs:584) \
             auto-loaded AchievementUI as a side effect — which would short-circuit the \
             LoadAddOn call below and invalidate this test's contract. If this assertion ever \
             trips, audit the panel baseline and any workaround paths for AchievementUI \
             auto-load."
        );

        let load_addon_returned_true = env
            .eval::<bool>(&format!(
                r#"local loaded, _reason = C_AddOns.LoadAddOn("{ROOT}")
                return loaded == true"#
            ))
            .expect("C_AddOns.LoadAddOn dispatch must run cleanly");

        assert!(
            load_addon_returned_true,
            "`C_AddOns.LoadAddOn(\"{ROOT}\")` MUST return `true` when invoked from Lua against \
             a discoverable LoD addon. The dispatch lives at \
             `c_addons_load_addon` (src/c_api/c_addons.rs:570): it locates the TOC via \
             `find_runtime_addon_toc`, parses it via `TocFile::from_file`, and walks deps + \
             foundations recursively before executing the file chunks. A `false` return means \
             one of: (a) `find_runtime_addon_toc` failed to locate the addon (addon_base_paths \
             regression), (b) the TOC parser tripped, (c) a dependency closure walked through \
             a disabled addon, OR (d) `load_addon_from_toc` itself errored. Tear-down line: \
             this is the FIRST gate keeping LoD-only addons usable from runtime Lua code."
        );

        let post_loaded = env
            .eval::<bool>(&format!(
                r#"return C_AddOns.IsAddOnLoaded("{ROOT}") == true"#
            ))
            .expect("post-load IsAddOnLoaded probe must run cleanly");

        assert!(
            post_loaded,
            "After `C_AddOns.LoadAddOn(\"{ROOT}\")` returned true, \
             `C_AddOns.IsAddOnLoaded(\"{ROOT}\")` MUST also return true — the LoadAddOn path \
             ends with `mark_addon_loaded` (src/c_api/c_addons.rs:661), which sets the addon's \
             `loaded` flag. A false reading here means the load completed but the loaded-state \
             bookkeeping wasn't updated — downstream `IsAddOnLoaded` callers would see the addon \
             as not-loaded and re-dispatch LoadAddOn, potentially infinitely."
        );

        let mixin_present = env
            .eval::<bool>(r#"return type(AchievementTemplateMixin) == "table""#)
            .expect("file-scope mixin probe after LoadAddOn must run cleanly");

        assert!(
            mixin_present,
            "After LoadAddOn returned true, the file-scope global \
             `AchievementTemplateMixin` (declared at Blizzard_AchievementUI.lua:1039) MUST be a \
             table. A nil reading here means the addon's loaded-state bookkeeping was updated \
             but its file chunks didn't actually execute — proving LoadAddOn took a fast path \
             past `load_addon_from_toc`'s file-load step. This is a stronger pin than \
             IsAddOnLoaded alone because it surfaces the case where the addon registry says \
             loaded but no Lua code actually ran."
        );
    });
}
