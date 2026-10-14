# PTR compile and sealed integration authentication: PASS

Source: `bde7ea277e86f9ad17f7135b3cd676f80e73057b`.
Epoch: `<epochs>/bullet-fixture-ptr-green-20261014T005940095211Z`.

- Complete `compile.stdout`: 741 valid JSON records (665 compiler-artifact, 75 build-script-executed, 1 build-finished); success=true; zero compiler diagnostics. Complete `compile.stderr`: six manifest deprecations, not a warning-free build; Cargo finished in 40.23s.
- `compile-terminal.txt`: Result=success, ExecMainCode=1 (exited), ExecMainStatus=0; October 13, 2026, 19:59:40–20:00:20 CDT (October 14, 00:59:40–01:00:20 UTC); peak 5,138,264,064 bytes.
- New sealed copy: `<epochs>/bullet-fixture-ptr-green-20261014T005940095211Z/artifacts/deps/integration-020202567c2bd5a4`. Single sequential SHA-256 read; 471237368 bytes. Digest `696e2160600513236b0dbbaa89d790a3080ca2d50098081eae4c641aa5aa08c7` matches `seal-integration.json`; seal/pre/post stats match. Integration artifact name and PTR features match saved compiler output.
- Receipt absent before authentication. `<epochs>/bullet-fixture-ptr-green-20261014T005940095211Z/receipt-integration.json` created with status PASS, sha256_matches_seal=true, hash_reads=1, path, stat_after, digest; mode 0600; file/directory fsync and exact readback passed.
- **Runtime GREEN UNPROVED.** No commands rerun, services changed, delegation, binary copying, normal/previous binaries, libraries, archives, or source trees authenticated. No helper contract/readability claim made.

Proof: `<verification>/bullet-fixture-ptr-auth-current/proof.json`.
Report: `<verification>/bullet-fixture-ptr-auth-current/report.md`.
