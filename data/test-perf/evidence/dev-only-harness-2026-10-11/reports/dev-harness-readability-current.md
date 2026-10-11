# Readability audit: dev-only harness extraction

**Scope:** Read-only review of `build.rs`, `crates/test-harness/build.rs`, and `crates/test-harness/src/lib.rs` at `256e54ef1`, including the three callsite edits in the preceding extraction/wiring commits. No code, tests, builds, or operational commands run. Repository status had pre-existing untracked `.code-index.db` and `verification/`; left untouched.

## Change attribution

- `256e54ef1` changes only `Cargo.lock`; none of the requested Rust files changed in that commit.
- The three Rust callsite edits were made in `c14ccbc4c` (`build.rs`, helper `build.rs`, `src/lib.rs`). The substantial generator transfer was introduced in `ab95adb95`.
- The helper build script is a near-verbatim relocation of the existing root build-script generator, not newly authored logic. Treat its inherited generator patterns as legacy; observations below focus on extraction-boundary changes and newly added wrapper/macro behavior, not a refactor wish list.

## Findings

[NAMING] `crates/test-harness/src/lib.rs:4` — `integration_tests!()`
  Problem: The doc comment says it “declares the discovered modules in the caller's scope,” while the included generated source and macro expansion details are implicit; the name can suggest it executes tests rather than declares modules.
  Fix: Clarify the macro doc as module declarations (or use a declaration-oriented name if callsite churn is acceptable).

[COMPLEXITY] `crates/test-harness/build.rs` — `module_path_for_test_source()`
  Problem: Caller-resolved module path generation depends on path components, `mod.rs` stem special-casing, and a root-marker assertion; this newly relocated helper contract is subtle and failure context is generic (`expect`/`assert`).
  Fix: Add concise context to the invariant failures and explain the `mod.rs` mapping at the branch; keep the algorithm otherwise unchanged.

[SIDE_EFFECT] `crates/test-harness/build.rs` — `project_root()` / generator entry points
  Problem: The moved generator reads `CARGO_MANIFEST_DIR`, resolves the parent project and performs filesystem reads/writes; helper names like `project_root` and `generate_*` do not reveal those effects. This is inherited implementation behavior, but the new crate boundary makes effects less apparent to maintainers.
  Fix: Prefer effect-revealing names such as `read_project_root` / `write_*_harness` if this code is next changed; not a blocker for the extraction.

## Macro scopes and caller-resolved cfg

- `integration_tests!` expands generated module declarations at the invocation site. Its paths and attributes therefore resolve in the consumer’s module/crate context, not inside the helper crate. The callsite intentionally supplies the context required by generated declarations.
- `prefork_full_ui_cases!` expands the generated expression at the caller, where `Case` and each `crate::...::run` path resolve. This is explicitly documented and matches the generated expression design.
- The helper’s generated `#[cfg(feature = "gui")]` is caller-resolved. That is important: the helper crate itself must not decide whether the consuming test target has GUI enabled. The macro-generated cfg appears attached to the generated perf module declarations; no scope mismatch identified from source inspection.
- `#[macro_export]` makes exported macros available at crate root, while expansion content is emitted in caller context. No macro hygiene issue identified in the three callsite edits.

## Builder effects and error context

- The callsites keep the helper dependency dev-only; production `build.rs` no longer owns the large harness-generation implementation. The generated output remains in `OUT_DIR` and is included from the helper library.
- The moved generator retains `expect("out dir")`, `expect("read tests dir")`, and similar terse errors. These are copied legacy behavior, not regressions introduced by the dependency wiring. New crate-relative root resolution is part of the boundary; the intended root is the consuming package, and the manifest-dir approach preserves that.
- No unrelated builder API redesign or refactor is recommended. No behavior was executed or tested per instruction.

**Audit result:** No substantive readability blocker in the three callsite edits. The helper contains transferred legacy generator code; findings above are limited to newly exposed contracts and error clarity, with the first item as a documentation-level concern. Static inspection only; no test/build proof.