# Authentication report — PASS (two copied executables only)

Epoch: `spell-display-retail-removal-green-20261013T193523870575Z`
Source metadata: `621649e56d8418d21c4539b26d787ad40c73431f`; profile: retail.

## Copied executable authentication

Both receipts were absent before authentication. Each copy received exactly one independent SHA-256 streaming pass using 1 MiB chunks. Before-read stat matched its exact seal stat; descriptor and path stats remained unchanged after reading. Seal bytes remained unchanged. Each receipt was exclusively created, flushed/fsynced, and immediately read back before authenticating the next copy.

- `integration`: PASS; `sha256_matches_seal=true`; `hash_reads=1`.
  Path: `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/spell-display-retail-removal-green-20261013T193523870575Z/artifacts/deps/integration-9f07e4a55f2bbbf7`
  SHA-256: `8a57b4feb648a2014e3617aac8202b79f08aaa9227ab94c9b55afe15d282e44a`
  Exact stat_before = stat_after = seal.stat: `{"st_dev": 66306, "st_ino": 142479842, "st_mtime_ns": 1791920372989653997, "st_size": 470982048}`.
  Receipt: `receipt-integration.json` (readback matched).

- `wow-sim`: PASS; `sha256_matches_seal=true`; `hash_reads=1`.
  Path: `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/spell-display-retail-removal-green-20261013T193523870575Z/artifacts/wow-sim`
  SHA-256: `88c42fc23dbbbbd76237e7bfb6882204334c3c3f33643b05ce59e15af0dc009a`
  Exact stat_before = stat_after = seal.stat: `{"st_dev": 66306, "st_ino": 142479845, "st_mtime_ns": 1791920284944505512, "st_size": 408320640}`.
  Receipt: `receipt-wow-sim.json` (readback matched).

## Compilation evidence — PASS (no-run only)

Read and parsed the complete compile.stdout: 741 JSON records, zero compiler-message/source diagnostic records, one build-finished with success=true. Read full compile.stderr and terminal metadata: SubState=exited, Result=success, ExecMainStatus=0. Six iced_wgpu manifest lint-name deprecations remain, plus the manifest's six-warning summary; these are not source compiler messages. Compilation finished in 4m 09s.

Exact deprecations:

- warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.large-enum-variant` is deprecated in favor of `lints.clippy.large_enum_variant` and will not work in a future edition
- warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.map-entry` is deprecated in favor of `lints.clippy.map_entry` and will not work in a future edition
- warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.match-wildcard-for-single-variants` is deprecated in favor of `lints.clippy.match_wildcard_for_single_variants` and will not work in a future edition
- warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.redundant-closure-for-method-calls` is deprecated in favor of `lints.clippy.redundant_closure_for_method_calls` and will not work in a future edition
- warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.trivially-copy-pass-by-ref` is deprecated in favor of `lints.clippy.trivially_copy_pass_by_ref` and will not work in a future edition
- warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.type-complexity` is deprecated in favor of `lints.clippy.type_complexity` and will not work in a future edition

## Normal artifact association — PASS (metadata only)

Exactly one normal wow-sim compiler-artifact record (profile.test=false, fresh=false), matching normal-artifact-association.json exactly. Exactly one integration compiler-artifact record (profile.test=true). Both feature lists match their seals. normal-deps-selection.json has one candidate, `<REPO>/target/debug/deps/wow_sim-a69cd69216e5522c`, matching the association's deps path and fingerprint-path metadata; embedded fingerprint features match the compiler artifact and selection. No referenced target/fingerprint files were opened. Metadata association does not establish binary/source cryptographic provenance.

## Policy and acceptance boundary

User-specified source621 policy fix: ordinary unknown-event removal returns false; strict registrations remain unchanged. revision.json identifies production delta 621649e56 and reports preserved registration/aspect gates after direct-fixture RED. This authentication task did not inspect source or independently prove those behavioral claims. No post-fix runtime GREEN established; successful no-run compilation and matching copied executable hashes are not runtime acceptance.

Only the two new copied executables were read for hashing. No old binaries, libraries, archives, source trees, or build targets were read/hashed. No runtime execution, build, delegation, commit, or tracked write occurred. Only receipt-integration.json, receipt-wow-sim.json, and this report were written in the epoch. Main owns operational actions.

Verifier bookkeeping: an initial metadata evaluation failed before any file read because a prior evaluation's Python globals were unavailable. A subsequent metadata assertion incorrectly expected the terminal file to contain a bare 0; inspection confirmed the recorded ExecMainStatus=0 and the structured terminal assertion passed. Neither issue caused a binary re-read or another hashing pass.
