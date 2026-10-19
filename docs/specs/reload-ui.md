# ReloadUI

`ReloadUI` requests a fresh Lua/UI environment after the invoking Lua call returns. The lifecycle lives in `src/lua_api/` and is serviced by the environment owner; GUI and IPC use the same operation. See [lifecycle mechanics](../wiki/systems/reload-ui.md).

## What it must do

- [ ] Coalesce Lua reload requests without dispatching reload events or destroying Lua inside the invoking call.
- [ ] Save declared account and character SavedVariables before retiring the old VM; read back table and scalar values during fresh addon loading. Do not apply the external import-size cap to just-written reload data or re-import stale WTF values over it. Keep `--no-saved-vars` disabled across reload and never write imported WTF files.
- [ ] Discard old globals, frames, scripts, timers, animation state, templates and addon-loading state. Release the old Lua VM; rerun the real startup/addon-loading path with the original launch settings.
- [ ] Preserve modeled server/session/player/world/party state and process IPC/texture/font resources. Clear GUI frame-ID, input and render caches before drawing the new UI.
- [ ] Service reloads requested by IPC, GUI shortcut, slash commands, events and timers only after the active Lua call unwinds; subsequent IPC commands use the new environment and retain print-output isolation.

Exact native event ordering remains unverified pending native evidence; tests must not invent teardown events. Existing simulator startup order is documented in [event system](../event-system.md#startup-event-sequence). Reload world-entry flags must distinguish reload from initial login.

## How it works

- [Reload lifecycle](../wiki/systems/reload-ui.md)
- [Startup and dispatch](../wiki/systems/event-system.md#startup-event-sequence)
- [Addon loading](../addon-loading-pipeline.md)

## Implementation inventory

- `src/c_api/c_ui.rs` — modeled `C_UI.Reload`, also called by Blizzard’s public `ReloadUI` wrapper.
- `src/lua_api/globals/state_backed_queries.rs` — legacy Lua `ReloadUI` registration sharing the request producer.
- `src/lua_api/env.rs` — Lua VM construction and handle retirement.
- `src/bin/wow_sim/main.rs`, `src/bin/wow_sim/gui_commands.rs` — dump/screenshot reload observation boundaries.
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

- [ ] Execute GREEN and startup/texture preservation checks. Behavioral RED observed: all four committed cases fail their fresh-environment assertions on f254e22ee.
- [ ] Establish native reload event sequence from native evidence supplied by the laptop through main.
- [ ] Collect bounded real CLI/socket acceptance and original-CVar integrity receipts.

## Out of scope

- Ellesmere runner or addon changes: the observed failure is simulator lifecycle state.
- Restarting the process, IPC server or modeled game session: only the UI/Lua environment reloads.
- Unconfirmed target/focus/native session-preservation claims: retain current model state without asserting native equivalence.
