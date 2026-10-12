# Independent corrected Friend RED audit

Epoch: `retail-red-20261012T121320784488Z` only.

**Verdict: authenticated corrected pre-producer RED. Compile succeeds; runtime 0 passed / 4 failed, exit 101. Not feature acceptance; no native acceptance inference.**

## Authentication and evidence chain

Authenticated only this epoch's `integration-sealed` once, then immediately saved `independent-auth-integration.json`. SHA-256 `547fa955c9a3501bb46ffff3856c98d1d994c70458ee30a1931484984da50781`; 467,809,368 bytes; both match `artifact.json`; inode/size/mtime stable during read. Receipt reused for subsequent audit steps; no rehash.

Read all epoch compile/runtime/toolchain/admission/source metadata and full runtime output. Parsed all 741 compile JSON records: 665 compiler artifacts, 75 build-script records, one successful build-finished, zero compiler-message records. One integration executable record, exactly matching artifact manifest; fresh=false. Compile exit 0, 48.73s. Six iced-wgpu manifest lint-name deprecation warnings remain; not a warning-free build. Cargo 1.99.0, rustc 1.99.0, x86_64-unknown-linux-gnu, LLVM 23.1.1. Default client-retail feature set includes retail-12-1-0.

Parent `controller-red-readiness-20261012T121320697183Z.stdout` contains completed JSON, explicitly naming this epoch and runtime exit 101. Its executed code equals the request JSONL and inspected worker file. Controller stderr empty; systemd submission exit 0 establishes submission, not runtime success by itself. Worker metadata/runtime summary provide execution proof. Compile admission succeeded on third observation; runtime admission on first. No resource block or timeout.

Source revision `41fb7d1603bfc7aa362b55ee28aa088375b9c5c1`. Recorded relevant diff empty; recorded before/after hashes equal. Independent read-only Git inspection confirms current HEAD and relevant source diff match; committed fixture equals inspected file. Recorded source hashes were compared, not rehashed. Correction retains warnings, actual load errors, handler provenance and prefill assertions, while logging missing-requirement observations instead of making all nil probes fatal. Source registration publishes GetFriendInfoByName, not exact GetFriendInfo. Runtime directly proves all four integration cases were wired/executed; integration root invokes the test-harness macro.

## Exact four failure boundaries

| Case | Observed boundary | Not proven/reached |
|---|---|---|
| `exact_friend_lookup_authenticates_secret_name_by_caller_taint` | `runtime.stderr:13-14`; Rust `tests/friend_info_exact.rs:43:6`: **missing exact C_FriendList.GetFriendInfo**, first assert_exact_info rawget/function guard | All secret-value and caller-taint controls |
| `exact_friend_lookup_reads_live_notes_without_environment_leaks` | `runtime.stderr:39-40`; same Rust line and missing-function guard, first environment lookup | Second environment read, live mutation, isolation |
| `exact_friend_lookup_rejects_nonstring_types_and_returns_nil_for_miss` | `runtime.stderr:53-54`; same Rust line and missing-function guard, before input/miss controls | Missing/nil/nonstring rejection, miss value and arity |
| `real_mainline_friend_note_popup_prefills_and_reopens_with_live_note` | `runtime.stderr:75-117`: real handler errors at cached `Blizzard_StaticPopup_Game/Mainline/GameDialogDefs.lua:750`, indexing nil `info`; Rust panic `tests/friend_info_exact.rs:190:6`: **real OnShow note prefill** | Successful first prefill/hide, post-call error-list assertion, note mutation and reopen |

The first three failures are presence guards, not failed semantics inside a working exact getter. No downstream state/security/input acceptance credit.

## Real popup reached unchanged OnShow: YES

Whole dependency closure readiness and Mainline handler-source provenance assertions precede StaticPopup_Show and completed without their own panic. Nine nil-probe observation lines (`runtime.stderr:66-74`) remain visible; they are not equivalent to actual loader errors. The fixture retains no-warning and empty recorded-Lua-error load assertions. This proves readiness for this bounded consumer, not universal completeness of every observed namespace.

Fixture sets FriendsFrame.NotesID to supplied `ExactFriendProbe-Realm` and invokes public StaticPopup_Show. Trace shows `StaticPopup_Show` (`StaticPopup.lua:413`) → Show → `GameDialog.lua:693` → `StaticPopup_OnShow` (`StaticPopup.lua:586`) → actual Mainline OnShow (`GameDialogDefs.lua:750`). Cached handler at lines 748-754 calls GetFriendInfo at 749 and immediately indexes info.notes at 750. No fixture handler/getter replacement exists. Handler provenance assertion identifies the real Mainline source. “Unchanged” describes this exercised vendor handler, not independent vendor byte authentication.

The Lua handler error is caught/logged during Show; fixture continues through shown-dialog and SET_FRIENDNOTE identity assertions, then fails the edit-box prefill assertion. Thus this is now a real consumer failure, not the earlier overbroad readiness failure. Runtime proves info=nil, but does not identify which generated/default provider produced that nil; do not infer direct-call missing-function error inside the popup. The two rendered Lua-error entries repeat the same boundary, not two independent failures.

## Proof limits

No successful note prefill or reopen. No native acceptance inference: synthetic SocialFriend state, exact matching, miss arity, coercion/security policy, catalogs, networking, lifecycle and native parity remain unaccepted. This is an authenticated simulator RED artifact audit only.

No builds, runtime reruns, code edits or delegation. Earlier epoch/proofs untouched. Only this epoch's independent authentication receipt, report and structured proof written.
