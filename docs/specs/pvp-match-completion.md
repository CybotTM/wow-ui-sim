# Supplied PvP match completion (CF-2)

Retail/PTR `C_PvP.IsMatchComplete` must project the existing supplied completion snapshot. The original selected `Blizzard_PVPMatch/PVPMatchResults.lua` entering-world consumer is the behavioral boundary. This is a first slice, not closure of Forever's selected consumer.

## What it must do

- [ ] Publish `C_PvP.IsMatchComplete` for runtime `client_profile::ACTIVE` Retail/PTR, returning exactly `SimState.pvp_catalog.match_completed` without mutation.
- [ ] Preserve the existing `HasMatchStarted` active-queue-or-completed behavior; do not alias completion to activity or queue status.
- [ ] One selector uses the normal XML-created `PVPMatchResults`, retaining its installed original `OnEvent` script and asserting handler identity/provenance. It covers inactive/incomplete → active/incomplete → inactive/complete → inactive/incomplete snapshots, resetting actual registrations between snapshots without replacing scripts or repairing presentation.
- [ ] Assert original active-event subscriptions and unchanged hidden presentation. The selector independently rejects constant true and constant false.
- [ ] Obtain independent Retail/PTR RED at the original completion call before producer edits, then GREEN and complete own-base stock-map comparisons against master571284aaa.

## How it works

- [Event dispatch](../event-system.md)
- [Client profiles](../wiki/systems/client-profiles.md)

## Implementation inventory

- `src/c_api/c_pvp/catalog.rs`: existing completion input and `HasMatchStarted`; completion producer remains absent pending RED.
- `src/c_api/c_pvp.rs`: namespace publication boundary.
- `src/lua_api/state/sim_state.rs`: existing epoch-gated `pvp_catalog` storage.

## Tests asserting this spec

- `tests/cf2_pvp_completion.rs`: directly invokes the normal XML-created frame's installed original event script so failures cannot be swallowed by dispatch; no surrogate frame, manual Mixin or SetScript. Explicit `retail-12-0-0` compile gate matches the existing input's availability; runtime profile assertion admits only Retail/PTR. Not compiled or executed yet.

## Known gaps (current cycle)

- [ ] Slot admission, observed RED, producer, GREEN, constant-producer mutation proofs, zero-new-warning test-target checks and actual stock deltas remain pending.
- [ ] Forever's selected original consumer needs an explicit normal completion input/publication contract; its current epoch has no `pvp_catalog` field. No Forever API or compatibility claim in this slice.

## Out of scope

Match simulation, reward/reset timing policy, Mists vehicle records, speculative Forever enablement, native parity and full-prefork completion. Other startup blockers remain independent.
