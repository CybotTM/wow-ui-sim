# Partial independent artifact verification — shared query profiles

Date: 2026-10-11. Epoch: `20261011T181742Z`. **PASS within completed pure-model scope; overall handoff OPEN.**

Read and followed `/home/osso/AgentConfig/skills/verify/SKILL.md` as the assigned independent verifier. `results.json` inspected exactly once: completed PTR, Wrath, Mists, Era only. No polling or authentication of subsequently completed profiles. No builds, reruns, test execution, runtime, checks, source edits, operations, delegation, or cwd changes. Only this external report written.

## Completed snapshot

| Configuration | Achievement | Garrison pure | Result |
|---|---:|---:|---|
| ptr | 8/8 | 3/3 | PASS; both exit 0 |
| wrath | 6/6 | 3/3 | PASS; both exit 0 |
| mists | 6/6 | 3/3 | PASS; both exit 0 |
| era | 6/6 | 3/3 | PASS; both exit 0 |
| Anniversary | — | — | PENDING |
| WowForever | — | — | PENDING |

**38/38 passing executions across four configurations**, not 38 distinct tests: six common achievement tests plus two PTR secret tests and three common Garrison tests (11 distinct names). Retail is outside this epoch audit, neither rerun nor credited here. No denied zero-case entries in the single snapshot; any denied zero-case lane is RESOURCE_BLOCKED, not test FAIL.

## Authentication ledger

Saved compile submissions use `/usr/bin/cargo test --offline --locked --jobs 12 --no-default-features --features sound,gui,casc,client-<profile> --test integration --no-run --message-format=json`. This is saved evidence, not a command executed by this verifier. All four compile results exit 0 and Cargo JSON ends with `build-finished: success=true`. Exactly one integration executable artifact per profile; source executable path, Cargo feature array and Cargo profile metadata match `artifact.json`. Exactly one expected client feature is enabled.

All four submissions identify source `ba413e8e177f04dc6fc7491834606ce36c750954`. Current inspected HEAD `c7d0c5ac3` changes only docs/retained evidence relative to that source, not runtime/test inputs. Eleven submitted SHA-256 inputs per profile independently match before/after receipts, committed source bytes at the submitted revision, and current files: 44/44 comparisons sets match. This is the declared eleven-input boundary, not a complete dependency/environment/content-addressed build proof.

Each completed sealed `integration-test` independently read and SHA-256 hashed exactly once; size and digest match its seal. Binary hashes were not repeated. Original artifact paths are mutable and were not hashed or run; sealed copies are the authenticated executable boundary.

| Profile | Independent SHA-256 |
|---|---|
| ptr | `e3ce97593765bf5fecf2016ca477a131fbf3645ce2d757765c1331df92f62756` |
| wrath | `e7253fa124af25f74156339fb97ac04b961a1ef111acbcec002ee4e50e7d88d0` |
| mists | `b4e6b6f94268a9135481c837a847cc0c26bfb01b9d54a203fc76815b6e53500a` |
| era | `4ff232bd8f9bdd140e7a5e8fcca5ced6dfce6851c519af08f737d60028cfe1c4` |

### Cargo feature receipts

- ptr: `aura-containers, aura-instance-enumeration, aura-xml-widgets, base-spell-relationships, casc, client-ptr, forbidden-animation-aspects, forbidden-aspects, gui, native-duration-formatting, numeric-rule-formatters, on-update-modes, player-cast-durations, retail-12-0-0, retail-12-0-5, retail-12-0-7, retail-12-1-0, retail-12-1-5, rodio, sound, timed-signal-maps`.
- wrath: `casc, client-wrath, gui, rodio, sound`.
- mists: `casc, client-mists, gui, prefork-full-ui, rodio, sound`.
- era: `casc, client-era, gui, rodio, sound`.

### Eleven input hashes

The following identical input manifest was authenticated separately for every completed configuration.

| Input | SHA-256 |
|---|---|
| `Cargo.lock` | `5577f98dd1c7b7124a51dae1ab30429366b58b2b065b54ac56dc1d0d69bc26f1` |
| `Cargo.toml` | `91c80dd9d2ff4d59ddce10af38a1dc09bee5ef0c5c166ebc19ec6ad485dac1e9` |
| `build.rs` | `cdebff6b6dec0e19e1124e734b2603432e2c47a7b956f9a4568ae412de182741` |
| `src/c_api/c_achievement_info_relations.rs` | `b45e91cff1c775fdf500b42be066b0034b0ca80948b1ba1bd44681a4f86895b3` |
| `src/c_api/c_garrison_damage_classes.rs` | `4319131c280f4cda7c6a75bd5b1fddf4704b5ba338fada122aa2361b0f6e1186` |
| `src/c_api/mod.rs` | `7c7a1b3b1b01063285bd6275f72a6f5f540a5a6c8cd1febf265e7521662368b3` |
| `src/lua_api/globals/font_strings_collection/colors.rs` | `9aa469106b6b23fdbe6a0074108c7213dacc0578d4af2e57356b7661a62b4811` |
| `src/lua_api/state.rs` | `0a9603dcb351ab356f41bb6f1fd8d6e6a7e3a114d790cea1236fe0157253e351` |
| `src/lua_api/state/sim_state.rs` | `1454e312e6996cd9b7bb64ed4a37bdd85ff881e3a67bd5ef10d19306e11e9cd4` |
| `tests/c_achievement_info_probes/superceding.rs` | `665a09046d8a5e1d4508a0d9a61711378c2732df4f8de3bd569afb40423ca641` |
| `tests/garrison_damage_classes.rs` | `e103b6725fa8f731d8292c06772168bd2b78fcdd37395a7db3147451dbf86146` |

Execution receipts, exact named `... ok` lines and final summaries agree with positive expected counts: PTR8+3, each other completed lane6+3, zero failed/ignored. Saved argv targets that profile’s sealed copy with achievement or Garrison selectors, `--nocapture --test-threads=1` under timeout90. Test stderr contains startup records only (PTR achievement88 lines, others66; Garrison33 each), no extra diagnostic lines. Compile stderr retains six existing iced_wgpu manifest deprecation warnings each; not warning-free proof. Saved toolchain: Arch Cargo1.99.0 and rustc1.99.0, host x86_64-unknown-linux-gnu, LLVM23.1.1. No fresh toolchain/check claim.

## Source-only gate and manual readability

`ba413e8e1` removes the module-wide Retail cfg and instead gates only `use crate::common` and the authentic cached-addon consumer test (`tests/garrison_damage_classes.rs:2,134`). Pure bootstrap/catalog/isolation tests are unguarded. No consumer execution is credited to these four profiles. Cargo.toml:154 enables PTR’s retail-12-1-5 -> retail-12-1-0 chain; superceding.rs gates its two secret tests on retail-12-1-0. Other audited configurations lack that feature: six ordinary achievement tests, not eight. Source feature/count gate PASS.

Manual readability audit of this cfg-only change: PASS, no violations. Both guards sit immediately beside their Retail-only dependency/consumer, with no new control flow, nested logic, suppressions, or opaque expression. Existing earlier palette/new bootstrap readability audit is retained in `data/patch-api/evidence/adventures-color-green-2026-10-11/source-report.md`; not rerun as a broad audit.

C API registration remains reachable at src/c_api/mod.rs:342–343; colors bootstrap at font_strings_collection/mod.rs:61. Existing tests assert actual outputs/state, not implementation shape. No new TODO/FIXME/HACK/XXX or commented-out implementation in the cfg-only diff.

## Exact covered contracts and exclusions

Achievement: one empty relation table for absent key; concrete supplied sequences preserve duplicates/order; updates/removal; fresh isolated tables without state mutation; strict numeric input; fractional/out-of-range identifiers do not alias. PTR additionally accepts untainted host-secret6 and rejects tainted secret without changing caller/parent taint or secret identity. This is supplied simulator-state/VM proof, not a native achievement-relation catalog or native security parity claim.

Garrison: one returned table; nonempty readable enUS labels for seven pure-school masks1/2/4/8/16/32/64; independently allocated outer and row tables across calls with mutation isolation. Test requires each expected mask/label but does not assert native order, exact row count/no additional schools, mixed schools, or locale parity. RGB bootstrap: four ADVENTURES_COMBAT_LOG globals, exactly three numeric GetRGB channels matching the shared simulator Mists-row palette. No new alpha/RGBA runtime assertion or Retail-native palette claim.

These profiles use pure fresh-environment tests, not the authentic cached Adventures consumer. No native, cached-addon, vendor/source-input acquisition, source refresh, full startup, whole-profile compatibility, broad-suite, or full-handoff acceptance is established. Prior separate Retail consumer proof remains separate.

## Remaining / proof preservation

Anniversary and WowForever remain PENDING by the sole results snapshot; their directories or eventual outcomes were not inspected. No waiting or retry initiated. Full handoff and excluded native/cache/source-input scopes remain OPEN. Existing historical failures/resource blocks are not overwritten by these bounded PASS results.

Evidence: epoch profile compile.stdout/compile.stderr, compile-submission.json, compile-result.json, artifact.json, outcome.json, execution-results.json and achievement/garrison-pure stdout/stderr. Independent authentication output retained at `/tmp/pi-tool-e374cf9dc10f7e08.log`; log-only diagnostics at `/tmp/pi-tool-f1503664cdc5c6da.log`. The initial Python name-resolution attempt failed before binary access; its correction performed the sole independent hashes. A later unavailable ctx lookup did not rehash or rerun anything.
