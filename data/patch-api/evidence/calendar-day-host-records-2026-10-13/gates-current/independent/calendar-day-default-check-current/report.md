# Calendar day default lib + binaries check acceptance

**PASS — bounded existing check epoch only.** No whole-parent completion claim.

Epoch: `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/calendar-day-default-check-20261013T215022485897Z`.

## Evidence

- Full stdout parsed: **733 JSONL records**: 658 compiler-artifact, 74 build-script-executed, 1 build-finished with success=true. **0 compiler-message records** (no source compiler diagnostics). Full stderr read: **6 manifest deprecations** in iced-wgpu-patched/Cargo.toml, followed by aggregate six-warning notice; Finished dev profile in 26.35s. This is not a warning-free claim.
- Saved terminal status: ActiveState=active, **SubState=exited**, Result=success, **ExecMainStatus=0**. RemainAfterExit=yes explains active state; no live process status asserted.
- Submission launcher exit 0; exact cargo argv: `/usr/bin/cargo check --offline --locked --lib --bins -j4 --message-format=json-render-diagnostics`, RUSTC=/usr/bin/rustc. Default features retained, no alternate profile flags. Admission allowed=true; worker MemoryMax=12G, MemoryHigh=10G, CPUQuota=400%.
- Recorded toolchain: Arch cargo 1.99.0 (5f94df478 2026-08-27), rustc 1.99.0 (b940084d7 2026-09-28), x86_64-unknown-linux-gnu, LLVM 23.1.1. Recorded source **4def990f697a664449f4b398e91ecd8743196129**, over producer **396f0c9a6ba705f9fd610d0a2c198ef910f67c4f**. Current observed HEAD **22ad8c5e5c085d8387c06e13069027bc719058b0** advanced by four evidence/documentation files only. Calendar scoped Rust matches requested source and producer by Git metadata. No byte-level cryptographic provenance claimed.
- Reused `calendar-day-producer-source-current` cargo fmt --check **exit 0**, empty stdout/stderr. No tracked Rust delta since its recorded eb453a0 HEAD, no tracked working Rust changes. No fmt rerun.

## Coverage and limits

| Claim | Result | Proof |
| --- | --- | --- |
| Default library check | PASS | wow_ui_sim lib artifact, profile.test=false, fresh=false |
| Default binary check | PASS | panel-visual-metrics, bench_talents, bench_spellbook, bench_steady_state, wow-sim, wow-cli artifacts; profile.test=false, fresh=false |
| Source diagnostics | 0 | Full JSONL has no compiler-message |
| Manifest warnings | 6 | Full stderr, preserved in proof.json |
| Formatting | PASS reused | Existing exit-0 gate plus unchanged tracked Rust metadata |
| Tests/helper/bench execution, native/runtime acceptance | UNPROVED / excluded | cargo check artifacts do not prove execution; bench-named bin artifacts are binary checks, not benchmarks |

No check/build/tests/runtime/authentication/hashing/archive/delegation rerun, no tracked edits. Saved revision/submission/admission/toolchain records support recorded source identity, not authenticated byte provenance. Full parsed stdout, full stderr/status and metadata retained/referenced in proof.json. One summary evaluation failed on unavailable variable before running any command; artifact parsing then succeeded. This acceptance does not close Calendar runtime or broader parent requirements.
