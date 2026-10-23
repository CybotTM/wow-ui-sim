# API shape-only contracts

Documented shapes, not invented native behavior, are the acceptance contract for these native-unknown surfaces (decision 2026-10-23). Tests live in `tests/api_shape_*.rs`; original consumer selection follows [consumer selection and proof](../wiki/investigations/consumer-selection-and-red-proof.md). All execution is deferred: no PASS or semantic RED is claimed.

## What it must do

### Stable active list

- [ ] Retail/PTR `C_StableInfo.GetActivePetList()` returns exactly one non-nil `table<PetInfo>`; validate every present record and nested numeric ability table without requiring any entry count, order or slot membership.
- [ ] Original `StableActivePetListMixin:Refresh` accepts actual results without Lua error. An empty table is shape-compatible, not native-confirmed. `uiModelSceneID` documents Default=718, not a required value for every record.

Sources: Retail/PTR `Blizzard_APIDocumentationGenerated/StableInfoDocumentation.lua:15-22,193-214`; selected `Blizzard_StableUI/Blizzard_StableUI.lua:447` via `Blizzard_StableUI/Blizzard_StableUI.toc:4`. Forever excluded: selected Camelot StableUI never calls this API despite cached Standard source and documentation.

### Item upgrade

- [ ] Retail/PTR/Forever/Mists/Era/Anniversary `C_ItemUpgrade.CanUpgradeItem` takes a required non-nil `ItemLocation` and returns exactly one non-nil boolean; no eligible/ineligible value is mandated.
- [ ] Retail/PTR/Forever/Mists original ItemUtil upgrade-context branch accepts actual results with an existing equipped item and original upgrade window, without Lua error. Match versus Mismatch is not prescribed.

Sources: `Blizzard_APIDocumentationGenerated/ItemUpgradeDocumentation.lua:11-24` (Retail/PTR/Forever), `:11-23` (Mists/Era/Anniversary). Selected generic `Blizzard_FrameXMLUtil/ItemUtil.lua:147` (Retail/PTR), `:214` (Forever, corrected generic file, **not Camelot**), `:142` (Mists/Era/Anniversary). FrameXMLUtil TOCs select generic ItemUtil on all six. Era/Anniversary have the utility consumer but cannot normally open its upgrade context: cached `Blizzard_ItemUpgradeUI_Mists.toc:4` restricts that window to Mists. Their context-specific subcheck stays blocked rather than forcing unselected UI or fabricating a frame.

### Store

- [ ] Retail/PTR/Forever/Mists/Era/Anniversary `C_StorePublic.DoesGroupHavePurchaseableProducts` accepts a required non-nil numeric group ID and returns exactly one non-nil boolean.
- [ ] Original `StoreInterfaceUtil.OpenToSubscriptionProduct` accepts the actual group-query result without Lua error on its normal legacy-shop path. Shop2 bypass is a blocked prerequisite, not a passing group-query test. Direct shape checks cover both original category IDs; the second original call remains conditional on the first actual result.

Sources: `Blizzard_APIDocumentationGenerated/StorePublicUIDocumentation.lua:11-24` (Retail/PTR/Forever), `:11-23` (Mists/Era/Anniversary); selected `Blizzard_SharedXML/InterfaceUtil.lua:44,48` (Retail/PTR/Forever), `:32,36` (Mists/Era/Anniversary), selected by SharedXML TOCs. Secure StoreUI is not the selected consumer on five of these profiles and is not forced into coverage.

## How it works

- [Runtime-selected consumers and proof classes](../wiki/investigations/consumer-selection-and-red-proof.md).

## Implementation inventory

- `tests/api_shape_stable_active_list.rs`: Retail/PTR list/record shape and original refresh acceptance; missing member is a shape blocker, not semantic RED.

- `tests/api_shape_item_upgrade.rs`: six-profile argument/boolean shape; original upgrade-context acceptance on four reachable profiles.

- `tests/api_shape_store.rs`: six-profile numeric-group/boolean shape and original InterfaceUtil acceptance.

## Tests asserting this spec

- `api_shape_stable_active_list::stable_active_list_shape_blocker_contract` — source-predicted FAIL for missing member; not executed.

- `api_shape_item_upgrade::item_upgrade_boolean_contract` — expected preservation PASS on six profiles, unexecuted.
- `api_shape_item_upgrade::item_upgrade_original_item_util_contract` — expected preservation PASS on Retail/PTR/Forever/Mists if normal setup succeeds, unexecuted. Era/Anniversary upgrade-context subchecks blocked and unregistered; no consumer acceptance claimed there.

- `api_shape_store::store_original_interface_util_contract` — expected preservation PASS on six profiles with legacy shop prerequisites, unexecuted.

## Known gaps (current cycle)

- [ ] Deferred execution must distinguish setup failures from shape blockers and preservation passes. No producer or vendor changes are authorized here.
- [ ] `GetInventoryItemsForSlot` (#7 q1, Retail/PTR/Forever consumer) and `IsInventoryItemProfessionBag` (#7 q3, Mists/Era/Anniversary consumer): native-unknown with **no generated documentation schema** in the six scoped profiles. No shape test, return type, mutation rule or semantics is inferred from call sites.

## Out of scope

Native behavior remains unknown: defaults, membership, ordering, eligibility/lock/purchase policy, invalid-input behavior, event timing and persistence. A constant shape-compatible implementation can satisfy these contracts; passing never proves a native producer. No Wrath coverage, forced unselected consumers, runtime N/A returns, ignored tests, API replacements or vendor edits.
