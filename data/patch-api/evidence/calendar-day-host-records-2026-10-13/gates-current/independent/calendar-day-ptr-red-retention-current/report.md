# Calendar PTR RED selected TEXT retention gate

**PASS — commit `4def990f697a664449f4b398e91ecd8743196129`.**

40 checks; 0 failures. 21/21 exact whole ordered-literal + LF projections; 23 public files.

## Evidence
- Private original + projected receipts and replacement ledgers match all 21 texts. Line counts and native IDs/opaque addresses preserved; no clipping or historical JSON/JSONL reserialization.
- All 741 compile JSONL records and all 13 JSON documents decoded; embedded keys/values recursively checked through slash, Unicode/hex, URL, and HTML escapes. No privacy findings. Decoder canaries cover each encoding and nested JSON/JSONL.
- Generated README receipt computed from actual bytes: 1583 bytes, SHA256 `2cd3689d1a1aa5f34d74d8006dac82ec40b106461dcc7a7f6149d622bd5c4dd1`. No unrelated sampling/2780-byte rule imposed.
- Five-up owning-spec link resolves to `docs/specs/calendar-day-event-records.md`; current publication status remains independent gate PENDING.
- RED remains 0/9, exit101: eight owned-record guards, one exact day count. All five downstream groups remain unreached; producer GREEN not awaited.
- Original auth missing-fullcompile.stderr claim retained verbatim under allowed projection. Separate addendum explains typo; complete expected compile.stderr is retained (nine lines). No new copy authentication.
- 23578 prior evidence metadata entries unchanged before/after commit; all 23 commit changes are additions inside the new packet. Old payloads not reopened.
- All 49 stage files are 0600. No tracked edits, builds, runtime, executable/archive/source/library reads or hashes, or delegation.

## Limits
Private staged originals establish retention/projection, not native provenance or producer behavior. Historical receipts/reports remain claims, not fresh authentication. Detailed per-file receipts, ledger and scan results in `proof.json`.

## Checks
- PASS: `mapping_order`
- PASS: `inventory`
- PASS: `whole:epoch/admission.json`
- PASS: `whole:epoch/auth-report.md`
- PASS: `whole:epoch/compile-result.json`
- PASS: `whole:epoch/compile-terminal-status.txt`
- PASS: `whole:epoch/compile.stderr`
- PASS: `whole:epoch/compile.stdout`
- PASS: `whole:epoch/receipt-integration.json`
- PASS: `whole:epoch/revision.json`
- PASS: `whole:epoch/seal-integration.json`
- PASS: `whole:epoch/source-scope.json`
- PASS: `whole:epoch/submission.json`
- PASS: `whole:epoch/toolchain.json`
- PASS: `whole:epoch/runtime-calendar-day-red/admission.json`
- PASS: `whole:epoch/runtime-calendar-day-red/argv.json`
- PASS: `whole:epoch/runtime-calendar-day-red/launcher.txt`
- PASS: `whole:epoch/runtime-calendar-day-red/result.json`
- PASS: `whole:epoch/runtime-calendar-day-red/stderr`
- PASS: `whole:epoch/runtime-calendar-day-red/stdout`
- PASS: `whole:independent/report.md`
- PASS: `whole:independent/proof.json`
- PASS: `whole:independent/auth-filename-addendum.md`
- PASS: `total_receipts`
- PASS: `public_inventory_exact`
- PASS: `generated_readme_receipt`
- PASS: `owning_spec_link`
- PASS: `published_pending_status`
- PASS: `complete_jsonl`
- PASS: `recursive_privacy`
- PASS: `privacy_decoder_canaries`
- PASS: `private_stage_permissions`
- PASS: `commit_scope_only_new_packet`
- PASS: `historical_packets_git_metadata_only`
- PASS: `published_bytes_at_requested_commit`
- PASS: `red_wait_not_green`
- PASS: `unreached_contracts_retained`
- PASS: `original_auth_gap_and_separate_addendum`
- PASS: `complete_stage_permissions`
- PASS: `no_tracked_edits`
