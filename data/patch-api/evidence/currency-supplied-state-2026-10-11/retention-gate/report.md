# Currency retention final artifact/document gate

**PASS — HEAD `09c64655f9524c1dae86c716f7ffc05a5d57145a`.** Artifact/document endpoint only. Source/runtime gates remain separately accepted; no new execution proof.

## Exact gate counts

| Gate | Result |
|---|---|
| Original/copy presence, byte equality, SHA-256, manifest size | 64/64 each; exactly 2,550,551 bytes |
| Manifest targets tracked at requested HEAD | 64/64 |
| All claimed retained/doc/source paths | 74/74 present and tracked: 66 packet files, five distinct source/test paths, three external document targets |
| README/spec relative links | 7/7 resolve to present tracked files; README five, spec two |
| Checked spec claims | 8/8 supported within explicitly bounded/sample preservation scope |
| Substantive claim boundary checks | 9/9 preserved; zero gate failures |

Three empty exact captured streams are legitimate: default-green/check-0.stdout and check-0.stderr (format), default-green/check-1.stdout (check). Originals, copied lengths and hashes match. No placeholder classification.

## Eight checked spec claims

| Spec line | Claim | Retained support and boundary |
|---|---|---|
| 7 | Strict boolean static backpack flag | Mists named watched-flag control asserts true/false in namespace and legacy slot5; mutation probe conflicts supplied flag with static watched=true |
| 8 | Numeric maximum quantity, zero/positive | Static cap control asserts numeric90; shared updates assert250 then0; Mists updates assert456 then0; sampled header cap0 |
| 9 | Mists positions5/8 | Two static controls plus mutable mapping probe assert those exact positions |
| 14 | Five mutable list display fields by existing ID | Named shared list-update case, 2245, two concrete updates and exact nine-field output |
| 15 | Mutable backpack name/quantity/icon | Named shared backpack-update case, two concrete updates and four-field output |
| 16 | Successive updates with static structure/membership | Shared helper preserves size19, header1, list6 and watched2/3 after updates/removal; Mists compares header/unwatched12-slot baselines and neighbor4-slot baseline after both mutations. Sampled, not exhaustive order/catalog acceptance |
| 17 | Missing map row yields nil/no returns, headers independent | Two named shared removal cases assert list one-nil/backpack zero returns; sampled header remains intact; no static resurrection |
| 18 | Invalid indices and supported Mists tuple slots/arity | Shared helper checks list0/20 and backpack0/4; Mists probe checks list arity12, supported slots1–8/11–12 and backpack arity4. Mutated unsupported slots9/10 remain explicitly unasserted |

Retained default stream contains 16 named PASS cases; Mists streams contain 16+2+1+1=20 named PASS cases. Default names are the shared16 included in Mists. Combined36 executions comprise20 distinct names, **not36 distinct behaviors**. Names and statuses retained in report.json. Current test assertions were read only to align documentation, not to repeat the source/runtime gate.

## Nine boundary checks

1. Initial compile exit101 remains preserved with zero case executions, not behavioral RED.
2. Actual RED retains12 PASS/four FAIL/exit101; four failures are the list/backpack update/removal regressions. This is expected historical failure, not a failure of this retention gate.
3. GREEN totals are actual retained16 default/20 Mists cases; no fresh runtime evidence or extrapolated36 behaviors.
4. Preservation is sampled; exhaustive catalog/default/order acceptance stays unchecked.
5. Mutated Mists slots9/10 remain explicitly unasserted; unchanged header/unwatched12-slot baselines are not promoted into mutated-row proof.
6. Native catalog, native update timing and comprehensive parity remain unclaimed.
7. Other five profile captures and new full suite remain unaccepted/asynchronous; prior c75 Garrison FAIL remains history.
8. Deprecated manifest names/audio diagnostics remain acknowledged; no warning-free, audio, full-profile/vendor-startup or full-handoff claim.
9. Copied reports' private original paths, historical HEAD references and historical pending-spec wording retain original context, as README states; reports were not rewritten into current-state claims.

## Read-only scope proof

Explicit-cwd git rev-parse confirms requested HEAD. `git diff --name-status 1dcc59c07 09c64655f -- src tests Cargo.toml Cargo.lock` exits0 with empty output. Working and staged comparisons to HEAD are empty; scoped untracked-file listing is empty. Thus current source/test/Cargo scope matches accepted1dcc59c07. README/spec/packet working diff to HEAD is also empty. Unrelated untracked `.code-index.db` and `verification/` remain outside this scope.

No binaries read/hashed; no builds, tests, checks, full suites, operations, acquisition, delegation, Bash, rg or fd. Original64 text files were compared directly to copies and manifest SHA-256/size. Relative links were resolved from each document's own directory. No tracked files modified; only requested report.md/json written.

Initial git commands without explicit cwd failed outside repository; corrected explicit-cwd commands supply the proof above. Two evaluations relying on unavailable cross-evaluation variables failed before producing evidence; self-contained evaluations resolved this. Neither diagnostic failure was used as acceptance evidence.

**Real retention/link/claim failures: 0.** Historical failures retained: initial compile101/zero executions; actual RED12 PASS/four FAIL; prior c75 Garrison FAIL. Proof remains valid only for specified HEAD and retained/original files; changes invalidate corresponding comparisons.
