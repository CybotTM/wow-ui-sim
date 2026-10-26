# Era and Anniversary engraving availability

## Contract

- Publish only `C_Engraving.IsEngravingEnabled` on Era and Anniversary from the shared C API subsystem, selected through `client_profile::ACTIVE`.
- Return one supplied environment-local boolean. Missing input raises `C_Engraving.IsEngravingEnabled: unsupplied input`; supplied false is not missing input.
- Remove the temporary constant-false Lua default. Do not seed stock availability, infer seasonal policy, or change Mists' existing supplied model.
- Prove supplied true/false and unset through the selected original `RuneFrameControlButton` XML-bound OnLoad, including transitions and independent environments. At least one exact RED selector must reject the old constant-false implementation.
- Attempt the complete original settings closure as preservation and report its actual boundary, not a full-settings acceptance claim.

## Declarations and consumers

Both pinned Era and Anniversary `Blizzard_APIDocumentationGenerated/EngravingInfoDocumentation.lua:179–186` declare the non-null bool query. Both select `Blizzard_CharacterFrame/Vanilla/PaperDollFrame.xml:716,722` and `PaperDollFrame.lua:950–954`. Both select settings `Classic/InterfaceOverrides.lua:136–138`.

## Tests

`tests/era_engraving.rs` covers the original XML control and settings preservation attempt. Execution evidence is pending.

## Exclusions and gaps

Bags and full LoD EngravingUI are excluded. The other 25 declared engraving members remain unpublished on Era/Anniversary: category/exclusive/equipped/search filters, catalog/count queries, equipment/inventory eligibility and rune queries, current-cast/mode queries, casting, refresh and mode commands. Those declarations do not imply simulator support. No native inactive default or complete engraving lifecycle is claimed. The existing six other Mists members remain Mists-only.
