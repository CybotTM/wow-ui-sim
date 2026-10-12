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
3. `asset_resolver_config::tests::gui_resolution_cache_preparation`: cold child fails before cache assertions reading encoding key `53dcc8beeb4c9e75b69c5f2a8a67c7e4` from `/syncthing/World of Warcraft` through archive lookup/fallback. This is not proof that the effective historical index lacked an entry, nor a cache-behavior verdict. Selected product/build/index mismatch or incomplete inputs remain unproved; no source fix, cache deletion, fallback or native-input acquisition follows from this failure.

Both performance selectors and the asset selector passed in saved1dcc. Compared test/harness and asset-wrapper/locked dependency sources are unchanged; this is an observed outcome change, not controlled causal attribution. Admission/resource ceilings do not explain per-test runtime. No per-test resource or vendor/cache input seal exists. Exclusive workload locking is conditional: inherited `WOW_SIM_TIMEOUT_FIXTURE_CAPACITY=reserved` bypasses it; its value was not captured.

## Next diagnostics

[Reduced ProductChoice result](../../../data/patch-api/evidence/productchoice-declaration-probe-2026-10-11/README.md) is independently authenticated: compile0/run0; declaration returns self-anchor Err, separate sibling control resolves its points. No full-template/startup/native repair follows.

[Two isolated exact controls](../../../data/patch-api/evidence/ba-isolated-perf-controls-2026-10-11/README.md) independently pass once each on one sealed existing ba executable: startup3.68s<20s; talent average501.18µs/max595.91µs/rounded delta59.66µs within35/75/20ms. Saved units exit0 under90s whole-cgroup limits; no remaining processes observed. No timeout was exercised. Environment inherited without overrides, but historical input/environment equality is unproved. An isolated PASS neither repairs the loaded-suite failure nor proves resource causality. Preserve thresholds, assertions and vendor behavior; measure narrowed simulator-side costs before proposing fixes. No broad-suite replay for docs or these diagnostics.

[Separate talent-stage observations](../../../data/test-perf/evidence/talent-stage-diagnostic-2026-10-11/README.md) now have independent coherence proof:54 observations, not acceptance cases; isolated total delta49.52µs/whole public OnUpdate49.30µs does not explain loaded failure. [Current CASC candidate prefix](../../../data/patch-api/evidence/casc-encoding-header-2026-10-11/README.md): one34-byte read finds BLTE at+30 and matching9-byte key prefix in data174; full-key/size layout differences are not checked by the asset-resolver. Independent2463 verifies cached metadata/primary-source limits/initial retention only; no effective historical lookup/payload/decode/root cause proved. Both diagnostics leave this frozen verdict unchanged.

### Pending startup attribution — 2026-10-11

`d85b88936` commits the [observation-only startup wrapper](../../../data/test-perf/evidence/startup-cost-diagnostic-2026-10-11/README.md) (`probe.rs`, `worker.py`, `runtime-worker.py` in that packet). Async controller submitted; first capacity guard denied compilation (load28.95, RAM22.001GiB, no relevant Cargo). Later guarded wrapper compile exited0/source unchanged; [independent compile/source gate](../../../data/test-perf/evidence/startup-cost-diagnostic-2026-10-11/compile-independent-report.md) authenticates the new probe once, with library provenance limited to Cargo-record/path-stat. Two runtime guards denied; no observation executed. Pending phase/file measurements cover environment initialization, addon discovery/ordered loads, post-load workarounds and startup events; nested timings are not exclusive costs or additive attribution. Phase clocks exclude snapshots; enclosing clock includes observer overhead. Library Cargo metadata/path/stat is neither hermetic provenance nor frozen-BA artifact provenance. No production/vendor/threshold change or loaded-failure attribution. Frozen **15,038 PASS / 3 FAIL / 19 skipped / 15,041 executed**, original failures and isolated controls remain unchanged; talent54 observations and collector3 done remain separate, Calendar/Bounty accepted scopes preserved. Full handoff OPEN.

## Sources

- [Pending startup observation wrapper](../../../data/test-perf/evidence/startup-cost-diagnostic-2026-10-11/README.md)
- [Saved report/counts/selected failures](../../../data/patch-api/evidence/query-final-suite-result-2026-10-11/README.md)
- [Startup fixture](../../../tests/startup_perf.rs)
- [Talent tick fixture](../../../tests/talent_panel_onupdate_perf.rs)
- [Asset wrapper and child fixture](../../../src/asset_resolver_config.rs)
- [Timeout/re-exec lifecycle](../../../tests/common/timeout_reexec.rs)

## See Also

- [[integrated-source-and-factory-proof-2026-10-09]] — acceptance authority and bounded profile matrices
- [[stock-api-startup-2026-10-10]] — separate stock/third-party startup receipts
- [[casc-asset-cache]] — cache layers and input boundaries
