# Generated data crates

Generated Rust catalogs live under `crates/sim-data` and `crates/sim-data-profile`. The root library preserves existing catalog API paths. See [design](../wiki/design/generated-data-crates.md).

## What it must do

- [ ] Preserve generated catalog bytes and observable lookup behavior.
- [ ] Compile common catalogs independently of client selection.
- [ ] Preserve atlas, global-string and aura attribute selection through forwarded features.
- [ ] Preserve root exactly-one-client validation.
- [ ] Generators and Docker builds use the moved files.

## How it works

- [Crate boundary](../wiki/design/generated-data-crates.md)

## Implementation inventory

- `crates/sim-data/`: common catalogs and profession item overrides.
- `crates/sim-data-profile/`: profile-selected catalogs.
- `src/lib.rs`, `src/c_api/c_secrets.rs`: compatibility re-exports.
- `Cargo.toml`, `Dockerfile`: dependency and build context wiring.
- `tools/gen*.py`, `src/bin/wow_cli/gen_*.rs`: generator paths.

## Tests asserting this spec

Existing integration catalog tests and data-crate unit tests are preservation coverage; this extraction introduces no new runtime behavior. Build timings and feature-tree evidence are stored in the crate-split handoff directory.

## Known gaps (current cycle)

- [ ] Same-slot baseline/branch and touched-source rebuild timings, RSS, Retail/Wrath checks and targeted tests remain pending.

## Out of scope

Other crate cuts, test-harness restructuring, data regeneration, pushes and runtime changes.
