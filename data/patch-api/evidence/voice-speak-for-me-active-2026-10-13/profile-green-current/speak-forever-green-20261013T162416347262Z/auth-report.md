# Artifact authentication: PASS

New copy: `artifacts/deps/integration-4c29cd3cf9c069c8`. Hashed exactly once; receipt persisted immediately and read back. SHA-256 `ba5f6e1d2ed7363aca2dd7126b5b64d1ee10dc1fbd97a5aa5e51ceaf1719a158` matches main seal. Before/after stat matches seal; subsequent checks stat-only. Receipt: `receipt-integration.json`.

All 387 compiler JSON records parsed: 349 compiler-artifact, 37 build-script-executed, 1 build-finished (success=true). Integration record unique, fresh=false, test profile; executable basename matches copy. All 12 features match seal. Actual argv uses --no-run --no-default-features --features client-wowforever. Original revision.json Mists label erroneous; compile-result.json records WoW Forever correction.

Full compile.stderr inspected: 8 source warnings, 6 manifest deprecations, 3 warning summary lines (17 warning-prefixed lines total), 0 error-prefixed lines. JSON has 0 compiler-message records; diagnostics counted from stderr.

Terminal systemctl --user show exited 0: loaded, active/exited, RemainAfterExit=yes, MainPID=0, ControlPID=0, Result=success, ExecMainCode=1 (exited), ExecMainStatus=0. InvocationID 29e349dc8377455aa532651d5f07aeec. Compile exit October 13, 2026 11:28:03 CDT.

Revision metadata agrees on `4eadd7cf3ef84581ad35967ee6760951939ec438`; read-only git show confirms commit exists. Selected source paths present: Cargo.toml, Cargo.lock, build.rs, build, src, tests, data, Interface, .cargo. Revision attribution is metadata, not source/archive equivalence proof; compiler JSON does not embed commit identity. Caches/WTF/settings/system libraries outside scope.

No delegation, build, runtime, operations mutation, canonical edits, old executable/library/source/archive hashes, or new-copy rehash performed. No warning-free/CLEAN/runtime/native/parent claims. Detailed proof: `auth-proof.json`.
