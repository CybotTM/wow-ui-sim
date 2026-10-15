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

`wowforever-lua-errors.json` is the actual corrected-enUS `1.60.1.70374` headless startup capture on October 15, 2026: **166 JSON records, 238 occurrences, exit 1**. First error: `Blizzard_EditMode/Mainline/EditModePresetLayouts.lua:1272`, attempt to index field `?` (nil). This is a failing baseline, not accepted clean startup. The [locale proof matrix](../specs/casc-loading.md#locale-checkpoint--october-15-2026) owns binary/source scope, acquisition validation and receipt location; independent saved locale/runtime receipt verification PASS does not change the startup FAIL.

The previous tracked `[]` is preserved privately as `wowforever-lua-errors.pre-70374.json` beside those receipts; it is not this capture. Historical first capture after `65139f909` synchronized 4,398 files and reported 422 records/514 occurrences, exit 1. That history and the [historical selected69977 checkpoint](../updating-blizzard-ui-to-a-new-patch.md#historical-selected-target--september-24-2026) retain their original identities; neither is relabelled as 70374 proof.

## Panel And Addon Evidence

- `mists-panels.md` lists the Blizzard panel parity matrix.
- `mists-panel-interactions.md` records interaction audit coverage.
- `mists-panel-visuals.tsv` records visual metric samples.
- `mists-test-coverage.md` maps parity rows to focused Rust tests.
- `mists-release-proof.md` records the full local release proof command.
- `classic-addon-test-targets.md` lists the installed Mists addon matrix.
- `mists-lod-audit.md` records LoD addon coverage.
