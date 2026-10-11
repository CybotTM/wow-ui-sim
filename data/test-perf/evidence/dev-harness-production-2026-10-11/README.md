# Ordinary production selection — bounded proof

At `e27aa007d`, ordinary default root `cargo build --offline --locked --bin wow-sim --message-format=json` exits 0 with jobs12. [Independent audit](independent-report.md) parses all 728 JSON records: 653 compiler artifacts, all fresh; 74 build-script output records; one successful completion. Root `wow_ui_sim` library and `wow-sim` binary are present; zero helper/test/bench records. Seven before/after/current input seals match. Six manifest deprecation warnings remain.

This establishes the captured ordinary production selection does not select the helper. Cargo still loads the helper manifest as a workspace/path dev-dependency input. Explicit workspace builds may select it. Cached build-script output is not proof of execution; no rebuild speedup, other-target/profile, Docker/native-host, runtime or full-branch readiness claim follows.

[Retention manifest](retention-manifest.json) records ten exact text copies, 587,161 bytes; binaries and private session/provider payloads excluded. No private original mutation. Copy preservation is direct caller evidence, not a separate independent retention gate.

Related: [core extraction](../dev-only-harness-2026-10-11/README.md), [staging input repair](../dev-harness-build-inputs-2026-10-11/README.md). Completed Rust suite remains FAIL on Garrison; native/full handoff remains OPEN.
