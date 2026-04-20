# Keybinding System

WoW keybinding system implementation: actions defined with Lua code, keys mapped to actions, dispatched on key press when no EditBox has focus.

## Architecture

Bindings are stored in two Lua registry tables:

- `__wow_binding_actions` — action definitions (action name → `{ action, lua_code }`) plus numeric index for enumeration
- `__wow_key_bindings` — key assignments (key name → action name)

## Key Press Pipeline

```
iced KeyPressed → iced_key_to_wow() → Message::KeyPress(key_name)
  → WowLuaEnv::send_key_press(key)
    → ESCAPE: special (UISpecialFrames, GameMenuFrame)
    → Other:
        1. Focused EditBox special handlers (Enter, Tab, Space)
        2. If NOT EditBox focused: lookup __wow_key_bindings → execute Lua code
        3. OnKeyDown dispatch with parent propagation
```

## Mainline Spellbook Follow-up

- `S` now routes to mainline `PlayerSpellsUtil.ToggleSpellBookFrame()`.
- `runtime_surface_bootstrap.lua` no longer owns a spellbook fallback wrapper.
- `dispatch_key_binding()` now resolves simple zero-arg global/table function
  paths directly instead of always compiling a Lua chunk first.

Why this matters:

- Raw `PlayerSpellsUtil.ToggleSpellBookFrame()` calls already matched Blizzard
  load-on-demand behavior.
- The remaining regression only happened through binding dispatch: chunk-eval
  keybind execution could leave `PlayerSpellsFrame` in a partial open state on
  first press even though the same Blizzard function worked when invoked
  directly.
- Direct function dispatch keeps Blizzard ownership of the spellbook logic
  while avoiding the binding-only partial-open path.

## Default Key Assignments (simulator-specific)

| Key | Panel |
|---|---|
| B | All bags (`ToggleAllBags()`) |
| C | Character sheet |
| N | Talents (`PlayerSpellsUtil.ToggleClassTalentFrame()`) |
| S | Spellbook |
| M | World map |
| O | Friends list |
| BACKSPACE | Backpack |
| F8–F11 | Individual bags |

Note: Some keys differ from live WoW defaults (WoW uses P for spellbook, Y for achievements, I for LFG). These are simulator-specific overrides.

## Lua API

- `GetBindingKey(action)` → up to 2 keys
- `GetBindingAction(key)` → action name
- `SetBinding(key, action)` → bind/unbind
- `SetBindingClick/Spell/Item/Macro` → no-op stubs
- `SaveBindings` / `LoadBindings` → no-op (bindings reset each session)

## Modifier Keys

Shift/Ctrl/Alt are not currently combined into key names — no `SHIFT-B` style bindings yet.

## Adding Bindings

Add to `BINDING_ACTIONS` and optionally `DEFAULT_KEYS` in `src/lua_api/keybindings.rs`. The `lua_code` string is executed directly via `lua.load(&code).exec()`.

## Key Files

- `src/lua_api/keybindings.rs` — storage, defaults, dispatch
- `src/lua_api/key_dispatch.rs` — key press pipeline
- `src/iced_app/keybinds.rs` — iced key → WoW key name conversion

## Sources

- [keybinding-system.md](../../keybinding-system.md) — full system description
