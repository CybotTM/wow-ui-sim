# Copied executable authentication: PASS

Exactly one sequential SHA256 pass over sealed executable. SHA256 matches seal: True. Before/opened-before/opened-after/after stat stable: True. Stat matches seal: True.

Path: `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/calendar-day-ptr-red-20261013T213459928274Z/artifacts/deps/integration-020202567c2bd5a4`
SHA256: `362841d7b5c3066db503cf8e50f380bf92ebd46984027cbc51d33216ea21104f`

## Scope
Copy integrity only; not native/runtime/source provenance. No other executable, library, source, or archive hashed. No builds, runtime execution, launch, delegation, or tracked edits. Parent owns launch and fresh admission. PASS does not establish genuine runtime RED.

## Requested log gap
`fullcompile.stderr` does not exist in this epoch; requested full-log evidence unavailable. Existing `compile.stderr` read completely and reproduced below, not authenticated as the missing file. This gap does not change the narrowly scoped copy-integrity result.

## Compilation metadata (read, not independently authenticated)
`compile-result.json`: build-finished success true; 741 records; source_compiler_messages empty; six manifest deprecations; no-run compilation only, runtime acceptance pending.
`revision.json`: source_revision 48902323e64bef2229376a1c1c8c8b4e4cf9e09a; production_delta_revision 3df756436; Calendar day passive DTO/empty map and nine tests; PTR pre-producer compile only; runtime genuine RED pending; numeric index defaults/filtering unchanged. Git status records untracked .code-index.db and verification/.

## Full terminal status
```text
ActiveState=active
SubState=exited
Result=success
ExecMainStatus=0
```

## Full available compile.stderr, including warnings
```text
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.large-enum-variant` is deprecated in favor of `lints.clippy.large_enum_variant` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.map-entry` is deprecated in favor of `lints.clippy.map_entry` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.match-wildcard-for-single-variants` is deprecated in favor of `lints.clippy.match_wildcard_for_single_variants` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.redundant-closure-for-method-calls` is deprecated in favor of `lints.clippy.redundant_closure_for_method_calls` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.trivially-copy-pass-by-ref` is deprecated in favor of `lints.clippy.trivially_copy_pass_by_ref` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.type-complexity` is deprecated in favor of `lints.clippy.type_complexity` and will not work in a future edition
warning: `iced_wgpu` (manifest) generated 6 warnings
   Compiling wow-ui-sim v0.1.0 (<REPO>)
    Finished `test` profile [optimized + debuginfo] target(s) in 3m 38s
```
