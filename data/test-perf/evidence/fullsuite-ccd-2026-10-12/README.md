# Frozen ccd full suite — terminal FAIL

Exact source: `ccd526c6c9f783cf7d3fe15eda05c42c6cabab59`. Recorded October 12, 2026, 22:59:10–23:43:46 CDT. **15,204 PASS / 7 FAIL / 19 skips / zero runner timeouts**: 15,211 executed invocations plus 19 skips. Counts describe this frozen run, not current source or unique capabilities.

| Suite | Passed | Failed | Skipped | Exit |
|---|---:|---:|---:|---:|
| Integration | 10,877 | 5 | 19 | 100 |
| Prefork primary + auxiliary | 2,326 | 2 | 0 | 1 |
| Library | 2,001 | 0 | 0 | 0 |

## Frozen failures

Integration:
- `addon_loading_perf::loading_phase_breakdown_stays_within_budgets`
- `animation_query_lifecycle::bounce_loop_callback_observes_direction_change`
- `animation_query_lifecycle::delaying_uses_order_start_and_longest_parallel_duration`
- `patch_10_2_5_probe_factory::patch_10_2_5_vertex_color_probe_reaches_missing_endpoint_methods`
- `startup_perf::full_game_startup_stays_under_budget`

Prefork:
- `patch_3_3_0_publication_sweep::patch_3_3_0_publication_sweep`
- `patch_4_0_1_publication_sweep::patch_4_0_1_publication_sweep`

Lua compilation measured 3.13s against 2s; startup 27.63s against 20s. Their timeout-child exit101 diagnostics are assertion failures, not runner timeouts. Causes remain unresolved; budgets unchanged. Six inherited manifest deprecations recur across steps. Skip identities/reasons are unavailable. Wrapper acceptance does not override failed suite exits; `new_failures` does not establish historical novelty.

## Retained evidence

- [Result](result.json): exact terminal JSON.
- [Artifact report](artifact-report.md): sanitized prior independent accounting, failures and limits.
- [Selected output](selected-output.log): summaries, warnings and seven failure boundaries, not full raw-log/map parity.
- [Excerpt provenance](excerpt-provenance.json): original text-stream hashes, inclusive ranges and redacted-line identities.
- [Retention manifest](retention-manifest.json): public text/data identities; validation remains separate from suite outcome.

Later selected lifecycle6 and renamed Vertex factory1 pass at `ff72f3c3b`; that bounded result does not rewrite this frozen FAIL or prove newer producers. Corrected prefork fixtures and performance acceptance remain open. [Integrated SSOT](../../../../docs/wiki/investigations/integrated-source-and-factory-proof-2026-10-09.md) owns subsequent source/execution boundaries. Whole handoff remains OPEN.
