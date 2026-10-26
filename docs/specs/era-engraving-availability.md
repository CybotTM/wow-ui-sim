# Era and Anniversary engraving availability

Era and Anniversary expose supplied engraving availability through `src/c_api/c_engraving.rs`, not an invented inactive default. See [the investigation](../wiki/investigations/era-engraving-availability.md) for original consumer selection and proof limits.

## What it must do

- [ ] Publish only `C_Engraving.IsEngravingEnabled` on Era and Anniversary, selected through `client_profile::ACTIVE`. Preserve Mists' existing seven-member model and other profiles' publication.
- [ ] Return one supplied environment-local boolean. Missing input raises `C_Engraving.IsEngravingEnabled: unsupplied input`; supplied false is not missing input.
- [ ] Remove the temporary constant-false Lua default without seeding stock availability or inferring seasonal policy.
- [ ] Drive the selected original `RuneFrameControlButton` XML-bound OnLoad with supplied true/false and unset, including transitions, recovery and independent environments. Exact RED selectors must reject the old constant-false implementation.
- [ ] Attempt the complete original settings closure as preservation; report its actual boundary rather than a full-settings acceptance claim.

## How it works

- [Selection, state and bounded evidence](../wiki/investigations/era-engraving-availability.md)

## Implementation inventory

- `src/c_api/c_engraving.rs` — shared availability reader; remaining six members stay Mists-only.
- `src/c_api/engraving_inputs.rs` — existing environment-local `enabled: Option<bool>` input.
- `src/lua_api/workarounds/temporary/classic_startup_defaults.rs` — no engraving policy remains.

## Tests asserting this spec

- `tests/era_engraving.rs` — original XML OnLoad and settings preservation attempt.

## Known gaps (current cycle)

- [ ] Complete Era/Anniversary RED/GREEN and stock pairing with per-key attribution.
- [ ] Seven full-feature all-target zero-warning checks, formatting and readability.

## Out of scope

Bags and full LoD EngravingUI are excluded. The other 25 declared engraving members remain unpublished on Era/Anniversary: category/exclusive/equipped/search filters, catalog/count queries, equipment/inventory eligibility and rune queries, current-cast/mode queries, casting, refresh and mode commands. Those declarations do not imply simulator support. No native inactive default or complete engraving lifecycle is claimed. The six other existing Mists members remain Mists-only.

## Pinned declaration

Both Era and Anniversary `Blizzard_APIDocumentationGenerated/EngravingInfoDocumentation.lua:179–186` independently declare the non-null bool query. Both select `Blizzard_CharacterFrame/Vanilla/PaperDollFrame.xml:716,722` and `PaperDollFrame.lua:950–954`; settings reads availability at `Classic/InterfaceOverrides.lua:136–138`.
