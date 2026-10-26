# Widget, texture and render crate boundaries

The root facade preserves API paths while reusable state and rendering code compile
without selecting a client profile. All seven profile all-target checks pass with
zero warnings; Retail stock startup is unchanged. Measured compile effect is mixed
(see Measurements); no repeatable gain is claimed.

## Ownership

- `sim-widget`: anchor, frame value/enumeration, widget identity, SimpleHTML style,
  atlas geometry, BlendMode and LayoutRect types. Frame, EditBox, registry and
  layout remain in root to preserve the original profile-dependent constructors
  and exact cooldown cfg without adding leaf epoch features.
- `sim-texture`: BLP/RGBA decoding, dimensions, crop/cache operations, directory
  observations and shared logging statics. No client or CASC dependency.
- `sim-render`: markup stripping and font measurement arithmetic; with `gui`, quad
  batches and canvas drawing. GPU atlas/pipeline/primitive/program remain in root.
  GPU code consumes texture/widget values, not Lua or simulator state.

Only `gui` is forwarded to the new leaves, to preserve optional GPU dependencies.
No client bundles or epoch features are forwarded. Root paths use re-exports,
including the same BlendMode, LayoutRect and nine-slice type identities.

## Deliberately retained composition

EditBox constructors and helpers directly inspect the active client profile.
Their types are retained by Frame presentation, and layout consumes Frame and
WidgetRegistry. Moving this coupled ownership requires signature/body changes
or a new interface. Keep it in root for this move-only phase; EditBox and its
caller are byte-identical to phase 2. The cooldown impl also retains its exact
`#[cfg(any(feature = "gui", feature = "retail-12-0-5"))]` guard in root.

TextureManager's inherent resolve/preload methods use selected CASC configuration,
atlases and spellbook state. Moving the manager would require a new resolver or
preload interface. The unchanged root preload module therefore remains above the
reusable decoding crate, including the spellbook-dependent enumeration.

WowFontSystem construction selects font files and CASC configuration. Moving its
inherent constructor requires a new loader interface or wrapper. Construction and
shaping remain together in root; glyph emission and headless orchestration remain
above those root types. Font/markup measurement stays below Lua, not in Lua APIs.

## Test boundaries and proof

Value-type/identity unit tests move with their code; Frame/layout tests stay in root. Texture manager/preload and
renderer-crossing tests remain in root. All shader cases and their cfg(test) pipeline snapshots remain in root.
The BC test swap hook and re-export retain their exact #[cfg(test)] guards and
share the production root atomic. The GPU ownership cut was reduced rather than
changing test-hook availability. Assertions and function names are unchanged. The root integration and binary files are
unchanged. Proof covers compilation of all seven profiles (all targets, zero warnings)
and Retail stock startup; the Rust test suites were not executed for this cut.

## Measurements

Warm incremental rebuilds, Retail, same slot, master `3b369ee1a` versus this split;
single sample each, CPU seconds (wall seconds):

| Workload | Master | Split |
|---|---:|---:|
| Root crate, mtime-only touch | 46.6 (58.6) | 40.6 (51.2) |
| Root crate, one-line edit | 50.5 (69.9) | 36.4 (53.6) |
| Integration tests, mtime-only touch | 46.5 (55.8) | 54.0 (62.1) |
| Integration tests, one-line edit | 58.9 (65.5) | 51.4 (58.1) |

Limits: one sample per cell on a shared host, so load noise is not bounded; the
integration touch regressed while the other three improved. Cold builds were not
measured, and the initial warm-ups reused unequal existing caches.

## Sources

- [Widget boundary](../../../crates/sim-widget/README.md)
- [Texture boundary](../../../crates/sim-texture/README.md)
- [Render boundary](../../../crates/sim-render/README.md)
- [Root facade](../../../src/lib.rs)

## See Also

- [[generated-data-crates]] — the preceding data cut and its separate proof.
- [[client-profiles]] — runtime client/epoch selection stays in composition.
