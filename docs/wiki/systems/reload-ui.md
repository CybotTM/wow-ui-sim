# ReloadUI

`ReloadUI` records a deferred request. The owner retires the old Lua VM and UI registries, then repeats addon bootstrap against the retained simulator game state. The [contract](../../specs/reload-ui.md) owns requirements and proof status.

## Request and ownership

Both `C_UI.Reload` and legacy `ReloadUI` call the same Rust request producer. Blizzard SharedXML replaces the public global with a wrapper that calls `C_UI.Reload`; wiring only the legacy global does not cover real startup. Lua calls only set `SimState::ui_reload_requested`; repeated calls coalesce. App services requests outside Lua borrows, at update boundaries and between IPC commands. An Exec response captures its output and acknowledges the completed invoking chunk before expensive rebootstrap. IPC transport and queued commands remain process-owned.

Dump-tree, screenshot and lua-errors owners also service requests after startup, script execution and settling callbacks, before observing their output. Headless owners settle rebuilt startup synchronously.

The GUI shortcut records the same request. App discards frame-ID input, hit-testing, quad and pending-texture caches, then restarts yielded GUI startup using current canvas dimensions. Texture manager, font system and glyph atlas allocations survive.

## Environment retirement

`WowLuaEnv::service_reload` writes declared SavedVariables before mutation. The persistence manager forgets old `Val` handles and registrations; each enabled addon re-registers through normal TOC loading. Previously saved addons bypass WTF import and read simulator storage. Their just-written data bypasses the external import-size cap and retains scalar globals as well as tables; ordinary initial imports keep the existing limit. Disabled persistence stays disabled. Imported WTF files are never written.

The old VM's self-reference and font handle are cleared before it drops. A fresh VM receives fresh app data, registries, API registration, templates and built-in frames. `SimState::discard_ui_environment` discards UI callbacks, animations, frames and addon runtime records without replacing player/world/party/session data. Selected target/focus and physical-input model fields (`mouse_position`, `mouse_buttons`, `pressed_keys`) are retained. Native keyboard/mouse persistence is INFERRED and unasserted pending lifecycle evidence; old frame-ID input caches are still discarded.

The launch pipeline installs a reload loader on the environment. It reuses Blizzard/third-party addon loading and post-load bootstrap, preserving original launch options. The initial EditMode layout name and a copied WTF config seed a clean cache reader with no old `Val` handles; the file-backed cache is reread for the fresh VM even when Lua SavedVariables are disabled. Native reread timing/ordering is INFERRED and unasserted. ServerSnapshot character/enable imports and initial party reseeding are still skipped on reload. No addon-specific sentinel cleanup exists.

## Constant dispatch factories

[`load_constant_factory`](../../../src/lua_api/script_helpers.rs) stores an unexecuted Lua-function anchor in the active VM registry, indexed by exact chunk name and source. The anchor roots the prepared prototype and its interned constants in that VM; no prototype or Lua handle lives in a process-global cache. HookScript and direct/indirect attribute dispatch retain their original Lua source, chunk names and invocation/error policies.

Each use allocates a new Lua closure from the prototype, binds it to the current global table and initializes independent nil top-level upvalues, matching `load_bytes`. The global table is shared within a VM; each closure's environment binding is independent. Returning/executing the anchor would let callback `setfenv` changes contaminate later calls, so the anchor is never exposed to dispatch. Nested hook closures and their captured arguments are still created by normal Lua execution.

The single constant `template-method-handler` factory also uses this path through [`load_constant_chunk`](../../../src/loader/chunk_cache.rs). Its exact source-derived generated chunk name is unchanged, and the builder still disables dynamic global slots while loading. Factory execution still creates a new method-name capture for each XML handler; inherited/chained handlers retain their existing execution policy. Other generated chunks keep the existing bytecode path: frame-ID key churn is not normalized or cached as a mutable closure.

Reload replaces the VM registry, so the prototype cache begins empty without a model-state reset or extra cleanup path. The [contract](../../specs/reload-ui.md#dispatch-factory-isolation) records matched before/after characterization and profile compilation; paired wall-time and full acceptance remain separate. Native lifecycle ordering remains unconfirmed.

## Lifecycle evidence and failure

Existing simulator reload world-entry flags are reused; native teardown order remains unconfirmed. No teardown event ordering is invented. App treats rebuild failure as fatal and prevents a partial VM from overwriting pre-reload SavedVariables.

## Sources

- [Request producer](../../../src/c_api/c_ui.rs) and [environment](../../../src/lua_api/env.rs) — deferred request and VM retirement.
- [Dispatch helpers](../../../src/lua_api/script_helpers.rs) — per-VM prototype rooting and per-use closure construction.
- [UI-owned state](../../../src/lua_api/state/ui_reload.rs) — reset boundary.
- [Launch bootstrap](../../../src/bin/wow_sim/startup.rs) — real addon reload loader.
- [App](../../../src/iced_app/app.rs) and [IPC boundary](../../../src/iced_app/update_servers.rs) — cache/startup reset and response-before-rebuild sequencing.
- [Persistence](../../../src/saved_variables.rs) — SavedVariables registration and storage.

## See Also

- [[event-system]] — existing startup dispatch.
- [[addon-loading]] — addon graph reconstruction.
- [[texture-source-resolution-cache]] — generation invalidation without resource replacement.
