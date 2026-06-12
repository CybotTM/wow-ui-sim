# XmlFrameLevelProbe

Settles the XML `frameLevel` semantics conflict between two simulator
investigations:

- **April (`11543bc5c`, tests/xml_frame_strata.rs):** a child's bare XML
  `frameLevel` is a parent-relative offset (`parent 50 + child 10 = 60`) and
  the child keeps `IsUsingParentLevel() == true`.
- **May (`2fad99bdf`, Azerite Essence layering):** bare XML `frameLevel` is an
  absolute level applied as fixed (`frame_level.fixed.unwrap_or(true)`).
- **Wowless reference:** XML `frameLevel` maps to plain `SetFrameLevel`
  (absolute, NOT fixed); unfixed frames are recomputed to `parent + 1` when
  parented.

Also re-verifies that Lua `SetFrameLevel` does not implicitly fix the level
across `SetParent` (pinned by `test_set_frame_level_does_not_fix_level`).

## Usage

Copy to the retail AddOns folder, log in on any character, then `/reload` or
log out. Inspect `WTF/.../SavedVariables/XmlFrameLevelProbe.lua`:

- `load.childPlain.frameLevel`: `60` → April offset semantics; `10` → absolute.
- `load.childPlain.isUsingParentLevel` / `hasFixedFrameLevel`: which flags a
  bare frameLevel sets.
- `afterParentSetLevel60.childPlain.frameLevel`: whether unfixed XML children
  follow later parent level changes (`70` = offset tracks, `61` = wowless
  parent+1, `10` = fully detached).
- `luaSetFrameLevel.levelAfterReparentToLevel50Parent`: `51` confirms
  SetFrameLevel does not fix; `5` means it does.

Update `tests/xml_frame_strata.rs` and
`src/lua_api/globals/template/direct/frame_level.rs` (`resolve_frame_level` /
`set_xml_frame_level`) to match whatever this captures.
