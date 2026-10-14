# Retail copied integration authentication: PASS

NEW copied Retail integration executable only. Exactly one SHA256 pass. No executable reread after hashing; no build/runtime performed.

Path: `<epochs>/bullet-dress-owned-retail-red-20261013T234617677640Z/artifacts/deps/integration-9f07e4a55f2bbbf7`
SHA256: `522ad64ba678dd5a29a4cd9f75b1e42bd30f92f11601bd2b79da7c67e6c043a9`; matches seal.
Before/after stat equal seal exactly: `{"st_dev": 66306, "st_ino": 143019396, "st_mtime_ns": 1791935721202960554, "st_size": 471532288}`.
Source association: `76e8b830931d1489ad5a89a75306bcbaa2bc61f7`; profile retail. Source association is not source-byte provenance.

Complete compile.stdout parsed: 741 JSON rows; reason counts: {"build-finished": 1, "build-script-executed": 75, "compiler-artifact": 665}. Zero compiler-message diagnostics; build-finished success=true. Single integration artifact fresh=false; features match seal.

Complete compile.stderr:
```
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.large-enum-variant` is deprecated in favor of `lints.clippy.large_enum_variant` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.map-entry` is deprecated in favor of `lints.clippy.map_entry` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.match-wildcard-for-single-variants` is deprecated in favor of `lints.clippy.match_wildcard_for_single_variants` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.redundant-closure-for-method-calls` is deprecated in favor of `lints.clippy.redundant_closure_for_method_calls` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.trivially-copy-pass-by-ref` is deprecated in favor of `lints.clippy.trivially_copy_pass_by_ref` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.type-complexity` is deprecated in favor of `lints.clippy.type_complexity` and will not work in a future edition
warning: `iced_wgpu` (manifest) generated 6 warnings
   Compiling wow-ui-sim v0.1.0 (<repo>)
   Compiling wow-ui-sim-test-harness v0.1.0 (crates/test-harness)
    Finished `test` profile [optimized + debuginfo] target(s) in 9m 04s
```
Six manifest deprecations plus six-warning summary; no UnitRole diagnostic. Build reported 9m 04s.

Terminal metadata inspected, not independently measured or promoted into caller proof:
```
SubState=exited
Result=success
ExecMainStatus=0
MemoryPeak=7015276544
CPUUsageNSec=1241686564000
```

No source/archive/library/cache/oldbinary authentication. Runtime acceptance remains pending. Initial persistence evaluation failed before opening executable because evaluation-local variables were unavailable; it performed zero executable reads. Authentication above performed exactly one pass.

Both evidence files mode 0600, file-fsynced and parent-directory-fsynced, with immediate full readback.
