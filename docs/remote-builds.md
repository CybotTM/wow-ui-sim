# Native desktop/local development

Normal builds **and runs** use `scripts/build-host.py`. `desktop` means native Rust and runtime in desktop WSL as `osso-test`; `local` means both on this machine. Choose local while gaming on desktop. Raw `cargo` commands remain local.

## Commands

```sh
python3 scripts/build-host.py --run
python3 scripts/build-host.py --build-host local --run
python3 scripts/build-host.py --bin wow-cli --run -- --no-addons --no-saved-vars lua-errors
python3 scripts/build-host.py --check
python3 scripts/build-host.py --test --test integration FILTER -- --nocapture
python3 scripts/build-host.py --no-default-features --features sound,gui,casc,client-ptr --check
python3 scripts/build-host.py --no-default-features --features sound,gui,casc,client-ptr --test --test integration FILTER -- --nocapture
```

`--test` is the last helper option: everything after it goes to Cargo, including test filters and the test-executable separator. `--check` checks the selected binary; `--bin wow-cli` selects the CLI. Alternate profiles disable default features. `--release` selects release output; `--no-build --run` runs an existing selected-host binary without Cargo.

```sh
scripts/start-profile.sh live
scripts/start-profile.sh --build-host local ptr
scripts/start-profile.sh all
```

The Python profile launcher preserves `live`/`retail`, `ptr`, `mists`, and `all`, selecting profile-specific Cargo features. `WOW_SIM_START_RELEASE=1`, `WOW_SIM_START_FOREGROUND=1` (single profile only), and `WOW_SIM_START_NO_BUILD=1` retain their meanings. Background logs and helper PIDs appear in `target/profile-runs/`; a printed PID is not GUI readiness proof.

## Host selection and dependencies

`python3 scripts/build-host.py --save-build-host desktop` saves the shared default in `~/.config/game-engine/build-host`; use `local` to change it. The configured default is desktop. Explicit `--build-host` overrides it for one invocation. Host failure is an error, never a host fallback.

Shared snapshot, selection, and native execution helpers live in `/syncthing/Sync/Projects/world-of-osso/game-engine/scripts/`. Set `BUILD_HOST_SCRIPTS` to override that dependency. Desktop uses pinned Rust 1.98.1 through rustup and same-ABI native runtime dependencies; local uses installed Arch Cargo/rustc without rustup, not fallback; observed 2026-10-10: `/usr/bin/cargo` and `/usr/bin/rustc` 1.99.0 (installed state, not a local pin). Record actual `cargo -V` and `rustc -Vv` for build evidence; there is no Docker runtime or bundled ICU substitution. Existing CI/release Docker pipelines remain separate and unchanged, including the [standalone PTR ICU workflow](ptr-icu-build.md).

## Source, cache, and lifetime

Desktop source and Cargo targets persist under `/home/osso-test/.cache/native-builds/<key>/`. Snapshots include current working edits and selected owned compile/runtime inputs, not only committed source. `crates/` includes internal workspace packages such as the dev-only test harness: Cargo resolves their manifests even for binary-only builds. The helper is not thereby a production compilation dependency. [Staged input proof](../data/test-perf/evidence/dev-harness-build-inputs-2026-10-11/README.md) records missing-manifest failures and corrected local workspace resolution; it is not a desktop build/transport pass. Previously supplied files are pruned without deleting app-owned runtime directories; content changes receive fresh mtimes, including A/B/A changes. Builds do not hold a runtime lock for the app's lifetime.

Runtime Blizzard caches remain host-local and profile-scoped under `~/.cache/wow-ui-sim/blizzard-ui/<profile>/AddOns`; desktop resolves that home as `osso-test`. Host selection does not populate or migrate those caches. SSH leases control remote cleanup. Source SCP has a 60-second attempt limit and at most three transient attempts; permanent failures fail immediately.

### Local admission CPU scope

Measure CPU usage and capacity over the same cgroup scope. On October16,2026, local `agents.slice/cpu.max` was `max 100000` and main `nproc` reported32 CPUs; a measurement service with `CPUQuota=20%` reported1 from `nproc`, while `nproc --all` reported32. The then-current local capacity helper paired slice-wide CPU counters with a `nproc` denominator when the slice quota was unlimited, so a smaller polling-service quota produced a misleading `agents 9.9/1 CPU` verdict. This is not evidence that the slice has a one-core limit.

Main temporarily removed only its polling unit's quota, preserving its PID, low priority, bounded memory and normal queue/memory/pressure admission; actual build limits and guards stayed separate. The shared helper was subsequently changed by the other main session to use `getconf _NPROCESSORS_ONLN` for an unlimited host-wide slice, retiring the need for that wrapper workaround. Main inspected the changed quota expression and observed `getconf` returning32 in the same20%-quota service boundary. This is bounded source/probe evidence, not a complete shared-helper gate. Receipts: `/home/osso/.local/state/wow-ui-sim/handoff/iconbackground-capacity-scope-main-disposition.md` and `/home/osso/.local/state/wow-ui-sim/handoff/capacity-getconf-shared-fix-20261016T112030751633Z.json`. No FIFO bypass or host-slice policy change; no Cargo bottleneck or jobs12 speedup claim.

## Evidence and pending runtime proof

Main-observed evidence on October 3, 2026: actual native desktop GUI binary build passed (4m9s); headless CLI startup `--no-addons --no-saved-vars lua-errors` returned `[]`, exit 0 (`target/native-desktop-lua-errors.log`). These are bounded build/CLI observations, not full parity or warning-free acceptance.

Normal desktop GUI failed before its first frame: `target/native-desktop-gui-long.log` ends with connection reset (101) while creating the Wayland event loop. Engine `target/native-wslg-boundary.log` records the correct `/run/user/1000/wayland-0` symlink to `/mnt/wslg/runtime-dir/wayland-0`, repeated Weston SIG11 in `stderr.log`, and repeated approximately 102-second RDP peer disconnect/restart cycles. Windows msrdc ran in Session 0 while the active console was Session 1. Host compositor crash is observed; crash root cause and session causality are not established. This is not a simulator Lua/CASC/font stall. No WSL, service, or process restart was performed.

The actual local `wow-cli` helper built and printed `--help` in 0.35 seconds: bounded local CLI proof, not GUI acceptance or a local server pass. Local capability remains supported; actual local runtime acceptance is deferred, not required now for the current gate. User rejected local server build attempts; actual local server proof is deferred and its no-local-compile guard remains. Corrected source gate passed: two repaired fixtures, zero Ruff diagnostics across 14 files, and pycompile for nine changed files. The prior 94 passing fixtures retain revision-scoped credit; this is not a fresh 96-test run. Evidence: engine `target/native-migration-corrected-verify.md`. Historical focused-test/check reports are not a fresh current-tree gate. Normal desktop GUI remains unresolved; source-gate success would not complete the whole workflow.

### Measured jobs12 compile — October 10, 2026

Reported Cargo timing: **117.5 s**, optimized test-profile `--lib` + `--test integration`, revision `d80ee1b7f5fc0aed8e868c593dc84c71ca3ef817`. Library: **60.3 s** (frontend **17.09 s**, codegen **43.21 s**); distinct library-test harness: **104.3 s**; integration: **52.92 s**. Concurrent unit durations are not additive wall phases. Prior **125.2 s integration-only** measured different work/source; no normalized speedup or measured optimization is claimed.

Cargo job limit **12**, observed peak **8 concurrent Cargo units**; cgroup ceiling **12 CPUs**, consumption **985.701681 CPU-s** (~**8.39 CPUs coarse average** over 117.5 s), memory peak ~**13.45 GiB**. Process samples are partial: their elapsed clock has its own monotonic origin, with no established Cargo-start offset or compiler-phase attribution. These observations do not establish saturation or a bottleneck.

Toolchain label: **reported timing-HTML rustc 1.99.0**, not independent Rustc/ABI proof or hermetic dependency provenance. See the [existing compile brief](rust-integration-test-target-size-brief.md) and [sanitized timing summary and hash receipts](../data/test-perf/evidence/jobs12-map-display-2026-10-10/summary.json). Native goal remains open; this retention adds no acceptance gate.

### Mists trace compile — separate jobs12 observation

[Timestamped Cargo HTML summary](../data/test-perf/evidence/jobs12-mists-trace-2026-10-10/summary.json), start October10 at22:22:48UTC, independently admits **148.2s wall**, **737 units /728 fresh /9 dirty**, and a **130.25s library** (69.05s frontend +61.20s codegen). Seven final targets overlap after library completion; peak compiler-unit concurrency is **7** with jobs12. Benchmark-named bins record compilation, not benchmark execution.

The 299 CPU samples average53.1513 and peak78.5714 on Cargo's report scale; CPU-core denominator and core utilization remain unknown. Compiler version1.99.0 is HTML-recorded metadata. This HTML-only admission establishes neither invocation/source endpoints/exit status nor a controlled jobs12 speedup; earlier build evidence remains separate. [Retention hash](../data/test-perf/evidence/jobs12-mists-trace-2026-10-10/hash-manifest.json).

### Five-profile saved compile receipts — jobs12, different workloads

[Independent compiler-message report](../data/test-perf/evidence/jobs12-five-profile-2026-10-10/report.md) admits each profile's **663 artifact records:655 fresh/8 nonfresh**. Eight nonfresh targets are one library, **six binaries**, and one integration test; benchmark-named binaries were compiled, not benchmarked. Submission-to-compile-end wall seconds: Wrath190.922430, Mists106.577779, Era128.349182, Anniversary142.407461, Forever73.417071. Compile exits0; six dependency manifest deprecations each.

Features differ (including Mists prefork and Forever capabilities); recorded revisions also differ. Source/artifact admission and ordinary runtime proof remain separate. These intervals establish no CPU attribution, matched cache state, controlled speedup or whole-profile acceptance. [Report hash](../data/test-perf/evidence/jobs12-five-profile-2026-10-10/hash-manifest.json).

### Required unit2 builds — measured cgroup CPU window

[Seven-profile cost/resource evidence](../data/test-perf/evidence/unit2-jobs12-2026-10-10/final-report.md) retains already-required jobs12 builds: each663artifact records/655fresh/8nonfresh, one library+sixbins+integration. Compile intervals sum552.436541s; features/revisions/cache states differ, no controlled speedup comparison.

Actual cgroup counters yield5.70–6.45 coarse logical CPU equivalents per compile window under a12-CPU quota, including compiler/worker stream handling, not frontend/codegen or exclusive-rustc attribution. Recorded memory limit16GiB; cumulative unit peak reaches about16GiB after PTR, not per-profile peaks or proof of OOM. Throttling counters have no assigned cause. No resource-limit change or extra build experiment.

### Remaining currency profiles — bounded jobs12 observation

[Saved retry2 packet](../data/patch-api/evidence/currency-seven-profile-2026-10-11/README.md#compilation-observation) records already-required Era/Anniversary/Forever builds:131.890/135.727/143.911s submission-to-compile-end, each665 artifacts/656fresh/9nonfresh (build script, library, six bins, integration). Six dependency manifest deprecations per lane remain. These are different profile/cache workloads, not a controlled speedup comparison.

A completed19-snapshot sampler observes90.178s across late Era/early Anniversary:241.883cgroupCPU-s,2.682CPU equivalents and maximum4rustc processes sampled. Interval includes worker/linker/test work; peak is a lower bound, not actual concurrency maximum. No whole-build utilization, twelve-busy-core exclusion or causal bottleneck claim. Earlier aligned unit2/timing-HTML evidence remains separate; no extra rebuild was submitted for sampling.

### Approved dev-only generation boundary — bounded saved proof

The [harness contract](specs/prefork-test-harness.md#dev-only-automatic-harness-generation-approved-2026-10-11-verification-pending) requires automatic integration/prefork generation in an internal dev-only crate, with no test generation/watches in the root production build script. This changes cache invalidation, not native host selection or Cargo's underlying six-binary integration selection; no unsupported CLI exclusion flag is part of the change.

Source `256e54ef1` extracts generation/watches into unpublished dev-dependency `crates/test-harness`, leaving production responsibilities in root `build.rs`. Two item macros populate integration/prefork module trees; an expression macro supplies the Retail registry. Explicit workspace builds may select the helper; Cargo.lock retains its root watch.

[Retained local evidence](../data/test-perf/evidence/dev-only-harness-2026-10-11/README.md) admits Retail/Mists compile/list parity, 13 selected cases, two original Retail prefork bodies, generated-inner-byte preservation, format exit 0 and default-profile check exit 0 with existing warnings. The existing-test timestamp trial keeps the root library and six binaries fresh, rebuilding only helper library and integration; native output snapshots are unchanged.

A separate local temporary-marker discovery/execution/cleanup control passes, and cached Retail stock startup reports zero Lua errors with audio diagnostics. These results do not establish desktop cache invalidation or native-client acceptance. Changed revisions/cache graphs prohibit normalized speedup or CPU-saturation claims. Historical Mists cast-bar RED, full-suite failures, desktop GUI failure and broader handoff gaps remain open.

## Asynchronous full suite

`python3 tools/full_suite.py submit [REF] --profile PROFILE --target-dir SLOT --build-jobs 1`
schedules one detached profile worker. `PROFILE` is `retail` (default), `ptr`,
`wrath`, `mists`, `wowforever`, `era`, or `anniversary`.
`python3 tools/full_suite.py status [REF]` lists all seven profiles for the resolved
SHA; without REF it lists the latest receipt's SHA. Missing profile receipts are
JSON `null`, not passes. The worker keeps
its existing suite lock and dedicated checkout/results under `~/Projects/wow/`.
The results `.lock` serializes revision resolution, checkout preparation and all
applicable child lifetimes, including unsuccessful exits. Each step invokes native
Cargo directly; the obsolete external builder-lock wrapper is not a dependency
and its lock does not block these children.

Both `submit` and `run` require `--target-dir SLOT`: an absolute path resolving
to one of the four approved cache slots in [build policy](../AGENTS.md#development-buildrun).
Canonical and suite checkout `target/` paths, non-slot paths, relative paths, and
slots redirected outside the approved set by symlinks are rejected before launch
or checkout. Select an available, coordinated slot; validation does not reserve it.

`--build-jobs N` accepts positive integers and defaults to **1** for both actions;
there is no caller-environment build-job default. Submit passes build jobs and the
resolved target as explicit worker arguments. Every Cargo phase receives `-jN`,
`CARGO_BUILD_JOBS=N`, and `CARGO_TARGET_DIR=SLOT`, overriding inherited values.
Cargo steps retain `--offline --locked`; dependencies must already be cached and
the lockfile current. `FULL_SUITE_JOBS` controls nextest test workers only
(default 16), independently of compilation jobs, and submit forwards its chosen
value explicitly via `systemd-run --setenv`. Complete
integration and library scopes use nextest with `--no-fail-fast` for every profile.
The complete `prefork_full_ui` scope uses `cargo test` only for Retail/Mists,
matching `prefork-full-ui` admission in Cargo.toml and the target's compile guards.
Other profiles record `steps.prefork.status = "not_applicable"` with a reason and
an N/A log entry, without an exit code or fabricated failure/pass record.
Nonzero Cargo exits, parsed failures and both stdout/stderr remain in the stored
JSON/log; later applicable steps still run.

Retail deliberately retains Cargo default features (`sound,gui,casc,client-retail`)
with unchanged child argv so historical Retail receipts remain comparable.
Every other profile passes `--no-default-features --features sound,gui,casc,client-<profile>`
to every applicable Cargo command. No extra prefork feature is forced.

Receipts and logs are `<sha>-<profile>.json` and `<sha>-<profile>.log`; each receipt
and worker summary identifies `profile`. Master runs update only
`master-latest-<profile>.json`; NEW failures compare only to that profile's baseline
(or JSON `null` when absent). Historical `<sha>.json` Retail receipts remain
readable in status under `legacy_retail`, separate from new profile receipts;
historical `master-latest.json` is neither overwritten nor used across profiles.
Submit forwards the profile explicitly; systemd unit names include it.

Submit each desired profile separately against the same final SHA and a coordinated
approved slot; the existing suite lock serializes them. No `--profile all` matrix
mode: retain the single-profile worker/receipt contract rather than add aggregate
orchestration. Seven receipts establish seven-profile coverage, not one aggregate
acceptance receipt; review each profile's applicable scopes and failures.

Source revision `3eab34fb9` adds two fields to each executed step's JSON:
`command` is the exact argv passed to the subprocess, including `--offline`
and `--locked` where passed; `environment` contains only
`WOW_SIM_TIMEOUT_FIXTURE_CAPACITY`. Its value is JSON `null` when unset,
`""` when empty, or the literal string (including `"reserved"`). No other
inherited environment is recorded. [Bounded metadata proof](../data/test-perf/evidence/full-suite-result-metadata-2026-10-12/README.md) covers three new cases, six existing sandbox controls and the installed controller's source-byte equality/mode0755. No suite was rerun. Future metadata cannot reconstruct historical environment or establish contention, failure cause or current full-suite acceptance.

Submit resource policy aligned October 10, 2026: detached `systemd-run --user`
uses `agents.slice`, `CPUQuota=1200%`, `MemoryHigh=16G` and `MemoryMax=16G`,
matching the bounded native units' operational CPU/memory limits. Unlike the guarded units in the
[bounded native-unit handoff](wiki/investigations/integrated-source-and-factory-proof-2026-10-09.md#current-bounded-native-workflow-correction--2026-10-10),
this controller has no available-memory/load admission guard. That remaining
mismatch is reported, not redesigned; submission still returns without waiting
for the worker.

Bounded worker regression tests (temporary git/Cargo executable fixtures and real
flocks, no actual builds or systemd) prove children execute while the old builder
lock remains held, independent jobs/flags, all-scope failure/stream retention,
and controller serialization before checkout and through child execution:
`python3 -I -m unittest discover -s tools -p test_full_suite.py -v`.
Fake-systemd coverage checks detached worker argv/resource limits, explicit
build-job/target forwarding, default jobs and all four slots; rejection coverage
checks both submit and worker paths, including symlink redirection. Installation
coverage writes only to a temporary home.

Main deploys with `./deploy.sh` from the repository root (a Python installer,
not Bash). It installs only `tools/full_suite.py` as executable
`~/bin/full-suite`; no simulator build, service restart or suite submission.
Tracked edits alone do not update the installed entrypoint.

## Related

- [Build-host contract](specs/build-host.md)
- [Agent commands](../AGENTS.md#development-buildrun)
