# Wrath companion cooldown and creature-ID source review

GetCompanionCooldown is staged for the selected legacy companion consumer, using existing simulator cooldown state. Verified source inspection: 2026-10-24. This edit-only cycle has no compilation or executed RED/GREEN evidence.

## Selected consumer and staged behavior

The prior lane report binds FrameXML.toc → PetPaperDollFrame.xml → PetPaperDollFrame.lua with no flavor/exclusion gates; its populated original button reaches line429, calling GetCompanionCooldown. The adapter registers only under runtime `client_profile::ACTIVE == Wrath` and chooses the same owned/eligible sorted mount spell. Creature-name metadata is not required by cooldown lookup.

Accepted laptop request11 Q5 gives seconds for start/duration, numeric enable, idle `(0,0,1)`, own cooldown, shared-category and start-recovery/GCD paths. The implementation reuses `SimState.spell_cooldowns`, `spell_cooldown_categories`, `gcd` and `action_bar_api::spell_cooldown_times`; category peers contribute their latest-ending active interval. No parallel cooldown store or vendor patch. Three staged selectors each change the relevant host state and assert idle plus distinct non-idle output, independently rejecting a constant/no-op contract. Execution and type checking remain pending.

The existing state lacks native shared-category flags, separate category intervals, per-spell GCD eligibility, excluded-effect fields and on-hold/disabled enable state. It also has no companion active-player absence projection. Native helper parity is therefore incomplete; modeled timers return numeric enable1. Critter spell joins remain unsupplied. Decimal numeric-string index conversion is supported; complete native numeric coercion is not claimed.

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
- Accepted `/home/osso/.local/state/wow-ui-sim/handoff/laptop-labels-answers-20261018/laptop-answers-req-11.md`, Q2–Q5 — laptop clean-room evidence; no decompiled client source read in this lane
- `/home/osso/.local/state/wow-ui-sim/handoff/wrath-s49-companion/REPORT.md` and `slot2-release-main.json` — prior consumer proof and released slot

## See Also

- [[consumer-selection-and-red-proof]] — selected consumers and standalone rejection
- [[blizzard-ui-test-lanes]] — original addon-bootstrap consumer tests
