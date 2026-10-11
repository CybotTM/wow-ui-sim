# Wowforever saved-artifact final audit

**PASS — bounded supplied-state model lane: 16/16 named cases, compile and execution receipt exits 0.**

## Authenticated evidence
- Lane: `/home/osso/.local/state/wow-ui-sim/verification/currency-remaining-profiles-retry2-current/20261011T082636Z/wowforever`. stdout: `/home/osso/.local/state/wow-ui-sim/verification/currency-remaining-profiles-retry2-current/20261011T082636Z/wowforever/currency.stdout`. `test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 8328 filtered out; finished in 1.81s`
- Cargo JSON: `/home/osso/.local/state/wow-ui-sim/verification/currency-remaining-profiles-retry2-current/20261011T082636Z/wowforever/compile.stdout` — 741 records parsed, one integration test artifact, sole client profile `client-wowforever`, successful build-finished record; artifact features, profile and executable agree with receipt.
- Binary: `/home/osso/.local/state/wow-ui-sim/verification/currency-remaining-profiles-retry2-current/20261011T082636Z/wowforever/integration-test` — 440197912 bytes; SHA-256 `c707c19d9a7b40570ff36f339a1b3dc0c545e0d763a2a1e7c39e8588e5af040f`; independently hashed exactly once, receipt matches.
- Eight scoped inputs match parent/compile-before, execution/parent-after, current files and source revision `1dcc59c0788d0e09ee3d02516123df965ba8dc63`. Exact absolute paths and hashes in report.json. Build receipt revision `842098a4e497c76bbf86e813a2e15e12600178b3` is not a whole-tree/current-revision identity claim.
- Six iced_wgpu deprecated manifest lint keys; not warning-free. Runtime stderr: 176 Startup-only lines, not vendor startup acceptance. Saved receipt authenticates recorded exit 0, not a new execution.

## Parent outcome and bounded aggregate
`/home/osso/.local/state/wow-ui-sim/verification/currency-remaining-profiles-retry2-current/20261011T082636Z/outcome.json` now contains three completed lanes (Era, Anniversary, Wowforever), each 16/16 and exit 0; its results equal `/home/osso/.local/state/wow-ui-sim/verification/currency-remaining-profiles-retry2-current/20261011T082636Z/results.json`.

| Configuration | Successful executions | Proof |
|---|---:|---|
| Default Retail | 16 | Reused prior report |
| Mists | 20 | Reused prior report |
| PTR | 16 | Reused prior report |
| Wrath | 16 | Reused prior report |
| Era | 16 | Reused independent report |
| Anniversary | 16 | Reused independent report |
| Wowforever | 16 | This audit |

**48 remaining + 68 prior = 116 successful executions across seven configurations, 20 distinct names.** Four additional names run only on Mists. Prior reports are identified in report.json; no prior binaries rehashed. Era/Anniversary independent reports reused:
`/home/osso/.local/state/wow-ui-sim/verification/currency-retry2-era-final/report.json`
`/home/osso/.local/state/wow-ui-sim/verification/currency-retry2-anniversary-final/report.json`

Historical Era RESOURCE_BLOCKED attempts retain zero cases; their earlier other-lane pending statements were epoch-local, not current aggregate status. No historical reports changed.

## Partial sampler — FINISHED, not active
Recomputed from `/home/osso/.local/state/wow-ui-sim/verification/currency-remaining-profiles-retry2-current/partial-compile-samples.json` and matched `/home/osso/.local/state/wow-ui-sim/verification/currency-remaining-profiles-retry2-current/partial-compile-summary.json` plus `/home/osso/.local/state/wow-ui-sim/verification/currency-remaining-profiles-retry2-current/compile-derived-summary.json`:
**19 samples; 90.178s wall; 241.883s cgroup CPU; 2.682 CPU equivalents; observed rustc peak 4.**

Saved completion event in `/home/osso/.local/state/wow-ui-sim/verification/currency-remaining-profiles-retry2-current/sampler.stdout` says `sampler complete 19`. This supersedes the stale “2196 active” claim; no live PID/service inspection or operation performed.

Interval: 2026-10-11T08:28:11.975131+00:00 to 2026-10-11T08:29:42.152126+00:00. Covers final Era / early Anniversary and worker/test gap, **not Wowforever compilation**. Partial interval only; cgroup CPU includes worker/linker/test work; sampled peak is a lower bound. Not whole-build utilization, twelve-busy-core exclusion, jobs12 speedup or causal performance proof.

## Acceptance boundaries
Native currency catalog, vendor/profile startup and all-profile handoff remain **unproven**. Source-lane blocks remain unchanged; BCC and AllowLoadGameType remain unknown. Eight input hashes do not authenticate unlisted dependencies/fixtures or whole build provenance. No reruns, builds, delegation, vendor loading/acquisition, operations, cwd changes or source edits.
