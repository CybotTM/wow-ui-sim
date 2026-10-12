# Final check — FAIL

Scope: `d7a34be4a`, final-check epoch `final-check-20261012T192741757481Z` only. Observed assigned unit terminal within 20-minute bound: active/exited, systemd Result=success/ExecMainStatus=0. Controller emitted semantic error; transport success is not verification success.

## Coverage / proof

| Requirement | Result | Evidence |
|---|---|---|
| ONE default `cargo check --offline --locked -j 12` | PASS, exit 0 | Exact argv `/usr/bin/cargo check --offline --locked -j 12`; 18.83s; six vendor-manifest deprecation warnings retained |
| Nine Python validator regressions | FAIL, exit 1 | Nine ran, eight passed, one failed in 0.934s |
| Real current 8.1.0 `validate.py` | NOT RUN | Zero invocations: controller halted on Python regression failure; no submission/results/output artifacts |
| Actual NEW GUID map / old533c / frozen captures | Preserved | Before, after cargo, after Python and current scoped bytes equal; new 67854 bytes, old533c 67850 bytes, frozen 67826 bytes |
| Frozen history vs current closures | Source/artifact inspection only for standalone gate | Frozen 145/87OK/58 gaps; old533c 145/93OK/52; actual new map 145/94OK/51. Seven raw function closures, unchanged IDs/expectations; current fixture exactly 51 gaps. Two frozen removals remain separate |

Both commands ran once in `/home/osso/Projects/wow/wow-ui-sim`, recorded environment overlay `PYTHONDONTWRITEBYTECODE=1`. Full inherited environment was not captured. Invocation evidence: phase loop, command-results, submission/result artifacts and controller output. No reruns.

## Failure / isolation boundary

`tools/test_patch_8_1_0_validation.py:111` calls `check_current_producer_closures`; `validate.py:279` fails required proof existence: `guid-is-player-current-2026-10-12/README.md`.

Read fixture isolation before command inspection. `setUp` excludes BOTH flat new-GUID and old533c packet directories, correctly preventing synthetic writes through directory symlinks. `store` unlinks file symlinks before replacement. `publish_current` writes only a synthetic result JSON into scratch, not the new packet's README. Actual source packet README exists. Thus positive seven-closure fixture fails on missing scratch proof, not actual source capture corruption. No source edits or synthetic test-map exports performed.

The preservation regression copies both actual captures into a reference tree and asserts reference and source bytes unchanged after scratch publication; it is among eight passing tests (aggregate inference, not verbose per-case output).

GUID failed-closure control rejects at the current-gap-set assertion before proof checks. GUID wrong-symbol and fallback-only controls receive NO behavioral credit: generic `assertRaises(AssertionError)` also passes for the missing scratch GUID README before the intended expected-symbol/detail checks. Required-new-packet/old533c-only control passes with FileNotFoundError; source has no old-map substitution.

## Targeted contract inspection

`c_player_info_guid.rs` provides real modeled raw registration, authentic secret unwrap before strict string validation, exact represented identity/player-kind projection, active populated party projection, one boolean and closed-world misses. `val_to_string` accepts only Val::Str with valid UTF-8. Registration is Retail/PTR-scoped in `mod.rs` and `registration.rs`. `tests/guid_is_player.rs` covers live record changes, party lifecycle, environment isolation, strict inputs/secrets and ordinary whole ObjectAPI consumers without account mocks. Source-inspected compliance, not fresh runtime/native evidence.

Validator requires exactly seven reviewed closures/51 current gaps and new packet identity/expectation/raw-provider integrity while frozen accounting remains 87/58. Two original removal IDs remain `wt-cvars-gxMTOpaque-158` and `wt-global-api-C_ReportSystem.ReportPlayer-97`; frozen historical/supersession checks are exercised within the nine-test run, not a newly completed standalone validator.

Already GREEN Retail/PTR fmt exit 0, empty stdout/stderr reused. Normalized GREEN source-only digest snapshots before/after compile/runtime match current Rust files; no repeated fmt. No executable, authentication, seal, full-suite or native checks performed. No whole-handoff PASS inferred.

## Exact command output

### default-check stdout
(empty)

### default-check stderr
```
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.large-enum-variant` is deprecated in favor of `lints.clippy.large_enum_variant` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.map-entry` is deprecated in favor of `lints.clippy.map_entry` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.match-wildcard-for-single-variants` is deprecated in favor of `lints.clippy.match_wildcard_for_single_variants` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.redundant-closure-for-method-calls` is deprecated in favor of `lints.clippy.redundant_closure_for_method_calls` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.trivially-copy-pass-by-ref` is deprecated in favor of `lints.clippy.trivially_copy_pass_by_ref` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.type-complexity` is deprecated in favor of `lints.clippy.type_complexity` and will not work in a future edition
warning: `iced_wgpu` (manifest) generated 6 warnings
    Checking wow-ui-sim v0.1.0 (/home/osso/Projects/wow/wow-ui-sim)
    Finished `dev` profile [optimized + debuginfo] target(s) in 18.83s
```

### validator-tests stdout
(empty)

### validator-tests stderr
```
........F
======================================================================
FAIL: test_original_two_removals_are_separate_from_seven_producers (__main__.ValidationTests.test_original_two_removals_are_separate_from_seven_producers)
----------------------------------------------------------------------
Traceback (most recent call last):
  File "/home/osso/Projects/wow/wow-ui-sim/tools/test_patch_8_1_0_validation.py", line 111, in test_original_two_removals_are_separate_from_seven_producers
    self.assertEqual(self.validator.check_current_producer_closures()['current_producer_closures'], 7)
                     ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~^^
  File "/home/osso/Projects/wow/wow-ui-sim/data/patch-api/evidence/8.1.0-session-2026-10-08/validate.py", line 279, in check_current_producer_closures
    assert (ROOT / 'data/patch-api/evidence' / proof).is_file(), proof
           ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~^^
AssertionError: guid-is-player-current-2026-10-12/README.md

----------------------------------------------------------------------
Ran 9 tests in 0.934s

FAILED (failures=1)
```

All nine method names and aggregate-derived statuses, exact argv/cwd/env/exit/invocations and preservation evidence are recorded in `final-check-current-proof.json`. Raw artifacts remain untouched under the named epoch. Standalone validator remains blocked/unproven; this terminal failure is not pending execution.
