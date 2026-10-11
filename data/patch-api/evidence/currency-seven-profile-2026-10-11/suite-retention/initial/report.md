# Currency suite retention final endpoint audit

**OVERALL: FAIL — bounded retention/document endpoint only. Enclosing suite: FAIL.**

Commit / HEAD: `842098a4e497c76bbf86e813a2e15e12600178b3` / `842098a4e497c76bbf86e813a2e15e12600178b3`.

- **EXIST / EXACT: FAIL** — all five retained files total 21,193 bytes and match manifest, target commit, HEAD and worktree. Four match current private originals; `gate/report.md` does not. Retained 5,131 bytes vs current private original 6,405 bytes. Per-file lengths/hashes and separate equality checks in report.json.
- **DERIVATION / ACCOUNTING:** count-summary explicitly derived, not an exact copy; private countproof digest, counting rule, every retained run field and comparison agree. Private outcome rows independently counted without exposing inventory. Current **15,028 PASS / 1 FAIL / 19 skips / 15,029 executions**; 15,048 execution-plus-skip slots. Prefork main 2,320 PASS/1 FAIL plus auxiliary 2+1+1 PASS counted once; integration 10,703 PASS/19 skips; library 2,001 PASS. Prior 15,024 PASS/1 FAIL/19 skips/15,025 executions. Four added PASS selectors, none removed; same recorded failure selectors, not causal or blanket regression proof.
- **LINKS / WIRED:** 16/16 added-line Markdown link occurrences resolve in new/changed README/spec/wiki prose; unchanged unrelated links excluded. Endpoint/line evidence in report.json.
- **HEAD SOURCE:** tested `1dcc59c0788d0e09ee3d02516123df965ba8dc63` matches current tracked src/tests/Cargo manifests/lock/build.rs/.cargo; no source worktree edits. Repository equality only.
- **CLAIM BOUNDS:** README/spec/wiki preserve suite FAIL and open native/profile/external/full-handoff acceptance. Matrix audit async wording describes earlier `2b1ab710d` endpoint and is explicitly contextualized in both READMEs, not a current pending-suite claim. Private countproof reference inside immutable gate names its private original, not a missing sixth packet copy; README expressly clarifies. Historical log entry remains history beneath completed reconciliation.

## Exact retained copies

- `data/patch-api/evidence/currency-fullsuite-2026-10-11/gate/report.md` — 5,131 bytes, original/commit/HEAD/worktree equality FAIL.
- `data/patch-api/evidence/currency-fullsuite-2026-10-11/gate/report.json` — 10,143 bytes, original/commit/HEAD/worktree equality PASS.
- `data/patch-api/evidence/currency-fullsuite-2026-10-11/matrix-gate/report.md` — 1,967 bytes, original/commit/HEAD/worktree equality PASS.
- `data/patch-api/evidence/currency-fullsuite-2026-10-11/matrix-gate/report.json` — 3,205 bytes, original/commit/HEAD/worktree equality PASS.
- `data/patch-api/evidence/currency-fullsuite-2026-10-11/suite-result.json` — 747 bytes, original/commit/HEAD/worktree equality PASS.

## Limits

- Saved evidence only; no independent raw-log/runtime/executable/native recertification.
- Private countproof read locally; only bounded accounting and comparison exposed, no outcome inventory or payload disclosed.
- Historical matrix gate accepted at its recorded endpoint; 33-copy packet not re-audited by this five-copy audit.
- No builds/tests/binary hashes/delegation/repository writes.
- Current private gate/report.md differs; mismatch timing/cause unproved.
- Source equality is repository equality, not executable provenance; skip identities and native/full-handoff acceptance remain open.

## Findings

- Current private original mismatch: `gate/report.md` (retained/manifest/commit/HEAD 5,131 bytes; private original 6,405 bytes). No inference about when/why it changed.
