# Profile Baselines

Validation artifacts for supported client profiles.

## Lua Errors

`mists-lua-errors.json` is the committed clean startup baseline for:

```bash
cargo build --no-default-features --features "sound,gui,casc,client-mists" --bin wow-sim
WOW_SIM_NO_SAVED_VARS=1 WOW_SIM_NO_ADDONS=1 timeout 90 ./target/debug/wow-sim lua-errors > docs/baselines/mists-lua-errors.json
```

Do not refresh this file to bless new startup errors. Refresh only after a
verified clean Mists capture.

Verified scoped capture on 2026-10-16: **0 records / 0 occurrences, exit0**;
142 Blizzard addons loaded. Sealed Mists source `4d5572f241cce3d99a7b486709186a2638566a56`
includes loader fix `480b9f934`; both historical self-anchor records (2 occurrences)
are absent. Evidence epoch: `/home/osso/.local/state/wow-ui-sim/handoff/mists-startup-confirm/capture-20261016T203207134694Z`.
Capture used `--no-addons --no-saved-vars lua-errors`, `WOW_SIM_CASC=0`,
`WOW_SIM_NO_SOUND=1`, and a byte-exact child-only copy of current CVars;
original CVar stat/hash remained unchanged. Runtime limits: 4GiB, CPU100%, 86s.
This replaces the stale two-record JSON, not a native-parity, current-HEAD,
GUI, full-suite, or all-profile acceptance. External cache inputs were not fully sealed.

Re-confirmed on 2026-10-16 at `039026c8c` (master after the EditBox cursor
policy candidate C merge): **0 records / 0 occurrences, exit0**. Between
`c8d79f3bf` and that merge, master's Mists startup emitted the StoreUISecure
`OnCursorChanged` pair (2 records), so the `[]` file was stale for that range.
Independent verifier evidence: `/home/osso/.local/state/wow-ui-sim/verification/mists-store-final/report.md`.

Use `scripts/diff-lua-errors.sh BASELINE NEW` to compare message sets and
report regressed/fixed errors. The Mists CI guard uses this for addon-induced
error counts and release proof diagnostics.

## Era / Anniversary — current baseline, 2026-10-17

At `781a26fef` (Classic B merged on top of Classic A and Retail 70448), both profiles report **24 records / 56 occurrences, exit1, FAIL**. The JSON is the byte-exact stdout captured by the rebase gates; each `.meta.json` holds the details. Open items:
- R20: `ShouldKnowUnitHealth` at `UnitFrame.lua:737` is missing. The producer is held for a user decision, and its tests are off master. The `CombatFeedback` nil record is downstream of R20.
- The skill and timer pairs are blocked on R10/R14, which depend on wrath-fixes.

Candidate C removed the StoreUI cursor pair. The earlier counts (92/98 below, then 36/38 after Classic A) are historical.

## Era / Anniversary — actual acquisition and startup, 2026-10-16

| Profile | Own cache | Startup lua-errors |
|---|---:|---|
| Anniversary (existing Vanilla profile) | 2,971/2,971 PASS | exit1, 92 records / 98 occurrences, FAIL |
| Era | 2,971/2,971 PASS | exit1, 92 records / 98 occurrences, FAIL |

Build/runtime source `b1e749ee3758c0e576b5cc1bf00e09211dc3c852` is base `70ebaee0e` plus authorized cherry-picks of manifest commit `0d244c899` and listfile commit `b036938ae`. Both profiles have distinct CASC-derived enUS caches and provenance for installed `wow_classic_era` build `1.15.9.70003`; manifest SHA-256 `6647aded693f013c7ed4e1e1e613450558a781da1bf8a0494d8887e1f7810978`. Cache file hashes remained stable across captures. Baseline JSON files are byte-exact stdout, independently executed and byte-identical; they are not profile substitutions.

Observed prerequisite failures include nil table keys in Classic Constants/UnitPopup/CombatLogColors, missing `NUM_LE_ITEM_QUALITYS` in Vanilla UIParent.lua:25, nil casting-bar `classicFillColor`/`classicFullColor`, and missing mixin globals. Other nil API/data callbacks remain unisolated. Records include reporter-wrapped forms of the same errors; 92 records do not mean 92 root causes. No fixes made. Original persisted CVars remained unchanged in every recorded run. Compiler warnings remain; not warning-free or clean-startup acceptance.

Anniversary bare controls separately passed 39/39 at `70ebaee0e`; no claim that they were rerun at this data-only revision. Existing Anniversary profile targets Vanilla, not Gethe current `classic_anniversary` TBC 2.5.6. Native parity, GUI and full-suite acceptance remain unproven. Metadata files bind seals, argv, source, provenance and capture hashes. These are failing compatibility inventories, not clean baselines or startup acceptance at current master.

Files: [Era JSON](era-lua-errors.json), [Era metadata](era-lua-errors.meta.json), [Anniversary JSON](anniversary-lua-errors.json), [Anniversary metadata](anniversary-lua-errors.meta.json). The clean-capture-only refresh rule above applies to Mists; these initial inventories record actual failures without blessing them.

Only Era/Anniversary source manifests changed. No existing mapped path/FDID was lost from any remaining profile. Wrath mapping coverage improves from 57/382 to 103/382 (325 to 279 unmapped); it is not unchanged. Wrath UI acquisition uses its separate pinned source snapshot (`src/blizzard_ui_sync.rs::sync_wrath_ui_to`), not those added CASC UI mappings. The other five source manifests are unchanged. This audit proves mapping preservation, not fresh runtime acceptance for those profiles.

## Wrath Lua Errors

`wrath-lua-errors.json` refreshes startup after bounded guard retirement at `4e58e23a1`: **44 unique errors / 55 occurrences, exit1**, versus historical `c06c84362` **45 / 89**. This remains a failing compatibility inventory, not a clean baseline. The original `UIPanelTemplates.lua:365` nil-`cursorOffset` failure is absent: **0 cursorOffset/ScrollingEdit records or occurrences**, previously 34 occurrences. Exact message comparison finds 8 added / 9 removed records; 8 pairs differ only in generated chunk hashes (including backtraces). Ignoring those identities for comparison only leaves **0 substantive new errors / 1 resolved error (34 occurrences)**; retained error counts are unchanged.

`wrath-lua-errors.meta.json` binds this observation to source, manifest hash, profile, command/environment and private sealed-artifact receipts. Absolute user-cache prefixes are replaced with `<wrath-ui>/`; messages and counts otherwise remain intact. The current external UI cache was consumed, not independently sealed or authenticated. Original persisted CVars were unchanged across isolated execution. See the [profile owner](../specs/client-profiles.md#actual-wrath-publication--2026-10-16) and [guard-retirement owner](../specs/editbox-text-position-selection.md#wrath-nil-field-guard-retirement--accepted-bounded). No native parity, causal isolation of other source changes, all-profile proof or full-startup acceptance.

## Forever Lua Errors

Current saved corrected-enUS `1.60.1.70374` capture at runtime/build source `40a010b5a`: **10 records / 14 occurrences, exit1**, still FAIL; all remaining records are Gamepad storage/empty-map errors, first nil player-storage arithmetic. The missing rich quest80002 error is absent after watch/log reconciliation. Historical source704's12/16 JSON remains privately retained as `baseline-before-watch.json` beside the current watch receipt; query coherence is not whole-startup acceptance. Historical c2bf/build57 capture **22 records / 26 occurrences, exit1**, first nil `Enum.VoiceProviderID`, remains privately retained beside current receipts. Source136 post-enum capture **58 records / 58 occurrences, exit1**, first failure missing `import`, remains privately retained. Baseline JSON copying belongs to main; this documentation update does not rewrite or authenticate the tracked JSON. Historical source `9a6ea924e` capture was **166 records / 238 occurrences, exit1**. Preset line1272 and setting-display-map cascade no longer appear; other preset errors remain. Counts are unique records/occurrences, not root-cause counts.

The two historical captures (source136 and `9a6ea924e`) report 272 Blizzard addons loaded and 1658/1658 bytecode-cache hits. Different explicit CASC/product environment settings and unresealed external cache inputs prevent whole-input equivalence, archive acceptance or controlled performance claims. See [verified receipts](#verified-scoped-receipts--2026-10-15) for attribution and the [locale owner](../specs/casc-loading.md#locale-checkpoint--october-15-2026) for acquisition scope. Clean startup remains open.

Historical pre-70374 `[]` remains private as `wowforever-lua-errors.pre-70374.json` beside the earlier receipts. The earlier 4,398-file, 422-record/514-occurrence capture and [selected69977 checkpoint](../updating-blizzard-ui-to-a-new-patch.md#historical-selected-target--september-24-2026) retain their historical identities.

## Verified scoped receipts — 2026-10-15

The historical model/GPU outcomes below bind runtime source `136748f0b3e122b6c9f897c69db19dcf1b317c5f`, before import. Current import/event execution is recorded separately below; two earlier host-memory guard aborts were not compiler errors. Independent saved-evidence reports: `/home/osso/.local/state/wow-ui-sim/verification/feedback-models-current/report.md` and `/home/osso/.local/state/wow-ui-sim/verification/feedback-gpu-startup-current/report.md`. Evidence root E: `/home/osso/.local/state/wow-ui-sim/verification/widget-discrete-progress-current/feedback-forever-integrated-tests-20261015T101651047924Z`. Reports reuse saved seals and matching stat metadata, not fresh artifact hashes or enforced immutability.

| Component | Actual bounded proof |
|---|---|
| [Locale/cache provenance](../specs/casc-loading.md) | Selector 5 + provenance 1 PASS |
| [Rotated tiling](../specs/rotated-tiled-edge-prefix.md) | 8 model tests PASS |
| [Slider](../specs/slider-bound-thumb-layout.md) | 18 model tests PASS |
| [Forever enums](../specs/forever-edit-mode-enums.md) | 5 model/real cached-consumer tests PASS |
| [Profile identity](../specs/client-profiles.md) | Manifest 1 + BuildInfo identity 1 PASS |

These are 39 distinct outcomes. A separate exact [Slider render-list regression](../specs/slider-bound-thumb-layout.md#tests-asserting-this-spec) passed 1/1 on the same source, independently verified from saved evidence; 40 model outcomes total, not full-suite execution. Separate production GPU test explicitly executed with `--ignored --exact`: 1 PASS, 0 ignored; all four controls passed (solid, texture roundtrip, half-alpha over black, additive), actual Vulkan NVIDIA RTX 5060 Ti, `Rgba8UnormSrgb`. Mapped RGBA respectively `[124,170,203,255]`, `[51,102,153,255]`, `[89,124,149,255]`, `[124,170,203,255]`. This is shader/attachment readback, not native parity or all-GPU/full-UI acceptance.

Historical existing require/sandbox/registry/path tests: 12 + 6 + 14 + 16 = 48 distinct PASS **before the new import binding**; not acceptance of `012df1e45`. Historical pre-binding diagnostic successfully completed `require` of Utility as a table, then failed `IMPORT_BINDING_RED` with `import=nil`; process exit0 does not make this assertion PASS.

Compilation and saved fmt passed for the scoped source/profile; UnitRole dead-code warnings remain. Later [default-profile compiler preservation](#default-profile-compiler-preservation--2026-10-15) is independently accepted; full suites, clean startup, native/cross-profile and full parent acceptance remain open. No new build/runtime execution for this documentation update.

### Current import/events execution — 2026-10-15

Evidence root E: `/home/osso/.local/state/wow-ui-sim/verification/widget-discrete-progress-current/feedback-forever-import-events-retry2-20261015T121651105820Z`. Actual runtime-code basis `c2bf42c4f` includes import `012df1e45`; build source `57f8e0b106b6581bb65d0d14eac97944c5c11b57`. Successful no-run compilation followed two guard aborts; four sealed binaries are lib-tests, integration, wow-cli and wow-sim. Actual inventories: **1816 library / 8406 integration** tests, not executed-suite totals. UnitRole dead-code warnings remain.

`current-model-results.json` records require14 + sandbox7 + registry14 + path16 + Forever events6 = **57 distinct PASS**, all group exits0. Seven new functions versus old source; source136's 48 backend controls are historical, not additional current passes. Source136's separate 39 model cases, one Slider render-list case and GPU readback remain separate evidence, not rerun here.

Normal probe stderr contains `FOREVER_IMPORT_CURRENT_GREEN completed-table-identity vendor-method-call context-type=nil` and `FOREVER_EVENTS_CURRENT_GREEN count=5`: completed Utility table identity, callable vendor `SmartNavigation:GetCurrentButtonContext` returning nil, and five finite-event registration lifecycles completed; no exec-Lua errors. This does not establish functioning gamepad roots. Actual new gamepad root failures remain open. Normal startup remains FAIL as recorded above; the external full cache was not independently resealed.

Independent report `/home/osso/.local/state/wow-ui-sim/verification/forever-import-events-runtime-next/report.md` reconciles compilation, seals, inventories, all 57 distinct passes and the positive probe: **bounded PASS; startup FAIL**. This does not establish full-suite coverage, native import/event producer or payload parity, cross-profile runtime proof, clean startup or full-parent acceptance. The [provider/experience enum follow-up](../specs/forever-finite-constants.md#provider-and-experience-enums--source-checkpoint-2026-10-15) is now source-integrated after this receipt; its new bounded execution is recorded below. These 57 passes do not cover that follow-up. No builds or runtime execution were performed by this documentation reconciliation.

### Current provider/quest execution — 2026-10-15

Evidence root E: `/home/osso/.local/state/wow-ui-sim/verification/widget-discrete-progress-current/forever-provider-quest-green-20261015T131745696813Z`. Captured runtime/build source `70459ec5a`, not current HEAD; later talent-fixture/watch changes have no new binaries yet. Guarded no-run compilation succeeded in17m25s, no compiler errors or safety abort; terminal-observed MemoryPeak7,264,423,936 bytes (`terminal-command-observation.json`, recovered history copy, not a new measurement). The separate sampled memory.current maximum is7,255,912,448 bytes. Four emitted artifacts sealed; integration inventory8412 is not suite execution. Current `cargo fmt --check` passed; retained UnitRole warnings preclude warning-free claims.

Current executed matrix: [quest model](../specs/forever-quest-difficulty.md) **3 PASS**; finite constants **19 PASS / 2 FAIL** (21 selected), including all three new provider/experience cases PASS. **22 distinct current successes / 2 failures**, not a GREEN group. Both older failures reproduce against the original c2bf sealed binary: missing `ProfessionMicroButtonMixin` during whole talent-override load and an assertion in the stance-mapping fixture. Those counterchecks are not new producer successes. A separate unchanged stance case then PASSed1/1 from an additional byte-identical `artifacts/deps/` copy: relocation had disabled `CVarStorage`'s existing test-path isolation. Original artifacts/seals and19/2 execution remain retained; the stance symptom is artifact context, not a production defect or permission to reset user CVars. This adds one separately executed current identity, not a newly executed20/1 group. Talent fixture commit `1bed3b904` supplies the two missing mixin containers; its GREEN is pending.

Four serialized normal probes have their required markers and no exec-Lua error: unchanged raw-enum regression, unchanged four-known-row quest coherence, unchanged whole quest-consumer fixture/assertions plus an appended success marker, and actual `VOICE_WINDOW_ID=ChatFrame3:GetID()=3`. Raw/lookup provider and experience getters remain callable but return nil. This verifies enum-driven initialization, not modeled provider/preset state. Whole quest-consumer dependencies remain explicit fixtures, not production behavior credit.

Actual startup `post-provider-quest-startup/stdout` is copied byte-for-byte into `wowforever-lua-errors.json`: **12 records / 16 occurrences, exit1**. Previous22/26 JSON is retained as `baseline-before-provider-quest.json`; `baseline-copy-receipt.json` records both hashes. Gamepad storage/empty-map failures and missing rich quest80002 remain. Default/profile checks, native parity, full suites and full-parent acceptance remain open. Independent bounded saved-runtime reconciliation: `/home/osso/.local/state/wow-ui-sim/verification/provider-quest-runtime-next/report.md`, including the append-only stance-path falsification. Later watch producer `9df7149e2` has original watch/log Lua RED and four authored UNEXECUTED tests; its [owning contract](../specs/forever-quest-difficulty.md#ordinary-watch-coherence-follow-up) preserves broader watch/query parent gaps.

### Current watch/talent execution — 2026-10-15

Evidence root E: `/home/osso/.local/state/wow-ui-sim/verification/widget-discrete-progress-current/forever-watch-talent-green-20261015T142714606242Z`. Runtime/build source `40a010b5a`: fair FIFO admission, guarded no-run compilation9m31s, exit0, no compiler errors/safety abort; actual terminal MemoryPeak6,789,902,336 bytes in `terminal-command.json`. Three emitted artifacts sealed; integration remains under `artifacts/deps/` for existing CVar test isolation. Inventory8416 is listing only; saved current formatting passed. UnitRole warnings remain, so no warning-free claim.

Executed **28 distinct PASS / 0 FAIL / 0 ignored**: ordinary-watch projection4, getter preservation3, finite constants21. This includes the corrected whole-Camelot talent fixture and unchanged stance mapping under proper artifact isolation. Earlier704 finite19/2 and its exact old-binary counterchecks remain retained, not rewritten. The [quest contract](../specs/forever-quest-difficulty.md#evidence-boundary) separates this historical all-ordinary projection from current explicit membership; this is not watch-subset parity.

Normal `current-watch-voice-probe` has both required GREEN markers and no exec-Lua error. Original watched-ID/log/difficulty assertions are an unchanged byte prefix; added checks cover the global count alias and actual `VOICE_WINDOW_ID=ChatFrame3:GetID()=3`. Actual startup `post-watch-startup/stdout` is the exact committed JSON: **10 records / 14 occurrences, exit1**. Previous12/16 bytes and both hashes are retained in `baseline-before-watch.json` / `baseline-copy-receipt.json`. All remaining startup records are Gamepad errors; no clean/native/full-profile claim.

A separate subsequent `watch-membership-missing-red` preserves primitive observations and fails its final assertion despite process0: removing/adding watched80000 returns zero values, count remains2, watch-type lookup returns one nil, and no watch-list events arrive. This proved the stateful add/remove/type/event path was missing at source40a; its query projection did not close it. No fallback, invented rich quest or CVar reset was introduced. Independent saved-runtime reconciliation is pending. Later [default-profile compiler preservation](#default-profile-compiler-preservation--2026-10-15) is independently accepted; other-profile checks, full suites, native parity, Gamepad policy and full-parent acceptance remain open.

### Current watch membership model execution — 2026-10-15

Evidence root E: `/home/osso/.local/state/wow-ui-sim/verification/widget-discrete-progress-current/forever-watch-membership-green-retry1-20261015T154858468871Z`. Runtime/build source `f91eb7aab640f7c19182355cbc169ee636948190`, profile `client-wowforever`. Saved no-run compilation succeeded in4m38s, terminal exit0, MemoryPeak4,304,048,128 bytes. `build-summary.json`, `terminal-command.json` and three `seal-*.json` consistently attribute the emitted integration, wow-cli and wow-sim artifacts to this source/profile; normal binaries also retain Cargo fingerprint/hardlink association. This is saved association, not cryptographic source provenance or external-input immutability. Integration stays under `artifacts/deps/` for existing CVar isolation; runtime receipts reuse stable sealed stat metadata, not fresh hashes. Pre-existing UnitRole dead-code warning remains. Prior source27 host-PSI-aborted attempt is retained separately at sibling `forever-watch-membership-green-20261015T152323858709Z`; it is not a compiler failure or current runtime proof.

`membership-model-results.json` and full saved group outputs record **39 distinct PASS / 0 FAIL / 0 ignored**, both group exits0: quest18 (membership8 + revised projection4 + getter3 + whole cached header-consumer3), plus finite constants21. Selectors are `wowforever_quest_` and `wowforever_finite_constants::`, serialized with CASC disabled. The [quest contract](../specs/forever-quest-difficulty.md#ordinary-watch-membership-follow-up--bounded-model-execution) owns behavior and fixture limits; whole-header model passes are not normal header execution. Integration inventory **8427** is discovery only, zero executed cases. Independent saved model-runtime reconciliation **PASS**: `E/model-runtime-report.md` confirms all 39 distinct names, full output, statuses, metadata and isolation; no commands were rerun.

Saved actual normal original membership and actual `QuestMapFrame` header-consumer executions at sourcef91 both exit0 without execLua error. Original stderr proves counts2→1→2, one-result types1→nil→1, one `true` per mutator, two synchronous `QUEST_WATCH_LIST_CHANGED` events (false/true), retained rich row/difficulty and final `WATCH_MUTATOR_REGRESSION_GREEN`. Header stderr records `WATCH_HEADER_REAL_FRAME_GREEN` with4,4; only the unrelated refresh tail is observed/replaced, not full refresh or UI parity. Headless `A_Print` routes print to stderr; stdout lacks these markers. Initial `accepted=false` flags remain in `membership-normal-results.json` and `membership-real-frame-header-result.json`; `normal-stream-reconciliation.json` corrects only saved marker stream inspection, without reruns or changed assertions. Independent saved normal-runtime reconciliation **PASS**: `/home/osso/.local/state/wow-ui-sim/verification/watch-membership-next/normal-runtime-report.md` verifies full streams, unchanged original assertions, marker routing and exact state/event observations; clean startup remains FAIL.

Fresh actual startup **FAIL:10 records /14 occurrences, exit1**, with Gamepad errors. `membership-startup-baseline-receipt.json` proves capture bytes identical to the committed baseline; no baseline JSON update. Source40a/source704 historical receipts remain separate. Later [default-profile compiler preservation](#default-profile-compiler-preservation--2026-10-15) is independently accepted; other-profile runtime, native parity, full suites, clean whole startup and full-parent acceptance remain open. This documentation reconciliation performs no build or runtime execution.

### Default-profile compiler preservation — 2026-10-15

Independent [saved-evidence report](/home/osso/.local/state/wow-ui-sim/verification/watch-default-check-next/report.md): **compiler preservation PASS** at exact source `70112e418760d1b861090e9d6b31b62665bd99f8`; docs-only `4c083f98a` does not invalidate it. Private epoch: `/home/osso/.local/state/wow-ui-sim/verification/widget-discrete-progress-current/watch-default-preservation-check-20261015T181448529607Z`. Default `cargo check --offline --locked --message-format=json -j 1` exited0 in41.86s with zero `compiler-message` diagnostics; terminal guard `build_processes_exited`, no recorded pressure abort, MemoryPeak4,703,543,296 bytes. All artifact profiles have `test=false`: cached-dependency production metadata checking only, not clean compilation, executable linking, `cfg(test)` compilation, runtime, alternate-profile, formatting or full-parent acceptance. No commands rerun for this receipt.

### Public offer rich-log coherence RED — 2026-10-15

[Owning contract](../specs/quest-offer-rich-log-coherence.md); evidence root O2: `/home/osso/.local/state/wow-ui-sim/verification/widget-discrete-progress-current/forever-offer-coherence-red-retry1-20261015T165133113920Z`. Saved source/runtime basis `f94aea875ca28b89a9678b4aa8e39bf6c678706a`. Full `public-offer-coherence-red/{stdout,stderr,result.json}` records actual **0 PASS / 1 FAIL / 0 ignored, exit101**, not a fixture failure: public preconditions complete and final Rust coherence assertion fails. Public `A_Admin.OpenMultiQuestNpc()` offers80002 “Supply Run”, level80; `SelectAvailableQuest(1)` → `ConfirmAcceptQuest()` leaves lightweightOccurrences1, pendingNone, selected80002 and queuedAccepted1. Rich `IsOnQuest=false`, index/info absent; explicit `AddQuestWatch=false`, count2 unchanged (80000,80001). No state/API/event-handler replacement was used.

Separate `existing-quest-verb-controls/{stdout,stderr,result.json}` records **13 distinct PASS, exit0** for `quest_verbs::`; these establish existing lightweight controls, not rich coherence. `approved-lightweight-results.json` retains both group receipts. Explicit lead approval covered14 lightweight cases without FIFO waiting: MemoryMax2GiB, CPUQuota25%, Nice19, timeout86s, scoped host memory full avg10>5 stop monitor. Both saved `*-host-psi-guard.json` receipts report no aborts, signals or errors. This handoff only reads saved evidence; no new runtime.

First source90 attempt stopped at E0277/u32 fixture compilation, **not behavioral RED**. Typed fixture fix `f94aea875` compiled in3m21s with guard `build_processes_exited`, integration-only `artifacts/deps/` seal; UnitRole dead-code warning retained. `build-summary.json`, `terminal-command.json`, `pressure-watch-result.json` and `seal-integration.json` own compile/seal attribution; compilation does not earn behavior credit.

Independent saved-runtime reconciliation **PASS**: `/home/osso/.local/state/wow-ui-sim/verification/quest-offer-coherence-next/runtime-report.md` confirms expected final-assertion RED, reachable public fixture, separate 13 controls, guard observations and fresh f94 formatting; the coherence behavior remains FAIL. The earlier rich-metadata question was answered: “can you enrich the data thanks”. Offer80002 “Supply Run” is simulator-authored, not native; the [owning contract](../specs/quest-offer-rich-log-coherence.md) now defines the bounded ordinary-offer enrichment policy. New baseline source `d59551d61` executed all three snapshot/enrichment cases: **1 negative control PASS / 2 `MISSING RICH ROW` FAIL, exit101**. Both failures stop at the first rich-row assertion; downstream metadata/watch assertions are unreached. Independent saved audits: `/home/osso/.local/state/wow-ui-sim/verification/quest-enrich-next/runtime-red-report.md` and `build-seal-report.md`. The latter's earlier runtime HOLD is superseded by the runtime report; its successful 5m25s Forever integration compile/seal evidence and existing UnitRole warning remain. First source `edf7dcec6` E0603/private-import fixture compilation failure is retained, not behavioral RED. Producer `ccd9e2d01` now implements selected snapshots and rich/light publication; [wiki source inventory](../wiki/systems/lua-api.md#public-offer-rich-log-state-split) explains the path. Test-control `8802fa400` adds the existing-row preservation case, now executed in the current group below; original f94 RED and13 controls remain separate historical evidence. Earlier [sourcef91 watch39 and normal GREEN/startup FAIL10/14](#current-watch-membership-model-execution--2026-10-15) remain separate. No baseline JSON update, clean-startup, native or full-parent acceptance.

Current main-observed Forever source `53cf24ab0`: **43/43 PASS, exit0**. Evidence E: `/home/osso/.local/state/wow-ui-sim/verification/widget-discrete-progress-current/quest-offer-enrichment-forever-green-20261015T202148238904Z/quest-enrichment-and-preservation`; full `stdout`4027 bytes / `stderr`30164 bytes, harness32.69s, no errors. Actual groups: offer enrichment3 + existing-row preservation1 + original acceptance1 + verbs13 + gossip13 + watch membership8 + projection4. Executed boundaries cover snapshots across gossip close/replacement, cancellation/isolation/repeat confirmation, represented metadata/reward preservation and explicit watchability. Explicit simulator80002→37/customNone mapping is proven, not native metadata.

Current no-run completed in0.98s by reusing fresh artifacts, not a clean rebuild; one UnitRole dead-code warning remains. Independent build/seal report: `/home/osso/.local/state/wow-ui-sim/verification/quest-enrich-next/green-build-seal-report.md`. Independent saved-runtime reconciliation **PASS**: `/home/osso/.local/state/wow-ui-sim/verification/quest-enrich-next/green-runtime-report.md` confirms43 unique PASS, exit0, complete streams, reachable assertions and stable artifact stats. Retail preservation is separately accepted below; bounded quest-source check/startup gates are [accepted separately](#quest-source-check-and-startup-gates--2026-10-15). Old Gamepad **10 records / 14 occurrences** remains unchanged and was not rerun. This docs update runs no builds/tests and does not change saved evidence.

#### Retail quest preservation — 2026-10-15

Source `a56e11267202be62fc9dd42171853dfb524fc360`; evidence E: `/home/osso/.local/state/wow-ui-sim/verification/widget-discrete-progress-current/quest-enrichment-retail-preservation-retry1-20261015T205548381925Z`. Independent saved audits **PASS**: `/home/osso/.local/state/wow-ui-sim/verification/quest-enrich-next/retail-green-build-report.md` and `retail-green-runtime-report.md`.

Default-Retail integration no-run compilation succeeded in **12m13s**; actual emitted features include `client-retail`, `default`, `profile-retail` and Retail epochs through12.1.0, not PTR/Forever or12.1.5. Saved compiler output ends `build-finished success=true`, with zero compiler-message diagnostics. Admission/pressure audit passes: saved guard terminal `build_processes_exited`, no pressure abort or max/OOM increase; collected watcher unit is not retained watcher exit-status proof. Independent complete byte comparison of original/sealed integration matches SHA256 `fcf228806a0203e11a18eea699f7245e88890f2652f5d133f511ffd6ccc5fbdf`. Metadata/stable-stat association is not cryptographic source provenance. First `204301Z` attempt remains a separate host-PSI guard SIGTERM abort, with no compiler verdict or tests.

`retail-quest-preservation/{stdout,stderr,result.json}` records **31 unique PASS / 0 FAIL, exit0**: confirmation5 + Retail verbs13 + gossip13; zero ignored/measured,11064 filtered, harness **16.50s**. Worker16.534s, CPU4.132s, peak65.9M; stdout2596/stderr19858 bytes. Complete streams and exact identities independently reconciled; stderr contains startup timing only. The earlier source estimate incorrectly included a Mists-only verb; actual Retail execution is31, not32. Real synchronous listeners assert exact three-argument payload/types/selected ID, immediate callback sentinels and post-callback state/events; no observed swallowed callback error. Current pending-snapshot Retail prompt preservation is accepted only at these bounded fixtures, not loaded-popup/native behavior.

Sourcea56 formatting receipt remains PASS, exit0, empty output. Original Forever43 at source53cf remains separate, not re-audited or pooled with Retail. The following receipt closes only quest-source check/startup gates; no C_UIWidgetManager closure, broad/native/full-parent acceptance or baseline JSON change.

#### Quest-source check and startup gates — 2026-10-15

**Accepted saved evidence, not whole-current-HEAD proof.** Independent [default-check audit](/home/osso/.local/state/wow-ui-sim/verification/quest-enrich-next/current-default-check-report.md) covers source `87c9b1eda663ad01cb41dd6a5fc19cf8584b74f0`: actual default-Retail `cargo check --offline --locked --message-format=json -j 1`, **exit0, 39.86s, zero compiler diagnostics**, terminal `build-finished success=true`. Source87 differs from a56 only in documentation; Rust source is unchanged. This metadata check is not a linked binary, test execution or other-profile proof. The report owns command, toolchain, guard, full-output and source-association details and their limits.

Independent [normal startup audit](/home/osso/.local/state/wow-ui-sim/verification/quest-enrich-next/current-startup-report.md) accepts the sealed normal binary at source `a56e11267202be62fc9dd42171853dfb524fc360`: isolated default-Retail `--no-addons --no-saved-vars lua-errors`, **exit0, stdout exactly `[]` plus LF**, complete14,682-byte stderr inspected with zero cleanup/handler/other observed error diagnostics; worker30.997s. Existing CVar input was copied under an explicit private `XDG_DATA_HOME`; saved original before/after hashes are stable, not a reset or native/user-state parity proof. Report owns seal association, full streams, CVar-copy provenance limits and evidence paths. CASC/sound were disabled; no full third-party-addon, SavedVariables or render proof.

These receipts and existing a56 fmt PASS close only bounded **QUEST-SOURCE** gates. Later `4cb2b5e98` / `52b73d664` DoubleIcon passive/test code is **not covered by check/startup/fmt** and awaits a new RED build; historical selector RED remains separate. No whole-current-HEAD startup pass, native parity, broad/full-parent acceptance or fresh rerun follows. Earlier aborts/REDs and Forever Gamepad10 records/14 occurrences remain unchanged, not rerun.

#### Abandonment profile correction — 2026-10-16

Matching cfg correction `dc2530c3c6440c0762efd8fd3e4fd2a914a8f3e8` has independently reconciled default and Forever library checks: both terminal exit0, exactly one successful `build-finished` each; default730 JSON records/zero compiler diagnostics, Forever731/one existing `UnitRole` dead-code warning. Full streams, unchanged pressure guards, source predicates, formatting exit0 and changed-line readability are recorded in `/home/osso/.local/state/wow-ui-sim/verification/quest-abandon-next/profile-correction-report.md`. The first default E0609 failure remains historical. Earlier abandonment3PASS at normal-library source `58bccba22` is retained separately, without replay: the Forever producer body is unchanged by the matching feature attribute. These are library compile checks, not Retail runtime, startup, native, all-profile or full-parent acceptance. [Owning contract](../specs/quest-offer-rich-log-coherence.md).

#### PTR shared controls — 2026-10-16

Normal-library basis `161dad02ad868ecce067b4b0c1d87108ec5b2dcb`; actual unchanged control tests compiled at `ecebf3dbf`. Independent saved-artifact report `/home/osso/.local/state/wow-ui-sim/verification/ptr-controls-next/report.md` accepts **39 distinct PASS, exit0** across two disjoint runs:

| Control | PTR cases | Executed boundary |
|---|---:|---|
| C1 | 5 | Slider quantization/effective callbacks; unsnapped StatusBar control |
| C2 | 14 | Real installed-font geometry/glyphs, UTF-8 editing and dirty/update ordering |
| C3 | 4 | Negative/reversed/empty/multibyte public selection endpoints |
| L1 | 5 | Protected-recipient direct/active-chain eligibility, unchanged geometry and moving controls |
| C7 | 3 | Bare button font-slot/state/render projection and child ownership |
| C8 | 5 | Public range callbacks and synchronous coherent value reapplication |
| C9 | 3 | Deferred range notifications at real OnUpdate/animation boundaries |

Six modules36PASS/12filtered; separate bare C7 run3PASS/45filtered. Nine other button cases were not run. Full streams, exact features/normal-dependency association, component seal and control/layout-source equality are independently reconciled; the newer IconTextAndBackground plumbing is excluded. Component compilation retains35 unused common-helper warnings, distinct from the accepted normal library's zero diagnostics. This is not genuine cached C2 consumer, ordinary startup, physical pointer, native/CASC-font/pixel, whole-current-HEAD, all-profile or full-parent proof. Wrath startup RED and guard remain unchanged; accepted Retail cases were not replayed.

#### Forever shared controls — 2026-10-16

Independent report `/home/osso/.local/state/wow-ui-sim/verification/forever-controls-next/report.md` accepts **39 distinct Forever PASS, exit0** at saved normal-library basis `58bccba22f785250c78ab4153fd64e49aa477a5e`: six-module36PASS/12filtered plus separate bare-C7 3PASS/45filtered. These separately execute the same [seven control boundaries](#ptr-shared-controls--2026-10-16), not PTR results credited as Forever proof. Current unchanged tests compiled at `756a59006`, with exact16 Forever feature flags and normal dependency records; component26 unused-common warnings retained. The earlier library/abandonment proofs were reused without another Cargo build or abandonment replay. Complete streams, seals, source equality and installed-font assertions are independently reconciled. No newer IconTextAndBackground plumbing, genuine cached C2 startup, nine other button cases, native font/pixel/pointer, ordinary startup, whole-current-HEAD, all-profile or full-parent acceptance.

#### Mists shared controls — 2026-10-16

**Independently accepted bounded saved execution.** Normal Mists library `d62a9af2626075ddc99c2467f02e940faab83229`; separate component checkout `85fb070bc75da72bbe8cfc0dfe142c5832da45a0` links that library. Library production basis727, build exit0 in5m34s,731 JSON records, one existing UnitRole warning and preserved normal-library seal. Component compile exit0 retains26 inherited common-helper warnings; neither is a zero-warning claim.

| Control | Mists cases PASS | Era cases PASS | Anniversary cases PASS | Owning contract |
|---|---:|---:|---:|---|
| C1 | 5 | 5 | 5 | [Slider callbacks](../specs/slider-value-callback.md#clean-room-step-assignment-follow-up), [range validation](../specs/widget-range-validation.md) |
| C2 | 14 | 14 | 14 | [EditBox geometry](../specs/editbox-text-position-selection.md#cursor-geometry-follow-up) |
| C3 | 4 | 4 | 4 | [EditBox selection](../specs/editbox-text-position-selection.md#tests-asserting-this-spec) |
| L1 | 5 | 5 | 5 | [Protected-recipient targets](../specs/protected-anchor-targets.md) |
| C7 | 3 | 3 | 3 | [Active button font](../specs/button-active-font-state.md), bare cases only |
| C8 | 5 | 5 | 5 | [Callback lifecycle](../specs/slider-value-callback.md#c8-changed-range-callbacklifecycle-candidate) |
| C9 | 3 | 3 | 3 | [ScrollFrame update boundary](../specs/scrollframe-script-bindings.md#c9-range-update-boundary) |

Seven disjoint groups total **39 unique PASS, all exit0**. Independent full-stream/source/feature/extern/seal/CVar reconciliation: `/home/osso/.local/state/wow-ui-sim/verification/mists-controls-next/report.md`. Epochs resolve through `/home/osso/.local/state/wow-ui-sim/handoff/classic-controls-mists-library-epoch.txt` and `classic-controls-mists-component-epoch.txt` beside it. All seven original-CVar stat/hash pairs match; fresh child copies are per invocation, not a historical guarantee. Retain the earlier host-PSI safety abort and precompiler admission holds as non-acceptance, not compiler/test failures.

C2 uses the no-CASC installed-font backend; C7's normal constructor requests CASC but `WOW_SIM_CASC=0` and full streams establish system-font fallback. Neither proves WoW font identity or native pixels. Source417 ItemDisplay is absent from runtime libraryd62; it gains no acceptance. All runtime stderr lines were inspected as bare simulator setup timings, not cached Blizzard application startup.

Accepted PTR/Forever receipts remain separate and were not rerun. Era and Anniversary bounded saved execution are recorded below; broader profile acceptance remains separate. Wrath guard and original pre-tick startup failure remain unresolved. No cached Blizzard startup/consumer, native, GUI/pointer/pixel, optional XML3, whole integration, all-profile or parent closure follows.

#### Era shared controls — 2026-10-16

Independent [saved-artifact report](/home/osso/.local/state/wow-ui-sim/verification/era-controls-next/report.md) accepts **39 unique Era PASS**, seven runtime exits0, 0 failed/ignored/measured. The [existing owner matrix](#mists-shared-controls--2026-10-16) records the same39 logical behaviors with separate profile-specific execution, not 78 distinct behaviors or Mists results credited to Era.

Normal Era library basis `3faaab8a92b5f998a76dfd63f73c7033db4ed3c4` and separate test-source snapshot revision `3faaab8a92b5f998a76dfd63f73c7033db4ed3c4` are artifact provenance, **not current HEAD/source acceptance**. The normal library build and direct seven-module component compilation are separate successful invocations; all seven groups use the same sealed component. Saved library excludes the unexpected uncommitted `src/lua_api/editbox_update.rs` draft; that draft gains no compile/runtime acceptance.

Exact diagnostics: **normal library1 warning, component26 warnings** (27 across separate compilations), 0 compiler errors; **runtime0 warning/error/panic lines**. Runtime stderr is setup telemetry, not full startup proof. C1 ran before the foreign-window hold; denied C2 admission was not execution or failure; remaining34 passed after removal on the same seal. The report owns exact roots, seals, names, admissions and unchanged original-CVar guards; no evidence replay is implied.

Anniversary was pending at this Era checkpoint; its separate bounded acceptance is recorded below. No native/font-identity, cached Blizzard startup/consumer, cache, GUI/pointer/pixel, optional XML3, nine other button cases, whole integration/crate, all-profile or full-parent closure. Wrath guard and original pre-tick startup failure remain unresolved; prior profile receipts remain separate.

#### Anniversary shared controls — 2026-10-16

Independent [saved-artifact report](/home/osso/.local/state/wow-ui-sim/verification/anniversary39-20261016/report.md) accepts **39 unique Anniversary PASS**, seven runtime exits0, 0 failed/ignored/measured. The [existing owner matrix](#mists-shared-controls--2026-10-16) records the same39 logical behaviors independently executed for Mists, Era and Anniversary, not117 distinct behaviors or foreign-profile results credited to Anniversary.

Normal Anniversary library and separate test-source snapshot revision `70ebaee0e70997b691edeaf894156ca376c2ad1f` are artifact provenance, **not current HEAD/source acceptance**. Normal library build and direct seven-module component compilation are separate successful invocations. Actual emitted features `casc,client-anniversary,gui,rodio,sound` match component cfgs; all seven groups use the same sealed component. Freshly recomputed normal-library SHA-256 `47c4f8c68c5994c3539516f974c1a01e2b77e5fec638a3681752d95c35b5c3bf` and component SHA-256 `d11254033bd84b72c57f40652b06d9520e52d9480c3710f8bcd2ff27f94af78f` match their saved seals.

Exact diagnostics: **normal library1 warning, component26 warnings**, 0 compiler errors; **runtime0 warning/error/panic lines**. The report owns exact epochs, raw per-name PASS output, recipe/Era/Mists name-set equality and all seven original-CVar before/after stat/hash matches; fresh independent child-only copies were selected per invocation. Earlier aborted build is not accepted execution. No build or test replay performed for this reconciliation.

No native/font-identity, cached Blizzard startup/consumer, cache publication, GUI/pointer/pixel, optional XML3, nine other button cases, whole integration/crate, all-profile or full-parent closure. Separate startup/cache and prior profile receipts retain their own scope.

## Panel And Addon Evidence

- `mists-panels.md` lists the Blizzard panel parity matrix.
- `mists-panel-interactions.md` records interaction audit coverage.
- `mists-panel-visuals.tsv` records visual metric samples.
- `mists-test-coverage.md` maps parity rows to focused Rust tests.
- `mists-release-proof.md` records the full local release proof command.
- `classic-addon-test-targets.md` lists the installed Mists addon matrix.
- `mists-lod-audit.md` records LoD addon coverage.

## Forever player-storage root: bounded RED/GREEN — 2026-10-16

[Player-storage contract and execution](../specs/forever-gamepad-player-storage.md#bounded-execution--2026-10-16) records simulator-owned slots4096–4179, three player acceptance cases and the existing pet control PASS at producer `0177adc75`. Normal sealed startup now reports **exit0,0 records/0 occurrences** with no third-party addons/SavedVariables/CASC/sound. Own preproducer startup reproduces **exit1,10 records/14 occurrences**:4 direct arithmetic,2 parent page-lookup and8 stance-anchor occurrences. Historical message equality differs only in the generated handler hash in four event rows. Fixing the producer resolves downstream initialization without map/anchor or vendor patches.

`wowforever-lua-errors.json` remains the historical preproducer inventory, not the current bounded result. The linked receipt owns precise revisions, scope, diagnostics and private evidence; native/GUI, full integration and parent acceptance are not established.

## Quest lifecycle integration

[Owning QL-1 / QL-2 receipt](quest-lifecycle.md) separates historical REDs, saved profile execution, clean startup checkpoints and the current latest-base preservation gate.
