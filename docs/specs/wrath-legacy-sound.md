# Wrath legacy named sounds

Under `client-wrath`, legacy global `PlaySound(name)` accepts any Lua string
and records the requested name plus an optional kit in simulator sound state.
This is the user-approved simulator policy from
`user-decisions-wrath-playsound-20261016.json`, not native lookup/error parity.
See [Lua API architecture](../lua-api.md) for the surrounding system.

## What it must do

- [x] Record every requested string byte-exact, including unknown, empty and
  non-UTF-8 names. Unknown names are a playback no-op, not an error; recording
  still occurs and clears any previous kit rather than leaving a stale ID.
- [x] Resolve only these bounded, exact spellings with unique public dump IDs:
  `WriteQuest` = 3093, `igAbiliityPageTurn` = 836,
  `igMainMenuOptionCheckBoxOn` = 856. No case folding or aliases.
- [x] Original TargetFrame hide records `INTERFACESOUND_LOSTTARGETUNIT` name-only:
  the public dump has duplicate rows 684/900, so neither ID is selected.
- [x] Original SpellBook XML initial tab records `igAbiliityPageTurn`/836;
  original Mail XML SendMail OnLoad records `igMainMenuOptionCheckBoxOn`/856;
  original `QuestInfoFadingFrame_OnUpdate` (`W/FrameXML/QuestInfo.lua:5-6`)
  records `WriteQuest`/3093.
- [x] Numeric global `PlaySound` and `C_Sound.PlaySound` remain unchanged:
  numeric requests use the existing kit path, namespace strings still reject,
  and numeric calls do not overwrite the last requested legacy name.

### Discovery pointers and evidence limits

[Flowy/cmangosDB wiki: SoundEntries.dbc](https://github.com/Flowy/cmangosDB/wiki/SoundEntries.dbc)
identifies the public name/ID dump as 3.3.5a; the page reports last edited
July 10, 2014. Its named rows supply the three unique IDs and the
684/900 duplicate above. These IDs are **not verified against an original DBC
checksum**. The dump is a discovery pointer, not proof of client alias rules,
lookup ambiguity resolution or error behavior.

[WoWDBDefs: SoundEntries.dbd](https://github.com/wowdev/WoWDBDefs/blob/master/definitions/SoundEntries.dbd)
identifies separate `ID` and `Name` columns for the build range ending in
3.3.5.12340. It is a schema/discovery pointer, not a row-value source or original
DBC verification. Later Mists SOUNDKIT constants are not mapping authority.
The bounded mapping is not a general import of the public sound catalog.

## How it works

- [Lua API architecture](../lua-api.md)

## Implementation inventory

- `src/lua_api/globals/real/legacy_sound.rs`: Wrath-only named-global adapter.
- `src/lua_api/globals/missing_surface.rs`: profile-specific global registration.
- `src/c_api/c_sound.rs`: shared sound request model; namespace validation unchanged.
- `src/lua_api/globals/real/mod.rs`: Wrath-only module gate.
- `src/lua_api/state.rs`, `src/lua_api/state/sim_state.rs`: initially absent
  last requested name bytes, alongside the existing optional requested kit.

## Tests asserting this spec

- `tests/wrath_legacy_sound.rs`: six passing tests; unchanged original consumers
  loaded from hash-pinned cache files. Quest callback uses ordinary widget
  inputs, not the complete QuestFrame XML tree. Mail uses the unchanged virtual
  FriendsFrameTabTemplate parsed from the complete pinned FriendsFrame.xml,
  without constructing the unrelated FriendsFrame tree. OptionsPanelTemplates
  and CharacterFrameTemplates supply original virtual prerequisites. No vendor
  function is replaced.

## Known gaps (current cycle)

Behavioral RED: six string-request failures against the passive-state library
`cd4e70018`. GREEN: six passes with the same repaired fixtures against sound
producer `16bf5b4de` and final production `6dab184f6` (2026-10-17). The final
44-case Wrath gate passes all six sound and 36 retained cases; two separate
loot cases remain blocked. Audio output is not tested.

## Out of scope

Native parity, complete startup recovery, general sound catalog resolution,
playback fidelity and group-transition semantics; this stage models requests only.
