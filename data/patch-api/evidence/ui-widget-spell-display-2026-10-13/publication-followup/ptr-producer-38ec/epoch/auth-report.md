# Two-copy authentication — PASS; runtime acceptance NOT GREEN

Epoch: `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/spell-display-ptr-green-20261013T185451536429Z`.

Read/followed `<HOME>/AgentConfig/skills/verify/SKILL.md` as assigned verifier. No delegation or operational actions.

## Independent authentication

Both corresponding receipts were absent before authentication. Only the two new copied executables named by `seal-integration.json` and `seal-wow-sim.json` were opened for binary reads. Each received exactly one independent SHA-256 streaming pass with 1 MiB chunks. Each pre-read stat matched its seal, and each post-read stat matched both pre-read stat and seal. Stat dictionaries contain exactly the seal keys: `st_dev`, `st_ino`, `st_size`, `st_mtime_ns`.

| Copy (epoch-relative path) | SHA-256 | Receipt |
| --- | --- | --- |
| `artifacts/deps/integration-020202567c2bd5a4` | `b6de3541123d51c35a1ede4bc57ac4546e8a14bf3e4dc04698a613a323e6ab69` | `receipt-integration.json` |
| `artifacts/wow-sim` | `7aa30a6aeda0f39dfbba88a2e12f61f6d8d98e4618dc5a5b91e04e4794c3511a` | `receipt-wow-sim.json` |

Immediately after each successful comparison, its receipt was exclusively created, flushed/fsynced, and read back for exact equality. Both receipts record `status: PASS`, `sha256_matches_seal: true`, `hash_reads: 1`, absolute `path`, `sha256`, and exact `stat_before`/`stat_after` dictionaries. No binary was rehashed.

## Compile and association metadata

Entire `compile.stdout` was read and all 741 JSON records parsed/inspected: 665 compiler-artifact, 75 build-script-executed, and one final build-finished record with `success: true`. Zero compiler-message records; `compile-result.json` records zero source compiler messages.

Entire `compile.stderr` was read: six manifest deprecation warnings in `iced-wgpu-patched/Cargo.toml` (large-enum-variant, map-entry, match-wildcard-for-single-variants, redundant-closure-for-method-calls, trivially-copy-pass-by-ref, type-complexity), plus one six-warning summary. No source compiler diagnostics. Finished test profile reported 2m 13s.

`compile-terminal-status.txt` was fully read and verified as `SubState=exited`, `Result=success`, `ExecMainStatus=0`.

`normal-artifact-association.json` contains the exact normal wow-sim compiler-artifact record present in the compile JSON (`profile.test=false`, `fresh=false`). `normal-deps-selection.json` records exactly one candidate, `wow_sim-616324da1af8e7f2`; its path/fingerprint match association metadata. Normal artifact features, captured fingerprint features, selection features, integration artifact features, and both seals match exactly; sole client feature is `client-ptr`. Integration compile record is `profile.test=true`, `fresh=false`, with basename matching its sealed copy. This is metadata association, not independent target-byte/source provenance.

Source revision `38ecbd8ee5dfae4315ca98001b7e794e324b1066` is metadata only, NOT authenticated. No source snapshot, source archive, library, old binary, or build-target file was read or hashed. Build-target paths were inspected only as strings within epoch metadata.

Inspection-script corrections: one metadata evaluation stopped before reads because its epoch variable was not carried across evaluations; another parsed/read compile evidence but incorrectly expected a bare terminal integer. Corrected parser verified the actual key/value terminal and distinguished six warnings from their summary. Neither incident caused binary rereads or receipt rewriting.

## Runtime boundary

No GREEN claim. User-supplied same-producer Retail runtime status remains 6 PASS / 3 FAIL, unknown-event unregister; not independently re-executed or authenticated here. PTR has not been executed yet. Authentication and no-run compile evidence do not establish runtime acceptance.

No runtime, build, delegation, commit, tracked writes, or operational changes performed. Main owns all operations. Only the two receipts and this report were written in the epoch.
