# Performance Test Support

This directory is the home for performance-focused test support code.

`cargo test` only auto-discovers integration tests from top-level files in
`tests/`, so files under `tests/perf/` are intended to be shared helpers,
fixtures, and notes used by those top-level perf entrypoints.

Planned use:

- shared Blizzard UI loaders for perf-sensitive test cases
- common timing helpers and budget assertions
- scenario builders reused by startup, layout, and rendering perf tests

The shared helpers currently live in:

- `tests/perf/game_ui.rs`
  - `load_timed_game_ui()` measures `WowLuaEnv::new` through Blizzard addon
    load, `apply_post_load_workarounds()`, and the startup event sequence up to
    `UPDATE_CHAT_WINDOWS`
  - returns both the loaded env and per-phase timings so slow startup runs do
    not disappear into an undifferentiated wall-clock total
- `tests/perf/cases.rs`
  - `run_game_ui_cases(...)` loads and settles the full Blizzard game UI once,
    then runs multiple named perf cases against that single `WowLuaEnv`

Keep regression-gated perf tests in ordinary `cargo test` workflows rather than
standalone benchmark-only harnesses.
