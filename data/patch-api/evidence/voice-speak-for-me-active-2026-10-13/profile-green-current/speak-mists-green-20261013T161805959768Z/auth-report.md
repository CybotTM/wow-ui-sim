# Copied Mists integration authentication

PASS — executable authenticated once; receipt persisted and read back immediately.

- Source: `f835b158a57620662f73ac5c6ed13a68a353532c`; production query: `2630c0ccf`.
- Artifact features: `client-mists`, `prefork-full-ui`; headless.
- SHA-256: `01e77b85b0f6636c979c664ff8c243ce5993b25d9be538bad0e3dfec76683998`; before/after stats stable and identical to seal.
- Compile stdout: 385 JSON records; summary matches; build-finished success=true.
- Full compile.stderr inspected: 6 manifest + 8 source warnings preserved; finished test profile in 3m 27s.
- Read-only systemctl: active/exited, success, ExecMainCode=1 (exited), ExecMainStatus=0; command exit 0.
- Receipt: `receipt-integration.json`; structured proof: `auth-proof.json`.

No runtime, build, tests, operations, delegation, commits, tracked edits, mounts, library/archive/source-tree hashing, old binaries, or re-authentication. Source gate 3781 PASS does not prove GREEN. Main must execute two actual cases using this authenticated copy.
