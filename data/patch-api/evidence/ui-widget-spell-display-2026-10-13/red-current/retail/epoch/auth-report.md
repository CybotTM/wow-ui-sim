# Retail copied integration authentication

Epoch: <PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/spell-display-retail-red-20261013T181246353729Z

## Authentication: PASS

Receipt absent before authentication; exclusively created and fsynced immediately after successful comparison. Exactly one sequential SHA-256 pass over new copied executable, with 1 MiB chunks and bounded digest buffer. SHA-256 matches seal; stat before matches seal and stat after unchanged.

Executable: <PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/spell-display-retail-red-20261013T181246353729Z/artifacts/deps/integration-9f07e4a55f2bbbf7
SHA-256: ff0d6346860308904ad0147f55b3a838417e7d41e615a0470b18a72a7d05815d
Receipt: <PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/spell-display-retail-red-20261013T181246353729Z/receipt-integration.json

Persisted receipt readback matched. Exact stat keys: st_dev, st_ino, st_size, st_mtime_ns.

```json
{
  "status": "PASS",
  "sha256_matches_seal": true,
  "hash_reads": 1,
  "path": "<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/spell-display-retail-red-20261013T181246353729Z/artifacts/deps/integration-9f07e4a55f2bbbf7",
  "sha256": "ff0d6346860308904ad0147f55b3a838417e7d41e615a0470b18a72a7d05815d",
  "stat_before": {
    "st_dev": 66306,
    "st_ino": 141951915,
    "st_size": 470973600,
    "st_mtime_ns": 1791915453848686892
  },
  "stat_after": {
    "st_dev": 66306,
    "st_ino": 141951915,
    "st_size": 470973600,
    "st_mtime_ns": 1791915453848686892
  }
}
```

## Compilation: PASS — separate from RED

Complete compile.stdout inspected: 741 JSON records; reason counts: {'compiler-artifact': 665, 'build-script-executed': 75, 'build-finished': 1}. All compiler-message JSON inspected: zero records. build-finished success=true. Terminal: SubState=exited, Result=success, ExecMainStatus=0.

FULL compile.stderr inspected: six distinct manifest deprecation warnings; aggregate summary is not a seventh warning. No source compiler-message diagnostics.

## RED: UNEXECUTED

No runtime/test execution. Compilation and authentication do not prove RED failure or behavioral acceptance.

## Scope

Only new Retail copied executable named in <PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/spell-display-retail-red-20261013T181246353729Z/seal-integration.json authenticated. No old binaries, libraries, source archives, source trees, or build targets hashed. No runtime, build, delegation, commit, or tracked writes. Main performed operations directly under explicit assignment.

## FULL compile.stderr

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
    Finished `test` profile [optimized + debuginfo] target(s) in 4m 49s
```

## FULL terminal status

```text
SubState=exited
Result=success
ExecMainStatus=0
```
