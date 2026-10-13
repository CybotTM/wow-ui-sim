# Owning publication proof — PASS

Saved-evidence audit only. Epoch: `<PRIVATE_VERIFICATION_ROOT>/ui-widget-bars-hud-green-current/20261013T103358540606Z`.
Compiled source `b2f161e68` (docs/tests delta), recorded production `5bd208dba`; mounted source snapshot `39ce8969a`. No runtime executed during this audit.

| Exact owning selector | Test | Rows | OK | Remaining known non-OK | New / stale |
|---|---|---:|---:|---:|---:|
| `patch_3_3_0_publication_sweep::patch_3_3_0_publication_sweep` | PASS | 11 | 11 | 0 | 0 / 0 |
| `patch_4_0_1_publication_sweep::patch_4_0_1_publication_sweep` | PASS | 419 | 305 | 114 | 0 / 0 |
| `patch_10_2_5_publication_sweep::patch_10_2_5_publication_sweep` | PASS | 59 | 56 | 3 | 0 / 0 |
| `patch_11_1_7_publication_sweep::patch_11_1_7_publication_sweep` | PASS | 48 | 46 | 2 | 0 / 0 |

**Four tests PASS; 119 known non-OK row instances remain** (537 total, 418 OK). PASS means exact agreement with reconciled known-gap sets, not all rows green. No new or stale gaps after f04 reconciliation. Historical 72a8 failures remain historical and untouched; this audit does not re-audit that epoch.

## Evidence audited

- Full stdout/stderr, row JSON, argv, per-selector admission, `--wait` launcher output, terminal records, individual results, and aggregate results read in full. Each stdout selects one test plus three empty fixture groups: `1 passed; 0 failed; 1 total`. All launchers report `status=0/SUCCESS`; terminal records report `Result=success`, `ExecMainStatus=0`. Blank exit timestamps and `[not set]` terminal resource metrics are not used as completion or resource evidence.
- Complete argv matches recorded owning launcher: sealed `prefork_full_ui` executable, exact selector, `--exact --nocapture`, `systemd-run --user --wait`, 90-second timeout, 4G/3G worker limits, 200% CPU, control-group kill, read-only mounted source/root/target, private temp/PID/network, and only its row directory explicitly read-write. Each recorded runtime admission is allowed; projected memory arithmetic agrees with raw counters and remains below recorded parent high/max. Root compile admission also records allowed. No live resource claim.
- All 537 row keys exactly match unique source register IDs. Every row's expected metadata agrees with source fields and same-client later-register supersession. Non-OK IDs exactly equal source known-gap sets. Duplicate JSON keys rejected. Full row data and exact IDs retained in `proof.json`.
- Scoped fixtures, their embedded register/known-gap/later-register data, and common publication sweep/probe sources directly equal mounted snapshot (complete file list in `proof.json`). Saved runtime-input reuse records source equality to compiled source. No git source-delta inspection needed; no hash computation.
- Existing authentication PASS receipt reused. Current executable device/inode/size/mtime stat matches receipt before/after and seal. No binary/library contents read or hashed. Full stderr consists of startup/workaround timings through post-event workarounds; no failure/panic/warning/mismatch diagnostics found. This is not a native startup-error or behavior certification.

## Boundaries

Publication/absence only. No signature, output, security, native/behavior parity, or whole-parent completion claim. No new tests, builds, checks, runtime launches, reruns, delegation, or tracked edits. Only this report and `proof.json` published. Saved evidence paths and verbatim runtime outputs are embedded in `proof.json`; original epoch remains unchanged.
