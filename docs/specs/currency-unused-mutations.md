# Retail unused currency mutation (B5)

## What it must do

- `C_CurrencyInfo.SetCurrencyUnused(index, unused)` targets the current displayed currency-list index, not currency ID. The generated Retail `CurrencyInfoDocumentation.lua` declares `index: luaIndex`, `unused: bool`.
- Original `InactiveCurrencyCheckboxMixin:OnClick` must set/clear unused status synchronously, reselect the currency by name, and refresh the original TokenFrame data provider. List `isTypeUnused` and ID-based `CurrencyInfo.isTypeUnused` must agree. Quantities remain unchanged.
- List rows expose `discovered` for original `TokenFrameMixin:UpdatePopup` checkbox visibility.

## Evidence and unresolved facts

Cached Retail `Blizzard_TokenUI/Blizzard_TokenUI.lua:505-542` consumes `discovered`/`isTypeUnused`, invokes the mutator, rescans names and refreshes the frame. This proves the consumer contract, not native grouping policy.

INFERRED pending laptop native evidence: unused header name/position, row ordering, empty source categories, watched coupling, collapse/expansion interactions, persistence. The staged `30-unused.patch` is a hypothesis, not an oracle. No native source read in this worktree.

## Tests asserting this spec

`tests/currency_unused_mutations.rs` loads original cached Retail TokenUI, collapses the preceding original header, invokes the original inactive checkbox and observes selected ID, API status and original provider status in both directions. No copied/replaced consumer.

## Proof

Pending behavioral RED, producer, GREEN, touched-module before/after preservation, clean stock Retail startup, warning-free build and formatting.

## Out of scope

No vendor edits, shims, other-profile behavior changes, new currency catalog, transfer/filter changes or rilua edits. Native grouping remains unresolved rather than being claimed from the checkbox. See [displayed indices](currency-displayed-indices.md) and [watched mutations](currency-watched-mutations.md).
