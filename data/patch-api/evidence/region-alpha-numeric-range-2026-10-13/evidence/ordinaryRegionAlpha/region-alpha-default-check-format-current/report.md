# Existing region-alpha format / default-check verification

PASS for retained format and default Retail type-check evidence only; not warning-free.

## Evidence and provenance

Base: `<PRIVATE_V>/widget-discrete-progress-current`

Recorded source: `51508fde9aa3657b09ea806d5c0411a28dc0e07b`; production delta: `71127a8e1`. These are unauthenticated recorded labels, not verified source identity. Both revision records list only `?? .code-index.db` and `?? verification/` in git_status.

Read all admission/submission/revision/toolchain/stdout/stderr bytes for both named runs. Full originals and all parsed check records are preserved in proof.json. Both admissions say allowed=true. Format projected 21505134488 bytes / host available 38277562368; check projected 30080775900 / host available 38311886848. Admission is launch metadata, not execution success.

Recorded toolchain: Cargo 1.99.0 (5f94df478 2026-08-27), rustc 1.99.0 (b940084d7 2026-09-28), Arch 1:1.99.0-1, x86_64-unknown-linux-gnu, LLVM 23.1.1. Full rustc details preserved in proof.json.

## Retained execution evidence

### region-alpha-full-format-20261014T033834425005Z

```text
Id=<UNIT_19>.service
LoadState=loaded
ActiveState=active
SubState=exited
Result=success
ExecMainStartTimestamp=Tue 2026-10-13 22:38:34 CDT
ExecMainExitTimestamp=Tue 2026-10-13 22:39:02 CDT
ExecMainCode=1
ExecMainStatus=0
ExecStart={ path=/usr/bin/env ; argv[]=/usr/bin/env RUSTC=/usr/bin/rustc /usr/bin/cargo fmt --all -- --check ; ignore_errors=no ; start_time=[Tue 2026-10-13 22:38:34 CDT] ; stop_time=[Tue 2026-10-13 22:39:02 CDT] ; pid=<PID_05> ; code=exited ; status=0 }
MemoryPeak=566022144
MemoryHigh=536870912
MemoryMax=1073741824
```

### region-alpha-default-check-20261014T033834533110Z

```text
Id=<UNIT_18>.service
LoadState=loaded
ActiveState=active
SubState=exited
Result=success
ExecMainStartTimestamp=Tue 2026-10-13 22:38:34 CDT
ExecMainExitTimestamp=Tue 2026-10-13 22:38:52 CDT
ExecMainCode=1
ExecMainStatus=0
ExecStart={ path=/usr/bin/env ; argv[]=/usr/bin/env RUSTC=/usr/bin/rustc /usr/bin/cargo check --offline --locked --lib --bins -j4 --message-format=json-render-diagnostics ; ignore_errors=no ; start_time=[Tue 2026-10-13 22:38:34 CDT] ; stop_time=[Tue 2026-10-13 22:38:52 CDT] ; pid=<PID_06> ; code=exited ; status=0 }
MemoryPeak=5461319680
MemoryHigh=8589934592
MemoryMax=9663676416
```

Both retained user units are loaded, active/exited, Result=success, ExecMainCode=1 (CLD_EXITED), ExecMainStatus=0, with ExecStart code=exited/status=0. Launcher exit 0 alone was not used. Initial system-manager queries found no such units and were discarded as execution evidence. User-unit timestamps are October 13 CDT, corresponding to October 14 UTC in the evidence directory names.

## Format result

Recorded command: `/usr/bin/env RUSTC=/usr/bin/rustc /usr/bin/cargo fmt --all -- --check`. Full stdout and stderr are each empty. Retained process exited 0.

**Soft-limit crossing retained:** MemoryPeak 566022144 bytes exceeds MemoryHigh 536870912 by **29151232 bytes**; MemoryMax 1073741824 (1 GiB) was not crossed. This is not a clean below-soft-limit run.

## Default check result

Recorded command: `/usr/bin/env RUSTC=/usr/bin/rustc /usr/bin/cargo check --offline --locked --lib --bins -j4 --message-format=json-render-diagnostics`.

Parsed every stdout line as JSON: **733 records**, comprising **658 compiler-artifact**, **74 build-script-executed**, **1 build-finished (success=true)**. **0 compiler-message records**: no emitted compiler warning/error diagnostics in this JSON stream. Manifest warnings below are separate stderr diagnostics and are not zeroed by that count.

Actual root library and bins (all fresh=false, profile.test=false, metadata/check artifacts, executable=null):

| Kind | Target | Recorded source path |
|---|---|---|
| lib | wow_ui_sim | <REPO>/src/lib.rs |
| bin | panel-visual-metrics | <REPO>/src/bin/panel-visual-metrics.rs |
| bin | bench_spellbook | <REPO>/src/bin/bench_spellbook.rs |
| bin | bench_steady_state | <REPO>/src/bin/bench_steady_state.rs |
| bin | bench_talents | <REPO>/src/bin/bench_talents.rs |
| bin | wow-sim | <REPO>/src/bin/wow_sim/main.rs |
| bin | wow-cli | <REPO>/src/bin/wow_cli/main.rs |

Additional root record: custom-build `build-script-build`, fresh=true; excluded from the six-bin count. All actual root library/bin records include client-retail/default/profile-retail and the same features (full lists in proof.json): aura-containers, aura-instance-enumeration, aura-xml-widgets, base-spell-relationships, casc, client-retail, default, forbidden-aspects, gui, native-duration-formatting, numeric-rule-formatters, on-update-modes, player-cast-durations, prefork-full-ui, profile-retail, retail-12-0-0, retail-12-0-5, retail-12-0-7, retail-12-1-0, rodio, sound.

Default MemoryPeak 5461319680 bytes is below MemoryHigh 8589934592 (8 GiB) and MemoryMax 9663676416 (9 GiB).

### Full stderr: six manifest deprecations preserved

```text
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.large-enum-variant` is deprecated in favor of `lints.clippy.large_enum_variant` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.map-entry` is deprecated in favor of `lints.clippy.map_entry` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.match-wildcard-for-single-variants` is deprecated in favor of `lints.clippy.match_wildcard_for_single_variants` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.redundant-closure-for-method-calls` is deprecated in favor of `lints.clippy.redundant_closure_for_method_calls` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.trivially-copy-pass-by-ref` is deprecated in favor of `lints.clippy.trivially_copy_pass_by_ref` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.type-complexity` is deprecated in favor of `lints.clippy.type_complexity` and will not work in a future edition
warning: `iced_wgpu` (manifest) generated 6 warnings
    Checking wow-ui-sim v0.1.0 (<REPO>)
    Finished `dev` profile [optimized + debuginfo] target(s) in 18.16s
```

Six distinct iced-wgpu-patched/Cargo.toml lint-key deprecations; one aggregate “generated 6 warnings” line, not a seventh distinct warning. No manifest source was opened.

## Claim boundary

Existing format and default Retail library+six-bin check succeeded with the warnings and resource crossing above. Compilation/check/format is NOT runtime, native-client, behavioral, or all-profile proof. No source/library/archive/binary/public-payload authentication, read or hashing; no reruns/builds/tests/Bash/delegation/canonical edits. Only named retained evidence, the saved observation log, and readonly systemctl observations were used.

## Persistence

report.md and proof.json: mode 0600; fsync file contents and output directory; full byte readback compared without hashes, entire proof JSON parsed and compared.
