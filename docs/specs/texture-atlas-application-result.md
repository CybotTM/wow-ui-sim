# Texture atlas application result

`Texture:SetAtlas` application-result policy supports unchanged cached Blizzard TextureKit consumers. This is an inferred **simulator compatibility policy**, not established native return parity. Source: [atlas methods](../../src/lua_api/frame/methods/widgets/texture/atlas.rs); [rendering architecture](../texture-atlas-system.md).

## What it must do

- [ ] A successfully resolved and applied atlas returns exactly one boolean `true`.
- [ ] An accepted input that does not apply an atlas returns exactly one boolean `false`; existing argument errors and mutation behavior remain unchanged.
- [ ] Real cached CaptureBar TextureKit consumers keep available neutral glow/spark textures visible, then consume updates, hidden/restored records, removal and unregister without vendor changes.

## How it works

- [Texture atlas system](../texture-atlas-system.md).
- [UI widget supplied-state contract](ui-widget-manager-state.md).

## Implementation inventory

- `src/lua_api/frame/methods/widgets/texture/atlas.rs`: atlas argument validation, lookup and application.

## Tests asserting this spec

- `tests/set_atlas_arguments.rs`: exact return count, known application, missing application and existing argument-validation regression.
- `tests/ui_widget_manager_bars_consumer.rs`: actual cached Lua/XML bar consumer; assertion59 currently fails, later lifecycle phases remain unreached.

## Known gaps (current cycle)

- [ ] Focused application-result RED and visibility diagnostics pending execution; production remains unchanged.
- [ ] Consumer GREEN, texture-call regressions, startup and affected profile acceptance pending.

## Out of scope

- Native return parity: cached generated `SimpleTextureBaseAPIDocumentation.lua` has no Returns for SetAtlas; current Wowless `TextureBase.SetAtlas` schema has empty outputs. Cached `TextureUtil.lua::SetupTextureKitOnFrame` nevertheless assigns SetAtlas's result to `success` and uses it for `SetShown`. This conflict is retained, not resolved by invented native evidence.
- Vendor edits, atlas-catalog fabrication, changing argument validation or unrelated texture behavior.
