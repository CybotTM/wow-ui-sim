# Bounded new-copy authentication — PASS

Epoch: `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/text-state-retail-red-speak-preservation-20261013T163822997097Z`
Declared source: `64a2a57b27c5c2b021375cbfddda557ec145507c` (metadata attribution only).

## Authenticated copies

- `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/text-state-retail-red-speak-preservation-20261013T163822997097Z/artifacts/deps/integration-9f07e4a55f2bbbf7`: PASS; SHA-256 `7436d81e41cce0892266455cdae620d2089a70a9cbd7323f3f1b8392e341e680`; exactly one hash read; before/after stat equals seal. Receipt `receipt-integration.json` persisted immediately and read back.
- `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/text-state-retail-red-speak-preservation-20261013T163822997097Z/artifacts/wow-sim`: PASS; SHA-256 `dfce1a94f9103bf9b1bd5e264d925b7985a61b7f2f45fa558b5c57cd3396cebf`; exactly one hash read; before/after stat equals seal. Receipt `receipt-wow-sim.json` persisted immediately and read back.

## Existing compile evidence

All 741 JSON records parsed/inspected: 665 compiler-artifact, 75 build-script-executed, 1 successful build-finished; zero compiler-message records. Both selected executable records are fresh=false (newly compiled), with features identical to corresponding seals. Full feature arrays and source paths are recorded in `auth-proof.json`.

Terminal unit `wow-discrete-text-state-retail-red-speak-preservation-20261013t163822997097z.service`: loaded, active/exited, Result=success, ExecMainCode=1 (exited), ExecMainStatus=0. InvocationID `4c9585e6205e4f8da4fe73c2f55b9518` matches submission; compile started 2026-10-13 11:38:24 CDT and ended 11:41:49 CDT. Command: `/usr/bin/env RUSTC=/usr/bin/rustc /usr/bin/cargo test --test integration --no-run --offline --locked -j4 --message-format=json-render-diagnostics`. Existing evidence only; no build invoked.

Full stderr (six manifest deprecations, not warning-free):
```text
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.large-enum-variant` is deprecated in favor of `lints.clippy.large_enum_variant` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.map-entry` is deprecated in favor of `lints.clippy.map_entry` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.match-wildcard-for-single-variants` is deprecated in favor of `lints.clippy.match_wildcard_for_single_variants` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.redundant-closure-for-method-calls` is deprecated in favor of `lints.clippy.redundant_closure_for_method_calls` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.trivially-copy-pass-by-ref` is deprecated in favor of `lints.clippy.trivially_copy_pass_by_ref` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.type-complexity` is deprecated in favor of `lints.clippy.type_complexity` and will not work in a future edition
warning: `iced_wgpu` (manifest) generated 6 warnings
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling wow-ui-sim v0.1.0 (<REPO>)
   Compiling wow-ui-sim-test-harness v0.1.0 (<REPO>/crates/test-harness)
    Finished `test` profile [optimized + debuginfo] target(s) in 3m 23s
```

## Dependency resolution

### integration
`ldd` exit 0; missing dependencies: none. Dependency bytes not authenticated.
```text
	linux-vdso.so.1 (0x00007f06e34fe000)
	libicui18n.so.78 => /usr/lib/libicui18n.so.78 (0x00007f06db800000)
	libicuuc.so.78 => /usr/lib/libicuuc.so.78 (0x00007f06db400000)
	libasound.so.2 => /usr/lib/libasound.so.2 (0x00007f06db707000)
	libgcc_s.so.1 => /usr/lib/libgcc_s.so.1 (0x00007f06e34b4000)
	libm.so.6 => /usr/lib/libm.so.6 (0x00007f06db2c9000)
	libc.so.6 => /usr/lib/libc.so.6 (0x00007f06db000000)
	/lib64/ld-linux-x86-64.so.2 => /usr/lib64/ld-linux-x86-64.so.2 (0x00007f06e3500000)
	libstdc++.so.6 => /usr/lib/libstdc++.so.6 (0x00007f06dac00000)
	libicudata.so.78 => /usr/lib/libicudata.so.78 (0x00007f06d8c00000)
```
Stderr: ''

### wow-sim
`ldd` exit 0; missing dependencies: none. Dependency bytes not authenticated.
```text
	linux-vdso.so.1 (0x00007ff539c70000)
	libicui18n.so.78 => /usr/lib/libicui18n.so.78 (0x00007ff533200000)
	libicuuc.so.78 => /usr/lib/libicuuc.so.78 (0x00007ff532e00000)
	libasound.so.2 => /usr/lib/libasound.so.2 (0x00007ff539b5a000)
	libssl.so.3 => /usr/lib/libssl.so.3 (0x00007ff533107000)
	libcrypto.so.3 => /usr/lib/libcrypto.so.3 (0x00007ff532800000)
	libgcc_s.so.1 => /usr/lib/libgcc_s.so.1 (0x00007ff539b2b000)
	libm.so.6 => /usr/lib/libm.so.6 (0x00007ff5326c9000)
	libc.so.6 => /usr/lib/libc.so.6 (0x00007ff532400000)
	/lib64/ld-linux-x86-64.so.2 => /usr/lib64/ld-linux-x86-64.so.2 (0x00007ff539c72000)
	libstdc++.so.6 => /usr/lib/libstdc++.so.6 (0x00007ff532000000)
	libicudata.so.78 => /usr/lib/libicudata.so.78 (0x00007ff530000000)
	libz.so.1 => /usr/lib/libz.so.1 (0x00007ff539b0e000)
	libbrotlienc.so.1 => /usr/lib/libbrotlienc.so.1 (0x00007ff533047000)
	libbrotlidec.so.1 => /usr/lib/libbrotlidec.so.1 (0x00007ff539aff000)
	libzstd.so.1 => /usr/lib/libzstd.so.1 (0x00007ff53231a000)
	libbrotlicommon.so.1 => /usr/lib/libbrotlicommon.so.1 (0x00007ff5335dd000)
```
Stderr: ''

## Attribution and boundaries

Revision/submission/source-scope/seals consistently name the declared source; Cargo records establish source paths and default Retail feature attribution, not cryptographic revision provenance. Source archive selected, not independently authenticated. Shared libraries/cache/settings not authenticated. No old executables, source, archive, or libraries hashed. Post-hash executable stat evidence reused from receipts.

Test/query validation remains UNRUN. Runtime validation UNRUN. No CLEAN, native, or parent claims. No runtime invocation, new build, operational mutation, delegation, or canonical edits.

Two inspection evaluations failed on unavailable cross-evaluation globals; durable hash receipts were preserved and no hash was repeated. Original successful terminal-query output was used without rerunning the command.
