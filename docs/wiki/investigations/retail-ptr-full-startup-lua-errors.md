# Retail/PTR Full Startup Lua Errors

Full GUI startup logs can expose script errors that `lua-errors` misses because they happen from startup-driven handlers, hover/click scripts, or frame update paths after initial addon loading. The June 2026 retail/PTR sweep fixed PVPUI, PTRFeedback cursor, and Store micro-button errors by adding backing API surfaces instead of masking the visible frames.

## Symptoms

Retail full startup with `WOW_SIM_NO_ADDONS=1 WOW_SIM_NO_SAVED_VARS=1 timeout 90 target/debug/wow-sim --no-addons --no-saved-vars` reported `Lua error:` lines from:

- `Blizzard_PVPUI.lua:2153` because `C_WeeklyRewards.GetConquestWeeklyProgress()` returned nil.
- `Blizzard_PVPUI.lua:1060` because `C_PvP.GetRandomEpicBGInfo()` returned nil.
- `Blizzard_PVPUI.lua:255` because legacy `GetPVPRoles()` was missing.
- `Blizzard_PVPUI.lua:154` because legacy `ClearBattlemaster()` was missing.
- `Blizzard_Shared_StoreUIInbound.lua:9` because vendor code overwrote the simulator `StoreFrame_SetShown` fallback and then dereferenced nil `StoreFrame`.

PTR full startup also reported `Blizzard_PTRFeedback_Frames.lua:917` because global `SetCursor("Interface\\CURSOR\\UI-Cursor-Move.blp")` was missing.

## Root causes and fixes

- PVP queue and weekly reward APIs belong to modeled runtime surface, not temporary shims. `C_WeeklyRewards.GetConquestWeeklyProgress()` now returns a stable table shape; `C_PvP.GetRandomBGInfo()` and `C_PvP.GetRandomEpicBGInfo()` return default random battleground info tables.
- Legacy PVP role globals are backed by existing `SimState.lfg_roles`, matching the LFG role surface instead of storing duplicate state.
- `ClearBattlemaster()` is a no-op compatibility surface until a real battlemaster model exists.
- `SetCursor()` and `ResetCursor()` are global cursor surface. `SetCursor()` accepts texture cursor paths and returns true.
- Store UI is intentionally not rendered, but the simulator tracks a store shown flag. Vendor `Blizzard_Shared_StoreUIInbound.lua` can overwrite the Rust fallback, so the temporary source patch must guard both `StoreFrame_IsShown()` and `StoreFrame_SetShown()` when `StoreFrame` is absent.
- A stale `GetNumBattlegroundTypes()` registration in `battlefield_lfg_probes` overwrote the real seeded PVP implementation and was removed.

## Verification

After the fix:

- Retail full startup log had zero lines starting `Lua error:`.
- PTR full startup log had zero lines starting `Lua error:`.
- Targeted tests covered PVP queue surfaces, cursor surface, Store UI source patching, seeded battleground info, and Admin vault activity regression.

## Related docs

- [Lua API](../../lua-api.md)
- [Event system](../../event-system.md)
