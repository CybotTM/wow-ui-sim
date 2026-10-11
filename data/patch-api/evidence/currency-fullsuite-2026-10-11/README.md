# Currency integration full suite — FAIL

Tested `1dcc59c0788d0e09ee3d02516123df965ba8dc63`. [Saved-artifact gate](gate/report.md) authenticates accounting and repository source equality, not native or executable provenance. Overall **15,028 PASS / 1 FAIL / 19 skips / 15,029 executions**. Prefork main has 2,320 PASS/1 FAIL; auxiliary batches add 2+1+1 PASS once. Integration has 10,703 PASS and library 2,001 PASS. All four new shared currency regressions PASS within these totals.

Compared with [historical c75](../../../../docs/wiki/investigations/fullsuite-c75-2026-10-11.md): four additional PASS executions, no removed PASS selector, same named Garrison failure at the same assertion. This does not prove causal identity, absence of other regressions or skipped/native behavior. Mutable `new_failures` is not regression authority. Garrison authentic native rows remain unavailable; no guessed catalog or loader repair follows.

[Result](suite-result.json) records phase exits0/1/0 and durations1417.1/123.9/133.8s. Captured timestamps span2026-10-11T02:47:33–03:15:28-0500. Actual nextest argv uses16 threads; configured compiler jobs, achieved utilization and runner durations are distinct. No broad replay for later docs/data changes.

[Retention manifest](retention-manifest.json): five exact UTF-8 copies,21,193 bytes. [Count summary](count-summary.json) is explicitly derived from private full countproof, not an exact copy. Raw logs, outcome-row inventory, binaries and session/provider payloads remain private. The retained gate's `countproof.json` reference names its private original; it is not a missing claimed packet copy.

[Matrix endpoint audit](matrix-gate/report.md) accepts the earlier33-copy partial-profile packet at `2b1ab710d`; its historical async wording predates this suite reconciliation. Four configurations'68 targeted executions/20 names remain distinct from broad suite totals. Era resource-blocked, Anniversary/Forever unattempted; vendor-input/native/catalog/external/full-handoff acceptance remains open. [Contract](../../../../docs/specs/currency-list-field-projection.md).
