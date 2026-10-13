# Current LFG follower dungeon

`C_LFGInfo.IsInLFGFollowerDungeon()` reports current instance membership from existing world LFG ID and dungeon classification. [Patch 10.2.5 source](../../data/patch-api/sources/10.2.5-api-changes.txt) publishes the function; cached `LFGInfoDocumentation.lua` declares zero arguments and one nonnil boolean. Current constant-false compatibility behavior is not a modeled query.

## What it must do

- [ ] Return one boolean: true when `world.in_instance` and its `instance_lfg_dungeon_id` identifies an existing follower-classified dungeon.
- [ ] **Modeled policy:** known non-follower, unknown/missing ID and outside-instance states return false, including stale follower IDs outside an instance.
- [ ] Observe live ID/classification updates and removal; isolate environments. Never infer ID from name, map, difficulty or party makeup.
- [ ] Extend existing host `A_Admin.SetInstanceInfo(name, type, difficulty, maxPlayers, lfgDungeonID?)` with optional fifth input, preserving earlier outputs and existing instance flag behavior.
- [ ] **Host replacement policy:** omitted/nil ID clears prior current LFG ID; an unknown numeric ID may be stored but does not become a follower. `SetInInstance` remains a flag-only toggle.
- [ ] Keep existing `GetInstanceInfo` tenth result and generic LFG membership consistent with supplied state.
- [ ] Exercise unchanged cached compact-frame name logic with controlled AI/walk-in/spectator inputs. This is a bounded caller fixture, not AI modeling or rendered/native acceptance.

## How it works

- [Host zone API](../admin-api/zone.md).
- [Lua API](../wiki/systems/lua-api.md).

## Implementation inventory

- `src/c_api/c_lfg_info.rs`: existing dungeon classification and new query destination.
- `src/lua_api/workarounds/temporary/lfg_legacy_defaults.rs`: current constant-false path to replace.
- `src/lua_api/globals/admin_zone_economy.rs`: existing host instance setter.
- Existing world `instance_lfg_dungeon_id` and `lfd_dungeons` classification: no new catalog needed.

## Tests asserting this spec

`tests/lfg_follower_instance.rs`: eight unrun cases for positive/false matrices, live mutation/isolation, optional host input/clearing, existing projections and exact cached caller logic. No RED/GREEN claim yet.

## Known gaps (current cycle)

- [ ] Execute genuine RED before new query/host input production, then targeted GREEN and verification.
- [ ] Validate narrow cached-caller fixture at execution; no full UnitFrame/startup claim.

## Out of scope

AI party systems, native catalogs, localization, physical input/rendering, unrelated classifier validation/security changes and earlier-profile parity are independent contracts. Optional host ID uses existing bridge extraction conventions; no new coercion/domain policy is claimed. No vendor patches.
