# Existing default check and full formatting proof

**PASS — scoped command evidence, recorded revision `bde7ea277e86f9ad17f7135b3cd676f80e73057b`.**

No build/test/check/fmt reruns. Existing full outputs consumed; all 733 check stdout lines parsed as JSON. Read-only `systemctl --user show` independently confirms actual process exits, invocation IDs, argv and terminal state; launcher exits alone are not used as success evidence. Both retained units are `active/exited` because `RemainAfterExit=yes`, with `Result=success`, `ExecMainCode=1` (CLD_EXITED), `ExecMainStatus=0`, and ExecStart `code=exited; status=0`.

| Command | Actual exit | Full output | Timing | MemoryPeak bytes |
|---|---:|---|---|---:|
| `cargo fmt --all -- --check` | 0 | stdout 0 bytes; stderr 0 bytes | 41s terminal elapsed | 571645952 |
| `cargo check --offline --locked --lib --bins -j4 --message-format=json-render-diagnostics` | 0 | stdout 522804 bytes; stderr 1274 bytes; 733 JSON records; final build-finished success=true; zero compiler-message records | Cargo 34.58s; terminal timestamps span 35s | 5482766336 |

## Admission, submission, revision, toolchain

Both admission records allow launch. Projected usage arithmetic matches memory.current minus recorded half-reclaim plus worker_max. Format projected 30256922776 bytes; check projected 39882292352 bytes. Both below recorded parent memory.high 42949672960 and memory.max 45097156608. Worker limits match observed unit state: format MemoryMax 1073741824 / MemoryHigh 536870912 / CPU 200%; check MemoryMax 9663676416 / MemoryHigh 8589934592 / CPU 400%. Both peaks below worker MemoryMax. Format peak 571645952 exceeds MemoryHigh 536870912 by 34775040 bytes; MemoryHigh is not a hard cap, and the actual terminal result remains success. Check peak is below MemoryHigh. Parent limits are historical admission evidence, not independently authenticated current parent state or a parent-wide pass.

Submission argv, output paths, recorded working directory and service invocation IDs match observed units. Invocation IDs: format <INVOCATION_013>; check <INVOCATION_008>. Local terminal timestamps are October 13, 2026 CDT; epoch/admission timestamps are October 14 UTC (same runs).

Both submission.source and revision.source_revision equal `bde7ea277e86f9ad17f7135b3cd676f80e73057b`. Format production_delta_revision=bde7ea277; check production_delta_revision=6af710814 does not replace source_revision. Recorded git status contains untracked .code-index.db and verification/. No source-tree authentication or cleanliness claim.

Recorded toolchain: /usr/bin/cargo 1.99.0 (5f94df478, 2026-08-27); RUSTC=/usr/bin/rustc 1.99.0 (b940084d7, 2026-09-28), Arch package 1:1.99.0-1, x86_64-unknown-linux-gnu, LLVM 23.1.1. No toolchain binary authentication or new version command.

## Exact scope

Formatting: workspace `--all -- --check`, no writes. Cargo check: default features enabled, client-retail/profile-retail; dev profile, lib wow_ui_sim plus six bins: `panel-visual-metrics`, `bench_talents`, `bench_steady_state`, `bench_spellbook`, `wow-sim`, `wow-cli`. All seven lib/bin records fresh=false; custom build-script-build record fresh=true. JSON reasons: 658 compiler-artifact, 74 build-script-executed, 1 build-finished. Cargo JSON records are compiler/check evidence, not binary/library/archive authentication.

Observed features: `aura-containers`, `aura-instance-enumeration`, `aura-xml-widgets`, `base-spell-relationships`, `casc`, `client-retail`, `default`, `forbidden-aspects`, `gui`, `native-duration-formatting`, `numeric-rule-formatters`, `on-update-modes`, `player-cast-durations`, `prefork-full-ui`, `profile-retail`, `retail-12-0-0`, `retail-12-0-5`, `retail-12-0-7`, `retail-12-1-0`, `rodio`, `sound`.

No tests, runtime behavior, native-client parity, alternate profiles, full-suite/full-parent acceptance, deployment, linking or artifact authenticity promoted from these commands. Services untouched; no delegation, Bash, cwd change or repository edits.

## Retained warnings (not warning-free)

Six manifest deprecations plus one aggregate warning line; zero compiler diagnostics. Full stderr:

```text
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.large-enum-variant` is deprecated in favor of `lints.clippy.large_enum_variant` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.map-entry` is deprecated in favor of `lints.clippy.map_entry` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.match-wildcard-for-single-variants` is deprecated in favor of `lints.clippy.match_wildcard_for_single_variants` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.redundant-closure-for-method-calls` is deprecated in favor of `lints.clippy.redundant_closure_for_method_calls` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.trivially-copy-pass-by-ref` is deprecated in favor of `lints.clippy.trivially_copy_pass_by_ref` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.type-complexity` is deprecated in favor of `lints.clippy.type_complexity` and will not work in a future edition
warning: `iced_wgpu` (manifest) generated 6 warnings
    Checking wow-ui-sim v0.1.0 (<repo>)
    Finished `dev` profile [optimized + debuginfo] target(s) in 34.58s
```

## Evidence files

Input epoch directories:
- `<epochs>/bullet-dress-full-format-20261014T010250494375Z`
- `<epochs>/bullet-dress-default-check-20261014T010257730489Z`

Each contains actual filenames `admission.json`, `submission.json`, `revision.json`, `toolchain.json`, `compile-terminal.txt`, `compile.stdout`, `compile.stderr`. Complete texts, byte counts, SHA-256 evidence hashes and observed terminal fields retained in `proof.json`; hashes identify evidence records only.

New output files: `report.md`, `proof.json` in `<verification>/bullet-dress-default-check-format-current`. Created exclusively with mode 0600, file-fsynced, directory-fsynced and byte-for-byte read back. Existing output files are never overwritten.
