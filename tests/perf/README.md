# Performance Test Support

This directory is the home for performance-focused test support code.

`cargo test` only auto-discovers integration tests from top-level files in
`tests/`, so files under `tests/perf/` are intended to be shared helpers,
fixtures, and notes used by those top-level perf entrypoints.

Planned use:

- shared Blizzard UI loaders for perf-sensitive test cases
- common timing helpers and budget assertions
- scenario builders reused by startup, layout, and rendering perf tests

The first shared helper lives in `tests/perf/mod.rs`:

- `run_game_ui_cases(...)` loads and settles the full Blizzard game UI once,
  then runs multiple named perf cases against that single `WowLuaEnv`

Keep regression-gated perf tests in ordinary `cargo test` workflows rather than
standalone benchmark-only harnesses.
