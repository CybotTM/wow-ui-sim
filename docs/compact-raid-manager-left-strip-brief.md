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

### Effective-scale capture

An `--exec-lua` probe was used to capture scales and resolved rects at runtime. Every relevant scale in the chain is `1`:

- `CompactRaidFrameManager`: `GetScale=1`, `GetEffectiveScale=1`
- `CompactRaidFrameManager.Background`: `GetScale=1`, `GetEffectiveScale=1`
- `CompactRaidFrameManager.toggleButtonForward`: `GetScale=1`, `GetEffectiveScale=1`
- `CompactRaidFrameManager.displayFrame`: `GetEffectiveScale=1`
- `UIParent`: `GetScale=1`, `GetEffectiveScale=1` (`1600x1200`)

No scale multiplier is compounding anywhere in the chain, so the oversize is **not** explained by a scale factor. That rules out the scale-divergence theory.

### Two fixtures, same forward-toggle quad

`toggleButtonForward` emitted vertices in each fixture:

| Fixture | Quad rect | Size | Notes |
|---|---|---|---|
| `SetPartySize(4)` only (no leader) | `(-1, 260) → (15, 295)` | 16×35 | isLeader=false, `usedY≈255`, manager `222x275` |
| `SetPartySize(4) + SetPartyLeader(0)` (screenshot) | `(-1, 296) → (15, 331)` | 16×35 | isLeader=true, `usedY=327`, manager `222x347` |

Same `16x35` size in both (size never stretches). The top moves from `y=260` to `y=296` because the manager grew from `275` to `347` tall and the toggle is anchored `RIGHT x=-7 y=0` (vertical center of manager).

### Closing out the render-divergence theory

Visual observation of the button in the screenshot fixture: approximately `y=300 → y=335`.
Emitted `QuadBatch` vertices for that same fixture: `(-1, 296) → (15, 331)`.

These agree within visual-estimation error (±4 px). **There is no post-emission Y translation.** The earlier "+40 px shift" write-up was a fixture-confusion artifact: it compared the no-leader *quad* (`y=260`) against a leader *screenshot* (`y≈300`). With matched fixtures, quad ≡ paint.

Combined with the effective-scale capture (all scales `=1`), this rules out:

- a render-pipeline Y translation
- a projection/viewport offset
- a coordinate-inversion reference-height bug
- a scale multiplier

The bug is entirely on the logical-layout side: the manager ends up `347` tall in the party-leader fixture, and the render pipeline paints that logical rect faithfully.

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

Ruled out by direct evidence:

- wrong source texture
- wrong mask
- one accidental overflowing texture
- a scale-multiplier bug (effective-scale capture: chain is all `1`)
- a render-side Y translation (live-GUI vertex capture matches the live-GUI visual within ±4 px for the toggle in the leader fixture)
- over-sized flow children in the sim: every object the flow measures matches retail XML exactly (buttons `40x40`, `raidMarkers` `222x99`, `RestrictPingsLabel` `158x0`, `RestrictPingsDropdown` `120x25` from `WowStyle1DropdownTemplate`, `BottomButtons` `160x53`)
- a divergent vendored source: `vendor/wow-ui-source` is at tag `12.0.5` and `Blizzard_CompactRaidFrameManager.lua` is byte-identical to the live retail copy at `~/Projects/wow/Interface/AddOns/Blizzard_CompactRaidFrames/` — so retail runs the exact same flow logic we do

## Root Cause

The "too tall" visual is an **upstream-state bug, not a render/layout bug**.

`SimState.party_leader_index` defaults to `None` in `src/lua_api/state.rs:180`, and `src/lua_api/globals/group_queries.rs:172` documents that `party_leader_index = None` means the player is the leader. So any seeded party (including the brief's `A_Admin.SetPartySize(4)` fixture, even without explicit `SetPartyLeader`) makes `UnitIsGroupLeader("player") == true`.

`CompactRaidFrameManager_UpdateOptionsFlowContainer()` is gated on that flag in two places. The leader branch is **72 px taller** than the non-leader branch:

- `+40 px` — second action-button row: `readyCheck`, `rolePoll`, `countdown` only show for leader (or raid+assist), so the `STRIDE=4` loop produces 5 visible buttons ⇒ 2 rows instead of 1
- `+32 px` — `RestrictPings` block (leader-only): `VerticalSpace(5) + Label 0-tall + VerticalSpace(2) + Dropdown 25-tall = 32`

Manual flow-math matches the probe exactly:

```
startingPrimary      48
+ editMode row       40
+ countdown row      40   (leader only)
+ primSpacer          10
+ raidMarkers         99
+ primSpacer           5  (leader only, before RestrictPingsLabel)
+ Label                0  (leader only)
+ primSpacer           2  (leader only, before Dropdown)
+ Dropdown            25  (leader only)
+ primSpacer           5
+ BottomButtons (lineMax) 53
= 327                     (= flowMaxPrimaryUsed)
⇒ SetHeight(usedY + 20) = 347
```

Non-leader trace produces `usedY = 255`, `SetHeight = 275` — matching the no-seed baseline probe that gave `manager H=275` and `toggle top y=260`.

So both states behave correctly per Blizzard's source. The reason the branch "looks too tall compared with retail" is that the simulator's default seeded party makes the player the leader, while the retail reference screenshot is almost certainly a non-leader state.

## Fix Direction

Because the bug is upstream state, the fix belongs upstream — not in the render path, not in a flow-container workaround, not in the vendored Blizzard source.

Candidate state-side fixes, ordered by scope:

1. **Fixture-only** (narrowest): change the brief's reproduction fixture to explicitly pass leadership to a member (e.g. `A_Admin.SetPartyLeader(1)`) so the probed state matches the retail reference. Keeps the sim default as-is.
2. **Sim default** (broader): change `SimState::party_leader_index`'s default from `None` (player-is-leader) to `Some(1)` (a member-is-leader) for seeded parties that weren't created by the player. This makes the "just dropped into a 4-man party" default match the more common retail scenario. Existing admin commands (`A_Admin.SetPartyLeader`) still give full control.
3. **Semantics** (broadest): introduce a distinction between "player solo" and "player in a seeded party" at the `SimState` level, so the leader default depends on how the party was seeded instead of falling out of `None`.

Whichever is chosen, the render/layout pipeline needs no change: it correctly paints the logical rect, and the logical rect correctly reflects the flow math for the seeded state.
- `UpdateOptionsFlowContainer` running the leader branch at all while `collapsed == true`; if retail gates on `collapsed` somewhere outside this file, we don't

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

Root cause is upstream state (`party_leader_index` default), not render or layout. Pick one of the three fix directions above and land it. A single re-probe afterwards with the new state default should show the collapsed strip at `222x275` (non-leader usedY=255) for the "just-joined-a-4-man-party" reference screenshot.
