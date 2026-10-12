# War Mode quest currency rewards

Current cached Retail/PTR `QuestUtils.lua` exposes `QuestUtils_AddQuestCurrencyRewardsToTooltip`. Its ordinary currency path consumes separate desired-mode, percentage, quest eligibility, currency bonus and faction-grant inputs. The [supplied currency bonus query](currency-war-mode-bonus.md) is already verified; that does not establish this public caller.

## What it must do

- [ ] Project live supplied desired War Mode and reward percentage as one boolean and one number, independent of generic `player.pvp_enabled`. Initial disabled/10-percent state is chosen simulator policy preserving existing percentage compatibility, not native state/catalog evidence; separate default-percentage getter remains unchanged.
- [ ] Project independently supplied quest eligibility as one boolean; absent/unrepresented input is false (preserves existing normal behavior), not an inference from world-quest classification or a native eligibility catalog.
- [ ] Project supplied currency-to-immediate-reputation faction IDs as one nullable number; no supplied relation means nil, not native unknown-ID truth. No inferred join or normal faction/eligibility membership seed.
- [ ] Required numeric relation identities reject wrong types without coercion or lossy aliasing; secret inputs authenticate before validation/lookup and preserve wrappers/caller taint according to current declarations.
- [ ] Unmodified loaded public QuestUtils ordinary reward handler emits exact currency lines and configured bonus percentage only when every real gate allows it. Cover disabled mode, ineligible/non-world quest, nullable/false currency flags, faction exclusion, repeated currency first-instance limits and independent currencies.
- [ ] Live updates/removal and separate environments preserve state ownership; current Retail/PTR publication/startup and relevant existing query behavior stay coherent without earlier-profile backports or current-profile fallback alternates.

## How it works

- [C API boundary](../../AGENTS.md#c-api-boundary)
- [Current query investigation](../wiki/investigations/patch-8-1-0-api-audit.md#currency-warmode-query--testspec-stage-2026-10-12)

## Implementation inventory

- `tests/quest_currency_war_mode.rs`: initial public query contracts and actual ordinary vendor-tooltip fixture; initial RED observed1 ordinary-baseline PASS/4 contractFAIL; positive supplied-input controls follow real owned scaffold.
- `src/c_api/c_pvp.rs`: existing namespace seam; `war_mode.rs::WarModeState` owns desired/reward inputs, getters not yet implemented.
- `src/c_api/item_spell/c_currency.rs`: existing currency query seam; faction relation not yet modeled.
- `src/lua_api/globals/quest_surface/register.rs`: existing false eligibility placeholder; no independent eligibility getter yet.
- `SimState.quest_war_mode_eligibility` and `currency_faction_grants`: empty supplied relations, independent of classification/display fields.

## Tests asserting this spec

`tests/quest_currency_war_mode.rs`; initial RED1PASS/4FAIL before owned scaffold; ordinary vendor baseline passes, positive supplied caller and new getters remain unproved.

## Known gaps (current cycle)

- [ ] Initial RED independent authentication; expanded positive RED after real input scaffold; producers, current-profile GREEN, startup/publication and independent Rust/readability/retention proof. Ordinary vendor baseline1PASS/4contractFAIL retained.

## Out of scope

Native eligible-quest/currency/faction catalogs, active War Mode/phase/toggle/setter lifecycle, currency-container reward path, XP/money/other quest reward paths and earlier-profile semantics. They are distinct contracts; no mocked getter, vendor rewrite or native-parity claim substitutes for this ordinary currency caller boundary. Parent full handoff remains OPEN.
