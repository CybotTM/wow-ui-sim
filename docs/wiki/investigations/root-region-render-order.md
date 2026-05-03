# Root Region Render Order

Root-level regions in the same draw layer must render in creation order, so newer regions draw on top of older regions.

## Content

`sort_root_regions()` handled regions whose parent is missing, invisible, in a different strata, or a strata root boundary such as `UIParent`. It previously used `Reverse(id)` as the final tie breaker, which made later-created root regions render before earlier regions. That inverted the normal "newer draws on top" rule for root-region buckets only.

The fix is to use ascending widget id as the tie breaker, matching the existing child-region ordering. `show_visible_region_repairs_parent_subtree_without_invalidating_buckets` now expects repaired visible regions to stay in creation order instead of jumping newly shown regions above older siblings. `tests/root_region_order.rs` covers the direct root-region case.

This does not change button-internal ordering: `NormalTexture` on `OVERLAY` still draws above `ARTWORK` regions inside the same action button. It only removes the root-region reverse ordering path.

## Sources

- [state_render.rs](../../../src/lua_api/state_render.rs) — root-region sorting and repair expectation
- [root_region_order.rs](../../../tests/root_region_order.rs) — regression test for root-region creation order

## See Also

- [[rendering-pipeline]] — strata and draw-layer sorting context
- [[transparent-wrapper-render-order]] — related investigation for grouped render ordering
