# Widget System

Every UI element is a `Frame` struct stored in the `WidgetRegistry`. The `WidgetType` enum discriminates 18 widget kinds; creation establishes default children, strata inheritance, and parent-child linkage.

## Frame Struct (~140 fields, `src/widget/frame.rs`)

Key field groups:

**Identity & hierarchy** — `id: u64` (atomic counter), `widget_type`, `name: Option<String>`, `parent_id`, `children: Vec<u64>`, `children_keys: HashMap<String, u64>` (named child refs).

**Rendering order** — `frame_strata`, `frame_level: i32`, `alpha: f32`, `scale: f32`, `draw_layer`, `draw_sub_layer`. The `BLIZZARD` input token is ignored rather than modeled as a drawable strata tier. XML strata literals remain non-fixed. `PARENT` copies the parent's effective strata during XML creation rather than becoming a persistent enum value. Generic propagation from parent `SetFrameStrata()` and later `SetParent()` operations accounts for subsequent effective-strata changes.

**Input** — `mouse_enabled`, `mouse_motion_enabled`, `keyboard_enabled`, `propagate_keyboard_input`, `movable`, `resizable`.

**FontString fields** — `text`, `font`, `font_size` (default 14.0), `font_outline` (None/Outline/ThickOutline), `text_color` (default gold), `justify_h/v`, `word_wrap`, `max_lines`.

**Texture fields** — `texture`, `color_texture`, `vertex_color`, state textures (`normal_texture`, `pushed_texture`, `highlight_texture`, `disabled_texture`) each with `_tex_coords` companions, `tex_coords`, `atlas_tex_coords`, `blend_mode`, `nine_slice_layout/atlas`, `horiz_tile/vert_tile`, `is_mask`.

**Widget-specific** — Slider, StatusBar, EditBox, ScrollFrame, Cooldown each have dedicated field groups.

## WidgetType Enum (18 types, `src/widget/mod.rs`)

Frame, Button, FontString, Texture, EditBox, ScrollFrame, Slider, CheckButton, StatusBar, Cooldown, Model (stub), ModelScene (stub), PlayerModel, ColorSelect, MessageFrame, SimpleHTML (stub), GameTooltip, Minimap.

`from_str()` maps WoW aliases: "ItemButton" → Button, "ScrollingMessageFrame" → MessageFrame.

## WidgetRegistry (`src/widget/registry.rs`)

```rust
pub struct WidgetRegistry {
    widgets: HashMap<u64, Frame>,
    names: HashMap<String, u64>,
    render_dirty: Cell<bool>,
}
```

`get_mut()` sets `render_dirty`. `take_render_dirty()` atomically checks and clears. `would_create_anchor_cycle()` is a BFS reachability check used by `SetPoint`.

## Default Children (`src/lua_api/globals/create_frame.rs`)

**Button/CheckButton** — 4 Textures (NormalTexture Artwork, PushedTexture Artwork, HighlightTexture Highlight additive, DisabledTexture Artwork) + 1 FontString (Text at Overlay). All textures fill the button via TOPLEFT+BOTTOMRIGHT anchors. `mouse_enabled = true`.

**Slider** — FontStrings: Low, High, Text; Texture: ThumbTexture.

**ItemButton** — 6 Textures (icon, searchOverlay, IconBorder, IconOverlay, IconOverlay2, ItemContextOverlay) + 2 FontStrings (Count, Stock).

**GameTooltip** — Inserts tooltip data into `SimState.tooltips`; sets strata to TOOLTIP.

## Strata Propagation Evidence

- XML `frameStrata` literals, including `PARENT`, report non-fixed state. `PARENT` resolves to the parent's current effective strata during XML creation.
- Base-to-derived template resolution keeps the first strata literal: a base `HIGH` remains effective when a derived template declares `PARENT`.
- Parent `SetFrameStrata()` recursively updates the tested non-fixed XML descendants and grandchildren, including explicit XML `MEDIUM` literals. `SetParent()` recomputes their effective strata through the moved subtree.
- These generic non-fixed propagation rules explain the observed `PARENT` behavior; the probe does not establish an intrinsic persistent dynamic `PARENT` state.
- Runtime-fixed behavior using `SetFixedFrameStrata(true)` remains unproven.

## Button Text Rendering and Three-Slice Issue

Button text is emitted twice: once by `build_button_quads()` as part of the non-region frame (renders before all child regions), and once by the child `Text` FontString at Overlay draw layer (renders after Artwork textures).

Three-slice buttons (ThreeSliceButtonTemplate) define their background as child Textures at BACKGROUND draw layer. These render after the button frame but before the child FontString — meaning the step-1 text gets covered. The child FontString should fix this, but its default CENTER anchor gives it zero width, so it renders nothing.

**Fix**: replace the child Text FontString's single CENTER anchor with fill-parent anchors (`add_fill_parent_anchors()`), so `resolve_multi_anchor_edges` computes proper width from the parent button bounds.

## Visibility

`Show()` fires `OnShow` recursively on visible children. `IsVisible()` walks the parent chain; `IsShown()` checks only the frame's own `visible` flag.

## Sources

- [FrameStrataProbe](../../../docs/addons/FrameStrataProbe/README.md) — retail XML `PARENT`, parent `SetFrameStrata()` cascade, and `SetParent()` observations
- [widget-system.md](../../widget-system.md) — Frame struct, WidgetType, WidgetRegistry, default children, strata
- [button-text-rendering.md](../../button-text-rendering.md) — three-slice rendering order problem and fix

## See Also

- [[layout-system]] — uses Frame.anchors to compute screen positions
- [[rendering-pipeline]] — dispatches quad emission per WidgetType
- [[event-system]] — Frame.registered_events and script handler storage
