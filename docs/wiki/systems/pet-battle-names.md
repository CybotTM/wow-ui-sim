# Pet-battle names

CF-2 adds a supplied-state `C_PetBattles.GetName` query. Staged 2026-10-24; no runtime RED/GREEN or compilation has been performed. [Contract](../../specs/pet-battle-names.md) owns requirements and acceptance totals.

## State projection

`src/c_api/c_pet_battles.rs` selects `SimState.pet_battles.player_pets` or `enemy_pets` by owner, then a one-based slot. `PetBattlePet.name` is the supplied effective battle display name. The pet's species ID joins `SimState.world.pets.species_id`; `PetData.name` supplies the species name. Journal `custom_name` is not a battle identity source. Both strings are copied before Lua allocation, releasing the state borrow. No new mutable model, fallback, sample value or native default was added.

An absent owner/slot/species row yields zero results. Generated `PetBattlesDocumentation.lua` permits `MayReturnNothing` but makes both strings nonnil when returned. Zero results is the documented-optional representation; choosing absent/invalid inputs as its conditions is explicit simulator policy, not native parity. Missing species metadata remains a native/input evidence gap, never a fabricated name.

## Selected original consumers

At base `1aaff4c94`, `find_toc_file` selects `Blizzard_PetBattleUI.toc` in each profile cache. All four are game-screen, non-LoD roots with no `AllowLoad`, `OnlyBetaAndPTR` or exclusion metadata. Retail/PTR allow `standard, mists`; Forever adds `camelot`; Mists allows `standard, mists`. These intersect the resolver's active game types. The TOC directly selects `Shared/Blizzard_PetBattleUI.lua` and `.xml` without inline filters. `[Family]/Blizzard_PetBattleUI.lua` resolves Mainline on Retail/PTR/Forever, Classic on Mists; no `[Game]` substitution occurs here.

The shared XML defines `PetBattleUnitFrameUnclickable`, binding `PetBattleUnitFrame_OnEvent`. `PetBattleUnitTooltipTemplate` inherits it and calls `PetBattleUnitTooltip_OnLoad`, which calls `PetBattleUnitFrame_OnLoad` to register `PET_BATTLE_PET_CHANGED`. `PetBattlePrimaryUnitTooltip` instantiates that template with real `Name` and `SpeciesName` FontStrings. On the event, the original shared Lua calls `PetBattleUnitFrame_UpdateDisplay`, then `GetName` (Retail/PTR/Forever :1055, Mists :1063), then sets Name and shows/hides SpeciesName according to equality. Startup also reaches UpdateDisplay through SetUnit.

Tests use the existing profile TOC/dependency closure loader, assert parsed selection and log consumer hashes. They neither force-load a cached unselected Lua file nor replace a C API closure. A second selector uses the existing full-stock environment. Both change concrete Rust fixture state and fire the real registered event. Event execution, setup completeness and mutation sensitivity remain parent proof obligations, not inferred PASS results.

## Native and integration boundaries

Generated docs define two arguments, two strings and optional no-result shape, not native invalid-selector conditions or live availability. Existing sample catalog/battle records are preserved as simulator inputs, including any naming discrepancies; this slice does not claim native species data. Related pet-stat compatibility methods remain separate existing workarounds and are not migrated here. No vendor or decompiled client source was changed/read for implementation.

## Sources

- [C API producer](../../../src/c_api/c_pet_battles.rs) — query and runtime registration predicate shared with GetIcon.
- [Original-consumer fixtures](../../../tests/cf2_pet_identity_consumers.rs) — selected TOC/XML and real event path.
- [TOC selection](../../../src/toc/mod.rs) — game-type filters and Family substitutions.
- Profile cache `Blizzard_APIDocumentationGenerated/PetBattlesDocumentation.lua` and `Blizzard_PetBattleUI/{Blizzard_PetBattleUI.toc,Shared/Blizzard_PetBattleUI.lua,Shared/Blizzard_PetBattleUI.xml}` — source-inspected 2026-10-24 on Retail/PTR/Mists/Forever; exact hashes in private CF-2 pet-identity handoff.

## See Also

- [[consumer-selection-and-red-proof]] — applicability versus constant/no-op proof.
- [[lua-api]] — related factory-removal pet service inputs.
