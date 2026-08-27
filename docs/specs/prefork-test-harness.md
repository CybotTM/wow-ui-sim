# Prefork test harness

The Linux prefork test harness provides a reusable custom test-runner contract for expensive parent-owned test state. Its source lives in `tests/common/prefork.rs`; implementation details are documented at [the prefork harness wiki page](../wiki/systems/prefork-test-harness.md).

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

- [x] Enter process-local parent-bypass bytecode-cache mode before building one parent-owned 1024x768 `ScreenKind::Game` `WowLuaEnv` from the synced default-retail Blizzard UI cache.
- [x] Stop GC, compile Blizzard Lua from source without reading or writing the bytecode pack, discover the normal game-screen Blizzard addon set, load it in dependency order without SavedVariables or third-party addons, and fire `ADDON_LOADED` after every successful load; disabled bytecode caching remains disabled.
- [x] Restore post-cleanup globals after `Blizzard_EnvironmentCleanup`, sync the string metatable, apply post-load workarounds, restart bootstrap GC, and run the normal game startup event sequence.
- [x] After successful startup, when bytecode caching is enabled, seal the bypassed cache as empty and initialized so read-only children cannot reload the disk pack; preserve whether `pack.bin` exists, return a successful zero-byte release outcome, and fail if cache state was initialized or populated before sealing. Disabled caching returns a successful no-op.
- [x] Fail setup explicitly with addon context on addon-load, `ADDON_LOADED`, EnvironmentCleanup restoration, startup Lua, bootstrap-GC, or bytecode-cache sealing errors instead of continuing with a partial parent snapshot.
- [x] Run registered full-UI cases as immutable `fn(&WowLuaEnv)` children with a 120-second child timeout and read-only bytecode-cache child setup.
- [x] Define migrated cases with the explicit `prefork_full_ui_case!` marker; `build.rs` parses marker items with `syn`, renders the registry with `quote`, and assigns stable `<module>::<function>` names.
- [x] Include the generated integration module tree in the prefork target so mixed modules compile once while unmarked tests remain under libtest.
- [x] Prove nested-module discovery and inherited startup state with `prefork_full_ui_nested::fixture::preloaded_parent_has_normal_game_startup`.
- [x] Register the generated default-retail full-UI registry with stable `<module>::<function>` names and retain 10 manual/nested prefork cases.

### Final coverage and eligibility

- [x] List 1,946 cases in the dedicated default-retail prefork target: 1,936 migrated full-environment cases and 10 manual/nested prefork cases.
- [x] Audit all 309 remaining ordinary startup-like tests and confirm zero eligible cases remain for the finalized shared parent preload.
- [x] Classify exclusions exactly: 75 pre-start custom fixtures; 9 non-equivalent lifecycle fixtures; 211 partial/custom/glue/render/thread-sensitive fixtures; 5 owned-timeout fixtures; 7 profile-specific fixtures; 1 post-drop global-state fixture; and 1 version-specific fixture.
- [x] Preserve exclusion rationale: the finalized parent is incompatible with the 75 pre-start and 9 lifecycle cases, while the 211 setup-family cases use partial/custom, alternate-screen, render-sensitive, or thread-sensitive state that is not normal-retail startup.

### Bytecode-cache child contract

- [x] Keep the Lua bytecode cache writable by default in production; use process-local parent-bypass mode only for the dedicated prefork preload.
- [x] In parent-bypass mode, return cache misses without loading `pack.bin`, compile source normally, and treat suppressed stores as non-failures; disabled caching remains a successful no-op.
- [x] Allow a forked child setup hook to transition the inherited bypassed state into one-way process-local read-only mode without changing parent state.
- [x] Preserve current-pack and legacy cache hits while suppressing legacy promotion and on-disk migration.
- [x] Compile cache misses normally without counting suppressed cache stores as failures.
- [x] In read-only mode, never create, append, replace, compact, truncate, remove, rename, or stage temporary bytecode-cache files.
- [x] Preserve warm-cache conformance in a fresh subprocess with an isolated `XDG_CACHE_HOME`: parent prewarm creates `pack.bin`, releases non-empty in-memory pack state, a unique child Lua chunk compiles, the prewarmed cache tree bytes and metadata remain identical, and the parent remains writable.
- [x] Prove parent bypass in a separate fresh process against an existing pack: parent source and child source compile, sealing reports zero loaded bytes, and neither phase changes cache bytes, metadata, or directory contents.

### Performance proof

- [x] Preserve the initial nine migrated behaviors with parent bypass enabled.
- [x] Reduce serial wall time from 23.49 seconds to 10.12 seconds (56.9%).
- [x] Reduce `/usr/bin/time` process maximum RSS from 1,190,600 KiB to 788,236 KiB (33.8%) and sampled process-tree PSS from 1,189,511 KiB to 1,040,276 KiB (12.5%).
- [x] Record sampled process-tree RSS separately: it rises from 1,196,596 KiB to 1,523,432 KiB because RSS counts shared copy-on-write pages in both parent and child, while PSS apportions them.

### Failure handling

- [x] Catch panics and report panic text as structured failure data.
- [x] Distinguish panic, signal, unexpected exit, and timeout failures.
- [x] Establish and parent-verify each child process group before releasing the child test body or scheduling another case.
- [x] On timeout, send `SIGTERM`, wait a bounded grace period, send `SIGKILL`, reap the child, and remove its process tree.
- [x] On parent-side runner failure, terminate every active process group, reap every direct child, and close every owned descriptor before returning.
- [x] Exit unsuccessfully when argument validation, runner operation, or any selected case fails.

## How it works

- [Prefork test harness system](../wiki/systems/prefork-test-harness.md)
- `prefork_full_ui_case!` marker bodies are the single implementation of migrated cases; they are not also emitted as ordinary `#[test]` functions.

## Implementation inventory

- `Cargo.toml` — declares the dedicated Linux prefork conformance target contract.
- `build.rs` — keeps the custom target root out of the generated integration harness and builds the stable marker registry with `syn`/`quote`.
- `tests/common/prefork.rs` — reusable eager and lazy-state test-only runner APIs and execution behavior.
- `tests/common/prefork_full_ui_preload.rs` — target-only normal retail game-screen preload and migrated-case helpers.
- `tests/prefork_full_ui.rs` — custom target entry point, real-case registry, fixture cases, and behavioral conformance checks.
- `tests/test_keybindings_panels_detail.rs` — nine manually registered immutable-environment case bodies.
- `tests/blizzard_behavioral_messaging_loads.rs` — two marker-defined immutable-environment case bodies; its lightweight tests remain under libtest.
- `tests/prefork_full_ui_nested/fixture.rs` — nested-module registry and preloaded-startup behavior fixture.
- `src/loader/bytecode_cache.rs` — writable, parent-bypass, and read-only process modes; mutation-boundary enforcement; and empty-state sealing before child forks.
- `src/loader/mod.rs` — narrow doc-hidden prefork cache entry points.

## Tests asserting this spec

- `tests/prefork_full_ui.rs`

## Known gaps (current cycle)

- [x] Migrate the initial nine `test_keybindings_panels_detail` `WowLuaEnv` cases onto the reusable runner.
- [x] Add the two behavioral-messaging cases and one nested registry/preloaded-startup fixture.
- [x] Benchmark the parent-bypass prefork execution against the retained current in-target baseline.
- [x] Migrate every eligible normal-retail full-environment test and complete the final 309-test ordinary startup-like eligibility audit.

## Out of scope

- Non-Linux prefork support, because the contract depends on Linux process and `/proc` semantics.
- General runtime read-only behavior beyond Lua bytecode-cache mutation suppression in explicitly configured forked test children.
- Compatibility fallbacks for unsupported custom-runner arguments.
