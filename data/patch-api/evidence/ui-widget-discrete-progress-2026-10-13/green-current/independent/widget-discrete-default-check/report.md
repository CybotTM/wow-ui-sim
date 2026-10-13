# Saved default-check verification — PASS

Repository: `<REPO>`
Current revision: `7fbcd5e475b3c16ae24a5a247ac3949bc4d48b27`.
Producer revision: `50f6f0ea894d0bbe2f281887ef36d25e0bbbf313`.

## Scope and evidence

- Full saved compiler stdout consumed: 522804 bytes, 733 JSON records (658 compiler-artifact, 74 build-script-executed, 1 build-finished). Final record: `{"reason":"build-finished","success":true}`. Zero compiler-message records, therefore zero source compiler warnings/errors in this stream.
- Full stderr consumed: six inherited `iced-wgpu-patched/Cargo.toml` lint-key deprecations, plus their summary; not six source compiler warnings. Finished dev profile [optimized + debuginfo] in 24.19s. Exact stderr retained in proof.json.
- Saved terminal: `Result=success`, `SubState=exited`, `ExecMainCode=1` (normal exit), `ExecMainStatus=0`; exit timestamp October 13, 2026, 07:28:15 CDT. `ActiveState=active` reflects saved RemainAfterExit configuration, not an ongoing compiler.
- Selected local executables: `/usr/bin/cargo`, `RUSTC=/usr/bin/rustc`; saved Cargo 1.99.0 (5f94df478), rustc 1.99.0 (b940084d7), x86_64-unknown-linux-gnu, LLVM 23.1.1. Saved command: `cargo check --offline --locked -j2 --message-format=json-render-diagnostics`; no feature override or no-default-features flag. JSON confirms default/client-retail/profile-retail features and non-test dev check targets. This is compilation checking, not execution or binary-link proof.
- Saved admission allowed=true; worker MemoryMax=8G, MemoryHigh=6G, CPUQuota=400%; projected=39740779112 bytes, host_available=25906421760 bytes. Launcher exit 0 is corroborated by terminal exit 0 and build-finished success, not treated alone as compilation success.

## Revision and formatting applicability

Complete `git diff --name-status 50f6f0ea8 7fbcd5e475b3c16ae24a5a247ac3949bc4d48b27` showed only docs and retained evidence records. Relevant `src`, `tests`, Cargo.toml, Cargo.lock, build.rs, .cargo, and crates comparison returned exit 0 with empty stdout/stderr. Current tracked tree had no changes. Thus the default check explicitly recorded at 7fb covers unchanged relevant source/Cargo from 50; docs/evidence-only changes do not invalidate producer formatting proof.

Saved `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/20261013T121200894915Z/cargo-fmt-check.json` records `cargo fmt --all -- --check`, exit 0, empty stdout/stderr, source 50 and current 7fb docs-only. Reused without rerunning. Direct formatter 3667 is reused as caller-supplied prior evidence only; not separately inspected or newly claimed here.

## Proof limits

PASS applies only to the saved current default check and applicable saved formatting gate. No new runtime/test, alternate-profile, deployment, linked-binary, full-suite, or feature-completeness claims. No repeated commands/tests/check/build/runtime, binary/library inspection, source-archive hashes, delegation, operations, or tracked edits. Repository inspection used only git status/revision/log/diffs; initial git inspection from Pyrun's detached job directory failed and supplied no proof, then explicit repository cwd was used.

Evidence: `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/default-check-20261013T122750974243Z`. Complete structured findings: `<REPO>/verification/widget-discrete-default-check/proof.json`.
