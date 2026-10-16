# Quest offer rich-log coherence

Forever public offer acceptance must agree with rich quest-log queries and permit explicit ordinary watch membership. Current acceptance lives in `src/lua_api/globals/quest_verbs.rs`; the observed state split is documented in the [existing quest wiki section](../wiki/systems/lua-api.md#public-offer-rich-log-state-split). This is a bounded simulator contract, not native parity. The user requested offer-data enrichment; the authored ordinary-offer policy below does not claim native quest metadata.

## What it must do

- [x] After the existing public `A_Admin.OpenMultiQuestNpc()` → `SelectAvailableQuest(1)` → `ConfirmAcceptQuest()` path, preserve one lightweight accepted ID, consume the pending offer and expose the matching accepted quest through rich membership/index/info queries.
- [x] For newly accepted rich entries, preserve the selected offer's supplied identity, title, level, meta flag and current completion, consistent with Forever difficulty lookup. Already represented rich rows are not duplicated or overwritten. Retain the selected snapshot across gossip closure or replacement rather than reading later gossip rows.
- [x] Model newly authored ordinary offers as not-world, not-failed, nonreplayable and not previously completed, with no item/currency rewards in these scenarios. Preserve existing same-quest rich metadata and recorded completion history where supplied. Carry known same-quest map data: Supply Run's existing authored POI blob supplies map37. Custom offers without known map data remain unmapped. Unknown waypoint, tag, theme and criteria metadata remain absent; do not copy another quest's seed defaults, infer an NPC position from a polygon, or claim native values.
- [x] After acceptance returns, expose one rich row and one lightweight ID and preserve the existing queued acceptance event. Repeated confirmation without a pending offer is inert; cancellation and separate environments must not gain membership. These assertions do not establish native event timing or internal enqueue ordering.
- [x] Allow explicit `C_QuestLog.AddQuestWatch` for the accepted ordinary quest, with coherent membership, count and log-index mapping. No automatic-watch policy is required by this fixture.
- [ ] Simulator abandonment removes target lightweight/rich active membership and ordinary watch membership/metadata, preserving unrelated active/watch membership and recorded completion history.

## How it works

- [Observed state split](../wiki/systems/lua-api.md#public-offer-rich-log-state-split)
- [Existing explicit watch model](forever-quest-difficulty.md#ordinary-watch-membership-follow-up--bounded-model-execution)

## Implementation inventory

Producer `ccd9e2d01` is committed; main-observed Forever execution at source `53cf24ab0` passes all43 selected cases, exit0. Independent saved-runtime reconciliation passes in `quest-enrich-next/green-runtime-report.md` under host-private verification state. Selection updates supplied fields on a cloned pending snapshot. Acceptance inserts that snapshot only for an absent rich ID; an already represented rich row remains untouched, preserving existing metadata and rewards.

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

`tests/quest_abandon_membership.rs`: **UNCOMPILED / UNEXECUTED**, not behavioral RED or acceptance. Publicly offers, accepts and explicitly watches target80002 and unrelated80000, then abandons80002. Assertions cover target lightweight/rich/watch removal, unrelated membership and unchanged internal/public completion history. Forever-gated because modeled watch mutation is Forever-only. Existing accepted receipts do not cover this new case.

`tests/wowforever_quest_offer_enrichment.rs`: three additional cases cover selected snapshots across `CloseGossip`/replacement, explicit supplied custom title/level/flags, the modeled absent-metadata/no-reward policy, explicit watching, repeat confirmation, cancellation and environment isolation. Final acceptance assertions are Rust-side. At source `d59551d61`, all three executed: **1 control PASS / 2 missing-rich-row FAIL, exit101** after public preconditions; later metadata/watch assertions remained unreached. Current source `53cf24ab0` executes all three enrichment cases PASS; see the bounded current receipt below.

`tests/wowforever_quest_offer_existing.rs`: additional **executed PASS at source53cf24ab0** preservation control reopens an already represented quest and verifies its metadata, same-quest reward and watch count are not replaced or duplicated by ordinary-offer initialization.

`tests/wowforever_quest_offer_acceptance.rs` and `.lua`: `seeded_public_offer_acceptance_preserves_rich_and_watch_coherence`, Forever-only fresh environment using actual public seed/selection/acceptance, without injected state, API replacement or event callbacks. Final coherence assertion is in Rust, outside Lua handlers. Actual **0 PASS / 1 FAIL, exit101** at source `f94aea875`; public fixture preconditions completed. [Central receipt](../baselines/README.md#public-offer-rich-log-coherence-red--2026-10-15) owns evidence, observations and separate controls.

### Current bounded execution — 2026-10-15

[Central receipt](../baselines/README.md#public-offer-rich-log-coherence-red--2026-10-15) owns saved evidence at source `53cf24ab0`: **43/43 PASS, exit0** observed by main: enrichment3 + existing-row preservation1 + original acceptance1 + verbs13 + gossip13 + watch membership8 + projection4. These execute selected snapshots across close/replacement, cancellation/isolation/repeat confirmation, represented metadata/reward preservation and explicit watchability. Exact simulator mapping is80002→37; custom map remainsNone. The five acceptance/enrichment requirements above are complete only for these executed boundaries, not broader lifecycle or native behavior. Independent saved-runtime reconciliation confirms all43 unique identities and full streams; build/seal reconciliation is separate. Separate current source `a56e11267` default-Retail preservation is independently accepted: **31 unique PASS, exit0** (confirmation5 + verbs13 + gossip13), harness16.50s. The [Retail receipt](../baselines/README.md#retail-quest-preservation--2026-10-15) owns retry1 evidence, actual emitted profile, successful guarded12m13s no-run build, seal digest and independent build/runtime reports. This accepts current pending-snapshot prompt preservation at real synchronous listeners, not Retail rich/watch coverage or native behavior. Existing a56 formatting PASS and bounded quest-source default check/startup gates are accepted in the [central gate receipt](../baselines/README.md#quest-source-check-and-startup-gates--2026-10-15). Later C_UIWidgetManager changes are not covered by these earlier quest-source check/startup/fmt proofs. Subsequent [DoubleIconAndText bounded execution](ui-widget-double-icon-and-text.md) has its own receipt; no whole-current-HEAD startup pass follows. First204301Z host-PSI guard SIGTERM abort has no compiler/test verdict; historical Gamepad10 records/14 occurrences is unchanged, not rerun.

## Known gaps (current cycle)

- [x] Reproduce the public acceptance/rich-query split at the final Rust coherence assertion.
- [x] User requested enrichment of the simulator-authored offer data; optional unsupported native metadata remains absent under the documented bounded model.
- [x] Execute the three-case snapshot/enrichment baseline: source d595 has1 control PASS /2 `MISSING RICH ROW` FAIL. First sourceedf attempt failed at private-module test imports (E0603), not runtime; public state re-exports fix that fixture error.
- [x] Independently reconcile the new saved baseline through `/home/osso/.local/state/wow-ui-sim/verification/quest-enrich-next/runtime-red-report.md`; failing behavior is not acceptance.
- [x] Independently reconcile Forever source53cf24ab0 selected-snapshot/publication and pending-payload controls:43/43 PASS, exit0, full streams and exact identities verified.
- [x] Independently reconcile the expected RED and 13 passing existing controls; no commands were rerun.
- [x] Prove current rich/watch coherence at the executed Forever boundaries; historical RED remains retained and does not count as a current pass.
- [x] Independently reconcile current a56 Retail confirmation/pending-payload preservation:31 unique PASS, exit0; separate from Forever43 at source53cf.
- [x] Close bounded quest-source default check/startup gates from independently audited saved receipts; see [source scope and exclusions](../baselines/README.md#quest-source-check-and-startup-gates--2026-10-15).
- [ ] Reconcile later C_UIWidgetManager changes with current check/startup coverage; [targeted DoubleIconAndText proof](ui-widget-double-icon-and-text.md) does not extend these earlier quest-source gates or establish whole-current-HEAD/native/full-parent acceptance.

## Out of scope

This probe does not define confirmation prompts, native validation, automatic watches or native event timing/payloads. The authored offer policy above is simulator data, not a native catalog or arbitrary map/reward assignment. The [Retail confirmation contract](quest-accept-confirmation.md#out-of-scope) explicitly excludes rich projection and is unchanged. The abandonment membership/history requirement above remains unverified; it does not define event, selection, idempotence or reacceptance policies. Completion/turn-in, other profiles and broader lifecycle work remain parent gaps, not newly excluded. Prior sourcef91 watch39/normal GREEN and startup10/14 FAIL remain separate evidence, not acceptance of this missing transition.
