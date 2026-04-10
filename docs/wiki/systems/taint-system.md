# Taint System

The taint system enforces WoW's secure/insecure execution boundary. The simulator implements the key runtime enforcement (combat lockdown on protected frames) and the dual-environment split (genv vs secureenv), while delegating per-call taint tracking to the Elune Lua runtime.

## Design Scope

Per DESIGN.md, full taint simulation is a non-goal: "Secure execution (taint system is stubbed as always-secure)." What the simulator does implement:

- **Combat lockdown**: blocks restricted methods on protected frames when caller is insecure and player is in combat
- **Dual Lua environment**: `genv` (addon code) vs `secureenv` (Blizzard secure code)
- **issecure() and securecall()**: provided by Elune's `baselib_shared`; permissive fallbacks installed if absent
- **securecallmethod()**: simulator-provided (not in Elune)
- **SecureHandler stubs**: `SecureHandlerSetFrameRef`, `SecureHandlerExecute`, `SecureHandlerWrapScript` are inert no-ops

## Combat Lockdown (`src/lua_api/frame/methods/combat_lockdown.rs`)

`check_and_fire(lua, state, frame_id, method_name)` returns true (block the call) when all three hold:
1. `issecure()` returns false (caller is addon code)
2. `state.player.in_combat` is true
3. Frame is protected, has a protected ancestor/descendant, or is anchored to a protected relation

When blocked: fires `ADDON_ACTION_BLOCKED(addon_name, "FrameName:Method()")` via `FireEvent` and silently no-ops.

Protected relation check walks ancestors (BFS), descendants (BFS), and anchor targets transitively. Anchor targets come from `frame.anchors[*].relative_to_id`.

Restricted methods covered: `SetPoint`, `ClearAllPoints`, `AdjustPointsOffset`, `Show`, `Hide`, `SetShown`, `SetParent`, `SetSize`, `SetWidth`, `SetHeight`, `SetFrameLevel`, `SetFrameStrata`, `SetFixedFrameLevel`, `SetFixedFrameStrata`, `SetToplevel`, `SetClampedToScreen`, `SetHitRectInsets`, `SetScrollChild`, `SetHyperlinksEnabled`, `SetPropagateKeyboardInput`, `SetForbidden`.

Remaining gaps: `SetClampRectInsets`, `SetUsingParentLevel`, `StartSizing` (stubs only); read-only restricted APIs (`GetRect`, `GetLeft`, `GetPoint`, `GetBoundsRect`) not yet enforced.

## Dual Lua Environment (`src/lua_api/secure_env.rs`)

Two environments share the same Lua state:

- **genv** (`_G`): addon code; `Blizzard_EnvironmentCleanup` nils secure APIs here
- **secureenv**: shallow copy of `_G` at startup with `__index = _G` fallback; retains secure APIs after cleanup; addons with `UseSecureEnvironment: 1` in TOC run here via `setfenv`

`set_in_both_envs(key, value)` registers named frames in both environments so frame globals are visible from both.

## issecure and securecall

Provided by Elune's C runtime (`baselib_shared`). `security_api.rs` installs permissive fallbacks only when absent (`set_if_missing`):
- `issecretvalue()` — checks if a function was created by `loadstring` (tracked in `__tainted_loadstring_functions` registry)
- `canaccessvalue()`, `canaccessallvalues()`, `canaccesstable()` — return true unless value is secret
- `scrub()`, `scrubsecretvalues()` — pass-through

`securecallmethod(obj, name, ...)` — calls `obj[name](obj, ...)` via `securecall`; simulator-provided since Elune omits it.

## State Driver Stubs

`RegisterStateDriver`, `UnregisterStateDriver`, `RegisterAttributeDriver`, `UnregisterAttributeDriver` — all inert no-ops. Real drivers depend on `SecureStateDriverManager` and protected attribute propagation, which the simulator does not model.

## Sources

- [protected-frame-enforcement.md](../../protected-frame-enforcement.md) — restricted method list, combat lockdown conditions, remaining gaps
- `src/lua_api/globals/security_api.rs` — taint helpers, securecallmethod, SecureHandler stubs
- `src/lua_api/secure_env.rs` — dual environment setup
- `src/lua_api/frame/methods/combat_lockdown.rs` — check_and_fire, protected relation traversal

## See Also

- [[lua-api]] — issecure, securecall, hooksecurefunc globals
- [[event-system]] — ADDON_ACTION_BLOCKED event firing
- [[frame-data-flow]] — frame is_protected field and Protect() method
