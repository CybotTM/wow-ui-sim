# Current preproducer GUID RED — independently observed

**RED confirmed; feature acceptance FAIL.** Revision `76f405e1d38668b3eed4eeab2c3957294d8cee3a`. Retail only. 0 passed, 9 failed, 0 ignored, 0 measured, 10796 filtered out; 3.14 seconds; runtime exit 101 (not timeout 124). Eight genuine missing raw API failures; one consumer exact-result mismatch. No fixture/load/input-type failure reported. This is not producer, native, security or consumer success.

## Owned evidence

Root: `/home/osso/.local/state/wow-ui-sim/verification/guid-is-player-current`. Epoch: `red-20261012T185227693536Z`. Unit: `wow-guid-player-red-20261012t185227528830z`. Submitted request/submission records preserved at root. Observation began 18:52:55 UTC and completed within 20 minutes; resource admission delayed runtime until 19:06:47 UTC. Final unit active/exited, controller exit 0: successful capture, **not passing tests**. No remaining unit MainPID/control group; no timeout or kill operation.

New epoch artifact authenticated exactly once at 2026-10-12T18:57:09.319684+00:00; 468339000 bytes; SHA-256 `77f3d02d33eb4219f17adcf4dbd41463fa82146c11b05f901a04be8c479f5e0c` matches artifact.json. Exclusive epoch `independent-auth-integration.json` saved immediately after single streaming read; original target executable and libraries never read/hashed independently.

## Commands/profile/source

Owned compile: `/usr/bin/cargo test --test integration --no-run --message-format=json --offline --locked -j 12`, exit 0. Parsed all 741 JSON records: 665 compiler-artifact (663 fresh, 2 rebuilt), 75 build-script-executed, 1 successful build-finished, 0 compiler-message. Integration record fresh=false. No compiler-error record.

Owned runtime: `/usr/bin/timeout 90 <epoch>/integration-sealed guid_is_player:: --nocapture --test-threads=1`, exit 101. Owned `/usr/bin/cargo fmt --check`, exit 0, empty stdout/stderr. Verifier did not rerun any command above.

Toolchain:
```
cargo 1.99.0 (5f94df478 2026-08-27) (Arch Linux rust 1:1.99.0-1)
rustc 1.99.0 (b940084d7 2026-09-28) (Arch Linux rust 1:1.99.0-1)
binary: rustc
commit-hash: b940084d7eb6a299eb4bfeb8e34901bc051e7ac4
commit-date: 2026-09-28
host: x86_64-unknown-linux-gnu
release: 1.99.0
LLVM version: 23.1.1
```

Profile: test=true, opt_level=1, line-tables-only debuginfo, debug assertions and overflow checks enabled. Exact features: `aura-containers`, `aura-instance-enumeration`, `aura-xml-widgets`, `base-spell-relationships`, `casc`, `client-retail`, `default`, `forbidden-aspects`, `gui`, `native-duration-formatting`, `numeric-rule-formatters`, `on-update-modes`, `player-cast-durations`, `prefork-full-ui`, `profile-retail`, `retail-12-0-0`, `retail-12-0-5`, `retail-12-0-7`, `retail-12-1-0`, `rodio`, `sound`. No PTR run.

Seven recorded source hashes match before compile, after compile, after runtime, and independent final read; final scoped git diff from revision empty, HEAD unchanged. See source-before.json/source-after-compile.json/source-after-runtime.json/independent-source-record.json. Scope includes tests/spec, existing identity/kind/player-info files, Cargo.toml and Cargo.lock; tracked src/tests/tools applicability was also checked. Tests wired by integration_tests macro's top-level discovery; runtime reports all nine cases. No source changes made.

## Actual assertion coverage (all source lines tests/guid_is_player.rs)

Shared guard at line 48: `missing raw C_PlayerInfo.GUIDIsPlayer`; Rust panic line 58. It establishes absent raw callable provider, not tested query semantics. Each fixture preceding its guard completed unless explicitly noted otherwise.

| Case | Failing assertion / Rust panic | Deepest reached | Unreached: no credit |
|---|---|---|---|
| `guid_is_player_exact_local_identity_and_closed_world_misses` | 48 / 58 | First local identity raw-provider guard; no query called | All local/miss values, arity and boolean results |
| `guid_is_player_target_and_focus_follow_explicit_kind_not_prefix` | 48 / 58 | Target/focus fixture construction completes; first target raw-provider guard | Focus query, replacements, kind/prefix semantics |
| `guid_is_player_same_env_kind_updates_and_unit_removal_are_live` | 48 / 58 | Target/focus construction completes; first target raw-provider guard | Live kind flips, unit removal and local query |
| `guid_is_player_active_populated_party_lifecycle` | 48 / 58 | Party construction and both String UnitGUID extractions complete; distinct GUID assertion passes; first party raw-provider guard | Classification, activation, shrink/clear/repopulate/remove lifecycle |
| `guid_is_player_does_not_leak_records_between_environments` | 48 / 58 | First environment target/focus/party setup and String UnitGUID extraction complete; first target raw-provider guard | Party classification, second environment creation and isolation mutations |
| `guid_is_player_requires_string_without_numeric_coercion` | 214 / 228 | Raw-provider guard before stringError calls | Missing/nil/nonstring validation, numeric coercion and error text |
| `guid_is_player_authenticates_real_secrets_before_type_and_lookup` | 240 / 290 | Fixtures complete; taint mode enabled; untainted-setup assertion passes; raw-provider guard fails | Secret wrapper construction/authenticity, query authorization, type ordering, tainted closure, wrapper retention/taint restoration |
| `guid_is_player_original_object_api_consumer_accepts_represented_player` | 364 / 367 | Whole ordinary closure readiness and provenance pass; constructor/dispatch/GUID assertions pass; pcall IsValid succeeds; exact result == true fails | Successful consumer acceptance; exact actual value/type and provider provenance not logged |
| `guid_is_player_original_consumer_preserves_actual_account_predicate_for_misses` | 48 / 58 (call 377) | Whole ordinary closure readiness and provenance pass; first NPC-focus raw-provider guard fails | Unknown direct query; account predicate provenance/return diagnostics; miss consumer calls and account equality |

Represented-player consumer's original whole-addon load and method-source checks completed, constructor produced original IsValid dispatch, GUID/IsGUID assertion passed, and pcall succeeded. Failure is `represented player must return true, not merely truthy` at assertion line 364/panic 367, not missing-provider guard or load failure. Exact actual return value/type was not printed: do not label it false, nil, boolean, account output or successful player classification. Cached original PlayerLocation.lua IsValid uses player query OR account predicate; provider attribution cannot be inferred from successful pcall. Miss-case account diagnostics never executed.

Security case passes only untainted setup before missing-provider guard; authentic wrappers have not yet been constructed. No secret authorization/order/retention credit. Strict-string case never calls its validation helper. Environment isolation never constructs second environment. Lifecycle mutations following first query never execute.

## Readability

All 408 lines manually reviewed, including embedded Lua. Nine tests below 200 body-line ceiling; Rust helper bodies below 30; Rust nesting shallow; no added warning suppressions, TODO/FIXME/HACK/XXX, unsafe or primitive parameter overload. rust-code-analysis-cli absent at documented location; no claimed measured complexity score.

[COMPLEXITY] tests/guid_is_player.rs:276 — guid_is_player_authenticates_real_secrets_before_type_and_lookup()
Problem: Embedded Lua loop and denial call compressed onto one line.
Fix: Split loop header, denied(value), and end onto separate lines. Minor readability note; not causal RED, no edits made.

## Preserved diagnostics

Full runtime.stdout/runtime.stderr inspected (1739/8771 characters); all nine panic records preserved. Full compile.stdout parsed and compiler stderr inspected. Full fmt/source/toolchain/controller records inspected. Six manifest deprecations retained verbatim:
```
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.large-enum-variant` is deprecated in favor of `lints.clippy.large_enum_variant` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.map-entry` is deprecated in favor of `lints.clippy.map_entry` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.match-wildcard-for-single-variants` is deprecated in favor of `lints.clippy.match_wildcard_for_single_variants` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.redundant-closure-for-method-calls` is deprecated in favor of `lints.clippy.redundant_closure_for_method_calls` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.trivially-copy-pass-by-ref` is deprecated in favor of `lints.clippy.trivially_copy_pass_by_ref` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.type-complexity` is deprecated in favor of `lints.clippy.type_complexity` and will not work in a future edition
warning: `iced_wgpu` (manifest) generated 6 warnings
   Compiling wow-ui-sim-test-harness v0.1.0 (/home/osso/Projects/wow/wow-ui-sim/crates/test-harness)
   Compiling wow-ui-sim v0.1.0 (/home/osso/Projects/wow/wow-ui-sim)
    Finished `test` profile [optimized + debuginfo] target(s) in 1m 04s
```

`proof.json` contains exact per-case boundaries, features, exits, warnings, authentication/source references and limitations. No rerun, source edits, service mutation, process kill or delegation. RED observation complete; producer and downstream acceptance remain unproven.
