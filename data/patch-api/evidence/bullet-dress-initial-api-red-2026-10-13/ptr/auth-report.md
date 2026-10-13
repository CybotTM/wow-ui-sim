# PTR copied integration authentication

{
  "status": "PASS",
  "hash_reads": 1,
  "sha256_matches_seal": true,
  "path": "TOKEN_PTR_ROOT/artifacts/deps/TOKEN_IDENTIFIER_08",
  "sha256": "TOKEN_IDENTIFIER_02",
  "stat_before": {
    "st_dev": "TOKEN_ST_DEV_1",
    "st_ino": "TOKEN_ST_INO_1",
    "st_size": 471164688,
    "st_mtime_ns": 1791934311826663310
  },
  "stat_after": {
    "st_dev": "TOKEN_ST_DEV_1",
    "st_ino": "TOKEN_ST_INO_1",
    "st_size": 471164688,
    "st_mtime_ns": 1791934311826663310
  },
  "source": "TOKEN_IDENTIFIER_06",
  "profile": "ptr",
  "scope": "Copied executable authenticated against supplied seal only; source association is not source-byte provenance. No archive, libraries, cache, settings, old binary, or source authenticated. No builds or runtime tests executed."
}

## Compilation evidence

{
  "actual_json_rows": 741,
  "json_reasons": {
    "compiler-artifact": 665,
    "build-script-executed": 75,
    "build-finished": 1
  },
  "compiler_diagnostic_rows": 0,
  "compiler_diagnostic_levels": {},
  "manifest_warning_diagnostics": 6,
  "manifest_summary_lines": 1,
  "UnitRole_mentions_stdout": 0,
  "UnitRole_mentions_stderr": 0,
  "build_finished": [
    {
      "reason": "build-finished",
      "success": true
    }
  ],
  "terminal_exit_code": {
    "value": 0,
    "evidence": "User-provided main-terminal confirmation; not independently rechecked"
  },
  "peak_bytes": {
    "value": 8590635008,
    "evidence": "User-provided main-terminal confirmation; not independently rechecked"
  }
}

## Full stderr

```text
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.large-enum-variant` is deprecated in favor of `lints.clippy.large_enum_variant` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.map-entry` is deprecated in favor of `lints.clippy.map_entry` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.match-wildcard-for-single-variants` is deprecated in favor of `lints.clippy.match_wildcard_for_single_variants` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.redundant-closure-for-method-calls` is deprecated in favor of `lints.clippy.redundant_closure_for_method_calls` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.trivially-copy-pass-by-ref` is deprecated in favor of `lints.clippy.trivially_copy_pass_by_ref` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.type-complexity` is deprecated in favor of `lints.clippy.type_complexity` and will not work in a future edition
warning: `iced_wgpu` (manifest) generated 6 warnings
   Compiling wow-ui-sim v0.1.0 (TOKEN_REPO_ROOT)
   Compiling wow-ui-sim-test-harness v0.1.0 (TOKEN_REPO_ROOT/crates/test-harness)
    Finished `test` profile [optimized + debuginfo] target(s) in 3m 27s
```

Source TOKEN_IDENTIFIER_06 is associated TEST/SPEC-ONLY input; current c7 docs-after-13a relationship supplied by caller, not source-byte authenticated here. No tests run. Main fresh admission must run Bullet 2 and item 2; runtime acceptance remains pending.
