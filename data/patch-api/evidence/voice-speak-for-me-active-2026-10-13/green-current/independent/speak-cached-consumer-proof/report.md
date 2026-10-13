# Cached Speak For Me consumer — saved-evidence verification

2026-10-13. **PASS: exact bounded cached consumer case. Parent/new-profile/native goals OPEN.** Followed `<HOME>/AgentConfig/skills/verify/SKILL.md` as assigned verifier; no delegation or test/build rerun.

Evidence root: `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/speak-consumer-20261013T144815785593Z/`. Compiled source `978e9bcd15b47ad53dcdfd7aadb0f9ae0afb95b8`; production `a8a148e98`. HEAD remains 978. Production src/Cargo/crates diff a8→978 empty; current production, constructor and consumer diff against 978 empty. Saved/current consumer text equal. Concurrent tracked spec/evidence edits exist; not made by this verifier, and not covered by unchanged-source claims.

## Terminal execution evidence

Read complete runtime stdout, stderr, argv, launcher and result. Full stdout:

```text
running 1 test
test voice_speak_for_me_consumer::cached_voice_edit_box_consumes_live_speak_activity_and_muted_state ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 10976 filtered out; finished in 4.77s
```

Saved argv uses `systemd-run --user --wait`, 86-second timeout with two-second TERM grace, bwrap read-only source snapshot binding and read-only root/target bindings, network unshared, CASC/sound disabled, integration executable, `voice_speak_for_me_consumer:: --nocapture --test-threads=1`. Launcher exit **0**, expected **0**; launcher explicitly reports success and main process exited status **0/SUCCESS**. Runtime 4.901 seconds. Successful submission alone is not the evidence.

Stdout: 224 bytes, 3 content lines. Entire stderr: 6,948 bytes, 102 content lines, all startup/workaround/edit-mode progress. No load FAILED, Lua/runtime warning/error diagnostic or AnimSetup failure observed. `seterrorhandler restored` is informational. Full collection cleanliness additionally rests on executed assertions, not stderr absence alone.

## Actually reached behavior

| Stage | Observable assertion reached in passing case | Proof |
|---|---|---|
| Original full constructor/startup | `crate::chat_frame::setup_env()` creates environment, selects cached Blizzard addon directory, discovers/loads addons, applies existing workarounds, fires startup sequence. Real ChatFrame3/editBox/prompt and cached consumer exist. | Saved constructor `tests/chat_frame.rs:73-118`; consumer `:28-42`; PASS |
| False/unmuted | Query false and mute false; explicit Show confirmed, then activation disabled, prompt empty and edit box hidden. | Consumer `:44-58`; PASS |
| True/unmuted | Query true and mute false; activation enabled and REMOTE_TEXT_TO_SPEECH prompt. No true-Show requirement. | Consumer `:60-71`; PASS |
| True/muted | Activity remains true; mute true; activation disabled; VOICE_TRANSCRIPTION_MUTED prompt; RGB equals RED_FONT_COLOR within 0.000001. | Consumer `:73-91`; PASS |
| False/unmuted restoration | Query false/mute false; explicit Show confirmed, then activation disabled, empty prompt and hidden again. | Consumer `:93-107`; PASS |

No early return, ignored case or expected-failure annotation. All stages run sequentially in the sole passing test. `assert_no_runtime_errors` clones **entire** `state.lua_errors` and asserts empty (`:7-12`); constructor check `:29`, then prerequisites and all four stages through `exec_consumer_step` (`:15-18`). No error reset/clear/filter. Exec result is separately unwrapped after the collection check. Constructor logs individual loader failures and ignores some event results; collection assertion is not a claim about every possible failure channel. Full saved stderr contains no loader failure.

## Genuine cached consumer boundary

Read cached `<HOME>/.cache/wow-ui-sim/blizzard-ui/retail/AddOns/Blizzard_ChatFrame/Shared/VoiceChatTranscriptionFrame.lua:123-145,201-202`. It queries activity/mute, chooses muted red prompt / active white prompt / empty prompt, assigns `disableActivate = muted or not speakForMeEnabled`, and hides when false. **No Show on true.** White RGB is source behavior, not asserted by this case. Cached file initializes ChatFrame3 directly.

Test supplies only Rust activity/mute inputs and directly invokes the real cached function. No fake globals, replacement function, vendor patch or source-copy consumer implementation added. Original constructor's existing workarounds remain; inspected chat voice workaround affects ChatFrame1/channel button/QuickJoin, not ChatFrame3 or this consumer. Saved source src search finds no consumer replacement. Commit 978 adds only consumer test and spec delta. Existing saved source snapshot/read-only launcher binding is inspected evidence, not a new source copy made by verification. No independent cache provenance/native identity claim.

This proves **direct-function decision behavior**, not activity event production/delivery, transport, native semantics/defaults/transitions, rendering or new-profile acceptance.

## Compile / source ledger

- Full compile.stdout parsed: **741 JSON records** = 665 compiler-artifact + 75 build-script-executed + 1 build-finished(success=true); **0 compiler-message records**. Saved terminal Result=success, SubState=exited, ExecMainStatus=0. Full compile.stderr read: six iced-wgpu-patched manifest deprecations plus aggregate six-warning line and compile completion. Not warning-free.
- Saved toolchain: cargo 1.99.0 (5f94df478, 2026-08-27); rustc 1.99.0 (b940084d7, 2026-09-28), x86_64-unknown-linux-gnu, LLVM 23.1.1. Saved build argv: cargo test --test integration --no-run --offline --locked -j4 --message-format=json-render-diagnostics. No build executed by verifier.
- Reused `verification/speak-consumer-source/report.md` and `proof.json`: direct rustfmt exit0/empty output and manual changed-line readability PASS at unchanged 978 consumer; no formatter/readability rerun. Source gate previously proved module discovery and no new anti-patterns. Saved execution supplies new consumer acceptance, not a new source gate.
- Receipt **stats only**: current executable stat dev=66306, ino=141832122, size=470905432, mtime_ns=1791902978844382397 matches recorded before/after stats. No executable/library content read, hash or authentication performed; recorded authentication fields are not independently endorsed.
- Query2/transcription3/normal-startup/default library-bin check remain separate existing gate3735 at `verification/speak-query-current-proof/`; not rerun or added to this consumer count.

Only new verification artifacts written. No tracked edit, auth, build/runtime execution, operational action, delegation, vendor write, source copy or executable/library read/hash. One metadata-count evaluation failed before inspection because `root` was undefined; corrected with explicit binding, no command rerun. Report/proof persisted and read back.
