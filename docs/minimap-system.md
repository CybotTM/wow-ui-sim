# Minimap System

## Current State

The minimap now has a **basic circular placeholder render path**, but it is still far from a WoW-like
minimap. The simulator currently renders a placeholder map texture and applies shader-based circle
clipping, so the old "solid dark green rectangle" state is no longer accurate. The remaining gaps
are about real minimap state, real mask handling, and actual content overlays.

### What Works

- **Widget type**: `WidgetType::Minimap` in the enum (`src/widget/mod.rs`)
- **XML parsing**: `<Minimap>` elements parse correctly via `FrameElement::Minimap`
- **Frame creation**: `CreateFrame("Minimap", ...)` creates a proper frame
- **Child hierarchy**: Zoom buttons, backdrop, border texture render as children
- **Basic map quad**: `build_minimap_quads()` emits a sim-bundled placeholder map (`Interface\\AddOns\\SimCommands\\textures\\minimap-placeholder.webp`, 256x256 stylized zone render)
- **Mask clipping**: minimap quads use `UIMinimapMask` (or `SetMaskTexture()` state) through the GPU mask-texture path
- **Zoom state**: `SetZoom()` / `GetZoom()` persist and clamp zoom instead of returning constants
- **Lua surface coverage**: minimap methods exist, with many still as no-op compatibility stubs
- **Global registration**: `Minimap` is registered as a Lua global in `global_frames.rs`

### What's Missing

- **Real map content**: the minimap still renders a fixed placeholder texture, not zone/map data
- **Mask-accurate contour edge cases**: `SetMaskTexture` drives the GPU mask now, but the simulator
  still does not model all minimap content layers that real WoW clips through the mask
- **Texture inputs**: `SetBlipTexture`, `SetIconTexture`, `SetPOIArrowTexture`, and related
  setters are still stubs
- **Player arrow**: Not rendered
- **Blips/POIs**: Not rendered
- **Blob overlays**: quest/task/arch blob setters are still no-ops

## Key Files

| File | Purpose |
|------|---------|
| `src/widget/mod.rs:23-42` | `WidgetType::Minimap` enum variant |
| `src/iced_app/quad_builders_textures.rs` | `build_minimap_quads()` — placeholder map + mask texture clipping |
| `src/iced_app/quad_builders.rs` | Dispatch: `WidgetType::Minimap => build_minimap_quads(...)` |
| `src/lua_api/frame/methods/map_frames.rs` | Minimap methods: real zoom and texture state plus remaining blob stubs |
| `src/lua_api/globals/global_frames.rs` | `Minimap` global registration |
| `src/render/shader/quad.wgsl` | WGSL shader — current `FLAG_CIRCLE_CLIP` path |
| `Interface/BlizzardUI/Blizzard_Minimap/Mainline/Minimap.xml` | Blizzard minimap XML definition |

## How WoW Clips the Minimap to a Circle

WoW uses **three layers** stacked on top of each other:

1. **Map content** (bottom): rectangular texture showing the zone map
2. **Mask texture** (middle): `UIMinimapMask.BLP` — a white circle on black, applied via `SetMaskTexture()`. The black areas make the map transparent, clipping it to the circle shape. The mask follows the compass frame contour (slightly indented at the 4 cardinal points).
3. **Border frame** (top): `UIMinimap.BLP` (`ui-hud-minimap-frame` atlas) — the decorative compass ring with gold cardinal markers. Purely cosmetic overlay, transparent both inside and outside the ring.

The border does **not** do the masking. The corners of the border texture are transparent, so without the mask, a rectangular map would show through. The mask is the essential piece.

### Mask Texture: `UIMinimapMask.BLP`

- Atlas entry: `ui-hud-minimap-frame-mask` (256x256, full UV)
- Source: `Interface/HUD/UIMinimapMask.BLP`
- White = opaque (show map), Black = transparent (hide map)
- Shape matches the compass frame contour (not a perfect circle — has indentations at N/S/E/W)

### Border Frame: `UIMinimap.BLP`

- Atlas entry: `ui-hud-minimap-frame` (438x460, sub-region of 512x512 texture)
- Source: `Interface/HUD/UIMinimap.BLP`
- Contains: compass ring, gold cardinal markers, other UI icons in the atlas
- Rendered via `MinimapCompassTexture` child at OVERLAY layer, sublevel 2

## Available Textures

All minimap textures are loaded directly from CASC as BLPs. The old
`textures/minimap/` webp mirror is gone — there is no conversion step.

| BLP File | Atlas | Purpose |
|----------|-------|---------|
| `Interface/HUD/UIMinimap.BLP` | `ui-hud-minimap-frame` | Compass border frame (512x512), rendered as `MinimapCompassTexture` child via XML |
| `Interface/HUD/UIMinimapMask.BLP` | `ui-hud-minimap-frame-mask` | Circular clip mask (256x256), not yet sampled — the shader uses `FLAG_CIRCLE_CLIP` instead |
| `Interface/HUD/UIMinimapBackground.BLP` | — | 32x32 tileable backdrop, currently unused (sim ships its own placeholder map) |

### Sim-bundled placeholder

The simulator ships a 256x256 stylized zone-map placeholder at
`Interface/AddOns/SimCommands/textures/minimap-placeholder.webp` (resolved through the addon-dir
tier of `TextureManager::resolve_path`). Replace this with frame-driven content once
`SetMaskTexture` and the rest of the minimap texture setters land.

## Rendering Architecture

### Current Flow

```
build_minimap_quads(batch, bounds, frame)
  → batch.push_textured_path(bounds, "Interface\\AddOns\\SimCommands\\textures\\minimap-placeholder", ...)
  → batch.set_extra_flags(4, FLAG_CIRCLE_CLIP)
```

### Shader Pipeline

The WGSL shader (`src/render/shader/quad.wgsl`) processes quads with:
- **Vertex attributes**: position, tex_coords, color, tex_index, flags
- **tex_index**: `-1` = solid color, `0-3` = tiered texture atlas, `4` = glyph atlas
- **flags**: blend mode (alpha or additive)
- **circle clip**: `FLAG_CIRCLE_CLIP` uses `local_uv` + `smoothstep()` to fade alpha outside the
  circular radius

The shader does **not** currently sample a minimap mask texture. The circular clip is enough for a
basic display, but it cannot reproduce the real WoW minimap contour.

## What Is Still Needed

For a **basic circular minimap display**, most of the foundational work is already present:

1. a `Minimap` widget type
2. a rendered map quad
3. circular shader clipping
4. persistent zoom state

What is still needed to move from "basic placeholder minimap" to "useful minimap" is:

1. **Real minimap texture state**
   - store `SetMaskTexture`, `SetBlipTexture`, `SetIconTexture`, `SetPOIArrowTexture`,
     `SetCorpsePOIArrowTexture`, and `SetStaticPOIArrowTexture` on the frame instead of dropping
     them
2. **Real minimap content selection**
   - replace the fixed `Interface\\AddOns\\SimCommands\\textures\\minimap-placeholder` fill with a
     frame-driven texture input or a simulator-owned dynamic minimap source
3. **Player/POI rendering**
   - render the player arrow and basic POI/blip overlays on top of the map quad
4. **Optional WoW-accurate mask**
   - if visual fidelity matters beyond a circular placeholder, add mask-texture sampling so the
     minimap follows the actual `UIMinimapMask` contour instead of a perfect circle

## WoW-Accurate Follow-Up: Map Texture with Mask

Render a static map texture clipped by the mask texture, matching WoW's approach.

### Recommended Approach: Dual-Texture Mask in Shader

Add a shader feature that samples a mask texture alongside the main texture, multiplying the output alpha by the mask value.

**Shader change** — add a mask texture binding and flag:
```wgsl
const FLAG_MASK_CLIP: u32 = 0x100u;  // bit 8

// Additional vertex attribute for mask UV (or reuse tex_coords if mask covers same bounds)
// In fs_main, after computing color:
if (in.flags & FLAG_MASK_CLIP) != 0u {
    let mask_value = textureSampleLevel(mask_texture, texture_sampler, in.tex_coords, 0.0);
    color.a *= mask_value.r;  // white = show, black = hide
}
```

**Rust change** — in `build_minimap_quads()`:
1. Load `UIMinimapMask.BLP` as a texture (or use the atlas entry `ui-hud-minimap-frame-mask`)
2. Load a static map image as the main texture
3. Emit a textured quad with the `FLAG_MASK_CLIP` bit set
4. The shader multiplies alpha by the mask, clipping to the contoured circle

**Pros**: Matches WoW's actual masking behavior, smooth anti-aliased edges from the mask texture, shape matches the compass frame contour exactly.

**Cons**: Requires a dedicated mask texture binding in the shader pipeline.

### Alternative: Smoothstep Circle in Shader

If adding a mask texture binding is too complex, approximate with a mathematical circle:

```wgsl
const FLAG_CIRCLE_CLIP: u32 = 0x100u;

if (in.flags & FLAG_CIRCLE_CLIP) != 0u {
    let centered = in.tex_coords * 2.0 - 1.0;
    let dist = length(centered);
    color.a *= 1.0 - smoothstep(0.96, 1.0, dist);
}
```

**Pros**: No extra texture needed, single flag bit.
**Cons**: Perfect circle doesn't match the compass frame's indented contour at cardinal points.

### Map Content Source

Options for the static map texture:

1. **User-provided image**: Load an image (like the Westguard Keep map) as a texture
2. **Procedural noise**: Generate terrain-like texture (green/brown patches)
3. **Solid with overlays**: Keep the dark fill but add compass ring and player arrow

Option 1 requires loading an arbitrary image into the GPU atlas. A dedicated texture slot outside the tiered atlas would be simplest.

### Minimal Follow-Up Steps

1. Store minimap texture setters as real frame state instead of no-op stubs
2. Replace the fixed placeholder map path with frame-driven minimap content
3. Render the player arrow/basic overlays using the stored minimap texture inputs
4. If contour fidelity matters, add mask-texture sampling for `UIMinimapMask`
5. Keep the existing compass/border children rendering on top
