# ReloadUI

`ReloadUI` records a deferred request. The owner retires the old Lua VM and UI registries, then repeats addon bootstrap against the retained simulator game state. The [contract](../../specs/reload-ui.md) owns requirements and proof status.

## Request and ownership

Lua calls only set `SimState::ui_reload_requested`; repeated calls coalesce. App services requests outside Lua borrows, at update boundaries and between IPC commands. An Exec response captures its output and acknowledges the completed invoking chunk before expensive rebootstrap. IPC transport and queued commands remain process-owned.

The GUI shortcut records the same request. App discards frame-ID input, hit-testing, quad and pending-texture caches, then restarts yielded GUI startup using current canvas dimensions. Texture manager, font system and glyph atlas allocations survive.

## Environment retirement

`WowLuaEnv::service_reload` writes declared SavedVariables before mutation. The persistence manager forgets old `Val` handles and registrations; each enabled addon re-registers through normal TOC loading. Previously saved addons bypass WTF import and read simulator storage. Disabled persistence stays disabled. Imported WTF files are never written.

The old VM's self-reference and font handle are cleared before it drops. A fresh VM receives fresh app data, registries, API registration, templates and built-in frames. `SimState::discard_ui_environment` discards UI callbacks, animations, frames and addon runtime records without replacing player/world/party/session data. Selected target/focus are retained as model state, not asserted as native behavior.

The launch pipeline installs a reload loader on the environment. It reuses Blizzard/third-party addon loading and post-load bootstrap, preserving original launch options. ServerSnapshot character/enable imports and initial party reseeding are skipped on reload. No addon-specific sentinel cleanup exists.

## Lifecycle evidence and failure

Existing simulator reload world-entry flags are reused; native teardown order remains unconfirmed. No teardown event ordering is invented. App treats rebuild failure as fatal and prevents a partial VM from overwriting pre-reload SavedVariables.

## Sources

- [Environment](../../../src/lua_api/env.rs) — request service and VM retirement.
- [UI-owned state](../../../src/lua_api/state/ui_reload.rs) — reset boundary.
- [Launch bootstrap](../../../src/bin/wow_sim/startup.rs) — real addon reload loader.
- [App](../../../src/iced_app/app.rs) — cache reset and startup restart.
- [IPC boundary](../../../src/iced_app/update_servers.rs) — response-before-rebuild sequencing.
- [Persistence](../../../src/saved_variables.rs) — SavedVariables registration and storage.

## See Also

- [[event-system]] — existing startup dispatch.
- [[addon-loading-pipeline]] — addon graph reconstruction.
- [[texture-source-resolution-cache]] — generation invalidation without resource replacement.
