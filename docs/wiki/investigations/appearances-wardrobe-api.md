# Appearances Wardrobe API

Collections Journal > Appearances opens and renders a `WardrobeCollectionFrame`, but most meaningful browsing behavior is still backed by fixed/default transmog API stubs instead of simulator state.

## Content

Baseline:

- `target/debug/wow-sim` builds in this worktree when `LD_LIBRARY_PATH=target/debug` is used for the local `libiced_dynamic.so`.
- `wow-sim --no-addons --no-saved-vars lua-errors` returns `[]`.
- Opening the Wardrobe tab with `ToggleCollectionsJournal(5)` plus `CollectionsJournal_SetTab(CollectionsJournal, 5)` produces a visible `WardrobeCollectionFrame` subtree.

Current API shape:

- `C_TransmogCollection.GetCategoryAppearances` returns seeded rows by category and active collection/source/search filters.
- Source/collection/search filter setters now mutate `WorldState`, and category rows/counts apply collected/uncollected, source-type, and search-text filters.
- Search completion is synchronous and deterministic: no DB loading, no in-progress state, and progress equals result size.
- `C_TransmogOutfitInfo.GetAllSlotLocationInfo` reports weapon appearance slots with `Enum.TransmogCollectionType.None`; Blizzard derives weapon browsing categories through `IsEitherHand()` and `GetCategoryInfo()`. Reporting weapon collection categories there makes `TransmogLocationMixin:GetArmorCategoryID()` non-nil for weapons, and Wardrobe's armor-only model setup path then indexes missing `MAINHANDSLOT` / `SECONDARYHANDSLOT` entries in `WARDROBE_MODEL_SETUP`.
- Tooltip and visual-state helpers such as `GetAppearanceSourceInfo`, `GetAppearanceInfoBySource`, `GetIsAppearanceFavorite`, `SetIsAppearanceFavorite`, `IsNewAppearance`, and `ClearNewAppearance` are not backed by first-class visual/source state.
- `C_TransmogSets` is currently a Lua bootstrap fallback returning empty/default values. That is enough to keep the UI from crashing, but not enough for real set-tab filtering.

Implementation direction:

- Treat the Wardrobe panel as a C API/state-model problem, not a Blizzard Lua patching problem.
- Add stateful transmog source, visual, filter, search, and favorite state before trying to polish UI output.
- Keep in-world `Blizzard_Transmog` compatible through the shared `C_TransmogCollection` surface, but defer transmogrifier transaction/apply-cost behavior until browsing/filtering works.
- Keep 3D model preview gaps isolated to existing model stubs.

## Sources

- [PLAN.mog.md](../../../PLAN.mog.md) — baseline commands, target scope, and audit checklist
- [Blizzard_Wardrobe.lua](../../../vendor/wow-ui-source/Interface/AddOns/Blizzard_Collections/Mainline/Blizzard_Wardrobe.lua) — Collections Journal Wardrobe call sites
- [Blizzard_Transmog.lua](../../../vendor/wow-ui-source/Interface/AddOns/Blizzard_Transmog/Blizzard_Transmog.lua) — in-world transmog call sites
- [Blizzard_TransmogShared.lua](../../../vendor/wow-ui-source/Interface/AddOns/Blizzard_TransmogShared/Blizzard_TransmogShared.lua) — shared tooltip/favorite/source helpers
- [transmog_collection.rs](../../../src/lua_api/globals/missing_surface/transmog_collection.rs) — current Rust `C_TransmogCollection` surface
- [transmog.rs](../../../src/lua_api/globals/missing_surface/transmog.rs) — current Rust `C_Transmog` surface
- [runtime_surface_bootstrap.lua](../../../src/lua_api/env_init/runtime_surface_bootstrap.lua) — current Lua fallback `C_TransmogSets` surface

## See Also

- [[lua-api]] — C API namespace registration and Lua surface conventions
- [[frame-data-flow]] — Lua/Rust state synchronization model
- [[addon-loading]] — Blizzard addon loading and startup sequence
