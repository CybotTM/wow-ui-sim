# Owning publication gap reconciliation

Recorded October 12, 2026 local / October 13 UTC. Bounded independent runtime/fixture audit PASS at `ccd526c6c9f783cf7d3fe15eda05c42c6cabab59`; public retention audit PASS: 18/18 complete copies, unchanged before/after rows, preserved failures, zero privacy/fidelity defects.

| Patch | Rows | OK / gaps | Old-fixture run at f3a7 | Reconciled run at ccd526 |
|---|---:|---:|---|---|
| 10.2.5 | 59 | 54 / 5 | FAIL: nine stale gap IDs, no new gaps | 1 PASS / 0 FAIL |
| 11.1.7 | 48 | 41 / 7 | FAIL: one stale gap ID, no new gaps | 1 PASS / 0 FAIL |

Fixtures remove exactly the observed nine + one resolved IDs, add none. Entire before/after row maps are identical; only expected gap fixtures changed. Six inherited manifest warnings remain per compilation. These are owning `prefork_full_ui` runs through cached Game UI, not bare factory probes. Trailing zero-case harness groups add no test credit.

[Independent report](after/report.md) and [ledger](after/ledger.json) preserve source, command, selection and exit-attribution limits. Complete original failure streams and all observations remain under `before/`; complete reconciled streams/observations under `after/`. Unsupported dual-filter attempt is retained as `after/runtime.stdout` and `after/runtime.stderr`: zero cases, no credit. It failed before startup; subsequent single-filter runs do not erase it. [Manifest](retention-manifest.json) covers eighteen complete sanitized text copies; originals remain private. Text hashes are not executable/library authentication.

Remaining gaps are ping texture kits, transcription and three legacy aura retirements for 10.2.5; assisted action discovery, three profiler methods, availability, currency transfer activity and ButtonHeader for 11.1.7. Publication only: raw/member/receiver lookup, not signatures, outputs, security or native parity. Prior 59/14 and 48/8 historical receipts remain unchanged. Later availability input scaffold `e1b3b0e3d` is outside this proof and supplies no getter credit. No other-profile, full-suite, performance or whole-handoff acceptance.
