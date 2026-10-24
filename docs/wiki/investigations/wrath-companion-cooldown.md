# Wrath companion cooldown and creature-ID source review

GetCompanionCooldown is staged for the selected legacy companion consumer, using existing simulator cooldown state. Verified source inspection: 2026-10-24. This edit-only cycle has no compilation or executed RED/GREEN evidence.

## Selected consumer and staged behavior

The prior lane report binds FrameXML.toc → PetPaperDollFrame.xml → PetPaperDollFrame.lua with no flavor/exclusion gates; its populated original button reaches line429, calling GetCompanionCooldown. The adapter registers only under runtime `client_profile::ACTIVE == Wrath` and chooses the same owned/eligible sorted mount spell. Creature-name metadata is not required by cooldown lookup.

Accepted laptop request11 Q5 gives seconds, numeric enable and ordinary idle
`(0,0,1)`. Own and same-category recovery use existing spell/category state.
Start recovery uses existing `gcd` only with matching supplied per-spell and GCD
categories; unrelated/unclassified GCDs cannot affect the result. Supplied
on-hold markers supply numeric enable=0; generated missing spell records and
supplied effects 78/47 return initialized idle outputs. No duplicate timer store.

[Spec UNKNOWN ledger](../../specs/wrath-s49-companion-producer-design.md#q5-evidence-and-unknown-ledger)
records Q5's unspecified shared-flag relationships, arbitration, tie and hold-clock
semantics. The latest-ending supplied-interval representation is not a claim
of native arbitration. Shared-flag expansion and active-player absence remain
gaps. No authentic catalog effect/creature data is invented. Critter spell joins
remain unsupplied; full native numeric coercion is not claimed.

## Creature-ID source review

Accepted request11 Q2 identifies `Spell.effectMiscValue[0]`, unconditionally pushed even when0. The native cache holds only the creature name; missing Spell/out-of-range returns no values.

Local `data/spells.rs::SpellInfo` exposes only name, subtext, icon fileDataID, school mask and implicit target. `src/bin/wow_cli/gen_spells.rs::load_spell_effect_targets` reads EffectIndex0/DifficultyID0's ImplicitTarget_0 and SpellID from SpellEffect.csv; it neither retains nor emits effect-misc values. Its `load_spell_misc` reads icon and school from SpellMisc.csv, not the effect-misc creature field. A bounded source/generator search for effect_misc/effectMisc/EffectMisc found no retained field. Therefore creature ID cannot currently be derived from the simulator's generated spell data. Keep the empty-default `CompanionMetadata` host input; retire its creature-ID source when a profile-correct effect-misc field is imported. Existing info code already pushes supplied0 numerically, while missing metadata yields no values.

Request11 Q3/Q4 remain UNKNOWN: no authentic fixture pairs, no verified values for458/23338. Existing91001/91002 fixture IDs remain labelled synthetic host inputs, not native rows. No new fixture pair or default metadata was invented.

## Remaining original-consumer blockers

GetCompanionCooldown's nil-call boundary has a staged producer, not a demonstrated GREEN. UnitHasRelicSlot at PaperDollFrame.lua:1094 belongs to wrath-relic-slot. Independent pre-existing cursorOffset at UIPanelTemplates.lua:365 remains untouched. The original final empty-error assertion remains unchanged and may still fail. The unsupplied default creature metadata also keeps the original default-catalog mount oracle unresolved. Earlier Wrath clean stock and Retail exact36unique/42occurrence preservation receipts do not prove this new revision.

## Sources

- [Companion spec](../../specs/wrath-s49-companion-producer-design.md) — scope and acceptance boundaries
- [Companion producer](../../../src/lua_api/globals/real/wrath_legacy/companions.rs) — runtime selection and timer projection
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
| learned_mount_refreshes_original_companion_buttons_inline | Visible 90.25/300.5 → visible 95/400 → hidden idle, plus unchanged row/event/error assertions |
| companion_cooldown_own_timer_seconds_and_expiry | Hidden idle → visible 90.25/300.5 → hidden on-hold → visible same timer → hidden expired |
| companion_cooldown_shared_category_tracks_existing_spell_state | Hidden different category → visible 80.5/250.25 matching category → visible 95/400 own → visible peer again → hidden cleared |
| companion_cooldown_gcd_and_longer_spell_interval | Hidden idle/unclassified/mismatched GCD → visible 99.25/120.5 matching start-recovery category → visible 90.5/400.25 own → visible GCD again → hidden expired |

Each row independently requires hidden and visible states plus timer changes.
Constant idle/nil/no-return and no-op producers cannot create the visible timer;
constant active producers cannot satisfy visibility transitions or distinct
timing pairs. These are static contradictions only; UnitHasRelicSlot still
blocks original panel setup until wrath-relic-slot merges. Existing
cursorOffset/error assertions remain intact.
