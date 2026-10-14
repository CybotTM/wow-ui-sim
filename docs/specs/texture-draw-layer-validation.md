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

- `src/lua_api/frame/methods/button_anchor_hierarchy/hierarchy.rs`: existing `CreateTexture` entry point; producer validation is not part of this stage.
- `tests/texture_draw_layer_validation.rs`: profile-gated integration cases using the region-alpha test pattern.
- `tests/texture_draw_layer_validation/ordinary_names.lua`: shared real-API valid and invalid modes.

## Tests asserting this spec

- `create_texture_accepts_all_five_draw_layers`: five explicit creation/getter roundtrips and region count.
- `create_texture_rejects_invalid_draw_layer_without_adding_region`: five valid baseline creations, then invalid-name rejection and unchanged region count.

These authored Rust tests have not been built or executed. Separate pre-producer normal-Lua Retail evidence reached all five valid roundtrips, then accepted `INVALID` and changed the region count from 5 to 6. The rejection assertion failed; the unchanged-count assertion was unreached. That observation is not executed proof for these new tests or PTR.

## Known gaps (current cycle)

- [ ] Executed Retail/PTR Rust cases and shared-fixture GREEN.
- [ ] Producer rejection and earlier-profile preservation remain unverified by this stage.

## Out of scope

Omitted/nil layers, coercion, secret-value semantics, `CreateFontString`, other creation methods, `SetDrawLayer`, error wording and renderer/native parity. Earlier-profile behavior is preserved without claiming native history.
