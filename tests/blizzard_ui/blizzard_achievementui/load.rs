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

const ROOT: &str = "Blizzard_AchievementUI";
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
