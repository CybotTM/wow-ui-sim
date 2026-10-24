# Wrath S49 companion projection

Legacy mount companion queries project explicit host inputs through the original cached PetPaperDollFrame consumer. Source: `src/lua_api/globals/real/wrath_legacy/companions.rs`. See [legacy fixture architecture](../wiki/systems/blizzard-ui-test-lanes.md).

## What it must do

- [x] Count owned companions intersected with explicit host eligibility, independently of modern journal usability. Missing creature metadata does not remove a known eligible mount from the count.
- [x] Return exactly five info positions for supplied mount metadata: numeric creature ID, optional cached name, spell ID, optional texture path, active numeric one or nil. Missing metadata or out-of-range position returns zero values, never fabricated identity.
- [ ] Sort resolved creature names case-insensitively and refresh original buttons inline after explicit companion events. Current consumer proof is blocked by GetCompanionCooldown.
- [x] Preserve Retail's original journal oracle; register the projection only when client_profile::ACTIVE is Wrath.

## How it works

- [Consumer selection and standalone RED rules](../wiki/investigations/consumer-selection-and-red-proof.md).
- [Widget/Lua state connection](../frame-data-flow.md).

## Implementation inventory

- `src/lua_api/globals/real/wrath_legacy/companions.rs`: selected owned/eligible mount list, count/info projection and runtime-profile registration.
- `src/lua_api/globals/real/wrath_legacy_state.rs`: spell-keyed CompanionMetadata host input; default map empty.
- `src/lua_api/globals/real/wrath_legacy.rs`: delegates companion registration before shared compat bootstrap.

## Tests asserting this spec

- `tests/wrath_s49_companion.rs`: explicit metadata, identity, optional slots, missing metadata, out-of-range position; original consumer list transitions.
- `tests/test_showuipanel_toggles.rs`: unchanged original collections oracle, passing on Retail; blocked on Wrath's unsupplied default metadata.

S49 creature IDs91001/91002 are labelled host fixture data, **not** native creatures for catalog spells458/23338. Journal mount IDs and icon fileDataIDs must never substitute for creature IDs or texture paths. A5 is the supplied laptop request-5 answer under `handoff/laptop-labels-answers-20261018/`; its native-facing signatures/sorting are reconstructed/static evidence, not authenticated native execution.

## Known gaps (current cycle)

- [ ] GetCompanionCooldown is missing at selected PetPaperDollFrame.lua:429; await laptop request11 Q5 before implementation.
- [ ] Default spell-to-creature metadata is unresolved (laptop request11). Missing metadata blocks populated default-catalog consumers, not explicit host-input tests. Original master oracle stays byte-identical.
- [ ] Independent UnitHasRelicSlot and scrolling-edit cursorOffset errors remain visible. Do not hide them or weaken S49's unchanged final empty-error assertion.

## Out of scope

Populated critter spell/creature joins, native learn/cache actions, pending-cache gating, event emission/order, complete numeric coercion, paging and authentic default catalog rows require separate evidence/work. Existing critter counts are preserved. Missing-name ordering is unspecified. Producer never calls the consumer directly or mutates buttons. Non-Wrath shared compat behavior remains unchanged.
