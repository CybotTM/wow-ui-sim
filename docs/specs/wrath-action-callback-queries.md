# Wrath action callback queries

Native Wrath action, stance and pet callbacks must read profile-correct query results from simulator state. Native fixture coverage is described in [the callback fixture](../../tests/fixtures/wrath-action-callbacks.md); the shared range backing model lives under `src/c_api/` without publishing modern functions on Wrath.

## What it must do

### Range snapshots and publication

- [ ] Supplied spell/target GUID range snapshots can be seeded by Wrath fixtures without publishing `C_ActionBar.IsActionInRange`, `HasRangeRequirements` or `GetActionAutocast` on Wrath. Modern registration remains `retail-12-0-0` only.
- [ ] Legacy `IsActionInRange(slot[, unit])` returns exactly one numeric 1/0 or nil; never boolean. Valid supplied in-range/out-of-range snapshots drive original `ActionButton_OnUpdate` hotkey colors.
- [ ] Invalid/missing slot or index outside 1..144 returns one nil; numeric strings are accepted. Empty/unresolved action, target or snapshot returns nil. Omitted unit uses current target for directly bound spells.
- [ ] No special unranged-action result is invented. **INFERRED host model:** absent supplied range/eligibility data remains indeterminate; snapshots represent the result of the shared native range/target-validity checker, not computed geometry.

### Stance and pet shapes

- [ ] Wrath `GetShapeshiftFormInfo` returns texture, name, numeric 1/nil active and castable flags; active forms are castable. Invalid zero/nonnumeric arguments error; out-of-range forms return zero values. Native ShapeshiftBar_UpdateState checks the selected button.
- [ ] Wrath `GetPetActionInfo` returns seven values name, subtext, texture, token, active, autocast-allowed, autocast-enabled; empty slots return seven nils and nonnumeric input errors. Spell subtext is empty when unsupplied, spell active/token flags are nil, other flags are numeric 1/nil. Token names/textures are supplied native token strings, subtext/autocast flags nil, active numeric 1/nil.
- [ ] Non-Wrath query values and registration remain unchanged.

## How it works

- [Lua API state architecture](../wiki/systems/lua-api.md)
- [F04/F05 native callback fixture](../../tests/fixtures/wrath-action-callbacks.md)

## Implementation inventory

- `src/c_api/action_spell_range.rs` — supplied spell/target snapshot type; no Lua registration.
- `src/c_api/c_action_bar_queries.rs` — existing Retail query registration and readers; Retail-only module.
- `src/lua_api/state{,/sim_state}.rs` — profile-scoped snapshot storage.
- `src/lua_api/globals/real/pet_bar.rs` — Wrath seven-value spell/token projection; modern nine-value output preserved.
- `src/lua_api/globals/real/shapeshift.rs` — Wrath texture/name/numeric-flags projection; modern texture/boolean-flags/spellID preserved.
- `src/lua_api/globals/real/wrath_legacy/action_bar.rs` — native legacy numeric/nil range adapter over shared snapshots; no C_* registration.
- `src/wrath/compat_bootstrap.lua` — obsolete nil-only range shim removed.

## Tests asserting this spec

- `tests/wrath_action_contract.rs` — sourced return shapes, unknown/invalid inputs and publication isolation.
- `tests/common/wrath_action_assertions.rs` — original native ActionButton_OnUpdate, ShapeshiftBar_UpdateState and PetActionBar_Update consumers, reached through original F04 test IDs.

## Known gaps (current cycle)

- [ ] Original-consumer RED and query-contract RED must precede producer behavior repairs; GREEN and Retail preservation remain pending.

## Out of scope

Native engine execution, range geometry/eligibility computation, unsupported macro stored-target sentinel distinctions, missing spell/item records, active-icon lookup, pet command/mode inference, pet-presence reconstruction and unrelated client profiles. Existing supplied pet/form records remain fixture inputs, not a claim that modern default spell IDs are 12340 game data.

Authoritative source: laptop static 12340 notes `handoff/laptop-labels-answers-20261018/{action-inrange-12340.md,pet-shapeshift-info-12340.md}` (2026-10-20). Native consumer positions: cached `FrameXML/{ActionButton.lua:458+,BonusActionBarFrame.lua:152+,PetActionBarFrame.lua:106+}`. Static reconstruction is not native execution.
