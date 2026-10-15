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

Use `scripts/diff-lua-errors.sh BASELINE NEW` to compare message sets and
report regressed/fixed errors. The Mists CI guard uses this for addon-induced
error counts and release proof diagnostics.

## Forever Lua Errors

Current saved corrected-enUS `1.60.1.70374` capture at runtime/build source `70459ec5a`: **12 records / 16 occurrences, exit1**, still FAIL; first failure is nil Gamepad player-storage arithmetic. Remaining records include an explicit missing rich quest80002 error; known four-row quest coherence is not whole-startup acceptance. Historical c2bf/build57 capture **22 records / 26 occurrences, exit1**, first nil `Enum.VoiceProviderID`, remains privately retained beside current receipts. Source136 post-enum capture **58 records / 58 occurrences, exit1**, first failure missing `import`, remains privately retained. Baseline JSON copying belongs to main; this documentation update does not rewrite or authenticate the tracked JSON. Historical source `9a6ea924e` capture was **166 records / 238 occurrences, exit1**. Preset line1272 and setting-display-map cascade no longer appear; other preset errors remain. Counts are unique records/occurrences, not root-cause counts.

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

Compilation and saved fmt passed for the scoped source/profile; UnitRole dead-code warnings remain. Default `cargo check`, full suites, clean startup, native/cross-profile and full parent acceptance remain open. No new build/runtime execution for this documentation update.

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

## Panel And Addon Evidence

- `mists-panels.md` lists the Blizzard panel parity matrix.
- `mists-panel-interactions.md` records interaction audit coverage.
- `mists-panel-visuals.tsv` records visual metric samples.
- `mists-test-coverage.md` maps parity rows to focused Rust tests.
- `mists-release-proof.md` records the full local release proof command.
- `classic-addon-test-targets.md` lists the installed Mists addon matrix.
- `mists-lod-audit.md` records LoD addon coverage.
