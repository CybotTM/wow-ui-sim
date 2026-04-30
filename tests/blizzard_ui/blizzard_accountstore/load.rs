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
