# Taint System

The taint system enforces WoW's secure/insecure execution boundary. The simulator keeps the practical pieces that addons rely on: protected-frame gating, the dual-environment split (`genv` vs `secureenv`), SecureHandler fallbacks, and shallow state/attribute driver application. Per-call taint tracking still comes from the Elune Lua runtime.

## Design Scope

Full retail taint simulation remains out of scope. What the simulator does implement:

- **Protected-frame gating**: `can_change_protected_state_for()` blocks protected mutations when the caller is insecure and the player is in combat
- **Dual Lua environment**: `genv` (`_G`) for addon code vs `secureenv` for Blizzard secure code
- **Elune runtime taint**: `issecure`, `securecall`, `issecurevariable`, `forceinsecure`, and `debug.*taint*` helpers come from Elune
- **securecallmethod()**: simulator-provided helper that Elune omits
- **Secret values**: simulator-owned identity values and tainted `CallMethod` payloads/results are tracked by fallback accessors
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

`Protect()` is implemented in `src/lua_api/frame/methods/misc/secret.rs`. It only sets `frame.is_protected` for secure callers; insecure callers silently fail.

## Dual Lua Environment (`src/lua_api/globals/security.rs`)

Two environments share the same Lua state:

- **genv** (`_G`): addon code; `Blizzard_EnvironmentCleanup` nils secure APIs here
- **secureenv**: shallow copy of `_G` at startup with `__index = _G` fallback; retains secure APIs after cleanup; addons with `UseSecureEnvironment: 1` in TOC run here via `setfenv`

`set_in_both_envs_rilua(key, value)` registers named frames in both environments so frame globals are visible from both.

## Elune Runtime Functions

Elune provides `issecure`, `issecurevariable`, `securecall`, `securecallfunction`, `forceinsecure`, `hooksecurefunc`, `secureexecuterange`, and the `debug.*taint*` helpers. The simulator relies on these VM-level functions instead of replacing them.

`securecallmethod(obj, name, ...)` calls `obj[name](obj, ...)` via protected pcall dispatch and returns `nil` on missing/non-function/error paths.

Frame method `CallMethod` is registered in `src/lua_api/frame/methods/text_attribute_event/mod.rs`. It preserves return values and marks tainted caller arguments/results as simulator secret values so insecure data cannot be laundered through secure snippets.

## Secret Values

`src/lua_api/globals/security.rs` provides fallback accessors for secret values:

- `issecretvalue(value)` returns true for values explicitly marked with the simulator secret marker, Elune-tainted functions, tables with tainted slots, and tables containing nested secret keys or values.
- `canaccessvalue(value)`, `canaccessallvalues(...)`, and `canaccesstable(table)` return false when those same checks find a secret value.
- `scrub()` and `scrubsecretvalues()` are still pass-throughs.

Current simulator-owned secret values include party/raid identity strings returned through unit/group APIs and tainted `CallMethod` payloads/results.

## SecureHandler Fallback

`src/lua_api/globals/security.rs` installs a Lua-side fallback for the SecureHandler APIs before `Blizzard_RestrictedAddOnEnvironment` arrives.

- `SecureHandlerSetFrameRef(frame, label, refFrame)` stores frame refs in weak-keyed registries
- `SecureHandlerGetFrameRef(frame, label)` reads those stored refs back
- `SecureHandlerExecute(frame, body, ...)` compiles `body` into a restricted closure and runs it with `pcall`
- `SecureHandlerWrapScript(frame, script, header, preBody, postBody)` wraps the original handler with pre/original/post callbacks
- `SecureHandlerUnwrapScript(frame, script)` restores the original handler

`SecureHandlerExecute` snippets run in a locked restricted environment, not the full `_G`. The fallback exposes utility functions/tables such as `assert`, `error`, `ipairs`, `math`, `next`, `pairs`, `print`, `select`, `string`, `tonumber`, `tostring`, `type`, and `unpack`; the `math` and `string` tables are read-only copies.

## State Drivers

`RegisterStateDriver`, `UnregisterStateDriver`, `RegisterAttributeDriver`, `UnregisterAttributeDriver` are backed by `SimState.secure_attribute_drivers`.

- `RegisterStateDriver(frame, "visibility", ...)` maps to the `state-visibility` special case and toggles visibility plus `statehidden`
- Other state/attribute drivers resolve the final clause of the driver string and write that value directly onto the frame
- `Unregister*` removes the stored driver text but leaves the last applied frame state in place

Driver limitations:

- Conditional grammar is not fully evaluated in this path.
- Driver values are not automatically reevaluated on every relevant state transition.
- This is a compatibility fallback for addon bootstrap, not a full `SecureStateDriverManager`.

## Blizzard `issecure()` Call-Sites

These are the real Blizzard Lua paths the simulator executes today that branch on `issecure()` or pass its value into secure APIs. They are the practical end-to-end checks for Elune taint integration.

### Registration and command routing

- `Blizzard_ChatFrameBase/Shared/SlashCommands.lua` uses `issecure()` to choose between secure slash-command registration and the insecure fallback registry.
- `Blizzard_ChatFrameBase/Shared/SlashCommandsRegistry.lua` uses `issecure()` to decide whether secure slash-command aliases can be added, and rejects insecure `AddSecureCmd()` calls.
- `Blizzard_SharedXMLBase/Mixin.lua` uses `issecure()` to allow secure mixin copying only during the secure bootstrap path.

### UI actions gated by secure state

- `Blizzard_UnitPopupShared/UnitPopupSharedButtonMixins.lua` uses `issecure()` to hide insecurely-invoked unit popup actions that would otherwise target or promote players.
- `Blizzard_SharedXMLBase/CvarUtil.lua` uses `issecure()` to decide whether a CVar read can be cached without tainting later reads.
- `Blizzard_DebugTools/DebugObjectUtil.lua` uses `issecure()` to allow object access when secure, even if the target is forbidden.
- `Blizzard_StaticPopup/StaticPopup.lua` uses `issecure()` to block secure edit-box dialogs from tainted callers.
- `Blizzard_GroupFinder/Mainline/LFGList.lua` uses `issecure()` to either begin a secure search or show an insecure-search warning popup.
- `Blizzard_ActionBar/WoWLabs/ActionButtonOverrides.lua` and `Blizzard_ActionBar/Shared/ActionButton.lua` use `issecure()` to gate action-bar grid state updates.
- `Blizzard_UIParentPanelManager/Shared/UIParentPanelManager.lua` uses `issecure()` with combat lockdown to gate panel show/hide operations.
- `Blizzard_SharedXMLGame/Tooltip/TooltipDataHandler.lua` uses `issecure()` to register secure callbacks directly or wrap insecure callbacks with `forceinsecure()`.
- `Blizzard_EditMode/Shared/EditModeManager.lua` uses `issecure()` with combat lockdown to choose the secure delegate path for clearing selected systems.

### Secure-environment plumbing

- `Blizzard_ScriptErrors/Blizzard_ScriptErrors.lua` and `Blizzard_ScriptErrorsFrame/Blizzard_ScriptErrorsFrame.lua` assert secure execution during their startup wiring.
- `Blizzard_NamePlates/Blizzard_NamePlates.lua` passes `issecure()` into `C_NamePlate` lookup APIs so secure and insecure views resolve differently.
- `Blizzard_NewPlayerExperience/Blizzard_TutorialTutorials.lua` passes `issecure()` into `C_NamePlate.GetNamePlateForUnit()` for the same reason.
- `Blizzard_RestrictedAddOnEnvironment/SecureHoverDriver.lua` uses `issecure()` to pick secure auto-hide helpers when possible, otherwise it falls back to attribute-driven emulation.
- `Blizzard_RestrictedAddOnEnvironment/SecureHandlers.lua`, `RestrictedInfrastructure.lua`, and `RestrictedExecution.lua` use `issecure()` as the guard for secure-handler APIs, restricted table mutation, frame-handle namespace initialization, forbidden-frame propagation, and restricted closure execution.

### End-to-end coverage in the simulator

- `tests/security_api.rs` covers the base Elune contract: `issecure()`, `forceinsecure()`, `loadstring()` tainting, and `securecall()` restoring secure execution.
- `tests/protected_frame_enforcement.rs` covers the combat/insecure gates that many `issecure()` branches are protecting.
- `tests/secure_handler_fallback.rs` covers the SecureHandler fallback path that runs before `Blizzard_RestrictedAddOnEnvironment` loads.
- `tests/secure_group_headers.rs` covers the secure group-header path after `Blizzard_RestrictedAddOnEnvironment` loads.
- `tests/startup_warnings.rs` and `tests/load_order.rs` cover the Blizzard addon startup/load path where these call sites are exercised together.

## Sources

- [protected-frame-enforcement.md](../../protected-frame-enforcement.md) — protected-frame behavior and remaining gaps
- `src/lua_api/frame/methods/methods_helpers.rs` — protected-state gating and `ADDON_ACTION_BLOCKED`
- `src/lua_api/globals/security.rs` — taint helpers, `securecallmethod`, SecureHandler fallback, state/attribute drivers, secure environment
- `src/lua_api/state.rs` — `secure_attribute_drivers` storage
- `src/loader/lua_file.rs` — per-addon compiled-closure taint stamping
- `src/lua_api/env.rs` — frame script-handler taint stamping
- `tests/protected_frame_enforcement.rs` — combat lockdown coverage
- `tests/secure_handler_fallback.rs` — SecureHandler fallback coverage
- `tests/security_api.rs` — state driver and `securecallmethod` coverage

Removed/stale paths that older docs may mention:

- `src/lua_api/globals/security_api.rs`
- `src/lua_api/secure_env.rs`
- `src/lua_api/frame/methods/combat_lockdown.rs`

## See Also

- [[lua-api]] — issecure, securecall, hooksecurefunc globals
- [[event-system]] — ADDON_ACTION_BLOCKED event firing
- [[frame-data-flow]] — frame is_protected field and Protect() method
- [[protected-frames]] — focused protected-frame enforcement notes
