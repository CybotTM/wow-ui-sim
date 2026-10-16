# Retail displayed currency indices — B5 proposal

Status: staged, uncompiled/unexecuted. Simulator policy proposal, not native parity.
Authority: 2026-10-16 B5 receipt; collapse/expand first. Watched and unused are independently selectable follow-ups, not part of base acceptance.

## What it must do

- [ ] Retail list size and list-info enumerate one contiguous, one-based visible projection of the existing catalog. Project catalog currencyListDepth for the original TokenUI header/entry factory. Header collapse hides all subsequent deeper rows until a same-or-shallower row. Headers remain visible; expansion restores catalog order and prior descendant expansion state.
- [ ] ExpandCurrencyList resolves its index against the current projection before mutation. Header identity is stable catalog position, not displayed position or currency ID (headers share ID zero). Expansion is environment-local, initially matching catalog flags, nonpersistent and idempotent.
- [ ] Out-of-range, nonpositive and non-header indices are no-ops. Invalid list-info indices return nil. Existing typed numeric/boolean extraction is retained; no new coercion/error policy is proposed.
- [ ] Mutations are synchronous; existing TokenHeaderMixin refresh and popup-close calls observe updated rows immediately. No new event ordering or currency-filter model.
- [ ] Currency quantities, ID reads, watched backpack enumeration and non-Retail behavior remain unchanged. Missing supplied rows retain nil list-info behavior; projection still counts their catalog positions as before.

## How it works

- [C API and Lua boundary](../lua-api.md)

## Implementation inventory

- `src/c_api/currency_list.rs` — Retail display projection/expansion state.
- `src/c_api/item_spell/c_currency.rs` — list readers and expansion mutator.
- `src/c_api/mod.rs`, `src/lua_api/state.rs`, `src/lua_api/state/sim_state.rs` — Retail state wiring.

## Tests asserting this spec

`tests/currency_displayed_indices.rs` loads original Retail cache through the existing addon harness, with no overrides. Real TokenHeaderTemplate instances call original ToggleCollapsed, original TokenFrame Update populates the scroll provider, and original popup CloseIfHidden clears the hidden selection. Fixture helpers do not replace Blizzard methods.

Cases: 19 baseline rows; first collapse ->14; shifted second-header collapse ->8; unrelated Miscellaneous/Valor rows survive; expand ->14 ->19 with exact snapshot restoration; visible provider indices agree; quantities/backpack preserved; invalid/non-header/repeated calls; fresh environment isolation.

Cached source: `~/.cache/wow-ui-sim/blizzard-ui/retail/AddOns/Blizzard_TokenUI/Blizzard_TokenUI.lua:23-31,363-404,459-475`; XML `:4-8` binds original header mixin/template. Cache content is mutable; handoff README records inspected hashes.

Predicted RED only: absent modeled ExpandCurrencyList may return through namespace default handling without changing rows, causing `B5: children must disappear` (19 versus14). A missing-method exception is an earlier registration failure, not the intended behavioral RED. Loader, template or provider setup failure is NOT accepted RED. No execution occurred; producer must not land until the owner records reached behavioral RED at this boundary.

## Known gaps (current cycle)

- [ ] Reached behavioral RED, compilation/GREEN, stock startup lua-errors and profile-preservation proof are not executed.
- [ ] Index-selected siblings without an existing modeled reader (for example GetCurrencyListLink) remain outside this producer; no blanket list API coverage.

## Out of scope

No new catalog, persistence, transfer, filters, vendor edits, pointer dispatch or rendering claims. See [existing field projection](currency-list-field-projection.md) for shared display/missing-row behavior. Optional watched/unused specs are separate files when selected; neither is required by these base tests.
