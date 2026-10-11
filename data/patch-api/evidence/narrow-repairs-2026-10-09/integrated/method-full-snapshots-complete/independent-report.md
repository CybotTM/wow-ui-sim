# Independent complete method-results audit

**Snapshot revision:** `2e7b7d2a5e00439ba6e6d0e25b4857b817bdd507`  
**Latest source/docs audited:** `b469261d29a1955e43f78f24d56d7d84f10a5cb6`  
**Original requested revision:** `4528df579c297c94531ec2acd572db619fdb1801` — incomplete, superseded; receipts preserved.

## Distinct verdicts

| Scope | Verdict | Exact coverage |
|---|---|---|
| Corrected full data | **PASS** |12,380extra+33missing=12,413rows;608 original→complete changed rows. |
| Saved tracked source/input | **PASS** | Relevant src/tests/Cargo/build/discovery unchanged469→61→4528→latest; assertion source unchanged. Original full files retained in Git643 and both packets. |
| Bounded simulator runtime | **PASS** |One fresh original extra assertion030409+one reused original missing PASS030002; two distinct assertions, NOT two new executions. |
| Native | **UNKNOWN** |16types/625distinct names/3,522type-method pairs are separate frozen discovery scope; native epoch/build/receiver/behavior/parity unproved. |

## Original4528 failure retained

The original packet is internally consistent:8/8 manifest hashes, four unique sorted newline-terminated files, exact105-row declared transformation;12007before/11934after and11901extra+17missing unchanged rows. Full saved469 diagnostic disproves completeness: removals6874–6969 total96, not84; omitted tail6958–6969 is12 StatusBar/Texture minimap setters. Additions6972–7462 total491 and were omitted entirely. Missing removals7495–7499/additions7502–7517 are5/16 and correct. Full delta96+491+5+16=608. Required extra11985−96+491=12380; total12413. JSON retains all exact rows/line references.

Fresh original030002 receipts independently reproduced remaining12stale/491new extra rows: extra exit101, `test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 10716 filtered out; finished in 0.16s`. Missing exit0, `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 10716 filtered out; finished in 0.14s`. Full stderr read: extra failure `tests/method_diff_coverage.rs:264:5`; not17stale rows. This is direct evidence, not an inference from prior alpha custody incident.

## Corrected retention/generation proof

Packet: `data/patch-api/evidence/narrow-repairs-2026-10-09/integrated/method-full-snapshots-complete/`.

All18 manifest hashes match. All622 numbered diagnostic excerpt lines exactly match saved rawlog. Complete-baseline delta lists equal complete raw sections. Generated files byte-equal retained/canonical/committed files and full saved-log reconstruction. Both complete files unique/sorted/newline-terminated; original unchanged rows11889extra+17missing retained. Original before files byte-equal saved469 and original643bb56e36679fa5f6cf2219f8e50af2c879e758 Git contents. Intermediate correction503rows=12removals+491additions, distinct from608original→complete.

Saved generator030142 admitted attempt1, original ignored `method_diff_coverage::refresh_method_diff_files`, exit0: `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 10716 filtered out; finished in 0.23s`. Complete stdout/stderr read. Actual canonical file write and exact503-row correction establish task input/output path, not cwd alone. No new compile-metadata gate imposed. Earlier025508 remains5denied admissions/0cases.

## Corrected runtime proof

Fresh receipt: `<private-verification-root>/method-full-extra-assertion-current/20261011T030409Z`. First4admissions denied;5th admitted31.618GiB/load22.56. Original exact extra selector, exit0: `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 10716 filtered out; finished in 0.14s`. Full stdout/stderr and execution/outcome/submission read; small canonical snapshot/assertion input hashes match before/after/current.

Missing proof reused `<private-verification-root>/method-full-snapshot-assertions-current/20261011T030002Z/1.stdout`: original exact missing selector PASS/exit0. Reuse justified by unchanged missing bytes/hash, frozen discovery, test source, same artifact identity and CASC-disabled environment. Correction touched only extra snapshot. **One new extra execution+one saved missing proof**; helper generation separate, no fresh build/all-profile/native/full-suite acceptance.

## Documentation audit

Original4528 full-result claims FAIL; later docs correct96/491/5/16=608 and12413 rows, explicitly supersede11934/105 while retaining older evidence.11corrected Markdown links resolve, including fragment targets. NativeUNKNOWN, historical saved suiteFAIL, Garrison unrepaired/not rerun, and parentOPEN preserved. Historical tables not reclassified as current results. Committed index still says both checks pending; other pages extra awaiting admission/missing reused. Later030409 receipts now settle that pending status; doc freshness wording should record1fresh+1saved, not2new. This status lag does not invalidate saved proof.

## Explicit limits/assumptions

Git proves tracked-source continuity, not hermetic runtime/caches. Sealed467MB artifact stability is saved main attestation; no independent large hash. Original rawlog hash and discovery hash receipts not independently recomputed under no-large-hashes restriction. Saved diagnostics establish simulator comparator sets, not native method contracts. Resource settings requested, effective limits not independently established. Alpha custody-restoration incident remains separate and does not invalidate these data. Only report.md/report.json written; no tracked/private mutation, build/test/replay/network/operations/process polling/delegation/Bash/rg/fd. Bounded waiting used savedfile reads only. Full-suite historical FAIL and Garrison/native/parent OPEN remain.

Retention note: absolute private verification root redacted; raw companion JSON remains private. The 18-hash audit covers the earlier corrected packet; later proof additions are recorded separately.
