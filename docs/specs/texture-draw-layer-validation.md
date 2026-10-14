# Texture creation draw-layer validation

Ordinary explicit draw-layer names passed to `Frame:CreateTexture` must be validated on Retail/PTR. The current entry point lives in `src/lua_api/frame/methods/button_anchor_hierarchy/hierarchy.rs`; [widget architecture](../wiki/systems/widget-system.md) describes region storage. This bounded contract does not establish native WoW parity.

## What it must do

- [ ] Accept `BACKGROUND`, `BORDER`, `ARTWORK`, `OVERLAY` and `HIGHLIGHT`; each created Texture returns that name from `GetDrawLayer` and adds one region.
- [ ] Reject an ordinary invalid name such as `INVALID` through the real Lua API without increasing the parent's `GetNumRegions` count.
- [ ] Apply this new rejection contract only to Retail/PTR; leave earlier-profile behavior unchanged.

## How it works

- [Widget architecture](../wiki/systems/widget-system.md)
- [Lua API architecture](../lua-api.md)

## Implementation inventory

- `src/lua_api/frame/methods/button_anchor_hierarchy/hierarchy.rs`: ordinary Retail/PTR `CreateTexture` validates supplied names before constructing/registering a region; other creation entry points and earlier profiles are unchanged.
- `tests/texture_draw_layer_validation.rs`: profile-gated integration cases using the region-alpha test pattern.
- `tests/texture_draw_layer_validation/ordinary_names.lua`: shared real-API valid and invalid modes.

## Tests asserting this spec

- `create_texture_accepts_all_five_draw_layers`: five explicit creation/getter roundtrips and region count.
- `create_texture_rejects_invalid_draw_layer_without_adding_region`: five valid baseline creations, then invalid-name rejection and unchanged region count.

Source producer `6e085fce9` and test formatting `6bdb02b06` follow tests/spec `ab0`. These authored Rust tests have not been built or executed. The complete shared Lua fixture ran before the producer on frozen Retail/PTR normal executables: valid mode completed five roundtrips and region-count assertions; invalid mode accepted `INVALID` and changed the region count from 5 to 6. The rejection assertion failed (CLI1); unchanged-count assertion and invalid completion marker were unreached. This is shared normal-Lua RED, not new Rust-case execution or producer GREEN. Initial independent Retail RED audit retains its separate first-failure boundary.

## Known gaps (current cycle)

- [ ] Executed Retail/PTR Rust cases and shared-fixture GREEN.
- [ ] Producer GREEN, valid creation preservation, startup diagnostics and check/format remain unverified. Earlier-profile behavior is source-preserved, not freshly executed or native-history proof.

## Out of scope

Omitted/nil layers, coercion, secret-value semantics, `CreateFontString`, other creation methods, `SetDrawLayer`, error wording and renderer/native parity. Earlier-profile behavior is preserved without claiming native history.
