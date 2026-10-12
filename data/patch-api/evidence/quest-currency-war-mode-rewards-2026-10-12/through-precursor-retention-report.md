# Through-precursor retention verification — 2026-10-12

Repo `/home/osso/Projects/wow/wow-ui-sim`, audited HEAD `7e136499558625b6dcdc6ac6bda186b4caf52d5a`.

**PASS — bounded retention only.** 108/108 declared copies byte-equal; 2,889,750 bytes. Zero missing files, unequal bytes or declared-size/count discrepancies. Direct comparison once per row; no source comparison repeated, no binary opened, no hash computed. Later destination-only text inspection does not authenticate binaries. Full compile/runtime streams, ten cached Lua snapshots and first-authentication receipts are included. Per-file evidence is in adjacent proof JSON.

| Manifest | Copies | Exact bytes | Result |
|---|---:|---:|---|
| RED | 40 | 1,241,131 | PASS |
| Source | 10 | 291,387 | PASS |
| Precursor | 58 | 1,357,232 | PASS |

## Stage scopes — retained evidence, not new execution

- Initial RED: compile0; five tests, 1 PASS/4 FAIL, 1.00s; terminal ExecMainStatus101/MainPID0. Separate from expanded totals.
- Expanded RED: compile0; 13 tests, 1 PASS/12 FAIL, 2.15s; terminal ExecMainStatus101/MainPID0. Baseline sole PASS; later downstream assertions are not proven by early failures.
- Retail precursor `4e19a8bad`: compile0/runtime0; 13 PASS/0 FAIL, 4.74s; normal stdout `[]`, exit0. Existing first integration/normal authentication receipts retained, not repeated.
- PTR precursor: compile0; six rejected admissions, zero executed tests/RESOURCE_BLOCKED. No normal artifact or execution; misleading follow-up receipt filename explicitly records UNAVAILABLE. Controller error JSON is not made successful by systemd exit0.
- Initial source/spec/wiring PASS had one readability duplication FAIL. Resolved follow-up `a5eaf4fc6`: source equivalence/bounded manual readability PASS, metrics unavailable. Prior producer source proof reused, not runtime proof. At audited commit current final/checks queued; no current-final acceptance claim.

## Links / payload

15/15 local links/anchors resolve.
README and spec links checked fully; only relevant QuestUtils/currency-query wiki sections checked, not unrelated wiki content. Anchors resolved against target Markdown headings. No implementation-shape assertion performed.

112 packet files inspected: 108 retained payload files plus README and three manifests; zero payload exclusion findings.
Only text extensions `.json`, `.md`, `.lua`, `.stdout`, `.stderr`; UTF-8/NUL, filename/private-key/cookie-format and symlink checks. Artifact paths/digests inside text are metadata, not included binary/key/cookie files. This is file-payload exclusion evidence, not a claim of exhaustive secret detection.

## Exact mismatches / remaining gaps

Zero retention/documentation-link/payload mismatches.

PTR runtime/normal remains absent; current-final runtime/startup/fmt/default-check evidence is not supplied. Cached source equality proves retained copies only, not native behavior or whole loaded source-to-artifact provenance. Parent handoff remains OPEN; historical frozen full-suite3FAIL not rerun or cleared. All-source matches do not close any of these boundaries.

No builds/tests/replays, live worker operations, binary authentication, old currency-packet hash/authentication access, implementation edits, delegation or questions. Only requested report/proof outputs written outside repo. Initial git invocation lacked explicit cwd and failed; subsequent explicit `git -C` confirmed requested HEAD. Untracked `.code-index.db` and `verification/` untouched.
