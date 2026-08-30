# Test Runtime Optimization Boundaries

Commit `cfa2e2e6d7ab8ab7f75d9ff34bd22b69122376f3` narrowed `Blizzard_SharedMapDataProviders` tests from a full Blizzard game load to an exact dependency-closure fixture. The measured bottleneck is repeated `WowLuaEnv` and addon setup, not timeout polling, process launch, prefork scheduling, or libtest target topology. Exact closure fixtures are safe for ordinary addon surface/load tests when full-game ordering is not the behavior under test; mutable environments must remain isolated per test.

## Content

### Baseline and scope

Before `cfa2e2e6d`, the 18-test `blizzard_shared_map_data_providers_loads` module ran its own full game-screen Blizzard load for the relevant tests. The controlled sequential baseline was:

- parent revision: `6ca841f38d64b606511ba9abe67cc10ca502a979`
- command: integration binary filtered to `blizzard_shared_map_data_providers_loads::`, `--nocapture`, `--test-threads=1`
- result: 18 passed in 20.10 seconds; 20.14 seconds elapsed
- resource sample: maximum RSS 1,246,460 KiB; 19.06 user seconds and 0.90 system seconds
- artifact: `/tmp/pi-shared-map-baseline-sequential.result.json`

Post-change runtime numbers remain pending. The available post-commit inspection records the source diff and clean inspection, but does not provide verifier timing evidence.

### Safe optimization: exact dependency closure

`cfa2e2e6d` replaces the module's full-game loader with
`build_blizzard_addon_closure_env(&ui_dir, &["Blizzard_SharedMapDataProviders"], &[])` and uses the shared helpers in `tests/common/blizzard_addon_harness.rs`:

- `new_blizzard_addon_env` creates a fresh game-screen environment;
- `build_blizzard_addon_closure_env` discovers and loads the root's transitive TOC dependencies in order;
- `load_blizzard_addon_closure_into_env` applies the same closure to an existing fresh environment.

This is the correct fixture for addon API, TOC, registration, and load-state assertions that do not require the complete game startup sequence. It preserves dependency order without paying for unrelated Blizzard addons. It is not a replacement for full-startup tests: `Blizzard_SharedMapDataProviders` remains ineligible for the finalized prefork parent because its fixture loads before post-load workarounds and omits startup events.

### Rejected alternatives

| Candidate | Finding | Decision |
|---|---|---|
| Timeout re-exec polling | `tests/common/timeout_reexec.rs` polls `Child::try_wait()` every 10 ms. Focused timeout-reexec coverage passed eight tests in about 1.07 seconds; polling contributes at most one interval per normal child. | Keep the 10 ms poll. It is negligible beside addon setup and is required for timeout and process-tree cleanup semantics. |
| Timeout child launch, pipes, and handshake | Exact named launch, two drain threads, and the handshake add sub-second overhead, while startup traces show about 0.25 s Lua initialization, 2.8–3.2 s addon loading, and about 0.27 s post-load layout per child. | Do not change the wrapper. It preserves panic aggregation, sibling continuation, output forwarding, and timeout isolation. |
| Prefork worker tuning | Prefork runs are already roughly 6–10 seconds for representative slices. More workers overlap child setup but do not remove it and increase copy-on-write process-tree memory. Persistent workers would also violate mutable-environment isolation. | Keep bounded workers and one isolated child per case; optimize the fixture graph instead. |
| Libtest wrappers | `test_timeout!`/`with_timeout` isolate ordinary tests by exact re-exec, but each ordinary test still constructs its own environment and loads its selected addon graph. | Retain wrappers for failure and timeout semantics; do not treat them as a startup optimization. |
| Grouped integration targets or target splitting | `build.rs` generates one integration harness from roughly 1,077 top-level test modules. Splitting it would duplicate simulator/test dependency compilation without removing per-test full-UI setup. | Do not split targets for this problem. Any future grouped target needs measured closure and compile evidence first. |

### Isolation boundary

Never share a mutable `WowLuaEnv` across tests. Tests mutate Lua globals, frame trees, addon-loaded state, registries, SavedVariables, and seeded domain state. The safe optimization narrows each test's dependency closure while still creating a fresh environment per test. Reusing a live environment or persistent worker would make sibling order and state leakage part of the test contract.

## Sources

- [narrowed SharedMapDataProviders test](../../../tests/blizzard_shared_map_data_providers_loads.rs) — commit `cfa2e2e6d` closure fixture and retained assertions
- [Blizzard addon closure harness](../../../tests/common/blizzard_addon_harness.rs) — fresh environments and dependency-ordered closure loading
- [timeout re-exec implementation](../../../tests/common/timeout_reexec.rs) — exact child launch, polling, output drains, handshake, and cleanup
- [timeout re-exec tests](../../../tests/timeout_reexec.rs) — panic, timeout, sibling, descendant, and nested-guard proofs
- [prefork runner](../../../tests/common/prefork.rs) — bounded workers and isolated child execution
- [generated integration harness](../../../build.rs) — top-level module discovery and target generation
- [Blizzard UI test lanes](../reference/blizzard-ui-test-lanes.md) — unit versus addon-bootstrap test intent
- [Prefork test harness](../systems/prefork-test-harness.md) — full-UI preload contract and eligibility boundary
- `/tmp/pi-shared-map-baseline-sequential.result.json` — controlled pre-change baseline

## See Also

- [[prefork-test-harness]] — immutable full-UI parent, child isolation, and finalized eligibility
- [[blizzard-ui-test-lanes]] — test intent and startup-shape boundaries
- [[development-phases]] — broader performance-regression roadmap
