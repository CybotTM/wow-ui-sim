# Legacy party import gate — abb979c44

**PASS — source, edition-correct format, changed-line readability only. Build/runtime PENDING.**

## Source applicability and visibility

- Complete saved compiler stderr reports E0603 at `src/lua_api/globals/group_queries_relationships.rs:93`: direct traversal through private `temporary` (`src/lua_api/workarounds/mod.rs:6`). Compilation failed before tests; no Speak behavioral RED credit. Log also contains six existing iced manifest deprecation warnings, not changed or suppressed.
- `workarounds/mod.rs:10–11` re-exports only `unit_in_party_is_ai` with `pub(crate)` and `cfg(not(feature = "retail-12-0-0"))`. `temporary` remains private. Its nested `party_ai_legacy` and helper already have crate visibility, so the parent can expose this item without exposing the whole temporary module or a public external-crate API.
- `group_queries_relationships.rs:92–93` uses the same condition and imports the parent alias. Registration remains `group_queries.rs:113`. Modern import at relationships line 91 is unchanged.
- Commit diff changes no helper body or modern producer. The legacy alias denotes the original helper directly: nullable `Option<String>` extraction, false boolean push, one return. Source equivalence, not executed behavioral proof.
- All three scoped files match the requested commit at final inspection. Doc change accurately distinguishes the observed compile failure from pending repaired build/runtime proof. No native or unknown-profile parity claim.

## Format proof — complete stdout/stderr inspected

1. Initial `rustfmt --check <two absolute files>`: exit 1. Default-edition parsing rejects an existing let-chain at relationships line 324; recursive child formatting emits an import-order diff in unchanged `temporary/house_exterior_debug.rs:7`. This is not a changed-line formatting defect.
2. `cargo fmt --check` with explicit repository cwd: exit 0; stdout/stderr empty.
3. `rustfmt --edition 2024 --config skip_children=true --check <two absolute files>`: exit 0; stdout/stderr empty. Edition matches Cargo.toml:4; skip_children limits direct checking to the requested files. No warnings suppressed or source edited.

Full retained command argv/results: `format-output.json` and `proof.json`. First corrected invocation lost its captured result to a Pyrun ctx lookup error; repeated solely because output was unavailable, not to recover a failed check.

## Readability and scope

No changed-line readability violations. Change adds one explicit feature-gated item re-export and replaces one import path; no functions, complexity, mutation, hidden effects, suppression, or duplicate behavior added. Existing function bodies are unchanged; no unrelated readability remediation requested.

No cargo check/test/build, runtime/binary/auth/operations, tracked edits, or commits performed. Main-owned retry compile and runtime acceptance remain pending. This report does not claim the crate compiles or any test passes.
