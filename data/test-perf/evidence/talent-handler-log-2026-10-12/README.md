# Talent handler-log observation — 2026-10-12

`1d2b0ea30` adds test-only per-tick markers and a standard-library parser. [Contract](../../../../docs/specs/talent-handler-log-observation.md), [source/parser proof](source-parser-independent-report.md), and [independent real-capture audit](diagnostic-20261012T134247318942Z/capture-independent-report.md) establish bounded observational coherence, not a performance fix.

## Actual capture

Original selector body executes and passes **once**; parent/child summaries are not two passes. All54 ordered windows match independent raw-log reconstruction. Setup,20 baseline/10 settle/24 steady ticks and35ms/75ms/+20ms assertions remain unchanged. No vendor/runtime optimization or production metric added.

| Region | Records | OnUpdate / OnPostUpdate / other | Rounded interval sum |
|---|---:|---|---:|
| Baseline20 | 800 | 760 / 0 / 40 | 7.189ms |
| Settle10 | 436 | 404 / 0 / 32 | 61.275ms |
| Steady24 | 1008 | 960 / 0 / 48 | 8.765ms |
| Outside windows | 1704 | 232 / 0 / 1472 | 470.814ms |
| Entire stderr | 3948 | 2356 / 0 / 1592 | 548.043ms |

Baseline38 OnUpdate records/tick, steady40. First-settle `ClassTalentsFrameMixin:OnUpdate` record55.552ms is outside the steady samples. Original displayed wall timings:511.77µs baseline average,564.29µs steady average,711.22µs steady maximum; displayed delta52.52µs. These logged-run results do not repair or explain the frozen24.47ms delta.

[Raw stderr](diagnostic-20261012T134247318942Z/runtime.stderr):4195 lines, comprising3948 handler records,108 markers and139 other lines outside windows. [Parsed JSON](diagnostic-20261012T134247318942Z/parse.stdout) retains repeats, phase/tick categories/grouping, outside records and unparsed-line counts. Global grouping includes outside-window work; phase grouping does not. Zero OnPostUpdate records does not establish zero registrations or coverage of every dispatch route.

## Evidence limits and provenance

[Outcome](diagnostic-20261012T134247318942Z/outcome.json): targeted compile/parser/fmt pass; source unchanged. Prior [default check](../../../patch-api/evidence/friend-info-exact-2026-10-12/check-20261012T124825577210Z/outcome.json) reused only for unchanged production/library; new Rust test compiled separately. Six inherited manifest warnings remain.

New integration seal authenticated once, receipt [retained](diagnostic-20261012T134247318942Z/independent-auth-integration.json); no executable copied or old seal rehashed. Parent launch records existing `WOW_SIM_LOG_HANDLER_TIMINGS=0` before process initialization; source supports child environment inheritance, but independent child environment/PID/cache-content attestation is absent. Parser conservatively retains `all_attempts_verified=false`.

Intervals are rounded, inclusive of helper/error/nested work; separately logged nested calls can double-count. Counts are dispatch-attempt records, not successful/unique/top-level callbacks or exclusive-body costs. Logging perturbs wall-clock timings. Historical16-thread run and isolated single-case conditions are not established equivalent; no causal/native/full-suite acceptance.

Parser development: genuine RED46 assertion failures, then12 tests pass; [source audit](source-parser-independent-report.md) binds current GREEN hashes. Initial syntax-error attempt is retained but not RED credit. Historical RED implementation was not independently cryptographically attested. Synthetic CLI54-window test remains distinct from this real capture.

## Retention

[Manifest](retention-manifest.json):57 byte-exact nonbinary copies,2,845,419 bytes. Root source-only `proof.json` is explicitly retained as `source-parser-proof.json`, not a runtime/final proof. Original capture/metadata/protocol/TDD reports remain separate. Independent retention/link/privacy audit at `e95d9f1901ef270058f18dac788814863f070423`: **bounded PASS**,57/57 copies/2,845,419 bytes,19/19 links/4/4 anchors,zero privacy findings; no binaries/images/replays. Actual retained-text inspection is not a universal secret-detection guarantee. Private reports remain outside the packet.

Frozen BA15,038PASS/3FAIL/19skips, startup96.24s/talent24.47ms-delta/cold-CASC failures and full handoff remain OPEN.
