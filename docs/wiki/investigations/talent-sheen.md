# Talent Sheen Animation

Every class talent button has a "sheen" — a translucent light sweep (150px, 6.5s) that slides left-to-right across the button border on a 22-second synchronized cycle.

## Bug

White rectangles appear over talent buttons/connections when talents change. Cause: `BorderSheen` (the `talents-sheen-node` atlas texture) renders without masking — either because the MaskTexture pipeline is broken, or the mask frame has 0×0 dimensions.

## Architecture

Each button has two texture children from `ClassTalentButtonArtTemplate`:

- `BorderSheen` — `talents-sheen-node` atlas, `alphaMode: ADD`, starts off-screen left, animated via Translation
- `BorderSheenMask` — per-template shape atlas (circle, square, choice diamond, etc.), `CLAMPTOBLACKADDITIVE` wrap, masks `BorderSheen`

Without the mask working, `BorderSheen` renders as a full unclipped rectangle sweeping across all buttons every 22 seconds.

## Sheen Visibility by State

Sheen is **visible** (alpha=1) on: Normal, Maxed, Disabled, Locked, DisplayError states.
Sheen is **hidden** (alpha=0) on: Gated, Selectable, Invisible, RefundInvalid states.

Visibility is controlled via alpha, not Play/Stop, to preserve sync timing across all nodes.

## Sync Mechanism

All buttons share `syncKey = ClassTalentBorderSheenSyncKey`. `SyncedAnimGroupMixin:PlaySynced()` computes offset via `GetTime()` so all buttons sweep in unison:

```lua
local timeSinceSyncedStart = GetTimeSinceSyncTimeForKey(syncKey)
local syncedOffset = timeSinceSyncedStart % self:GetDuration()
self:Play(reverse, syncedOffset)
```

## Animation Timing

- `offsetX=150`, `duration=6.5s`, `startDelay=5s`, `endDelay=10.5s`
- Total cycle: **22 seconds**

## Per-Template Mask Atlases

| Template | Mask Atlas |
|---|---|
| Choice | `talents-node-choice-sheenmask` |
| Circle | `talents-node-circle-sheenmask` |
| Square | `talents-node-square-sheenmask` |
| LargeSquare | `talents-node-choiceflyout-square-sheenmask` |
| LargeCircle | `talents-node-choiceflyout-circle-sheenmask` |

## Sources

- [talent-sheen-animation.md](../../talent-sheen-animation.md) — full architecture and bug description

## See Also

- [[mask-texture]] — MaskTexture rendering pipeline
- [[glow-effects]] — `alphaMode: ADD` additive blending used by BorderSheen
