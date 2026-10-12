# Independent retry gate — PASS, bounded current run

Source `490bc10807155cbfee8710142c28bc88991d35a5`; owned epoch `diagnostic-20261012T140758688372Z`.
Original `59f3ff136` compile **101 E0583** remains separate in `independent-20261012T140800990027Z`; not erased by this pass.

## Coverage matrix

| Contract | Result | Evidence / proof level |
|---|---|---|
| Integration compilation | PASS exit 0 | Full saved compile stdout decoded; build-finished success=true, current integration compiler artifact; compile stderr retained below |
| Typed serializer concrete behavior | PASS 1/1, exit 0 | Exact selector `startup_perf::observation::tests::addon_observation_roundtrip_preserves_values_and_only_diagnostic_counts`; decoded exact names, distinct durations, file/cache/diagnostic counts and no diagnostic text. Raw stdout retained in proof.json |
| Canonical startup body | PASS 1/1, exit 0 | Exact selector `startup_perf::full_game_startup_stays_under_budget`; raw stdout has parent and child summaries, **one body execution**, not two tests credited. Timeout runner drops parent closure and runs guarded child closure, tests/common/timeout_reexec.rs:75–85,104–114,175–188,274–295 |
| Readiness / original budget | PASS **2,856,325,824ns < 20,000,000,000ns**, readiness=true | Same-run JSON summary plus actual original assertions pass. This observation does not supersede any earlier threshold failure or establish repair/historical equivalence |
| JSON reconstruction | PASS 220 addon rows + 1 summary | Independently parsed raw stderr, contiguous rows, ordinals 1–220, 220 unique discovered names, summary last, summary count 220; all eight timing keys, six cache keys, diagnostic counts-only shape and nonnegative integer units |
| Returned-field coherence | PASS 220/220 | Each LoadResult stored alongside its own elapsed duration in canonical ordered loop; each total matches existing LoadTiming arithmetic, each lua_exec matches compile+call. Returned/display names are not required to equal folder names. 0 warning counts, 185 nil-symbol observations, 90 missing requirements retained as counts; no claim of zero compatibility gaps |
| Ordering / timing boundaries | PASS source audit + runtime evidence | Canonical new_game_env/Game discovery/load loop/workarounds/events and readiness/strict threshold unchanged. Clock stops directly after load_addon, before unwrap/push. Argument evaluation env.loader_env() included; not pure-body timing. Vec/clone/push overhead remains in phase/full clock; full clock stops after startup events, JSON emitted after clock and before assertions. Main's follow-up only adds explicit module path |
| Phase containment | PASS same run | Addons 2,150,189,470ns; post-load 188,796,337ns; startup events 408,696,493ns. Full minus these phases=108,643,524ns (not independently attributed). Addon phase exceeds sum of call clocks by 60,187,597ns (not independently attributed). No overlapping loader buckets summed as elapsed-time proof |
| Seal authentication | PASS exactly once | New seal 468,126,144 bytes; sha256 `7a98eeb84ad8a149d619a0ceb504d312d55b9f1c57c2d12ffcadad487fc00933`; immediate receipt at owned epoch/independent-auth-integration.json, timestamp 2026-10-12T14:08:35.495810+00:00. No old seal hashed, no new seal rehashed |
| Source before/after | PASS | Before compile, after compile, after runtime and independent final five-file digests equal; relevant diff from 490bc1080 empty |
| Formatting | PASS exit 0 | Existing main-owned fmt output inspected, no invocation replayed |
| Production check reuse | PASS limited unchanged scope | Friend check-20261012T124825577210Z check exit 0; recorded HEAD 05fddbf399 versus current src/Cargo/crates/iced-wgpu-patched diff empty. Does **not** cover new integration tests; new targeted integration compile provides that evidence |

## Environment and limits

Captured explicit overrides={}, all captured allowlist values null, including **WOW_SIM_LOG_HANDLER_TIMINGS** and **XDG_CACHE_HOME**. No missing-value claim for these two. Actual bytecode-cache enable/disable controls, **ASSET_RESOLVER_CACHE_DIR**, and **WOW_SIM_VERBOSE** child values were not captured: unknown. **WOW_SIM_BYTECODE_CACHE_DIR** was captured but is not an actual reader; no full cache-policy certification. No replay to reconstruct current/historical child values; no cache-tree attestation.

No native-client parity, pure-body/exclusive timing buckets, causal repair or historical equivalence claim. This proof is the exact current canonical run, not a full test suite. Source establishes emission order; failed-assertion emission was not separately executed because this current run passed. Unit operation success alone was not used as proof.

## Rust readability

[LENGTH] tests/startup_perf/observation.rs:15 — encode_addon_observation(): 32 body lines, above skill's 30-line guideline. Flat declarative JSON projection; optional pure timing/cache projection extraction only, no change made. No deep nesting, high branching, warning suppression, new TODO/FIXME/HACK/XXX, empty error handling or fallback. Concrete test under 200 lines. Explicit module path resolves observed wiring failure; no adjacent refactor authorized.

## Exact raw stdout

### Serializer

```

running 1 test
test startup_perf::observation::tests::addon_observation_roundtrip_preserves_values_and_only_diagnostic_counts ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 10785 filtered out; finished in 0.00s

```

### Startup (parent/child, one body)

```

running 1 test
test startup_perf::full_game_startup_stays_under_budget ... 
running 1 test
test startup_perf::full_game_startup_stays_under_budget ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 10785 filtered out; finished in 3.05s

ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 10785 filtered out; finished in 3.09s

```

## Retained compile warnings / toolchain

Cargo 1.99.0 (5f94df478 2026-08-27), rustc 1.99.0 (b940084d7 2026-09-28), Arch Linux, x86_64-unknown-linux-gnu, LLVM 23.1.1. Six manifest deprecation warnings remain, not suppressed or treated as clean.

```
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.large-enum-variant` is deprecated in favor of `lints.clippy.large_enum_variant` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.map-entry` is deprecated in favor of `lints.clippy.map_entry` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.match-wildcard-for-single-variants` is deprecated in favor of `lints.clippy.match_wildcard_for_single_variants` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.redundant-closure-for-method-calls` is deprecated in favor of `lints.clippy.redundant_closure_for_method_calls` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.trivially-copy-pass-by-ref` is deprecated in favor of `lints.clippy.trivially_copy_pass_by_ref` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.type-complexity` is deprecated in favor of `lints.clippy.type_complexity` and will not work in a future edition
warning: `iced_wgpu` (manifest) generated 6 warnings
   Compiling wow-ui-sim-test-harness v0.1.0 (/home/osso/Projects/wow/wow-ui-sim/crates/test-harness)
   Compiling wow-ui-sim v0.1.0 (/home/osso/Projects/wow/wow-ui-sim)
    Finished `test` profile [optimized + debuginfo] target(s) in 35.72s
```

`proof.json` retains reconstructed rows/names, all raw runtime stderr, compiler diagnostic records, raw stdout, stage digests, exact checks and saved first authentication receipt. No source edits, replay, unit mutation, delegation, or old binary/cache hashing by verifier.
