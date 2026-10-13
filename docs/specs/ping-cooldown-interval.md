# Explicit ping cooldown interval

`C_Ping.GetCooldownInfo()` must project an explicit per-environment interval from `src/c_api/c_ping_cooldown.rs`. Only pure type/zero-state scaffolding is integrated; getter implementation is deferred until genuine RED. Tests target `client-retail`/`client-ptr`; backing state uses their shared `retail-12-0-0` feature floor. Integration is uncompiled/unexecuted.

Accepted draft cached Retail evidence: `Blizzard_APIDocumentationGenerated/PingManagerDocumentation.lua` declares no arguments and nonnil `PingCooldownInfo`, without secret/restriction annotations on this getter. `PingManagerSharedDocumentation.lua` requires numeric nonnil `startTimeMs`/`endTimeMs`. `Blizzard_PingUI/Blizzard_PingUI.lua:127–141,194–205` compares `GetTime()*1000 < endTimeMs` and requests `(endTimeMs/1000)-GetTime()` seconds. Sources are under `~/.cache/wow-ui-sim/blizzard-ui/retail/AddOns`, as recorded by the draft, not freshly fetched or native-observed. Fresh DTO allocation and inactive `(0,0)` are accepted simulator assumptions; PTR/native parity is unverified.

## What it must do

- [ ] Return exactly one fresh nonnil table with numeric `startTimeMs` and `endTimeMs`, projecting explicit values unchanged as absolute milliseconds, not seconds/elapsed/remaining duration.
- [ ] Default each environment to `(0,0)`; explicit replacement/reset changes the next read while older DTOs remain snapshots.
- [ ] Caller mutation cannot alter other DTOs, future reads or backing state; simultaneous environments remain independent.
- [ ] Allow genuinely tainted zero-argument calls; no secret-argument policy from adjacent ping operations may be imported.
- [ ] Actual cached vendor OnEnter selects normal cursor/no timer for zero state at clock 12, error cursor/3.5-second request for `(12000,15500)`, normal cursor/cleared listener after direct callback, and no new timer at equality clock 15.5. Getter state remains unchanged.

## How it works

- [Lua API overview](../lua-api.md).
- [Runtime architecture](../wiki/systems/lua-api.md).

## Implementation inventory

- `src/c_api/c_ping_cooldown.rs`: public `PingCooldownInterval { start_time_ms: f64, end_time_ms: f64 }`; zero default, no producer/clock/timer fields.
- `src/c_api/mod.rs`: model re-export, no getter registration change.
- `src/lua_api/state/sim_state.rs`: public `ping_cooldown_interval` fixture field.
- `src/lua_api/state.rs`: independent zero initialization.

## Tests asserting this spec

[tests/ping_cooldown_interval.rs](../../tests/ping_cooldown_interval.rs): six unexecuted cases for default/tainted reads, supplied milliseconds, fresh DTO/mutation, live replacement/reset, environment isolation, and bounded actual vendor consumer.

Vendor test reads the active profile cache without fallback and executes the unchanged Lua definition file, not XML/startup. Only clock/cursor/timer sinks are controlled; getter and vendor methods remain real. Direct callback delivery proves requested delay and closure effects, not real scheduler behavior. No tooltip operation exists on the chosen path. Active PTR cache routing is an integration adaptation, not PTR proof.

## Known gaps (current cycle)

- [ ] Obtain compiled behavioral RED; missing symbols/cache or load failures are not getter behavioral RED.
- [ ] Add getter producer/registration only after RED, then obtain bounded GREEN.
- [ ] Native default/fresh allocation, output security and PTR consumer equivalence remain unverified.

## Out of scope

Send-ping/charge/refill generation, timer policy, automatic expiry/reset, events/callback dispatch, secret arguments, malformed interval validation, extra fields, full startup, other profiles, vendor/config/WTF edits.
