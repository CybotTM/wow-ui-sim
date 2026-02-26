# Method Dispatch Refactor

## Current Architecture (broken)

Every `frame.Method()` call goes through:

1. `__index` Rust closure fires
2. `methods_table.raw_get(key)` — Lua table lookup to find the method
3. `is_method_allowed(widget_type, key)` — Rust HashSet lookup to filter by type
4. Returns a Lua closure that calls back into Rust

Three indirections per property access. The Lua methods table is pointless — it's a Lua table wrapping Rust functions that we look up from Rust.

## Problem

LightUserData has one shared metatable. All frame types share one `__index`. So a single Lua table holds every method for every type, and we bolt on runtime filtering to hide methods from wrong types.

The filtering only exists for Wowless test compliance (checking that each type's metatable only exposes the correct methods). It's not a correctness guard — calling a wrong-type method just returns a default.

## Target Architecture

Dispatch directly in Rust. No Lua methods table.

```
__index(ud, key):
  1. mixin_overrides[frame_id][key]     — Mixin() shadows
  2. children_keys[key]                  — child frame lookup
  3. custom_fields[frame_id][key]        — script handlers, properties
  4. rust_dispatch(widget_type, key)     — direct Rust match, no Lua table
```

Step 4 resolves the method directly from the widget type enum + method name, returning the Rust function. No intermediate Lua table, no HashSet filtering.

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

### Per-Type Metatable for Wowless

The visible `getmetatable(f).__index` table (used by Wowless to enumerate methods) can be built from the same data — iterate global + type-specific methods and populate a Lua table per type. This is read once for the test, not on every property access.
