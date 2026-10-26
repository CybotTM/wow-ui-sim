# Supplied transmog availability

Retail/PTR and Mists use one explicitly supplied environment-local availability observation in `src/c_api/transmog_availability.rs`. Neither the query nor the modern pending/apply model infers a native default. [Model and proof boundary](../wiki/systems/transmog-availability.md).

## What it must do

- [ ] Publish `C_TransmogOutfitInfo.IsTransmogEnabled` only for Retail, PTR and Mists through `client_profile::ACTIVE`; preserve absence on the other profiles.
- [ ] Initialize availability to None; Some(true)/Some(false) return exactly one corresponding boolean; None raises `C_TransmogOutfitInfo.IsTransmogEnabled: transmog availability has not been supplied`.
- [ ] Modern pending staging, set/custom-set imports and application consume this same observation; collected pending records reflect supplied availability, while commit checks current availability, not only the staged record.
- [ ] Original selected set/custom-set mouse callbacks and popup apply callbacks discriminate both boolean values, independent environments, unset rejection and recovery; failed application preserves money/pending/saved state.
- [ ] Preserve the original Mists settings branch contract and replace old modern fixture defaults with explicit observations.

## How it works

- [Shared observation and original-consumer proof](../wiki/systems/transmog-availability.md)
- [Mists display branch](../wiki/systems/mists-transmog-enabled.md)

## Implementation inventory

- `src/c_api/transmog_availability.rs`: supplied input, named error and profile-gated query.
- `src/c_api/c_transmog_outfit_info/pending.rs`: pending-record and current apply guards.
- `src/c_api/registration.rs`: shared registration, without ungating the outfit module.
- `src/lua_api/state/sim_state.rs` and `state.rs`: per-environment input without guessed true.
- `tests/retail_transmog_enabled.rs`: selected original interactions and direct query contract.
- `tests/mists_transmog_enabled.rs`: preserved original Mists control branches.

## Tests asserting this spec

`tests/retail_transmog_enabled.rs` and `tests/mists_transmog_enabled.rs`. Exact execution evidence is recorded in the wiki, not inferred from source.

## Known gaps (current cycle)

- [ ] Complete producer-only RED/GREEN, paired stock raw-map comparison and seven zero-warning all-target checks.

## Out of scope

Native entitlement/default policy, automatic supply, account-wide semantics, persistence, inferred events, new admin methods, new other-profile publication and cached-slot invalidation redesign. Retail 70448 has no original direct query caller; PTR IsAvailable:512 exists in a selected file but its only original constructor is excluded Camelot code. Original interaction tests do not establish a live spellbook query flow or complete wardrobe acceptance.
