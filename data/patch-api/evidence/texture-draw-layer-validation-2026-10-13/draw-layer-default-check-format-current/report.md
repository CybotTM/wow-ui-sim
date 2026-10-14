# Draw-layer retained default check and format proof

**PASS with caveats — retained evidence only.** No commands rerun.

## Attribution and scope

Both captures record source `1b707b28b16eb291e36180b2a12ac2f0411264af` and producer delta `6e085fce9`. These are recorded identifiers, not freshly computed hashes or proof of live checkout equivalence. Recorded git status contains untracked `.code-index.db` and `verification/`.

Fully read admission, submission, revision, toolchain, compile.stdout, compile.stderr, and compile-terminal.txt from each named capture. Parsed every default-check stdout JSON line. No artifact or source files referenced by that JSON were opened.

## Coverage matrix

| Retained capture | Result | Exact proof | Limits |
| --- | --- | --- | --- |
| draw-layer-full-format-20261014T041951817431Z | PASS | `/usr/bin/cargo fmt --all -- --check`; empty stdout and stderr; launcher exit 0; terminal exited/success, ExecMainCode=1, ExecMainStatus=0 | Format/source proof only |
| draw-layer-default-check-20261014T041951980839Z | PASS with warnings | `/usr/bin/cargo check --offline --locked --lib --bins -j4 --message-format=json-render-diagnostics`; launcher exit 0; terminal exited/success, ExecMainCode=1, ExecMainStatus=0 | Default retail library and six bins only; no runtime or all-profile acceptance |

Default-check stdout: **733 JSON records**: 658 compiler-artifact, 74 build-script-executed, one successful build-finished, **zero compiler-message records**. Project library `wow_ui_sim` and six bins: `panel-visual-metrics`, `bench_spellbook`, `bench_steady_state`, `bench_talents`, `wow-sim`, `wow-cli`; all seven project check records have fresh=false. Artifact metadata is check evidence, not executable/runtime proof.

Stderr contains **six iced-wgpu-patched/Cargo.toml manifest deprecations**, plus `warning: `iced_wgpu` (manifest) generated 6 warnings`. That aggregate line repeats the same six warnings, not six more. Zero JSON compiler-message records does **not** establish warning-free output. Stderr also records package-cache lock waiting, Checking wow-ui-sim, and Finished dev profile in 29.75s.

## Memory and admission

Both retained admissions allowed submission. Full format: peak **567672832 bytes**, soft high **536870912**, hard max **1073741824**. Peak crossed soft high by **30801920 bytes**, remained below hard max; crossing is retained, not concealed by success. Default check: peak **5049040896 bytes**, below soft high **8589934592** (8 GiB) and hard max **9663676416** (9 GiB). No inference about throttling or OOM events beyond retained fields. Both terminal records show ActiveState=active with SubState=exited; RemainAfterExit=yes explains retained active state, not a running workload.

## Actual retained toolchain (both captures)

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

Both submissions specify RUSTC=/usr/bin/rustc. Capture timestamps are October 14, 2026 UTC as recorded; no new execution time is asserted.

## Exclusions and persistence

Not warning-free, runtime, test, or all-profile proof. No reruns, workloads, builds, tests, hashes, service operations, canonical edits, delegation, library/source-archive/binary/public-payload reads, or accepted alpha raw rereads. Only the two named captures supply evidence.

Saved report.md and proof.json as mode 0600, fsynced files and output directory, then fully read both back and compared exact contents. proof.json retains commands, actual toolchain, terminal/admission summaries, read coverage, JSON counts, and warning distinctions.
