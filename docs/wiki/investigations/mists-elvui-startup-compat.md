# Mists ElvUI Startup Compatibility

Full-addon Mists startup exposed several unrelated simulator compatibility gaps that made ElvUI and Blizzard aura headers fail before the remaining addon errors could be isolated: missing trim aliases, plain frames exposing MessageFrame-only methods, Mists aura callbacks using the wrong tuple shape, scaled screen-size mismatches, missing chat hook globals, and per-Font-object metatables.

## Content

### Symptoms

The full Mists addon pass reported these startup failures:

- `SecureGroupHeaders.lua:742` called `value:trim()` and failed because simulator strings did not expose the Mists-era `trim` alias.
- ElvUI scrollbar skinning saw a plain frame field named `ScrollUp` as a callable method and treated that function as a button, later failing while indexing `btn`.
- `SecureGroupHeaders.lua:951` compared nil aura sort keys because Mists Blizzard code destructures `AuraUtil.ForEachAura` callbacks as legacy `UnitAura` tuples.
- ElvUI slider skinning hit `string.find` with nil anchor data because simulator-created Slider `Low`, `High`, and `Text` fontstrings had no default points.
- ElvUI Tooltip initialization failed at `GameTooltipText:FontTemplate(...)` because the method was added through one Font object's metatable but not visible on other Font objects.
- ElvUI DataTexts durability initialization failed because `GetInventoryItemDurability` was missing from the inventory probe globals.

### Root Causes

The string error was a runtime surface gap: Mists vendor Lua expects both `strtrim(value, chars?)` and `string.trim(value, chars?)`.

The ElvUI scrollbar error was not a missing ElvUI button. The shared frame metatable exposed MessageFrame scroll-navigation methods such as `ScrollUp` on non-MessageFrame widgets, so ElvUI's child-field probing found a function where real WoW would return nil.

The aura sort error came from profile drift. Mists `SecureGroupHeaders` checks `AuraUtil.ForEachAura`, but its callback body still expects the legacy multi-return aura tuple rather than a mainline aura table. The simulator's generic AuraUtil behavior therefore supplied incompatible data after Blizzard created `AuraUtil`.

The slider error came from incomplete default child geometry. Real slider label regions have anchor points; simulator-generated default slider fontstrings were unanchored, so ElvUI's skinning logic received nil `anchorPoint`.

The Tooltip font error came from simulator Font objects each receiving a fresh metatable. Real WoW exposes a shared object-type method table for Font objects, so ElvUI's `AddAPI(GameFontNormal)` mutation of the Font metatable must also make `FontTemplate` visible on `GameTooltipText` and `GameTooltipHeaderText`.

The durability error was a missing inventory global. The simulator already tracked equipped item presence, but had no wear model; returning full current/max durability for equipped player slots and nil for empty slots matches the addon-facing shape without inventing persistent damage state.

### Fix Pattern

Keep the fixes at the compatibility surface that owns each behavior:

- `shared_bootstrap.lua` defines `strtrim` and `string.trim` because this alias is runtime-wide string surface.
- `frame_metatable.rs` filters MessageFrame-only method names from non-MessageFrame widget metatables while preserving ScrollingMessageFrame access.
- `mists/post_load.lua` patches `AuraUtil.ForEachAura` after Blizzard creates `AuraUtil`, adapting callbacks back to the Mists legacy tuple.
- `helpers_shared.rs` anchors generated Slider `Low`, `High`, and `Text` fontstrings at creation time.
- Font objects use a shared registry-backed metatable, because addon metatable mutations target the object type, not only one global Font instance.
- `inventory_probes.rs` exposes `GetInventoryItemDurability(slot)` from equipped item presence, returning full durability for modeled equipped items and nil for empty slots.

The full-addon probe after these fixes still reports separate ElvUI/Syndicator/StaticPopup issues, but no longer reports `SecureGroupHeaders`, `string.trim`, scrollbar `btn`, residual slider `string.find`, ElvUI Chat `SecureHook`, ElvUI Tooltip `FontTemplate`, or ElvUI DataTexts durability errors.

### Screen Size and ElvUIParent Placement

The ElvUI install panel text and raid-control position shared a later geometry root cause. ElvUI sets `UIParent:SetScale(0.64)` and expects `GetScreenWidth()` / `GetScreenHeight()` to return UI units, while `GetPhysicalScreenSize()` returns physical pixels. The simulator returned fixed physical values for all three during bootstrap, so ElvUI sized `ElvUIParent` to `1024x768` logical units under a `0.64` scale. That produced a physically smaller parent anchored to the bottom of the screen, pushing `RaidUtility_ShowButton` toward the middle and making install-panel content appear displaced or covered.

The fix keeps `GetPhysicalScreenSize()` physical, but makes `GetScreenWidth()` / `GetScreenHeight()` divide by `UIParent:GetEffectiveScale()`. `WowLuaEnv::set_screen_size()` also fires `DISPLAY_SIZE_CHANGED` and `UI_SCALE_CHANGED` so addons recompute layout when screenshot/GUI paths resize after addon startup.

### ElvUI Chat Hook Targets

ElvUI Chat initialized far enough to create chat frames, then aborted while installing AceHook secure hooks. The first missing target was `RedockChatWindows`, which Mists static popup definitions also call. After adding that Mists post-load function, the next missing hook target was the runtime global `GetPlayerInfoByGUID`, which Blizzard chat code and ElvUI both expect.

`RedockChatWindows` belongs in Mists post-load compatibility because it depends on Blizzard chat globals such as `FCF_DockFrame` and `GENERAL_CHAT_DOCK`. `GetPlayerInfoByGUID` belongs in the shared runtime surface because it is a WoW global used across chat, social queue, static popup, and shared unit utilities. After both were present, the ElvUI Chat `SecureHook` startup error disappeared and `ChatFrame1` was parented to visible `LeftChatPanel`.

### Font Object Metatable

ElvUI's `E:AddAPI(GameFontNormal)` adds methods such as `FontTemplate` to the Font object's metatable. The simulator previously attached a new metatable to each Font table, so `GameTooltipText` had the base Rust Font methods but not ElvUI's metatable additions. Reusing one registry-backed Font metatable matches the object-type API shape and removes the `Tooltip.lua:1089` startup failure from the full-addon Mists probe.

## Sources

- [shared_bootstrap.lua](../../../src/lua_api/env_init/shared_bootstrap.lua) — trim alias compatibility
- [frame_metatable.rs](../../../src/lua_api/methods/frame_metatable.rs) — per-widget method filtering
- [mists/post_load.lua](../../../src/mists/post_load.lua) — post-load Mists AuraUtil tuple adapter
- [helpers_shared.rs](../../../src/lua_api/globals/create_frame/helpers_shared.rs) — default Slider label anchoring
- [fonts.rs](../../../src/lua_api/globals/font_strings_collection/fonts.rs) — shared Font object metatable registration
- [inventory_probes.rs](../../../src/lua_api/globals/inventory_probes.rs) — inventory item durability probe
- [runtime_surface_bootstrap.lua](../../../src/lua_api/env_init/runtime_surface_bootstrap.lua) — bootstrap screen-size fallback
- [env_runtime.rs](../../../src/lua_api/env_runtime.rs) — runtime screen-size globals and resize event dispatch
- [mists/post_load.lua](../../../src/mists/post_load.lua) — Mists `RedockChatWindows` compatibility
- [PLAN.md](../../../PLAN.md) — remaining Mists full-addon error list

## See Also

- [[frame-data-flow]] — frame method/property lookup and why exposed methods affect addon probing
- [[talent-performance]] — earlier Mists full-addon startup investigation that exposed ElvUI login cost
