# Independent Bounty publication / retention snapshot

**SCOPED_PUBLICATION_RETENTION_PASS_DOC_RECONCILIATION_PENDING**. Bounded saved-evidence gate only; no new execution. Owning epoch `publication-prefork-compile-20261012T030415401015Z`, revision `e5277a5bf3b0ceeab0b6f9786e5883e0eebcf94c`. Snapshot 2026-10-12T03:12:39.426164+00:00.

## New seal authenticated before long output

`publication-independent-auth.json` was saved immediately following the single streaming payload read, before compile/runtime/retention output. New `prefork-sealed`: **509599256 bytes**, SHA-256 `e6fc78d2ff76f0aca532fc1a39be2445d917ef105886ff77b3b1ce8c9c10e2a3`; digest and size match artifact metadata. No second read. All741 compile JSON records parsed; exact selected Cargo artifact matches metadata, target `prefork_full_ui`, `fresh:false`, final `build-finished success:true`. Saved compile exit0/source unchanged. Actual recorded local cargo/rustc1.99.0, x86_64-unknown-linux-gnu, LLVM23.1.1; six inherited iced manifest warnings retained, not warning-free proof.

## Exact owning runtime proof

Recorded selector `patch_8_1_0_publication_sweep::patch_8_1_0_publication_sweep --exact --nocapture --test-threads=1`, sealed executable, `/usr/bin/timeout 90`, repository cwd. Saved runtime exit0. Complete stdout:

```text
running 1 tests
test patch_8_1_0_publication_sweep::patch_8_1_0_publication_sweep ... ok

test result: ok. 1 passed; 0 failed; 1 total
running 0 tests

test result: ok. 0 passed; 0 failed; 0 total
running 0 tests

test result: ok. 0 passed; 0 failed; 0 total
running 0 tests

test result: ok. 0 passed; 0 failed; 0 total
```

Exactly **1 PASS / 0 FAIL**, followed by **three zero-case groups**, not extra executed cases. Generic `summary:null` is parser nonrecognition, not missing evidence. Complete8731-byte/138-line runtime stderr inspected. Public-query sweep calls shared `run_publication_sweep` (`tests/patch_8_1_0_publication_sweep.rs:13`); `tests/common/publication_sweep.rs:294-318` persists results then asserts exact non-OK ID set equals reviewed fixture.

`publication.json`: **145 classification rows, 90 OK / 55 current known gaps**; not145 behavior cases. False-ID set equals current55-entry fixture exactly. `wt-global-api-C_Map.GetBountySetMaps-77`: expected `published`, observed `raw=function; lookup=function`, `ok=true`.

## Source and retained boundaries

`e5277a5bf` removes only Bounty row77 from current fixture:56→55, no additions, all other entries/order unchanged. Current fixture matches exact commit. All six producer/registration/state/test files byte-match `262987b1a65f049d5c3e158b18a8eb73cc22e4ba`; test SHA-256 `1eec1a5c32e2b610c6e485972b55af6f2e906af41f903104d2a1e457c4c27f56`. Details: `publication-independent-source.json`.

Reused `green-independent-report.md` and original root `proof.json` unchanged, with **11 logical cases per Retail/PTR =22 scope executions**, not22 distinct cases. Four durable authentication receipts have matching hashes/sizes; no GREEN payload reread. Both normal outputs exactly `[]\n`, with Bounty empty/fresh/raw publication, retired old getter absent and all Calendar3 triples preserved. Saved fmt/default check exit0 reused without rerun.

Frozen historical receipt still **145 rows /87 OK /58 gaps**; accepted Calendar publication receipt still **145 rows /89 OK /56 gaps**. Their files match exact e527 git objects and both historical evidence directories have empty262→e527 diffs. Counts represent separate snapshots, not contradictory current results. No old Calendar binaries/libraries/cache payloads read.

**Expanded RED authentication GAP remains.** Initial RED6 authenticated proof reused; expanded RED11 behavior fully observed and retained, but prior printed digest comparison was lost and no durable auth receipt exists. New seal authentication PASS does not authenticate expanded RED; no old payload reread or gap concealment.

## Retention and scoped links

Manifest as observed: **77 nonbinary copies /250064 bytes**, all77 private-source and public-copy SHA-256/size pairs match declared values. Every declared path exists. README and manifest are extra public files, not counted as copies. No binary/library/vendor/cache payload in retention audit. Full rows: `publication-independent-retention.json` and publication proof.

**14/14 scoped relative Markdown links/anchors PASS**: entire Bounty evidence README/spec, Bounty sections in both investigation pages, e527-added wiki index/log links. File hashes and link destinations saved in `publication-independent-linkaudit.json`. This is not an unrelated whole-wiki audit.

## Documentation phase boundary

At inspected snapshot, README says owning publication/retention gate pending; committed spec/wiki still say independent GREEN/owning sweep pending and default check queued despite saved completed receipts. Main may reconcile concurrently. This report proves bounded saved artifacts/publication/retention, **not final reconciled documentation closure**. Link/source hashes bind this snapshot; later edits are not covered automatically.

No build/test/check reruns, polling, operations, delegation, repository edits, adjacent tasks or old RED/GREEN/Calendar executable/library/cache payload reads. No full BountyBoard lifecycle, native catalog/order/unknown-ID, all-profile, broad-suite or full-handoff acceptance. Historical broad3FAIL and parent OPEN remain.

Artifacts written only under private ROOT: `publication-independent-auth.json`, `publication-independent-source.json`, `publication-independent-retention.json`, `publication-independent-linkaudit.json`, `publication-independent-proof.json`, this report. Original GREEN `proof.json` preserved.
