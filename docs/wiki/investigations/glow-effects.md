# Glow Effects and Blend Modes

Status: additive blending is implemented end-to-end. One remaining gap: `SetBorderBlendMode()` has no Rust handler.

## What Works

- XML `alphaMode="ADD"` / `blendMode="ADD"` parsed and stored on `Frame.alpha_mode` / `Frame.blend_mode`
- `SetBlendMode()` / `GetBlendMode()` persist raw WoW mode strings on the frame
- Additive quads in `quad.wgsl`: premultiply RGB, set alpha to 0 — works with premultiplied-alpha pipeline
- Button highlights use `BlendMode::Additive`
- `GlowEmitterMixin` + `EffectFactory` lifecycle, pooling, positioning, and alpha pulse animations work

## Remaining Gap

`GlowEmitter.lua` calls `self.NineSlice:SetBorderBlendMode("ADD")` but no Rust handler exists. Currently a missing method error.

## Data Flow

```
XML alphaMode="ADD" ──> Frame.alpha_mode + Frame.blend_mode
Lua SetBlendMode()  ──> Frame.blend_mode
render.rs           ──> QuadVertex.flags carry blend mode
quad.wgsl additive  ──> premultiply rgb, zero alpha
pipeline.rs         ──> premultiplied alpha blend: src + dst
```

## Additive Blend in Shader

For additive quads: `color = vec4(color.rgb * color.a, 0.0)`. With the premultiplied-alpha pipeline, this adds source light to the destination without overwriting it.

A single-pipeline approach (no second pipeline variant) is used. The old 1.5x alpha-boost workaround is gone.

## WoW Blend Modes

| Mode | GPU blend |
|---|---|
| `BLEND` | `src * src.a + dst * (1 - src.a)` |
| `ADD` | `src * src.a + dst` |
| `MOD` | `src * dst` |
| `DISABLE` | `src` (opaque) |
| `ALPHAKEY` | Discard if alpha < threshold |

Only `ADD` is needed now. Others can be added with additional pipeline variants.

## Sources

- [glow-plan.md](../../glow-plan.md) — implementation plan and current state

## See Also

- [[talent-sheen]] — `BorderSheen` uses `alphaMode: ADD`
- [[mask-texture]] — glow frames often paired with mask textures
