# Artifact-only default Cargo check verification

**PASS — default-feature library/binary Cargo check only. Not full-goal completion.**

## Existing execution and terminal evidence

Read `submission.json`, the complete `check.stdout` JSON stream, complete `check.stderr`, `terminal-systemd.txt`, and `journal.txt`. No check/build/test/format/service command was launched or repeated; no delegation or source edits.

- Unit: `wow-pass2-five-check-20261013t032956876900z.service`.
- Submitted revision: `2691111d5ab85328ebcc5dae70eb38303b0ce2d3`.
- Submitted command: `/usr/bin/env RUSTC=/usr/bin/rustc /usr/bin/cargo check --offline --locked -j2 --message-format=json-render-diagnostics`, working directory `$REPO`.
- Saved terminal evidence: `ActiveState=inactive`, `SubState=dead`, `Result=success`, `ExecMainStatus=0`.
- Journal contains 2 entries: start at `2026-10-12T22:29:56-05:00`; terminal resource accounting at `2026-10-12T22:30:44-05:00`. Reports `34.930s` CPU, `47.670s` wall, `6G` memory peak. These are October 13 UTC, not an inconsistent local date.
- Neither saved terminal nor journal evidence is missing. This verifier read saved evidence only; did not query or mutate the service.

## Exact output counts

`check.stdout`: 522,804 bytes; 733 lines; 733 successfully parsed JSON records; 0 invalid JSON records.

| Record type | Count |
| --- | ---: |
| compiler-artifact | 658 |
| build-script-executed | 74 |
| compiler-message | 0 |
| build-finished | 1 |

The sole completion record, final in the stream: `{"reason":"build-finished","success":true}`. JSON compiler errors: **0**. JSON compiler warnings: **0**.

Complete stderr contains **6 distinct manifest deprecation warnings**, **1 warning summary** (not a seventh distinct warning), **0 error lines**, one `Checking` line and one `Finished` line. Completion text: `Finished \`dev\` profile [optimized + debuginfo] target(s) in 47.52s`.

All six warnings originate at `iced-wgpu-patched/Cargo.toml`; hyphenated Clippy lint keys are deprecated in favor of underscore keys and will not work in a future edition:

| Existing key | Replacement |
| --- | --- |
| `large-enum-variant` | `large_enum_variant` |
| `map-entry` | `map_entry` |
| `match-wildcard-for-single-variants` | `match_wildcard_for_single_variants` |
| `redundant-closure-for-method-calls` | `redundant_closure_for_method_calls` |
| `trivially-copy-pass-by-ref` | `trivially_copy_pass_by_ref` |
| `type-complexity` | `type_complexity` |

Summary: `warning: \`iced_wgpu\` (manifest) generated 6 warnings`. PASS is **not warning-free**. No warning fixes authorized or performed.

## Revision and working-tree binding

Observed HEAD: `2691111d5ab85328ebcc5dae70eb38303b0ce2d3`, exactly the submitted revision.

The caller's `c490` abbreviation is ambiguous with tree objects; resolved commit is `c490d77f15dce96ba757ff2e1f0a24faeaf2ba9e` (`Wire supplied item and ping queries into namespace setup`). `git diff --name-status c490d77f1 2691111d5` lists exactly 8 documentation/instruction paths, no Rust/Cargo changes: `AGENTS.md`, `docs/lua-api.md`, `docs/wiki/index.md`, `docs/wiki/investigations/integrated-source-and-factory-proof-2026-10-09.md`, `docs/wiki/investigations/method-dispatch-refactor.md`, `docs/wiki/log.md`, `docs/wiki/systems/lua-api.md`, `docs/wiki/systems/widget-system.md`.

`git diff 2691111d5 -- '*.rs' '**/Cargo.toml' '**/Cargo.lock' Cargo.toml Cargo.lock .cargo` is empty (includes tracked staged/unstaged differences against the submitted revision). Porcelain status shows 14 modified documentation paths plus untracked `.code-index.db` and `verification/`, not a globally clean working tree. An unrestricted untracked Rust/Cargo listing finds historical source/dependency copies under `verification/`; these are not changes to active source. A bounded untracked listing for `src`, `crates`, `tests`, `build.rs`, `Cargo.toml`, `Cargo.lock`, `.cargo`, and `iced-wgpu-patched` is empty.

Thus observed active tracked Rust/Cargo state is unchanged from the submitted revision, itself Rust/Cargo-equivalent to `c490d77f1`. Source freeze and absence of concurrent Rust writers throughout the existing check are caller-provided conditions, not independently established by a point-in-time Git inspection. No executable/dependency hashes or native/model/runtime evidence were collected.

## Proven scope and exclusions

Project JSON artifacts: **8** total — **1 fresh custom-build artifact**, **1 non-fresh library** (`wow_ui_sim`), **6 non-fresh binaries** (`bench_spellbook`, `bench_steady_state`, `panel-visual-metrics`, `bench_talents`, `wow-sim`, `wow-cli`). All have `profile.test=false`. Default features include `client-retail` and `profile-retail`; this is the default development-profile library/binary check, not every client profile or all targets.

Proof does not cover runtime behavior, executable linking or execution, native client capture, model API acceptance, tests, all profiles, deployment, or full-goal completion. Cargo fmt was not duplicated; the caller reports prior proof `3272` PASS, not independently reverified here. No toolchain-version invocation was performed; the artifact establishes configured executable paths, not a fresh version capture.
