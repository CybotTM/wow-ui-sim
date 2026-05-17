# Mists ElvUI Startup Compatibility

Full-addon Mists startup exposed three unrelated simulator compatibility gaps that made ElvUI and Blizzard aura headers fail before the remaining addon errors could be isolated: missing trim aliases, plain frames exposing MessageFrame-only methods, and Mists aura callbacks using the wrong tuple shape.

## Content

### Symptoms

The full Mists addon pass reported these startup failures:

- `SecureGroupHeaders.lua:742` called `value:trim()` and failed because simulator strings did not expose the Mists-era `trim` alias.
- ElvUI scrollbar skinning saw a plain frame field named `ScrollUp` as a callable method and treated that function as a button, later failing while indexing `btn`.
- `SecureGroupHeaders.lua:951` compared nil aura sort keys because Mists Blizzard code destructures `AuraUtil.ForEachAura` callbacks as legacy `UnitAura` tuples.
- ElvUI slider skinning hit `string.find` with nil anchor data because simulator-created Slider `Low`, `High`, and `Text` fontstrings had no default points.

### Root Causes

The string error was a runtime surface gap: Mists vendor Lua expects both `strtrim(value, chars?)` and `string.trim(value, chars?)`.

The ElvUI scrollbar error was not a missing ElvUI button. The shared frame metatable exposed MessageFrame scroll-navigation methods such as `ScrollUp` on non-MessageFrame widgets, so ElvUI's child-field probing found a function where real WoW would return nil.

The aura sort error came from profile drift. Mists `SecureGroupHeaders` checks `AuraUtil.ForEachAura`, but its callback body still expects the legacy multi-return aura tuple rather than a mainline aura table. The simulator's generic AuraUtil behavior therefore supplied incompatible data after Blizzard created `AuraUtil`.

The slider error came from incomplete default child geometry. Real slider label regions have anchor points; simulator-generated default slider fontstrings were unanchored, so ElvUI's skinning logic received nil `anchorPoint`.

### Fix Pattern

Keep the fixes at the compatibility surface that owns each behavior:

- `shared_bootstrap.lua` defines `strtrim` and `string.trim` because this alias is runtime-wide string surface.
- `frame_metatable.rs` filters MessageFrame-only method names from non-MessageFrame widget metatables while preserving ScrollingMessageFrame access.
- `mists/post_load.lua` patches `AuraUtil.ForEachAura` after Blizzard creates `AuraUtil`, adapting callbacks back to the Mists legacy tuple.
- `helpers_shared.rs` anchors generated Slider `Low`, `High`, and `Text` fontstrings at creation time.

The full-addon probe after these fixes still reports separate ElvUI/Syndicator/StaticPopup issues, but no longer reports `SecureGroupHeaders`, `string.trim`, scrollbar `btn`, or residual slider `string.find` errors.

### Screen Size and ElvUIParent Placement

The ElvUI install panel text and raid-control position shared a later geometry root cause. ElvUI sets `UIParent:SetScale(0.64)` and expects `GetScreenWidth()` / `GetScreenHeight()` to return UI units, while `GetPhysicalScreenSize()` returns physical pixels. The simulator returned fixed physical values for all three during bootstrap, so ElvUI sized `ElvUIParent` to `1024x768` logical units under a `0.64` scale. That produced a physically smaller parent anchored to the bottom of the screen, pushing `RaidUtility_ShowButton` toward the middle and making install-panel content appear displaced or covered.

The fix keeps `GetPhysicalScreenSize()` physical, but makes `GetScreenWidth()` / `GetScreenHeight()` divide by `UIParent:GetEffectiveScale()`. `WowLuaEnv::set_screen_size()` also fires `DISPLAY_SIZE_CHANGED` and `UI_SCALE_CHANGED` so addons recompute layout when screenshot/GUI paths resize after addon startup.

### ElvUI Chat Hook Targets

ElvUI Chat initialized far enough to create chat frames, then aborted while installing AceHook secure hooks. The first missing target was `RedockChatWindows`, which Mists static popup definitions also call. After adding that Mists post-load function, the next missing hook target was the runtime global `GetPlayerInfoByGUID`, which Blizzard chat code and ElvUI both expect.

`RedockChatWindows` belongs in Mists post-load compatibility because it depends on Blizzard chat globals such as `FCF_DockFrame` and `GENERAL_CHAT_DOCK`. `GetPlayerInfoByGUID` belongs in the shared runtime surface because it is a WoW global used across chat, social queue, static popup, and shared unit utilities. After both were present, the ElvUI Chat `SecureHook` startup error disappeared and `ChatFrame1` was parented to visible `LeftChatPanel`.

## Sources

- [shared_bootstrap.lua](../../../src/lua_api/env_init/shared_bootstrap.lua) — trim alias compatibility
- [frame_metatable.rs](../../../src/lua_api/methods/frame_metatable.rs) — per-widget method filtering
- [mists/post_load.lua](../../../src/mists/post_load.lua) — post-load Mists AuraUtil tuple adapter
- [helpers_shared.rs](../../../src/lua_api/globals/create_frame/helpers_shared.rs) — default Slider label anchoring
- [runtime_surface_bootstrap.lua](../../../src/lua_api/env_init/runtime_surface_bootstrap.lua) — bootstrap screen-size fallback
- [env_runtime.rs](../../../src/lua_api/env_runtime.rs) — runtime screen-size globals and resize event dispatch
- [mists/post_load.lua](../../../src/mists/post_load.lua) — Mists `RedockChatWindows` compatibility
- [PLAN.md](../../../PLAN.md) — remaining Mists full-addon error list

## See Also

- [[frame-data-flow]] — frame method/property lookup and why exposed methods affect addon probing
- [[talent-performance]] — earlier Mists full-addon startup investigation that exposed ElvUI login cost
