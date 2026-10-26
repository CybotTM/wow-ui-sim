# Mists engraving inputs

The old Mists storage-flow test reaches the original shared bag's missing `C_Engraving.GetEngravingModeEnabled` call. The [bounded contract](../../specs/mists-engraving-inputs.md) covers the seven normal bag/settings members with explicit supplied state, not invented native inactive values.

## Selected consumers

In the pinned Mists UI, `Blizzard_UIPanels_Game_Classic.toc:67–69` selects `Mists/ContainerFrame.lua`, `Classic/ContainerFrame_Shared.lua` and `Classic/ContainerFrame.xml`. The shared Lua calls:

| Member | Call lines | Consumed result |
|---|---|---|
| GetEngravingModeEnabled | 54,95,315 | Enables rune overlay branch |
| GetCurrentRuneCast | 316 | Nonnil enables targeting desaturation |
| RefreshRunesList | 1078 | No result used; conditional on alwaysShowRuneIcons |
| IsEngravingEnabled | 1088 | Gates overlay and slot lookup |
| IsInventorySlotEngravable | 1088 | Gates per-slot rune projection |
| GetRuneForInventorySlot | 1089 | Optional iconTexture; nil uses original empty-slot texture |
| IsInventorySlotEngravableByCurrentRuneCast | 1111 | Negated into item-icon desaturation |

`Blizzard_SettingsDefinitions_Frame_Classic.toc:8` selects `Classic/InterfaceOverrides.lua`; line138 tests `IsEngravingEnabled` to register the rune-icon checkbox. All seven members and the complete rune structure are declared in pinned generated `EngravingInfoDocumentation.lua`; declaration lines are linked from the contract.

## Excluded cached files

Mists `Blizzard_CharacterFrame.toc:18–21` selects Cata/PaperDollFrame.lua, not Vanilla/PaperDollFrame.lua. The latter contains equipment queries and the normal `ToggleEngravingFrame` entry. Era/Anniversary select that Vanilla file; both also select the same shared bag/settings consumers. Their generated engraving documentation is byte-identical to Mists. Wrath's complete pinned profile tree has no C_Engraving references.

`Blizzard_EngravingUI` is LoadOnDemand and explicitly loadable on Mists, but has no normal Mists UI entry. Its fourteen additional query/filter/cast members are outside this normal-flow slice. Their declarations do not establish native catalog filtering, cast transitions or default values. Era/Anniversary's existing temporary false availability remains a separate gap, not native evidence.

## Supplied input boundary

Absence of input is not supplied false/nil. Tests supply state before loading addons. Each reached query must reject missing input explicitly. Refresh requests have an observable simulator count, not inferred native refresh side effects. The old storage-flow test must stay byte-identical: a named unsupplied-state failure is boundary advancement only. Separate supplied tests establish actual overlay and targeting behavior. Complete settings remains blocked before engraving at `InterfaceOverrides.lua:4`, where `C_TransmogOutfitInfo.IsTransmogEnabled` is absent. Its selector was removed on main's direction. No settings-path GREEN, copied vendor excerpt or adjacent shim is claimed.

## Proof status

Producer `f7a9227b8` registers exactly seven members only when `client_profile::ACTIVE` is Mists. Corrected tests are byte-identical in RED `e97b841e7` and GREEN `eb3dca9dc`; their whole-tree difference contains only the producer module and registrar file. Each exact list selects one test, followed by one exact run. Both bag selectors reject the missing namespace at the original `ContainerFrame_OnShow` line315. All three GREEN cases pass: original overlay, original targeting, and direct projection/missing-input/isolation.

The overlay exercises true/false mode and availability, per-slot eligibility, present/nil rune textures and the original CVar-driven refresh request. Targeting exercises nil/present/nil current cast with opposite per-slot results. Direct proof checks six named missing-input errors, all record fields, one nil result versus no refresh results, independent tables, repeated requests and independent environments. Distinct observations reject constant/no-op behavior; no separately compiled constant-mutant binaries are claimed.

The unchanged old storage test still fails, now with `C_Engraving.GetEngravingModeEnabled: unsupplied input` at original315 instead of a missing namespace. This is the accepted boundary advance, not a passing storage-flow claim.

Bound Mists stock maps remain6 keys/12 occurrences, with no added, removed or count-changed raw keys. BEFORE `c50e6386a` is base `e98282ab8` plus scaffold types and the default-initialized environment field, with no producer; it is not a separately rebuilt exact master. AFTER `f7a9227b8` adds the producer. Later changes are test/documentation-only. The existing generic `(string):1` pair remains unattributed; no new key needs a masking-predecessor trace.

Corrected-source Mists all-target compilation has zero JSON/plain warnings. Six profile checks remain. Formatting and manual changed-Rust readability pass. Runtime invocations use fresh byte-exact child CVar copies and O_NOATIME source snapshots; originals are not reset/restored.

## Fixture fidelity

Initial fixture failures were not producer RED: missing original `CanOpenPanels`, then missing `MainMenuBarBackpackButton` before the engraving call. Loading the original panel-manager and bag-button closures lets original OnShow reach315. No vendor function is replaced.

Original GenerateFrame assigns reversed button IDs at953–955. Runtime observation gives Item1/Item2 IDs16/15, while inventory slots1/2 are Item16/Item15. Fixtures now locate original controls by GetID instead of equating visual index and bag slot.

The inherited empty-texture assertion expected a path string, contrary to the established `methods_texture.rs` contract. Guarded diagnostics return numeric134400 for the supplied icon and numeric136509 for the original empty-slot path. Main authorized those exact, distinct expectations and a renewed original315 RED. Neither texture semantics nor the engraving producer changed. This is existing simulator-contract evidence, not a new native-client texture probe.

## Sources

- [Feature contract](../../specs/mists-engraving-inputs.md)
- [Original consumer and direct tests](../../../tests/mists_engraving.rs)
- [Established texture round-trip contract](../../../tests/methods_texture.rs)
- [Selected consumer proof rules](consumer-selection-and-red-proof.md)
- [Original storage-flow predecessor](invented-query-defaults.md#legacy-tutorial-request-boundary)

## See Also

- [Addon loading pipeline](../../addon-loading-pipeline.md)
