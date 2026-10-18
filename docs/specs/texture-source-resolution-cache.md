# Texture source resolution cache

Environment-owned source lookup reduces repeated filesystem enumeration in `SetTexture` and rendering without changing texture selection. Implementation: `src/texture/source_index.rs`; [mechanics](../wiki/systems/texture-source-resolution-cache.md).

## What it must do

- [ ] Share one source resolver between an environment's setters and renderers; isolate environments and binary client profiles.
- [ ] Preserve normalized request spelling, ordered addon roots, exact-before-case-insensitive extension precedence, file/directory distinctions and existing return values.
- [ ] Discover file creation/deletion and root changes without retaining negative decisions; invalidate source listings on root changes and UI reload.
- [ ] Repeated assignments retain field, secret, atlas, color and tiling effects, including nil/empty clears.
- [ ] Bound directory opens for repeated mixed-case requests, measured against a concrete filesystem fixture rather than internal calls.

## How it works

- [Source lookup ownership and invalidation](../wiki/systems/texture-source-resolution-cache.md).

## Implementation inventory

- `src/texture{.rs,/resolve.rs,/source_index.rs}` — source enumeration, metadata freshness and CPU load retries.
- `src/lua_api/state{.rs,/sim_state.rs}` — per-environment manager and root synchronization.
- `src/lua_api/frame/methods/widgets/texture/atlas.rs` — setter existence checks; assignment logic unchanged.
- `src/iced_app/` and `src/bin/wow_sim/gui_commands.rs` — environment-associated renderers share the manager.
- `src/lua_api/globals/state_backed_queries.rs` — Lua reload invalidation.
- `src/dump/display.rs` — standalone diagnostics retain no global source cache.

## Tests asserting this spec

- `tests/texture_source_cache.rs` — live fixtures, root/environment isolation, setter effects, CPU miss recovery and calibrated Linux directory-open observation.
- `tests/texture_set_texture_result.rs`, `src/texture/tests.rs` — existing result and source-selection contracts.
- `tests/wowforever_texture_metatable.rs`, `tests/aura_secret_display.rs`, `tests/texture_file_dimensions.rs` — profile and secret controls.
- `tests/methods_texture.rs`, `tests/texture_atlas_regressions.rs`, `tests/methods_button_textures.rs` — atlas, tiling and dirty controls.

## Known gaps (current cycle)

- [ ] GREEN and preservation evidence pending.

## Out of scope

- Decoded pixel, dimensions, crop and GPU-handle hot reload: source lookup freshness is not rendered-byte freshness.
- Startup timer yielding or real Ellesmere first-draw/IPC acceptance: this fix only addresses measured texture resolution work.
- File-dimension getters retain their independent metadata-only lifetime; no persistent dimension-cache behavior change.
