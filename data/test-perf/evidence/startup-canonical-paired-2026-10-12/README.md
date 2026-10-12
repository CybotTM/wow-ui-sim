# Canonical startup paired observation — 2026-10-12

**Bounded observation PASS, not performance repair.** Source `490bc1080`, actual corrected epoch `diagnostic-20261012T140758688372Z`; [independent report](independent-retry1-20261012T141010076080Z/report.md) and [proof](independent-retry1-20261012T141010076080Z/proof.json). One concrete serializer test and one original canonical startup body passed; parent/child summaries are not two executions. [Contract](../../../../docs/specs/startup-addon-paired-observation.md).

## Same-run results

- Integration compile0, serializer1/1, canonical startup1/1, fmt0; source digests equal before/after/final. Six pre-existing vendor manifest deprecations retained, not suppressed.
- **220 ordered addon rows + one summary; readiness=true.** Full elapsed2,856,325,824ns against unchanged strict20,000,000,000ns budget. Addon phase2,150,189,470ns; post-workarounds188,796,337ns; startup events408,696,493ns. Residual108,643,524ns is not independently attributed.
- Same-call outer clocks sum2,090,001,873ns; addon phase minus these60,187,597ns is not independently attributed. Returned total sums1,999,711,352ns; `lua_exec` already includes compile+call. XML/frame overlays overlap; do not add every bucket as exclusive cost.
- Returned cache counters1,497 hits/0 misses/0 replay failures; evidence of reported cache behavior, not cache-content identity. Returned diagnostics0 warnings/185 nil-symbol observations/90 missing requirements: **not compatibility-clean startup**.
- New seal468,126,144 bytes authenticated once; [immediate receipt](diagnostic-20261012T140758688372Z/independent-auth-integration.json). Executable excluded; no old seal read or rehash.

[Raw startup stdout](diagnostic-20261012T140758688372Z/runtime.stdout), [stderr/JSON](diagnostic-20261012T140758688372Z/runtime.stderr), [main extraction](diagnostic-20261012T140758688372Z/observations.json), [serializer stdout](diagnostic-20261012T140758688372Z/serialization.stdout), [outcome](diagnostic-20261012T140758688372Z/outcome.json). The independent proof reconstructs raw rows; extraction alone is not the gate.

## Failure retained

Initial source `59f3ff136` [compile101/E0583](diagnostic-20261012T140542064588Z/compile.stdout) resolved `mod observation` against `tests/observation.rs` in the generated harness. No artifact or runtime credit. [Initial audit](independent-20261012T140800990027Z/report.md) remains separate. Follow-up `490bc1080` adds explicit module path only; new compile epoch justified by that source change. Unit operation exit0 was not compile success.

## Measurement and input limits

Original environment/discovery/ordered loads/post-workarounds/events/readiness/20s assertions remain. Per-addon elapsed includes `env.loader_env()` argument evaluation. Retaining rows and allocations remains inside canonical phase/full elapsed, with no subtraction. JSON encoding/emission occurs after the full clock and before assertions; failure-emission order is source-proven, not separately executed by this passing run. Flat JSON projection's32-line readability advisory did not justify an adjacent refactor.

[Input context](diagnostic-20261012T140758688372Z/input-context.json): no explicit simulator overrides; captured values null, including handler timing and XDG cache root. Actual bytecode enable/disable controls, `ASSET_RESOLVER_CACHE_DIR`, and `WOW_SIM_VERBOSE` child values unknown. Captured `WOW_SIM_BYTECODE_CACHE_DIR` is not an actual cache reader. [Cache metadata](diagnostic-20261012T140758688372Z/cache-context.json) is not cache-content/historical identity. No direct child environment attestation.

[Compile admission](diagnostic-20261012T140758688372Z/compile-admission.json):36.93GiB available/load12.47; [runtime admission](diagnostic-20261012T140758688372Z/runtime-admission.json):37.93GiB/load11.04, no same-checkout Cargo. Pressure snapshots retained. CPUQuota1200%/MemoryMax16G are recorded execution conditions, not historical equivalence. Runtime single selector/thread differs from frozen16-slot suite conditions. Default check [reused unchanged production/library scope](../../../patch-api/evidence/friend-info-exact-2026-10-12/check-20261012T124825577210Z/outcome.json); new tests covered by current target compilation, not that old check.

## Retention

[Manifest](retention-manifest.json):69 nonbinary copies/1,893,138 bytes plus4 source snapshots/14,582 bytes at490bc1080. No executable/images/cache/userdata payloads; future private retention gates excluded to prevent recursive copying. Independent retention/link/privacy gate pending.

Frozen BA15,038PASS/3FAIL/19skips remains FAIL: startup96.24s, talentdelta24.47ms and cold-CASC lookup. This isolated current observation does not repair or causally explain those failures. Native/external/full-handoff acceptance remains OPEN.
