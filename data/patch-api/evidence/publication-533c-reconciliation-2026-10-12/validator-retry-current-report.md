# Validator retry: bounded PASS; parent OPEN

Revision: `e48ec18a07bd1f5cefd40965cbbfc07696fe810f`.

- Fixture setup excludes current-publication-results directory from reference symlinks. Regression creates an existing capture in a private scratch reference root and asserts fixture publication leaves it unchanged; real source capture only read.
- Both requested commands invoked exactly once, explicit repository cwd, bytecode writes disabled. 9/9 tests PASS (exit 0); validator PASS (exit 0).
- Actual current: 145 rows / 93 OK / 52 gaps, six producer closures. Frozen historical: 145 / 87 / 58; two historical removals separate. Current gaps exactly match fixture; row identities match frozen capture.
- Actual tracked capture before/after: 67,850 bytes, SHA-256 `9c042e3c42c5d6770f4602e1f3feacc6f9e055d28ba35023ec1c094c0801f0e1`; byte-identical and matches HEAD blob.
- Frozen results/summary/review, validator assertions and hashes, test source, generic helper unchanged before/after. Commit only fixture/spec; no generic LATER_AUDIT_REPLACEMENTS change.

Historical validator FAIL line 81, 2878 preflight FAIL / commands NOT RUN / capture intact, and safe scratch development RED remain preserved, not superseded. Prior outcomes are user-supplied; prior artifacts/seals not read or modified. No native/full-suite PASS. Parent OPEN.

## Harness boundaries

Initial Pyrun undefined `repo` occurred before requested command invocation. Reporting evaluation later raised AssertionError after saving tracked-capture record; inspected saved result only, confirmed HEAD byte equality. Postcheck status output unavailable; no repository-status equality claim. Neither requested command rerun.

## Full command evidence

### tests

Argv: `["python3", "tools/test_patch_8_1_0_validation.py"]`

Cwd: `/home/osso/Projects/wow/wow-ui-sim`; exit 0; invocations 1.

stdout:
```text
```

stderr:
```text
.........
----------------------------------------------------------------------
Ran 9 tests in 0.520s

OK
```

### validator

Argv: `["python3", "data/patch-api/evidence/8.1.0-session-2026-10-08/validate.py"]`

Cwd: `/home/osso/Projects/wow/wow-ui-sim`; exit 0; invocations 1.

stdout:
```text
{
  "status": "PASS",
  "inventory_rows": 145,
  "extract_rows": 28,
  "ledger_rows": 173,
  "gaps": 58,
  "statuses": {
    "partial-development-green": 59,
    "bounded-coverage": 28,
    "audit-pending": 60,
    "metadata-only": 26
  },
  "preserved_inputs": 194,
  "preserved_extract_modes": 74,
  "reproduced_registers": 40,
  "inherited_extract_failures": [
    "12.0.5",
    "12.0.7",
    "12.1.0"
  ],
  "retirement_candidates": 21,
  "retired": [
    "C_Calendar.EventGetClubID",
    "C_Calendar.EventSetClubID",
    "C_Map.GetBountySetIDForMap"
  ],
  "negative_baseline": 58,
  "negative_gaps": 59,
  "required_receipts": 12,
  "publication_sweeps": 40,
  "non_vendor_mists_warnings": 0,
  "integrated_gap_closures": 2,
  "current_producer_closures": 6,
  "current_gaps": 52
}
```

stderr:
```text
```

Raw results, stdout/stderr, capture bytes and scope hashes: `/home/osso/.local/state/wow-ui-sim/verification/publication-533c-reconciliation/validator-final-retry`.
