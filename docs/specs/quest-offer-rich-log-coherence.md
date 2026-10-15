# Quest offer rich-log coherence

Forever public offer acceptance must agree with rich quest-log queries and permit explicit ordinary watch membership. Current acceptance lives in `src/lua_api/globals/quest_verbs.rs`; the observed state split is documented in the [existing quest wiki section](../wiki/systems/lua-api.md#public-offer-rich-log-state-split). This is a failing bounded simulator contract, not native parity or an approved rich-metadata default policy.

## What it must do

- [ ] After the existing public `A_Admin.OpenMultiQuestNpc()` → `SelectAvailableQuest(1)` → `ConfirmAcceptQuest()` path, preserve one lightweight accepted ID, consume the pending offer and expose the matching accepted quest through rich membership/index/info queries.
- [ ] Preserve the supplied offer identity, title and level in the accepted rich view, consistent with Forever difficulty lookup. Additional rich metadata policy remains a user-owned pending decision; do not invent defaults.
- [ ] Allow explicit `C_QuestLog.AddQuestWatch` for the accepted ordinary quest, with coherent membership, count and log-index mapping. No automatic-watch policy is required by this fixture.

## How it works

- [Observed state split](../wiki/systems/lua-api.md#public-offer-rich-log-state-split)
- [Existing explicit watch model](forever-quest-difficulty.md#ordinary-watch-membership-follow-up--bounded-model-execution)

## Implementation inventory

- `src/lua_api/globals/admin_quests.rs`: existing public multi-NPC offer seed.
- `src/lua_api/globals/quest_verbs.rs`: existing pending-offer consumption and lightweight-log append; no rich projection added.
- `src/lua_api/globals/quest_surface/legacy_globals.rs`: offer-index selection and public offer metadata.
- `src/lua_api/globals/missing_surface/{gossip_info,quest_log}.rs`: public offer and rich-log queries.
- `src/lua_api/sim_substates/quest_log.rs`: rich state representation.
- `src/c_api/c_quest_log.rs`: Forever rich-level difficulty lookup.
- `src/c_api/c_quest_log_watches.rs`: explicit membership intersected with represented ordinary rich rows.

## Tests asserting this spec

`tests/wowforever_quest_offer_acceptance.rs` and `.lua`: `seeded_public_offer_acceptance_preserves_rich_and_watch_coherence`, Forever-only fresh environment using actual public seed/selection/acceptance, without injected state, API replacement or event callbacks. Final coherence assertion is in Rust, outside Lua handlers. Actual **0 PASS / 1 FAIL, exit101** at source `f94aea875`; public fixture preconditions completed. [Central receipt](../baselines/README.md#public-offer-rich-log-coherence-red--2026-10-15) owns evidence, observations and separate controls.

## Known gaps (current cycle)

- [x] Reproduce the public acceptance/rich-query split at the final Rust coherence assertion.
- [ ] Resolve pending user rich-metadata choice before production implementation; no defaults authorized here.
- [x] Independently reconcile the expected RED and 13 passing existing controls; no commands were rerun.
- [ ] Implement and prove rich/watch coherence; current RED is not acceptance.

## Out of scope

This probe does not define confirmation prompts, native validation, automatic watches, native event timing/payloads or rich-only field defaults. The [Retail confirmation contract](quest-accept-confirmation.md#out-of-scope) explicitly excludes rich projection and is unchanged. Abandonment, completion/turn-in, other profiles and broader lifecycle work remain parent gaps, not newly excluded. Prior sourcef91 watch39/normal GREEN and startup10/14 FAIL remain separate evidence, not acceptance of this missing transition.
