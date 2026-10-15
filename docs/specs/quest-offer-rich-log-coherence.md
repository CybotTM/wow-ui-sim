# Quest offer rich-log coherence

Forever public offer acceptance must agree with rich quest-log queries and permit explicit ordinary watch membership. Current acceptance lives in `src/lua_api/globals/quest_verbs.rs`; the observed state split is documented in the [existing quest wiki section](../wiki/systems/lua-api.md#public-offer-rich-log-state-split). This is a bounded simulator contract, not native parity. The user requested offer-data enrichment; the authored ordinary-offer policy below does not claim native quest metadata.

## What it must do

- [ ] After the existing public `A_Admin.OpenMultiQuestNpc()` → `SelectAvailableQuest(1)` → `ConfirmAcceptQuest()` path, preserve one lightweight accepted ID, consume the pending offer and expose the matching accepted quest through rich membership/index/info queries.
- [ ] Preserve the selected offer's supplied identity, title, level, meta flag and current completion in the accepted rich view, consistent with Forever difficulty lookup. Retain the selected snapshot across gossip closure or replacement rather than reading later gossip rows.
- [ ] Model newly authored ordinary offers as not-world, not-failed, nonreplayable and not previously completed, with no item/currency rewards in these scenarios. Preserve existing same-quest rich metadata and recorded completion history where supplied. Carry known same-quest map data: Supply Run's existing authored POI blob supplies map37. Custom offers without known map data remain unmapped. Unknown waypoint, tag, theme and criteria metadata remain absent; do not copy another quest's seed defaults, infer an NPC position from a polygon, or claim native values.
- [ ] After acceptance returns, expose one rich row and one lightweight ID and preserve the existing queued acceptance event. Repeated confirmation without a pending offer is inert; cancellation and separate environments must not gain membership. These assertions do not establish native event timing or internal enqueue ordering.
- [ ] Allow explicit `C_QuestLog.AddQuestWatch` for the accepted ordinary quest, with coherent membership, count and log-index mapping. No automatic-watch policy is required by this fixture.

## How it works

- [Observed state split](../wiki/systems/lua-api.md#public-offer-rich-log-state-split)
- [Existing explicit watch model](forever-quest-difficulty.md#ordinary-watch-membership-follow-up--bounded-model-execution)

## Implementation inventory

- `src/lua_api/globals/admin_quests.rs`: public NPC offer seeds; Retail confirmation extracts only the pending ID before dispatch, preserving its existing prompt contract.
- `src/lua_api/globals/quest_verbs.rs`: consumes the selected snapshot and deduplicates rich/lightweight membership, retaining the existing queued event and already represented rich rows.
- `src/lua_api/globals/quest_surface/legacy_globals.rs`: offer-index selection and public offer metadata.
- `src/lua_api/globals/missing_surface/{gossip_info,quest_log}.rs`: public offer and rich-log queries.
- `src/lua_api/sim_substates/quest_log.rs`: shared selected-offer snapshot builder; retains same-ID metadata or builds explicit ordinary-offer state from supplied gossip and exact-ID POI data.
- `src/lua_api/state/sim_state.rs`: `pending_quest_offer` now owns an existing `QuestLogEntry`, not only its numeric ID; no parallel metadata mirror.
- `data/quest_poi_blobs.rs`: existing exact-ID authored map/area metadata; Supply Run maps to37. Generated `data/quest_ui_map.rs` is a separate projection and is not hand-edited.
- `src/c_api/c_quest_log.rs`: Forever rich-level difficulty lookup.
- `src/c_api/c_quest_log_watches.rs`: explicit membership intersected with represented ordinary rich rows.

## Tests asserting this spec

`tests/wowforever_quest_offer_enrichment.rs`: three additional cases cover selected snapshots across `CloseGossip`/replacement, explicit supplied custom title/level/flags, the modeled absent-metadata/no-reward policy, explicit watching, repeat confirmation, cancellation and environment isolation. Final acceptance assertions are Rust-side. At source `d59551d61`, all three executed: **1 control PASS / 2 missing-rich-row FAIL, exit101** after public preconditions; later metadata/watch assertions remained unreached. Current snapshot/publication producer is implemented but GREEN is not yet verified.

`tests/wowforever_quest_offer_acceptance.rs` and `.lua`: `seeded_public_offer_acceptance_preserves_rich_and_watch_coherence`, Forever-only fresh environment using actual public seed/selection/acceptance, without injected state, API replacement or event callbacks. Final coherence assertion is in Rust, outside Lua handlers. Actual **0 PASS / 1 FAIL, exit101** at source `f94aea875`; public fixture preconditions completed. [Central receipt](../baselines/README.md#public-offer-rich-log-coherence-red--2026-10-15) owns evidence, observations and separate controls.

## Known gaps (current cycle)

- [x] Reproduce the public acceptance/rich-query split at the final Rust coherence assertion.
- [x] User requested enrichment of the simulator-authored offer data; optional unsupported native metadata remains absent under the documented bounded model.
- [x] Execute the three-case snapshot/enrichment baseline: source d595 has1 control PASS /2 `MISSING RICH ROW` FAIL. First sourceedf attempt failed at private-module test imports (E0603), not runtime; public state re-exports fix that fixture error.
- [x] Independently reconcile the new saved baseline through `/home/osso/.local/state/wow-ui-sim/verification/quest-enrich-next/runtime-red-report.md`; failing behavior is not acceptance.
- [ ] Verify the implemented selected-snapshot/publication producer and updated pending-payload control fixtures; no GREEN, cross-profile or startup claim yet.
- [x] Independently reconcile the expected RED and 13 passing existing controls; no commands were rerun.
- [ ] Prove current rich/watch coherence; historical RED and source implementation are not GREEN acceptance.

## Out of scope

This probe does not define confirmation prompts, native validation, automatic watches or native event timing/payloads. The authored offer policy above is simulator data, not a native catalog or arbitrary map/reward assignment. The [Retail confirmation contract](quest-accept-confirmation.md#out-of-scope) explicitly excludes rich projection and is unchanged. Abandonment, completion/turn-in, other profiles and broader lifecycle work remain parent gaps, not newly excluded. Prior sourcef91 watch39/normal GREEN and startup10/14 FAIL remain separate evidence, not acceptance of this missing transition.
