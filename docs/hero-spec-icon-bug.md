# Hero Spec Icon Position Bug

## Problem

The hero talent spec icon (large circular paladin fire icon with ring border) renders at the **bottom-right** of the talent panel instead of **top-center** where it belongs.

## Frame Hierarchy

```
PlayerSpellsFrame (scale=0.88)
  └─ ClassTalentsFrame (TalentsFrame)
      ├─ ButtonsParent (__tpl_25748, 1418x681, clipChildren=true)
      │   └─ [talent node buttons positioned via ApplyPosition]
      └─ HeroTalentsContainer (__tpl_25774, 200x800)
          │  anchor: TOP -> ButtonsParent:TOP offset(0,0)
          ├─ HeroSpecLabel (.HeroSpecLabel) "TEMPLAR"
          │    anchor: BOTTOM -> parent:TOP offset(0,23)
          ├─ HeroSpecButton (__tpl_25780, 108x108)
          │    anchor: TOP -> parent:TOP offset(0,-102)
          │    ├─ Icon1 (talentsheroclassicons) setAllPoints
          │    ├─ IconMask (common-mask-circle) setAllPoints
          │    └─ Border (talents-heroclass-ring-mainpane, 192x192)
          │         anchor: CENTER -> parent:CENTER offset(0,-2)
          └─ CurrencyFrame (30x30) "PV" / "0"
               anchor: CENTER -> HeroSpecButton:BOTTOM offset(0,-3)
```

Key: HeroTalentsContainer is a **sibling** of ButtonsParent (not a child), anchored to its TOP.

## Lua Positioning

`ClassTalentsFrameMixin:UpdateSpecBackground()` (Blizzard_ClassTalentsFrame.lua:204):
```lua
local heroContainerOffset = specVisuals and specVisuals.heroContainerOffset or 0;
self.HeroTalentsContainer:SetPoint("TOP", self.ButtonsParent, heroContainerOffset, 0);
```

4-arg SetPoint: `(point, relativeTo, offsetX, offsetY)` — relativePoint defaults to "TOP".

## Investigation Findings

### dump-tree shows correct position

```
__tpl_25774 [Frame] (176x704) x=711, y=61   anchor: TOP -> __tpl_25748:TOP -> (800,61)
  __tpl_25780 [Button] (95x95) x=752, y=151  anchor: TOP -> parent:TOP -> (800,163)
    .Icon1 [Texture] (95x95) x=752, y=151
    .Border [Texture] (168x168) x=715, y=115
```

### screenshot renders at bottom-right (~x=1000, y=610)

### No stale layout_rect

The `layout_rect` cached on every frame in this subtree **matches** the freshly computed rect (no `[layout_rect=...]` stale annotations in dump). This holds true even when dumping AFTER `ensure_layout_rects()` runs in the screenshot path.

### Identical loading sequence

Both dump-tree and screenshot follow the same startup:
1. `fire_startup_events` (OnShow -> UpdateSpecBackground -> SetPoint)
2. `apply_post_event_workarounds`
3. `rebuild_anchor_index`
4. `process_pending_timers`
5. `fire_one_on_update_tick`
6. `sleep(2s)` + `run_extra_update_ticks(3)`

### Rendering pipeline

`collect_sorted_frames` (frame_collect.rs:102) reads `f.layout_rect` directly — confirmed correct.
`emit_all_frames` (render.rs:190-192) converts to screen bounds with `UI_SCALE=1.0` — no transform.

### Quad emission matches layout rect

`tests/hero_talents_render.rs::hero_spec_icon_and_mask_quads_match_layout_rect` now verifies the
actual `QuadBatch` output for the active hero spec button:

- `HeroSpecButton.Icon1` emits exactly one textured quad request
- that request's vertex bounds exactly match `Icon1`'s computed layout rect
- `HeroSpecButton.IconMask` emits exactly one mask request
- that mask request's vertex bounds also exactly match the same layout rect

Observed values in the full UI harness:

```
Icon1 layout rect: x=481.58 y=111.41 w=60.83 h=60.83
Icon1 textured quad bounds: (481.58, 111.41) -> (542.42, 172.24)
IconMask quad bounds:       (481.58, 111.41) -> (542.42, 172.24)
```

That rules out divergence in:

- anchor/layout resolution
- visible-frame collection
- texture quad emission
- mask quad clipping setup

### Atlas crop request matches atlas metadata

`tests/hero_talents_render.rs::hero_spec_icon_crop_request_matches_atlas_entry` also verifies that
the emitted cropped texture request for `HeroSpecButton.Icon1` matches the atlas database entry for
`talents-heroclass-paladin-lightsmith` exactly.

Current emitted request:

```
Interface\talentframe\talentsheroclassicons@crop:0.395020,0.492676,0.790039,0.985352
```

Atlas DB entry:

```
talents-heroclass-paladin-lightsmith
  file: Interface\talentframe\talentsheroclassicons
  left/right/top/bottom: 0.395020 / 0.492676 / 0.790039 / 0.985352
```

That rules out divergence in:

- `SetAtlas()` atlas lookup for `Icon1`
- `atlas_tex_coords` storage on the frame
- `remap_atlas_crop()` crop-key generation

### Loaded crop content and GPU sampling still match

`tests/hero_talents_render.rs::hero_spec_icon_full_ui_render_matches_isolated_crop_render`
pushes the investigation one step further down the pipeline:

- it loads the exact emitted crop request for `HeroSpecButton.Icon1` via `load_texture_or_crop()`
- it verifies several stable interior sample points in that crop stay substantially opaque
- it renders the exact same crop request in isolation through `render_to_image()`
- it renders the full class-talent UI through that same headless GPU path
- it compares several stable interior sample points inside the circular icon (`top-center`,
  `center-left`, `center`, `center-right`, `bottom-center`)

The full UI render matches the isolated crop render at those interior points, while the loaded crop
itself is also non-empty and opaque there. That rules out divergence in:

- the extracted `talentsheroclassicons` sub-region content for the active Lightsmith icon
- CPU-side crop extraction via `TextureManager::load_sub_region()`
- downstream GPU texture upload / sampling for the actual `Icon1` quad at those interior points

## What's Ruled Out

- **Stale layout_rect**: Confirmed correct after ensure_layout_rects
- **SetPoint parsing**: 4-arg form correctly parsed (anchor shows TOP->TOP offset 0,0)
- **UI_SCALE**: Is 1.0, no transform
- **Duplicate frames**: Only 3 instances of talentsheroclassicons texture, all within HeroSpecButton
- **Pan offset**: Pan system moves individual talent buttons, not ButtonsParent position
- **clipChildren**: ButtonsParent clips, but HeroTalentsContainer is a sibling not a child

## Remaining Hypotheses

1. **Texture content / stale source asset**: The crop request is correct, but the sampled pixels in the local `talentsheroclassicons` texture could still be stale or wrong for that atlas region.
1. **GPU-side mask sampling interaction**: The icon quad and mask quad line up in the CPU batch, and interior icon samples match the loaded crop, but the shader-side mask path could still distort edge pixels or leave another artifact visible elsewhere.
2. **Misidentified artifact**: The bottom-right visual may not be `Icon1` at all. Another hero-talent texture in the same subtree could be the one rendering unexpectedly, while `Icon1` itself is already correct through crop extraction and interior GPU sampling.

## Debug Tools Added

`--dump-tree` flag on `screenshot` subcommand:
```bash
wow-sim screenshot --dump-tree __tpl_25774   # dump subtree after ensure_layout_rects
wow-sim screenshot --dump-tree               # dump all (no filter)
```

## Next Steps

- Identify which on-screen artifact in the screenshot corresponds to which texture request in the `HeroTalentsContainer` subtree
- Check whether the suspicious bottom-right visual is emitted by another hero-talent texture rather than `HeroSpecButton.Icon1`
- If needed, isolate mask-edge sampling separately from interior icon sampling
