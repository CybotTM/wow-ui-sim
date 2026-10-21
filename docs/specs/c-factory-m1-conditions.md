# Explicit startup conditions after C namespace factory removal

M1 restores only original-consumer conditions exposed by [factory removal](c-namespace-nil-members.md). The named defaults live in `src/lua_api/workarounds/temporary/startup_condition_capability_defaults.rs`; [implementation notes](../wiki/systems/c-factory-startup-followup.md) distinguish placeholders from modeled policy.

## What it must do

- [ ] `C_PaperDollInfo.IsRangedSlotShown()` returns one nonnil boolean on Retail, PTR, Mists, Era, Anniversary and Forever, matching their generated documentation.
- [ ] `C_GameRules.IsHardcoreActive()` returns one nonnil boolean for the actual Mists/Era/Anniversary/Forever popup consumers; an arbitrary game-rule key must not be interpreted as native Hardcore policy.
- [ ] Forever `C_DateAndTime.IsDayTime()` returns one nonnil boolean for the original Camelot diel consumer.
- [ ] Each condition is a named temporary false default, preserving the old factory's false-y branch without claiming native policy. Other unknown namespace members remain nil; Wrath remains unchanged.

## How it works

- [Named conditions and missing backing systems](../wiki/systems/c-factory-startup-followup.md#m1-conditions)

## Implementation inventory

- `src/lua_api/workarounds/temporary/startup_condition_capability_defaults.rs` — raw-presence-guarded, profile-scoped conditions.
- `src/lua_api/workarounds/{mod.rs,temporary/mod.rs}` — bootstrap registration.
- `src/lua_api/workarounds/temporary/game_rules_namespace_fallback.rs` — preserves its unrelated unknown-member diagnostic test.

## Tests asserting this spec

- `tests/cfactory_m1_conditions.rs` — executes active-profile generated documentation and verifies the declared nonnil boolean contract plus unknown-member absence.

## Known gaps (current cycle)

- [ ] Observe new contract-test RED at test-only revision, then targeted GREEN and combined stock profile proof.

## Out of scope

Native realm game-rules state, ranged-slot display capability state and an authoritative day/night clock are missing. These defaults must be retired individually when those systems exist. No native-policy inference, vendor edits or arbitrary member factory is authorized.
