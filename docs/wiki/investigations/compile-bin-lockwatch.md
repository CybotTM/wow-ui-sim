# Compile bin selection and lockfile watches

Source-inspected on 2026-10-25 at `50e19162b5d287d994b66dacd37b0ef34a2eb802`.
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

## Sources

- [Build contract](../../specs/integration-bin-selection.md)
- [Compiler cache identity contract](../../specs/compiler-bytecode-cache.md)
- [Runner policy](../../remote-builds.md)
- `Cargo.toml`, `tests/`, `xtask/src/main.rs`, `scripts/build-host.py`, `Dockerfile`, `deploy.sh`, `tools/full_suite.py`

## See Also

- [[bytecode-cache-growth]] — resolved compiler identity is part of persistent bytecode compatibility.
