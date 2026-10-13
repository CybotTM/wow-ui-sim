# Speak query: saved runtime/check/source proof

Verified 2026-10-13. **PASS within supplied-query scope; parent goal OPEN.** Followed `<HOME>/AgentConfig/skills/verify/SKILL.md` as bounded artifact verifier. No runtime/build/check/formatter reruns, operations, delegation, tracked edits, binary hashing or authentication. Receipts inspected for recorded stats only; not independent executable identity proof.

## Saved runtime coverage

Evidence root: `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/speak-active-green-retry1-20261013T143805954256Z/runtime-query-preservation`. Saved source `a8a148e98d8dc32cf81e789fc8926830c9ed0161`. Full stdout/stderr, argv, launcher and results read for all three processes. Each argv uses `systemd-run --wait`; each saved launcher exit is 0 and launcher reports success, exited status 0. These are terminal results, not merely successful submission.

| Case | Concrete coverage | Saved result |
|---|---|---|
| `voice_speak_for_me_active_input::` | Zero-argument function/one boolean/default false; retained function reads false→true→false; retained value stays false after true transition; independent environments; true activity survives transcription enable plus channel-clear/disconnect fixture | 2 passed, 0 failed/ignored/measured, 10974 filtered; exit 0 |
| `voice_transcribing_input::` | Retained live query; one false boolean default; transcription independent of permission/channel snapshots | 3 passed, 0 failed/ignored/measured, 10973 filtered; exit 0 |
| Fresh normal startup | Separate `wow-sim --no-addons --no-saved-vars --exec-lua … lua-errors`, Blizzard addons loaded, CASC/sound disabled | Exact stdout below; exit 0 |

```text
SPEAK_ACTIVE_DEFAULT	function	1	boolean	false
[]
```

This normal load is not GUI, native-client, consumer-transition or all-profile acceptance. False is simulator policy. Permission is observed false, not independently varied; channel clearing/disconnection is a combined fixture.

## Full diagnostic classification

Counts explicitly distinguish non-whitespace **content lines** from `split('\n')` entries (including terminal empty entry):

| Artifact | Bytes | Content lines | Newline split entries | `splitlines()` |
|---|---:|---:|---:|---:|
| speak-input stdout | 328 | 4 | 8 | 7 |
| speak-input stderr | 1925 | 33 | 34 | 33 |
| transcription stdout | 403 | 5 | 9 | 8 |
| transcription stderr | 2568 | 44 | 45 | 44 |
| normal stdout | 49 | 2 | 3 | 2 |
| normal stderr | **14714** | **219** | **220** | **219** |

All 33/44 test stderr lines are informational environment-initialization progress. Normal stderr: 213 informational progress/configuration/loading/workaround lines, one identity-bearing cache-information line (values withheld), five final summary lines. No observed runtime warnings, Lua/runtime error diagnostics or AnimSetup failures in any of these three logs. Informational `seterrorhandler restored` entries are not failures; bytecode `replay_fail=0` and `store_fail=0` counters are not failure reports. Sound/CASC/addon/SavedVariables disable messages are configuration, not warnings. Normal font fallback is logged runtime behavior, not an added verification fallback.

Normal stderr lines 215–219 explicitly report:

```text
=== Final observed Lua error summary ===
Status: CLEAN
Lua errors: 0 unique, 0 occurrence(s)
Attributed owners: 0
Unattributed Lua errors: 0 occurrence(s)
```

Cleanliness rests on full stderr inspection plus explicit summary, **not `[]` alone**. `proof.json` classifies every content line without reproducing identity values.

## Compiler/check ledger

1. GREEN compile root `speak-active-green-retry1-20261013T143805954256Z`: full stdout parses to **741 JSON records**: 665 compiler-artifact, 75 build-script-executed, one successful build-finished. Zero compiler-message records. Saved terminal `SubState=exited`, `Result=success`, `ExecMainStatus=0`. Command: `cargo test --test integration --no-run --offline --locked -j4 --message-format=json-render-diagnostics`, saved a8 source. This is compilation evidence, not execution of all integration tests.
2. Default-check root `speak-active-default-check-20261013T144318269352Z`: **733 JSON records**: 658 compiler-artifact, 74 build-script-executed, one successful build-finished. Zero compiler-message records; terminal exit 0. Command: `cargo check --offline --locked -j2 --message-format=json-render-diagnostics`. Saved source `9b027baaf8931e1e3780a33b0a7fd098e9476f87`; its delta from a8 is three wiki files only. Exact scope: default library/binaries, **not integration-test targets**. Later consumer-only test addition does not invalidate this production check and is not covered by it.
3. Both complete compile stderr logs contain **six iced-wgpu-patched manifest deprecations**, plus one aggregate six-warning line and compile/check progress/completion. Deprecated hyphenated Clippy lint names: large-enum-variant, map-entry, match-wildcard-for-single-variants, redundant-closure-for-method-calls, trivially-copy-pass-by-ref, type-complexity. No compiler-message records does **not** mean warning-free. No changes made to those manifests.
4. Saved native toolchain: Cargo 1.99.0 (5f94df478, 2026-08-27), rustc 1.99.0 (b940084d7, 2026-09-28), x86_64-unknown-linux-gnu, LLVM 23.1.1. Full saved toolchain and command/terminal records retained in proof.json.

## Reused source/format/readability proof

Reused `verification/speak-active-producer-source`: target a8, direct rustfmt and cargo fmt checks both exit 0 with empty stdout/stderr; changed-lines manual readability PASS with zero violations. No formatter rerun. Current producer/state/input-test bytes compare equal to a8. Explicit registration reads the separate state boolean and pushes exactly one `Val::Bool`; no permission/channel aggregation, mutation, event or setter. Source wiring/anti-pattern evidence remains the prior source gate, not a new broad audit.

Observed HEAD at final source comparison: `978e9bcd15b47ad53dcdfd7aadb0f9ae0afb95b8`. Concurrent docs/evidence and new consumer-test work are outside this bounded proof. Saved whole-checkout formatting does not cover newly added Rust; unchanged producer/input-test formatting remains valid. Any later intersecting production/source change invalidates the relevant proof; no immutable whole-checkout identity claim.

## OPEN

Cached consumer runtime acceptance; new-profile coverage; native meaning/defaults/parity; events/transport; parent goal. No promotion to overall completion. `report.md` and `proof.json` persisted under `verification/speak-query-current-proof/` and read back.
