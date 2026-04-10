# Frame Data Flow

The simulator maintains two parallel systems that stay in sync through a shared `id: u64`. Rust owns layout and rendering state; Lua owns mixin methods, script handlers, and custom properties.

## Parallel Systems

| System | Storage | Contents |
|--------|---------|----------|
| Rust (`SimState`) | `WidgetRegistry` HashMap | Frame geometry, visibility, event registrations |
| Lua globals | `__frame_fields`, `__scripts` | Mixin methods, handlers, custom properties |

Each `FrameHandle` userdata holds only `id: u64` — everything else is looked up at call time.

## Global Lua Tables

| Table | Key structure | Purpose |
|-------|--------------|---------|
| `__frame_fields[frame_id][key]` | nested: id → string | Mixin methods and custom properties |
| `__scripts["{frame_id}_{handler}"]` | string | Script handler functions |
| `__frame_{frame_id}` | individual globals | Frame userdata for event dispatch |
| `_G["FrameName"]` | named globals | Named frame references |

## Method Lookup Order (`__index`)

When Lua reads `frame.SomeKey`:

1. **mlua method table** — Rust-registered methods (`SetSize`, `SetPoint`, etc.); cannot be overridden from Lua
2. **`children_keys`** — Rust HashMap for child frame refs (`frame.Text`, `frame.NormalTexture`)
3. **`__frame_fields[id]["SomeKey"]`** — mixin methods and custom properties
4. **Fallback stubs** — hardcoded `Clear`, `Lower`, `Raise`
5. **nil**

Critical implication: Rust methods shadow mixin methods. `self.SetShownBase = self.SetShown` stores nil because `self.SetShown` via `__index` step 1 returns the method result only when called, not as a value.

## Property Storage (`__newindex`)

When Lua assigns `frame.Key = value`:
- `FrameHandle` userdata → stored in `children_keys` (Rust) AND `__frame_fields[id]["Key"]`
- Any other value → stored only in `__frame_fields[id]["Key"]`

## Mixin Application

`Mixin(target, MixinTable)` iterates the mixin table and assigns each key to the target via `__newindex`, landing in `__frame_fields`. Applied at two points: inside `apply_templates_from_registry()` (before frame name available in Lua) and again after `CreateFrame()` returns in xml_frame.rs (redundant but harmless).

Template chain order: `[TemplateBase, TemplateA, TemplateB]` (depth-first, parents before children). Within each template: Mixin → Size → Anchors → KeyValues → Layers → Children → Scripts.

## Event Dispatch Flow

```
fire_event("PLAYER_LOGIN")
  ├─ Rust: state.widgets.get_event_listeners("PLAYER_LOGIN") → Vec<u64>
  └─ For each id:
       ├─ Lua: __scripts["{id}_OnEvent"] → handler function
       ├─ Lua: _G["__frame_{id}"] → FrameHandle userdata
       └─ Call: handler(frame, "PLAYER_LOGIN", ...args)
            └─ self:OnEvent(...)  →  __index lookup for "OnEvent"
                 └─ __frame_fields[id]["OnEvent"]  ← from Mixin
```

## Frame Creation Order (xml_frame.rs)

```
1. CreateFrame(type, name, parent, inherits)
   ├─ register_new_frame() → assigns frame_id
   ├─ create_widget_type_defaults() → button textures etc.
   ├─ set _G["name"] and _G["__frame_{id}"]
   └─ apply_templates_from_registry()
       └─ for each template: mixin → size → anchors → keyValues → layers → children → scripts

2. append_parent_key_code() → parent.Key = frame
3. append_mixins_code() → Mixin(frame, ...) again
4. append_size/anchors/hidden/EnableMouse/scripts from frame's own XML

5. create_child_frames(), create_layer_children()
6. apply_animation_groups(), apply_button_textures()
7. fire_lifecycle_scripts() → OnLoad, then OnShow if visible
```

## Known Pitfalls

**Rust method shadow**: `self.SetShownBase = self.SetShown` stores nil. Workaround: simulator pre-initializes aliases like `SetScaleBase` explicitly during `EditModeSystemMixin` application.

**`__frame_{id}` namespace**: anonymous template children must use `__tpl_` prefix (not `__frame_`), since `__frame_{id}` is reserved for event dispatch. Historical collision caused wrong frame to be dispatched for events.

**Script chaining order**: `inherit="prepend"` runs new handler before old. If new handler depends on state the old handler sets up, it will fail on first call.

## Sources

- [frame-data-flow.md](../../frame-data-flow.md) — parallel systems, __index order, Mixin flow, event dispatch, creation sequence, pitfalls

## See Also

- [[lua-api]] — FrameHandle, method submodules, __index/__newindex implementation
- [[event-system]] — fire_event dispatch, __scripts table population
- [[xml-template-system]] — template chain resolution and application order
