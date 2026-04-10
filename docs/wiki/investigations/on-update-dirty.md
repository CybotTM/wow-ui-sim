# OnUpdate Dirty Handlers

`handle_process_timers()` blanket-discards `render_dirty` after firing OnUpdate handlers, suppressing legitimate visual changes like the cast bar's `SetValue()` calls.

## Root Cause

`WidgetRegistry::get_mut()` unconditionally sets `render_dirty = true` for any mutable access, even when writing the same value. The blanket discard was added as a workaround because some handlers (e.g. `MainMenuMicroButton` setting identical atlas values every second) mark dirty without producing any visual change.

## Classified Handlers (37 visible at startup)

**Noisy (false dirty):**
- `ActionBarButtonUpdateFrame` — calls `SetChecked()` on all buttons each tick; idle after first few ticks once buttons unregister
- `MainMenuMicroButton` — calls `SetNormalAtlas` etc. with the same values every second
- `QueueStatusButton` — calls `Show()` on an already-shown texture every tick

**Legitimate (should trigger redraws):**
- `PlayerCastingBarFrame` — `SetValue()` and `SetText()` genuinely change every frame during a cast
- `PlayerFrame` — `SetAlpha()` on StatusTexture oscillates smoothly during combat
- Action button flash textures — `Show()`/`Hide()` toggling at `ATTACK_BUTTON_FLASH_TIME` intervals

**Inert (no dirty, no issue):**
- `ChatFrame1`, `WorldFrame`, ModelScene frames, idle PartyMemberFrame buttons

## Fix Strategies

**Option A: Same-value guards in Rust methods** — Make `SetValue`, `SetText`, `SetAlpha`, `Show`, `Hide` etc. skip `get_mut()` when the new value equals the current value. Fixes the root cause; blanket discard can be removed entirely. Requires touching many API methods.

**Option B: Per-frame dirty tracking** — Replace single `render_dirty: bool` with a set of dirty frame IDs. Selectively preserve dirty from known-legitimate frames. More complex, doesn't fix the underlying `get_mut()` problem.

**Option C: StatusBar-specific check** — After `fire_on_update()`, check if any StatusBar's `statusbar_value` actually changed. Smallest change, but requires special-casing each new legitimate visual change.

## Sources

- [on-update-dirty-handlers.md](../../on-update-dirty-handlers.md) — full handler classification and fix options

## See Also

- [[talent-performance]] — OnUpdate loop in talent panel caused by rect-dirty bugs
