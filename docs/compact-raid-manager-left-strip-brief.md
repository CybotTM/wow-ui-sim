# Compact Raid Manager Left Strip Brief

Status: open investigation

## Problem Statement

In the current branch, the collapsed `CompactRaidFrameManager` strip on the left looks too tall compared with WoW retail. This is visible in the fully collapsed state next to the 4-person party.

The important visual symptom is not just “a texture extends too far.” The whole visible strip reads as uniformly too large:

- the background extends too far downward
- the header area reads too tall
- the forward toggle button appears too low

That means the issue is broader than one stray child texture.

## Observed Mismatch To Preserve

These values are the core discrepancy to keep in mind:

- stored frame-tree bottom: about `y=415`
- visually painted bottom in the problematic current-branch screenshot: about `y=480`
- stored forward-toggle top: about `y=260`
- observed displayed forward-toggle top in the problematic current-branch screenshot: about `y=300`

So the logical layout state and the final painted result disagree.

### Correction: two different fixtures were being compared

A follow-up probe showed the above “logical vs painted” mismatch is an artifact of comparing two fixtures, not a render-time divergence:

- The `222x275` / toggle-top `y=260` numbers come from a **no-party baseline** (`IsInGroup=false`, manager is hidden anyway).
- The screenshot exhibiting the oversize strip is a **party-leader fixture** (`A_Admin.SetPartySize(4); A_Admin.SetPartyLeader(0)`).

Running an `--exec-lua` probe after seeding that party fixture (captured in `/tmp/crfm_probe_out.txt`) gives:

- `CompactRaidFrameManager`: `222x347`, `GetScale=1`, `GetEffectiveScale=1`
- rect: `GetLeft=-200 GetRight=22 GetTop=1060 GetBottom=713` (WoW Y-up; dump-tree y = 1200 − wow_y ⇒ top=140, **bottom=487**)
- `Background`: `222x347`, scale `1`, atlas `GM-bgOpen-party-leads` (native `222x344`)
- `toggleButtonForward`: `16x35`, scale `1`, `GetTop=904 GetBottom=869` (dump-tree y top ≈ **296**)
- `displayFrame.flowMaxPrimaryUsed = 327` ⇒ `SetHeight(usedY + 20) = 347`
- `UIParent`: `1600x1200`, `GetScale=1`, `GetEffectiveScale=1`

So in the actual screenshot fixture:

- logical manager bottom ≈ 487, visual painted bottom ≈ 480 — **match**
- logical toggle top ≈ 296, visual toggle top ≈ 300 — **match**
- every relevant scale in the chain is `1` — no scale multiplier is at play

There is no render-time divergence to solve. The oversize **is** the logical layout. The real question is why `usedY = 327` when retail's collapsed party-leader strip is visibly shorter.

## What Was Compared

The investigation compared:

- current branch
- clean `master`
- WoW retail screenshot reference

The branch-vs-retail mismatch is the real target. `master` is useful as a simulator reference point, but retail is the user-visible correctness bar.

## What Has Been Verified

### 1. The wrong thing is not just one overflowing texture

The visible strip reads as uniformly oversized. The button being too low is especially important: if the toggle is also wrong, this is not merely one background quad extending past the edge.

### 2. Basic frame-tree logical rects are not enough to explain the screenshot

In the basic `dump-tree` probe, current and `master` both reported the same collapsed manager frame/tree rects:

- manager: `222x275` at `x=-200, y=140`
- forward toggle: `16x35` at `x=-1, y=260`

So the stored frame/tree rects, by themselves, do not explain the retail mismatch.

This was not a one-off reading. The same frame/tree rect result was rechecked multiple times during the investigation and stayed consistent:

- collapsed manager logical size stayed `222x275`
- collapsed forward-toggle logical rect stayed `16x35` with top around `y=260`

So this point should be treated as established unless a new probe contradicts it.

The button mismatch is explicit:

- stored forward-toggle top: about `y=260`
- observed displayed forward-toggle top: about `y=300`

### 3. Raw emitted GPU quads were captured for the background and forward toggle

This now needs to be split into two parts: proven current-branch quad data, and what is still not proven.

Raw quad capture from `QuadBatch` in the collapsed `A_Admin.SetPartySize(4)` fixture produced:

- background frame rect: `(-200, 140) -> (22, 415)`
- background textured quad: `(-200, 140) -> (22, 415)`
- background path:
  - `Interface\hud\uigroupmanager@crop:0.219727,0.436523,0.338867,0.622070`

- forward-toggle frame rect: `(-1, 260) -> (15, 295)`
- forward-toggle textured quad: `(-1, 260) -> (15, 295)`
- forward-toggle path:
  - `Interface\hud\uigroupmanager@crop:0.977539,0.993164,0.000977,0.035156`

So for these two visible collapsed-strip elements, raw emitted textured quad bounds match the frame-tree logical rects exactly in the current branch.

What remains true:

- the final painted result in the problematic screenshot still appears lower/taller than those quad bounds would imply
- the mismatch still looks uniform across the visible collapsed strip, not isolated to one edge
- the painted bottom in the problematic screenshot is about `y=480`
- the displayed forward toggle appears around `y=300`

What is still **not** proven:

- whether some later render stage transforms these already-correct quads
- whether a different screenshot fixture than the probe fixture is the one exhibiting the visual oversize
- whether the perceived visual oversize is caused by something other than the background/toggle quads themselves

The earlier atlas-native-height theory is now weaker than before:

- the painted bottom near `y=480` is still visually close to `140 + 344 = 484`
- `344` is the native atlas height of `gm-bgopen-party-leads`
- but the captured background quad for the probed collapsed fixture is `275` tall, not `344`

So atlas-native-size rendering is **not** supported by the captured background/toggle quad data from this fixture.

### 4. The correct collapsed manager elements are displaying

This should be stated explicitly because it changed the direction of the investigation:

- not a hidden-descendants theory
- not “the wrong subtree is accidentally still visible”

The correct collapsed manager strip is what is being shown. The issue is that its final visual result is wrong relative to retail.

### 5. This is not a mask issue

The manager strip problem is unrelated to the separate portrait-mask investigation.

### 6. This is not a `UIGroupManager` asset-content problem

Both of these assets were checked:

- `textures/hud/uigroupmanager.webp`
- `/home/osso/Projects/wow/Interface/HUD/UIGroupManager.BLP`

They decode pixel-identically. So the branch bug is not caused by one source file containing different art.

### 7. This is not caused by BC-compressed rendering for this texture

`UIGroupManager.BLP` is a `BLP2 Raw3` texture, not a DXT/BC-compressed manager texture in this case. So the left-strip problem is not explained by “BC fast path corrupted this specific texture.”

## Relevant Blizzard Behavior

The collapse path in `Blizzard_CompactRaidFrameManager.lua` is important:

- `CompactRaidFrameManager_Collapse()` sets:
  - `collapsed = true`
  - `SetPoint("TOPLEFT", UIParent, "TOPLEFT", -200, -140)`
  - `displayFrame:Hide()`
  - `toggleButtonBack:Hide()`
  - `toggleButtonForward:Show()`
  - `BottomButtons:Hide()`

What it does **not** do:

- it does not explicitly shrink the frame height
- it does not reposition the forward toggle independently

The forward toggle is anchored to the manager itself:

- `RIGHT`, `x=-7`, `y=0`

So if the toggle appears too low, one likely cause is that the manager’s effective final height or visual envelope is wrong.

The other critical code path is `CompactRaidFrameManager_UpdateOptionsFlowContainer()`, which ends with:

```lua
CompactRaidFrameManager:SetHeight(usedY + 20);
```

That means the manager height is flow-driven, not fixed by the collapse function itself.

## Current Interpretation

At this point, the bug is narrower than:

- wrong source texture
- wrong mask
- one accidental overflowing texture
- simple stored-rect mismatch in the basic frame tree
- a render-time transform that paints quads at a different size than the logical rect (ruled out by scale=1 everywhere in the party-leader fixture, and matching logical/visual extents)

What remains:

- `CompactRaidFrameManager_UpdateOptionsFlowContainer()` produces `usedY = 327` (⇒ manager height 347) for the `A_Admin.SetPartySize(4); A_Admin.SetPartyLeader(0)` fixture, which is bigger than the retail reference.
- The bug is therefore in **what the flow container is accumulating**, not in how that accumulated height is painted.

Likely sub-causes to investigate next:

- children added to the flow whose `:GetHeight()` is too tall in this simulator (e.g. `raidMarkers`, `RestrictPingsDropdown`, `BottomButtons`, or any added even though they should be hidden/excluded while collapsed)
- `UpdateOptionsFlowContainer` running while `collapsed == true` and using the leader-with-party branch, when retail's flow for this exact state produces a shorter result
- a line-break/spacer contribution that differs from retail (e.g. `VerticalSpace(...)` values, `RestrictPingsLabel` / `RestrictPingsDropdown` rows, the raid-markers grid)

## Why The Button Matters

The button observation is one of the strongest constraints:

- if only the background were wrong, this could still be a texture or quad issue
- but if the forward toggle is also too low, the problem is tied to the manager strip as a whole

That is why the investigation moved away from texture-content theories and toward the collapsed manager’s layout/update lifecycle.

## What This Brief Is Explicitly Not Claiming

To avoid over-claiming:

- it is **not** claiming hidden descendants are the cause
- it is **not** claiming the source asset is wrong
- it is **not** claiming BLP preference is the bug
- it is **not** claiming the basic dump-tree rects are wrong
- it is **not** claiming every relevant quad in the full scene was captured
- it is **not** claiming the screenshot mismatch is solved just because the background/toggle quads in the probe fixture were captured

## Next Step

Effective scale has been captured (see the correction block above): every scale in the chain is `1`, and at the problematic fixture the logical rect already matches the painted result. So step 4 ("where the final painted visual extent diverges from the stored logical rect") is answered: it doesn't. The divergence was a fixture mismatch.

The remaining question is purely on the flow side. Useful probes:

1. With the party-leader fixture seeded, enumerate `displayFrame.flowFrames`/`flowFrameTypes` and record each child's `:GetWidth()` / `:GetHeight()` — this pinpoints which child inflates `usedY`.
2. Compare the same enumeration to retail's expected flow content for a 4-man party with player as leader (roles-assigned, not in raid).
3. Determine whether retail's `UpdateOptionsFlowContainer` is expected to early-exit (or skip `SetHeight`) while `collapsed == true`. Current Blizzard source in this repo does not early-exit, so this needs retail-side verification.
