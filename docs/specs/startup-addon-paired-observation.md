# Startup addon paired observation

Test-only observation in [`tests/startup_perf.rs`](../../tests/startup_perf.rs) pairs each successful canonical addon load result with that same call's outer wall time. This is an observation contract, not a startup optimization; runtime proof is pending.

## What it must do

- [ ] Preserve canonical `new_game_env`, discovery, ordered `load_addon` calls, post-load workarounds, startup events, readiness query (`UIParent ~= nil and PlayerFrame ~= nil and IsLoggedIn()`), original assertions, and strict elapsed `< 20s` threshold. Do not substitute a separate startup path.
- [ ] Start an outer wall clock immediately before each `load_addon` call and stop it immediately on return, before result conversion or row recording. Pair the successful result with that duration; preserve existing load-error behavior.
- [ ] Retain typed rows until the full startup clock stops after startup events. Recording and allocation overhead remain inside canonical elapsed time; subtract nothing. JSON encoding and stderr emission occur only after that clock stops.
- [ ] Emit one JSON row per successful addon load and one full-startup summary to stderr with prefix `[startup-observation]`, before either original readiness or budget assertion, so assertion failures leave observation evidence. Include full-startup total and existing addon-load, post-load-workaround, and startup-event phase durations as integer nanoseconds, plus the original readiness boolean.
- [ ] Each addon row includes discovered identity, returned addon name, discovery-order ordinal, same-call outer wall nanoseconds, Lua/XML file counts, and eight base timing fields as integer nanoseconds: `io`, `xml_parse`, `xml_process`, `lua_compile`, `lua_call`, `lua_exec`, `savedvars`, `total`. Include all six cache counters (`cache_hits`, `cache_misses`, `cache_lookup_misses`, `cache_replay_failures`, `cache_store_successes`, `cache_store_failures`) and counts of `warnings`, `nil_symbol_observations`, and `missing_requirements`; emit no diagnostic text.

## Implementation inventory

- `tests/startup_perf.rs` — canonical startup timing test and intended test-only observation/serialization coverage.
- `src/loader/mod.rs` — existing `LoadResult` and `LoadTiming` inputs; unchanged by this contract.

## Tests asserting this spec

- `tests/startup_perf.rs::full_game_startup_stays_under_budget` — existing canonical readiness/budget test; observation requirements not yet proven.
- `tests/startup_perf/observation.rs::tests::addon_observation_roundtrip_preserves_values_and_only_diagnostic_counts` — concrete distinct durations, counters and identities; asserts decoded JSON values, integer nanosecond units and diagnostic counts without text. Execution proof pending.

## Known gaps (current cycle)

- [ ] Implement observation and focused concrete serialization coverage.
- [ ] Obtain runtime evidence for canonical ordering, timing boundaries, and emission before failing assertions. No runtime proof yet.

## Out of scope

- Exclusive buckets, pure-body timings, causal attribution, or summing overlapping buckets as elapsed time: loader timing buckets overlap.
- Performance repair, runtime instrumentation, native-client parity, and historical benchmark equivalence: paired observation establishes none of these.
