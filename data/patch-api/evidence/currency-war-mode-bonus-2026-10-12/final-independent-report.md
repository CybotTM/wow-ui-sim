# Final bounded retained-evidence gate — 2026-10-12

**BOUNDED PASS; documentation update required. Parent/native/full handoff remains OPEN.**

## Authentication and precise reuse

Only new `publication-prefork-compile-20261012T080803204224Z/prefork-sealed` was read/hash-authenticated, exactly once: **509463320 bytes**, SHA-256 `e714c14649a889eb5f2e4a52f09b2194f5b3e48ed240b6144304afeee19bad8a`. Both comparisons match artifact.json. `publication-independent-auth.json` was persisted immediately at 2026-10-12T08:16:16.602978+00:00, before this report. Its Cargo record matches exactly one retained compile record; build-finished success/compile exit0 recorded. Cargo/rustc1.99.0 retained toolchain, x86_64-unknown-linux-gnu, LLVM23.1.1; no toolchain command rerun.

First-auth receipts reused without reading or hashing any old seal:

| Lane / prior verifier | Receipt | Prior digest / bytes |
|---|---|---|
| Retail integration 2608 | `default-green-independent-auth-integration.json` | `86774e8187f356f75b4f9b87b1afc6ad2d0bcf987c3bbbab6f82f3832410fbce` / 467645008 |
| Retail normal 2608 | `default-green-independent-auth-normal.json` | `20bd4d6832306badbf4c2dd5465fa7584b6fdc37f99bf6b4d0c20136666fc0b2` / 407293168 |
| PTR integration 2609 | `ptr-green-independent-auth-integration.json` | `9ba40034c051ad4a29cfc45937271e83c39148bf746fcba26aadcb26ad4148a6` / 467474048 |
| PTR normal 2609 | `ptr-green-independent-auth-normal.json` | `fd8c3c13321f6c76a98b770fb07e261d216881aa58c7473d7643866587b263c2` / 414537496 |
| Initial RED 2598 | `red-independent-auth.json` | `35bc2c79e3b11052f0a79227eb7e3c377c4ba07e8aa0f91d7af452ee21e34789` / 467732696 |
| Expanded RED 2606 | `positive-red-independent-auth.json` | `b09f48ba38bdf471119f3b13e899efc1941bf92b1735020f4530d3b6f0a5acf1` / 467618248 |

Receipt digests/sizes match their artifact metadata. This is receipt reuse, not fresh seal authentication. Retail execution/readability reused from `default-green-independent-proof.json`; initial RED from `red-independent-proof.json`; expanded RED from `positive-red-execution-independent-proof.json`. Earlier PTR artifact-only/PENDING snapshot remains historical, now supplemented by complete retained execution streams; not rewritten. No additional old cases inspected or replayed.

## Concrete coverage matrix

| Exact capability | Evidence / counts | Boundary |
|---|---|---|
| Absence, strict numeric input, numeric nonrepresentation | Same nine logical tests Retail9PASS/0FAIL and PTR9PASS/0FAIL | Selected simulator absent policy, not native catalog |
| Nine nullable combinations; live owned updates/removal; currency isolation; preserved scalar observations | Supplied state assertions in unchanged tests | No positive normal-session seed |
| i32 positive identity and non-aliasing edges | Concrete seeded IDs and fractional/out-of-range/nonfinite negatives | Not native ID semantics |
| Secret-first denial, invalid type, wrapper/caller preservation | Three secret controls within each nine-case run | Security contract tested; no consumer parity |
| Retail normal raw registration/arity/empty flags/string rejection | Reused exit0/`[]`,291 addons | Loaded startup query only |
| PTR normal raw registration/arity/empty flags/string rejection | Full normal streams/outcome exit0/`[]`,283 addons;0 unique/0 occurrences | Loaded startup query only |
| Owning Retail prefork publication | One selected custom-harness case PASS,0FAIL;3 auxiliary blocks select0 | Publication/absence only |
| Exact current classification |145 rows,91OK,54gaps;failed-ID set equals54 fixture entries | Current query row OK: `raw=function; lookup=function`, expected published |
| Final formatting/default check | Retained fmt/check both exit0;all before/after/current scoped hashes equal | No rerun;default profile check only |
| RED regression evidence | Initial0PASS/5FAIL;expanded0PASS/9FAIL;runtime101 | Prior admitted execution;resource-blocked snapshots preserved |
| QuestUtils firstInstance | Literal cached vendor lines899–914 computes seen-ID flag locally | Source proof only;no new Rust field needed |
| PvP, quest eligibility, faction grant, native consumer/catalog | Separate unmodeled/unproven boundaries | Positive QuestUtils reward tooltip/native/full handoff OPEN |

PTR runtime full stdout/stderr reads establish9PASS/0FAIL,9936 filtered out,exit0. All full retained PTR normal and publication runtime streams/outcomes and final fmt/check streams/outcomes were read. Stream byte/line ledger is in JSON.

Publication `runtime-outcome.json` summary=null is a **regex mismatch**, not missing proof: retained producer regex expects `ignored;`, while actual custom harness outputs `1 passed; 0 failed; 1 total`. Raw stdout shows the exact named case PASS followed by three0/0/0 auxiliary blocks. These auxiliary blocks contribute zero cases. Earlier compile-only `outcome.json` is not the terminal execution receipt. Owning test uses the normal loaded prefork environment, no substituted getter/vendor rewrite; classification proves presence only.

## Source and fixture applicability

Current observed HEAD `cd17c335cd27e57417765f11cd8fb8fc43c11b46`. Implementation/state/tests/Cargo scoped diff against producer `ce2c37ad5623642bec418733ac363600ea3e7b63` is empty. Whole src/tests/Cargo delta is only `tests/data/patch_8_1_0_sweep_known_gaps.json`, deliberately updated at `dcbc347ad3eede33ef56369fde663a48caa32546`; whole src/tests/Cargo diff against that fixture revision is empty. Newer docs/evidence commit does not invalidate unchanged code. Final check epoch `check-20261012T081437494305Z` records identical six source hashes before/after, independently equal current files. Full comparisons retained in JSON.

Query reads `HashMap<i32,(Option<bool>,Option<bool>)>` initialized empty, authenticates secret input before type/lookup, accepts finite integral i32 identities without string coercion, projects two independently nullable scalar values. No eligibility inference or normal seed. Cached unmodified Retail `QuestUtils.lua` lines683–693 separately gates `IsWarModeDesired`, both currency flags, world-quest status, `QuestCanHaveWarModeBonus`, and `GetFactionGrantedByCurrency`. Lines899–914 compute `firstInstance = not uniqueCurrencyIDs[currencyReward.currencyID]`, mark seen IDs, and carry the value into currencyInfo before sorting. Local duplicate accounting is not missing simulator state. This gate performed no vendor edits; no independent upstream/native identity comparison is claimed. Source search finds no IsWarModeDesired implementation, QuestCanHaveWarModeBonus registered to return_false, and no dedicated faction-grant relation; fallback behavior/native caller output not proven.

## Warnings and pending docs

Six inherited iced_wgpu manifest deprecations remain: large-enum-variant, map-entry, match-wildcard-for-single-variants, redundant-closure-for-method-calls, trivially-copy-pass-by-ref, type-complexity. Check exit0 does not erase them. No compilation warning repair or broader suite acceptance claimed.

Earlier pre-retention observation (superseded by da0ddb5ca) had **no staged changes** and unstaged docs: `docs/specs/currency-war-mode-bonus.md`, `docs/wiki/investigations/patch-8-1-0-api-audit.md`, `docs/wiki/log.md`. Spec/wiki/log still leave final independent gate pending. Current evidence `data/patch-api/evidence/currency-war-mode-bonus-2026-10-12/README.md` now records PTR9PASS/normal283, publication1PASS145/91/54 and checks0; expected final independent-gate wording/retention remains **update required**, not code invalidation. Final report/auth/proof need retention by owner if packet is to represent this gate. Verifier edited no docs, committed nothing, copied no packet. Untracked `.code-index.db`/`verification/` observed, not altered.

No build,test,run,check,format,delegation,network,operational mutation,polling,or broad rerun. Only requested auth/report/proof files written. Read-only git inspection initially failed from missing cwd; corrected with explicit repository cwd. Bounded query/publication/check gate passes; documentation handoff and independent native/positive consumer/full parent acceptance remain separate.

## Public packet follow-up audit — da0ddb5ca

Retained text/JSON streams and receipts audited against original sources and both manifests: **105/105 byte-equal copies,895538 bytes; initial54 preserved plus51 new**. Every copied digest and size matches through-check manifest; no binary seals opened or hashed. README/spec/wiki investigation/log worktree and index contents equal committed da0ddb5ca, so earlier dirty-doc observation is superseded, not a current staged change. Changed currency-query doc links resolve. Two pre-existing repeated log links to `mists-cast-startup-visibility.md#current-fixture-phase--2026-10-11-unverified` do not match the current heading; adjacent/out-of-scope, not edited. Directory links resolve as directories. Exact link audit in proof JSON. Final-gate status pending is expected until caller retains these final reports/auth and updates docs; no verifier docs edits. Commit contains evidence/docs only, preserving prior implementation and fixture proof; no proof command rerun.
