# Frozen ba full-suite result — 2026-10-11

[Saved packet](../../../data/patch-api/evidence/query-final-suite-result-2026-10-11/README.md) independently authenticates the frozen `ba413e8e177f04dc6fc7491834606ce36c750954` execution: **15,038 PASS / 3 FAIL / 19 skipped / 15,041 executed**. Unit `full-suite-1791749250.service` completed at20:48:40UTC; runner0 is not test success. Broad acceptance FAIL, full handoff OPEN.

## Coverage

| Scope | Result | Proof level |
|---|---|---|
| Integration | 10,713 PASS /2 FAIL;19 skips | Unique named terminal rows,10715 executed; phase exit100 |
| Prefork | 2321 main +4 auxiliary PASS | Four result blocks, each named case counted once; phase exit0 |
| Lib | 2000 PASS /1 FAIL | Unique named terminal rows,2001 executed; phase exit100 |
| Original Garrison load | One prefork PASS | Already inside totals; historical1dcc failure preserved |
| New achievement and Garrison/bootstrap | Eight +four integration PASS | Already inside totals; not native/catalog/profile-wide proof |

## Observed failures and root boundaries

1. `startup_perf::full_game_startup_stays_under_budget`:96.24s>20s. Addon-loading interval89.75s dominates the measured fixture; owner/file/primitive cost is unmeasured. Timer includes environment creation, discovery/loading, post-load workarounds and startup events. Outer nextest625.669s includes wrapper/gate lifecycle, not startup duration.
2. `talent_panel_onupdate_perf::talent_panel_open_onupdate_tick_latency_stays_under_budget`: baseline1.18ms, talent25.65ms, max45.69ms; rounded delta24.47ms>20ms. Average35/max75 ceilings pass. Tick timer includes timer processing, layout and OnUpdate dispatch, not callback bodies alone. Exact child35.37s differs from outer522.698s.
3. `asset_resolver_config::tests::gui_resolution_cache_preparation`: cold child fails before cache assertions reading encoding key `53dcc8beeb4c9e75b69c5f2a8a67c7e4` from `/syncthing/World of Warcraft`; local archive indexes cannot locate it. Fresh cache isolation cannot supply unavailable source content. Selected product/build/index mismatch or incomplete inputs remain unproved; no source fix, cache deletion, fallback or native-input acquisition follows from this failure.

Both performance selectors and the asset selector passed in saved1dcc. Compared test/harness and asset-wrapper/locked dependency sources are unchanged; this is an observed outcome change, not controlled causal attribution. Admission/resource ceilings do not explain per-test runtime. No per-test resource or vendor/cache input seal exists. Exclusive workload locking is conditional: inherited `WOW_SIM_TIMEOUT_FIXTURE_CAPACITY=reserved` bypasses it; its value was not captured.

## Next diagnostics

ProductChoice's broad-job completion prerequisite is met, but its separate capacity guard still controls launch; no compiled/runtime result exists. After that ordered diagnostic and fresh admission, unchanged exact performance controls may run once using the existing frozen binary and whole-cgroup90s cleanup. An isolated PASS neither repairs the loaded-suite failure nor proves resource causality. Preserve thresholds, assertions and vendor behavior; measure narrowed simulator-side costs before proposing fixes. No broad-suite replay for docs or these diagnostics.

## Sources

- [Saved report/counts/selected failures](../../../data/patch-api/evidence/query-final-suite-result-2026-10-11/README.md)
- [Startup fixture](../../../tests/startup_perf.rs)
- [Talent tick fixture](../../../tests/talent_panel_onupdate_perf.rs)
- [Asset wrapper and child fixture](../../../src/asset_resolver_config.rs)
- [Timeout/re-exec lifecycle](../../../tests/common/timeout_reexec.rs)

## See Also

- [[integrated-source-and-factory-proof-2026-10-09]] — acceptance authority and bounded profile matrices
- [[stock-api-startup-2026-10-10]] — separate stock/third-party startup receipts
- [[casc-asset-cache]] — cache layers and input boundaries
