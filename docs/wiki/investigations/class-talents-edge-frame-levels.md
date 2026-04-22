# Class Talents Edge Frame Levels

Class-talent connector edges were rendering above node icons in the talents panel. The simulator was leaving some edge frames at high levels relative to the visible button stack.

## Finding

`PlayerSpellsFrame.TalentsFrame` showed active edge frames at levels like `1411`, `1421`, and `1491`, while the minimum visible talent button level in the same panel was `586`. That let edge lines overdraw node icons.

The root issue was the class-talent edge-level rule not being forced onto the live frame instance after addon load. Patching only `ClassTalentsFrameMixin` is insufficient once methods are copied onto already-instantiated frames.

## Fix

- Added `CLASS_TALENT_EDGE_FRAME_LEVEL_WORKAROUND_LUA` in `src/lua_api/workarounds.rs`.
- Applied it in both startup workarounds and runtime addon-load workarounds for `Blizzard_PlayerSpells`.
- Patched both:
  - `ClassTalentsFrameMixin.GetFrameLevelForEdge`
  - live `PlayerSpellsFrame.TalentsFrame.GetFrameLevelForEdge` when present
- Immediately re-leveled existing active edges via `UpdateEdgeFrameLevel`.

The patched edge-level rule now caps edges below:
- both connected endpoint buttons, and
- the lowest visible talent-button frame level in the panel.

## Regression Coverage

Added `test_class_talent_edges_render_below_visible_talent_buttons` in `tests/hero_talents.rs`.

The test opens class talents, enumerates visible buttons + active edges, and asserts every edge frame level is strictly below the minimum visible button frame level.

## Sources

- [workarounds.rs](../../../src/lua_api/workarounds.rs) — class-talents edge-level workaround implementation
- [hero_talents.rs](../../../tests/hero_talents.rs) — regression test

## See Also

- [[class-talents-trait-loadout-state]] — earlier hero-subtree visibility/edge restoration work
- [[hero-spec-icon-bug]] — adjacent class-talents rendering investigation
