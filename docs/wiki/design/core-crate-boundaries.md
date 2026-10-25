# Widget, texture and render crate boundaries

The root facade preserves API paths while reusable state and rendering code compile
without selecting a client profile. These cuts are statically staged; compile CPU
savings and seven-profile compatibility are not yet measured.

## Ownership

- `sim-widget`: widget registry, frames, SimpleHTML styles, atlas geometry and layout,
  including line/slider resolution and unit fixtures. Root supplies the existing
  Wrath comparison to EditBox presentation. Pure cooldown arithmetic is always
  compiled; its root callers retain their capability guards.
- `sim-texture`: BLP/RGBA decoding, dimensions, crop/cache operations, directory
  observations and shared logging statics. No client or CASC dependency.
- `sim-render`: markup stripping and font measurement arithmetic; with `gui`, quad
  batches and canvas drawing. GPU atlas/pipeline/primitive/program remain in root.
  GPU code consumes texture/widget values, not Lua or simulator state.

Only `gui` is forwarded to the new leaves, to preserve optional GPU dependencies.
No client bundles or epoch features are forwarded. Root paths use re-exports,
including the same BlendMode, LayoutRect and nine-slice type identities.

## Deliberately retained composition

TextureManager's inherent resolve/preload methods use selected CASC configuration,
atlases and spellbook state. Moving the manager would require a new resolver or
preload interface. The unchanged root preload module therefore remains above the
reusable decoding crate, including the spellbook-dependent enumeration.

WowFontSystem construction selects font files and CASC configuration. Moving its
inherent constructor requires a new loader interface or wrapper. Construction and
shaping remain together in root; glyph emission and headless orchestration remain
above those root types. Font/markup measurement stays below Lua, not in Lua APIs.

## Test boundaries and proof

Widget/layout unit tests move with their code. Texture manager/preload and
renderer-crossing tests remain in root. All shader cases and their cfg(test) pipeline snapshots remain in root.
The BC test swap hook and re-export retain their exact #[cfg(test)] guards and
share the production root atomic. The GPU ownership cut was reduced rather than
changing test-hook availability. Assertions and function names are unchanged. The root integration and binary files are
unchanged. Compilation, test execution and compile-CPU proof await a main-owned slot.

## Sources

- [Widget boundary](../../../crates/sim-widget/README.md)
- [Texture boundary](../../../crates/sim-texture/README.md)
- [Render boundary](../../../crates/sim-render/README.md)
- [Root facade](../../../src/lib.rs)

## See Also

- [[generated-data-crates]] — the preceding data cut and its separate proof.
- [[client-profiles]] — runtime client/epoch selection stays in composition.
