# Wrath temporary startup capability defaults

Wrath-only legacy defaults live in `src/lua_api/workarounds/temporary/wrath_startup_defaults.rs`, naming each missing backing system and retirement condition. The user explicitly approved the three boolean values on 2026-10-17. Empty counts apply the separately approved standing empty/zero/no-selection policy to the observed unsupplied domains. None is native-capability or hardware evidence. The former constant voice probe is now replaced by the state-backed reader in [Wrath loot/party support](wrath-initial-loot.md); only one registration remains.

## What it must do

### Empty map domains

- [x] `HasDebugZoneMap()` returns false until a debug-zone tile/capability model exists. Original WorldMapFrame.lua:363 skips debug tiles; no populated debug-map state is invented.
- [x] `GetNumMapDebugObjects()` returns numeric zero until a supplied map-debug-objects model owns both count and records. Original WorldMapFrame.lua:402 OnLoad must traverse the empty domain without calling a missing count producer.
- [x] `GetNumDungeonMapLevels()` returns zero for an unsupplied selected-map floor catalog, under the same standing empty-count policy. Original OnLoad reaches WorldMapFrame.lua:592; its original dropdown update clears/hides the floor selector and up/down controls. Never derive total floors from the selected-floor scalar.

### Voice and player textures

- [x] `UnitIsTalking(name)` reads existing channel members' speaking flags by exact character-name bytes, through the [state-backed voice reader](wrath-initial-loot.md). Original PlayerFrame.lua:150 passes `UnitName("player")`, not a unit token. Unsupplied/unknown/empty names and the original unknown Unicode/binary controls remain false; supplied matching active speakers return true. No network/audio access.
- [x] Original world-entry voice handling hides the speaker indicator through unchanged PlayerFrame.lua:366–373 and VoiceChat.lua:197–205.
- [x] `IsPlayerResolutionAvailable()` returns true until the player-texture-resolution capability model exists. Original Effects event must not force-disable an enabled PlayerTexture control.

## How it works

- [Wrath startup investigation](../wiki/investigations/wrath-startup-u-residuals.md)
- [Empty map overlays](wrath-empty-map-overlays.md)

## Implementation inventory

- `src/lua_api/workarounds/temporary/wrath_startup_defaults.rs`: two explicit capability producers and empty debug-object/floor counts; no namespace/global catch-all.
- `src/lua_api/workarounds/temporary/mod.rs` and `workarounds/mod.rs`: existing client-wrath-only module/export gates.
- `src/lua_api/globals/real/wrath_startup_queries.rs`: existing Wrath initialization route.

## Tests asserting this spec

- `tests/wrath_startup_boolean_consumers.rs`: complete original Wrath startup reaches the map and voice boundaries; voice accepts concrete empty/unknown/Unicode/non-UTF-8 names and the original indicator hides.
- `tests/wrath_unmasked_video.rs`: original Effects event with explicitly registered and set BaseMip=0; a preceding child-only override1 reproduces the shared-process fixture ordering. PlayerTexture begins enabled and stays enabled. RegisterCVar alone does not override a persisted current value; this test supplies input, not a default.

## Known gaps (current cycle)

- [x] Before producers, all three original-consumer tests fail at WorldMapFrame.lua:363, PlayerFrame.lua:150 and VideoOptionsPanels.lua:441 against sealed d11a25154.
- [x] The final rebased gate passes 48 named cases (36 controls + 12 queue cases). Original OnLoad:592 RED precedes the floor producer; the original empty-floor dropdown transition passes. Startup drops from the exact e90 baseline 22/30 to 15/21, with no new records. See the bounded proof below.
- [x] The new debug-object count reaches original WorldMapFrame.lua:402 in a behavioral RED before its producer, under the approved empty/zero policy.
- [ ] Populated debug objects and floor catalogs remain unmodeled; count defaults do not invent records. The newly unmasked floor count has an original OnLoad:592 RED before its producer.
- [ ] BaseMip has no approved default; the explicit video fixture does not establish one. The generated BaseMip/gamma records remain attribution-only.

## Out of scope

Native parity, actual rendering/hardware support, populated debug tiles, real voice transport (supplied speaking state belongs to the linked loot/party support), BaseMip defaults, gamma operations, Blizzard/vendor edits and blanket missing-API handling.

## Original resolution branch

VideoOptionsPanels.lua:441–443:

```lua
if ( not IsPlayerResolutionAvailable() ) then
    VideoOptionsEffectsPanelPlayerTexture:Disable();
end
```

False force-disables the control. True only skips this block; it never calls Enable and therefore does not itself establish that the control must be enabled under every other state.

## Final bounded proof — 2026-10-17

Historical u-resid receipt before combined loot/party support; the [combined proof](wrath-initial-loot.md#combined-proof--2026-10-17) supersedes it for current integration. This receipt used the old rilua pin.

Rebased on `e90d921cc`, whose committed Wrath baseline is exactly equal to the independently captured base messages/counts. Normal sealed Wrath source `8f7644b3a` contains the final production inputs from `112241401`; later changes are documentation only.

| Scope | Result |
|---|---|
| Wrath named consumers and controls | 48/48 PASS in two bounded groups: 36 existing controls and 12 queue cases. |
| Wrath startup | 22 records / 30 occurrences → 15 / 21, exit1. Seven original records removed, no new records. The numeric-only PlaySound aggregate rises6→7 as further original work runs; TargetFrame.lua:232 is its representative stack, not every occurrence. The [combined raw-stack disclosure](../baselines/README.md#wrath-lua-errors) distinguishes TargetFrame and CombatLog callers. |
| Retail preservation | Both existing combo tests pass; same-file speed controls and identical-source admin/query trace pass. Exact d4/current trace equality:535 bytes, SHA-256 `27356d533851e86f9773b22db33b73efd8e66ff41e5be53791fba2aa17bff983`. Separate startup[]. |
| Forever preservation | Five unchanged target-bound model cases plus the corrected original-consumer case pass. That stale fixture fails on the old library too; its corrected event-handler form passes on both. Separate startup[]. |
| Era engraving relocation | Five existing namespace cases plus original empty-history startup/advance control pass; original guarded false policy and strict inventory preserved. |

Whole `cargo fmt --check`, default `cargo check` and Wrath `cargo check` pass. Wrath retains the base's existing Tank/Healer/Damage dead-code warning; no new diagnostic or suppression. Retail/Forever compiler dependency records exclude the later Wrath-only module changes; their receipts are not relabeled as whole-tree or binary identity.

All executions use sealed copies, MemoryMax4G, timeout86, CASC0/no-sound1 and fresh child-only byte-exact current CVar copies, with unchanged-original checks. No vendor/Blizzard/rilua changes or native-parity claim. Sound/loot work remains separate; BaseMip default, gamma policy and populated combat-log replay remain unlanded. Exact commands, source/binary hashes, input pins, intermediate unmasked records and final inventories are retained in the private `wrath-u-resid` handoff.
