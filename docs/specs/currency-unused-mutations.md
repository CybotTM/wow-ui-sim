# Retail unused currency mutation (B5)

## What it must do

- `C_CurrencyInfo.SetCurrencyUnused(index, unused)` targets the current displayed currency-list index, not currency ID. The generated Retail `CurrencyInfoDocumentation.lua` declares `index: luaIndex`, `unused: bool`.
- Original `InactiveCurrencyCheckboxMixin:OnClick` must set/clear unused status synchronously, reselect the currency by name, and refresh the original TokenFrame data provider. List `isTypeUnused` and ID-based `CurrencyInfo.isTypeUnused` must agree. Quantities remain unchanged.
- List rows expose `discovered` for original `TokenFrameMixin:UpdatePopup` checkbox visibility.
- Preserve existing backpack membership/flags. Invalid, header, currency-ID-as-index and missing supplied-row targets are no-ops; repeated requests are idempotent. State remains environment-local.

The latter preservation/no-op policy is parent-directed simulator scope, not established current-Retail native parity.

## Evidence and unresolved facts

Cached Retail `Blizzard_TokenUI/Blizzard_TokenUI.lua:505-542` consumes `discovered`/`isTypeUnused`, invokes the mutator, rescans names and refreshes the frame. This proves the consumer contract, not native grouping policy.

Current Retail 12.0.x native grouping is UNKNOWN: laptop reported no current source available (2026-10-20). Historical 3.3.5a observations are not a Retail contract. INFERRED/held: unused header name/position, localized ordering, empty source categories, collapse/expansion interactions and persistence. No grouping producer until a separate decision adopts a model. The staged `30-unused.patch` is a hypothesis, not an oracle. No native source read in this worktree.

Original checkbox rescans by name, not currency ID; duplicate names may select the first visible match. This work does not replace that original consumer with an identity-based scan.

## Tests asserting this spec

`tests/currency_unused_mutations.rs` loads original cached Retail TokenUI, collapses the preceding original header, invokes the original inactive checkbox and observes selected ID, API status, unchanged backpack membership and original provider status in both directions. A real TokenEntryTemplate drives original UpdatePopup and checks discovered-checkbox visibility. Direct API controls cover invalid/header/ID targets, missing supplied rows, repeated requests and fresh-environment isolation. No copied/replaced consumer.

## Proof

Pending behavioral RED, producer, GREEN, touched-module before/after preservation, clean stock Retail startup, warning-free build and formatting.

## Out of scope

No vendor edits, shims, other-profile behavior changes, new currency catalog, transfer/filter changes or rilua edits. Native grouping remains unresolved rather than being claimed from the checkbox. See [displayed indices](currency-displayed-indices.md) and [watched mutations](currency-watched-mutations.md).
