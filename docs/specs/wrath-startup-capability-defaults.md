# Wrath temporary startup capability defaults

Wrath-only legacy defaults live in `src/lua_api/workarounds/temporary/wrath_startup_defaults.rs`, naming each missing backing system and retirement condition. The user explicitly approved the three boolean values on 2026-10-17. Empty counts apply the separately approved standing empty/zero/no-selection policy to the observed unsupplied domains. None is native-capability or hardware evidence.

## What it must do

### Empty map domains

- [x] `HasDebugZoneMap()` returns false until a debug-zone tile/capability model exists. Original WorldMapFrame.lua:363 skips debug tiles; no populated debug-map state is invented.
- [x] `GetNumMapDebugObjects()` returns numeric zero until a supplied map-debug-objects model owns both count and records. Original WorldMapFrame.lua:402 OnLoad must traverse the empty domain without calling a missing count producer.
- [ ] `GetNumDungeonMapLevels()` returns zero for an unsupplied selected-map floor catalog, under the same standing empty-count policy. Original OnLoad reaches WorldMapFrame.lua:592; its original dropdown update clears/hides the floor selector and up/down controls. Never derive total floors from the selected-floor scalar.

### Voice and player textures

- [x] `UnitIsTalking(name)` returns false for every Lua string until the voice-chat speaking model exists. Original PlayerFrame.lua:150 passes `UnitName("player")`, not a unit token. Unknown/empty names, Unicode and arbitrary byte strings are accepted without lookup, decoding or network/audio access.
- [x] Original world-entry voice handling hides the speaker indicator through unchanged PlayerFrame.lua:366–373 and VoiceChat.lua:197–205.
- [x] `IsPlayerResolutionAvailable()` returns true until the player-texture-resolution capability model exists. Original Effects event must not force-disable an enabled PlayerTexture control.

## How it works

- [Wrath startup investigation](../wiki/investigations/wrath-startup-u-residuals.md)
- [Empty map overlays](wrath-empty-map-overlays.md)

## Implementation inventory

- `src/lua_api/workarounds/temporary/wrath_startup_defaults.rs`: three explicit capability producers and empty debug-object/floor counts; no namespace/global catch-all.
- `src/lua_api/workarounds/temporary/mod.rs` and `workarounds/mod.rs`: existing client-wrath-only module/export gates.
- `src/lua_api/globals/real/wrath_startup_queries.rs`: existing Wrath initialization route.

## Tests asserting this spec

- `tests/wrath_startup_boolean_consumers.rs`: complete original Wrath startup reaches the map and voice boundaries; voice accepts concrete empty/unknown/Unicode/non-UTF-8 names and the original indicator hides.
- `tests/wrath_unmasked_video.rs`: original Effects event with explicitly registered and set BaseMip=0; a preceding child-only override1 reproduces the shared-process fixture ordering. PlayerTexture begins enabled and stays enabled. RegisterCVar alone does not override a persisted current value; this test supplies input, not a default.

## Known gaps (current cycle)

- [x] Before producers, all three original-consumer tests fail at WorldMapFrame.lua:363, PlayerFrame.lua:150 and VideoOptionsPanels.lua:441 against sealed d11a25154.
- [ ] The capability/debug checkpoint passes 47 named cases (36 controls + 11 queue cases); startup drops from the exact e90 baseline22/30 to16/22 and exposes the original floor-count call at:592. The final post-floor-count gate and capture remain pending.
- [x] The new debug-object count reaches original WorldMapFrame.lua:402 in a behavioral RED before its producer, under the approved empty/zero policy.
- [ ] Populated debug objects and floor catalogs remain unmodeled; count defaults do not invent records. The newly unmasked floor count has an original OnLoad:592 RED before its producer.
- [ ] BaseMip has no approved default; the explicit video fixture does not establish one. The generated BaseMip/gamma records remain attribution-only.

## Out of scope

Native parity, actual rendering/hardware support, populated debug tiles, real voice transport/speaking state, BaseMip defaults, gamma operations, Blizzard/vendor edits and blanket missing-API handling.

## Original resolution branch

VideoOptionsPanels.lua:441–443:

```lua
if ( not IsPlayerResolutionAvailable() ) then
    VideoOptionsEffectsPanelPlayerTexture:Disable();
end
```

False force-disables the control. True only skips this block; it never calls Enable and therefore does not itself establish that the control must be enabled under every other state.
