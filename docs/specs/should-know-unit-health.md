# Classic unit-health knowledge

User decision, October 17, 2026: `ShouldKnowUnitHealth(unit)` reads explicitly supplied per-unit simulator state; an unsupplied unit returns false. This is simulator policy, not a measured native default. The [cascade investigation](../wiki/investigations/classic-startup-producer-cascades.md#r20-approved-health-knowledge-producer) explains publication and initialization ordering.

## What it must do

- [ ] Era and Anniversary return one boolean from one per-unit state store writable by Rust fixtures. Explicit true/false, later updates and removal are observable independently for each token; unsupplied tokens return false.
- [ ] Original Classic health-bar consumers retain their display rules. Supplied player health 250/500 renders 50% when unknown and numeric values when known under NUMERIC mode.
- [ ] Original OverrideActionBar XML OnLoad and party UpdateMember complete. Original Player/Pet initialization reaches CombatFeedback initialization and first OnUpdate without feedback guards or fabricated widgets.
- [ ] Wrath and Retail retain their startup behavior and do not gain this global.

## How it works

- [Classic producer cascades](../wiki/investigations/classic-startup-producer-cascades.md#r20-approved-health-knowledge-producer)
- [Client profiles](../wiki/systems/client-profiles.md)

## Implementation inventory

- `src/lua_api/state/sim_state.rs`, `state.rs`: one empty-by-default `unit_health_knowledge` map, keyed by supplied unit token.
- `src/lua_api/globals/real/unit_health_knowledge.rs`: modeled non-C global reading that map.
- `src/era/compat_bootstrap.rs`: narrow registration shared by Era/Anniversary; no other profile publication.

## Tests asserting this spec

- `tests/classic_startup_b.rs`: reuses original lifecycle and visible-text tests from `classic-startup-b` commits `bb6597550` and `c18dc805f`; additionally calls the original vehicle OnLoad and pooled party UpdateMember independently, verifies declared feedback FontString identities, and exercises known-health numeric text.
- `tests/should_know_unit_health.rs`: supplied true/false, independent tokens, live updates, removal, post-load preservation and other-profile absence.

Private evidence: `/home/osso/.local/state/wow-ui-sim/handoff/classic-r20-health/`. Both-profile original-consumer RED at `fd6ee4eb9`: vehicle and party fail at UnitFrame.lua:737; original Player/Pet OnUpdate exposes CombatFeedback.lua:104. Both base startups are 20 records/52 occurrences. GREEN and final checks remain pending.

## Known gaps (current cycle)

- [ ] GREEN, three-profile checks/formatting and four-profile fresh startup comparisons are pending.

## Out of scope

Mists ships the same Classic UnitFrame consumer and generated API documentation, but its existing expansion-level guard excludes this pre-Wrath branch. Selected Wrath and Retail caches contain no consumer. No broader publication or native API-absence claim follows.

No inference from health values, ownership, secrecy or token class; no alias/GUID normalization, new admin API, vendor edits, constant passthrough, or changes to CombatFeedback. The earlier [B approval gate](classic-startup-b.md#r20-held-health-knowledge-policy) is historical and superseded by this explicit user decision.
