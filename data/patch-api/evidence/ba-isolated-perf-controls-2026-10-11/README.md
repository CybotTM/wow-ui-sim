# Frozen ba isolated performance controls — two bounded PASS cases

Epoch `20261011T220436Z`; [independent assessment](report.txt) authenticates one sealed copy of the existing ba integration executable and two sequential exact controls, without Cargo rebuild, threshold changes or environment overrides. Count each parent/child recap once: **two logical executions**, separate from [loaded suite FAIL](../query-final-suite-result-2026-10-11/README.md).

| Exact control | Displayed isolated measurements | Unchanged budgets |
|---|---|---|
| Startup |3.68s total; addons2.78s, post-load246.54ms, events506.28ms |20s |
| Talent headless tick |baseline441.52µs, avg501.18µs, max595.91µs; rounded delta59.66µs |avg35ms, max75ms, delta20ms |

Both saved unit observations report active/exited0, RuntimeMax90s, KillModecontrol-group, TimeoutStop3s, CPUQuota1200%, MemoryMax16GiB, agents.slice. Post-control snapshot reports no remaining processes. No timeout occurred; this does not claim timeout cleanup was exercised. Source checkout remains frozen ba; actual/historical environment values and vendor/cache provenance were not sealed. No exclusive-lock or matched-input claim.

[Manifest](retention-manifest.json):15 exact metadata/stream/forensic-report copies,31,119bytes; no executable retained. Report's original private references are provenance, not current public navigation. Both runs use the same recorded sealed binary; no repeated binary authentication for the second case.

[Investigation](../../../../docs/wiki/investigations/fullsuite-ba-2026-10-11.md) preserves startup96.24s and talent delta24.47ms failures in the loaded suite, plus the separate CASC lookup failure. Isolated PASS proves neither resource causality nor repair. No broad rerun; full handoff OPEN.
