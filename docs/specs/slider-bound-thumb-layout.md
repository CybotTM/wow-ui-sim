# Bound Slider thumb layout

Sized unanchored Textures bound to a parent Slider's `ThumbTexture` receive
engine geometry in `src/layout_slider_thumb.rs`. See [layout](../layout-system.md)
for the shared layout pipeline. Contained travel is simulator policy, not a claim
of native endpoint parity.

## What it must do

- [ ] Preserve thumb dimensions/scale; center on the track cross-axis and at the
  value midpoint. Increasing values move right horizontally and down vertically.
- [ ] Value, range, orientation, binding and track/thumb resize changes update
  geometry, including synchronous queries inside `OnValueChanged`.
- [ ] Query and render admission require the same real positive finite resolved
  thumb rectangle, not binding identity alone.
- [ ] Ordinary and formerly bound unanchored textures remain unresolved; unsized
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

Authored, not executed:
- `tests/widget_slider.rs`: `slider_bound_thumb_tracks_value_range_orientation_and_resize`;
  `slider_thumb_rebinding_preserves_unresolved_and_authored_anchor_controls`.
- `src/iced_app/strata_emit.rs`:
  `slider_thumb_render_rect_tracks_mutations_without_query_side_effects`.

## Known gaps (current cycle)

- [ ] Main-owned compilation, Rust/public Lua RED/GREEN and midpoint GPU GREEN.
  Missing-thumb GPU RED records zero magenta pixels for the bound 18×24 thumb.
  Public Lua query RED capture is main-owned; no result is claimed here.
- [ ] Integration and actual rendering after live mutations remain unverified.

## Out of scope

Native endpoint inset/parity claims require native evidence. No vendor Lua edits,
input/drag/reverse-mode rules, foreign-parent thumb reassignment, generic visibility
patches, or non-Slider layout behavior changes belong to this bounded slice.
