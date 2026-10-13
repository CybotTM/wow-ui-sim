# Saved Mists query runtime RED — CONFIRMED (implementation FAIL)

Bound exclusively to `abb979c4465efcf75c6731dd08077e90abaf5509`, not current HEAD or later publication changes. Features `client-mists,prefork-full-ui`; no GUI/CASC/sound.

## Exact counts

- Initial `runtime/`: precondition FAIL, wait exit 1, zero test executions; stdout 0 bytes, stderr 90 bytes. Read-only bwrap target mountpoint creation failed. Retained; no RED credit.
- Actual `runtime-retry1/`: **0 passed / 2 failed**, 0 ignored, 0 measured, 7859 filtered out; **0.33s**, wait exit **101**. Full stdout 546 bytes and stderr 2716 bytes read.
- Saved compilation: 385 JSON records, build-finished success=true; six manifest deprecations plus eight source warnings, not warning-free. Historical be2 E0603 is a separate caller-reported compiler precondition FAIL, not RED.

## Reached boundaries

| Test | Reached | Failure | Unreached |
|---|---|---|---|
| documented_profile_query_returns_one_false_boolean | Environment creation, explicit host false assignment, query invocation, one-result arity assertion passed | saved test line 15: `activity must be a boolean, got nil`; Rust unwrap at 19:6 | false-value assertion line 16 |
| documented_profile_retained_query_reads_live_isolated_activity | Two environments created, query captured, explicit host true assignment, retained invocation, one-result arity assertion passed | saved test line 35: `retained activity must be a boolean, got nil`; Rust unwrap at 39:10 | true-value assertion line 36; isolation assertion line 41; false restoration mutation line 43; retained false assertion line 45 |

Arity success proves exactly one return value, not boolean type. Both captured values report type nil. Creation of two environments does not prove isolation. Later isolation/restoration never executed.

## Authentication reuse / limitations

Existing authentication receipt reused without rehash or reauthentication. Current executable stat matches all saved receipt fields; exact stat recorded in proof.json. Metadata continuity is not new content authentication. Saved source mount and external libraries/cache/settings remain outside executable authentication.

Only supplied-state Mists query result boundary gets RED credit. No native/default/event/activity-producer/full-Blizzard-startup/other-profile claim. `prefork-full-ui` feature alone does not establish full startup. No claim about later publication changes or current HEAD.

No runtime rerun, build, hash, authentication, operational command/mutation, code edit, delegation, or commit. Only read/stat inspection and requested report artifacts. Initial report-generation evaluation stopped at undefined Path before writes; imports corrected without test/command rerun.

## Full saved text evidence

### source/tests/voice_speak_for_me_profile_input.rs

```text
//! Query-only profile contract; native activity producers/defaults remain unknown.

#![cfg(any(feature = "client-mists", feature = "client-wowforever"))]

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn documented_profile_query_returns_one_false_boolean() {
    let env = WowLuaEnv::new().unwrap();
    env.state().borrow_mut().voice_chat.is_speak_for_me_active = false;
    env.exec(
        r#"
        local value = C_VoiceChat.IsSpeakForMeActive()
        assert(select('#', C_VoiceChat.IsSpeakForMeActive()) == 1, 'activity arity')
        assert(type(value) == 'boolean', 'activity must be a boolean, got ' .. type(value))
        assert(value == false, 'supplied false activity')
        "#,
    )
    .unwrap();
}

#[test]
fn documented_profile_retained_query_reads_live_isolated_activity() {
    let first = WowLuaEnv::new().unwrap();
    let second = WowLuaEnv::new().unwrap();
    first
        .exec("retained_activity_query = C_VoiceChat.IsSpeakForMeActive")
        .unwrap();
    first.state().borrow_mut().voice_chat.is_speak_for_me_active = true;
    first
        .exec(
            r#"
            local value = retained_activity_query()
            assert(select('#', retained_activity_query()) == 1, 'retained activity arity')
            assert(type(value) == 'boolean', 'retained activity must be a boolean, got ' .. type(value))
            assert(value == true, 'supplied true activity')
            "#,
        )
        .unwrap();
    second
        .exec("assert(C_VoiceChat.IsSpeakForMeActive() == false, 'isolated second environment')")
        .unwrap();
    first.state().borrow_mut().voice_chat.is_speak_for_me_active = false;
    first
        .exec("assert(retained_activity_query() == false, 'live false restoration')")
        .unwrap();
}
```

### runtime/argv.json

```text
[
  "systemd-run",
  "--user",
  "--wait",
  "--unit=wow-speak-mists-red-runtime-20261013t160238403005z",
  "--slice=agents.slice",
  "-p",
  "MemoryMax=4G",
  "-p",
  "MemoryHigh=3G",
  "-p",
  "CPUQuota=200%",
  "-p",
  "KillMode=control-group",
  "-p",
  "StandardOutput=file:<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/speak-mists-red-20261013T155414920393Z/runtime/stdout",
  "-p",
  "StandardError=file:<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/speak-mists-red-20261013T155414920393Z/runtime/stderr",
  "--working-directory=<REPO>",
  "/usr/bin/timeout",
  "--signal=TERM",
  "--kill-after=2",
  "86",
  "/usr/bin/bwrap",
  "--die-with-parent",
  "--unshare-net",
  "--ro-bind",
  "/",
  "/",
  "--ro-bind",
  "<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/speak-mists-red-20261013T155414920393Z/source",
  "<REPO>",
  "--ro-bind",
  "<REPO>/target",
  "<REPO>/target",
  "--tmpfs",
  "/tmp",
  "--dev",
  "/dev",
  "--proc",
  "/proc",
  "--chdir",
  "<REPO>",
  "--setenv",
  "HOME",
  "<HOME>",
  "--setenv",
  "WOW_SIM_CASC",
  "0",
  "--setenv",
  "WOW_SIM_NO_SOUND",
  "1",
  "<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/speak-mists-red-20261013T155414920393Z/artifacts/deps/integration-d6edc2b011e58495",
  "voice_speak_for_me_profile_input::",
  "--nocapture",
  "--test-threads=1"
]
```

### runtime/launcher.txt

```text
Running as unit: wow-speak-mists-red-runtime-20261013t160238403005z.service
          Finished with result: exit-code
Main processes terminated with: code=exited, status=1/FAILURE
               Service runtime: 24ms
             CPU time consumed: 6ms
                   Memory peak: 2.2M (swap: 0B)
```

### runtime/result.json

```text
{
  "source": "abb979c4465efcf75c6731dd08077e90abaf5509",
  "expected_cases": 2,
  "launcher_exit": 1,
  "summaries": [],
  "stdout_bytes": 0,
  "stderr_bytes": 90,
  "scope": "Mists supplied result-boundary only; laterisolation/restoration unreached iftrueassertfails; no native/default/eventproducer/fullstartupclaim"
}
```

### runtime/stdout

```text

```

### runtime/stderr

```text
bwrap: Can't create file <REPO>/target: Read-only file system
```

### runtime-retry1/argv.json

```text
[
  "systemd-run",
  "--user",
  "--wait",
  "--unit=wow-speak-mists-red-runtime-20261013t160323879126z",
  "--slice=agents.slice",
  "-p",
  "MemoryMax=4G",
  "-p",
  "MemoryHigh=3G",
  "-p",
  "CPUQuota=200%",
  "-p",
  "KillMode=control-group",
  "-p",
  "StandardOutput=file:<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/speak-mists-red-20261013T155414920393Z/runtime-retry1/stdout",
  "-p",
  "StandardError=file:<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/speak-mists-red-20261013T155414920393Z/runtime-retry1/stderr",
  "--working-directory=<REPO>",
  "/usr/bin/timeout",
  "--signal=TERM",
  "--kill-after=2",
  "86",
  "/usr/bin/bwrap",
  "--die-with-parent",
  "--unshare-net",
  "--ro-bind",
  "/",
  "/",
  "--ro-bind",
  "<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/speak-mists-red-20261013T155414920393Z/source",
  "<REPO>",
  "--ro-bind",
  "<REPO>/target",
  "<REPO>/target",
  "--tmpfs",
  "/tmp",
  "--dev",
  "/dev",
  "--proc",
  "/proc",
  "--chdir",
  "<REPO>",
  "--setenv",
  "HOME",
  "<HOME>",
  "--setenv",
  "WOW_SIM_CASC",
  "0",
  "--setenv",
  "WOW_SIM_NO_SOUND",
  "1",
  "<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/speak-mists-red-20261013T155414920393Z/artifacts/deps/integration-d6edc2b011e58495",
  "voice_speak_for_me_profile_input::",
  "--nocapture",
  "--test-threads=1"
]
```

### runtime-retry1/launcher.txt

```text
Running as unit: wow-speak-mists-red-runtime-20261013t160323879126z.service
          Finished with result: exit-code
Main processes terminated with: code=exited, status=101/n/a
               Service runtime: 360ms
             CPU time consumed: 348ms
                   Memory peak: 84.7M (swap: 0B)
```

### runtime-retry1/result.json

```text
{
  "source": "abb979c4465efcf75c6731dd08077e90abaf5509",
  "expected_cases": 2,
  "launcher_exit": 101,
  "summaries": [
    [
      "FAILED",
      "0",
      "2",
      "0",
      "0",
      "7859"
    ]
  ],
  "stdout_bytes": 546,
  "stderr_bytes": 2716,
  "scope": "Mists supplied result-boundary only; initialbwrapsetupFAILzeroexecutionsretained; no native/default/eventproducer/fullstartupclaim"
}
```

### runtime-retry1/stdout

```text

running 2 tests
test voice_speak_for_me_profile_input::documented_profile_query_returns_one_false_boolean ... FAILED
test voice_speak_for_me_profile_input::documented_profile_retained_query_reads_live_isolated_activity ... FAILED

failures:

failures:
    voice_speak_for_me_profile_input::documented_profile_query_returns_one_false_boolean
    voice_speak_for_me_profile_input::documented_profile_retained_query_reads_live_isolated_activity

test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 7859 filtered out; finished in 0.33s

```

### runtime-retry1/stderr

```text
[  0.000s] [Startup] WowLuaEnv::new begin
[  0.002s] [Startup] SimState::default complete in 2.16ms
[  0.002s] [Startup] rilua VM created in 115.96µs
[  0.002s] [Startup] builtin frames initialized in 77.18µs
[  0.002s] [Startup] template registry cleared in 8.61µs
[  0.004s] [Startup] intrinsic templates registered in 1.17ms
[  0.004s] [Startup] initial app_data lua handle installed in 621.00ns
[  0.143s] [Startup] init_lua_state complete in 139.32ms
[  0.143s] [Startup] final app_data lua handle installed in 721.00ns
[  0.143s] [Startup] initial screen globals installed in 89.96µs
[  0.143s] [Startup] WowLuaEnv::new complete

thread 'voice_speak_for_me_profile_input::documented_profile_query_returns_one_false_boolean' (1114121) panicked at <REPO>/tests/voice_speak_for_me_profile_input.rs:19:6:
called `Result::unwrap()` on an `Err` value: Lua(Runtime(RuntimeError { message: "activity must be a boolean, got nil", level: 0, traceback: [] }))
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[  0.150s] [Startup] WowLuaEnv::new begin
[  0.152s] [Startup] SimState::default complete in 1.64ms
[  0.152s] [Startup] rilua VM created in 81.46µs
[  0.152s] [Startup] builtin frames initialized in 30.31µs
[  0.152s] [Startup] template registry cleared in 3.96µs
[  0.152s] [Startup] intrinsic templates registered in 156.42µs
[  0.152s] [Startup] initial app_data lua handle installed in 311.00ns
[  0.232s] [Startup] init_lua_state complete in 80.46ms
[  0.232s] [Startup] final app_data lua handle installed in 211.00ns
[  0.232s] [Startup] initial screen globals installed in 73.53µs
[  0.232s] [Startup] WowLuaEnv::new complete
[  0.232s] [Startup] WowLuaEnv::new begin
[  0.234s] [Startup] SimState::default complete in 1.45ms
[  0.234s] [Startup] rilua VM created in 78.15µs
[  0.234s] [Startup] builtin frames initialized in 31.29µs
[  0.234s] [Startup] template registry cleared in 11.36µs
[  0.234s] [Startup] intrinsic templates registered in 124.91µs
[  0.234s] [Startup] initial app_data lua handle installed in 281.00ns
[  0.315s] [Startup] init_lua_state complete in 80.44ms
[  0.315s] [Startup] final app_data lua handle installed in 220.00ns
[  0.315s] [Startup] initial screen globals installed in 67.44µs
[  0.315s] [Startup] WowLuaEnv::new complete

thread 'voice_speak_for_me_profile_input::documented_profile_retained_query_reads_live_isolated_activity' (1114122) panicked at <REPO>/tests/voice_speak_for_me_profile_input.rs:39:10:
called `Result::unwrap()` on an `Err` value: Lua(Runtime(RuntimeError { message: "retained activity must be a boolean, got nil", level: 0, traceback: [] }))
```

### receipt-integration.json

```text
{
  "authenticated_at": "2026-10-13T16:00:40.592038+00:00",
  "sealed_path": "<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/speak-mists-red-20261013T155414920393Z/artifacts/deps/integration-d6edc2b011e58495",
  "source": "abb979c4465efcf75c6731dd08077e90abaf5509",
  "features": [
    "client-mists",
    "prefork-full-ui"
  ],
  "sha256": "e9d1ce3b45c962c88be8e2734c4cb91fd7b2e1a2635e89f728fbf11d79ff9342",
  "expected_sha256": "e9d1ce3b45c962c88be8e2734c4cb91fd7b2e1a2635e89f728fbf11d79ff9342",
  "sha256_matches": true,
  "stat_before": {
    "st_dev": 66306,
    "st_ino": 141989074,
    "st_mode": 33261,
    "st_size": 279023496,
    "st_mtime_ns": 1791907090334856790,
    "st_ctime_ns": 1791907185155346675
  },
  "stat_after": {
    "st_dev": 66306,
    "st_ino": 141989074,
    "st_mode": 33261,
    "st_size": 279023496,
    "st_mtime_ns": 1791907090334856790,
    "st_ctime_ns": 1791907185155346675
  },
  "stat_stable": true,
  "seal_stat_matches_before": true,
  "seal_stat_matches_after": true,
  "authentication_count": 1,
  "scope": "Independent authentication of new copied integration executable only; no other binary/library/archive/source-tree hashed; no runtime or tests",
  "pass": true
}
```

### auth-proof.json

```text
{
  "authentication": "PASS",
  "private_epoch": "<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/speak-mists-red-20261013T155414920393Z",
  "receipt": "receipt-integration.json",
  "receipt_persisted_immediately": true,
  "receipt_readback_confirmed": true,
  "source_revision": "abb979c4465efcf75c6731dd08077e90abaf5509",
  "features": ["client-mists", "prefork-full-ui"],
  "binary": "artifacts/deps/integration-d6edc2b011e58495",
  "sha256": "e9d1ce3b45c962c88be8e2734c4cb91fd7b2e1a2635e89f728fbf11d79ff9342",
  "authentication_count": 1,
  "seal_sha256_match": true,
  "stat_before_after_stable": true,
  "seal_stat_match_before_and_after": true,
  "compile_json_records_recorded": 385,
  "build_finished_success_recorded": true,
  "full_compile_stderr_read": true,
  "compile_terminal_observed": {
    "command_exit": 0,
    "ActiveState": "active",
    "SubState": "exited",
    "Result": "success",
    "ExecMainCode": 1,
    "ExecMainStatus": 0,
    "ExecMainPID": 1101109,
    "ExecMainStartTimestamp": "Tue 2026-10-13 10:54:14 CDT",
    "ExecMainExitTimestamp": "Tue 2026-10-13 10:58:10 CDT"
  },
  "actual_compile_wait_used": false,
  "actual_compile_wait_evidence": "Caller states main did not use compile wait; verifier performed read-only systemctl show, no wait",
  "manifest_deprecations": 6,
  "source_warnings": 8,
  "warning_free": false,
  "historical_E0603": "Caller-reported separate historical be2 failed compile; no RED credit; absent from current stderr",
  "source_archive_stat_only": {
    "st_dev": 66306,
    "st_ino": 141989078,
    "st_mode": 33188,
    "st_size": 1176012800,
    "st_mtime_ns": 1791907188607437323,
    "st_ctime_ns": 1791907188607437323
  },
  "source_mount": "<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/speak-mists-red-20261013T155414920393Z/source",
  "source_mount_authenticated": false,
  "other_binary_library_archive_source_tree_hashes": 0,
  "runtime_or_tests_executed": false,
  "red_credit": false,
  "native_parity_claim": false,
  "report": "auth-report.md"
}
```

### auth-report.md

```text
# Independent new-copy authentication: PASS

Private epoch: `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/speak-mists-red-20261013T155414920393Z`.

## Authentication proof

Authenticated exactly once, only `artifacts/deps/integration-d6edc2b011e58495`. SHA256 `e9d1ce3b45c962c88be8e2734c4cb91fd7b2e1a2635e89f728fbf11d79ff9342` matches `seal-integration.json`. Before/after stat identical: device 66306, inode 141989074, mode 33261, size 279023496, mtime_ns 1791907090334856790, ctime_ns 1791907185155346675. Both match every recorded seal stat field. `receipt-integration.json` persisted immediately after authentication and read back with all checks true. No second authentication performed.

## Compile evidence

Read complete `compile.stderr` (untruncated), `compile-result.json`, `revision.json`, `submission.json`, `toolchain.json`, `ldd.txt`, and `source-scope.json`. Source revision: `abb979c4465efcf75c6731dd08077e90abaf5509`. Result records 385 JSON records, one build-finished success=true, non-fresh integration compiler artifact, features `client-mists,prefork-full-ui`, runtime_launched=false. Submission requested `cargo test --test integration --no-run --no-default-features --features client-mists --offline --locked -j4 --message-format=json-render-diagnostics`; resultant artifact additionally records prefork-full-ui. Headless scope; no GUI/CASC/sound feature authenticated. Cargo/rustc 1.99.0; rustc host x86_64-unknown-linux-gnu, LLVM 23.1.1, commit b940084d7eb6a299eb4bfeb8e34901bc051e7ac4.

Read-only command, explicit cwd equal to private epoch:
`systemctl --user show wow-speak-mists-red-20261013t155414920393z.service --property=ActiveState,SubState,Result,ExecMainCode,ExecMainStatus,ExecMainPID,ExecMainStartTimestamp,ExecMainExitTimestamp`

Command exit 0; output:
```text
ActiveState=active
SubState=exited
Result=success
ExecMainStartTimestamp=Tue 2026-10-13 10:54:14 CDT
ExecMainExitTimestamp=Tue 2026-10-13 10:58:10 CDT
ExecMainPID=1101109
ExecMainCode=1
ExecMainStatus=0
```

This independently confirms terminal compiler success, not merely launcher_exit=0. Main reports actual compile wait was not used; no wait performed here. Complete stderr ends `Finished test profile [optimized + debuginfo] target(s) in 3m 55s`. No E0603 in this epoch's compile stderr; initial failed compile at historical be2 is separate, per caller, and receives no RED credit.

## Warnings: not warning-free

Six manifest deprecations in iced-wgpu-patched/Cargo.toml: large-enum-variant, map-entry, match-wildcard-for-single-variants, redundant-closure-for-method-calls, trivially-copy-pass-by-ref, type-complexity.

Eight source warnings: src/blizzard_ui_sync.rs:28 (PROVENANCE_SCHEMA), :180 (CacheProvenance::new), :663 (remove_missing_marker); src/casc_asset_fallback.rs:93 (ensure_known_asset_cached); src/render/font.rs:44 (encoding_key_hex); src/lua_api/globals/real/unit_roles.rs:17 (Tank/Healer/Damage); src/widget/frame.rs:601/610 (cooldown methods, one diagnostic); src/bin/wow_sim/main.rs:20 (unused SavedVariablesManager import). Seven library warnings plus one binary warning. Empty compiler_messages in compile-result.json does not erase stderr warnings.

## Source/runtime boundaries

Source scope text records selected tracked inputs Cargo.toml, Cargo.lock, build.rs, build, src, tests, data, Interface, .cargo; runtime mount is private epoch/source. Archive exists, stat only: device 66306, inode 141989078, mode 33188, size 1176012800, mtime_ns/ctime_ns 1791907188607437323. Archive neither opened nor hashed; mount content not authenticated. ldd text lists linux-vdso.so.1, libgcc_s.so.1, libm.so.6, libc.so.6 and ld-linux-x86-64.so.2. Libraries remain unsealed. Vendor caches/WTF/settings remain external; no native parity claim.

No tests, runtime, other artifact authentication, restarts, builds, operational mutations, delegation, commits, or tracked edits. Only private receipt/report/proof written. One initial metadata command attempt stopped at NameError before any stat or command; corrected metadata-only call succeeded, without rehashing. Authentication permits main's next genuine two-case RED execution; no RED proof claimed here.
```

### compile-result.json

```text
{
  "json_records": 385,
  "build_finished": [
    {
      "reason": "build-finished",
      "success": true
    }
  ],
  "compiler_messages": [],
  "integration_artifact": [
    {
      "reason": "compiler-artifact",
      "package_id": "path+file://<REPO>#0.1.0",
      "manifest_path": "<REPO>/Cargo.toml",
      "target": {
        "kind": [
          "test"
        ],
        "crate_types": [
          "bin"
        ],
        "name": "integration",
        "src_path": "<REPO>/tests/integration.rs",
        "edition": "2024",
        "doc": false,
        "doctest": false,
        "test": true
      },
      "profile": {
        "opt_level": "1",
        "debuginfo": "line-tables-only",
        "debug_assertions": true,
        "overflow_checks": true,
        "test": true
      },
      "features": [
        "client-mists",
        "prefork-full-ui"
      ],
      "filenames": [
        "<REPO>/target/debug/deps/integration-d6edc2b011e58495"
      ],
      "executable": "<REPO>/target/debug/deps/integration-d6edc2b011e58495",
      "fresh": false
    }
  ],
  "runtime_launched": false
}
```

### revision.json

```text
{
  "source_revision": "abb979c4465efcf75c6731dd08077e90abaf5509",
  "scope": "client-mists headless integration compilation only; runtime unrun",
  "git_status": "?? .code-index.db\n?? verification/\n"
}
```
