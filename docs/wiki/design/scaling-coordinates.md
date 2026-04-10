# Scaling and Coordinates

WoW uses a non-standard coordinate system that differs from most GUI frameworks. Getting this right is critical for correct anchor resolution and layout rendering.

## WoW Coordinate System

**Bottom-left origin, Y increases upward** (not the typical Y-down of web/desktop frameworks):

- `(0, 0)` = bottom-left corner
- `(screenWidth, screenHeight)` = top-right corner
- Default anchor point when none is specified: `TOPLEFT`
- Reference screen size (from wowless): `1280×720`

This Y-up convention means that when converting from WoW coords to GPU clip space, the Y axis must be inverted. The projection matrix in `pipeline.rs` handles this via an orthographic projection that maps `(0,0)-(width,height)` to clip space `(-1,-1)-(1,1)`.

## Current Implementation

**Canvas-based, dynamic sizing**: the layout does not use a fixed screen size. The canvas `size` parameter drives layout, so WoW coords map 1:1 to canvas pixels and the view adapts to the window.

**Lua API** currently returns hardcoded values: `GetScreenWidth()` → `1280.0`, `GetScreenHeight()` → `720.0`. These should eventually return the actual canvas size dynamically (tracked as a TODO).

**`UI_SCALE`** is defined as `1.0` in `src/render/texture.rs`, so no scaling is applied. The constant multiplies WoW coords to get display coords.

## Known Issues Fixed

Two coordinate bugs were resolved during development:

1. **Hardcoded anchor override** — `main.rs` was forcing `TOPLEFT (10, -10)` on the root frame instead of using the XML-defined `CENTER` anchor.
2. **Screen size mismatch** — internal screen size was hardcoded; now uses canvas size.

## Remaining TODOs

- `GetScreenWidth`/`GetScreenHeight` should return actual canvas size, not hardcoded `1280×720`
- Y-axis inversion needs explicit verification (WoW Y-up vs GPU/iced Y-down)
- `CENTER` anchor behavior with dynamic canvas size needs testing
- Debug purple border should be removed when layout work is complete

## Key Files

| File | Role |
|------|------|
| `src/iced_app.rs` | Layout calculation using canvas `size` |
| `src/render/shader/pipeline.rs` | Orthographic projection matrix |
| `src/render/texture.rs` | `UI_SCALE` constant |
| `src/lua_api/globals.rs` | `GetScreenWidth` / `GetScreenHeight` stubs |

## Sources

- [SCALING.md](../../../SCALING.md) — coordinate system, implementation status, TODOs

## See Also

- [[architecture-overview]] — overall Lua/Rust system design
- [[debug-tools]] — debug overlays for verifying anchor positions
