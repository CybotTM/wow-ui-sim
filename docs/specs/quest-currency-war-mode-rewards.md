# War Mode quest currency rewards

Current cached Retail/PTR `QuestUtils.lua` exposes `QuestUtils_AddQuestCurrencyRewardsToTooltip`. Its ordinary currency path consumes separate desired-mode, percentage, quest eligibility, currency bonus and faction-grant inputs. The [supplied currency bonus query](currency-war-mode-bonus.md) is already verified; that does not establish this public caller.

## What it must do

- [ ] Project live supplied desired War Mode and reward percentage as one boolean and one number, independent of generic `player.pvp_enabled`. Initial disabled/10-percent state is chosen simulator policy preserving existing percentage compatibility, not native state/catalog evidence; separate default-percentage getter remains unchanged.
- [ ] Project independently supplied quest eligibility as one boolean; absent/unrepresented input is false (preserves existing normal behavior), not an inference from world-quest classification or a native eligibility catalog.
- [ ] Project supplied currency-to-immediate-reputation faction IDs as one nullable number; no supplied relation means nil, not native unknown-ID truth. No inferred join or normal faction/eligibility membership seed.
- [ ] Required numeric relation identities reject non-number types without coercion; fractional, nonfinite or outside-i32 numeric identities are unrepresented and return false eligibility/nil faction, never truncated aliases. With `retail-12-1-0` secret support, authentication precedes number validation/lookup; wrapper/caller-taint behavior remains subject to verification.
- [ ] Unmodified loaded public QuestUtils ordinary reward handler emits exact currency lines and configured bonus percentage only when every real gate allows it. Cover disabled mode, ineligible/non-world quest, nullable/false currency flags, faction exclusion, repeated currency first-instance limits and independent currencies.
- [ ] Live updates/removal and separate environments preserve state ownership; current Retail/PTR publication/startup and relevant existing query behavior stay coherent without earlier-profile backports or current-profile fallback alternates.

## How it works

- [C API boundary](../../AGENTS.md#c-api-boundary)
- [Current query investigation](../wiki/investigations/patch-8-1-0-api-audit.md#currency-warmode-query--testspec-stage-2026-10-12)

## Implementation inventory

- `tests/quest_currency_war_mode.rs`: initial public query contracts and actual ordinary vendor-tooltip fixture; initial RED1 ordinary-baseline PASS/4 contractFAIL; expanded controls use real input scaffold, actual gate/percentage outputs and duplicate-currency first instances.
- `src/c_api/c_pvp.rs`: existing namespace seam; `war_mode.rs::WarModeState` owns desired/reward inputs and live current-profile getters.
- `src/c_api/item_spell/c_currency.rs`: current-profile faction getter projects the independently supplied relation.
- `src/c_api/c_quest_log.rs`: current-profile eligibility getter; earlier-profile false placeholder remains isolated in `quest_surface/register.rs`.
- `SimState.quest_war_mode_eligibility` and `currency_faction_grants`: empty supplied relations, independent of classification/display fields.

Committed producers `bea55a2ae` (faction/shared numeric helper), `a295a7d72` (PvP) and `067fdbb90` (quest/central registration) publish only for current Retail/PTR. Utility bootstrap registers the quest getter; later `quest_surface` registration cannot overwrite it because its canned false entry is excluded for these profiles. The current canned reward getter is likewise excluded; `GetWarModeRewardBonusDefault` remains 10, and earlier-profile providers remain unchanged. Desired=false/reward=10 and empty relations are independent supplied-state/default policy, not native catalogs or inference from generic PvP/world-quest state.

## Tests asserting this spec

`tests/quest_currency_war_mode.rs`; initial five-case RED1PASS/4FAIL remains separate. Expanded pre-producer RED executed at `/home/osso/.local/state/wow-ui-sim/verification/quest-currency-war-mode-current/positive-red-compile-20261012T091819552684Z`: compile exit0; runtime13 cases,1PASS/12FAIL, exit101,2.15s. Tests pinned to `9e3fd32551e67e06653e6224e94be8f56a6da33b`; docs HEAD `8a10302064ed91b8f0944eef475871902b311bf0`. First independent authentication saved matching467982416 bytes, digest `4e6d1f18bec3b6c186d9380e128c8a1881fe45165034d9bdb5d22f4e70dd0dbf`; reused without artifact reread or rehash. Main terminal failed exit101/MainPID0. Ordinary vendor caller baseline is the sole PASS; supplied-input contracts and positive bonus controls fail. Later gates/duplicate downstream assertions were not reached, not independently falsified.

## Known gaps (current cycle)

- [x] Expanded pre-producer RED executed and independently authenticated; initial five-case RED retained separately.
- [ ] Current-profile GREEN async pending after committed producers; startup/publication and independent Rust/readability/retention proof remain pending. No GREEN, native parity or full handoff closure claim.

## Out of scope

Native eligible-quest/currency/faction catalogs, active War Mode/phase/toggle/setter lifecycle, currency-container reward path, XP/money/other quest reward paths and earlier-profile semantics. They are distinct contracts; no mocked getter, vendor rewrite or native-parity claim substitutes for this ordinary currency caller boundary. Parent full handoff remains OPEN.
