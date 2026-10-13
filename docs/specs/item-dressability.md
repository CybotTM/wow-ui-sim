# Item dressability

Retail/PTR `C_Item.IsDressableItemByID(itemInfo)` projects an explicit host-supplied per-item boolean. Its C API home is `src/c_api/item_spell/c_item.rs`; no implementation is added by the integrated test/spec checkpoint `13a818ba5`. Profile-scoped native declarations establish a required `ItemInfo`, `AllowedWhenUntainted`, and one non-nil boolean result. Empty defaults and supplied classification are user-approved simulator policy, not a native item catalog claim.

## What it must do

### API and security

- [ ] Expose the Retail/PTR public API and return exactly one boolean for numeric identities and supported item links, including the existing generated hyperlink consumed by PlayerChoice.
- [ ] Reject absent/nil arguments and values outside the supported ItemInfo input types. Do not invent numeric range, coercion, item-name resolution, or malformed-string grammar contracts.
- [ ] Authenticate original VM secrets before parsing or lookup: untainted callers may query wrapped supported inputs; tainted callers must be denied wrapped inputs, including malformed/nil payloads. Ordinary inputs remain usable by tainted callers. Preserve wrapper identity and caller taint through denial and subsequent secure recovery.

### Approved host input

- [ ] Each environment starts with empty explicit per-item boolean records; an absent record yields false. Generated metadata, equipment/consumable sets, ownership and mount relations do not seed or infer this classification.
- [ ] Supplied true and supplied false project directly through numeric identity and corresponding supported item links. Neither projection requires a generated catalog entry, appearance record, or 3D model.
- [ ] Host insertion, replacement and removal are visible on the next query; removal restores the false miss. Inputs remain isolated between environments and do not mutate mount relations or equipment/consumable classification.

### Real consumer

- [ ] Preserve the whole profile-scoped cached PlayerChoice/ItemButton/SharedXML closure, original XML reward template, original ItemButton SetItem/GetItemInfo path and original IsDressupReward identity/provenance. Supplied classification must drive the original consumer without replacing globals or vendor methods.
- [ ] With the mount relation absent, supplier-controlled false/true/false classification for the existing real item yields the corresponding original consumer contrast. This asserts the supplied policy, not native Hearthstone dressability.
- [ ] Preserve independent supplied mount-relation behavior: false dressability must not suppress a mapped mount; changing the mount relation must not change the direct dressability projection.

## How it works

- [Lua/C API integration](../lua-api.md).
- [Independent mount relation contract](mount-item-relations.md).

## Implementation inventory

- `src/c_api/item_spell/c_item.rs` — existing C_Item registration and numeric/link parsing context; new predicate not implemented by this stage.
- `src/c_api/item_spell/helpers.rs` — existing metadata projections; not authoritative dressability inputs.
- `tests/item_dressability.rs` — two initial real-WowLuaEnv API cases integrated without production behavior; PTR initial RED independently classified, Retail execution pending. Public lookup, not raw registration-table shape, is the contract.
- `tests/mount_item_player_choice.rs` — existing whole cached consumer; later supplier contrast must retain its original loader/method path.

## Tests asserting this spec

Integrated `tests/item_dressability.rs` contains exactly two initial cases:

- `item_dressability_empty_input_returns_exactly_one_boolean_false` — numeric ID and supported bare/generated item-link defaults, type and arity.
- `item_dressability_requires_item_info_and_authenticates_vm_secrets` — required argument/type rejection, authentic wrapped numeric/link queries, malformed wrapped payloads, tainted denial and secure recovery.

Initial PTR API RED at `13a818ba5` executed two cases: 0 PASS / 2 FAIL, exit101, independently classified with successful environment initialization. First numeric miss passed exact-one-return arity then failed `empty supplied classification must return boolean false`; actual value/type was not printed. First no-argument call failed the combined rejection/string-message assertion `required ItemInfo must reject input`; successful return versus non-string error is not distinguished by output. Later selectors and all secret/taint assertions were unreached. Retail execution remains pending. No passing contract checkboxes or modeled/native consumer coverage follow from this RED.

## Known gaps (current cycle)

- [ ] Execute initial Retail cases before adding production behavior; PTR genuine RED is recorded above with actual boundaries and unreached assertions.
- [ ] After passive host inputs exist, add and execute host-state RED for explicit true/false, uncatalogued supplied identity, link identity, replacement/removal, environment isolation and independent mount/equipment/consumable state. No new production types/state are staged now.
- [ ] After passive host inputs exist, add and execute real-consumer RED through the unchanged whole cached closure with supplied classification and independent mount controls. No sliced/recreated/mocked consumer.
- [ ] Producer, mapped-secret proof, earlier-profile preservation and broader acceptance remain main-owned later work.

## Out of scope

Native catalog/classification, ownership or transmog eligibility inference and 3D rendering: not authorized or established by declarations. No inference from Hearthstone metadata, inventory type, equipment sets or mount membership. Numeric-domain/coercion/name grammar and malformed-string outcomes are not specified by this bounded declaration evidence. This stage excludes production changes and all operational execution; C API implementation remains in the broader approved scope.
