# Speak For Me consumer — bounded source gate

Date: 2026-10-13
Revision: `978e9bcd15b47ad53dcdfd7aadb0f9ae0afb95b8` (HEAD observed)
Repo: `<REPO>`
Verdict: **PASS — source-only gate. Consumer compilation/execution NOT established.**

Followed `<HOME>/AgentConfig/skills/verify/SKILL.md` as assigned verifier (no delegation) and `<HOME>/.config/pi/agent/skills/rust-readability/SKILL.md`. Scope: new `tests/voice_speak_for_me_consumer.rs` and spec delta only. No tracked edits, Cargo/build/runtime/auth/operations/vendor writes or old-binary inspection.

## Evidence matrix

| Check | Result | Source evidence / limitation |
|---|---|---|
| EXIST / SUBSTANTIVE | PASS | Commit adds 108-line consumer test and modifies only `docs/specs/voice-speak-for-me-active-input.md`. Three concrete helpers and one behavioral `#[test]`, no placeholder bodies. |
| WIRED | PASS (source) | `tests/integration.rs:1` invokes harness macro; `crates/test-harness/src/lib.rs:7` includes generated macro; `crates/test-harness/build.rs:29-66` generates modules from discovered top-level Rust files; `:441-453` discovers them and `:462-476` exclusion list excludes neither voice file. No generation/build executed. |
| Constructor / gates | PASS (source) | New file `:3` requires both `gui` and `retail-12-0-5`, `:28` calls `crate::chat_frame::setup_env()`. Unchanged `tests/chat_frame.rs:1` GUI gate and `:78-98` pub(crate) constructor discover/load Blizzard addons, apply existing post-load workarounds and fire startup. Parent-to-commit constructor diff empty; working-to-commit constructor/consumer diff empty. Cache resolver at `src/client_profile.rs:307-322` selects profile cache. Existing constructor logs loader failures / ignores some event returns; new full-error assertion proves only collected errors if executed, not every possible failure channel. |
| Full error collection | PASS (source) | `tests/voice_speak_for_me_consumer.rs:7-12` clones entire `state.lua_errors` and asserts empty, without filtering/clearing/reset. `:16-18` checks after every exec before unwrapping its result; `:29` checks constructor/startup. Four supplied-input stages are each followed by exec/error checks. |
| Actual false / true / muted behavior | PASS (source) | Cached `VoiceTranscriptionFrame_UpdateEditBox` at `VoiceChatTranscriptionFrame.lua:123-145` queries activity and mute; muted sets muted prompt/red color, true/unmuted sets remote prompt/white color, else clears text; assigns `muted or not speakForMeEnabled`; false calls Hide; no Show call. Test `:44-58` and `:93-107` first Show and assert shown, then assert false query, disabled activation, empty prompt and hidden. `:60-71` asserts true query, enabled activation and remote prompt, with no true-Show expectation. `:73-91` asserts true remains true when muted, disabled activation, muted prompt and red RGB with tolerance. White RGB is vendor source behavior but is not asserted or required by this spec delta. |
| Genuine consumer / no fake surface | PASS (source) | `:30-42` asserts real ChatFrame3/editBox/prompt and cached function exist; `:21-24` writes only simulator voice input fields. Lua blocks directly call cached function and C_VoiceChat queries; no replacement globals, stubs or vendor patch. Commit file list contains only test/spec. Existing constructor workarounds remain unchanged. |
| Separate inventories | PASS | `tests/voice_speak_for_me_active_input.rs`: 2 `#[test]` cases; consumer file: 1. Spec preserves separate input-query and staged consumer inventories; previous query/transcription receipts do not establish consumer execution. Spec consumer acceptance checkboxes remain unchecked. Historical execution receipts in prose were not revalidated by this gate. |
| ANTI-PATTERN | PASS | New file counts: TODO=0, FIXME=0, HACK=0, XXX=0, `#[allow`=0, `rawset`=0, consumer replacement declaration=0, error clear/reset=0. Manual read: no commented-out code or empty error handler. |
| Changed-line readability | PASS | Read all 108 new Rust lines and embedded Lua. Helpers expose assertions, execution and input mutation; no hidden I/O helper or warning suppression, primitive parameter overload, dense Boolean chain, excessive nesting or state-accumulation pattern. Test is below 200-line test threshold; false restoration intentionally repeats observable assertions across a different transition. Manual review only; no metrics program executed. No violations found. |
| Formatting | PASS | Direct `rustfmt --check <REPO>/tests/voice_speak_for_me_consumer.rs`: empty stdout/stderr, exit 0. New file only; no cargo fmt or build. |

## Cached vendor source

Read-only evidence: `<HOME>/.cache/wow-ui-sim/blizzard-ui/retail/AddOns/Blizzard_ChatFrame/Shared/VoiceChatTranscriptionFrame.lua`.

`VoiceTranscriptionFrame_UpdateEditBox` assigns `self.editBox.disableActivate = muted or not speakForMeEnabled` and only hides when activity is false. It does **not** show on true. `:201-202` initializes the real `ChatFrame3`. Event handler references exist in the cached file, but this test directly invokes the edit-box consumer. Event production/delivery, native activity/defaults/transitions and rendering are **unknown/outside this gate**. Cache was inspected as present source, not independently proven native or committed provenance.

## Proof ledger

- `git -C <repo> show --stat --oneline 978e9bcd1`: exit 0, test/spec only, 117 insertions / 2 deletions.
- `git -C <repo> rev-parse HEAD`: exit 0, full revision above.
- `git -C <repo> show --format=fuller 978e9bcd1 -- docs/specs/voice-speak-for-me-active-input.md`: exit 0, full delta inspected.
- `git -C <repo> diff 978e9bcd1 -- tests/chat_frame.rs tests/voice_speak_for_me_consumer.rs`: exit 0, empty.
- `git -C <repo> diff-tree --no-commit-id --name-status -r 978e9bcd1`: exit 0, exactly M spec / A consumer.
- `git -C <repo> diff 978e9bcd1^ 978e9bcd1 -- tests/chat_frame.rs`: exit 0, empty.
- Direct rustfmt command above: exit 0, empty output; executed once.
- Literal marker scan and test-count scan: full output inspected, counts above. Source lines from read tools/harness/cache inspected directly.
- Initial git commands without explicit repo failed (not a git repository, exit 128); corrected with `-C`. One subsequent evaluation failed because local `repo` variable was undefined before commands executed; corrected with explicit binding. Find tool unavailable (`fd` absent); cached filename discovered using Python Path traversal, no vendor mutation.
- `git -C <repo> status --short` before artifacts: only `?? .code-index.db` and `?? verification/`; no tracked modifications observed. Artifacts written only under `verification/speak-consumer-source/`.

No test pass count, compilation success, runtime-cleanliness outcome or execution acceptance is claimed. Source proof remains bounded to the inspected revision and cached source.
