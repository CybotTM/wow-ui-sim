# Talent tick stages — observational ledger

Private epoch `compile-20261011T233940578091Z`: one new wrapper compilation and one runtime exit0; **54 observations**, not tests or acceptance cases. Independent2459 authentication/source/count gate pending. [Manifest](retention-manifest.json), [raw rows](runtime.stdout), [aggregates](summary.json), [source](probe.rs), [handoff SSOT](../../../../docs/wiki/investigations/integrated-source-and-factory-proof-2026-10-09.md#talent-stage-diagnostic--private-observations-2026-10-11).

Existing talent latency fixture setup/order retained:20 baseline ticks, open panel,10 settling ticks,24 steady ticks. Additional clocks and stored-count snapshots introduce observational overhead. Output follows all samples; timer scheduling uses real time, not synthetic16ms advancement. No budget assertions or acceptance-selector execution.

| Phase | Rows | Mean total | Mean whole public OnUpdate |
|---|---:|---:|---:|
| Baseline | 20 | 355.54µs | 353.08µs |
| Talent settling | 10 | 5.72ms | 5.41ms |
| Talent steady | 24 | 405.06µs | 402.38µs |

Preliminary steady-minus-baseline means: total **49.52µs**, whole public OnUpdate **49.30µs**. Timer means2,337.45ns/2,563.0833ns; explicit pre-OnUpdate layout24.6ns/20.875ns. Nonnegative residuals preserve measurement overhead; raw nanosecond values remain authoritative. Settling maximum52.98ms is a separate phase, not the steady24-sample maximum.

Stored tracked entries647→653; visible-cache values all`null` (absent), not zero or visible-handler proof. Timer return counts are not successful callback counts. Whole public OnUpdate includes internal work, not a callback-body/internal-stage breakdown; private `fire_on_update_timed` is not used or exposed.

Selected Retail library identification is saved Cargo projection/path/stat only, not frozen-ba, hermetic-input or native identity. No Cargo/library rebuild or repeated old library/binary hash. Original233855 worker failed at epoch directory creation before Rust; retry1 used a separate microsecond namespace and produced the only actual wrapper compilation. Both history and actual receipts are retained. Runtime had90s/control-group bounds; terminal receipt and process observation record no remaining probe.

No production/vendor/threshold change. This isolated measurement neither explains nor repairs loaded [ba FAIL](../../../../docs/wiki/investigations/fullsuite-ba-2026-10-11.md); existing query67/currency116/XML3/isolated-control scopes remain separate. Full handoff **OPEN**. No executable, library or vendor payload retained.
