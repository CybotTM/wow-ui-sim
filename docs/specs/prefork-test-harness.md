# Prefork test harness

The Linux prefork test harness provides a reusable custom test-runner contract for expensive parent-owned test state. Its source lives in `tests/common/prefork.rs`; detailed design documentation may be added at [the prefork harness wiki page](../wiki/systems/prefork-test-harness.md).

## What it must do

### Platform and lifecycle

- [x] Run only on Linux.
- [x] Keep the runner parent single-threaded and reject every fork attempt unless `/proc/self/task` reports exactly one task immediately before the fork.
- [x] Borrow immutable state created by the parent in each selected child without leaking child mutations back to the parent or sibling cases.
- [x] Support one optional `fn()` child setup hook, defaulting to a no-op, after child process-group establishment and before the registered case body under the same panic capture.
- [x] Run one child process for every selected case.
- [x] Bound concurrent children with explicit default and hard-maximum worker counts.
- [x] Honor both `--test-threads` forms and `RUST_TEST_THREADS`, with the command-line value taking precedence.
- [x] Run all runner conformance cases in a fresh subprocess before the normal full-UI preload during ordinary target execution; hide successful conformance output and print it on failure.
- [x] Parse selection before setup so `--list` and zero-match execution run neither conformance nor preload; zero matches report a successful zero-test result, while selected-case setup failure exits explicitly.

### Selection and output

- [x] Support one positional substring filter and exact matching with `--exact`.
- [x] Support repeatable `--skip VALUE` and `--skip=VALUE` exclusions, rejecting empty values.
- [x] Support `--list` with libtest-compatible case and summary data.
- [x] Reject unsupported arguments, duplicate singleton flags/options, and conflicting arguments instead of ignoring them.
- [x] Capture child stdout and stderr by default, suppress successful captured output, and print captured output for failures.
- [x] Inherit child stdout and stderr when `--nocapture` is selected.

### Normal retail full-UI preload

- [x] Build one parent-owned 1024x768 `ScreenKind::Game` `WowLuaEnv` from the synced default-retail Blizzard UI cache.
- [x] Stop GC, discover the normal game-screen Blizzard addon set, load it in dependency order without SavedVariables or third-party addons, and fire `ADDON_LOADED` after every successful load.
- [x] Restore post-cleanup globals after `Blizzard_EnvironmentCleanup`, sync the string metatable, apply post-load workarounds, restart bootstrap GC, and run the normal game startup event sequence.
- [x] Fail setup explicitly with addon context on addon-load, `ADDON_LOADED`, EnvironmentCleanup restoration, startup Lua, or bootstrap-GC errors instead of continuing with a partial parent snapshot.
- [x] Run the nine `test_keybindings_panels_detail` cases as immutable `fn(&WowLuaEnv)` children with a 120-second child timeout and read-only bytecode-cache child setup.

### Bytecode-cache child contract

- [x] Keep the Lua bytecode cache writable by default in production and the prefork parent.
- [x] Allow a forked child setup hook to enter one-way process-local read-only bytecode-cache mode without changing parent state.
- [x] Preserve current-pack and legacy cache hits while suppressing legacy promotion and on-disk migration.
- [x] Compile cache misses normally without counting suppressed cache stores as failures.
- [x] In read-only mode, never create, append, replace, compact, truncate, remove, rename, or stage temporary bytecode-cache files.
- [x] Prove the contract in a fresh subprocess with an isolated `XDG_CACHE_HOME`: parent prewarm creates `pack.bin`, a unique child Lua chunk compiles, and the prewarmed cache tree bytes and metadata remain identical.

### Failure handling

- [x] Catch panics and report panic text as structured failure data.
- [x] Distinguish panic, signal, unexpected exit, and timeout failures.
- [x] Establish and parent-verify each child process group before releasing the child test body or scheduling another case.
- [x] On timeout, send `SIGTERM`, wait a bounded grace period, send `SIGKILL`, reap the child, and remove its process tree.
- [x] On parent-side runner failure, terminate every active process group, reap every direct child, and close every owned descriptor before returning.
- [x] Exit unsuccessfully when argument validation, runner operation, or any selected case fails.

## How it works

- [Prefork test harness system](../wiki/systems/prefork-test-harness.md)

## Implementation inventory

- `Cargo.toml` — declares the dedicated Linux prefork conformance target contract.
- `build.rs` — keeps the custom target root out of the generated integration harness.
- `tests/common/prefork.rs` — reusable eager and lazy-state test-only runner APIs and execution behavior.
- `tests/common/prefork_full_ui_preload.rs` — target-only normal retail game-screen preload and migrated-case helpers.
- `tests/prefork_full_ui.rs` — custom target entry point, real-case registry, fixture cases, and behavioral conformance checks.
- `tests/test_keybindings_panels_detail.rs` — nine migrated immutable-environment case bodies.
- `src/loader/bytecode_cache.rs` — process-local cache mode and mutation-boundary enforcement.
- `src/loader/mod.rs` — narrow doc-hidden child entry point.

## Tests asserting this spec

- `tests/prefork_full_ui.rs`

## Known gaps (current cycle)

- [x] Migrate the nine `test_keybindings_panels_detail` `WowLuaEnv` cases onto the reusable runner.
- [ ] Benchmark prefork execution against the current in-target test execution path.

## Out of scope

- Non-Linux prefork support, because the contract depends on Linux process and `/proc` semantics.
- General runtime read-only behavior beyond Lua bytecode-cache mutation suppression in explicitly configured forked test children.
- Compatibility fallbacks for unsupported custom-runner arguments.
