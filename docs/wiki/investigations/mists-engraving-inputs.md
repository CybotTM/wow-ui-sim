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

Absence of input is not supplied false/nil. Tests supply state before loading addons. Each reached query must reject missing input explicitly. Refresh requests have an observable simulator count, not inferred native refresh side effects. The old storage-flow test must stay byte-identical: a named unsupplied-state failure is boundary advancement only. Separate supplied tests must establish actual overlay, targeting and settings behavior.

## Proof status

State/test scaffolding is committed. Executable RED/GREEN, stock captures and seven-profile all-target checks are pending build-slot admission. No implementation or passing-behavior claim is made here.

## Sources

- [Feature contract](../../specs/mists-engraving-inputs.md)
- [Selected consumer proof rules](consumer-selection-and-red-proof.md)
- [Original storage-flow predecessor](invented-query-defaults.md#legacy-tutorial-request-boundary)

## See Also

- [Addon loading pipeline](../../addon-loading-pipeline.md)
