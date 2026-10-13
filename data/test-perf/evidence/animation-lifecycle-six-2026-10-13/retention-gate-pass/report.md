# Animation six-case retention verification

**PASS — new retention gate only.**

Committed artifact revision: `ad75348f867cd5d1b8e862fa82e596c63bbf3c6d`. Saved source/runtime revision: `a8a148e98d8dc32cf81e789fc8926830c9ed0161`. Report does not edit the packet's historical PENDING text or grant parent acceptance.

## Retained artifacts

[EXIST] PASS: exact 11-file inventory, nine payloads + manifest + README; no helper/mapping retained. All 11 files agree with committed blobs.
[SUBSTANTIVE] PASS: nine complete TEXT originals independently compared to public projections; exact sizes and SHA-256 values match manifest. **27,208 public bytes**, 28,609 original bytes. Full digests and sources in `proof.json`.

| Payload | Original bytes | Public bytes | Complete projection/digests |
|---|---:|---:|---|
| runtime/animation-complete.admission.json | 2389 | 2389 | PASS |
| runtime/argv.json | 1702 | 1478 | PASS |
| runtime/launcher.txt | 299 | 299 | PASS |
| runtime/stdout | 661 | 660 | PASS |
| runtime/stderr | 3831 | 3831 | PASS |
| runtime/result.json | 435 | 435 | PASS |
| independent/report.md | 5868 | 5516 | PASS |
| independent/proof.json | 12771 | 11967 | PASS |
| receipt-integration.json | 653 | 633 | PASS |

Declared LF normalization independently reproduced, helper read only, never imported/executed/overwritten. All outputs have LF-only text and exactly one final LF; leading/interior lines and trailing spaces preserved. Stdout 661→660 bytes removes only its second final LF; leading blank line remains. **5/5 JSON semantic equivalents** after projection.

[WIRED] PASS: **9/9 new local links**, including **1/1 heading anchor**, resolve at committed revision. Existing inherited links outside this change are not reaccepted. Exact source lines/targets in `proof.json`.
[ANTI-PATTERN / privacy] PASS: all nine selected originals plus decoded JSON string leaves independently scanned for EditMode account/realm/player, WTF account paths, and numeric account tokens: **0 actual identity records**. Complete packet (11 files) + both complete committed owning docs (2 files): **0 mapped identity/private-prefix occurrences; 0 generic home/syncthing/game-identity prefixes**. All nine mapping entries tested even though streams have no discovered identities. Mapping mode **0600**. This is selected privacy scope, not general secret absence.

## Saved execution and source requirement

Saved stdout independently confirms six distinct named lifecycle cases: **6 PASS / 0 FAIL**, actual launcher/wait **exit0**, **0.60s**, 10,970 filtered out. Exact a8 agrees across saved revision/result/independent proof. No replay.

Full stderr: **3,831 bytes / 66 lines / six VM initialization sequences**. Every line is a Startup diagnostic. **Not full Blizzard CLEAN**; argv disables CASC and sound.

Entire saved `tests/animation_query_lifecycle.rs` (333 lines) inspected; direct text comparison equals committed a8 without source-tree hashing. Six named cases cover owner pause/resume/restart/stop, pending Finish before callback, reverse/repeat, actual bounce boundaries, delayed local elapsed, and ordered/parallel delays.

Original requirement preserved, not narrowed: source line298 pauses at elapsed2.75 then calls `Play(true)` **before** reverse traversal. Subsequent tuples prove elapsed/delay at1.5/1.25/1.0/.75. Lines308–332 separately test paused omitted Play retaining reverse and explicit false retaining elapsed origin while switching forward; a later .5 tick asserts `(1.25,true)`. Reverse/repeat case proves .75→.5, forward wrap .25, reverse wrap .75; bounce updates1.25/1.0 reach both callbacks and alternating direction. Exact source line ranges and selector evidence in `proof.json`.

Historical c81 controls, frozen CCD **15204/7/19**, historical suites and security/native limits remain separate and unchanged; eight exact requirement/scope-preservation comparisons pass. Existing receipt says PASS; current executable stat agrees with saved before/after/current evidence. **Stat/TEXT reuse only**, not fresh content authentication.

## Proof scope and ledger

`proof.json` records exact artifact/source revisions, all original/public sizes/digests, JSON equivalence, inventory, links/anchors, per-file privacy counts, saved runtime/source observations, receipt stat, and command/scope/result/invalidation ledger. Later concurrent voice README changes excluded: owning-doc acceptance uses committed `git show` text, not the changed checkout.

No old executable/library/archive/source-tree hashing, re-authentication, runtime replay, build, operations, delegation, tracked edits or commit. No native/PTR/broad-suite/geometry/security/smoothing/performance-cause/full-Blizzard-clean or parent acceptance.
