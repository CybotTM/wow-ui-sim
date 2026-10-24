# Wrath companion cooldown and creature-ID source review

GetCompanionCooldown is staged for the selected legacy companion consumer, using existing simulator cooldown state. Verified source inspection: 2026-10-24. This edit-only cycle has no compilation or executed RED/GREEN evidence.

## Selected consumer and staged behavior

The prior lane report binds FrameXML.toc → PetPaperDollFrame.xml → PetPaperDollFrame.lua with no flavor/exclusion gates; its populated original button reaches line429, calling GetCompanionCooldown. The adapter registers only under runtime `client_profile::ACTIVE == Wrath` and chooses the same owned/eligible sorted mount or supplied critter spell. Creature-name metadata is not required by cooldown lookup.

Accepted req-11 Q5 and req-13 define seconds/numeric enable, idle `(0,0,1)`,
zero returns without an active player, and both companion categories. Host critter
species→spell joins and creature metadata enable original CRITTER rows; no default
native joins are fabricated. Source category flags bit0x2 qualify the same category
branch even without matching target category/spell/item.

Req-13 arrived during this edit-only repair and replaces the guessed arbitration.
`RecoverySelection::accept` compares wrapping32-bit endpoints as signed differences,
accepting ties. Explicit host history Vec order is Head/Next order; each node
visits own → category/shared → GCD. Later accepted candidates overwrite equal ends.
Accepted held own/category candidates make enable0 sticky; rejected holds do not.
Held own compares `now+recovery+1` but returns `(now,1ms)`. GCD always competes when
its supplied category matches, including category0, and its time is nonzero.

The generic cooldown maps cannot encode these independent per-node fields.
This producer now consumes ordered `CompanionRecovery` host inputs, with no
fallback to generic spell timers/GCD and no HashMap-order tie policy. Unknown
native history contents/order are not replaced by inferred chronology. Rune
applicability remains UNKNOWN and rune recovery unimplemented by request.
See [contract](../../specs/wrath-s49-companion-producer-design.md#accepted-req-13-arbitration).

## Creature-ID source review

Accepted request11 Q2 identifies `Spell.effectMiscValue[0]`, unconditionally pushed even when0. The native cache holds only the creature name; missing Spell/out-of-range returns no values.

Local `data/spells.rs::SpellInfo` exposes only name, subtext, icon fileDataID, school mask and implicit target. `src/bin/wow_cli/gen_spells.rs::load_spell_effect_targets` reads EffectIndex0/DifficultyID0's ImplicitTarget_0 and SpellID from SpellEffect.csv; it neither retains nor emits effect-misc values. Its `load_spell_misc` reads icon and school from SpellMisc.csv, not the effect-misc creature field. A bounded source/generator search for effect_misc/effectMisc/EffectMisc found no retained field. Therefore creature ID cannot currently be derived from the simulator's generated spell data. Keep the empty-default `CompanionMetadata` host input; retire its creature-ID source when a profile-correct effect-misc field is imported. Existing info code already pushes supplied0 numerically, while missing metadata yields no values.

Request11 Q3/Q4 remain UNKNOWN: no authentic fixture pairs, no verified values for458/23338. Existing91001/91002 fixture IDs remain labelled synthetic host inputs, not native rows. No new fixture pair or default metadata was invented.

## Remaining original-consumer blockers

GetCompanionCooldown's nil-call boundary has a staged producer, not a demonstrated GREEN. UnitHasRelicSlot at PaperDollFrame.lua:1094 belongs to wrath-relic-slot. Independent pre-existing cursorOffset at UIPanelTemplates.lua:365 remains untouched. The original final empty-error assertion remains unchanged and may still fail. The unsupplied default creature metadata also keeps the original default-catalog mount oracle unresolved. Earlier Wrath clean stock and Retail exact36unique/42occurrence preservation receipts do not prove this new revision.

## Sources

- [Companion spec](../../specs/wrath-s49-companion-producer-design.md) — scope and acceptance boundaries
- [Companion producer](../../../src/lua_api/globals/real/wrath_legacy/companions.rs) — runtime selection and timer projection
- [Recovery arbitration](../../../src/lua_api/globals/real/wrath_legacy/companions/recovery.rs) — req-13 ordered candidates and signed endpoint selection
- Accepted `laptop-answers-req-13.md` — accepted clean-room arbitration evidence, no decompiled source read
- [Spell generator](../../../src/bin/wow_cli/gen_spells.rs) — retained source fields
- [Generated spell schema](../../../data/spells.rs) — no effect-misc field
- Accepted `handoff/laptop-labels-answers-20261018/laptop-answers-req-11.md`, Q2–Q5 — laptop clean-room evidence; no decompiled client source read in this lane
- `handoff/wrath-s49-companion/REPORT.md` and `slot2-release-main.json` — prior consumer proof and released slot

## See Also

- [[consumer-selection-and-red-proof]] — selected consumers and standalone rejection
- [[blizzard-ui-test-lanes]] — original addon-bootstrap consumer tests

## Static selector contradiction table (not executed RED)

All four selectors load active Wrath FrameXML.toc → PetPaperDollFrame.xml →
PetPaperDollFrame.lua through panel_fixtures::setup_env. No foreign variant or
replacement consumer is loaded. Original updater :429 feeds the real
CompanionButton1Cooldown via CooldownFrame_SetTimer. Direct tuple checks are
contract tests, not consumer proof. Timings below are seconds; widget getters
are asserted in milliseconds. Idle/on-hold hides without clearing retained
timing, so hidden timing is deliberately not asserted.

| Selector (`wrath_s49_companion::`) | Consumer observations rejecting constant/no-op producers |
|---|---|
| learned_mount_refreshes_original_companion_buttons_inline | Visible90.25/300.5 → visible95/400 → hidden idle; unchanged row/event/final-error assertions |
| companion_cooldown_own_timer_seconds_and_expiry | Hidden idle/held → visible90.25/300.5 → hidden excluded → visible again → hidden expired |
| companion_cooldown_same_category_history | Hidden mismatched → visible80.5/250.25 matching category → visible95/400 separate own → peer again → hidden cleared |
| companion_cooldown_matching_gcd_and_separate_own_interval | Hidden absent/mismatched category → visible99.25/120.5 matching GCD → visible90.5/400.25 separate own → GCD again → hidden expired |
| companion_cooldown_flagged_shared_category | Hidden mismatched/unflagged → visible80.5/250.25 with source flag2 → visible95/400 → hidden flag1; no own/matching category source |
| companion_cooldown_absent_player_preserves_original_timer | Hidden idle → visible80.5/250.25 → absent player retains that timer despite changed history → restored player visible95/400 → hidden idle; direct zero-return contract also required |
| companion_cooldown_known_critter_uses_supplied_spell | Original tab2/CRITTER row enabled → hidden idle → visible80.5/250.25 → visible95/400 → hidden idle, absent player zero returns |
| companion_cooldown_req13_endpoint_and_history_ties | Hidden idle → category90/310 ties own → GCD80/320 overwrites tie → shorter GCD loses → later node100/300 overwrites → reversed node order90/310 → shorter-duration later-end GCD390/20 → hidden cleared |
| companion_cooldown_req13_accepted_hold_enable_is_sticky | Hidden idle → visibleGCD390/20 → hidden long held-own winner emitting1ms → hidden GCD winner with earlier accepted own hold → visible after reversing nodes (held rejected) → hidden with accepted category hold → visible GCD alone |

Each selector alone requires hidden and visible transitions; several distinct
non-held intervals reject any constant active tuple as well. Constant idle,
nil/no-return or no-op producers cannot create the visible timer. Constant active
results cannot meet hidden transitions; absent-player idle results would hide
rather than retain the existing timer. Direct arity/enable checks supplement,
not replace, these original widget observations. These are static contradictions,
not executed mutation/RED/GREEN results. Unit fixed-clock specimens additionally
cover signed wrap, initial-now equality, held comparison/output discrepancy and
equal-end sticky enable; they are not original-consumer RED claims.

Relic setup and independent cursorOffset/errors remain runtime blockers. The
original final empty-error assertion remains intact. No runtime verification ran.
