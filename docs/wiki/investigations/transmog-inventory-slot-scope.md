# Blizzard_TransmogShared inventory-slot scope

Retail 12.1 removes the legacy `GetInventorySlotInfo` global from the public Lua surface, but `Blizzard_TransmogShared` still calls it while loading on demand. The runtime loader now supplies the registered simulator function only inside that addon's loading environment, then restores the prior global and environment state.

## Symptoms

- `C_AddOns.LoadAddOn("Blizzard_TransmogShared")` failed at `Blizzard_TransmogShared.lua:150` because `GetInventorySlotInfo` was nil.
- Publishing the function globally would violate the retail 12.1 removal contract.
- Restoring only `_G` was insufficient: later `TransmogUtil` closures still needed the function through their compiled environment.

## Root cause

`GetInventorySlotInfo` remains registered internally so supported simulator code can provide its slot lookup behavior, but retail strict-removal handling leaves the public global nil. `Blizzard_TransmogShared` is an exact compatibility window: its vendor files need the function during loading, while addon code after the load must continue to observe the removed global.

## Fix

`C_AddOns.LoadAddOn("Blizzard_TransmogShared")` temporarily:

1. Saves the current public `GetInventorySlotInfo` value and any existing loading-scoped environment.
2. Creates a target-scoped environment containing the registered function and forwarding other reads/writes to the public global table.
3. Installs that environment for loader-created closures and temporarily exposes the function while the TOC files execute.
4. Restores the prior public global and loading environment after success or a `LoadError`.

The target-scoped environment is retained by the compiled vendor closures, so subsequent `TransmogUtil` calls remain functional without republishing the removed global. The scope is not a general fallback for other addons.

## Contract and verification

- Retail `GetInventorySlotInfo` is nil before and after `Blizzard_TransmogShared` loads.
- `TransmogUtil.GetTransmogLocation("HEADSLOT", ...)` remains callable after the load.
- A deliberate load error restores both the prior global and the prior loading-scoped environment.
- Focused coverage: `tests/blizzard_transmog_shared_loads.rs::runtime_load_scopes_removed_inventory_slot_global` and the Rust restoration test in `src/c_api/c_addons_runtime.rs`.

## Scope

This preserves the retail removed-global contract and the exact Blizzard loader dependency. It does not model additional transmog collection/source state or make `GetInventorySlotInfo` public again.

## Sources

- [c_addons_runtime.rs](../../../src/c_api/c_addons_runtime.rs) — runtime `Blizzard_TransmogShared` scope, restoration, and retained closure environment
- [inventory_slot.rs](../../../src/lua_api/globals/inventory_slot.rs) — internal registered inventory-slot function
- [blizzard_transmog_shared_loads.rs](../../../tests/blizzard_transmog_shared_loads.rs) — public nil, post-load TransmogUtil, and scope regression coverage
- [strict_removals.lua](../../../src/ptr/strict_removals.lua) — profile removal handling for legacy globals
- [loader_env.rs](../../../src/lua_api/loader_env.rs) — loading-scoped function environments
- [appearances-wardrobe-api](appearances-wardrobe-api.md) — broader transmog/wardrobe API coverage and remaining state gaps

## See Also

- [[addon-loading]] — runtime LoadOnDemand execution and loader environments
- [[lua-api]] — public versus scoped Lua API boundaries
- [[lua-call-frame-restoration]] — nested runtime-load state restoration
