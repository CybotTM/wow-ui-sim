# Bounty-set map associations

`C_Map.GetBountySetMaps` queries explicit simulator-owned bounty-set membership. Current Retail/PTR cached `MapDocumentation.lua` declares required numeric `bountySetID`, a non-nil table of numeric map IDs and `AllowedWhenUntainted`; cached `MapUtil` forwards/memoizes the result and `WorldMapBountyBoard` consumes it with `ipairs`. [Source boundary](../wiki/investigations/patch-8-1-0-api-audit.md). Historical8.1 evidence is name-only, not this current contract.

## What it must do

- [ ] Return exactly one fresh numeric sequence for explicitly supplied set membership, preserving supplied values/order rather than deriving membership from map hierarchy/faction metadata.
- [ ] Return an empty sequence when no modeled association exists. This is simulator absence policy respecting the non-nil sequence contract, not native unknown-ID evidence.
- [ ] Keep normal defaults free of invented bounty memberships; a test's supplied relation is not production/native catalog data.
- [ ] Reflect owned-input updates/removal in subsequent queries while preserving previous snapshots and caller mutation isolation.
- [ ] Validate required numeric inputs without wrong-type coercion or lossy identity aliasing; apply documented secret authentication before validation/lookup and preserve wrappers/caller taint.
- [ ] Register the modeled query for current Retail/PTR channels. No inferred earlier-profile backport or revival of retired `GetBountySetIDForMap`.

## How it works

- [C API ownership](../../AGENTS.md#c-api-boundary)
- [Current handoff](../wiki/investigations/integrated-source-and-factory-proof-2026-10-09.md)

## Implementation inventory

- Planned C API-owned query and explicit `SimState` membership map; initialized empty, no default associations.
- `src/c_api/c_map.rs`: existing map surface registration seam.

## Tests asserting this spec

- `tests/c_map_bounty_set_maps.rs`: initial public-query absence/shape, snapshots, validation/security tests being authored without future state symbols or mock getter.
- Positive explicit-association/update/consumer controls follow real owned inputs; none executed yet.

## Known gaps (current cycle)

- [ ] Authentic RED, producer and focused GREEN/default checks/independent proof pending.
- [ ] Current publication gap retained until real runtime publication and relevant owning-target sweep proof; frozen historical evidence unchanged.

## Out of scope

Invented/default memberships, native catalog acquisition, native ordering/unknown-ID fidelity, full bounty-board lifecycle, retired APIs and unproved profile matrices. Calendar's generated normal-session data policy does not authorize Bounty seeding. These boundaries distinguish this explicit-input query from missing catalogs or native acceptance; parent handoff remains open.
