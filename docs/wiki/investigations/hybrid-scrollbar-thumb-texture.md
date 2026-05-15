# Hybrid Scrollbar Thumb Texture

## Summary

`HybridScrollBarTemplate` declares its thumb in Blizzard XML with
`<ThumbTexture name="$parentThumbTexture" ... parentKey="thumbTexture">`.
The simulator already creates the intrinsic slider `ThumbTexture` child, so the
runtime template path must configure that existing child from XML rather than
creating a second texture with the same resolved name.

## Root Cause

The runtime template path applied child `<Frames>` and layer textures, but did
not apply slider-specific `<ThumbTexture>` elements. A temporary
HybridScrollBar-specific fallback hid the issue by creating placeholder
children, but those placeholders had no Blizzard file, size, or texcoords.

The regression test also missed the real path because `env_with_shared_xml()`
was looking for the removed `Interface/BlizzardUI` tree. Tests now use the same
Blizzard UI cache path as the simulator and fall back only if the cache is
missing.

## Fix

`apply_thumb_texture()` resolves direct or inherited `<ThumbTexture>` XML,
reuses `parent:GetThumbTexture()` when present, applies texture source, size,
texcoords, color, visibility, global name, and parent key, then binds it through
`SetThumbTexture`. The HybridScrollBar-specific synthetic fallback was removed;
Blizzard XML now creates/configures the scrollbar pieces.

