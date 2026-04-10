# Protected Frames

Protected frame enforcement: blocks insecure addon code from moving, showing, or modifying protected frames during combat.

## Enforcement Rules

A method call is blocked when **all three** are true:
1. The calling code is insecure (addon code — `issecure()` returns false)
2. `InCombatLockdown()` is true (between `PLAYER_REGEN_DISABLED` and `PLAYER_REGEN_ENABLED`)
3. The target frame is protected, or an ancestor/descendant/anchor-relative of a protected frame

Blocked calls: silently no-op and fire `ADDON_ACTION_BLOCKED`. Blizzard secure code can still call these methods during combat.

## Covered Methods

- Anchor/movement: `SetPoint`, `ClearAllPoints`, `AdjustPointsOffset`, `StartMoving`, `StopMovingOrSizing`
- Visibility: `Show`, `Hide`, `SetShown`
- Hierarchy/size: `SetParent`, `SetSize`, `SetWidth`, `SetHeight`
- Strata/level: `SetFrameLevel`, `SetFrameStrata`, `SetFixedFrameLevel`, `SetFixedFrameStrata`, `SetToplevel`
- Other: `SetClampedToScreen`, `SetHitRectInsets`, `SetScrollChild`, `SetHyperlinksEnabled`, `SetPropagateKeyboardInput`, `SetForbidden`

## Remaining Gaps

**No-op stubs needing real implementations before enforcement matters:**
- `SetClampRectInsets`, `SetUsingParentLevel`, `StartSizing`

**Read-only restricted APIs not yet enforced:**
- `GetRect`, `GetLeft`, `GetPoint`, `GetBoundsRect`

## Open Questions

Exact live-WoW error behavior is unknown:
- Does blocked call silently no-op, or raise a Lua error?
- Is `[ADDON_ACTION_BLOCKED]` sent to the UI error handler or just chat?
- Does it fire a specific event?

## Note

Wowless does not enforce protected frames — only this simulator and live WoW enforce the rule.

## Sources

- [protected-frame-enforcement.md](../../protected-frame-enforcement.md) — full method list and open questions
