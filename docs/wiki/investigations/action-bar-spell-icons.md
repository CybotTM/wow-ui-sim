# Action Bar Spell Icons

Four compounding bugs caused spell icon textures to be invisible on action bar buttons despite the action bar frame rendering correctly.

## Root Causes

### 1. No-op `SetDrawLayer` override (FIXED)

`widget_model.rs:add_model_scene_rendering_stubs` registered a no-op `SetDrawLayer` after the real one in `methods_texture.rs`. In mlua, later `add_method` wins. `BaseActionButtonMixin_OnLoad` calling `self.NormalTexture:SetDrawLayer("OVERLAY")` was silently ignored.

**Fix**: Removed the no-op from `widget_model.rs`.

### 2. Draw order within same layer (FIXED)

Icon and SlotArt were both at BACKGROUND sub-level 0. SlotArt (opaque dark wing texture) had a higher widget ID and rendered on top of the icon. WoW's engine renders earlier-created textures on top within the same draw layer/sublevel.

**Fix**: Reversed the ID tiebreaker in `intra_strata_sort_key` — lower IDs now sort last (render on top). Added `type_flag` so FontStrings always render above Textures in the same layer.

### 3. `SetDrawLayer` sublevel parameter ignored (FIXED)

`SetDrawLayer(layer, sublevel)` ignored the second argument; `GetDrawLayer()` always returned 0 for sublevel.

**Fix**: Both now read/write `frame.draw_sub_layer`.

### 4. XML `textureSubLevel` not parsed (FIXED)

`<Layer textureSubLevel="N">` was not parsed, so all textures in the same layer got sublevel 0.

**Fix**: `LayerXml` now parses `textureSubLevel` and passes it through to texture/fontstring creation.

## Render Sort Key

```
(frame_level, region_flag, draw_layer, draw_sub_layer, type_flag, Reverse(id))
```

- `region_flag`: 0 for frames, 1 for textures/fontstrings
- `type_flag`: 0 for Texture, 1 for FontString
- `Reverse(id)`: Earlier-created regions render on top within the same layer

## Action Button Layer Structure

```
BACKGROUND:  icon → IconMask → SlotBackground → SlotArt
OVERLAY:     NormalTexture (transparent center) → PushedTexture
```

## Files Modified

- `src/lua_api/frame/methods/widget_model.rs`
- `src/lua_api/frame/methods/methods_texture.rs`
- `src/iced_app/frame_collect.rs`
- `src/xml/types_elements.rs`
- `src/loader/xml_frame.rs`, `xml_texture.rs`, `xml_fontstring.rs`

## Sources

- [action-bar-spell-icons.md](../../action-bar-spell-icons.md) — full investigation and fix

## See Also

- [[mask-texture]] — IconMask clips icon to rounded shape
- [[on-update-dirty]] — rendering pipeline and dirty tracking
