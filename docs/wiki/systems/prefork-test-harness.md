# Prefork Test Harness

Linux-only custom test-runner core for reusing immutable parent-owned test state across isolated child cases without entering libtest worker threads. Migrated cases use explicit `prefork_full_ui_case!` marker bodies and a build-generated stable `<module>::<function>` registry.

## Content

`tests/common/prefork.rs` accepts a borrowed parent state and registered `fn(&S)` cases. Its lazy-state entry point parses selection before invoking setup: `--list` returns listing data, and zero selected cases return a successful zero-test result, without conformance or expensive preload. Each selected case forks into its own process, while the parent schedules at most the configured bounded worker count.

Every fork is immediately preceded by a `/proc/self/task` count check requiring exactly one task. The child establishes its process group, reports setup status over a socket, and waits for the parent to verify `getpgid(child) == child` before the test body is released or the case counts as scheduled. Setup failure terminates and reaps the direct child without allowing test code to run.

Parent-owned pipes and setup sockets use `OwnedFd`, so partial construction or setup failure closes every acquired descriptor through ownership. If any runner operation fails after cases have started, the parent sends `SIGKILL` to every active process group and direct child, reaps every direct child, then drops all remaining status/capture descriptors before returning the error.

Each child catches panic payloads into a status channel and terminates with `_exit`. `Config` carries one `fn()` child setup hook, defaulting to a no-op; the child invokes it after process-group establishment and before the registered case body inside the same panic boundary. The parent classifies structured panic status, signals, unexpected exits, and timeouts. Captured mode redirects stdout and stderr into parent-drained pipes; successful output stays hidden, failed output is printed with stream labels, and `--nocapture` leaves both streams inherited.

The Lua bytecode cache has three process-local modes. Production starts writable. The dedicated prefork parent enters `ParentBypass` before constructing `WowLuaEnv`, so cache lookups return misses without reading `pack.bin`, source compilation continues, and stores are skipped as non-failures. When caching is enabled, the parent seals the cache as empty and initialized after startup while recording whether the pack exists; disabled caching remains a successful no-op. Forked children then transition that inherited state to read-only, so they cannot reload the pack or mutate invalid/oversized removal, torn-pack truncation, standalone legacy migration, legacy-key promotion, append, replacement/compaction, temporary-file creation, rename, or cleanup paths.

Timeout handling remains separate: it signals the whole child process group with `SIGTERM`, waits the configured grace period, escalates to `SIGKILL`, and reaps the direct child. Worker selection honors `--test-threads` before `RUST_TEST_THREADS`, with both capped by the hard maximum.

`tests/prefork_full_ui.rs` runs runner conformance in a fresh subprocess before expensive setup during ordinary execution; successful conformance output stays hidden, while failure output is printed. Driver and process-tree fixture modes bypass that orchestration, and `--list` bypasses both conformance and preload.

The target enters parent-bypass mode, then builds one 1024x768 default-retail game-screen `WowLuaEnv`: synced Blizzard UI path, stopped GC, source-compiled dependency-ordered eager addon discovery/loading, one `ADDON_LOADED` per successful addon, post-`Blizzard_EnvironmentCleanup` global restoration, string-metatable sync, post-load workarounds, bootstrap GC restart, and normal game startup events. It rejects partial setup with addon context when loading, addon events, EnvironmentCleanup restoration, startup Lua, GC restart, or cache sealing fails.

`build.rs` parses explicit `prefork_full_ui_case! { fn name(env: &WowLuaEnv) { ... } }` items with `syn` and renders one registry with `quote`. Registry names are stable `<module>::<function>` paths. The prefork target includes the generated integration module tree, so mixed modules keep unmarked tests under libtest while marked bodies are registered only in the prefork runner. The nested fixture `prefork_full_ui_nested::fixture::preloaded_parent_has_normal_game_startup` proves both nested registry resolution and observable inherited Game startup state.

After startup succeeds, enabled-cache sealing verifies that cache state was not initialized or populated, marks that state initialized with empty values/index, and records `pack.bin` existence without reading its contents. Disabled caching skips sealing as a successful no-op. The immutable parent snapshot backs each registered case, one child per case, with 120-second timeouts and read-only bytecode-cache child setup. The initial nine-case benchmark below predates the registry expansion.

The generated ordinary full-UI migration aggregate is now 1,936 cases: 1,780 previously migrated cases plus 156 newly migrated post-start LoadOnDemand cases across 17 modules. These cases use borrowed-environment child setup that performs only the original explicit addon loads after the shared normal-retail preload, preserving dependency order; they do not alter the parent preload. `Blizzard_SharedMapDataProviders` remains excluded: its nine-case fixture loads before post-load workarounds and omits startup events, so it is not equivalent to the finalized shared snapshot. Two PTR-only GuildBank/ItemUpgrade tests were restored to ordinary libtest with profile-specific full startup rather than using the retail prefork snapshot. GenericTraitUI's listing includes three previously migrated cases, so generic-batch registry conformance covers 159 listed cases while the unique new migration is 156. The conformance case passed 1/1, the `is_addon_loaded` filter passed 135/135, GenericTraitUI publication passed 4/4, each restored PTR test passed 1/1, and no orphan processes remained. The nine manual keybinding cases, two behavioral-messaging cases, one nested registry/preloaded-startup fixture, and earlier SpellSearch/explicit/housing batches remain separately categorized. Pre-start, partial/custom, glue, and otherwise non-equivalent lifecycle fixtures remain excluded from the normal-preload migration.

Warm-cache conformance remains in a fresh subprocess with an isolated XDG cache root: it prewarms `pack.bin`, releases non-empty memory, compiles a unique child chunk without mutation, and proves the parent remains writable. Separate parent-bypass conformance reuses an existing pack, compiles unique parent and child chunks, requires a zero-byte seal result, and compares cache-tree bytes and metadata before and after both phases.

## Measured result

The committed parent-bypass target passed the initial nine migrated cases with one worker in 10.12 seconds, down 56.9% from the retained 23.49-second pre-migration serial baseline. `/usr/bin/time` process maximum RSS fell from 1,190,600 KiB to 788,236 KiB (33.8%), and sampled process-tree PSS fell from 1,189,511 KiB to 1,040,276 KiB (12.5%). Parent-only peak PSS was 768,995 KiB; the one-child phase produced the whole-tree PSS peak.

Sampled process-tree RSS rose from 1,196,596 KiB to 1,523,432 KiB. This metric double-counts copy-on-write pages mapped by both parent and child; PSS is the aggregate host-footprint comparison because it apportions shared pages. The benchmark used the same nine-case filter and `--test-threads=1`, with `/usr/bin/time -v` plus 20 ms `/proc/*/smaps_rollup` sampling.

## Sources

- [Prefork test harness spec](../../specs/prefork-test-harness.md) — behavioral contract and current gaps
- [Conformance target](../../../tests/prefork_full_ui.rs) — behavioral proof and fixture processes
- [Reusable runner](../../../tests/common/prefork.rs) — current implementation
- [Build registry](../../../build.rs) — `syn`/`quote` marker parsing and stable case generation
- [Behavioral-messaging cases](../../../tests/blizzard_behavioral_messaging_loads.rs) — two marker-defined cases with remaining libtest tests
- [Nested registry fixture](../../../tests/prefork_full_ui_nested/fixture.rs) — nested path and inherited-startup proof

## See Also

- [[blizzard-ui-test-lanes]] — existing Blizzard UI test organization
- [[development-phases]] — broader test and performance work
- [[bytecode-cache-growth]] — cache size limits and persistence boundaries
