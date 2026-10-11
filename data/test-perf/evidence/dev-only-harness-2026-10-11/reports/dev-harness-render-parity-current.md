# Dev harness render parity audit

Read-only audit on 2026-10-11. No source files, builds, tests, or generated build outputs were changed or run.

## Inputs and proof limits

- Immutable old generator: `/home/osso/.local/state/wow-ui-sim/verification/test-harness-extraction-current/original-build.rs` (SHA-256 `df6b286152daa948557c17c4abb8776af2aed573c45f1fcb4901fd2b4a8ccde2`).
- Its saved output snapshots: `original-integration_tests.rs` (`d89ff1af720d1c9e02ee8d1bf29f00ef0e93d888ea5455c4f9fac54a8ddf2765`) and `original-prefork_full_ui_cases.rs` (`9f61e8dc544c99f1da8b0dfe415f67602a584640c9c789089c3c857170c78ab1`).
- The requested `newchildbuild.rs` is absent from the snapshot directory and elsewhere under its verification tree. Audited current `crates/test-harness/build.rs` instead; current checkout files are not represented by the requested immutable child-build snapshot.
- The saved *old* generated outputs are not substitutes for new generated outputs. No compiled/generated new output was present or compared. Therefore this audit does **not** claim generated-output or compiled expansion parity.

## Source-level comparison

**Copied discovery/parser/marker/render core: exact match.** Compared function bodies in old `original-build.rs` and current `crates/test-harness/build.rs`: `discover_prefork_full_ui_sources`, recursive insertion and `#[path]` traversal, Rust-source discovery, candidate filtering, duplicate rejection, case sorting, prefork registry rendering, module-path derivation, top-level marker selection, brace-delimited marker parsing, marker-attribute serialization, and every function/signature/parameter validation helper all match byte-for-byte as extracted function blocks.

Integration discovery also matches: immediate `tests/*.rs` only; the same seven excluded filenames (`blizzard_addon_closure_harness.rs`, `blizzard_addon_smoke_targets.rs`, `frame_positions.rs`, `integration.rs`, `mists_addonlist_interactions.rs`, `prefork_full_ui.rs`, `test_keybindings_panels_detail.rs`); same perf-helper discovery/sort; same common/support/token fixture declarations; same sorted module declarations and formatting. Current prefork markers remain parsed only from discovered source files plus sibling directories and explicit `#[path]` descendants, with the same attributes moved onto emitted `Case::new` entries.

**Concrete source-level differences:**

1. Generator ownership moved from root `build.rs` into `crates/test-harness/build.rs`. Its project root is derived lexically as `CARGO_MANIFEST_DIR.parent().and_then(Path::parent)`; with the declared location `crates/test-harness`, this resolves to the checkout root. Both generators then use that root's `tests/` paths. Integration `#[path]` values and prefork source-path comments are consequently rooted at the project checkout, not the helper crate.
2. Each rendered body is now enclosed in an exported function-like macro. `integration_tests!()` expands to the complete former integration-module item body. `prefork_full_ui_cases!()` expands to the former `&[Case::new(...)]` expression. Helper `src/lib.rs` includes the generated macro definitions.
3. Current invocation sites: integration harness macro once in `tests/integration.rs`, once in `tests/prefork_full_ui.rs`; prefork expression macro once in `tests/prefork_full_ui.rs` at the const initializer (line 224). These replace former direct `include!` sites while retaining their surrounding caller cfgs and scope imports. Notably `tests/prefork_full_ui.rs` still explicitly declares `test_keybindings_panels_detail` under `client-retail`; its filename remains excluded from auto-discovery, as before.
4. The helper build script adds `cargo:rerun-if-changed=build.rs` and a project-root `tests` directory watch. Per-source watches in prefork collection remain unchanged. Root production build script no longer runs either generator or emits a `tests` watch; it retains compiler-identity, optional native ICU, and Windows resource work.

## Exact macro envelopes and path spelling

The envelope source makes the intended expansions clear: the integration macro emits the exact former rendered integration item text inside a macro arm; the prefork macro emits the exact former registry expression inside an expression-context macro arm. The render functions themselves are unchanged. This is source inspection, **not evidence that rustc accepted expansion in both contexts** or that emitted bytes match the saved old snapshots.

The path construction uses `PathBuf` joins and `{:?}` formatting as before; `project_root()` is lexical and introduces no canonicalization. With a normal absolute `CARGO_MANIFEST_DIR` ending in `crates/test-harness`, `project_root()/tests` spells the same project-root paths as the former root manifest directory plus `tests`. The audit did not execute either build script under controlled manifest directories, so exact emitted path strings (including relative-path and platform-specific separator cases) remain unproven.

## Verdict

Copied discovery, exclusion, parser, marker-attribute, sorting, and inner rendering logic are source-identical. The integration-item and prefork-expression macro wrappers are structurally appropriate for the three observed call sites; project-root path derivation is lexically consistent with the declared crate location. Actual new generated files and compiled expansions were unavailable, so byte-for-byte generated parity and compile parity remain **unverified**, not passed.