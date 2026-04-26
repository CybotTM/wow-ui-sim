# Scaling and Coordinates

WoW uses a non-standard coordinate system that differs from most GUI frameworks. Getting this right is critical for correct anchor resolution and layout rendering.

## WoW Coordinate System

**Bottom-left origin, Y increases upward** (not the typical Y-down of web/desktop frameworks):

- `(0, 0)` = bottom-left corner
- `(screenWidth, screenHeight)` = top-right corner
- Default anchor point when none is specified: `TOPLEFT`
- Reference screen size (from wowless): `1280×720`

The renderer itself runs in iced's top-left Y-down screen space. The orthographic projection in `src/render/shader/pipeline.rs` (`Uniforms::new`) maps `(0,0)–(width,height)` screen coords to clip space `(-1,-1)–(1,1)`, with the Y row negated and translated `+1` so screen-top sits at clip `+Y`. WoW's Y-up convention is reconciled inside the anchor/layout pass before quads reach the GPU.

## Current Implementation

**Canvas-driven sizing** — layout has no fixed screen size. The canvas `size` is threaded through `RebuildStrataBatches` in `src/iced_app/render/rebuild.rs` and consumed by the strata emit / quad-builder pipeline, so WoW coords map 1:1 to canvas pixels and adapt to the window.

**Lua screen-size globals** are dynamic. `WowLuaEnv::set_screen_size()` (`src/lua_api/env_runtime.rs:75`) re-runs `install_screen_size_globals()` (`env_runtime.rs:288`), which redefines `GetScreenWidth`, `GetScreenHeight`, and `GetPhysicalScreenSize` to return the live canvas dimensions. There is no hardcoded `1280×720` fallback in the Lua surface — the values track whatever the host window passes in.

**`UI_SCALE`** is still defined as `1.0` in `src/render/texture.rs:8` and is referenced throughout `src/iced_app/` (masking, strata emit, quad builders, rebuild, render textures) when converting from unscaled WoW coordinates to display pixels. The constant is the single knob if global scale ever needs to change.

## Known Issues Fixed

1. **Hardcoded anchor override** — `main.rs` previously forced `TOPLEFT (10, -10)` on the root frame instead of using the XML-defined anchor. Removed.
2. **Screen size mismatch** — internal screen size was hardcoded; now driven by the canvas via `set_screen_size()`.
3. **Hardcoded `GetScreenWidth/Height`** — replaced by `install_screen_size_globals()` re-run on resize.
4. **Debug purple border** — removed.

## Open Items

- Y-axis convention is split between WoW (bottom-left Y-up) and the renderer (top-left Y-down). The conversion path in anchor resolution / quad emission should be documented end-to-end; see [[../investigations/anchor-resolution]] / `docs/anchor-resolution.md`.
- `CENTER` anchor behavior under live canvas resize is not covered by an automated regression.

## Key Files

| File | Role |
|------|------|
| `src/iced_app/render/rebuild.rs` | Strata rebuild — threads canvas `size` into emitters |
| `src/iced_app/strata_emit.rs`, `quad_builders_line.rs`, `masking.rs`, `update_helpers.rs`, `render_textures.rs` | Apply `UI_SCALE` when converting WoW rects to display pixels |
| `src/render/shader/pipeline.rs` | Orthographic projection `Uniforms::new(width, height)` |
| `src/render/texture.rs` | `UI_SCALE` constant |
| `src/lua_api/env_runtime.rs` | `set_screen_size`, `install_screen_size_globals` (`GetScreenWidth`/`GetScreenHeight`/`GetPhysicalScreenSize`) |

## Sources

- `SCALING.md` — short status note kept in sync with this page
- Verified against source 2026-04-26

## See Also

- [[architecture-overview]] — overall Lua/Rust system design
- [[debug-tools]] — debug overlays for verifying anchor positions
