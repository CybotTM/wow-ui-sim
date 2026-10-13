# Integration-copy authentication: PASS

Scope: bounded one-time authentication of only the NEW copied Mists integration executable at this epoch. Source revision recorded by seal/build metadata: `a90f84c5de9d09aedcec93aeda24c6bf5463e2b2`; XML test-fixture-only scope. Source content/archive provenance was not independently authenticated.

## Copy integrity

- Receipt was absent before authentication. Exactly one SHA-256 stream read of `artifacts/deps/integration-37489d648a0b36d0`.
- SHA-256 matches seal: `f97c606939f22bc6879e1041dea31e330416e696fb1d6d84a5afb64168e4080c`.
- Stat before and after stable and exactly equal seal: `st_dev=66306`, `st_ino=147738711`, `st_size=438138336`, `st_mtime_ns=1791930024466337059`.
- Immediately wrote `receipt-integration.json` with exclusive creation, mode 0600, flushed/fsynced, read back and verified content/mode. Receipt status PASS; hash_reads 1; stat_after uses exact st_* keys.
- Initial follow-on evaluation failed on undefined `receipt` before accessing the artifact or hashing; no extra hash read occurred.

## Compilation evidence

Read complete compile-terminal-status.txt, compile-result.json, compile.stdout JSON stream and compile.stderr. Terminal: ActiveState=active, SubState=exited, Result=success, ExecMainStatus=0. JSON stream: 741 records, one build-finished with success=true, zero compiler-message records. Cargo stderr: Finished test profile in 5m 15s. This is no-run compilation, NOT warning-free and NOT behavioral RED.

Full warning accounting:

1. iced-wgpu-patched/Cargo.toml: lints.clippy.large-enum-variant deprecated; replacement large_enum_variant.
2. iced-wgpu-patched/Cargo.toml: lints.clippy.map-entry deprecated; replacement map_entry.
3. iced-wgpu-patched/Cargo.toml: lints.clippy.match-wildcard-for-single-variants deprecated; replacement match_wildcard_for_single_variants.
4. iced-wgpu-patched/Cargo.toml: lints.clippy.redundant-closure-for-method-calls deprecated; replacement redundant_closure_for_method_calls.
5. iced-wgpu-patched/Cargo.toml: lints.clippy.trivially-copy-pass-by-ref deprecated; replacement trivially_copy_pass_by_ref.
6. iced-wgpu-patched/Cargo.toml: lints.clippy.type-complexity deprecated; replacement type_complexity.
7. src/lua_api/globals/real/unit_roles.rs:17:5: UnitRole variants Tank, Healer, Damage never constructed (also lines 18 and 19); dead_code/unused warning. Derived Clone ignored during dead-code analysis.

Stderr summary: iced_wgpu manifest generated 6 warnings; wow-ui-sim lib generated 1 warning. Zero JSON compiler-message records do not negate this source warning.

## Resource/admission evidence

Terminal measured MemoryPeak=8234831872 bytes (about 7.669 GiB), below worker MemoryMax=9663676416 (9 GiB) and MemoryHigh=8589934592 (8 GiB). Submission records CPUQuota=400%, -j4, no-run Mists integration compile. Admission allowed=true. Two earlier admission denials with zero launches are separately recorded in revision/admission scope; not this successful compile, and not reauthenticated here. Their underlying epoch evidence was not independently read in this bounded task.

## Exclusions and remaining owner work

No other binary/library/source/archive hash or authentication; no runtime execution, build, delegation, tracked edits, Bash, or cwd change. External libraries, caches and settings remain unauthenticated. Compile/copy integrity proves neither XML behavior nor genuine RED. Parent owns fresh runtime admission and launch of the two XML behavioral RED cases; no behavioral result claimed here.
