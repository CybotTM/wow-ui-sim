# XML Template System

WoW UI definitions declared in XML are parsed into typed Rust structs, then converted to widgets by generating and executing Lua code. Virtual frames become reusable templates stored in a global registry and applied when frames inherit from them.

## XML Format and Parsing

Every WoW XML file has a `<Ui>` root deserializing into `UiXml { elements: Vec<XmlElement> }` via `quick_xml` serde. Tag names map to enum variants via `#[serde(rename_all = "PascalCase")]`.

**XmlElement** (30+ variants): Frame-like widgets (Frame, Button, CheckButton, EditBox, ScrollFrame, Slider, StatusBar, GameTooltip, ModelScene, etc.) all map to `FrameXml`. Regions: `Texture(TextureXml)`, `FontString(FontStringXml)`. File refs: `Script`, `Include` (both with lowercase variants). Font defs: `Font`, `FontFamily`. Container: `ScopedModifier` (transparent wrapper).

**FrameXml** key attributes: `name`, `parent`, `parentKey` (property on parent), `inherits` (comma-separated templates), `mixin`, `virtual/intrinsic` (template-only), `hidden`, `alpha`, `setAllPoints`, `enableMouse`, `parentArray` (appends to parent array). Child elements via `FrameChildElement`: Size, Anchors, Layers, Frames, Scripts, Animations, button-specific textures, widget-specific fields.

## Template Registry

Virtual/intrinsic frames are registered in a process-global `OnceLock<RwLock<HashMap<String, TemplateEntry>>>` and not instantiated:

```rust
pub struct TemplateEntry { pub name: String, pub widget_type: String, pub frame: FrameXml }
```

A separate registry holds virtual texture templates for mixin chain resolution.

## Inheritance Chain Resolution

`get_template_chain(names: &str) -> Vec<TemplateEntry>` splits comma-separated names and recursively follows each template's own `inherits`, depth-first with cycle detection. Returns base-to-derived order — for `inherits="A, B"` where A inherits C: chain is `[C, A, B]`.

Property resolution per template walk:
- **Size**: most-derived wins per dimension; frame's own overrides all. Partial XML sizes (`<Size x="..."/>` or `<Size y="..."/>`) apply only the declared dimension.
- **Anchors**: frame's own if present; otherwise most-derived template with anchors
- **Mixins**: accumulated base-to-derived, then frame's own (duplicates skipped)
- **KeyValues**: later values overwrite; frame's own applied last
- **Hidden**: first template with a value wins (break on hit)

## XML-to-Widget Conversion (`src/loader/xml_frame.rs`)

`create_frame_from_xml()` pipeline for each non-virtual frame:
1. Virtual/intrinsic check — register template and return early
2. Name resolution — `$parent` substitution, `__anon_{id}` for anonymous children
3. Build Lua string: `CreateFrame(type, name, parent, inherits)` + `Mixin()`, `SetSize()`, `SetPoint()`, `Hide()`, `EnableMouse()`, `SetScript()`, etc.
4. Execute single `env.exec()` call
5. Recurse into `<Frames>` children, then `<Layers>` (textures/fontstrings)
6. Apply animation groups, button textures, button text
7. Fire lifecycle scripts: OnLoad, then OnShow if visible

The `inherits` parameter in `CreateFrame()` triggers `apply_templates_from_registry()` at runtime, so template children are created before the XML loader recurses into direct children.

## Lua-Side Template Application (`src/lua_api/globals/template/mod.rs`)

Called from `CreateFrame()` at runtime (no `LoaderEnv` access). `apply_single_template()` order: Mixin → Size → Anchors → SetAllPoints → KeyValues → Layers → button textures → child frames → Scripts. OnLoad for ALL template-created children is deferred until after the entire chain is applied.

## Inline Scripts

Three `ScriptBodyXml` forms:
- `function="X"` — uses X directly
- `method="X"` — wraps as `function(self, ...) self:X(...) end`
- Inline body — wraps as `function(self, ...) <body> end`

`inherit="prepend"` or `"append"` chains new/existing handlers, both wrapped in `pcall`. Without `inherit`, new handler replaces old.

## Name Substitution and parentKey

`$parent` in frame names resolves to the actual parent name. `$parent.ScrollBox` in `relativeKey` resolves to `parent["ScrollBox"]`; `$parent.$parent.X` chains via `GetParent()`.

`parentKey="Title"` produces `parent.Title = frame`. `parentArray="Buttons"` appends to `parent.Buttons`. Both resolved via template inheritance.

## Sources

- [xml-template-system.md](../../xml-template-system.md) — XML types, registry, inheritance, conversion pipeline, inline scripts

## See Also

- [[addon-loading]] — TOC parsing and per-file XML/Lua loading that feeds this system
- [[widget-system]] — WidgetType and Frame structs produced by XML conversion
- [[frame-data-flow]] — Mixin() application and __frame_fields storage
