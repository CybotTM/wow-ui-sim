# Startup diagnostic preflight — PARTIAL preparation only

Revision inspected: `28eae9f08b9de09d14472f210a27f134d67fc26e`. **Compile proof PENDING; behavior proof PENDING; goal OPEN.** No probe artifact, compile-result, artifact receipt, runtime submission or runtime output exists in this private root. Controller exit 0 is not rustc success or test PASS.

## Preparation evidence

- **Sequence: statically aligned.** [Probe](probe.rs):86–124 follows [base_game.rs](../../../../../Projects/wow/wow-ui-sim/tests/perf/base_game.rs):6–23 and [startup_perf.rs](../../../../../Projects/wow/wow-ui-sim/tests/startup_perf.rs):26–66: new environment → 1024×768 → Game screen → active-profile UI path as the sole addon base path → Game addon discovery → ordered iteration/load using `env.loader_env()` → post-load workarounds → Game startup events. Hardcoded repository root resolves the same input as this checkout's `CARGO_MANIFEST_DIR`; portability elsewhere is not proven. Diagnostic discovery has a separate clock and an extra path resolution; this is not identical timing overhead to the benchmark. Readiness uses the same Lua expression after the enclosing clock stops. No settling ticks, talents, acceptance assertions or startup budget replacement. The benchmark's 20-second threshold remains in the unchanged reference file.
- **Counters/boundaries: statically coherent.** [Probe](probe.rs):42–84,128–218 sums cumulative snapshot records/counters, retains only records appended after `before.records`, and rejects a shrinking snapshot. Phase clocks include only each operation; before/after snapshots and row storage are outside them. The enclosing clock includes observer overhead and stops before readiness, serialization and output. Existing [loader snapshot implementation](../../../../../Projects/wow/wow-ui-sim/src/loader/mod.rs):76–126 is append-based and clone-on-read; [Lua file timing deltas](../../../../../Projects/wow/wow-ui-sim/src/loader/lua_file.rs):105–136 are dynamic-scope records. Aggregate counters/new-file sums are not proven exclusive attribution: nested activity can overlap. No runtime JSON rows were produced or behaviorally checked.
- **Manual Rust readability:** independently read all 218 probe lines against the rust-readability skill. No actionable violations found: named setup/orchestration/measurement/serialization boundaries, shallow control flow, contextual returned errors, no warning suppressions, no TODO/FIXME/HACK/XXX markers. Mutable counter folding and observation retention represent the actual aggregation/output contract, not unnecessary state. Reused [format receipt](format.json), exact `{"exit": 0, "stdout": "", "stderr": ""}`; no formatter rerun. [Saved readability receipt](readability.json) is preparation evidence, not compiler evidence.

## Worker safety and blocked attempt

[Worker](worker.py):11–45 requires RAM≥6 GiB, load<24 and no relevant Cargo before reading source/saved library metadata or invoking rustc. Its matching request and completed-controller payload were compared as UTF-8 bytes. [Admission](compile-20261012T035859290370Z/admission.json) records `admitted=false`, RAM `22.001453399658203`, load `28.95`, relevant Cargo `[]`, at **2026-10-12T03:58:59.308652+00:00**. Load fails the guard. [Outcome](compile-20261012T035859290370Z/outcome.json) is `RESOURCE_BLOCKED`, `compiled=false`, `executed_cases=0`; the compile epoch contains only those two files. [Controller stdout](controller-20261012T035859120805Z.stdout) records completion of the blocked worker; stderr is exactly 0 bytes. [Submission](compile-controller-submission-20261012T035859120805Z.json) is asynchronous, agents.slice, CPU1200%, memory16G, KillMode=control-group. [Saved terminal](blocked-controller-terminal.json) says `ActiveState=active`, `SubState=exited`, `Result=success`, `ExecMainStatus=0`, empty ControlGroup: successful retained oneshot controller, not successful compilation. Receipt dates above are literal UTC values; no current-time/guard refresh performed.

[Runtime helper](runtime-worker.py):1–24 has a fresh admission call, existing-submission refusal, missing-binary refusal, async agents.slice submission, CPU1200%, memory16G, RuntimeMaxSec=90, TimeoutStopSec=3 and whole-control-group termination. Profiling is enabled; CASC/sound/third-party addons/SavedVariables are disabled. This is warm-cache, profiled diagnostic scope, not benchmark-environment equality or acceptance/control replay.

**Concrete retained limitations (main owns actions):**

1. Runtime helper is not standalone: it requires `datetime`, `capacity`, `save`, `cli` and `REPO` supplied by its caller. Importing/executing the whole worker to obtain them also calls `compile_probe()` at worker.py:85. No safe runtime-helper composition/submission receipt exists yet; do not infer runtime readiness from source alone.
2. Compile worker selects one saved Retail library per name and checks paths/stats, not payload identity or toolchain compatibility. Producer revision is explicitly older. Compilation/link compatibility remains unproven. New-source/new-artifact hashes in the unexecuted worker are not verifier evidence; no hashes or old binary/library reads were performed here.
3. Capacity admission is a point-in-time check, not a reservation; Cargo detection is limited to the two named cwd paths and `comm=cargo`. Runtime duplicate refusal is a sequential existence check, not concurrent arbitration. Scope remains one main-owned submission, not concurrent launch safety.
4. Compile controller has no build deadline, as documented. Runtime bounds are source-level configuration only, not observed enforcement. Submission exit 0 would still require actual runtime outcome/JSON proof.
5. Ready=false deliberately returns success; returned load/eval errors retain partial rows, but internal logged/callback errors are not promoted and panics/abort cannot flush rows. No completion or error-free-startup claim follows from an eventual probe exit alone.

## Tracked packet byte identity

Tracked packet: [README](../../../../../Projects/wow/wow-ui-sim/data/test-perf/evidence/startup-cost-diagnostic-2026-10-11/README.md). Its nine source/receipt copies all match their private originals exactly, using nonbinary byte comparison only:

| Relative packet file | Private source | Equal bytes |
|---|---|---:|
| probe.rs | [probe.rs](probe.rs) | 7,828 |
| worker.py | [worker.py](worker.py) | 5,565 |
| runtime-worker.py | [runtime-worker.py](runtime-worker.py) | 2,106 |
| format.json | [format.json](format.json) | 40 |
| blocked/admission.json | [admission](compile-20261012T035859290370Z/admission.json) | 149 |
| blocked/outcome.json | [outcome](compile-20261012T035859290370Z/outcome.json) | 79 |
| blocked/readability.json | [readability.json](readability.json) | 210 |
| blocked/blocked-controller-terminal.json | [terminal](blocked-controller-terminal.json) | 203 |
| blocked/compile-controller-submission-20261012T035859120805Z.json | [submission](compile-controller-submission-20261012T035859120805Z.json) | 1,110 |

Read-only git status showed only untracked `.code-index.db` and `verification/`; no tracked modifications. Packet membership was established by `git ls-files`; HEAD's stat shows only blocked receipts/docs for this commit. This is bounded preparation/source inspection, not a repository-wide audit or proof of compiled production state. Verifier wrote only this report and [preflight-proof.json](preflight-proof.json).

## Retained bounds

No runtime/guard execution, polling, Cargo, rustc, check/build/tests, formatter, hashing, sourcecache access, network, native/tool changes, production/vendor/threshold edits or delegation. Existing binary/library payloads untouched. Compile and behavior proof **MUST remain PENDING**; broader handoff remains OPEN. Main owns any follow-up admission, compile, runtime and acceptance actions.
