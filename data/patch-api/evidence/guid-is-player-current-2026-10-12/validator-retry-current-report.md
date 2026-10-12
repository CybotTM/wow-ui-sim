# Validator retry — PASS (bounded scope only)

Source `d21ca3f64`; unit `wow-guid-validator-retry-20261012t193338173458z` finished October 12, 2026, 19:33:41 UTC (started 19:33:38 UTC), within 20 minutes. Observed active/exited, Result=success, ExecMainStatus=0. Controller completed without stderr or semantic error.

| Required proof | Result |
|---|---|
| Nine Python validator regressions | 9/9 PASS; 0 failures; 2.105s; exit 0; exactly one invocation |
| Actual `validate.py` | PASS; exit 0; exactly one invocation |
| Current vs frozen accounting | Seven current producer closures, 51 gaps, 94 OK; frozen 58 gaps, 87 OK; all 145 rows |
| Supersession closures | Two separate integrated closures; not counted as seven producer closures |
| Source/captures preservation | Before = after regressions = after validator = current: byte sizes and SHA-256 equal for all 14 recorded paths |

## Invocations and complete output

Both ran from `/home/osso/Projects/wow/wow-ui-sim` with recorded overlay `PYTHONDONTWRITEBYTECODE=1`. Full inherited environment was not recorded. Phase loop, submission/result artifacts, command-results and full controller execution establish two main Python command invocations, one each; no verifier replay.

1. `/usr/bin/python3 tools/test_patch_8_1_0_validation.py`

stdout empty; stderr:
```
.........
----------------------------------------------------------------------
Ran 9 tests in 2.105s

OK
```

2. `/usr/bin/python3 data/patch-api/evidence/8.1.0-session-2026-10-08/validate.py`

stderr empty; stdout:
```json
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
  "current_producer_closures": 7,
  "current_gaps": 51
}
```

## Isolation and negative controls

`tools/test_patch_8_1_0_validation.py:28-66`: BOTH flat packet directories excluded from directory symlinks. New scratch packet created as a REAL directory; only required README leaf symlinked to actual source metadata. Tests never write this metadata; read-only is usage, not OS-enforced symlink protection. `store` unlinks file symlinks before replacement.

Reference preservation test copies both actual result files and actual README bytes into its own real reference packet directories before constructing nested scratch fixtures. This restores proper context without exposing source capture files to synthetic writes.

Clean seven-closure validation precedes both fixture-negative controls and result mutation controls. Each result mutation reconstructs a fresh current map. GUID wrong-symbol and fallback-only now reach exact expected-value/raw-provider assertions, not a missing README. Nine passing tests provide aggregate behavioral credit; subtest names are source-inspected, not separately logged. Missing-current/old533c-only still rejects rather than falling back.

## Preservation / reuse

New actual map: 67854 bytes; old533c: 67850; frozen: 67826. Both actual current packet directories and result files are non-symlinks. Full per-path SHA-256 and three producer snapshots plus current comparison retained in adjacent proof JSON. Frozen summary and gap review also unchanged. Old533c remains 93 OK/52 gaps.

Resource admission: 23.9944 GiB available against 6 GiB minimum; load1 16.14 against limit 24; no relevant Cargo process. First admission succeeded; memory/IO pressure snapshots retained. Source applicability: empty diff against `d21ca3f64`, exit 0.

Reused default Cargo check: `/usr/bin/cargo check --offline --locked -j 12`, exit 0, 18.83s; six existing vendor manifest deprecations retained. No rerun. Recorded Rust/Cargo scope hashes still match.

Reused GREEN Retail/PTR fmt: exit 0, empty streams. Reused Rust behavior: 9/9 each, 18/18 total, exit 0; filtered cases receive no credit. Source-only snapshots before/after compile/runtime match current Rust and Cargo inputs. `docs/specs/guid-is-player.md` hash differs from GREEN; not claimed unchanged and not executable code. No binaries, authentication receipts or seals opened.

## History and limits

Earlier final-check stays FAIL: eight of nine tests passed, one missing scratch README failure; standalone validator NOT RUN in that earlier epoch. Retry does not rewrite those facts or old reports/proofs. No native parity, full-suite or whole-handoff PASS. No source edits, ops, delegation, builds/checks/tests rerun by verifier.

All full retry outputs, exits, invocations, controller, conditions and snapshots stored in `validator-retry-current-proof.json`. Only this report and that proof created using exclusive `open("x")`; no generic proof or existing file written.
