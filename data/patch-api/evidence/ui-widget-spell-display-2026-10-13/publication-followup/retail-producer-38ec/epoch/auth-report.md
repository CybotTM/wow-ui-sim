# Authentication report

Status: PASS — only the two new sealed executable copies authenticated.

Producer-declared source: `38ecbd8ee5dfae4315ca98001b7e794e324b1066`; profile: retail. Source bytes were not authenticated.

| Copy | Receipt | SHA-256 | Result |
| --- | --- | --- | --- |
| integration | receipt-integration.json | 001ceaec2591fe82e4d25f16100565cca43ac329d4fb92f41ff941513464b433 | PASS |
| wow-sim | receipt-wow-sim.json | aacdf0417806f789282de03967294f790ba3ce74788859e1efb81acd9a036b83 | PASS |

Both receipts were asserted absent before hashing. Each copy was independently hashed exactly once using 1 MiB chunks. Before-stat matched all exact seal keys (st_dev, st_ino, st_size, st_mtime_ns); after-stat and opened-file stats were unchanged. Each PASS receipt was immediately persisted, fsynced, read back, and checked for equality before proceeding. Both record sha256_matches_seal=true and hash_reads=1.

## Compilation evidence

Entire compile.stdout read and all 741 JSON records parsed ({'compiler-artifact': 665, 'build-script-executed': 75, 'build-finished': 1}). Zero compiler-message records, matching compile-result.json's zero source compiler messages. Build-finished success=true. Entire compile.stderr read: six iced-wgpu-patched/Cargo.toml manifest deprecations, plus the manifest aggregate warning; finished test profile in 2m 41s. Full terminal status: SubState=exited, Result=success, ExecMainStatus=0.

## Normal artifact association

Full normal-artifact-association.json and normal-deps-selection.json read. Exactly one normal wow-sim compiler artifact matches the association record; exactly one selection candidate matches its profile_unique_deps_path and fingerprint_path. Fingerprint features, selected features, Cargo artifact features, and seal features match. Integration's unique executable artifact basename and features match its seal. These are Cargo feature/profile/fingerprint/time metadata associations only, not binary-to-source cryptographic authentication. No build-target file was opened or hashed.

## Limits

No GREEN runtime acceptance yet; compilation was no-run evidence only. No runtime, build, delegation, commit, tracked writes, or operational mutation performed. No old binaries, libraries, archives, source snapshots, or build-target contents hashed. Source 38ec is producer-declared, not independently authenticated. Main owns operational steps.

One metadata evaluation stopped before file access because evaluation globals did not persist (E undefined). The subsequent evaluation loaded metadata and receipts afresh; neither executable was rehashed.
