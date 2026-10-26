# Wrath S49 companion projection

Legacy companion queries project explicit host inputs through the original cached PetPaperDollFrame consumer. Source: `src/lua_api/globals/real/wrath_legacy/companions.rs`. See [legacy fixture architecture](../wiki/reference/blizzard-ui-test-lanes.md).

## What it must do

- [x] Count owned companions intersected with explicit host eligibility, independently of modern journal usability. Missing creature metadata does not remove a known eligible mount from the count.
- [x] Return exactly five info positions for supplied mount/critter metadata: numeric creature ID, optional cached name, spell ID, optional texture path, active numeric one or nil. Missing metadata or out-of-range position returns zero values, never fabricated identity.
- [ ] Sort resolved creature names case-insensitively and refresh original buttons inline after explicit companion events. Cooldown producer is staged; original-consumer proof still requires execution and retains independent error blockers.
- [ ] Return companion cooldown start/duration in seconds and numeric enable, with idle `(0, 0, 1)`, from ordered host spell history, category flags and matching start-recovery state. Implementation/tests staged; runtime proof deferred.
- [x] Preserve Retail's original journal oracle; register the projection only when client_profile::ACTIVE is Wrath.

## How it works

- [Consumer selection and standalone RED rules](../wiki/investigations/consumer-selection-and-red-proof.md).
- [Widget/Lua state connection](../frame-data-flow.md).

## Implementation inventory

- `src/lua_api/globals/real/wrath_legacy/companions.rs`: selected owned/eligible mount/critter lists, count/info projection and runtime-profile registration.
- `src/lua_api/globals/real/wrath_legacy_state.rs`: explicit metadata, critter spell joins, category flags and ordered CompanionRecovery history; empty defaults.
- `src/lua_api/globals/real/wrath_legacy/companions/recovery.rs`: req-13 endpoint arbitration and ordered own/category/GCD branches.
- `src/lua_api/globals/real/wrath_legacy.rs`: delegates companion registration before shared compat bootstrap.

## Seeded mount spell backing

The compact spell generator must select every summon spell in the simulator's
seeded mount catalog, including uncollected or unusable mounts. Selection must
not depend on current ownership. Rows still require authoritative SpellName
input; unknown IDs must not receive invented metadata. BattlePetSpecies joins
and creature metadata are outside this repair.

`src/bin/wow_cli/gen_spells.rs::tests::required_spells_include_the_seeded_mount_catalog`
asserts the nine distinct seeded spell IDs, including Brown Horse 458.
`src/spell_lookup.rs::tests::brown_horse_has_catalog_spell_metadata` retains the
name/icon contract. Both require slot execution; current generated data remains
pre-repair until regeneration.

## Tests asserting this spec

- `tests/wrath_s49_companion.rs`: explicit metadata, identity, optional slots, missing metadata, out-of-range position; original consumer list transitions.
- `tests/test_showuipanel_toggles.rs`: unchanged original collections oracle, passing on Retail; blocked on Wrath's unsupplied default metadata.

S49 creature IDs91001/91002 are labelled host fixture data, **not** native creatures for catalog spells458/23338. Journal mount IDs and icon fileDataIDs must never substitute for creature IDs or texture paths. A5 is the supplied laptop request-5 answer under `handoff/laptop-labels-answers-20261018/`; its native-facing signatures/sorting are reconstructed/static evidence, not authenticated native execution.

## Known gaps (current cycle)

- [ ] Execute staged GetCompanionCooldown selectors and original consumer at PetPaperDollFrame.lua:429. Source-derived contract is accepted laptop request11 Q5; no executed RED/GREEN claim in this edit-only cycle.
- [ ] Default spell-to-creature metadata remains unsupplied. Request11 Q2 identifies `Spell.effectMiscValue[0]`, pushed even when zero; cache holds only the name. `data/spells.rs::SpellInfo` and `src/bin/wow_cli/gen_spells.rs` do not retain that field, so the explicit host input remains. Q3/Q4 provide no authentic fixture pairs or values for spells458/23338. Missing metadata blocks populated default-catalog consumers, not explicit host-input tests. Original master oracle stays byte-identical.
- [ ] Independent UnitHasRelicSlot and scrolling-edit cursorOffset errors remain visible. Do not hide them or weaken S49's unchanged final empty-error assertion.

## Cooldown scope and proof boundary

The Wrath runtime `client_profile::ACTIVE` registration adds `GetCompanionCooldown`.
It selects the same owned/eligible name-sorted spell as info for MOUNT or CRITTER.
Critter species-to-spell joins are explicit host input, never journal-ID aliases.
Info uses the supplied spell-keyed creature metadata for original critter rows.
Wrong/missing category or zero/non-numeric index raises the usage error; negative
or out-of-range indices return no values. Decimal numeric-string indices work.
Explicit active-player absence returns zero Lua values for both categories, not
idle values: the original consumer then leaves a populated button's timer intact.

### Accepted req-13 arbitration

Accepted `laptop-answers-req-13.md` arrived during this repair. The previous
unsupported arbitration assumption and simultaneous-timer assertions are replaced
with its source-derived rule. Times are wrapping unsigned 32-bit milliseconds;
accept when the signed difference `candidateEnd - latest` is nonnegative, including
equality with initial `now`. `RecoverySelection::accept` isolates the comparison.
A host-supplied `Vec<CompanionRecovery>` supplies native Head/Next order, without
HashMap iteration or inferred cast chronology. Each node visits own → category
(matching nonzero category OR source SpellCategory flag 0x2) → matching start
recovery/GCD. Equal endpoints overwrite, including across nodes. GCD always
competes when category matches and its recovery time is nonzero, even category0.

Held own comparison uses `now + recovery + 1`, but emits `(now, 1ms)`; held
category uses `(now, categoryRecovery)`. Only accepted held own/category candidates
set numeric enable0, which stays0 even if a non-held/GCD interval wins later.
Rejected held candidates do not alter enable. Ordinary idle is `(0,0,1)`.
Missing generated spell records or supplied excluded-effect78/47 markers return
initialized idle. No candidate acceptance is inferred from the helper's Boolean.

Ordered history replaces this producer's use of generic `spell_cooldowns`/global
`gcd`: those maps cannot express independent category timing, node order or held
comparison endpoints. They remain untouched for other APIs; there is no generic
fallback. Target category and start-recovery membership remain explicit inputs.
History/category flags/critter joins default empty and player absence defaults
false as simulator initialization, not evidence about native character state.

### Evidence and remaining UNKNOWNs

Req-11 Q5 and req-13 are accepted static clean-room evidence, not native runtime
certification. Actual spell/category records, native populated history/order and
authentic default critter spell/creature joins remain unsupplied. Rune applicability
is UNKNOWN; conditional rune recovery remains unimplemented, as requested. Full
helper/native equivalence is not claimed. Generated spell availability is only the
local record-presence oracle. No native data is invented.

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

Authentic default critter spell/creature joins, native learn/cache actions, pending-cache gating, event emission/order, complete numeric coercion, paging and authentic default catalog rows require separate evidence/work. Existing critter counts are preserved. Missing-name ordering is unspecified. Producer never calls the consumer directly or mutates buttons. Non-Wrath shared compat behavior remains unchanged.
