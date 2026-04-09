# Class Talents Right-Side Artifact

Status: resolved as a screenshot artifact, not a live class-talent render-batch bug.

## Problem

The current class-talent screenshot still shows a separate visual oddity on the right-hand side of
the talents window. It is not the hero-spec icon:

- the hero icon still renders at the expected top-center location
- the old hero-spec bottom-right marker was retired as stale evidence
- this remaining artifact appears lower and farther right, near the bottom portion of the
  Protection tree / lower edge of the talents panel

## Current Evidence

Current reproduction command:

```bash
env LD_LIBRARY_PATH=target/debug ./target/debug/wow-sim \
  --no-addons \
  --no-saved-vars \
  --exec-lua 'PlayerSpellsUtil.ToggleClassTalentFrame()' \
  screenshot -o /tmp/hero-current.webp
```

Observed in `/tmp/hero-current.webp` on April 9, 2026:

- the hero-spec orb is top-center, above the middle hero tree, which matches current layout and
  render evidence
- a separate gold circular visual remains visible near the lower-right portion of the class-talent
  UI, beneath the Protection tree area

This initially looked like a different bug from the retired hero-spec investigation in
[`docs/hero-spec-icon-bug.md`](/syncthing/Sync/Projects/wow/wow-ui-sim/docs/hero-spec-icon-bug.md),
but the raw render-batch investigation below ruled that out.

## Resolution

The apparent lower-right artifact does not correspond to any live frame content in the raw
render batch for the filtered class-talent screenshot.

The investigated bbox was `(1134, 664) -> (1173, 708)` in the current `1600x1200`
`PlayerSpellsFrame` screenshot-path render. For that region:

- there are no overlapping non-background texture requests
- there are no overlapping mask texture requests
- there are no overlapping solid-color quads
- the raw `render_to_image(...)` output for that bbox matches a marble-only baseline exactly

That means the visible gold blob in the saved screenshot is not backed by a live class-talent
frame, texture request, or render quad. The remaining evidence points to the lossy WebP
screenshot encoding path, not the UI render batch itself.

## Known Non-Causes

The following were ruled out by the combined hero-spec and raw-batch investigations:

- `HeroTalentsContainer`
- `HeroSpecButton.Icon1`
- other children of the hero-spec subtree
- the old historical point `(1000, 610)`, which now resolves to `framegeneral/ui-background-marble`
- the current lower-right bbox in the raw filtered class-talent render batch

## Follow-Up

If the screenshot output still matters, the next investigation should move to the screenshot
export pipeline itself:

- confirm whether WebP quality `15` is introducing the visible blob in that region
- decide whether screenshot debugging should use a lossless format or higher-quality encode option
- treat this separately from live class-talent rendering bugs
