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
- `src/iced_app/slider_scale_tests.rs`: authored inherited/local-scale CPU render-list cases, registered through the existing test-only `strata_emit` module.

## Tests asserting this spec

Executed bounded integration cases in `tests/widget_slider.rs` cover value/range/orientation/resize geometry and rebinding/unresolved/authored-anchor controls. See [central receipt](../baselines/README.md#verified-scoped-receipts--2026-10-15) for runtime-source attribution and the full Slider group. A separate saved exact run of `slider_thumb_render_rect_tracks_mutations_without_query_side_effects` passed 1/1 on source136: CPU render-list inclusion and horizontal/vertical/value/range/size mutation, without Lua geometry queries priming layout. Independent report: `/home/osso/.local/state/wow-ui-sim/verification/slider-query-free-saved-next/report.md`. Scale remains 1; this is not GPU or whole-current-tree proof.

Two new library cases in `src/iced_app/slider_scale_tests.rs` remain **UNEXECUTED**: ancestor scales0.5/2 and thumb-local scales0.5/1.5 at ancestor2, both orientations, each min/mid/max. They warm layout before scale changes. Within each of eight fixtures, min/mid/max CPU render-list assertions precede that fixture's first Lua geometry query: 24 endpoint observations containing 48 track/thumb rectangles in total. Query agreement and unchanged rendered rectangles are checked only at final/max. The first post-scale value change also dirties layout, so these cases do not isolate scale-only invalidation. This is authored simulator-policy coverage, not a production counterexample, GPU proof or native endpoint claim.

## Known gaps (current cycle)

- [x] Bounded public-query/mutation and callback geometry tests passed on runtime source136.
- [x] Separate earlier [GPU receipt](../rendering-pipeline.md#combined-feedback-gpu-receipt--2026-10-15) establishes midpoint thumb visibility at its recorded source.
- [x] Source136 query-free CPU render-list mutation regression passed separately.
- [ ] Execute the two new non-unit-scale CPU render-list cases; source-oracle inspection is not runtime acceptance.
- [ ] Native endpoint parity, full integration and current-source live-render acceptance remain open.

## Out of scope

Native endpoint inset/parity claims require native evidence. No vendor Lua edits,
input/drag/reverse-mode rules, foreign-parent thumb reassignment, generic visibility
patches, or non-Slider layout behavior changes belong to this bounded slice.
