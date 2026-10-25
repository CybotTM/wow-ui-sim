# Compile bin selection and lockfile watches

Rebased and source-inspected on 2026-10-25 against `b2ca0becd60a3e3630da8d74133ac1851bab212f`.
This change is static-only; compilation and CPU-time savings are unverified.

## Integration binaries

The manifest declares four bins and auto-discovers `bench_talents` and
`panel-visual-metrics`. Test executable consumers use only `wow-sim`.
Self-executing tests use their own test executable; Mists script guards create
fake metrics executables rather than requiring the production metrics bin.

The dependency-free opt-in `tools` feature gates all five unused binaries.
The two existing GUI-gated benches retain their GUI requirement. The formerly
auto-discovered targets now have explicit declarations so the gate applies.
`wow-sim`, integration selection, and all full-suite stages stay unchanged.
Release aliases always enable `tools`, including with custom release features;
the native build helper adds it only for non-test `wow-cli` selection.
Checked-in utility build scripts enable it explicitly. Direct Cargo utility
commands now need `--features tools`; default `cargo build` no longer builds
utilities. Docker still requests only headless `wow-sim`. `deploy.sh` only
installs the full-suite Python controller and does not compile simulator bins.

## Lockfile after the data-crate merge

Master already includes Cargo normalization in `91a4e437e`. Two offline
`/usr/bin/cargo metadata --format-version 1` runs with Cargo 1.99.0 leave the
lock bytes unchanged. All 781 sorted complete package records and non-package
fields match the new master exactly, including `sim-data` and `sim-data-profile`.
The stale 779-package lock commit is dropped; no additional normalization or
lock-related CPU saving is claimed. The compiler identity watch remains intact.

Current utility commands include `--features tools`; historical commands remain
unchanged with a [replay note](../../../README.md#utility-binary-builds).

## Sources

- [Build contract](../../specs/integration-bin-selection.md)
- [Compiler cache identity contract](../../specs/compiler-bytecode-cache.md)
- [Runner policy](../../remote-builds.md)
- `Cargo.toml`, `tests/`, `xtask/src/main.rs`, `scripts/build-host.py`, `Dockerfile`, `deploy.sh`, `tools/full_suite.py`

## See Also

- [[bytecode-cache-growth]] — resolved compiler identity is part of persistent bytecode compatibility.
