# Authentication report

PASS — exactly two new copied executables authenticated against supplied seals.

## Independent byte authentication

Each respective receipt was asserted absent before opening its executable. Each copy received exactly one streaming SHA-256 pass (8 MiB chunks, one open/read-to-EOF traversal); pre-open path stat, open-file pre/post stat, and post-read path stat matched its seal exactly. No executable reread or second hash pass occurred.

`receipt-integration.json` was created exclusively, flushed, fsynced, directory-fsynced, and read back successfully before opening wow-sim. `receipt-wow-sim.json` then received the same exclusive persistence/fsync/readback sequence. Both satisfy runtime consumer fields: `status: PASS`, `sha256_matches_seal: true`, `hash_reads: 1`, exact sealed `path`, and `stat_after` containing `st_dev`, `st_ino`, `st_size`, `st_mtime_ns`.

### integration

Path: `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/spell-display-ptr-removal-green-20261013T194351858902Z/artifacts/deps/integration-020202567c2bd5a4`

SHA-256: `442b87f11d10f6b7a89ac9338e2140b3a20009df5c1c867a84edeac5621112c8`

Exact stat before/after: `{"st_dev": 66306, "st_ino": 144611177, "st_mtime_ns": 1791920806237428414, "st_size": 470526080}`

### wow-sim

Path: `<PRIVATE_VERIFICATION_ROOT>/widget-discrete-progress-current/spell-display-ptr-removal-green-20261013T194351858902Z/artifacts/wow-sim`

SHA-256: `827452f9d90ec9cb191729496606d6c6d33a148f69b5082f0bdd6f04c22a0b12`

Exact stat before/after: `{"st_dev": 66306, "st_ino": 144611178, "st_mtime_ns": 1791920744155869300, "st_size": 415361736}`

## Complete saved compilation evidence

Full `compile.stdout` consumed and parsed: 574600 bytes, 741 lines, 741 JSON records, zero non-JSON lines. Declared `compile-result.json.records`: 741; independently observed actual: 741 (matches). Breakdown: 665 compiler-artifact, 75 build-script-executed, one build-finished. Terminal record: `{"reason":"build-finished","success":true}`. Zero compiler-message records; zero source compiler diagnostics observed, distinct from manifest warnings.

Full `compile.stderr` inspected. Six distinct manifest deprecations from `iced-wgpu-patched/Cargo.toml` (not six source diagnostics):

```text
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.large-enum-variant` is deprecated in favor of `lints.clippy.large_enum_variant` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.map-entry` is deprecated in favor of `lints.clippy.map_entry` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.match-wildcard-for-single-variants` is deprecated in favor of `lints.clippy.match_wildcard_for_single_variants` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.redundant-closure-for-method-calls` is deprecated in favor of `lints.clippy.redundant_closure_for_method_calls` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.trivially-copy-pass-by-ref` is deprecated in favor of `lints.clippy.trivially_copy_pass_by_ref` and will not work in a future edition
warning: iced-wgpu-patched/Cargo.toml: `lints.clippy.type-complexity` is deprecated in favor of `lints.clippy.type_complexity` and will not work in a future edition
```

Manifest summary: `iced_wgpu` (manifest) generated 6 warnings. Terminal stderr: Finished `test` profile [optimized + debuginfo] target(s) in 2m 54s.

Terminal service metadata inspected read-only: `wow-discrete-spell-display-ptr-removal-green-20261013t194351858902z.service`, loaded, active/exited (RemainAfterExit), Result=success, ExecMainCode=1 (exited), ExecMainStatus=0. Start October 13, 2026 14:43:52 CDT; exit October 13, 2026 14:46:46 CDT. InvocationID=dfefc24a5d1847de815c797ffd17db89 matches saved submission. Launcher exit 0 alone was not treated as compiler exit proof. Saved command is offline/locked/no-default-features `cargo test --features sound,gui,casc,client-ptr --no-run --test integration -j4 --message-format=json-render-diagnostics`.

## Normal executable association — metadata only

Saved normal compiler-artifact exactly matches the wow-sim record in full stdout. Target kind bin, target name wow-sim, fresh=false; normal profile test=false (integration test=true). Normal profile matches saved selection: opt_level=1, debuginfo=line-tables-only, debug_assertions=true, overflow_checks=true. Features match stdout integration/normal records, both seals, selection metadata, and saved fingerprint feature list:

`aura-containers, aura-instance-enumeration, aura-xml-widgets, base-spell-relationships, casc, client-ptr, forbidden-animation-aspects, forbidden-aspects, gui, native-duration-formatting, numeric-rule-formatters, on-update-modes, player-cast-durations, retail-12-0-0, retail-12-0-5, retail-12-0-7, retail-12-1-0, retail-12-1-5, rodio, sound, timed-signal-maps`

Saved unique normal-deps selection: `target/debug/deps/wow_sim-616324da1af8e7f2`; compiler-artifact names `target/debug/wow-sim`. Only saved metadata was inspected; neither target executable nor its fingerprint file was opened. Association is feature/profile metadata correlation, not independent byte-to-source provenance. Seal source label: `844a63adb95c7111e1286fa668e632f7cd237806`; source content was not authenticated. Saved toolchain: cargo/rustc 1.99.0, x86_64-unknown-linux-gnu (metadata only).

## Scope and procedural note

No runtime, build, delegation, tracked edits, or commits performed. No old binary, library, archive, source tree, or target executable read/hashed. Only the two seal-selected new copies received byte authentication. External dependencies/cache/settings, source provenance, and runtime behavior remain unverified.

One attempted second evaluation referenced a helper unavailable across Pyrun evaluations and failed with NameError before any file operation. A self-contained evaluation then authenticated wow-sim once; no hash pass was repeated.
