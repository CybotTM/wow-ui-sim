# Wrath temporary startup capability defaults

Wrath-only legacy defaults live in `src/lua_api/workarounds/temporary/wrath_startup_defaults.rs`, naming each missing backing system and retirement condition. The user approved these simulator choices on 2026-10-17; none is native-capability or hardware evidence.

## What it must do

- [ ] `HasDebugZoneMap()` returns false until a debug-zone tile/capability model exists. Original WorldMapFrame.lua:363 skips debug tiles; no populated debug-map state is invented.
- [ ] `UnitIsTalking(name)` returns false for every Lua string until the voice-chat speaking model exists. Original PlayerFrame.lua:150 passes `UnitName("player")`, not a unit token. Unknown/empty names, Unicode and arbitrary byte strings are accepted without lookup, decoding or network/audio access.
- [ ] Original world-entry voice handling hides the speaker indicator through unchanged PlayerFrame.lua:366–373 and VoiceChat.lua:197–205.
- [ ] `IsPlayerResolutionAvailable()` returns true until the player-texture-resolution capability model exists. Original Effects event must not force-disable an enabled PlayerTexture control.
- [ ] Publication remains Wrath-only; no Retail/Forever public surface change.

## How it works

- [Wrath startup investigation](../wiki/investigations/wrath-startup-u-residuals.md)
- [Empty map overlays](wrath-empty-map-overlays.md)

## Implementation inventory

- `src/lua_api/workarounds/temporary/wrath_startup_defaults.rs`: three explicit temporary producers; no namespace/global catch-all.
- `src/lua_api/workarounds/temporary/mod.rs` and `workarounds/mod.rs`: existing client-wrath-only module/export gates.
- `src/lua_api/globals/real/wrath_startup_queries.rs`: existing Wrath initialization route.

## Tests asserting this spec

- `tests/wrath_startup_boolean_consumers.rs`: complete original Wrath startup reaches the map and voice boundaries; voice accepts concrete empty/unknown/Unicode/non-UTF-8 names and the original indicator hides.
- `tests/wrath_unmasked_video.rs`: original Effects event with an explicitly supplied BaseMip=0 prerequisite; PlayerTexture begins enabled and stays enabled.

## Known gaps (current cycle)

- [x] Before producers, all three original-consumer tests fail at WorldMapFrame.lua:363, PlayerFrame.lua:150 and VideoOptionsPanels.lua:441 against sealed d11a25154.
- [ ] GREEN and updated startup delta pending.
- [ ] The next debug-object catalog is unmodeled; this policy does not supply its count or records.
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
