# Taint System

The taint system enforces WoW's secure/insecure execution boundary. The simulator keeps the practical pieces that addons rely on: protected-frame gating, the dual-environment split (`genv` vs `secureenv`), SecureHandler fallbacks, and shallow state/attribute driver application. Per-call taint tracking still comes from the Elune Lua runtime.

## Design Scope

Per DESIGN.md, full taint simulation is a non-goal: "Secure execution (taint system is stubbed as always-secure)." What the simulator does implement:

- **Protected-frame gating**: `can_change_protected_state_for()` blocks protected mutations when the caller is insecure and the player is in combat
- **Dual Lua environment**: `genv` (`_G`) for addon code vs `secureenv` for Blizzard secure code
- **issecure() and securecall()**: provided by Elune's `baselib_shared`; permissive fallbacks are installed only when absent
- **securecallmethod()**: simulator-provided helper that Elune omits
- **SecureHandler fallback**: `SecureHandlerSetFrameRef`, `SecureHandlerGetFrameRef`, `SecureHandlerExecute`, `SecureHandlerWrapScript`, and `SecureHandlerUnwrapScript` are backed by the Lua-side fallback in `src/lua_api/globals/security.rs`
- **State/attribute drivers**: `RegisterStateDriver`, `UnregisterStateDriver`, `RegisterAttributeDriver`, `UnregisterAttributeDriver` store raw driver text in `SimState.secure_attribute_drivers` and eagerly apply the resolved state

## Protected Frame Gating (`src/lua_api/frame/methods/methods_helpers.rs`)

`can_change_protected_state_for(state, id)` returns true when the caller is secure or the player is out of combat. In combat, it blocks if `frame_blocks_protected_state()` says the target frame is protected, has a protected descendant, or anchors to protected state.

The frame methods that use this gate live under `src/lua_api/frame/methods/`, primarily:
- `core_state/visibility.rs`
- `core_state/size.rs`
- `core_state/scale.rs`
- `core_state/strata_level.rs`
- `button_anchor_hierarchy/anchors.rs`
- `button_anchor_hierarchy/hierarchy.rs`
- `text_attribute_event/attributes.rs`

When blocked, callers emit `ADDON_ACTION_BLOCKED` via `emit_addon_action_blocked()` and return without mutating the frame.

## Dual Lua Environment (`src/lua_api/globals/security.rs`)

Two environments share the same Lua state:

- **genv** (`_G`): addon code; `Blizzard_EnvironmentCleanup` nils secure APIs here
- **secureenv**: shallow copy of `_G` at startup with `__index = _G` fallback; retains secure APIs after cleanup; addons with `UseSecureEnvironment: 1` in TOC run here via `setfenv`

`set_in_both_envs_rilua(key, value)` registers named frames in both environments so frame globals are visible from both.

## issecure and securecall

Provided by Elune's C runtime (`baselib_shared`). `src/lua_api/globals/security.rs` installs permissive fallbacks only when absent:
- `issecretvalue()` — checks if a function was created by `loadstring` (tracked in `__tainted_loadstring_functions` registry)
- `canaccessvalue()`, `canaccessallvalues()`, `canaccesstable()` — return true unless value is secret
- `scrub()`, `scrubsecretvalues()` — pass-through

`securecallmethod(obj, name, ...)` — calls `obj[name](obj, ...)` via `securecall`; simulator-provided since Elune omits it.

## SecureHandler Fallback

`src/lua_api/globals/security.rs` installs a Lua-side fallback for the SecureHandler APIs before `Blizzard_RestrictedAddOnEnvironment` arrives.

- `SecureHandlerSetFrameRef(frame, label, refFrame)` stores frame refs in weak-keyed registries
- `SecureHandlerGetFrameRef(frame, label)` reads those stored refs back
- `SecureHandlerExecute(frame, body, ...)` compiles `body` into a restricted closure and runs it with `pcall`
- `SecureHandlerWrapScript(frame, script, header, preBody, postBody)` wraps the original handler with pre/original/post callbacks
- `SecureHandlerUnwrapScript(frame, script)` restores the original handler

## State Drivers

`RegisterStateDriver`, `UnregisterStateDriver`, `RegisterAttributeDriver`, `UnregisterAttributeDriver` are backed by `SimState.secure_attribute_drivers`.

- `RegisterStateDriver(frame, "visibility", ...)` maps to the `state-visibility` special case and toggles visibility plus `statehidden`
- Other state/attribute drivers resolve the final clause of the driver string and write that value directly onto the frame
- `Unregister*` removes the stored driver text but leaves the last applied frame state in place

## Sources

- [protected-frame-enforcement.md](../../protected-frame-enforcement.md) — protected-frame behavior and remaining gaps
- `src/lua_api/frame/methods/methods_helpers.rs` — protected-state gating and `ADDON_ACTION_BLOCKED`
- `src/lua_api/globals/security.rs` — taint helpers, `securecallmethod`, SecureHandler fallback, state/attribute drivers, secure environment
- `src/lua_api/state.rs` — `secure_attribute_drivers` storage
- `tests/protected_frame_enforcement.rs` — combat lockdown coverage
- `tests/secure_handler_fallback.rs` — SecureHandler fallback coverage
- `tests/security_api.rs` — state driver and `securecallmethod` coverage

## See Also

- [[lua-api]] — issecure, securecall, hooksecurefunc globals
- [[event-system]] — ADDON_ACTION_BLOCKED event firing
- [[frame-data-flow]] — frame is_protected field and Protect() method
