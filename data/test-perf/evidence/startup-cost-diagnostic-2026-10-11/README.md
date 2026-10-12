# Startup cost diagnostic — pending execution

Private observational wrapper for the startup benchmark sequence: environment initialization, game-addon discovery and ordered loads, post-load workarounds, startup events. Readiness is queried after clocks stop. No settling ticks, talent workload, acceptance assertion, threshold or simulator/vendor change.

Existing `WOW_SIM_PROFILE_LUA_FILES` records supply compile/call durations and cache counters. Phase clocks exclude snapshots; the enclosing clock includes observer overhead. Counters are cumulative; each phase emits only its new file records. Nested Lua/loader timings are not exclusive costs and must not be summed as exclusive attribution.

Main will compile the wrapper against the previously recorded Retail library, using saved Cargo metadata/path/stat without rereading or hashing old library payloads. This is not hermetic provenance or a frozen-BA artifact. Execution uses a fresh capacity guard, async `agents.slice`, CPU1200%/memory16G, runtime≤90s and whole-control-group termination; no build deadline or Cargo rebuild. Warm-cache/CASC-disabled/profiled observations do not replace unchanged acceptance tests.

Execution and independent observation proof remain pending. Frozen BA retains 15,038 PASS/3 FAIL/19 skips/15,041 executions; full handoff OPEN. Completed collector/Calendar/Bounty and other scopes are not replayed.
