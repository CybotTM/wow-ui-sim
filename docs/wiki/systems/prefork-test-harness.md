# Prefork Test Harness

Linux-only custom test-runner core for reusing immutable parent-owned test state across isolated child cases without entering libtest worker threads.

## Content

`tests/common/prefork.rs` accepts a borrowed parent state and registered `fn(&S)` cases. Its lazy-state entry point parses selection before invoking setup: `--list` returns listing data, and zero selected cases return a successful zero-test result, without conformance or expensive preload. Each selected case forks into its own process, while the parent schedules at most the configured bounded worker count.

Every fork is immediately preceded by a `/proc/self/task` count check requiring exactly one task. The child establishes its process group, reports setup status over a socket, and waits for the parent to verify `getpgid(child) == child` before the test body is released or the case counts as scheduled. Setup failure terminates and reaps the direct child without allowing test code to run.

Parent-owned pipes and setup sockets use `OwnedFd`, so partial construction or setup failure closes every acquired descriptor through ownership. If any runner operation fails after cases have started, the parent sends `SIGKILL` to every active process group and direct child, reaps every direct child, then drops all remaining status/capture descriptors before returning the error.

Each child catches panic payloads into a status channel and terminates with `_exit`. `Config` carries one `fn()` child setup hook, defaulting to a no-op; the child invokes it after process-group establishment and before the registered case body inside the same panic boundary. The parent classifies structured panic status, signals, unexpected exits, and timeouts. Captured mode redirects stdout and stderr into parent-drained pipes; successful output stays hidden, failed output is printed with stream labels, and `--nocapture` leaves both streams inherited.

Prefork children can explicitly enter process-local read-only Lua bytecode-cache mode through the runner hook. Cache reads, current hits, legacy hits, and source compilation continue, but all on-disk mutation paths are suppressed: invalid or oversized removal, torn-pack truncation, standalone legacy migration, legacy-key promotion, append, replacement/compaction, temporary-file creation, rename, and cleanup removal. Suppressed stores have a distinct non-failure result. Production and the prefork parent remain writable because the one-way mode switch is called only after `fork()` in the child process.

Timeout handling remains separate: it signals the whole child process group with `SIGTERM`, waits the configured grace period, escalates to `SIGKILL`, and reaps the direct child. Worker selection honors `--test-threads` before `RUST_TEST_THREADS`, with both capped by the hard maximum.

`tests/prefork_full_ui.rs` runs runner conformance in a fresh subprocess before expensive setup during ordinary execution; successful conformance output stays hidden, while failure output is printed. Driver and process-tree fixture modes bypass that orchestration, and `--list` bypasses both conformance and preload.

The target then builds one 1024x768 default-retail game-screen `WowLuaEnv`: synced Blizzard UI path, stopped GC, dependency-ordered eager addon discovery/loading, one `ADDON_LOADED` per successful addon, post-`Blizzard_EnvironmentCleanup` global restoration, string-metatable sync, post-load workarounds, bootstrap GC restart, and normal game startup events. It rejects partial setup with addon context when loading, addon events, EnvironmentCleanup restoration, startup Lua, or GC restart fails. The immutable parent snapshot backs nine migrated `test_keybindings_panels_detail` cases, one child per case, with 120-second timeouts and read-only bytecode-cache child setup.

Bytecode conformance still runs in a fresh subprocess with an isolated XDG cache root, prewarms `pack.bin` in the parent, compiles a unique generated addon chunk in a read-only child, compares cache-tree bytes and metadata, and then proves the parent remains writable. Benchmarking remains open.

## Sources

- [Prefork test harness spec](../../specs/prefork-test-harness.md) — behavioral contract and current gaps
- [Conformance target](../../../tests/prefork_full_ui.rs) — behavioral proof and fixture processes
- [Reusable runner](../../../tests/common/prefork.rs) — current implementation

## See Also

- [[blizzard-ui-test-lanes]] — existing Blizzard UI test organization
- [[development-phases]] — broader test and performance work
