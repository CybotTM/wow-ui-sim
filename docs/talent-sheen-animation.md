# Talent Sheen Animation

## Overview

Every class talent button has a "sheen" — a translucent light sweep that slides left-to-right across the button border, creating a subtle shine effect. The sheen uses a MaskTexture to clip to the button's shape (circle, square, choice diamond, etc.).

**Bug**: White rectangles appear over talent buttons/connections when talents change. This is the sheen texture (`BorderSheen`) rendering without proper masking or rendering when it shouldn't be visible.

## Architecture

### XML Structure (ClassTalentButtonArtTemplate)

```
ClassTalentButtonArtTemplate
├── BorderSheen (Texture, parentKey)
│   ├── atlas: "talents-sheen-node"
│   ├── alphaMode: ADD
│   ├── anchor: RIGHT relative to LEFT (starts off-screen left)
│   └── Anim (AnimationGroup, looping=REPEAT, synced)
│       ├── syncKey: ClassTalentBorderSheenSyncKey
│       └── Translation: offsetX=150, startDelay=5, endDelay=10.5, duration=6.5
└── BorderSheenMask (MaskTexture, parentKey)
    ├── atlas: per-template sheenMaskAtlas (see below)
    ├── hWrapMode: CLAMPTOBLACKADDITIVE
    ├── vWrapMode: CLAMPTOBLACKADDITIVE
    ├── anchor: CENTER
    └── MaskedTextures: [BorderSheen]
```

Source: `Blizzard_ClassTalentButtonTemplates.xml:7-36`

### Per-Template Mask Atlas (sheenMaskAtlas KeyValues)

Each button template specifies its own mask shape:

| Template | sheenMaskAtlas |
|---|---|
| ClassTalentButtonChoiceTemplate | talents-node-choice-sheenmask |
| ClassTalentButtonCircleTemplate | talents-node-circle-sheenmask |
| ClassTalentButtonSquareTemplate | talents-node-square-sheenmask |
| ClassTalentButtonLargeSquareTemplate | talents-node-choiceflyout-square-sheenmask |
| ClassTalentButtonLargeCircleTemplate | talents-node-choiceflyout-circle-sheenmask |
| ClassTalentButtonCapstonePipCircleTemplate | talents-node-circle-sheenmask |

Source: `Blizzard_ClassTalentButtonTemplates.xml:61-159`

### Animation Details

The sheen animation is a `Translation` with:
- `offsetX="150"` — sweeps 150px right
- `duration="6.5"` — 6.5 second sweep
- `startDelay="5"` — waits 5s before starting
- `endDelay="10.5"` — waits 10.5s after completing
- `looping="REPEAT"` — cycles continuously
- Total cycle: 5 + 6.5 + 10.5 = **22 seconds**

All sheens are synchronized via `SyncedAnimGroupTemplate` with key `ClassTalentBorderSheenSyncKey` so they sweep in unison across all visible buttons.

### Synced Animation (SyncedAnimGroupMixin)

`PlaySynced()` (in `AnimationTemplates.lua:52-58`) uses `GetTime()` to compute offset, so all buttons sharing the same `syncKey` sweep together:
```lua
function SyncedAnimGroupMixin:PlaySynced(reverse, syncKey)
    local timeSinceSyncedStart = GetTimeSinceSyncTimeForKey(syncKey)
    local syncedOffset = timeSinceSyncedStart % self:GetDuration()
    self:Play(reverse, syncedOffset)
end
```

## When Sheen Is Visible

### Lifecycle

- `OnShow()` → `BorderSheen.Anim:PlaySynced()` — starts on button show
- `OnHide()` → `BorderSheen.Anim:Stop()` — stops on button hide
- `UpdateStateBorder(visualState)` → sets alpha based on visual state (hides rather than stopping to keep sync)

Source: `Blizzard_ClassTalentButtonTemplates.lua:9-31`

### Alpha by Visual State

`ClassTalentUtil.GetSheenAlphaForVisualState()` returns:

| Visual State | Sheen Alpha | Meaning |
|---|---|---|
| Normal | 1 | Spent but not maxed |
| Maxed | 1 | Fully ranked |
| Disabled | 1 | Can't interact |
| Locked | 1 | Prereqs not met |
| DisplayError | 1 | Error state |
| **Gated** | **0** | Gate threshold not met |
| **Selectable** | **0** | Available to purchase |
| **Invisible** | **0** | Hidden node |
| **RefundInvalid** | **0** | Can't refund |

Source: `Blizzard_ClassTalentUtil.lua:102-112`

**Key insight**: Sheen is visible on buttons in Normal/Maxed/Disabled/Locked/DisplayError states. It is HIDDEN on Gated/Selectable/Invisible/RefundInvalid. The sheen is a continuous ambient effect, not a commit-triggered animation.

The visibility is controlled via alpha (Show/Hide) rather than Play/Stop — this preserves sync timing across all nodes.

## Commit-Time Animations (Separate System)

These are distinct from the sheen and only fire on "Apply Changes":

### SetCommitVisualsActive (pre-commit)
- Plays `backgroundAnims` (background pulse)
- Shows commit flash on `stagedPurchaseNodes`
- Source: `Blizzard_ClassTalentsFrame.lua:949-979`

### SetCommitCompleteVisualsActive (post-commit)
- Plays `commitFlashAnims:Restart()`
- Calls `PlayPurchaseEffectOnNodes()` with delayed per-node effects
- Uses FxModelScene (3D particle effects)
- Source: `Blizzard_ClassTalentsFrame.lua:981-1039`

### PlayPurchaseCompleteEffect (per-button)
- Plays `PurchaseCompleteAnim` on talent buttons
- Only on `TalentButtonCircularGlowTemplate` (inherits AnimateWhileShownTemplate)
- Source: `Blizzard_TalentButtonArt.lua:234-259`

## MaskTexture Requirement

The sheen relies on MaskTexture to clip to the button shape. Without masking, `BorderSheen` (the `talents-sheen-node` atlas) renders as a full unclipped rectangle that sweeps across the screen. Key rendering behaviors:

1. `BorderSheenMask` has `is_mask=true` → should NOT render as a visible quad
2. `BorderSheen` has `mask_textures` containing the mask ID → rendering applies mask UV sampling
3. `CLAMPTOBLACKADDITIVE` wrap mode means areas outside the mask atlas are fully transparent

If masking is broken, the sheen renders as white/bright rectangles sliding across every visible talent button on a 22-second cycle.

## Related Files

- `Blizzard_ClassTalentButtonTemplates.xml` — Template structure (sheen texture + mask + animation)
- `Blizzard_ClassTalentButtonTemplates.lua` — OnShow/OnHide/UpdateStateBorder
- `Blizzard_ClassTalentUtil.lua:102-112` — SheenAlphaByVisualState table
- `Blizzard_SharedXML/AnimationTemplates.lua:52-58` — SyncedAnimGroupMixin
- `docs/mask-texture-system.md` — MaskTexture rendering pipeline
