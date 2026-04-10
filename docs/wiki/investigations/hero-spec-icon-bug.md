# Hero Spec Icon Bug

Status: retired — the original report is stale in the current build.

## Original Claim

Hero talent spec icon rendered at the bottom-right of the talent panel instead of top-center.

## Investigation Result

The current build does not reproduce this bug. Five layers of evidence ruled it out:

1. **dump-tree**: `HeroTalentsContainer` at `x=711, y=61`; `HeroSpecButton` at `x=752, y=151` — correct top-center position.
2. **Layout rects match**: `layout_rect` on every frame in the hero subtree matches freshly computed rects after `ensure_layout_rects()`.
3. **Quad emission matches layout rect**: `tests/hero_talents_render.rs` verifies `Icon1` emits exactly one textured quad whose vertex bounds exactly match the layout rect (x=481.58, y=111.41, w=60.83).
4. **Atlas crop correct**: Emitted crop request matches atlas DB entry for `talents-heroclass-paladin-lightsmith` exactly.
5. **Hiding `HeroTalentsContainer`** only changes the top-center region — the old bottom-right point `(1000, 610)` is unaffected and resolves only to `framegeneral/ui-background-marble`.

## Frame Hierarchy

`HeroTalentsContainer` is a sibling of `ButtonsParent` (not a child), anchored to its TOP. `UpdateSpecBackground()` uses a 4-arg `SetPoint("TOP", ButtonsParent, heroContainerOffset, 0)` — relativePoint defaults to "TOP".

## What Was Ruled Out

- Stale layout_rect
- SetPoint parsing error (4-arg form correctly parsed)
- UI_SCALE transform (is 1.0)
- Duplicate frames
- Pan offset (moves individual buttons, not ButtonsParent)
- clipChildren clipping (HeroTalentsContainer is a sibling, not a child of ButtonsParent)

## Debug Tool Added

`--dump-tree` flag on the `screenshot` subcommand.

## Sources

- [hero-spec-icon-bug.md](../../hero-spec-icon-bug.md) — full investigation with test coverage

## See Also

- [[class-talents-artifact]] — separate visual oddity near lower-right, ruled out as screenshot encoding artifact
