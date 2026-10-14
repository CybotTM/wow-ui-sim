# Store product group query shape

`C_StoreSecure.GetProductGroups` exposes existing simulator demo groups in the shape consumed by each audited profile. Current seed/query code lives in `src/lua_api/env_init/runtime_surface_bootstrap.lua`; modeled C_* query ownership belongs in `src/c_api/`. See [Lua API architecture](../lua-api.md).

Evidence is inferred from original cached consumers plus explicit simulator policy, **not native proof**. Cached official StoreSecure declarations are absent. Retail/PTR cached shared Store consumers read `groupID`/`parentGroupID`; the Mists TBC consumer treats each entry as an ID.

## What it must do

- [ ] Retail/PTR: return one demo record in a fresh simulator with `groupID=501`, `parentGroupID=0`.
- [ ] Mists: return one demo numeric ID in a fresh simulator, `501`; no record adapter in the consumer.
- [ ] Resolve the returned actual ID through `GetProductGroupInfo` to `groupName="Featured"` and through `GetProducts` to the single entry `1003`.
- [ ] Resolve returned entry `1003` through `GetEntryInfo` to `entryID=1003`, `productID=2003`, and `sharedData.name="Apprentice Rider Bundle"`.

Concrete fresh demo values above are explicit simulator policy, not claims about a live/native catalog.

## How it works

- [Lua API architecture](../lua-api.md)
- [Addon loading pipeline](../addon-loading-pipeline.md)

## Implementation inventory

- `src/lua_api/env_init/runtime_surface_bootstrap.lua`: existing demo Lua table and queries; unchanged by this stage.
- `src/c_api/`: main implements the small native group query over the existing Lua table using the existing published `__wow_store_state` Lua table; no producer included here and no new module name assumed.

## Tests asserting this spec

- `tests/store_group_contract.rs`: active-profile API contract in a fresh `WowLuaEnv`; auto-discovered by integration harness.
- `tests/fixtures/store_group_contract.lua`: shared behavioral assertions; directly executable via normal `--exec-lua` with explicit mode prefix.

## Known gaps (current cycle)

- [ ] Run Mists preproducer RED; staged assertion must fail with `Mists GetProductGroups[1] must be numeric`, before any group/product lookup.
- [ ] Run Retail/PTR record preservation checks and postproducer Mists GREEN.
- [ ] Evaluate separate public UI proposal only after API shape passes; it is not current API-test acceptance.

## Out of scope

- Other Classic profiles: no newly established consumer evidence; preserve existing behavior.
- Native parity claims: no official cached StoreSecure declarations or native execution evidence.
- Whole Store migration, new catalog defaults, transaction/settlement semantics, forced handlers, spies, getter overrides, and nil public category helpers.
