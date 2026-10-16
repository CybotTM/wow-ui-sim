# Supplied animation transforms and Translation layout motion

Status: A1 folded after B1/B3, B4 and C11 against HEAD e7a3f2109; tests/passive scaffold only in first patch, producer separate. RED/GREEN unexecuted. Missing queries have neutral passive returns, never runtime skips.

Translation, Scale and Rotation preserve supplied Lua/XML configuration. Translation uses the simulator's existing presentation-offset layout model. This staged contract is **simulator policy, not native parity**; no tests have run. Public sources establish signatures and consumption, not interpolation/restoration semantics. See [Path](path-control-point.md), [VertexColor](vertex-color-animation.md) and [query lifecycle](animation-query-lifecycle.md) for existing model boundaries.

## What it must do

- [ ] Translation `SetOffset(x,y)` / `GetOffset()` preserve signed/fractional supplied values, exactly two returns, independent per animation.
- [ ] Scale `SetScaleFrom/To(x,y)` / `GetScaleFrom/To()` preserve separate supplied pairs. `SetScale/GetScale` preserve a separate legacy two-number pair; **stage policy:** no inferred linkage to endpoint pairs and no scalar Frame-scale mutation. Scale/Rotation `SetOrigin(point,x,y)` / `GetOrigin()` preserve the three supplied values independently.
- [ ] Rotation `SetDegrees/GetDegrees` and `SetRadians/GetRadians` address one angle: degree/radian conversion, without wrapping. This mutual conversion is a simulator policy, not inferred from Wowless's independent stored fields.
- [ ] Normal addon XML loading applies `offsetX/Y`, `scaleX/Y`, `fromScaleX/Y`, `toScaleX/Y`, `degrees` and `<Origin point x y>` through the same public state. No synthetic `fromScale`/`toScale` scalar XML aliases are introduced.
- [ ] **Stage policy:** one positive-duration, order-1 Translation on a target in a NONE-looping group adds interpolated displacement to its pre-play presentation offset in existing screen-layout axes, at unit scale. NONE is linear; OUT uses existing VertexColor quadratic easing. Owner/default and childKey targets work; child-target motion leaves owner/sibling rectangles and target dimensions unchanged. Pause freezes, resume preserves baseline, Stop/natural finish/hidden-owner stop restore baseline, and replay snapshots changed baseline. `SetToFinalAlpha(true)` remains alpha-only; it does not retain offsets. No native restoration or coordinate-sign claim.

## How it works

- [Layout system](../layout-system.md)
- [Animation Path contract](path-control-point.md)
- [VertexColor contract](vertex-color-animation.md)
- [Animation lifecycle](animation-query-lifecycle.md)

## Implementation inventory

Proposed producer only, not installed. Tests-stage `transforms.rs` attaches passive query methods before template application: zero offset/angles, identity scale pairs, CENTER/zero origin. These are scaffold policy, not native defaults; existing HEAD setters and absent Translation motion remain unchanged. Producer replaces the passive module, without a second attachment/publication path:

| Path | Role |
|---|---|
| `src/lua_api/animation.rs` | Supplied transform fields and shared saved motion offsets. |
| `src/lua_api/frame/methods/button_anchor_hierarchy/animations/transforms.rs` | Type-local configuration/query methods. |
| `animations/{creation,path_runtime,play,runtime}.rs` and `animations.rs` | Attach endpoints before template application; reuse target resolution, offset application/restoration and existing lifecycle. |
| `animations/vertex_color.rs` | Share the already-modeled progress/easing calculation without changing color output. |
| `src/xml/types_animation.rs`, `src/loader/helpers_anim.rs` | Existing parsed attributes and setter emission, unchanged. |
| `src/layout.rs` | Existing `anim_offset_x/y` projection, unchanged. |

## Tests asserting this spec

`tests/animation_transforms.rs`, discovered into `--test integration`, seven staged cases:

| Test | Observable assertions |
|---|---|
| `a1_translation_offset_round_trips` | Signed/fractional pairs, return arity, mutation, independent instance. |
| `a1_scale_pairs_and_origin_round_trip` | Distinct from/to/legacy pairs, mutation, independent origins, owner scale unchanged. |
| `a1_rotation_units_and_origin_round_trip` | Signed/unwrapped angle conversions, origin mutation, independent instance. |
| `a1_xml_transform_values_reach_queries` | Real addon TOC/XML loader, all listed attributes and explicit Scale/Rotation origins. |
| `a1_translation_ticks_restore_and_replay` | Child layout at quarter/half/three-quarter time, nonzero baseline, owner/sibling isolation, pause/resume, Stop/natural finish, changed-baseline replay, final-alpha independence. |
| `a1_translation_xml_out_moves_only_declared_child` | Literal CatalogShop Translation tag in reduced fixture, half-time OUT layout, owner isolation and natural restoration. |
| `a1_translation_owner_and_hidden_stop` | Default owner target moves; Hide stops/restores before explicit Stop. |

All cases are shared-source and ungated. Retail first; other profiles require separate execution. Reduced public-tag fixture is not whole-vendor lifecycle proof.

## Known gaps (current cycle)

- [ ] Execute tests-only against recorded HEAD and retain genuine RED before applying producer; current failures are source-predicted value/layout assertions only. Existing setter-callability tests in `animation_anim.rs` remain unchanged and earn no new feature credit.
- [ ] Compile and execute producer plus preservation filters; no build, format, type-check, runtime or startup proof is claimed by staging.
- [ ] Scale/Rotation visual effects remain unmodeled. Frame scalar scaling and static texture rotation exist but are not per-axis region animation/pivot hooks; this stage does not repurpose them or claim effective-size/pixel motion.

## Out of scope

Native defaults/coercion/errors/security/aliasing, full supported-profile certification, non-unit scale, native coordinate conversion, multi-animation/group composition, ordered translation accumulation, loop/reverse geometry proof, zero-duration behavior, callback ordering, SetTarget-family redesign, LineTranslation/LineScale, GPU/quads, and vendor changes. Neutral new field defaults (zero offset/angle, identity scale pairs, CENTER origin) are implementation initialization, not native-contract assertions. Existing Path behavior must remain unchanged; its tests are preservation controls, not new A1 credit.
