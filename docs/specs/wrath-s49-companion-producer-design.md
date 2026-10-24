# Wrath S49 companion projection

Legacy mount companion queries project explicit host inputs through the original cached PetPaperDollFrame consumer. Source: `src/lua_api/globals/real/wrath_legacy/companions.rs`. See [legacy fixture architecture](../wiki/reference/blizzard-ui-test-lanes.md).

## What it must do

- [x] Count owned companions intersected with explicit host eligibility, independently of modern journal usability. Missing creature metadata does not remove a known eligible mount from the count.
- [x] Return exactly five info positions for supplied mount metadata: numeric creature ID, optional cached name, spell ID, optional texture path, active numeric one or nil. Missing metadata or out-of-range position returns zero values, never fabricated identity.
- [ ] Sort resolved creature names case-insensitively and refresh original buttons inline after explicit companion events. Cooldown producer is staged; original-consumer proof still requires execution and retains independent error blockers.
- [ ] Return companion cooldown start/duration in seconds and numeric enable, with idle `(0, 0, 1)`, from existing spell timers, host category membership and matching start-recovery state. Implementation/tests staged; runtime proof deferred.
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

- [ ] Execute staged GetCompanionCooldown selectors and original consumer at PetPaperDollFrame.lua:429. Source-derived contract is accepted laptop request11 Q5; no executed RED/GREEN claim in this edit-only cycle.
- [ ] Default spell-to-creature metadata remains unsupplied. Request11 Q2 identifies `Spell.effectMiscValue[0]`, pushed even when zero; cache holds only the name. `data/spells.rs::SpellInfo` and `src/bin/wow_cli/gen_spells.rs` do not retain that field, so the explicit host input remains. Q3/Q4 provide no authentic fixture pairs or values for spells458/23338. Missing metadata blocks populated default-catalog consumers, not explicit host-input tests. Original master oracle stays byte-identical.
- [ ] Independent UnitHasRelicSlot and scrolling-edit cursorOffset errors remain visible. Do not hide them or weaken S49's unchanged final empty-error assertion.

## Cooldown scope and proof boundary

The Wrath runtime `client_profile::ACTIVE` registration adds `GetCompanionCooldown`. It selects the same owned/eligible sorted mount spell as the info query, without requiring creature metadata. Wrong/missing category or zero/non-numeric index raises the usage error; negative or out-of-range indices return no values. Supported decimal numeric-string indices are converted. Missing critter spell joins return no values, not fabricated timers.

Timing reads the existing `spell_cooldowns` and `spell_cooldown_categories`
directly; it must not call the generic unconditional-GCD helper. The existing
`gcd` interval contributes only when a supplied non-absent per-spell
start-recovery category equals its supplied `gcd_category`. Absent applicability
is UNKNOWN, not permission to apply a global GCD. Supplied on-hold markers
produce numeric enable=0 on the selected spell/category recovery; other active
intervals and ordinary idle use numeric enable=1. No second timer store exists.
Missing generated spell records or supplied excluded-effect (78/47) markers
return the wrapper's initialized `(0,0,1)`.

### Q5 evidence and UNKNOWN ledger

Q5 establishes own, same-category, shared-flag and matching start-recovery
paths, but does not specify their complete arbitration/tie rules, which shared
categories can affect a different category, on-hold clock/output evolution,
or simultaneous hold/GCD precedence. Do not infer those rules from this model.
The minimal supplied-state representation retains latest-ending intervals for
own/same-category/matching GCD (including existing longer-own assertions).
No shared-flag expansion, separate category timer, fabricated spell effect
catalog, hold freeze/resume mutation, or native precedence claim is added.
`cooldowns_on_hold`, `cooldown_excluded_spells`, start-recovery membership and
GCD category are explicit host inputs, empty/absent by default; those defaults
are simulator initialization, not native character state. Generated spell
availability is the local record-presence oracle, not proof of native cache
availability. Companion active-player absence and critter spell joins remain
unmodeled. Full Q5 helper equivalence remains unproved for these dimensions.

Each cooldown selector now loads selected original FrameXML and drives
PetPaperDollFrame's original updater, observing real cooldown visibility and
changing start/duration. Direct tuple/argument checks remain contract tests.
The original row/event selector also changes a supplied timer twice and clears
it, without weakening its row or final error assertions. See the wiki's static
contradiction table; executed RED/GREEN is deferred until wrath-relic-slot merges.
Retail exact36/42 full message→count preservation and Wrath stock startup still
require admitted runtime proof.

See [source review and implementation limits](../wiki/investigations/wrath-companion-cooldown.md).

## Out of scope

Populated critter spell/creature joins, native learn/cache actions, pending-cache gating, event emission/order, complete numeric coercion, paging and authentic default catalog rows require separate evidence/work. Existing critter counts are preserved. Missing-name ordering is unspecified. Producer never calls the consumer directly or mutates buttons. Non-Wrath shared compat behavior remains unchanged.
