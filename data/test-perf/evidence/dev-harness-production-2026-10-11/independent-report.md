# Independent saved-artifact audit

**PASS — ordinary default root production selection only.**

Artifact: `/home/osso/.local/state/wow-ui-sim/verification/dev-harness-production-build-current/20261011T063917Z`
Recorded revision: `e27aa007d9aea470b451b344dc2d607da1198cb8`

## Exact facts

- `/usr/bin/cargo build --offline --locked --bin wow-sim --message-format=json`; CARGO_BUILD_JOBS=12 (overrides checked-in jobs=4), CARGO_NET_OFFLINE=true. Queue CPUQuota=1200% (12 CPUs), not live enforcement proof.
- Queue exit 0; worker/request/controller executed code exactly match; controller completed with empty stderr. Saved build exit 0; build-finished success=true.
- Full stdout: 563009 bytes; 728 valid JSON lines; 0 parse errors. 653 compiler-artifact, 74 build-script-executed, 1 build-finished, 0 compiler-message records. 653 fresh=true, 0 nonfresh; matches saved outcome.
- Root: build-script-build/custom-build, wow_ui_sim/lib from src/lib.rs, wow-sim/bin from src/bin/wow_sim/main.rs. All fresh=true; lib/bin profile.test=false, opt-level=1, line-tables-only. Executable recorded at target/debug/wow-sim, not executed or hashed.
- 0 helper records across all JSON fields; 0 test/bench target or profile.test=true records. target.test=true on lib/bin describes test capability, not test selection.

74 cached build-script-executed output records (including 1 root record) **do not establish script execution during this invocation**.

## Seven seals: before = after = current

- `Cargo.toml`: `91c80dd9d2ff4d59ddce10af38a1dc09bee5ef0c5c166ebc19ec6ad485dac1e9`
- `Cargo.lock`: `5577f98dd1c7b7124a51dae1ab30429366b58b2b065b54ac56dc1d0d69bc26f1`
- `build.rs`: `cdebff6b6dec0e19e1124e734b2603432e2c47a7b956f9a4568ae412de182741`
- `crates/test-harness/Cargo.toml`: `c258faffce700aeee379a2bc07feb3a4511011da44c76f2758122437864d09a0`
- `crates/test-harness/build.rs`: `783339eafec5b3f7304df21b10a16f73834b74ebb6968eb433bf587addbcac06`
- `crates/test-harness/src/lib.rs`: `794bd63ff8c47d44fb4f9e00c74cc57f3cb2a6d591ec1c1a83520666e5019c32`
- `.cargo/config.toml`: `77d5c6fce277bdfc1a97ed3425170f552e5db6143d64e45abf1f6b522b973ead`

## Full stderr

```text
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.large-enum-variant` is deprecated in favor of `lints.clippy.large_enum_variant` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.map-entry` is deprecated in favor of `lints.clippy.map_entry` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.match-wildcard-for-single-variants` is deprecated in favor of `lints.clippy.match_wildcard_for_single_variants` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.redundant-closure-for-method-calls` is deprecated in favor of `lints.clippy.redundant_closure_for_method_calls` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.trivially-copy-pass-by-ref` is deprecated in favor of `lints.clippy.trivially_copy_pass_by_ref` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.type-complexity` is deprecated in favor of `lints.clippy.type_complexity` and will not work in a future edition
warning: `iced_wgpu` (manifest) generated 6 warnings
    Finished `dev` profile [optimized + debuginfo] target(s) in 1.02s
```

Six manifest lint-name deprecations; one warning summary. Recorded 1.02s is not performance acceptance.

## Boundary

Workspace members: ., xtask, crates/test-harness. Root helper dependency is dev-only, not normal. Child manifest remains needed for Cargo workspace resolution; child source is not a normal production dependency. No manifest-removal inference.

Not workspace-wide/all-targets/other profiles/native-host/container/deployment/clean-build proof. Seven seals are not a whole-source seal; recorded revision not checked against current HEAD. No rerun/build/check/binary hash/ops/delegation/Bash/rg/fd.

**No full branch readiness. Garrison FAIL remains caller context, not retested here.**
