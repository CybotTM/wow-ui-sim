# Speakquery2RED and bounded animation saved-evidence verification

Verified: 2026-10-13. Mode: saved-artifact inspection under verify/SKILL.md; no delegation or execution of saved launchers.

Private epoch: `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/speak-active-red-refined-20261013T143003506802Z`.
Saved source revision: `c81f10375ae50b36f077757e9ba38f3a162b9d17`; production-delta attribution: `50f6f0ea8` (revision.json). Current and saved Speak/animation test files are byte-identical; this does not establish whole-checkout identity.

## Capability matrix

| Exact scope | Result | Minimum established / missing coverage | Proof |
| --- | --- | --- | --- |
| Speakquery2RED / `voice_speak_for_me_active_input::` | FAIL: 0 PASS, 2 FAIL; 0 ignored/measured; 10974 filtered; actual wait exit101 | Existing callable query type checks pass on reached paths; subsequent Lua result assertions fail. Not a missing-function failure. | Full runtime stdout/stderr/argv/launcher/result; saved source test |
| `speak_for_me_active_is_one_false_boolean_by_default` | FAIL at tests/voice_speak_for_me_active_input.rs:23:6 | Environment creation and Rust type check at lines10–15 complete. Lua block lines18–20 is entered; at minimum its first assertion is reached. Failure could be arity, boolean type, or false default; generic `assertion failed!` supplies no actual value or precise Lua assertion. | Runtime stderr + sequential source |
| `retained_speak_for_me_query_reads_live_independent_environment_input` | FAIL at tests/voice_speak_for_me_active_input.rs:44:10 | Both environments construct. First loop iteration's function-type check completes; query is retained and invoked, reaching `assert(retained_speak_value == false)` at line41. Actual returned value/type/arity unknown. Second iteration's query checks are unreached. All supplied-input mutations, retained/direct true checks, channel/transcription independence and final false/other-environment true transitions from line47 onward are unreached. | Runtime stderr + sequential source |
| runtime-animation-reverse-timeline / `animation_query_lifecycle::delaying_uses_order_start_and_longest_parallel_duration` | PASS: 1 PASS, 0 FAIL; 10975 filtered; actual wait exit0 | Exact ordered/parallel forward and reverse elapsed/delaying timeline, paused omitted-direction preservation, explicit-false forward resume and preserved elapsed origin. Directory label is not a broader reverse-animation suite. | Full saved streams; `--exact` argv; source lines264–333 |
| runtime-animation-bounce-loop / `animation_query_lifecycle::bounce_loop_callback_observes_direction_change` | PASS: 1 PASS, 0 FAIL; 10975 filtered; actual wait exit0 | Two OnLoop callbacks, callback state checks and alternating reverse direction after explicit updates1.25 and1.0. | Full saved streams; `--exact` argv; source lines180–203 |

The Speak type PASS is inferred from control reaching the later `env.exec(...).unwrap()` failures, not an additional passing test count. Retained query invocation has no call error; its sole Lua result assertion fails. No failure value is reconstructed from source implementation.

## Saved build and executable provenance

Read all741 JSON records in compile.stdout: final `{"reason":"build-finished","success":true}`; zero compiler-message records. compile-terminal.txt records Result=success, SubState=exited, ExecMainStatus=0. submission.json records compile-launcher exit0. Compilation is not the Speak failure boundary.

Full compile.stderr contains six manifest deprecations from iced-wgpu-patched/Cargo.toml: large-enum-variant, map-entry, match-wildcard-for-single-variants, redundant-closure-for-method-calls, trivially-copy-pass-by-ref, type-complexity; final summary says6 warnings. Build is successful, not warning-free. Saved toolchain: Arch cargo1.99.0, rustc1.99.0, x86_64-unknown-linux-gnu.

Reused receipt-integration.json and seal-integration.json only. Current sealed executable stat matches saved receipt stat_after: device66306, inode141707785, size470892264, mtime_ns1791901875274548805. Saved receipt says authentication PASS/digest_matches true; this verification performed no binary read, hash or new authentication. Stat equality is metadata continuity only, not cryptographic reauthentication or whole-runtime/source/dependency equivalence.

## Full runtime inspection

For each of the three runtime directories, read complete stdout, stderr, argv.json, launcher.txt and result.json, and complete corresponding run-*.py launcher source without executing it. Speak stderr2671 bytes shows three successful environment constructions followed by two generic Lua assertion panics; no setup/compile failure. Reverse stderr638 bytes and bounce stderr639 bytes contain successful environment startup only, no additional runtime errors/warnings. Parent counts agree with result receipts and actual systemd-run --wait termination; no duplicate child counts added. Saved limits: timeout86s, TERM/kill-after2, memory max4G/high3G, CPU200%, test-threads1, bwrap read-only source/network isolation, CASC0 and no sound1. These describe historical invocation, not newly exercised isolation or resource guarantees.

## Bounds / frozen history

No runtime rerun, build, operational mutation, delegate, tracked edit, commit, deployment, vendor edit or binary authentication. Only verification/speak-red-animation-proof/report.md and proof.json are written and read back. No fresh runtime acceptance claim; these are current inspections of saved execution evidence.

Native WoW defaults/events/transport/security parity, whole animation subsystem, other profiles, full suite and parent acceptance remain unproven. Frozen CCD suite FAIL remains unchanged; this packet neither rechecks nor rewrites it. Existing wiki statement in docs/wiki/investigations/integrated-source-and-factory-proof-2026-10-09.md:21 preserves that boundary. Two bounded animation PASS results do not repair or supersede historical suite failure or Speak RED.
