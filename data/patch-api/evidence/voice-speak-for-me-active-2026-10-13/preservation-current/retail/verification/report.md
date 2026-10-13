# Retail Speak preservation — saved-execution verification

Epoch: `text-state-retail-red-speak-preservation-20261013T163822997097Z`.
Source: `64a2a57b27c5c2b021375cbfddda557ec145507c` (Retail).

**PASS: bounded voice preservation, 6/6 tests. TextState remains historical RED, not GREEN.**

## Saved evidence coverage

Complete stdout, stderr, argv, launcher, result, and admission inspected for each scope. Full contents retained in `proof.json`; byte lengths match saved results.

| Scope | Passed / failed | Exit | Filtered | Test duration | stderr bytes / lines |
|---|---:|---:|---:|---:|---:|
| runtime-speak-query | 2 / 0 | 0 | 10982 | 0.83s | 1924 / 33 |
| runtime-transcription | 3 / 0 | 0 | 10981 | 0.66s | 2568 / 44 |
| runtime-speak-consumer | 1 / 0 | 0 | 10983 | 6.61s | 6959 / 102 |
| runtime-text-state-red (historical, separate) | 1 / 6 | 101 | 10977 | 0.72s | 7024 / 108 |

Voice scopes: 0 ignored, 0 measured. Full voice stderr: 179 lines / 11451 bytes, startup/workaround/EditMode timing records; **0 warnings, 0 Lua-error lines, 0 panics, 0 addon-load failure lines**. Diagnostic cleanliness derives from complete stderr inspection, not exit status alone. Nonempty timing output is not a failure.

Saved launchers independently report success/status 0 for all three voice scopes (service runtimes 915ms, 695ms, 6.766s). All saved admissions allowed=true. Caller supplies actual admission waited0 for each; admission JSON contains no wait-duration field, so zero is caller-provided rather than independently timed. Launcher `--wait` waits for completion, not admission.

## Contracts reached by saved tests

- **Speak query, 2 PASS:** one false boolean by simulator default; retained function reads live supplied activity; retained value stays stale; two environments remain independent; activity preserved through transcription/channel/connection input changes and restored false.
- **Transcription, 3 PASS:** one false boolean; retained function reads live independent environment input; independent permission and channel snapshots (including detached snapshot mutation and community-stream query); channel clearing and disconnected inputs do not replace supplied transcription activity. Old prospective/unrun source comments do not override actual saved PASS output.
- **Actual cached ChatFrame3 consumer, 1 PASS:** real edit box/prompt and cached `VoiceTranscriptionFrame_UpdateEditBox` exist. Inactive/unmuted disables activation, clears prompt, hides shown edit box; active/unmuted enables activation and uses `REMOTE_TEXT_TO_SPEECH`; active/muted retains activity, disables activation, uses `VOICE_TRANSCRIPTION_MUTED` and red RGB; inactive/unmuted restoration clears/hides again. The entire `lua_errors` collection is asserted empty after cached setup and after every consumer step, including prerequisites—not merely a filtered voice-error subset.

Saved `tests/chat_frame.rs:79-117` constructs the cached environment, discovers/loads Blizzard addons, applies workarounds and fires test startup events. This is bounded cached-consumer evidence, **not full normal CLI startup, native behavior, event transport, audio, or rendering proof**.

Caller states source includes 2630 voice publication. Saved `src/c_api/c_voice_chat_activity.rs` directly shows profile-aware `IsSpeakForMeActive` registration and one boolean from supplied state; commit ancestry/2630 identity was not independently authenticated.

## Boundaries and receipt reuse

Saved argv: same epoch integration executable, read-only source overlay at canonical project path, read-only target, network unshared, private tmpfs, `WOW_SIM_CASC=0`, `WOW_SIM_NO_SOUND=1`, single test thread, nocapture, timeout 86s with TERM/kill-after 2, agents.slice, MemoryMax 4G/MemoryHigh 3G/CPUQuota 200%. These are saved restrictions, not new runs.

Existing integration receipt PASS reused. All runtime result stat tuples equal receipt before/after and current stat-only observation: dev 66306, inode 142364914, size 471033384, mtime_ns 1791909708720754089. **0 new hashes, 0 binary-byte reads, 0 new authentication.** Selected saved source inputs do not authenticate external libraries/cache/settings. Historical compile/auth metadata is not fresh runtime proof.

## Historical TextState RED retained

7 actual tests: 1 PASS, 6 FAIL, exit 101. Five failures stop at missing supplied TextWithState getter record; feed failure stops at initial-event supplied-record boundary. All six panics reference `tests/widget_discrete/fixtures.rs:42:33`. Full historical stderr contains 1 emitted OnEvent Lua error (missing supplied record), 6 panic lines, 0 warnings. Passing case: `text_state_feed_rejects_type_mismatch_unknown_set_without_mutation_or_notification`. Consumer, three query cases, feed-commit case, and secret-ID case failed; downstream assertions remain unreached. Literal “unexecuted RED candidate” diagnostic text does not erase these actual executions. No child duplicate counts added. Voice PASS does not establish TextState GREEN or explain causality.

## Unrun / excluded

`runtime-normal-startup` absent at final readiness inspection: **UNRUN**, not awaited or polled. No inference of full normal startup, native/render parity, causal preservation, parent/full-suite completion, or TextState GREEN. No runtime, replay, build, ops, delegation, canonical edits, or new authentication performed. Only these two epoch-local report artifacts written.
