# StorePublic eligibility observations

The [supplied eligibility contract](../../specs/store-public-eligibility.md) separates StorePublic eligibility from the existing secure Store catalog. Group membership is not evidence of purchase eligibility. The prior adapter reduced `GetProducts(groupID)` to nonempty-table truth and treated an unavailable secure getter as false; neither equation follows from the pinned public declarations.

## Environment-local observations

`SimState.store_public_eligibility` is a map of integer group IDs to explicitly supplied boolean observations. A host supplies or updates it through the environment's existing state handle. New environments start without observations. Missing keys remain distinct from supplied false and raise the named `missing-input` diagnostic. That failure is simulator input policy, not a native default claim.

The query lives under `src/c_api/store_public_eligibility.rs`. Its ACTIVE gate publishes on Retail, PTR, Forever, Mists, Era and Anniversary, not Wrath. It reads no secure product lists, entry flags, ownership fields or CatalogShop DTOs. Store enabled/visibility compatibility defaults remain separate in the workaround layer.

## Original consumer and test boundary

Each declaring profile selects `Blizzard_SharedXML/InterfaceUtil.lua` through its active SharedXML TOC. `StoreInterfaceUtil.OpenToSubscriptionProduct` first tests group 156 and only tests 37 when 156 is false. It selects subscription/game time and toggles the store for true observations. Both false lead to the original sound and URL index 22 branch. Shop2 must be disabled to reach these public queries; this prerequisite does not establish native shop policy.

`tests/store_public_eligibility.rs` loads the active original addon closure without overrides, checks TOC selection and helper source identity, and observes query lines with a debug hook. Clearly labeled test-only selection/toggle/sound/URL recording sinks observe outbound decisions. They do not replace the helper or query and do not prove a working storefront. In particular, classic inbound Lua lacks `SelectSubscriptionProduct`; that remains an end-to-end blocker, not a reason to invent a production endpoint.

Fixed secure lists contradict the supplied eligibility observations through subscription/game-time/web transitions; a second list arrangement preserves those outcomes. Separate direct assertions test arity and boolean shape, updates, missing-input recovery and two-environment isolation. Original helper conditions alone do not prove strict shape and shape assertions alone do not reject constant producers.

## Proof state

Retail RED at `6c5d12610` has an accepted slot-build receipt. Both exact selector lists contain one case. The selected original SharedXML helper has no setup diagnostics and fails because the old adapter chooses `gametime,toggle` for supplied `(true,false)` against fixed contradictory lists. The separate direct input test fails on the supplied-observation assertion. This is producer RED, not a missing storefront dependency.

The input/default-test-only RED sibling is `cf6c807c4`; its integration test bytes are unchanged from the executed RED. GREEN execution, other profiles, paired stock and all-target checks are pending. No native eligibility policy, coercion, secrecy parity, storefront or purchase completion is implied.

## Sources

- [Feature contract](../../specs/store-public-eligibility.md).
- [Existing secure Store investigation](../investigations/store-secure-pool-constructors.md) — distinct catalog/secure Store boundary.
- Profile-pinned `Blizzard_APIDocumentationGenerated/StorePublicUIDocumentation.lua`: Retail/PTR/Forever 11–24, Mists/Era/Anniversary 11–23. All declare Environment All at line 6; only Retail/PTR/Forever include AllowedWhenUntainted secret metadata at line 13.
- Profile-pinned `Blizzard_SharedXML/InterfaceUtil.lua`: Retail/PTR/Forever queries 44/48; Mists/Era/Anniversary 32/36.

## See Also

- [[store-secure-pool-constructors]] — secure catalog projection and downstream limitations.
- [[consumer-selection-and-red-proof]] — original selected consumer evidence and RED boundaries.
