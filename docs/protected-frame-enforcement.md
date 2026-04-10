# Protected Frame Enforcement Plan

## Current State

The simulator now enforces the core protected-frame rule for the live movement,
visibility, hierarchy, and frame-property methods we actually implement:

- block only when all three conditions are true:
  - the caller is insecure
  - the player is in combat
  - the target frame is protected, an ancestor/descendant of a protected frame,
    or anchored to a protected relation
- silently no-op and fire `ADDON_ACTION_BLOCKED`
- allow the same calls from secure code during combat
- allow the same calls from insecure code out of combat

Wowless also does not enforce this — only we and live WoW would.

## WoW Behavior

When all three conditions are true, the method call is **blocked**:
1. Frame is **protected** (`IsProtected()` returns true)
2. `InCombatLockdown()` is true (between `PLAYER_REGEN_DISABLED` and `PLAYER_REGEN_ENABLED`)
3. Caller is **insecure** (`issecure()` returns false — i.e., addon code)

Blocked calls produce: `[ADDON_ACTION_BLOCKED] AddOn 'X' tried to call the protected function 'FrameName:Method()'`

Secure (Blizzard) code can still call these methods on protected frames during combat.

## Restricted Methods (Frame Movement Subset)

From [warcraft.wiki.gg/Category:API_functions/restricted](https://warcraft.wiki.gg/wiki/Category:API_functions/restricted):

### Movement & Positioning
- `ScriptRegionResizing:SetPoint()`
- `ScriptRegionResizing:AdjustPointsOffset()`
- `ScriptRegionResizing:ClearPointsOffset()`
- `Line:ClearAllPoints()`
- `Frame:StartMoving()`
- `Frame:StartSizing()`
- `Frame:StopMovingOrSizing()`

### Visibility
- `ScriptRegion:Show()`
- `ScriptRegion:Hide()`
- `ScriptRegion:SetShown()`

### Strata & Level
- `Frame:SetFrameLevel()`
- `Frame:SetFrameStrata()`
- `Frame:SetFixedFrameLevel()`
- `Frame:SetFixedFrameStrata()`
- `Frame:SetToplevel()`

### Other Frame Properties
- `Frame:SetClampedToScreen()`
- `Frame:SetClampRectInsets()`
- `Frame:SetHitRectInsets()`
- `Frame:SetHyperlinksEnabled()`
- `Frame:SetPropagateKeyboardInput()`
- `Frame:SetUsingParentLevel()`
- `Region:SetIgnoreParentScale()`
- `ScrollFrame:SetScrollChild()`
- `FrameScriptObject:SetForbidden()`

### Read-Only (Also Restricted)
- `ScriptRegion:GetRect()`
- `ScriptRegion:GetLeft()`
- `ScriptRegionResizing:GetPoint()`
- `Frame:GetBoundsRect()`

## Covered Runtime Methods

- Anchor/movement:
  - `SetPoint`
  - `ClearAllPoints`
  - `AdjustPointsOffset`
  - `StartMoving`
  - `StopMovingOrSizing`
- Visibility:
  - `Show`
  - `Hide`
  - `SetShown`
- Hierarchy and size:
  - `SetParent`
  - `SetSize`
  - `SetWidth`
  - `SetHeight`
- Strata/level:
  - `SetFrameLevel`
  - `SetFrameStrata`
  - `SetFixedFrameLevel`
  - `SetFixedFrameStrata`
  - `SetToplevel`
- Other live property setters:
  - `SetClampedToScreen`
  - `SetHitRectInsets`
  - `SetScrollChild`
  - `SetHyperlinksEnabled`
  - `SetPropagateKeyboardInput`
  - `SetForbidden`

## Remaining Gaps

- no-op stubs still need real implementations before enforcement matters for them:
  - `SetClampRectInsets`
  - `SetUsingParentLevel`
  - `StartSizing`
- read-only restricted APIs are still not enforced:
  - `GetRect`
  - `GetLeft`
  - `GetPoint`
  - `GetBoundsRect`

## Open Questions

### 1. Determine exact live-WoW error behavior

**Unknown — needs live WoW testing:**
- Does the blocked call silently no-op? Or raise a Lua error?
- Is the `[ADDON_ACTION_BLOCKED]` message sent to the UI error handler or just the chat frame?
- Does it fire a specific event?

Candidate test addon for live WoW:
```lua
local f = CreateFrame("Button", "ProtTestBtn", UIParent, "SecureActionButtonTemplate")
f:SetPoint("CENTER")
f:SetSize(50, 50)
-- Wait for combat, then:
-- /run ProtTestBtn:SetPoint("TOP")  -- should be blocked
-- /run ProtTestBtn:Hide()           -- should be blocked
-- /run ProtTestBtn:StartMoving()    -- should be blocked
```
