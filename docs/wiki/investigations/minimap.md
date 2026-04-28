# Minimap

Current state of the minimap implementation: basic circular placeholder render, not a functional WoW minimap.

## What Works

- `WidgetType::Minimap` enum variant and XML parsing
- `CreateFrame("Minimap", ...)` creates a proper frame
- Child hierarchy renders (zoom buttons, backdrop, border texture)
- `build_minimap_quads()` emits `Interface\\AddOns\\SimCommands\\textures\\minimap-placeholder` (sim-bundled 256×256 webp) with `FLAG_CIRCLE_CLIP` shader clipping
- `SetZoom()` / `GetZoom()` persist and clamp zoom
- `Minimap` registered as Lua global

## What's Missing

- Real map content (fixed placeholder texture, not zone/map data)
- `SetMaskTexture` is a no-op stub — Blizzard code cannot drive clip shape
- Current clip is a mathematical circle, not the real `UIMinimapMask` contour (slightly indented at N/S/E/W)
- Player arrow, blips/POIs, blob overlays — all stubs

## How WoW Clips the Minimap

Three layers stacked:
1. Map content (rectangular texture)
2. `UIMinimapMask.BLP` (`ui-hud-minimap-frame-mask`, 256×256) — white circle on black, applied via `SetMaskTexture()`. Black areas make map transparent.
3. `UIMinimap.BLP` (`ui-hud-minimap-frame`, 438×460) — decorative compass ring, cosmetic only

The border does **not** do the masking. The mask is the essential piece.

## Path to Useful Minimap

1. Store `SetMaskTexture`, `SetBlipTexture`, `SetIconTexture`, `SetPOIArrowTexture` on the frame instead of dropping them
2. Replace fixed placeholder map path with frame-driven texture input
3. Render player arrow and basic POI/blip overlays
4. Optional: add mask-texture sampling for `UIMinimapMask` to match the real contour

## Shader Approach for WoW-Accurate Mask

Add `FLAG_MASK_CLIP` bit. In `build_minimap_quads()`, load `UIMinimapMask` and emit a textured quad with the flag set. Shader multiplies `color.a` by `mask.r`. Requires a dedicated mask texture binding.

The existing `FLAG_CIRCLE_CLIP` smoothstep approach is a simpler alternative but produces a perfect circle, not the compass contour.

## Key Files

- `src/iced_app/quad_builders_textures.rs` — `build_minimap_quads()`
- `src/lua_api/frame/methods/methods_misc.rs` — minimap methods
- `src/render/shader/quad.wgsl` — `FLAG_CIRCLE_CLIP` path

## Sources

- [minimap-system.md](../../minimap-system.md) — full system description and follow-up plan

## See Also

- [[mask-texture]] — the MaskTexture system used for real WoW minimap masking
