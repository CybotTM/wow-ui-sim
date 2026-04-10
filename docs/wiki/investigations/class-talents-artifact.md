# Class Talents Artifact

A gold circular visual appeared near the lower-right of the class-talent panel. Investigation ruled out all live frame content as the source.

## Finding

The artifact does not correspond to any live frame in the render batch. Investigated bbox `(1134, 664) -> (1173, 708)` in the 1600×1200 `PlayerSpellsFrame` screenshot render:

- No overlapping non-background texture requests
- No overlapping mask texture requests
- No overlapping solid-color quads
- Raw `render_to_image()` output for that bbox matches a marble-only baseline exactly

**Conclusion**: The visible gold blob is a lossy WebP screenshot encoding artifact, not a UI render batch issue.

## What Was Ruled Out

- `HeroTalentsContainer` and all its children
- `HeroSpecButton.Icon1`
- Any other child of the hero-spec subtree
- The old historical point `(1000, 610)` — now resolves only to `framegeneral/ui-background-marble`

These were eliminated by raw render-batch inspection and the hiding test from the hero-spec investigation.

## Follow-Up

If the screenshot output matters, investigate the export pipeline:
- Confirm whether WebP quality 15 introduces the blob in that region
- Consider lossless format or higher-quality encode for screenshot debugging

This is separate from any live rendering bug.

## Reproduction Command

```bash
env LD_LIBRARY_PATH=target/debug ./target/debug/wow-sim \
  --no-addons --no-saved-vars \
  --exec-lua 'PlayerSpellsUtil.ToggleClassTalentFrame()' \
  screenshot -o /tmp/hero-current.webp
```

## Sources

- [class-talents-right-side-artifact.md](../../class-talents-right-side-artifact.md) — raw batch investigation

## See Also

- [[hero-spec-icon-bug]] — related hero-spec investigation that ruled out the hero subtree
