# OnUpdate Dirty Handlers

`handle_process_timers()` blanket-discards `render_dirty` after firing OnUpdate handlers, suppressing legitimate visual changes like the cast bar's `SetValue()` calls.

## Root Cause

`WidgetRegistry::get_mut()` unconditionally sets `render_dirty = true` for any mutable access, even when writing the same value. The blanket discard was added as a workaround because some handlers (e.g. `MainMenuMicroButton` setting identical atlas values every second) mark dirty without producing any visual change.

## Classified Handlers (37 visible at startup)

**Noisy (false dirty):**
- `ActionBarButtonUpdateFrame` — calls `SetChecked()` on all buttons each tick; idle after first few ticks once buttons unregister
- `MainMenuMicroButton` — calls `SetNormalAtlas` etc. with the same values every second
- `QueueStatusButton` — calls `Show()` on an already-shown texture every tick
- `LeaveInstanceGroupButton` — only while the compact raid manager subtree is actually shown; as of 2026-04-14, `A_Admin.SetPartySize(0)` fires `GROUP_ROSTER_UPDATE`, so solo transitions now hide `CompactRaidFrameManager` and drop this button out of the visible `OnUpdate` set outside party content

**Legitimate (should trigger redraws):**
- `PlayerCastingBarFrame` — `SetValue()` and `SetText()` genuinely change every frame during a cast
- `PlayerFrame` — `SetAlpha()` on StatusTexture oscillates smoothly during combat
- Action button flash textures — `Show()`/`Hide()` toggling at `ATTACK_BUTTON_FLASH_TIME` intervals

**Inert (no dirty, no issue):**
- `ChatFrame1`, `WorldFrame`, ModelScene frames, idle PartyMemberFrame buttons

## 2026-04-14 Handler Audit Follow-up

Two focused audit tests narrowed the remaining work after the earlier `SetText` / `SetEnabled` no-op guards:

- `LeaveInstanceGroupButton`
  - A settled second tick still calls `C_PartyInfo.IsPartyWalkIn()` once, `PartyUtil.CanLeaveInstance()` once, `IsInGroup()` twice, `IsInInstance()` once, and `GetPartyLFGID()` once.
  - The handler still invokes `SetText` and `SetEnabled`, but the dirty batch stays empty once the button is already settled.
  - Conclusion: the remaining cost is query/dispatch work while the button is visible, not visual mutation churn. After the solo visibility fix, this handler no longer matters outside grouped content.

- `AuraButtonMixin:OnUpdate` (BuffFrame buttons)
  - A settled second tick still runs `SecondsToTimeAbbrev()`, `Duration:SetFormattedText()`, `Duration:SetFontObject()`, `Duration:SetPoint()`, `Duration:SetShown()`, `Duration:SetVertexColor()`, and `SetAlpha()` once each.
  - The dirty batch also stays empty on that settled tick.
  - Conclusion: the remaining cost is Lua-side duration formatting and font-threshold branching before the guarded mutators decide nothing changed.

That shifts the next optimization target away from more setter no-op guards and toward short-circuiting the redundant work in the handlers themselves, especially `AuraButtonMixin:OnUpdate`.

## Fix Strategies

**Option A: Same-value guards in Rust methods** — Make `SetValue`, `SetText`, `SetAlpha`, `Show`, `Hide` etc. skip `get_mut()` when the new value equals the current value. Fixes the root cause; blanket discard can be removed entirely. Requires touching many API methods.

**Option B: Per-frame dirty tracking** — Replace single `render_dirty: bool` with a set of dirty frame IDs. Selectively preserve dirty from known-legitimate frames. More complex, doesn't fix the underlying `get_mut()` problem.

**Option C: StatusBar-specific check** — After `fire_on_update()`, check if any StatusBar's `statusbar_value` actually changed. Smallest change, but requires special-casing each new legitimate visual change.

## Sources

- [on-update-dirty-handlers.md](../../on-update-dirty-handlers.md) — full handler classification and fix options

## See Also

- [[talent-performance]] — OnUpdate loop in talent panel caused by rect-dirty bugs
