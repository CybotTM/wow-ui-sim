# Calendar Retail RED retention gate — FAIL

Commit: `708a8bca5084b711648a9a088bf99b0c31c0b84b`. Scope: retention/privacy/link/metadata only; not producer/runtime acceptance.

## Finding

README.md is **1532 bytes**, not requested **2780**. Commit/worktree agree. Current generated manifest receipt matches 1532 bytes; prior stage README is 1365 bytes. No tracked correction made.

## Proof

- 21/21 whole TEXT match complete private originals after exact ordered literal replacements and LF projection; all bytes/digests/line counts/replacement counts match.
- 23/23 public files match commit; private originals and two maps: 23/23 mode0600.
- 741 compile JSONL records preserved; recursive JSON/JSONL, keys/values, slash/unicode/hex/URL/HTML privacy decoding: 73053 nodes, zero private-needle hits. Five decoder self-tests pass.
- Native identifiers/numeric leaves/addresses preserved; no historical serialization or clipping.
- Published status, updated generated receipt and five-ups spec link pass. Prior denial separate; recorded 0/9, exit101, eight MissingSuppliedRecord, one exact day count; all nine downstream contracts unreached.
- Other evidence packets unchanged across parent→commit→HEAD→worktree via Git metadata; old payloads not reopened.

## Check results

- PASS — selection
- PASS — wholetext selected-text-001
- PASS — wholetext selected-text-002
- PASS — wholetext selected-text-003
- PASS — wholetext selected-text-004
- PASS — wholetext selected-text-005
- PASS — wholetext selected-text-006
- PASS — wholetext selected-text-007
- PASS — wholetext selected-text-008
- PASS — wholetext selected-text-009
- PASS — wholetext selected-text-010
- PASS — wholetext selected-text-011
- PASS — wholetext selected-text-012
- PASS — wholetext selected-text-013
- PASS — wholetext selected-text-014
- PASS — wholetext selected-text-015
- PASS — wholetext selected-text-016
- PASS — wholetext selected-text-017
- PASS — wholetext selected-text-018
- PASS — wholetext selected-text-019
- PASS — wholetext selected-text-020
- PASS — wholetext selected-text-021
- PASS — exact_inventory
- PASS — private_originals_maps_mode0600
- PASS — recursive_public_privacy
- PASS — privacy_decoder_selftests
- PASS — published_not_staged
- PASS — README_current_generated_receipt
- FAIL — README_requested_2780_bytes
- PASS — five_ups_spec_link
- PASS — RED_counts_exit_boundaries
- PASS — all_nine_downstream_unreached
- PASS — prior_denial_separate
- PASS — complete_compile_JSONL
- PASS — native_ID_numeric_values
- PASS — commit_only_packet_additions
- PASS — historical_packets_immutable_metadata_only
- PASS — current_files_match_commit

Only private report.md/proof.json written. No build/runtime/authentication/delegation or tracked edits.

Focused ID proof: 726 native opaque leaves unchanged; 226 numeric leaves unchanged; 14 path-bearing ID leaves changed only by authorized privacy mappings. Initial broad suffix check was corrected, not a packet defect. 37/38 checks pass; sole failed check is requested README byte length.
