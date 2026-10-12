# Independent real talent-handler capture audit

2026-10-12. Epoch: `diagnostic-20261012T134247318942Z`. Revision: `1d2b0ea30afa2c76128bc1356b2125615c6e1748`.

**PASS — bounded capture coherence. Original selected test: PASS, credited once. Not performance repair, frozen BA, native parity, or full acceptance.** No Cargo, tests, simulator runtime, delegation, or repository edits performed by this audit. Read verify skill. Existing root source-only report/proof preserved; reused only after all ten source hashes matched. All five epoch source/Cargo hashes match before, after compilation, after runtime, and current files.

## Coverage matrix

| Evidence | Result / proof level |
|---|---|
| New `integration-sealed` authentication | PASS / independently SHA256-hashed exactly once; immediate `independent-auth-integration.json` saved. 468040824 bytes; SHA256 `56921ac7f2235fc2027f51893b85b1564f4936dea0a6f3900b10e319bda93bd4`. No old Friend/Quest seals read or rehashed. |
| Actual targeted compile | PASS / saved exit 0, Cargo `build-finished success=true`, unique integration compiler-artifact exactly equals artifact.json record; integration/test-harness fresh=false; library fresh=true. |
| Toolchain | Saved `/usr/bin/cargo -V`: Arch Cargo 1.99.0, commit 5f94df478; `/usr/bin/rustc -Vv`: Arch rustc 1.99.0, commit b940084d7eb6a299eb4bfeb8e34901bc051e7ac4, x86_64-unknown-linux-gnu, LLVM 23.1.1; both exit 0. Not a newly rerun toolchain inspection. |
| Original exact selector and timeout | PASS / saved outer runtime exit 0; one child body and successful parent wrapper. Outer timeout 90 seconds; inner child timeout 120 seconds. |
| Complete 20/10/24 windows and real stderr vs parsed output | PASS / independent reconstruction, no project parser import; every tick, phase, category, group, outside aggregate, and unparsed-line count equals saved parse.stdout. 108 correctly ordered markers, 54 windows. |
| Formatting | PASS / already terminal saved `cargo fmt --check` exit 0, stdout/stderr empty. No wait/poll/rerun. |
| Scoped default check | PASS reuse / Friend `check-20261012T124825577210Z` saved exit 0; independent Git diff from its fixture revision `6ec0c1cb8` is empty for src/Cargo/crates/build.rs. Covers unchanged production/library, not new test instrumentation; that test compiled separately above. |

Compile argv: `/usr/bin/cargo test --test integration --no-run --message-format=json --offline --locked -j 12`, cwd repo. Finished in 1m06s. Six existing iced_wgpu manifest Clippy-key deprecation warnings; zero Cargo compiler-message records. Do not describe build as warning-free. Reused default check finished in 38.52s with the same six manifest warnings.

Runtime argv: `/usr/bin/timeout 90 <epoch>/integration-sealed talent_panel_onupdate_perf::talent_panel_open_onupdate_tick_latency_stays_under_budget --exact --nocapture --test-threads=1`. Logger override `WOW_SIM_LOG_HANDLER_TIMINGS=0` installed on the launcher command before process creation. Recorded inherited CASC/sound/addon/SV/product/install/data-path allowlist values are all unset; no new overrides of these supplied. Fixture still initializes Game mode at 1024x768, discovers/loads Blizzard Game addons, applies existing workarounds and settles startup.

### Child protocol and environment

Linux `tests/common/mod.rs:30-32,153-157` routes performance macro to exclusive workload gate and timeout reexec. `timeout_reexec.rs` parent **drops** its closure; child executes `current_exe()` with exact same registered selector, `--include-ignored --test-threads=1`, forwarded nocapture, selector guard and handshake path. Command inherits environment; no env_clear or logger override/removal. Guarded child validates selector, create_new handshake records entry, then invokes closure once. Parent validates child status and handshake before success, drains/replays output once. The two `1 passed` summaries (child 4.69s, parent 4.77s) are **one actual body execution**, not two passes. One startup sequence and one canonical 54-window stream corroborate this.

Logger `handler_timing.rs:62-65` initializes OnceLock from environment inside the new child process; no Rust logger-setting mutations found. Saved launch source/metadata support zero threshold before first logger initialization. No retained independent child `/proc` environment/PID or handshake snapshot; handshake is deleted by design. This is protocol/source-backed evidence, not a historical independently witnessed process environment. Parser's conservative `all_attempts_verified=false` remains unchanged.

## Actual recorded intervals and counts

All numbers below are sums/maxima of **three-decimal rounded inclusive dispatch intervals**, not exclusive Lua body time.

| Region | Windows | Records | OnUpdate / OnPostUpdate / other | Total ms | Largest record ms |
|---|---:|---:|---:|---:|---:|
| baseline | 20 | 800 | 760 / 0 / 40 | 7.189 | 0.123 |
| settle | 10 | 436 | 404 / 0 / 32 | 61.275 | 55.552 |
| steady | 24 | 1008 | 960 / 0 / 48 | 8.765 | 0.143 |
| outside markers | — | 1704 | 232 / 0 / 1472 | 470.814 | 91.695 |
| entire stderr | — | 3948 | 2356 / 0 / 1592 | 548.043 | 91.695 |

Inside-window total: 2244 records, 77.229ms. Entire OnUpdate total/max: 82.553/55.552ms; other: 465.490/91.695ms; OnPostUpdate: zero observed. Handler-name counts: OnUpdate 2356, OnSizeChanged 864, OnEvent 471, C_Timer 254, OnFinished 3. 1056 distinct `(addon, handler, frame, source)` identities; 279 repeat identities, 2892 additional records beyond first identity occurrence. Zero zero-rounded-duration records. These are attempted-call records, not distinct callback/body counts.

Full stderr: 4195 lines = 3948 handlers + 108 markers + 139 other lines. All 139 other lines outside windows: startup/workaround diagnostics and one original tick summary. Zero other lines inside windows; zero malformed handler-prefixed or marker-prefixed lines. Complete line-numbered unparsed inventory and every independently derived grouped/phase/tick value retained in capture-independent-proof.json. All seven structured comparisons match, including exact deterministic group order, counts, integer-thousandth sums and maxima; not merely successful parser exit.

Largest settle record: Blizzard_PlayerSpells `__Blizzard_PlayerSpells_38159` OnUpdate, prototype `Blizzard_ClassTalentsFrame.lua:117`, one record 55.552ms. It is in settle, **not steady**. Largest entire-log group: GlobalCallbackRegistry OnEvent `#335`, seven records totaling 235.895ms, maximum 91.695ms, outside windows. Baseline MicroMenu group: 20 records, 2.090ms total, 0.123ms max; steady MicroMenu: 24 records, 2.479ms total, 0.143ms max. Baseline per-window summed intervals range 0.346–0.413ms; settle 0.362–57.152ms; steady 0.347–0.467ms.

Original tick summary, separate from handler interval sums: baseline average **511.77µs**, talent steady average **564.29µs**, steady maximum **711.22µs**. Rounded displayed delta **52.52µs**. Actual original inclusive assertions remain 35ms average, 75ms maximum, baseline+20ms average delta and passed. Capture preserves this observed PASS only; neither historical FAIL nor a claimed performance cause is erased or repaired.

## Cache metadata and observational boundaries

Current retail manifest SHA256 matches saved metadata/provenance: `aa7dfec3fb3bc9440a8c737c2274363502cb3d22c7939b8a570a58e717202edd`. Current provenance text exactly matches saved text: schema1, retail/wow, version12.1.0.69933, build_key dcfc90fffd79ba00406ae46f5f657592, source casc-local-or-cdn, fallback none, empty install_key. This attests metadata only, not all cache contents, archive/decode integrity, historical cache identity, or original sixteen-thread environment equivalence.

Existing source-only proof establishes markers outside original tick clock, unchanged setup/order/assertions, and logger wiring. Handler logging itself remains inside measured dispatch and can perturb latency. Prototype label is `proto.source:proto.line_defined`, not an inner hot line or unique invocation ID. Intervals can contain helper, owner/budget/error and nested work; nested separately logged callbacks can double-count. Quota exhaustion can yield attempted interval without executing Lua body. Visibility/mode/absent-binding filters prevent calls before dispatch. No OnPostUpdate records does not prove none registered or every dispatch entry point instrumented. Syntactically complete windows cannot independently prove absence of lost records. No causal diagnosis, performance fix, native parity, frozen BA, full-suite or broader acceptance inferred.

Proof artifacts written additively in this epoch only: authentication receipt, capture-independent-report.md, capture-independent-proof.json; independent reconstruction script retained for inspectability. Root source-only proof/report untouched.
