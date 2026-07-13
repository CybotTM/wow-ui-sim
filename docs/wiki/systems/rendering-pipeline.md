# Rendering Pipeline

The rendering pipeline traverses the frame hierarchy, batches geometry into a `QuadBatch`, uploads it to the GPU, and draws via custom WGSL shaders. A separate headless path renders screenshots without a window.

## QuadBatch System

**QuadVertex** (`src/render/shader/quad.rs`) — 36 bytes per vertex:

| Field | Type | Purpose |
|-------|------|---------|
| `position` | `[f32; 2]` | Screen pixels, top-left origin |
| `tex_coords` | `[f32; 2]` | UV 0.0–1.0 |
| `color` | `[f32; 4]` | RGBA premultiplied |
| `tex_index` | `i32` | Atlas tier 0–3, 4=glyph, -1=solid, -2=pending |
| `flags` | `u32` | Blend mode bits |

Each quad = 4 vertices + 6 indices (two triangles). `BlendMode`: Alpha (default) or Additive (glow/highlights — zeroes output alpha so GPU produces `src + dst`).

Key batch methods: `push_solid()`, `push_textured_uv()`, `push_textured_path()` (deferred), `push_three_slice_h_path()`, `push_nine_slice()`, `push_tiled()`, `push_border()`.

## GPU Pipeline (`src/render/shader/pipeline.rs`)

Orthographic projection matrix with Y-flip: `[2/w, 0, 0, 0], [0, -2/h, 0, 0], [-1, 1, 0, 1]`. Blend state: `SrcAlpha / OneMinusSrcAlpha`. Topology: `TriangleList`. Cull: none.

Prepare phase: resize power-of-two buffers if needed, upload vertices and indices. Render phase: set viewport/scissor, bind pipeline + uniforms + textures, `draw_indexed`.

## WGSL Shaders (`src/render/shader/quad.wgsl`)

Fragment shader samples the four-tier texture atlas; `tex_index < 0` uses vertex color directly (solid quads). All colors are premultiplied: `color = vec4f(rgb * a, a)`. Additive quads zero output alpha. UV coords are clamped to `[0, 0.9999]` to prevent edge bleeding. No mipmapping (`textureSampleLevel(..., 0.0)`).

## Tiered Texture Atlas (`src/render/shader/atlas.rs`)

| Tier | Cell size | Grid | Capacity |
|------|-----------|------|----------|
| 0 | 64×64 | 64×64 | 4,096 |
| 1 | 128×128 | 32×32 | 1,024 |
| 2 | 256×256 | 16×16 | 256 |
| 3 | 512×512 | 8×8 | 64 |

All tiers back a 4096×4096 texture. Glyph atlas is a separate 2048×2048 texture at binding 5. Tier selection: smallest tier that fits; falls back to largest with scaling if oversized.

## Strata and Draw Layer Sorting

Sort key (primary → tie-breaker): frame strata → frame level → region vs non-region (non-regions first) → draw layer → widget ID.

Strata order: `WORLD < BACKGROUND < LOW < MEDIUM < HIGH < DIALOG < FULLSCREEN < FULLSCREEN_DIALOG < TOOLTIP`.  
Draw layer order: `BACKGROUND < BORDER < ARTWORK < OVERLAY < HIGHLIGHT`.

## Alpha Propagation

`collect_ancestor_visible_ids()` BFS from roots: `eff_alpha = parent_alpha * frame.alpha`. Frames with `eff_alpha <= 0` are skipped entirely.

## Hit Testing (`src/iced_app/frame_collect.rs`, `src/iced_app/hit_grid.rs`, `src/iced_app/view.rs`)

`frame_collect` owns the shared `HitOrderKey` and collects visible, mouse-enabled frames in render order. The GUI-only `hit_grid` module imports that key for spatial indexing; headless builds do not compile the grid or its GUI dependency tree. Queries iterate in reverse (highest strata first), returning the first frame containing the cursor point. Several system frames (UIParent, Minimap, WorldFrame, chat frames) are excluded.

## Performance

- Quad batch rebuilt only when `quads_dirty` flag is set or screen resized
- Hit-test list cached, invalidated on layout changes
- Glyph atlas uploaded to GPU only when dirty
- Frame time smoothed via EMA (alpha=0.33, ~5-sample window)

## Software Rendering

Headless path (`src/render/software.rs`): creates a wgpu device without a window, renders to `Rgba8UnormSrgb` texture with `LoadOp::Clear`, reads back pixels via `copy_texture_to_buffer` + `map_async` + `poll`.

## Sources

- [rendering-pipeline.md](../../rendering-pipeline.md) — QuadBatch, shaders, atlas, hit testing, alpha, text

## See Also

- [[layout-system]] — produces LayoutRect consumed by quad building
- [[widget-system]] — WidgetType dispatch per frame type
- [[texture-atlas]] — TextureManager and atlas resolution feeding the GPU atlas
- [[addon-compatibility]] — Docker headless release build contract
