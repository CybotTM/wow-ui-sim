# Currency suite retention refresh audit

**OVERALL: PASS — bounded retention endpoint only. Enclosing suite: FAIL.**

Commit / HEAD: `9245bdddfa1d558771153b1626313c896b31e160` / `9245bdddfa1d558771153b1626313c896b31e160`.

- **EXACT:** 6 copies, 28,274 bytes; each matches current original, manifest length/SHA-256, commit, HEAD and worktree. Includes refreshed `gate/report.md` (6,405 bytes) and `gate/reuse-audit.json` (5,807 bytes).
- **LINKS:** earlier 16/16 proof reused; five unchanged referring files match prior commit, current commit, HEAD and worktree. Changed packet README keeps the same prior link sequence plus exactly one new reuse link, resolving at README line 9 to `/home/osso/Projects/wow/wow-ui-sim/data/patch-api/evidence/currency-fullsuite-2026-10-11/gate/reuse-audit.json`. Total 17 occurrences; no broad link rerun.
- **ACCOUNTING:** unchanged `count-summary.json` and private countproof digest justify reuse of prior derived-accounting proof. Refresh reuse-audit totals agree. Current 15,028 PASS / 1 FAIL / 19 skips / 15,029 executions (15,048 slots). Prior 15,024 PASS / 1 FAIL / 19 skips / 15,025 executions. Four added PASS executions, no removed PASS selectors; same recorded failure, not causal or blanket regression proof.
- **PRESERVATION:** `/home/osso/.local/state/wow-ui-sim/verification/currency-suite-retention-final` stays byte-for-byte unchanged with original FAIL. No source audit report edited. Source scope unchanged from prior endpoint; tracked source status empty.

## Exact copies

- `/home/osso/Projects/wow/wow-ui-sim/data/patch-api/evidence/currency-fullsuite-2026-10-11/gate/report.md` — 6,405 bytes; original/manifest/commit/HEAD/worktree equality PASS.
- `/home/osso/Projects/wow/wow-ui-sim/data/patch-api/evidence/currency-fullsuite-2026-10-11/gate/report.json` — 10,143 bytes; original/manifest/commit/HEAD/worktree equality PASS.
- `/home/osso/Projects/wow/wow-ui-sim/data/patch-api/evidence/currency-fullsuite-2026-10-11/matrix-gate/report.md` — 1,967 bytes; original/manifest/commit/HEAD/worktree equality PASS.
- `/home/osso/Projects/wow/wow-ui-sim/data/patch-api/evidence/currency-fullsuite-2026-10-11/matrix-gate/report.json` — 3,205 bytes; original/manifest/commit/HEAD/worktree equality PASS.
- `/home/osso/Projects/wow/wow-ui-sim/data/patch-api/evidence/currency-fullsuite-2026-10-11/suite-result.json` — 747 bytes; original/manifest/commit/HEAD/worktree equality PASS.
- `/home/osso/Projects/wow/wow-ui-sim/data/patch-api/evidence/currency-fullsuite-2026-10-11/gate/reuse-audit.json` — 5,807 bytes; original/manifest/commit/HEAD/worktree equality PASS.

## Reused proof and limits

Prior proof: `/home/osso/.local/state/wow-ui-sim/verification/currency-suite-retention-final/report.json` and `report.md`; initial overall FAIL was copy mismatch, not invalid accounting/link checks. Per-file evidence and commands recorded in new report.json.

- No broad reruns, builds, tests, binary hashes, delegation or source/repository writes.
- Earlier 16-link and derived-accounting proof reused for unchanged inputs; raw outcome rows not recounted.
- Repository equality is not executable or native provenance.
- Initial private endpoint FAIL preserved; refreshed endpoint PASS does not change enclosing suite FAIL.
- Audit runtime did not preserve ctx and initial detailed output was truncated; repeated bounded artifact checks solely to recover missing proof output. No test/build command executed.
