//! Load smoke for `Blizzard_AccountStore`.
//!
//! TOC reference (`Interface/BlizzardUI/Blizzard_AccountStore/
//! Blizzard_AccountStore.toc`):
//!
//! ```text
//! ## Title: Blizzard Account Store
//! ## AllowLoad: Both
//! ## Dependencies: Blizzard_SharedXML, Blizzard_StaticPopup
//! ## OptionalDep: Blizzard_UIParentPanelManager
//! ## LoadOnDemand: 1
//! ```
//!
//! Why this lane uses the game-shape `with_blizzard_addon_smoke_shape`
//! harness rather than the glue counterpart: `## AllowLoad: Both` keeps
//! the addon visible to both screen pools, and `Blizzard_UIParentPanelManager`
//! (the panel-management dep that AccountStore registers itself with via
//! `RegisterUIPanel`) only ships in the game lane. The smoke-shape harness's
//! `load_panel_addons` baseline pre-loads `Blizzard_UIParentPanelManager`
//! before the closure walker runs (`tests/common/panel_fixtures.rs:53-56`),
//! so the addon's `RegisterUIPanel` / `UIPanelWindows` calls land against a
//! fully initialized panel manager — without that baseline the lane would
//! emit nil-call errors at file scope.
//!
//! Why the closure-walked `loaded` set won't contain `Blizzard_UIParentPanelManager`:
//! the simulator's TOC parser only recognises `## OptionalDeps:` (plural,
//! `src/toc.rs:229-234`), but this addon's TOC uses `## OptionalDep:`
//! (singular). The closure walker therefore won't pull
//! `Blizzard_UIParentPanelManager` transitively. The panel-addons baseline
//! covers the gap at runtime, so this load smoke still passes — but a
//! future regression to the singular-form parsing surface would fail the
//! companion `every TOC dependency present in loaded set` task, not this
//! one. Keeping the two assertions in separate tests means a TOC-parser
//! regression doesn't drown out unrelated Lua-error regressions.
//!
//! Assertion pinned: loading the smoke-shape closure rooted at
//! `Blizzard_AccountStore` completes cleanly with zero lane-specific Lua
//! errors recorded. The lane's six file-scope Lua chunks
//! (`Blizzard_AccountStoreUtil.lua`, `Blizzard_AccountStoreCategoryList.lua`,
//! `Blizzard_AccountStoreCardTemplates.lua`, `Blizzard_AccountStoreItemRack.lua`,
//! `Blizzard_AccountStoreItemDisplay.lua`, `Blizzard_AccountStore.lua`) plus
//! their XML siblings register every `AccountStore*Mixin` /
//! `Fullscreen*AccountStore*Mixin` global at file scope; any nil-call,
//! missing global, or template-resolution failure would be recorded into
//! `state.lua_errors` and surface in the filtered list below.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AccountStore";
const REQUIRED_DEPS: &[&str] = &["Blizzard_SharedXML", "Blizzard_StaticPopup"];
const OPTIONAL_DEP: &str = "Blizzard_UIParentPanelManager";
const LANE_FILE_SCOPE_GLOBALS: &[&str] = &[
    "AccountStoreMixin",
    "FullscreenAccountStoreContainerMixin",
    "FullscreenLeaveAccountStoreButtonMixin",
    "AccountStoreItemRackMixin",
    "AccountStoreCategoryMixin",
    "AccountStoreCategoryListMixin",
    "AccountStoreItemDisplayMixin",
    "AccountStoreBaseCardMixin",
    "AccountStoreCreatureCardMixin",
    "AccountStoreIconCardMixin",
    "AccountStoreTransmogSetCardMixin",
    "AccountStoreUtil",
];

#[test]
fn account_store_load_emits_no_lane_specific_lua_errors() {
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
            .filter(|message| message.contains("AccountStore"))
            .cloned()
            .collect();

        assert!(
            lane_lua_errors.is_empty(),
            "Blizzard_AccountStore emitted lane-specific Lua errors during the smoke-shape \
             closure load. The addon defines AccountStoreMixin / FullscreenAccountStoreContainerMixin / \
             FullscreenLeaveAccountStoreButtonMixin / AccountStoreItemRackMixin / \
             AccountStoreCategoryMixin / AccountStoreCategoryListMixin / AccountStoreItemDisplayMixin / \
             AccountStoreBaseCardMixin / AccountStoreCreatureCardMixin / AccountStoreIconCardMixin / \
             AccountStoreTransmogSetCardMixin / AccountStoreMountCardMixin / AccountStoreUtil at \
             file scope across six Lua files plus their XML registrations — any nil-call, missing \
             global, or template-resolution failure would surface here. The filter matches any \
             error message containing the substring `AccountStore`, which covers both file paths \
             (`Interface/BlizzardUI/Blizzard_AccountStore/...`) and global identifiers \
             (`*AccountStore*Mixin`, `AccountStoreUtil`). Got:\n  {}",
            lane_lua_errors.join("\n  ")
        );
    });
}

#[test]
fn account_store_dependency_closure_includes_every_declared_dep() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
        for required in REQUIRED_DEPS {
            assert!(
                loaded.iter().any(|name| name == required),
                "RequiredDep `{required}` MUST appear in the closure-walked `loaded` set after \
                 the smoke-shape harness runs `Blizzard_AccountStore` as a root. The TOC's \
                 `## Dependencies: Blizzard_SharedXML, Blizzard_StaticPopup` line is parsed by \
                 `split_metadata_list` (src/toc.rs) and fed to the closure walker \
                 (src/loader/mod.rs:454 — `toc.dependencies().chain(toc.optional_deps())`). If \
                 either name is missing here, downstream addons that inherit StaticPopup / \
                 SharedXML templates would fail to resolve their parent template chain. \
                 Loaded set: {loaded:?}"
            );
        }

        let optional_dep_loaded = env
            .eval::<bool>(&format!(
                r#"return C_AddOns.IsAddOnLoaded("{OPTIONAL_DEP}") == true
                "#,
            ))
            .expect("IsAddOnLoaded probe must run cleanly");

        assert!(
            optional_dep_loaded,
            "OptionalDep `{OPTIONAL_DEP}` MUST be available at runtime when the smoke-shape \
             harness finishes loading `Blizzard_AccountStore`. Note: this addon's TOC declares \
             the dep using the singular form `## OptionalDep:`, but the simulator's TOC parser \
             only recognises the plural `## OptionalDeps:` (src/toc.rs:229-234). The closure \
             walker therefore won't pull `{OPTIONAL_DEP}` transitively — it lands in the runtime \
             solely via the panel-addons baseline that `with_blizzard_addon_smoke_shape` runs \
             before walking the closure (`tests/common/panel_fixtures.rs:53-56`). A regression \
             that drops `{OPTIONAL_DEP}` from the `PANEL_ADDONS` baseline, OR that fixes the \
             singular-form parser without adding the addon to the closure for this lane, would \
             flip this assertion. The test queries `C_AddOns.IsAddOnLoaded` because that surface \
             reads from `sim.addons` (the runtime registry the panel baseline populates), not \
             from the closure walker's per-call output. Closure-walked set: {loaded:?}"
        );

        assert!(
            !loaded.iter().any(|name| name == OPTIONAL_DEP),
            "Documented gap: `{OPTIONAL_DEP}` is currently NOT expected in the closure-walked \
             `loaded` set, because the AccountStore TOC uses singular `## OptionalDep:` and the \
             parser only handles plural `## OptionalDeps:`. If a future change extends the parser \
             to accept the singular form, this assertion will flip and serve as a tripwire — \
             remove this negative assertion and add `{OPTIONAL_DEP}` to `REQUIRED_DEPS` (or move \
             it into a positive list) so the closure walker is the single source of truth. Got \
             `{OPTIONAL_DEP}` unexpectedly present in: {loaded:?}"
        );
    });
}

#[test]
fn account_store_load_on_demand_root_executes_file_scope_code() {
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

        for global_name in LANE_FILE_SCOPE_GLOBALS {
            let is_table = env
                .eval::<bool>(&format!(
                    r#"return type({global_name}) == "table"
                    "#,
                ))
                .expect("file-scope global type probe must run cleanly");

            assert!(
                is_table,
                "Global `{global_name}` MUST be a table after the smoke-shape harness loads \
                 `Blizzard_AccountStore`. Each entry in `LANE_FILE_SCOPE_GLOBALS` is declared \
                 via `Mixin = {{}}` (or `CreateFromMixins(...)`) at file scope across the lane's \
                 six Lua files. If the LoadOnDemand flag silently skipped the addon's load, the \
                 closure walker would still list it in `loaded` (since LoD routing happens \
                 upstream) but the file chunks would never run — leaving these globals as nil. \
                 A nil reading here means the LoadOnDemand handling in `load_addon` regressed: \
                 the addon was discovered but its file chunks didn't execute. Got \
                 `type({global_name}) == \"table\"` returned false."
            );
        }
    });
}
