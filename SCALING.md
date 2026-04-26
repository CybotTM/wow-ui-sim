# UI Scaling and Coordinate Systems

Canonical doc: [`docs/wiki/design/scaling-coordinates.md`](docs/wiki/design/scaling-coordinates.md). This file is a short status pointer.

## WoW Coordinate System

- Bottom-left origin, Y increases upward
- `(0, 0)` = bottom-left, `(screenWidth, screenHeight)` = top-right
- Default anchor point when none specified: `TOPLEFT`
- Wowless reference size: `1280×720`

The renderer runs in iced top-left Y-down screen space. Y is flipped in the orthographic projection (`Uniforms::new` in `src/render/shader/pipeline.rs`).

## Current Status

- `UI_SCALE = 1.0` in `src/render/texture.rs:8`. Applied throughout `src/iced_app/` (masking, strata_emit, quad builders, rebuild, render textures).
- `GetScreenWidth` / `GetScreenHeight` / `GetPhysicalScreenSize` are dynamic — installed by `install_screen_size_globals()` (`src/lua_api/env_runtime.rs:288`), re-run from `set_screen_size()` whenever the canvas changes.
- Layout has no fixed screen size: canvas `size` flows through `RebuildStrataBatches` in `src/iced_app/render/rebuild.rs`.
- `main.rs` no longer hardcodes `TOPLEFT (10, -10)`; XML anchors are honored.
- Debug purple border has been removed.

## Open

- Document the WoW Y-up → renderer Y-down conversion end-to-end (anchor resolution).
- Add a `CENTER`-anchor regression test against a live canvas resize.
