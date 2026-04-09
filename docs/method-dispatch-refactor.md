# Method Dispatch Refactor

## Status

The runtime pollution bug is fixed.

Actual `frame.Method` lookup now goes through the shared FrameRef metatable wrapper in
`src/lua_api/frame/metatable.rs`, which:

1. checks per-frame mixin overrides and custom fields
2. rejects shared Rust methods that are not allowed for the frame's widget type
3. falls through to mlua's registered method table only for allowed methods

The allow-list comes from `src/lua_api/frame/method_registry/*.rs`, and the visible
`getmetatable(frame).__index` tables are still built from that same registry in
`src/lua_api/globals_legacy.rs`.

That means:

- runtime property access no longer exposes wrong-type methods such as
  `Button:GetScrollChild`
- Wowless-style metatable enumeration still sees the per-type surface
- the remaining work is static cleanup of the diff inventory, not a broken runtime dispatch path

## Current Architecture

There is still one shared mlua UserData method table for `FrameRef`, so this is not yet the
"direct Rust dispatch, no shared method table" end state described below.

Today the system is split like this:

- runtime lookup: `src/lua_api/frame/metatable.rs`
- per-type method ownership: `src/lua_api/frame/method_registry/*.rs`
- per-type metatable exposure for tests/discovery: `src/lua_api/globals_legacy.rs`

This is acceptable for correctness. The remaining downside is architectural duplication:
method ownership lives in the registry, but actual callable implementations still live in the
shared mlua registration path.

## Target Architecture

Dispatch directly in Rust. No Lua methods table.

```
__index(ud, key):
  1. mixin_overrides[frame_id][key]     — Mixin() shadows
  2. children_keys[key]                  — child frame lookup
  3. custom_fields[frame_id][key]        — script handlers, properties
  4. rust_dispatch(widget_type, key)     — direct Rust match, no Lua table
```

Step 4 would resolve the method directly from the widget type enum + method name, returning the
Rust function. No intermediate Lua table, no shared-method fallback.

### Method Registration

Instead of building a `mlua::Table` with all methods at startup:

```rust
// Current: register into Lua table
methods_table.set("GetName", lua.create_function(...))?;
methods_table.set("SetPoint", lua.create_function(...))?;
```

Build a `HashMap<&str, mlua::Function>` per widget type, or a single `HashMap<(WidgetType, &str), mlua::Function>`:

```rust
// Target: Rust-side dispatch
fn get_method(lua: &Lua, wtype: WidgetType, name: &str) -> Option<mlua::Function>
```

Global methods (92) resolve for all types. Type-specific methods resolve only for matching types. Unknown methods return None (no passthrough hack needed).

## Remaining Work

The next method-dispatch tasks are:

- keep trimming `diff_methods_extra.txt` by fixing registry ownership for types that still expose
  too many methods in discovery
- add diff-driven coverage so future method-surface changes fail only when intentional
- optionally collapse runtime dispatch and metatable exposure onto one direct Rust lookup path

### Per-Type Metatable for Wowless

The visible `getmetatable(f).__index` table (used by Wowless to enumerate methods) can be built from the same data — iterate global + type-specific methods and populate a Lua table per type. This is read once for the test, not on every property access.
