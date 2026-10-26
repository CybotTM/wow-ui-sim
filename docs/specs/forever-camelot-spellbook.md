# Forever supplied spellbook outfit observations

Forever 1.60.1.70532 uses the shared transmog availability observation and a separately supplied outfit-list observation. See [shared availability](transmog-availability.md) and [model and proof](../wiki/systems/forever-camelot-spellbook.md).

## What it must do

- [ ] Publish only IsTransmogEnabled and GetOutfitsInfo on Forever via ACTIVE, without enabling the modern outfit catalog.
- [ ] Reuse the shared Option<bool> and its named unset error; preserve true, false and independent environments.
- [ ] Distinguish unset (named `C_TransmogOutfitInfo.GetOutfitsInfo: outfit list has not been supplied` error), explicit no-result (zero results), and complete supplied records (one array, including empty).
- [ ] Return fresh rows and nested situationCategories arrays on each snapshot, preserving supplied record order and all seven declared fields.
- [ ] Original selected Camelot construction, availability and sparse range consumers discriminate transitions, replacements and independent environments.

## How it works

- [Model and proof boundary](../wiki/systems/forever-camelot-spellbook.md)

## Implementation inventory

- `src/c_api/transmog_availability.rs`: shared availability registrar and observation.
- `src/c_api/forever_outfits.rs`: Forever-only supplied query.
- `src/c_api/outfit_entry.rs`: complete shared record and fresh snapshot conversion.
- `src/lua_api/state/sim_state.rs` and `state.rs`: environment-local supplied list.
- `tests/forever_camelot_spellbook.rs`: original selected consumers.

## Tests asserting this spec

Three exact selectors in `tests/forever_camelot_spellbook.rs` cover enabled-first availability, disabled-first availability, and independent sparse catalog projection.

## Known gaps (current cycle)

- [ ] Execute RED/GREEN, paired stock and all seven zero-warning all-target checks.

## Out of scope

Native availability/default values, automatic supply, events, persistence, wardrobe mutations and other outfit queries. Pinned generated documentation establishes return shape, not policy: `TransmogOutfitInfoDocumentation.lua:383–392` declares GetOutfitsInfo with MayReturnNothing; entries at896–907 declare seven non-nil fields. IsTransmogEnabled at631–638 declares one non-nil bool.
