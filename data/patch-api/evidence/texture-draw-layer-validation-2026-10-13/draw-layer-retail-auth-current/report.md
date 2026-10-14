# NEW copy-integrity gate: PASS

Epoch: `<PRIVATE_V>/widget-discrete-progress-current/draw-layer-retail-green-20261014T040918198388Z`.

NEW copied executable copy integrity only; not Rust/runtime GREEN, native parity, library or source/archive authenticity. No workloads/builds/services/canonical edits/delegation; no old binary/library/archive/source-tree hashes.

741 compile JSON records; zero compiler-message records; six manifest deprecation warnings (not warning-free). Terminal: Result=success, ExecMainStatus=0, SubState=exited. Revision/normal association/fingerprint metadata inspected; source label not authenticated.

- integration: PASS, hash_reads=1, SHA256 matches seal=True, pre/post stat equality=True. Receipt: `<PRIVATE_V>/widget-discrete-progress-current/draw-layer-retail-green-20261014T040918198388Z/receipt-integration.json`.
- wow-sim: PASS, hash_reads=1, SHA256 matches seal=True, pre/post stat equality=True. Receipt: `<PRIVATE_V>/widget-discrete-progress-current/draw-layer-retail-green-20261014T040918198388Z/receipt-wow-sim.json`.

Initial gate-tool metadata-key error preserved in `gate-schema-failure-report.md` and `gate-schema-failure-proof.json`; corrected nested compiler_artifact lookup, integration never reread or rehashed. Hash order: integration, then wow-sim.

## Full compile.stderr

```text
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.large-enum-variant` is deprecated in favor of `lints.clippy.large_enum_variant` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.map-entry` is deprecated in favor of `lints.clippy.map_entry` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.match-wildcard-for-single-variants` is deprecated in favor of `lints.clippy.match_wildcard_for_single_variants` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.redundant-closure-for-method-calls` is deprecated in favor of `lints.clippy.redundant_closure_for_method_calls` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.trivially-copy-pass-by-ref` is deprecated in favor of `lints.clippy.trivially_copy_pass_by_ref` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.type-complexity` is deprecated in favor of `lints.clippy.type_complexity` and will not work in a future edition
warning: `iced_wgpu` (manifest) generated 6 warnings
   Compiling wow-ui-sim v0.1.0 (<REPO>)
   Compiling wow-ui-sim-test-harness v0.1.0 (<REPO>/crates/test-harness)
    Finished `test` profile [optimized + debuginfo] target(s) in 5m 06s
```
