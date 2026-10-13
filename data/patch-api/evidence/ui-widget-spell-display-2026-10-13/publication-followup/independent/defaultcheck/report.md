# Spell removal default check — PASS

## Scope and provenance
Epoch: `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/spell-removal-default-check-20261013T195013783049Z`.
Source: `844a63adb95c7111e1286fa668e632f7cd237806`; removal: `621649e56`; SpellDisplay: `38ec` (caller abbreviation). Documentation overlay source844 over removal621/Spell38ec per caller. Saved metadata binds source; no independent checkout identity verification.

## Terminal metadata
Metadata-only systemctl show exited 0, empty stderr:
```text
ActiveState=active
SubState=exited
Result=success
ExecMainCode=1
ExecMainStatus=0
```
Submission records RemainAfterExit=yes; active/exited is terminal, not a running compiler.

## Complete logs
Parsed all 733 JSONL records / 522804 bytes; zero parse errors. Counts: 658 compiler-artifact, 74 build-script-executed, 1 build-finished success=true. Compiler-message records: 0; source errors: 0; source warnings: 0.
Library wow_ui_sim and six binaries panel-visual-metrics, bench_steady_state, bench_talents, bench_spellbook, wow-sim, wow-cli recorded fresh=false. Root build script fresh=true. Default/client-retail features recorded.
Read full stderr (9 lines). Six unsuppressed manifest deprecations; summary is not a seventh deprecation:
```text
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.large-enum-variant` is deprecated in favor of `lints.clippy.large_enum_variant` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.map-entry` is deprecated in favor of `lints.clippy.map_entry` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.match-wildcard-for-single-variants` is deprecated in favor of `lints.clippy.match_wildcard_for_single_variants` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.redundant-closure-for-method-calls` is deprecated in favor of `lints.clippy.redundant_closure_for_method_calls` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.trivially-copy-pass-by-ref` is deprecated in favor of `lints.clippy.trivially_copy_pass_by_ref` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.type-complexity` is deprecated in favor of `lints.clippy.type_complexity` and will not work in a future edition
warning: `iced_wgpu` (manifest) generated 6 warnings
    Checking wow-ui-sim v0.1.0 (<REPO>)
    Finished `dev` profile [optimized + debuginfo] target(s) in 33.04s
```

## Actual saved toolchain
```text
cargo 1.99.0 (5f94df478 2026-08-27) (Arch Linux rust 1:1.99.0-1)
rustc 1.99.0 (b940084d7 2026-09-28) (Arch Linux rust 1:1.99.0-1)
binary: rustc
commit-hash: b940084d7eb6a299eb4bfeb8e34901bc051e7ac4
commit-date: 2026-09-28
host: x86_64-unknown-linux-gnu
release: 1.99.0
LLVM version: 23.1.1
```

## Formatting and limitations
Main reports cargo fmt --check once at source844, exit 0. Caller evidence, not independently artifact-confirmed; no rerun.
PASS covers saved default library/binaries cargo-check evidence only. No full-suite/native/runtime/other-profile/linked-build claims. No rerun/build/runtime/hash/delegation/tracked edits performed.
