# Prefork Test Harness

Linux-only custom test-runner core for reusing immutable parent-owned test state across isolated child cases without entering libtest worker threads.

## Content

`tests/common/prefork.rs` accepts a borrowed parent state and registered `fn(&S)` cases. Selection happens before execution. Each selected case forks into its own process, while the parent schedules at most the configured bounded worker count.

Every fork is immediately preceded by a `/proc/self/task` count check requiring exactly one task. Each child creates its own process group, catches panic payloads into a status channel, and terminates with `_exit`. The parent classifies structured panic status, signals, unexpected exits, and timeouts.

Captured mode redirects each child's stdout and stderr into parent-drained pipes. Successful output stays hidden; failed output is printed with stream labels. `--nocapture` leaves both streams inherited.

Timeout handling signals the whole child process group with `SIGTERM`, waits the configured grace period, escalates to `SIGKILL`, and reaps the direct child. Worker selection honors `--test-threads` before `RUST_TEST_THREADS`, with both capped by the hard maximum.

`tests/prefork_full_ui.rs` currently preloads only lightweight conformance state. Its default execution proves runner semantics before the future full-UI preload and real `WowLuaEnv` case migration.

## Sources

- [Prefork test harness spec](../../specs/prefork-test-harness.md) — behavioral contract and current gaps
- [Conformance target](../../../tests/prefork_full_ui.rs) — behavioral proof and fixture processes
- [Reusable runner](../../../tests/common/prefork.rs) — current implementation

## See Also

- [[blizzard-ui-test-lanes]] — existing Blizzard UI test organization
- [[development-phases]] — broader test and performance work
