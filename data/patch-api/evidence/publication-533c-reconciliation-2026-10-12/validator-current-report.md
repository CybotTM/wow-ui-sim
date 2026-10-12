# Validator current verification — FAIL

Revision: `9424d198177f2ae210a00d6409d9472f6be0cca6`.

## First boundary: fixture isolation

`tools/test_patch_8_1_0_validation.py:39-41` symlinks every sibling evidence directory into the temporary root. This includes the real reconciliation directory containing `current-publication-results.json`. `store()` at lines 58-63 checks only whether the final file is a symlink, not its parent. `publish_current()` at lines 65-71 therefore writes through that parent symlink into the real capture. Rejection mutations at lines 122-138 would also overwrite it. This is a concrete write path, not an implementation-shape test assertion.

Stopped before command execution to preserve real evidence. Both requested commands have execution count **0**, exit **null**; no fresh eight-test or validator PASS claim. Private `validator-final/` contains exact argv/cwd metadata and empty stdout/stderr artifacts explicitly marked NOT_RUN. No retries, source edits, capture rewrites, broad tests, builds, Rust commands, delegation, seals, or native parity claims.

## Read-only observations

Committed validator, tests, capture, and 52-gap fixture match working files. HEAD is the requested revision; only untracked paths were present. Real current JSON: 145 rows, 93 OK, 52 gaps; failed identities match the committed fixture. Frozen JSON: 145 rows, 87 OK, 58 gaps.

Development stderr proves **two** genuine RED assertion failures at the historical/current conflations, then eight GREEN tests. It does not prove all eight tests each had RED. Pre-rename GREEN and the ledger's `invalidated: false` cannot replace fresh committed verification.

Tests use concrete fixture mutations and output/exception assertions, not source-substring or construction-shape assertions. However the fixture write path invalidates safe acceptance/rejection execution against the real capture. Six proof-reference review, original removal/proof-hash review, full frozen preservation comparison, and generic replacement diff verification remain unverified after the stop boundary.

Full-suite `533c`: **actual FAIL**. Corrected prefork one-case PASS remains bounded prior evidence only. Parent: **OPEN**.
