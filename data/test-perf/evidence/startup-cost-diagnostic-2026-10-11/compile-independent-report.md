# Independent compile/source gate — PARTIAL_COMPILE_SOURCE

Epoch: `compile-20261012T040530851181Z`. **Behavior PENDING; fix/acceptance NOT established; goal OPEN.**

## First authentication

[First-auth receipt](compile-independent-auth.json) was written immediately after the verifier's single streamed read of this epoch's `probe`, before long output/reporting. Computed **349,260,352 bytes**, SHA256 **1ad974424c3f89b2e82f4a3e3e2e3380e775c0e0b6058638fd0bdefe0318dae1**. Both comparisons match. No other binary/library payload read or hashed. **Future final behavior verifier MUST reuse this first receipt, never rehash.**

## Evidence matrix

| Boundary | Evidence | Proof level |
|---|---|---|
| New wrapper compilation | Saved `compile-result.json`: exit 0, source_unchanged=true; full compile stdout/stderr both empty; outcome COMPILED_NOT_EXECUTED, executed_cases=0 | Compile/link only |
| Fresh compile admission | 2026-10-12T04:05:30.869348+00:00: admitted=true, load1=21.81, available RAM=18.49740219116211 GiB, relevant Cargo=[] | Saved point-in-time admission |
| Frozen/current diagnostic sources | Epoch probe equals private probe; probe.rs (7,828 bytes), worker.py (5,565), runtime-worker.py (2,106) each equal current HEAD packet and working packet | Exact named source bytes only |
| Runtime preparation | Exact final compile_probe() call removal plus runtime worker; exact request definitions prefix and runtime-only suffix | Static composition, no compiler invocation |
| Runtime observation | 2026-10-12T04:06:15.407649+00:00: load1=28.26, RAM=22.182044982910156 GiB, Cargo=[]; admitted=false; submitted=false; executed_observations=0 | RESOURCE_BLOCKED; no behavior proof |

## Compile submission and provenance

Saved toolchain exit 0 reports `/usr/bin/rustc` 1.99.0 (b940084d7 2026-09-28), Arch 1:1.99.0-1, x86_64-unknown-linux-gnu, LLVM 23.1.1. No fresh toolchain execution. Actual submission uses edition 2024, opt-level=1, clang/mold, target-cpu=native, linker thread count 4, explicit `wow_ui_sim`/`serde_json` --extern paths and dependency directory; private epoch `probe.rs` is input and epoch `probe` is output. See [complete submission](compile-20261012T040530851181Z/compile-submission.json) and [proof JSON](compile-independent-proof.json).

Compile controller submission exited 0; full stdout is a completed record whose executed payload exactly equals current worker.py, also equal to the compile request. Console names this exact epoch; controller stderr is empty. Controller success alone is not compiler success; the separate compile-result/streams supply that boundary. Submission config: asynchronous agents.slice, CPUQuota=1200%, MemoryMax=16G, KillMode=control-group, no build deadline. Configuration is not independent observed enforcement.

Selected library Cargo records independently match the saved producer Cargo log exactly; current path/stat matches saved selection:

| Selected library path under `/home/osso/Projects/wow/wow-ui-sim/target/debug/deps/` | Bytes | mtime_ns | Saved Cargo fresh |
|---|---:|---:|---|
| libwow_ui_sim-65495b7501c08485.rlib | 430762454 | 1791773709051369162 | false |
| libserde_json-d1222ed4258e331c.rlib | 2871460 | 1791648084346200629 | true |

Both Cargo profiles are opt-level 1, debug assertions/overflow checks enabled, line-tables-only, test=false. wow_ui_sim has client-retail/default/gui/sound/casc and retail feature epochs through 12-1-0; serde_json has default/std. Full records/features are retained in the proof. **Path/stat and Cargo projection are not payload authentication.** No library payload read/hash. Library producer revision `262987b1a65f049d5c3e158b18a8eb73cc22e4ba`, compile HEAD `a135e01a25718b7aacc7f169bcd60ecf070b1335`, current inspected HEAD `3a7c2f6a3475322011e7777adb91ad06dfd90119` differ. Wrapper compile/link proves this invocation's compatibility, not a hermetic current-source rebuild or frozen-ba acceptance identity.

## Runtime request boundary

Inspected actual `runtime-request-20261012T040615389243Z.jsonl` as data, not executed. Its code is exactly runtime-definitions.py followed by `print(submit_runtime(Path('<this epoch>')))`. AST top-level statements are imports, constants, function definitions, and that single runtime expression (line 110). compile_probe remains only an uncalled definition; definitions have no decorators/default calls that invoke it. Composition equals worker.py stripped **only** of final standalone compile_probe(), followed by runtime-worker.py. This resolves the earlier preflight's unsafe whole-worker invocation finding for this exact request, without changing compile source.

Saved runtime guard denied load; only timestamped runtime-admission/runtime-blocked receipts exist in the epoch, no runtime-submission/stdout/stderr. Runtime NOT submitted. CPU/memory/90-second/whole-control-group limits are source configuration, not observed enforcement. Guard is point-in-time, duplicate refusal sequential; no concurrency/resource reservation proof. Diagnostic environment disables CASC/sound/third-party addons/SavedVariables and enables profiling; not benchmark/control equivalence.

## Reused source boundary and exclusions

Reused [preflight report](preflight-report.md), [source/copy proof](preflight-proof.json), [readability](readability.json), [format](format.json), including earlier 9/9 copyproof. No repeated source audit, formatter, hashes, old builds/checks. Historical preflight absence/unsafe-composition claims are superseded only by this epoch's fresh compile and exact runtime composition evidence; other source/timing/error-attribution limits remain. Named diagnostic byte equality does not prove whole repository/current production-source equality, runtime readiness, exclusive timing attribution, an error-free startup, threshold compliance, or a fix. “Frozen source” means the saved epoch snapshot, not filesystem immutability.

Only three authorized files written. No guard/runtime/process polling/network/delegation/ops/tests/Cargo/rustc/format execution. See proof ledger and incidents in [compile-independent-proof.json](compile-independent-proof.json).
