# sim-render

Profile-neutral markup stripping, GPU atlases/quads and canvas texture drawing.
Only `gui` is forwarded: it enables the existing optional iced/wgpu/bytemuck
code without forwarding any client bundles or asset/profile selection.
Shared logging lives in sim-texture, preserving one set of process statics.

Font measurement arithmetic is shared in `font`; font construction and shaping,
glyph emission and headless orchestration stay in root. Font construction selects
profile/CASC data; glyphs use that font type. Moving those requires a new font
loader interface or wrapper, which this cut does not introduce. Font/markup
measurement remains below Lua. GPU pipeline/primitive/program and their existing
upload values move without introducing a texture service. Five manager/crop tests
stay in composition; renderer-only tests move with the GPU code, retaining its
cfg(test) vertex snapshots. GPU code depends only on sim-texture and sim-widget.

The existing BC support test swap hook is public and unconditional so root
renderer-crossing tests can still use the same leaf atomic. Runtime behavior is
unchanged. Other boundary visibility is widened only where modules used to share
a crate. Root re-exports retain original public paths.
