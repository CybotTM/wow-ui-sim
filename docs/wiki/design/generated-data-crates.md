# Generated data crate boundary

The [contract](../../specs/generated-data-crates.md) separates generated production catalogs from simulator code.

## Boundary

`sim-data` has no client features and depends only on phf. Common tables therefore share a Cargo unit across profiles when toolchain, compilation flags and dependencies match. Items and profession overrides stay together because their lookups refer to each other.

`sim-data-profile` contains atlas variants, global-string variants and the aura attribute table. Root feature bundles forward only the predicates used by these tables. Original cfg expressions remain on the selected modules. Root `client_profile` still rejects missing or conflicting profiles; leaf crates do not choose a runtime profile. Splitting the selected tables prevents forwarded features from invalidating the large common crate.

Root re-exports preserve existing paths and type identity. Generators write to each crate’s `data/` directory; non-Rust source data and provenance stay in root `data/`. Docker copies both crates before Cargo dependency resolution. No catalog contents are regenerated.

## Proof status

Static extraction staged on 2026-10-24. Runtime preservation and timing measurements are pending a released build slot. No speedup is claimed from line counts.

## Sources

- [Contract](../../specs/generated-data-crates.md)
- [Workspace manifest](../../../Cargo.toml)
- [Root facade](../../../src/lib.rs)

## See Also

- [[client-profiles]] — runtime profile and epoch selection.
