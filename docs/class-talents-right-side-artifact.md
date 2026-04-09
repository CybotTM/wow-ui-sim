# Class Talents Right-Side Artifact

Status: open, separate from the retired hero-spec icon report.

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

This should be treated as a different bug from the retired hero-spec investigation in
[`docs/hero-spec-icon-bug.md`](/syncthing/Sync/Projects/wow/wow-ui-sim/docs/hero-spec-icon-bug.md).

## Known Non-Causes

The following were already ruled out by the hero-spec investigation:

- `HeroTalentsContainer`
- `HeroSpecButton.Icon1`
- other children of the hero-spec subtree
- the old historical point `(1000, 610)`, which now resolves to `framegeneral/ui-background-marble`

## Next Steps

- capture the exact bounds of the remaining right-side artifact in the current screenshot
- identify which frame / texture request covers that actual region in the current batch
- determine whether it belongs to the Protection tree, another class-talent subpanel, or an
  unrelated overlapping frame
