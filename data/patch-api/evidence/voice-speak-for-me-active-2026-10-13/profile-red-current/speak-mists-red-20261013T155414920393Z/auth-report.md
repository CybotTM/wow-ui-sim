# Independent new-copy authentication: PASS

Private epoch: `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/speak-mists-red-20261013T155414920393Z`.

## Authentication proof

Authenticated exactly once, only `artifacts/deps/integration-d6edc2b011e58495`. SHA256 `e9d1ce3b45c962c88be8e2734c4cb91fd7b2e1a2635e89f728fbf11d79ff9342` matches `seal-integration.json`. Before/after stat identical: device 66306, inode 141989074, mode 33261, size 279023496, mtime_ns 1791907090334856790, ctime_ns 1791907185155346675. Both match every recorded seal stat field. `receipt-integration.json` persisted immediately after authentication and read back with all checks true. No second authentication performed.

## Compile evidence

Read complete `compile.stderr` (untruncated), `compile-result.json`, `revision.json`, `submission.json`, `toolchain.json`, `ldd.txt`, and `source-scope.json`. Source revision: `abb979c4465efcf75c6731dd08077e90abaf5509`. Result records 385 JSON records, one build-finished success=true, non-fresh integration compiler artifact, features `client-mists,prefork-full-ui`, runtime_launched=false. Submission requested `cargo test --test integration --no-run --no-default-features --features client-mists --offline --locked -j4 --message-format=json-render-diagnostics`; resultant artifact additionally records prefork-full-ui. Headless scope; no GUI/CASC/sound feature authenticated. Cargo/rustc 1.99.0; rustc host x86_64-unknown-linux-gnu, LLVM 23.1.1, commit b940084d7eb6a299eb4bfeb8e34901bc051e7ac4.

Read-only command, explicit cwd equal to private epoch:
`systemctl --user show wow-speak-mists-red-20261013t155414920393z.service --property=ActiveState,SubState,Result,ExecMainCode,ExecMainStatus,ExecMainPID,ExecMainStartTimestamp,ExecMainExitTimestamp`

Command exit 0; output:
```text
ActiveState=active
SubState=exited
Result=success
ExecMainStartTimestamp=Tue 2026-10-13 10:54:14 CDT
ExecMainExitTimestamp=Tue 2026-10-13 10:58:10 CDT
ExecMainPID=1101109
ExecMainCode=1
ExecMainStatus=0
```

This independently confirms terminal compiler success, not merely launcher_exit=0. Main reports actual compile wait was not used; no wait performed here. Complete stderr ends `Finished test profile [optimized + debuginfo] target(s) in 3m 55s`. No E0603 in this epoch's compile stderr; initial failed compile at historical be2 is separate, per caller, and receives no RED credit.

## Warnings: not warning-free

Six manifest deprecations in iced-wgpu-patched/Cargo.toml: large-enum-variant, map-entry, match-wildcard-for-single-variants, redundant-closure-for-method-calls, trivially-copy-pass-by-ref, type-complexity.

Eight source warnings: src/blizzard_ui_sync.rs:28 (PROVENANCE_SCHEMA), :180 (CacheProvenance::new), :663 (remove_missing_marker); src/casc_asset_fallback.rs:93 (ensure_known_asset_cached); src/render/font.rs:44 (encoding_key_hex); src/lua_api/globals/real/unit_roles.rs:17 (Tank/Healer/Damage); src/widget/frame.rs:601/610 (cooldown methods, one diagnostic); src/bin/wow_sim/main.rs:20 (unused SavedVariablesManager import). Seven library warnings plus one binary warning. Empty compiler_messages in compile-result.json does not erase stderr warnings.

## Source/runtime boundaries

Source scope text records selected tracked inputs Cargo.toml, Cargo.lock, build.rs, build, src, tests, data, Interface, .cargo; runtime mount is private epoch/source. Archive exists, stat only: device 66306, inode 141989078, mode 33188, size 1176012800, mtime_ns/ctime_ns 1791907188607437323. Archive neither opened nor hashed; mount content not authenticated. ldd text lists linux-vdso.so.1, libgcc_s.so.1, libm.so.6, libc.so.6 and ld-linux-x86-64.so.2. Libraries remain unsealed. Vendor caches/WTF/settings remain external; no native parity claim.

No tests, runtime, other artifact authentication, restarts, builds, operational mutations, delegation, commits, or tracked edits. Only private receipt/report/proof written. One initial metadata command attempt stopped at NameError before any stat or command; corrected metadata-only call succeeded, without rehashing. Authentication permits main's next genuine two-case RED execution; no RED proof claimed here.
