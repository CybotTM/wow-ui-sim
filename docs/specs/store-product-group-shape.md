# Store product group query shape

`C_StoreSecure.GetProductGroups` exposes existing simulator demo groups in the shape consumed by each audited profile. Demo backing and other queries remain in `src/lua_api/env_init/runtime_surface_bootstrap.lua`; the modeled group getter lives in `src/c_api/c_store_secure.rs`. See [Store source checkpoint](../wiki/investigations/store-secure-pool-constructors.md#store-group-shape--source-checkpoint-2026-10-14).

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

- `src/lua_api/env_init/runtime_surface_bootstrap.lua`: existing demo Lua table remains published as `__wow_store_state`; replaced Lua GetProductGroups definitions are removed.
- `src/c_api/c_store_secure.rs`: modeled group query reads the existing table and projects IDs only on Mists; other profiles retain their record path. No duplicated catalog or initialization bridge.

## Tests asserting this spec

- `tests/store_group_contract.rs`: active-profile API contract in a fresh `WowLuaEnv`; auto-discovered by integration harness.
- `tests/fixtures/store_group_contract.lua`: shared behavioral assertions; directly executable via normal `--exec-lua` with explicit mode prefix.

## Known gaps (current cycle)

- [x] Shared Mists preproducer normal-Lua fixture fails at `Mists GetProductGroups[1] must be numeric` (CLI1), before group/product lookups. Authored Rust case remains unexecuted. Independent existing-artifact audit read all 18 selected artifacts: Mists first-shape RED and Retail/PTR complete baseline lookups accepted. Source-audit ENOENT was a separate agent visibility limitation; main read the delivered report.
- [ ] Postproducer Rust/runtime GREEN and modern record preservation. Preproducer normal-Lua Retail/PTR record/lookup fixture completed with marker and `[]`; no Store UI acceptance follows.
- [ ] Evaluate separate public UI proposal only after API shape passes; it is not current API-test acceptance.

## Source checkpoint — 2026-10-14

Producer `85d8d7bd4`, tests `f199e9424`: modeled getter ownership and initialization order are source-inspected, not runtime acceptance. Mists no-run compilation finished successfully with 741 JSON records and no compiler-message records; stderr retains six manifest deprecations and one `UnitRole` dead-code warning. Both new copied executables passed independent once-per-copy authentication. Fresh Mists Rust API case passed (1 case); the full normal-session fixture failed at its first data assertion, `GetProductGroups must return a list` (CLI1, two error records for the same assertion). Later length/shape/group/product/entry assertions were unreached; Store UI was not exercised. Preproducer modern preservation does not cover this producer.

Source inspection identifies an original lifecycle gap: cached Mists `Blizzard_EnvironmentCleanup` clears `C_StoreSecure`; simulator restoration reruns the Lua bootstrap without re-registering the modeled getter. That is source-grounded explanation, not proof of the exact dynamically returned value/dispatch. Commit `d9c7c6aa1` adds a real cached-cleanup regression, checking the shared contract after original addon loading and post-load workarounds; compilation/execution remains pending. No getter override, artificial namespace clearing or forced vendor handler.

## Out of scope

- Other Classic profiles: no newly established consumer evidence; preserve existing behavior.
- Native parity claims: no official cached StoreSecure declarations or native execution evidence.
- Whole Store migration, new catalog defaults, transaction/settlement semantics, forced handlers, spies, getter overrides, and nil public category helpers.
