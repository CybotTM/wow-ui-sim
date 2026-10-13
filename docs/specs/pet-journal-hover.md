# Pet journal hovered battle pet

`C_PetJournal.SetHoveredBattlePet` and `ClearHoveredBattlePet` select an environment-owned hovered GUID. Existing registrations live in [pet_journal.rs](../../src/lua_api/globals/font_strings_collection/pet_journal.rs); the default-only backing state lives in [c_pet_journal_hover.rs](../../src/c_api/c_pet_journal_hover.rs). See [Lua API architecture](../wiki/systems/lua-api.md). All contract proofs remain unverified: no compilation, RED, GREEN, or native acceptance was performed for this slice.

## What it must do

### Cached documented surface

- [ ] Setter requires non-nil `battlePetGUID` (`WOWGUID`), with `SecretArguments = "AllowedWhenUntainted"`. Clear has no documented arguments. Both return zero Lua values (inferred from absent Returns entries).

### Requested simulator policies — not native observations

- [ ] Retain the supplied UTF-8 string exactly as an opaque GUID, including whitespace, case, arbitrary syntax, and non-ASCII characters. Each successful setter replaces the previous value without catalog or ownership validation, unit-token resolution, trimming, or numeric coercion.
- [ ] Empty string is retained as `Some("")`, distinct from cleared state. A fresh environment defaults to `None`; clear removes selection and is idempotent. Separate environments own independent selections.
- [ ] Missing, nil, or non-string setter arguments raise a string-validation error before mutation. Error text identifies the required string; exact wording is unspecified. This follows the existing strict GUID boundary rather than coercing string extraction.
- [ ] Authentic VM secret strings succeed when untainted and retain exact payload. Malformed untainted secret payloads reach string validation. Tainted secret inputs fail with an untainted-caller error before type validation and mutation. Public strings and clear remain usable when tainted. Preserve caller taint and the original secret wrapper.

## How it works

- [Lua API architecture](../wiki/systems/lua-api.md)
- [Client profiles](../wiki/systems/client-profiles.md)

## Implementation inventory

- [c_pet_journal_hover.rs](../../src/c_api/c_pet_journal_hover.rs): public `PetJournalHoverState`, public optional GUID, default `None`; no mutation producer or registration.
- [c_api/mod.rs](../../src/c_api/mod.rs): unconditional module, matching the existing unconditional PetJournal registration in [font_strings_collection/mod.rs](../../src/lua_api/globals/font_strings_collection/mod.rs). This adds state availability, not cross-profile behavior proof.
- [sim_state.rs](../../src/lua_api/state/sim_state.rs): public `pet_journal_hover` owned state field.
- [state.rs](../../src/lua_api/state.rs): normal empty-state initialization.
- [pet_journal.rs](../../src/lua_api/globals/font_strings_collection/pet_journal.rs): existing Set/Clear handlers remain no-ops.

## Tests asserting this spec

- [pet_hover_api.rs](../../tests/pet_hover_api.rs): strict argument errors, void returns, real VM secret authentication/type ordering and taint/wrapper preservation.
- [pet_hover_state.rs](../../tests/pet_hover_state.rs): exact UTF-8/empty retention, replacement, default, host-seeded clear, isolation, secure payload retention, tainted public mutations, and immediately observed unchanged state after invalid/security errors.

Tests are Retail/PTR-gated and unexecuted. No passing-proof boxes are checked.

## Known gaps (current cycle)

- [ ] Parent commits tests/spec/default-only scaffold before compiling behavioral RED. Missing-field compiler errors do not count as RED.
- [ ] Parent proves genuine behavioral RED against unchanged handlers before implementing mutations; then proves targeted GREEN.
- [ ] Original Collections consumers remain inspection-only. Retail cached `PetJournalInfoDocumentation.lua` declares the setter argument/security metadata and argumentless clear. Cached `Blizzard_Collections/Shared/Blizzard_PetCollection.lua` list-item OnEnter passes guarded `self.petID`; drag-button OnEnter passes guarded parent `petID` before tooltip work. Both OnLeave handlers clear unconditionally. Exact `petID` comes from `GetPetInfoByIndex`, not species ID. Actual consumer execution, native semantics, and PTR cache parity remain unverified.

## Out of scope

Native GUID syntax/catalog/ownership validation, invented events, rendering/tooltips, persistence, production getters, and adjacent PetJournal APIs are excluded. Extra-argument and invalid-byte policies are unspecified; this contract covers supplied valid UTF-8 strings, not arbitrary Lua byte strings. Cached metadata establishes signature/security evidence, not native empty/arbitrary-GUID acceptance or default/lifecycle proof.
