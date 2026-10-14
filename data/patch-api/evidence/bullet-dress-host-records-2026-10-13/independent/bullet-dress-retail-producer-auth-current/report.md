# Retail producer / new sealed copies: PASS (bounded scope)

Epoch: `<epochs>/bullet-dress-retail-green-retry2-20261014T004047052063Z`
Source revision (metadata association, not source authentication): `69f2569cb48e2dbc9ae778bc70e76210fb6ffc4c`.

## Compile evidence
Full stdout parsed: **741/741 JSON records**: 665 compiler-artifact, 75 build-script-executed, 1 build-finished (success=true). **0 compiler diagnostics**. Both integration and wow-sim artifact records have fresh=false and matching Retail features; normal artifact association and embedded fingerprint features match.
Launcher exit 0. Saved terminal and live read-only systemctl show agree: ActiveState=active, SubState=exited, Result=success, ExecMainCode=1 (CLD_EXITED), ExecMainStatus=0. Initial running observation is not terminal proof. Start October 13, 2026 19:40:47 CDT; exit 19:44:49 CDT (October 14 UTC). Duration **4m 02s / 242 seconds**; peak **8,115,019,776 bytes**.
Full stderr read: **6 manifest deprecations**, plus one aggregate warning line (7 warning-prefixed lines total), not zero warnings.

```text
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.large-enum-variant` is deprecated in favor of `lints.clippy.large_enum_variant` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.map-entry` is deprecated in favor of `lints.clippy.map_entry` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.match-wildcard-for-single-variants` is deprecated in favor of `lints.clippy.match_wildcard_for_single_variants` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.redundant-closure-for-method-calls` is deprecated in favor of `lints.clippy.redundant_closure_for_method_calls` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.trivially-copy-pass-by-ref` is deprecated in favor of `lints.clippy.trivially_copy_pass_by_ref` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.type-complexity` is deprecated in favor of `lints.clippy.type_complexity` and will not work in a future edition
warning: `iced_wgpu` (manifest) generated 6 warnings
   Compiling wow-ui-sim v0.1.0 (<repo>)
   Compiling wow-ui-sim-test-harness v0.1.0 (crates/test-harness)
    Finished `test` profile [optimized + debuginfo] target(s) in 4m 02s
```

## New copy authentication
Receipts absent before hashing. Sequential single SHA-256 pass per copy; pre-open, opened, post-read descriptor and post-read path seal fields agree (device, inode, size, mtime_ns). Each receipt created exclusively, mode **0600**, file and directory fsync, JSON readback verified.

| Copy | Size bytes | SHA-256 | Hash passes | Receipt |
|---|---:|---|---:|---|
| integration | 471369544 | 667006f8d2f3e7a621e3d825f75ad9024e589b3f3a5117a58038241e374d72df | 1 | `<epochs>/bullet-dress-retail-green-retry2-20261014T004047052063Z/receipt-integration.json` |
| wow-sim | 408384784 | 0f45288c02b0ee75253b4b9e5097b23397f23b8b6df36f93b3d0a2299b796b35 | 1 | `<epochs>/bullet-dress-retail-green-retry2-20261014T004047052063Z/receipt-wow-sim.json` |

## Limits / harness metadata
**Runtime unproved. 0 tests/workloads run.** No Cargo/test/check/fmt rerun, delegation, service mutation, Bash or cwd tools. No authentication of old binaries, libraries, archives, source tree, cache or settings. This verifies producer records and sealed-copy identity, not cryptographic source-to-binary provenance or functional acceptance.
Caller-reported initial sealer ctx-field error is preserved as harness metadata: failed before copying/hashing; no standalone error artifact observed. Verifier had one read-only discovery NameError (root undefined), recovered before authentication with explicit path; no hashing in that failed call.

Proof: `<verification>/bullet-dress-retail-producer-auth-current/proof.json`
