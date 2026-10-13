# XML self-relativeKey anchors

Approved simulator policy for the observed ProductChoice path: a Texture XML direct `$parent.<ownParentKey>` self-anchor targets that receiver's parent instead. The generator publishes that single-segment parentKey immediately before its anchors. Direct Lua self-anchoring remains invalid. Source owner: `src/loader/xml_texture.rs`; see [XML templates](../xml-template-system.md). Not a native WoW parity claim.

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
- `src/loader/xml_texture.rs`: normalize only the published direct self-key to `$parent`; retain all other named-key handling and borrow unchanged anchor data without cloning.
- `src/lua_api/frame/methods/button_anchor_hierarchy/anchors.rs`: direct Lua anchor validation and mutation.

## Tests asserting this spec

`tests/xml_self_relative_key.rs` (auto-discovered primary integration target, Mists-only):

- `xml_self_relative_key_product_choice_targets_parent_and_preserves_later_sibling`
- `xml_self_relative_key_direct_lua_self_setpoint_rejects_without_mutation`

## Known gaps (current cycle)

- [x] Execute focused pre-producer Mists RED at `a90f84c5d`: 1 PASS / 1 FAIL, actual exit101. Real prerequisites load; ProductChoice creation fails at layer-child SetPoint self-anchor. Direct Lua rejection/preservation/distinct-target control passes. Two prior admission denials are separate zero-launch records.
- [x] Implement bounded Texture XML direct self-key normalization after executed RED; runtime SetPoint validation and shared frame/font-string generation remain unchanged.
- [ ] Execute both targeted cases against the producer revision, then check Mists normal startup and named prefork control. Implementation is not GREEN or native proof.

## Out of scope

Native parity; synthetic baseline-frame attribution to ProductChoice; generic missing-key fallbacks; other self-reference spellings/aliases; frame/font-string anchor changes; direct Lua validation changes; editor behavior; vendor patches; new ModelScene behavior; startup-wide acceptance.
