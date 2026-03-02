# Protected Frame Enforcement Plan

## Current State

The simulator tracks taint via Elune (stack taint, `issecure()`, `issecurevariable()`) and has a `is_protected` flag on frames, but **does not enforce** any restrictions. All methods work regardless of combat state or caller security.

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

## Implementation Plan

### 1. Add `is_protected_action_blocked` helper

In `src/lua_api/frame/methods/` or a shared module:

```rust
/// Returns true if the action should be blocked (protected frame + combat + insecure caller)
fn is_protected_action_blocked(lua: &Lua, state: &SimState, frame_id: u64) -> bool {
    let frame = state.widgets.get(frame_id);
    let is_protected = frame.map(|f| f.is_protected).unwrap_or(false);
    if !is_protected || !state.player.in_combat {
        return false;
    }
    // Check Elune's issecure() — stack taint is nil means secure
    // lua.load("return not issecure()").eval::<bool>().unwrap_or(true)
    // Or call Elune's debug.getstacktaint() directly
    !is_caller_secure(lua)
}
```

### 2. Wrap restricted methods

For each restricted method, add the check before the current logic:

```rust
methods.add_method("StartMoving", |lua, this, ()| {
    let state_rc = get_sim_state(lua);
    let s = state_rc.borrow();
    if is_protected_action_blocked(lua, &s, this.0) {
        // Log: [ADDON_ACTION_BLOCKED] ...
        return Ok(());  // silently blocked (or return error?)
    }
    drop(s);
    // ... existing logic
});
```

### 3. Determine error behavior

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

### 4. Propagation to parents/anchored frames

Per Wowpedia: restrictions apply to protected frames, **their parents**, and **any frame anchored to them**. This means the check must walk up the parent chain and check anchor targets.

### 5. Test coverage

Add tests in `Interface/AddOns/BlizzMove/tests/` or a dedicated `ProtectedFrameTest` addon:
- Protected frame + in_combat + insecure caller → blocked
- Protected frame + in_combat + secure caller → allowed
- Protected frame + out of combat → allowed
- Non-protected frame + in_combat → allowed
- Parent of protected frame → blocked (propagation)
