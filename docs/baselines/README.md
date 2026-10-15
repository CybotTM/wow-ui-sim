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

`wowforever-lua-errors.json` now contains the actual source136 post-enum corrected-enUS `1.60.1.70374` capture: **58 records / 58 occurrences, exit1**, still FAIL; first failure is missing `import`. Main copied this JSON; this documentation update does not rewrite it. Historical source `9a6ea924e` capture was **166 records / 238 occurrences, exit1**. Preset line1272 and setting-display-map cascade no longer appear; other preset errors remain. Counts are unique records/occurrences, not root-cause counts.

Both captures report 272 Blizzard addons loaded and 1658/1658 bytecode-cache hits. Different explicit CASC/product environment settings and unresealed external cache inputs prevent whole-input equivalence, archive acceptance or controlled performance claims. See [verified receipts](#verified-scoped-receipts--2026-10-15) for attribution and the [locale owner](../specs/casc-loading.md#locale-checkpoint--october-15-2026) for acquisition scope. Clean startup remains open.

Historical pre-70374 `[]` remains private as `wowforever-lua-errors.pre-70374.json` beside the earlier receipts. The earlier 4,398-file, 422-record/514-occurrence capture and [selected69977 checkpoint](../updating-blizzard-ui-to-a-new-patch.md#historical-selected-target--september-24-2026) retain their historical identities.

## Verified scoped receipts — 2026-10-15

All new outcomes below bind runtime source `136748f0b3e122b6c9f897c69db19dcf1b317c5f`, before the import change. Import `012df1e45` and finite-event `c2bf42c4f` compilation/execution remain pending: the first new build stopped at the host-memory safety guard before producing new binaries. Their new tests and normal startup remain unexecuted/NOT GREEN; this was not a compiler error. Independent saved-evidence reports: `/home/osso/.local/state/wow-ui-sim/verification/feedback-models-current/report.md` and `/home/osso/.local/state/wow-ui-sim/verification/feedback-gpu-startup-current/report.md`. Evidence root E: `/home/osso/.local/state/wow-ui-sim/verification/widget-discrete-progress-current/feedback-forever-integrated-tests-20261015T101651047924Z`. Reports reuse saved seals and matching stat metadata, not fresh artifact hashes or enforced immutability.

| Component | Actual bounded proof |
|---|---|
| [Locale/cache provenance](../specs/casc-loading.md) | Selector 5 + provenance 1 PASS |
| [Rotated tiling](../specs/rotated-tiled-edge-prefix.md) | 8 model tests PASS |
| [Slider](../specs/slider-bound-thumb-layout.md) | 18 model tests PASS |
| [Forever enums](../specs/forever-edit-mode-enums.md) | 5 model/real cached-consumer tests PASS |
| [Profile identity](../specs/client-profiles.md) | Manifest 1 + BuildInfo identity 1 PASS |

These are 39 distinct outcomes. A separate exact [Slider render-list regression](../specs/slider-bound-thumb-layout.md#tests-asserting-this-spec) passed 1/1 on the same source, independently verified from saved evidence; 40 model outcomes total, not full-suite execution. Separate production GPU test explicitly executed with `--ignored --exact`: 1 PASS, 0 ignored; all four controls passed (solid, texture roundtrip, half-alpha over black, additive), actual Vulkan NVIDIA RTX 5060 Ti, `Rgba8UnormSrgb`. Mapped RGBA respectively `[124,170,203,255]`, `[51,102,153,255]`, `[89,124,149,255]`, `[124,170,203,255]`. This is shader/attachment readback, not native parity or all-GPU/full-UI acceptance.

Separate existing require/sandbox/registry/path tests: 12 + 6 + 14 + 16 = 48 distinct PASS **before the new import binding**; not acceptance of `012df1e45`. Actual current-binding diagnostic successfully completed `require` of Utility as a table, then failed `IMPORT_BINDING_RED` with `import=nil`; process exit0 does not make this assertion PASS.

Compilation and saved fmt passed for the scoped source/profile; UnitRole dead-code warnings remain. Default `cargo check`, full suites, clean startup, native/cross-profile and full parent acceptance remain open. No new build/runtime execution for this documentation update.

## Panel And Addon Evidence

- `mists-panels.md` lists the Blizzard panel parity matrix.
- `mists-panel-interactions.md` records interaction audit coverage.
- `mists-panel-visuals.tsv` records visual metric samples.
- `mists-test-coverage.md` maps parity rows to focused Rust tests.
- `mists-release-proof.md` records the full local release proof command.
- `classic-addon-test-targets.md` lists the installed Mists addon matrix.
- `mists-lod-audit.md` records LoD addon coverage.
