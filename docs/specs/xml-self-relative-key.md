# XML self-relativeKey anchors

Approved simulator policy: an XML layer Texture anchor whose `relativeKey` resolves to the receiver targets that receiver's parent instead. Direct Lua self-anchoring remains invalid. Source owner: `src/loader/xml_texture.rs`; see [XML templates](../xml-template-system.md). Not a native WoW parity claim.

## What it must do

- [ ] Instantiate unchanged cached Mists `ProductChoiceItemDisplayTemplate` through real `CreateFrame` inheritance, preserving layers, decorations, scripts and child-frame subtrees with actual cached ModelScene and ProductChoice Lua prerequisites. Mouse handlers must bind to their real cached functions, not missing-global substitutes.
- [ ] Preserve Icon size 63 × 63 and its single CENTER anchor: target exactly parent card, relative point CENTER, offsets (0, 15). Resolving `$parent.Icon` to Icon itself must not install a self-target or abort production.
- [ ] Create later distinct IconBorder with parent card and a single CENTER anchor targeting sibling Icon, relative point CENTER, offsets (0, -5); complete following child-frame production with the entire Lua error accumulator empty.
- [ ] Direct Lua self-`SetPoint` fails under `pcall` without changing any existing point; a distinct-target call succeeds with exact identity, relative point and offsets, preserving the untouched point.

## How it works

- [XML template loading](../xml-template-system.md)
- [Addon loading pipeline](../addon-loading-pipeline.md)
- [Anchor resolution](../anchor-resolution.md)

## Implementation inventory

- `src/xml/parse.rs`: public cached XML parsing.
- `src/xml/template.rs`: virtual frame and texture registration.
- `src/lua_api/globals/create_frame/template_chain/runtime_loader_effects.rs`: runtime inheritance enters production layer loading.
- `src/loader/xml_layer_batch.rs`: ordered layer production.
- `src/loader/xml_texture.rs`: Texture XML anchor generation; candidate policy owner, not implemented by this stage.
- `src/lua_api/frame/methods/button_anchor_hierarchy/anchors.rs`: direct Lua anchor validation and mutation.

## Tests asserting this spec

`tests/xml_self_relative_key.rs` (auto-discovered primary integration target, Mists-only):

- `xml_self_relative_key_product_choice_targets_parent_and_preserves_later_sibling`
- `xml_self_relative_key_direct_lua_self_setpoint_rejects_without_mutation`

## Known gaps (current cycle)

- [ ] Execute focused RED on a freshly admitted Mists artifact before any producer change. Staged tests have source-only evidence, not compile/runtime proof.
- [ ] Implement bounded XML policy after genuine disputed-anchor RED, then execute both cases against that producer revision.

## Out of scope

Native parity; synthetic baseline-frame attribution to ProductChoice; generic missing-key fallbacks; frame-scope anchor changes; direct Lua validation changes; editor behavior; vendor patches; new ModelScene behavior; startup-wide acceptance.
