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
  - Current regression coverage still sees the root button plus the duration `FontString` in the dirty set on that settled tick.
  - Conclusion: the remaining cost is still centered in the Lua-side duration formatting / font-threshold path, not in missing coverage around whether the handler runs.

That shifts the next optimization target away from more setter no-op guards and toward short-circuiting the redundant work in the handlers themselves, especially `AuraButtonMixin:OnUpdate`.

## 2026-04-22 BuffFrame slow-handler interpretation

Slow handler logs shaped like `addon=Blizzard_BuffFrame handler=OnUpdate frame=#21946`
do **not** point at the named `BuffFrame` root. They point at an anonymous
`AuraButtonTemplate` child:

- `AuraFrameMixin:AuraFrame_OnLoad()` creates buff buttons with
  `CreateFrame("BUTTON", nil, self.AuraContainer, "AuraButtonTemplate")`, so
  the buttons have no global name.
- The timing logger prints `#<widget_id>` when a frame has no name.
- `AuraButtonMixin:UpdateExpirationTime()` enables `OnUpdate` only for timed
  auras (`expirationTime > 0`), which matches the visible buff buttons whose
  `Duration` labels are active.

That matters because the top-level `BuffFrameMixin:OnUpdate()` has a different
shape:

- it would log as `frame=BuffFrame`, not `frame=#...`
- it is a hidden-buff maintenance path with a `0.2s` throttle
  (`hiddenBuffUpdatePeriod`), not a per-frame countdown update

A headless `dump-tree --filter-key BuffFrame --visible-only` run on
2026-04-22 showed five visible anonymous BuffFrame buttons, each with a
visible `Duration` font string. That lines up with the repeated anonymous
`OnUpdate` timings and rules out the named root frame as the hot path.

The remaining work is still at the handler level. The current Rust mutators for
`SetAlpha`, `SetFormattedText`, `SetFontObject`, and `SetPoint` already have
same-value guards, so the main waste is that `AuraButtonMixin:OnUpdate()` keeps
recomputing countdown formatting and threshold branches every tick before those
guards can bail out.

## 2026-04-22 BuffFrame settled-tick optimization

The BuffFrame hot path now has a simulator-side workaround in
`src/lua_api/workarounds.rs` that patches `AuraButtonMixin:OnUpdate()` without
editing Blizzard vendor files.

The optimization deliberately stays narrow:

- keep Blizzard behavior for temp enchants
- keep Blizzard behavior while the tooltip owns the aura button
- keep Blizzard behavior once `timeLeft < BUFF_DURATION_WARNING_TIME` (`90s`),
  where the countdown and warning color legitimately change often
- short-circuit the common long-buff settled path where the visible state is
  unchanged

For long buffs (`timeLeft >= 90s`), the patch now caches the pieces that matter
to rendering:

- warning alpha target
- duration visibility
- duration display bucket (minute / hour / day bucket + rounded value)
- duration font-threshold mode

When those cached values are unchanged, the settled tick skips:

- `SecondsToTimeAbbrev()`
- `Duration:SetFormattedText()`
- `Duration:SetShown()`
- `Duration:SetVertexColor()`
- `Duration:SetFontObject()`
- `Duration:SetPoint()`
- `SetAlpha()`

That turns the old “call a stack of no-op mutators every frame” behavior into a
state-driven path for long buffs, while still falling back to the Blizzard
logic in the short-duration and tooltip-owned cases where the UI legitimately
changes every tick.

Regression coverage now lives in `tests/onupdate_handler_audit.rs`:

- `buff_button_onupdate_skips_settled_duration_reformatting_and_alpha_churn`
  proves a second settled tick on a long buff performs none of the duration /
  font / alpha updates and leaves the render-dirty batch empty
- `leave_instance_group_button_queries_group_state_even_when_mutators_noop`
  still covers the compact-raid button path separately

## 2026-04-14 GameTimeFrame calendar atlas follow-up

The `GameTimeFrame_SetDate()` follow-up showed a different no-op churn shape than
the visible `OnUpdate` handlers above:

- `GameTimeFrame_SetDate()` re-applies three atlas-backed button textures every
  time it runs (`up`, `down`, `mouseover`) even when the calendar day has not
  changed.
- In the simulator, the plain `SetNormalTexture` / `SetPushedTexture` /
  `SetHighlightTexture` path resolved the atlas string, then immediately called
  `get_mut_visual()` on both the button and child texture.
- `get_mut_visual()` marks render-dirty on mutable borrow, so a same-day
  `GameTimeFrame_SetDate()` still dirtied the minimap strata even though the
  resolved file path, UVs, and visibility were identical.

The fix was to make `apply_set_button_texture_path()` check the current button
field and texture-child state first. When the resolved path/UVs, `fileDataID`,
parent key, anchors, and visibility already match, it now returns before taking
any visual mutable borrows.

Regression coverage now includes:

- a low-level button-texture test that repeats
  `SetNormalTexture("ui-hud-calendar-1-up")`
- a full-UI `GameTimeFrame_SetDate()` test that proves the second same-day call
  leaves the render-dirty batch empty

## Fix Strategies

**Option A: Same-value guards in Rust methods** — Make `SetValue`, `SetText`, `SetAlpha`, `Show`, `Hide` etc. skip `get_mut()` when the new value equals the current value. Fixes the root cause; blanket discard can be removed entirely. Requires touching many API methods.

**Option B: Per-frame dirty tracking** — Replace single `render_dirty: bool` with a set of dirty frame IDs. Selectively preserve dirty from known-legitimate frames. More complex, doesn't fix the underlying `get_mut()` problem.

**Option C: StatusBar-specific check** — After `fire_on_update()`, check if any StatusBar's `statusbar_value` actually changed. Smallest change, but requires special-casing each new legitimate visual change.

## Sources

- [on-update-dirty-handlers.md](../../on-update-dirty-handlers.md) — full handler classification and fix options
- [BuffFrame.lua](../../../../Interface/BlizzardUI/Blizzard_BuffFrame/BuffFrame.lua) — `BuffFrameMixin` and `AuraButtonMixin` `OnUpdate` paths
- [BuffFrameTemplates.xml](../../../../Interface/BlizzardUI/Blizzard_BuffFrame/BuffFrameTemplates.xml) — `AuraButtonTemplate` script registration
- [handler_timing.rs](../../../../src/lua_api/handler_timing.rs) — `frame=#id` fallback formatting
- [script_helpers.rs](../../../../src/lua_api/script_helpers.rs) — `OnUpdate` dispatch timing scope
- [onupdate_handler_audit.rs](../../../../tests/onupdate_handler_audit.rs) — focused regression test for BuffFrame button `OnUpdate`

## See Also

- [[talent-performance]] — OnUpdate loop in talent panel caused by rect-dirty bugs
