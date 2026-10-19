# ReloadUI

`C_UI.Reload` and legacy `ReloadUI` request a fresh Lua/UI environment after the invoking Lua call returns. The modeled request lives in `src/c_api/c_ui.rs`; environment retirement lives in `src/lua_api/` and is serviced by the environment owner. GUI and Lua IPC use the same operation. See [lifecycle mechanics](../wiki/systems/reload-ui.md).

## What it must do

- [x] Coalesce Lua reload requests without dispatching reload events or destroying Lua inside the invoking call.
- [x] Save declared account and character SavedVariables before retiring the old VM; read back table and scalar values during fresh addon loading. Do not apply the external import-size cap to just-written reload data or re-import stale WTF values over it. Keep `--no-saved-vars` disabled across reload and never write imported WTF files.
- [x] Discard old globals, frames, scripts, timers, animation state, templates and addon-loading state. Release the old Lua VM; rerun the real startup/addon-loading path with the original launch settings.
- [x] Preserve modeled server/session/player/world/party state and process IPC/texture/font resources. Clear GUI frame-ID, input and render caches before drawing the new UI.
- [x] Service reloads requested by Lua IPC, GUI shortcut, slash commands, events and timers only after the active Lua call unwinds; subsequent IPC commands use the new environment and retain print-output isolation.

Exact native event ordering remains unverified pending native evidence; tests must not invent teardown events. Existing simulator startup order is documented in [event system](../event-system.md#startup-event-sequence). Reload world-entry flags must distinguish reload from initial login.

## How it works

- [Reload lifecycle](../wiki/systems/reload-ui.md)
- [Startup and dispatch](../wiki/systems/event-system.md#startup-event-sequence)
- [Addon loading](../addon-loading-pipeline.md)

## Implementation inventory

- `src/c_api/c_ui.rs` — modeled `C_UI.Reload`, also called by Blizzard’s public `ReloadUI` wrapper.
- `src/lua_api/globals/state_backed_queries.rs` — legacy Lua `ReloadUI` registration sharing the request producer.
- `src/lua_api/env.rs` — Lua VM construction and handle retirement.
- `src/bin/wow_sim/startup.rs` — real startup/addon-loading pipeline.
- `src/startup.rs` — startup event dispatch.
- `src/saved_variables.rs` — declared-variable persistence.
- `src/iced_app/update.rs` — GUI lifecycle owner and shortcut.
- `src/iced_app/update_servers.rs` — IPC execution boundary.
- `src/lua_server.rs` — persistent socket transport.
- `src/bin/wow_sim/main.rs`, `src/bin/wow_sim/gui_commands.rs`, `src/lua_errors.rs` — one-shot owner service before output.

## Tests asserting this spec

- `src/iced_app/app_tests.rs` — shortcut sentinel/frame/VM retirement and Lua-requested SavedVariables persistence.
- `src/lua_server/exec_tests.rs` — public Unix Exec request, reload, subsequent fresh environment and print output.
- `src/bin/wow_sim/main.rs`, `src/bin/wow_sim/gui_commands.rs` — dump/screenshot reload observation boundaries.
- `src/bin/wow_sim/startup.rs` — real `--no-addons` startup with declared Blizzard SavedVariables rebuilt through App.

## Known gaps (current cycle)

- [ ] Native teardown ordering remains unconfirmed; no native-order or target/focus-preservation assertion is made.
- [ ] Full Ellesmere acceptance and standalone `wow-cli` executable reruns remain separate from the bounded Unix-socket client proof.

### Bounded retail proof — 2026-10-19

Source `bbc3f5d7c`: 64 library/App/socket/persistence tests and four binary owner/startup tests pass, zero ignored. Historical RED: four original freshness assertions on `f254e22ee`; dump/screenshot owner assertions on `6987f6145`; scalar/import-cap assertion on `5703ecc56`. Private receipts are named `red-{lib,startup}-receipt.json`, `owner-red-{lib,startup}-receipt.json`, `persistence-red2-lib-receipt.json`, and `green-final-{lib,startup}-receipt.json`.

| Capability | Behavioral evidence | Limit |
| --- | --- | --- |
| Deferred request and IPC isolation | Invoking chunk returns; already-queued Exec and real subsequent socket connection see fresh globals; print capture stays isolated | Acknowledgement is not startup-completion notification |
| VM/UI retirement | Old VM weak handle expires; old globals/frames/timer callbacks disappear; timer-originated request rebuilds | Native teardown sequence unconfirmed |
| Persistence | Declared Blizzard table survives real addon re-registration; account/character scalars and larger local tables survive; stale WTF does not win or change | External import cap remains intact |
| Startup/output owners | Real cached Blizzard startup loads twice; dump, screenshot preparation and lua-errors observe fresh environment | No full-addon acceptance claim |
| Preservation | Existing yielded startup/texture tests pass; modeled level, zone, party count and font/texture allocations survive | No native session-parity claim |

Both GREEN runtimes use child-only XDG directories with copied current CVars; original CVar hash/stat remain unchanged. `cargo fmt --check` passes. Full-default retail `cargo check` is pending capacity.

## Out of scope

- Ellesmere runner or addon changes: the observed failure is simulator lifecycle state.
- Restarting the process, IPC server or modeled game session: only the UI/Lua environment reloads.
- Unconfirmed target/focus/native session-preservation claims: retain current model state without asserting native equivalence.
