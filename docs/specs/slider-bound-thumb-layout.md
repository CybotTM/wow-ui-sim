# Bound Slider thumb layout

Sized unanchored Textures bound to a parent Slider's `ThumbTexture` receive
engine geometry in `src/layout_slider_thumb.rs`. See [layout](../layout-system.md)
for the shared layout pipeline. Contained travel is simulator policy, not a claim
of native endpoint parity.

## What it must do

- [x] Preserve thumb dimensions; center on the track cross-axis and at the
  value midpoint. Increasing values move right horizontally and down vertically.
- [ ] Preserve non-default thumb/ancestor scale across query/render geometry;
  selected executed cases use scale 1.
- [x] At scale 1, value, range, orientation, binding and track/thumb resize changes
  update query geometry, including synchronous queries inside `OnValueChanged`.
- [ ] Query and render admission require the same real positive finite resolved
  thumb rectangle, not binding identity alone.
- [x] Ordinary and formerly bound unanchored textures remain unresolved; unsized
  bound textures remain unresolved. Explicit thumb anchors stay authoritative.

### Limited simulator policy — not native endpoint parity

Contained travel uses track extent minus thumb extent, without extra endpoint
insets. Oversized thumbs stay centered on the travel axis. Collapsed or non-finite
normalization centers the thumb. These policies are not measured native behavior.

Cached Blizzard consumers establish direction, not intrinsic endpoint math:
`Shared/Slider/MinimalSlider.xml` places Forward on the right and
`MinimalSlider.lua::OnStepperClicked` increments forward.
`SecureScrollTemplates.lua::UIPanelScrollBarScrollDownButton_OnClick` increments
value; `UIPanelScrollBar_OnValueChanged` passes it to vertical scroll. Separately
anchored modern `Shared/Scroll/ScrollBar.lua` thumbs also move right/down with
increasing percentage. All paths are within cached `Blizzard_SharedXML`.

## How it works

- [Shared layout](../layout-system.md)
- [Rendering](../rendering-pipeline.md)

## Implementation inventory

- `src/layout_slider_thumb.rs`: thumb geometry and resolved-rectangle eligibility.
- `src/layout.rs`: anchorless thumb layout producer.
- `src/layout_render_eligibility.rs`, `src/iced_app/strata_emit.rs`: shared admission.
- `src/lua_api/rect_geometry.rs`: resolve layout before query admission.
- `src/lua_api/frame/methods/widgets/slider.rs` and `slider/thumb_texture.rs`:
  mark Slider layout dirty after modeled mutations.

## Tests asserting this spec

Executed bounded integration cases in `tests/widget_slider.rs` cover value/range/orientation/resize geometry and rebinding/unresolved/authored-anchor controls. See [central receipt](../baselines/README.md#verified-scoped-receipts--2026-10-15) for runtime-source attribution and the full Slider group. `slider_thumb_render_rect_tracks_mutations_without_query_side_effects` is not among those selected cases; no new proof claimed for it.

## Known gaps (current cycle)

- [x] Bounded public-query/mutation and callback geometry tests passed on runtime source136.
- [x] Separate earlier [GPU receipt](../rendering-pipeline.md#combined-feedback-gpu-receipt--2026-10-15) establishes midpoint thumb visibility at its recorded source.
- [ ] Query-free render mutation regression, native endpoint parity, full integration and current-source live-render acceptance remain open.

## Out of scope

Native endpoint inset/parity claims require native evidence. No vendor Lua edits,
input/drag/reverse-mode rules, foreign-parent thumb reassignment, generic visibility
patches, or non-Slider layout behavior changes belong to this bounded slice.
