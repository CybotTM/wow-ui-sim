# PTR / Wrath saved-artifact final gate

PASS within bounded supplied-state scope: 32/32 actual named results, 16 per lane, compile and execution exits 0.

| Lane | Effective profile | Passed | Failed/ignored/measured | Filtered | Duration | Compile JSON records | Seal |
|---|---|---:|---|---:|---|---:|---|
| ptr | client-ptr | 16 | 0/0/0 | 9885 | 1.71s | 741 | SHA256 match, once |
| wrath | client-wrath | 16 | 0/0/0 | 8024 | 2.41s | 741 | SHA256 match, once |

## Warnings
6 iced-wgpu-patched deprecated hyphenated Clippy manifest keys per lane, plus one generated-six-warnings summary per lane (12 warning occurrences total, six unique kinds). No compiler-message records. Full runtime stderr inspected: 176 Startup lines per lane, no other lines; not vendor startup zero-error proof. Full compile stdout JSON parsed, every record, with effective features/artifact and build-finished success authenticated. Full stderr/stdout retained in report.json. Actual files are currency.stdout/stderr, not files literally named actualcurrency.stdout/stderr.

## Provenance
1dcc59c0788d0e09ee3d02516123df965ba8dc63 → ae6c25baef6c59e058bc204361fd4f49d1e2853a; 70 changed paths, exclusively docs/data. All eight before, per-lane submission/execution-after, terminal-after and current hashes match. Both sealed binaries independently hashed once; size matches. No builds/tests/checks/polls/cache/source acquisition/ops/delegation.

src/c_api/item_spell/mod.rs:33 unconditionally calls c_currency::register_c_currency_info; implementation src/c_api/item_spell/c_currency.rs; behavior probes tests/currency_info_probes.rs.

## Terminal outcome versus results
outcome.json contains completed PTR/Wrath and Era RESOURCE_BLOCKED with executed_cases=0; results.json contains only completed PTR/Wrath. Era blocked entry is absent from results.json. Consistent with break before persistence of blocked entry; worker implementation not supplied in this artifact directory, so exact control flow not independently authenticated.

## Gaps / exclusions
- Era RESOURCE_BLOCKED, zero executed; not source failure, zero-error acceptance, or PASS.
- Anniversary and Forever unattempted; no compile/execution proof.
- Existing blocked Wrath/Era/Anniversary vendor-input lanes unchanged; no vendor inputs accessed.
- Pure WowLuaEnv::new / supplied Retail metadata only; not native-profile currency catalogs or vendor startup acceptance.
- Saved metadata proves historical recorded exits; this gate authenticates logs, identity, features, seals, and current scope, not a new execution.
- No default/Mists artifacts rehashed; no toolchain-version evidence supplied in these lane metadata.

## Exact evidence
Full metadata, named results, hashes, warning text, raw runtime outputs, terminal outcome and results retained in report.json.
