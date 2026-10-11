# Dev-harness dependency scope — source audit

**Audited tree:** `/home/osso/Projects/wow/wow-ui-sim`, requested revision `256e54ef1` (working-tree revision was not independently checked). **Method:** read-only inspection of manifests and named source files; no commands, builds, tests, shell, or delegation. Findings describe source wiring only, not build/cache behavior.

## Findings

- `Cargo.toml` makes `wow-ui-sim-test-harness` a root **dev-dependency**, while also listing `crates/test-harness` as a workspace member. Its manifest declares `quote` and `syn` as **build-dependencies**. This keeps those parsing/code-generation dependencies out of the root package's production build-script graph by manifest classification. Cargo lockfile records resolution, not by itself production reachability; no assertion is made here about lockfile changes or resolved-graph behavior at a particular invocation.
- Root `build.rs` has no test-harness import or test-source traversal. It reads the lockfile to publish the pinned rilua compiler revision, optionally calls `build/intl_native.rs` under `native-duration-formatting`, and handles the Windows icon resource. These are the observed root build-script responsibilities; no test source consumption found in the inspected file.
- The child crate's `build.rs` is the test-discovery/generation path. It declares `cargo:rerun-if-changed` for its own `build.rs` and the project `tests` directory, then discovers top-level integration modules, perf helper modules, and prefork full-UI cases, reading/parsing test sources and emitting generated files in `OUT_DIR`. The prefork scan follows sibling Rust files and explicit `#[path]` modules; discovered prefork sources receive individual rerun directives. This preserves automatic child-source watching and new-module discovery.
- `crates/test-harness/src/lib.rs` includes those generated artifacts. `tests/integration.rs` invokes `wow_ui_sim_test_harness::integration_tests!()`. `tests/prefork_full_ui.rs` invokes `prefork_full_ui_cases!()` for generated fixture registration. These are test-target consumers, not observed production-code consumers.

## Remaining production-code test-source watch/consumption

**None found in the inspected root `build.rs` or manifest wiring.** Test-source discovery and watching reside in the dev-only harness crate's build script. This is a bounded source audit, not a proof over every file/revision: broad repository-wide searching was not performed because the request prohibited command/search execution. Specifically, no claim about production incremental-cache cleanliness or a GREEN cache result can be inferred from these source observations.

## Preserve

Keep the harness as a root dev-dependency; keep discovery and `rerun-if-changed` declarations in `crates/test-harness/build.rs`. Its scans preserve automatic discovery, including nested/path-declared prefork modules. Avoid moving this scanner or its test-source watch declarations into root `build.rs`.
