# VertexColor animation

Patch 10.2.5 introduces texture vertex-color interpolation. [Literal source](../../data/patch-api/sources/10.2.5-api-changes.txt), lines 15–47, specifies individual RGBA interpolation and equivalent two-second red/blue BOUNCE Lua/XML examples. The active retail cache's `Blizzard_APIDocumentationGenerated/SimpleAnimVertexColorAPIDocumentation.lua` specifies four endpoints: setters take one non-nil ColorMixin `colorRGBA`; getters return one non-nil ColorMixin `colorRGBA`. [Publication audit](patch-10-2-5-publication-sweep.md) remains separate from behavioral proof.

## What it must do

- [ ] `SetStartColor`/`SetEndColor` accept complete RGBA color objects; `GetStartColor`/`GetEndColor` return color objects exposing `GetRGBA`, not four numbers. Endpoint state is independent of texture state.
- [ ] Animate each RGBA component from start to end without color-space conversion; apply to texture owner or resolved animation child target, leaving other textures unchanged.
- [ ] Run the literal two-second BOUNCE example through both directions and repeat; reverse playback starts at the end of the existing group timeline.
- [ ] XML `StartColor`/`EndColor` configure the same endpoints and produce the same output as Lua; omitted XML alpha is one, matching `CreateColor(r,g,b)`.
- [ ] **Modeled inference:** snapshot endpoint channels and return independent color objects. Endpoint defaults are opaque white; invalid ordinary arguments require a table with numeric RGBA channels. Native aliasing/default/coercion/error policy is unknown.
- [ ] **Modeled inference:** smoothing changes time progress, not channel color space: NONE linear, IN quadratic ease-in, OUT quadratic ease-out, IN_OUT piecewise quadratic. Start/end delays use the existing group scheduler. Native smoothing curve parity is unknown.
- [ ] **Modeled inference:** save original target vertex color once per playback; pause freezes and resume retains the snapshot. Stop, hidden-owner stop, natural finish, and pending Finish restore it; replay captures the new baseline. `SetToFinalAlpha` controls alpha only and does not retain vertex color. Native vertex-color restoration policy is unknown; this extends existing alpha restoration semantics without inventing a final-color flag.

## How it works

- [Animation factory contract](animation-factory.md).
- [Lua API architecture](../lua-api.md).
- [Widget state and rendering](../widget-system.md).

## Implementation inventory

- `src/lua_api/animation.rs`: animation/group endpoint and saved-target state.
- `src/lua_api/frame/methods/button_anchor_hierarchy/animations/vertex_color.rs`: RGBA endpoint snapshots, independent color objects, smoothing and texture target application/restoration.
- `src/lua_api/frame/methods/button_anchor_hierarchy/animations/`: playback lifecycle and direction-aware boundary reflection; `animations.rs` connects stop/restart.
- `src/lua_api/frame/methods/button_anchor_hierarchy/mod.rs`: method registration.
- `src/xml/types_animation.rs`: parsed endpoint elements.
- `src/loader/helpers_anim.rs`: shared XML endpoint application.

## Tests asserting this spec

- `tests/vertex_color_animation.rs`: endpoint objects, channel interpolation, bounce/reverse, smoothing/delays, target isolation, pause/resume/restoration/replay and actual addon XML loading.

## Known gaps (current cycle)

- [ ] Parent-confirmed RED at `6c4f39abc3b8a3987e7d45baca80271fa233091d`: six selected tests failed at missing `SetStartColor`; no downstream assertions reached. Parent artifact: `verification/patch-pass2-red-current/red-20261013T002128.187277Z/` (not available in this implementer's local checkout). Production implementation awaiting logical commit and authorized targeted GREEN; no PASS claim.
- [ ] Explicit XML alpha, ordered-group composition and large-delta crossing proof require bounded tests before claiming those boundaries.

## Out of scope

Native/historical/security parity, invalid-argument compatibility, multi-group conflict resolution, nontexture color animation, other animation features, vendor changes and animation-system redesign. Publication fixtures and frozen historical evidence are parent-owned reconciliation, not behavioral proof.
