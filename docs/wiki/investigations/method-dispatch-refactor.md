# Method Dispatch Refactor

Refactor of frame method dispatch to fix runtime pollution (wrong-type methods exposed) and align `__index` lookup order with real WoW behavior.

## Current State (Fixed)

The runtime pollution bug is fixed. `frame.Method` lookup goes through the shared FrameRef metatable wrapper in `src/lua_api/frame/metatable.rs`, which:

1. Checks per-frame mixin overrides and custom fields (fenv table — Lua fields first)
2. Rejects shared Rust methods not allowed for the frame's widget type
3. Falls through to mlua's registered method table only for allowed methods

Allow-list source: `src/lua_api/frame/method_registry/*.rs`. The visible `getmetatable(frame).__index` tables (used by Wowless for method enumeration) are still built from the same registry in `src/lua_api/globals_legacy.rs`.

**Key behavioral change**: `__index` now checks the per-frame fenv table (Lua fields/mixin overrides) **before** Rust methods. This matches real WoW. Side effect: EditMode method overrides (`SetPointOverride` etc.) now take effect — see [[editmode-layout]].

## Architectural Split

- Runtime lookup: `src/lua_api/frame/metatable.rs`
- Per-type method ownership: `src/lua_api/frame/method_registry/*.rs`
- Per-type metatable exposure for tests/discovery: `src/lua_api/globals_legacy.rs`

There is still one shared mlua UserData method table for `FrameRef` — not yet the "direct Rust dispatch" end state.

## Target Architecture

```
__index(ud, key):
  1. mixin_overrides[frame_id][key]    — Mixin() shadows
  2. children_keys[key]                — child frame lookup
  3. custom_fields[frame_id][key]      — script handlers, properties
  4. rust_dispatch(widget_type, key)   — direct Rust match, no Lua table
```

Step 4 resolves methods directly from widget type + name via a `HashMap<(WidgetType, &str), mlua::Function>`. No intermediate Lua table, no shared-method fallback.

## Remaining Work

- Trim `diff_methods_extra.txt` (types still exposing too many methods in discovery)
- Add diff-driven coverage so unintentional method-surface changes fail tests
- Optionally collapse runtime dispatch and metatable exposure onto one direct Rust lookup

## Sources

- [method-dispatch-refactor.md](../../method-dispatch-refactor.md) — design and current state

## See Also

- [[editmode-layout]] — regression caused by the Lua-fields-first `__index` change
- [[global-frame-index]] — related `__index` on `_G` for lazy frame lookup
