# Independent artifact verification

**PASS — bounded discovery/cleanup claim.**

Epoch: `/home/osso/.local/state/wow-ui-sim/verification/dev-harness-new-input-current/20261011T053714Z`

Recorded revision: `afaf8da61db0b6794dd42ba01d24fe51d78516b4`

Baseline: `/home/osso/.local/state/wow-ui-sim/verification/dev-harness-green-current/20261011T051913Z/retail`

PASS — one temporary top-level tests/dev_harness_discovery_probe.rs; worker/request/controller agree. Original nested fixture reconstructed with only function-name replacement matches recorded SHA-256 382923398dea386f8cd13eb6995828d3953d0cd60a34f7a9a83947bab9c68fdc. Assertions preserve UIParent, WorldFrame, 1024x768.

PASS — probe prefork 2326 versus baseline 2325: only added name dev_harness_discovery_probe::new_marker_271828, exactly once; no removed names or duplicates. Integration 10718 unchanged, marker absent.

PASS — recorded exact-selector run exit 0: 1 passed, 0 failed, 1 total; three additional zero-case result groups. Full stderr records actual simulator startup; this is not a listing-only or empty-fixture pass.

PASS — cleanup prefork 2325 and integration 10718 exactly restore baseline name sets, with zero marker occurrences; all four listing exits 0. Probe physically absent. All eight persistent input SHA-256 seals match before, cleanup, and current files.

PASS — both compile exits 0, each 742 complete JSON records and build-finished success=true. Root library plus six binaries fresh=true (reused, not newly rebuilt); both test targets fresh=false. Six iced-wgpu manifest deprecation warnings per stage; no compiler-message records.

PASS (recorded metadata) — worker restores tests directory atime/mtime using saved nanoseconds; source-cleanup.json records tests_directory_mtime_restored=true. Numeric before/after timestamps were not persisted; independently rechecking the historical equality is unavailable.

## Exact runtime stdout

```text
running 1 tests
test dev_harness_discovery_probe::new_marker_271828 ... ok

test result: ok. 1 passed; 0 failed; 1 total
running 0 tests

test result: ok. 0 passed; 0 failed; 0 total
running 0 tests

test result: ok. 0 passed; 0 failed; 0 total
running 0 tests

test result: ok. 0 passed; 0 failed; 0 total
```

## Proof boundary

Recorded source/compile/runtime discovery and cleanup only; no new policy, native parity, full-suite, or full-profile closure. No builds, reruns, operations, delegation, shell commands, source edits, or retention mutations performed. Only independent-report.md/json written.

Full compiler JSON (742 records/stage), all listing lines, runtime stdout/stderr, worker/request/controller, before/after seals, and cleanup artifacts read in full. JSON report contains per-stage targets, independently recomputed listing deltas, seals, and artifact digest ledger. Temporary source is proven by worker plus matching reconstruction digest, not a retained snapshot. Historical numeric directory timestamps are unavailable.
