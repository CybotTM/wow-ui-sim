# Minimap Map Ring Alignment

The active minimap bug is the map texture/mask/ring alignment. It is not the SimCommands minimap button, and future debugging must not redirect this issue to addon button placement without fresh evidence.

## Content

### Reasoning Correction

The mistake was treating a nearby visible control as the likely cause before proving the rendered minimap geometry. The user's screenshot showed the minimap map texture protruding past the circular ring aperture, so the correct target is the minimap render path: map texture bounds, circular or texture mask, and ring alignment.

Do not reopen the SimCommands button explanation for this bug. The SimCommands button can overlap the minimap visually, but it is a separate frame/control and was not established as the cause of the map texture protruding beyond the ring.

### Current Evidence

Mists Blizzard layout gives `MinimapCluster` a 192x192 frame, `Minimap` a 140x140 frame, and `MinimapBackdrop`/`MinimapBorder` a 192x192 ring around it. The simulator currently renders the minimap through `build_minimap_quads()` with a placeholder texture and synthetic `FLAG_CIRCLE_CLIP`, while the `Frame.minimap_mask_texture` state set by `Minimap:SetMaskTexture()` is not consumed by that renderer.

The investigation should start from backing render/model state:

- Verify the real default minimap mask for the active client profile.
- Make `Minimap` rendering respect the correct mask/clip aperture.
- Compare the rendered map texture against the ring opening, not against addon buttons or debug controls.

## Sources

- [PLAN.md](../../../PLAN.md) — active task explicitly states this is not the SimCommands minimap button.
- [quad_builders_textures.rs](../../../src/iced_app/quad_builders_textures.rs) — current minimap renderer uses synthetic circle clipping.
- [map_frames.rs](../../../src/lua_api/frame/methods/map_frames.rs) — `SetMaskTexture` stores minimap mask state.

## See Also

- [[minimap]] — broader minimap system status.
- [[rendering-pipeline]] — texture and quad rendering path.
