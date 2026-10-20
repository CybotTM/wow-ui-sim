# Retail unused currency mutation (B5)

Retail unused flags for the original TokenUI checkbox and popup. Producer: `src/c_api/item_spell/c_currency.rs`; [C API architecture](../wiki/systems/lua-api.md) owns subsystem context. This bounded flag contract does not adopt native unused-group ordering.

## What it must do

- [ ] `C_CurrencyInfo.SetCurrencyUnused(index, unused)` targets the current displayed list index, not currency ID, and synchronously updates the existing currency's unused flag. Generated Retail `CurrencyInfoDocumentation.lua:566-574` declares `index: luaIndex`, `unused: bool`.
- [ ] Original `InactiveCurrencyCheckboxMixin:OnClick` sets/clears status, rescans the selected name and refreshes the original TokenFrame provider. List `isTypeUnused` and ID-based `CurrencyInfo.isTypeUnused` agree.
- [ ] List `discovered` and `isTypeUnused` project supplied false/true values. Original `TokenFrameMixin:UpdatePopup` exposes the discovered currency checkbox and displays its checked unused state.
- [ ] Invalid/header/currency-ID-as-index and missing supplied-row targets are no-ops. Repeated requests are idempotent; fresh environments do not inherit unused state.
- [ ] Preserve active catalog order, collapse behavior, quantities and watched backpack membership/flags. No grouping or automatic watch mutation. Preservation/no-op policy is parent-directed simulator scope, not current-Retail native parity.

## How it works

- [C API and Lua boundary](../lua-api.md)
- [Existing query models](../wiki/systems/lua-api.md)

## Implementation inventory

- `src/c_api/item_spell/c_currency.rs` — Retail mutator and list status projection over the existing supplied CurrencyInfo row; no second membership store.
- `src/c_api/currency_list.rs` — existing displayed-index/expansion model, unchanged by this slice.

## Tests asserting this spec

- `tests/currency_unused_mutations.rs` — original checkbox/current-index/provider transitions, real TokenEntryTemplate -> original popup visibility/checked state, supplied boolean controls, invalid/missing targets, idempotence and environment isolation. No copied/replaced consumers.
- `tests/currency_displayed_indices.rs` — original header expansion/index/provider preservation.
- `tests/currency_watched_mutations.rs` — existing displayed-index and ID-based watched behavior.
- `tests/currency_info_probes.rs` — supplied display/missing-row preservation; Retail list field count12 includes the two new boolean fields.

### Bounded proof

Tests-only Rust scope `bd45dfd02` reached original-consumer RED: checkbox failed `B5 unused: original checkbox must mark selected currency unused`; real-entry original popup failed discovered-checkbox visibility. Repeated mutation also failed; no-op controls passed. No registration/loader/setup exception counted as RED. The same worktree-bound ELF passed displayed2/watched1/info16 BEFORE controls with zero compiler warnings.

Private handoff `b5-unused` records exact commands, source hashes and ELF binding. Supplemental supplied-state/checked-state controls `295fbd07a` precede producer `d27e79dd9`; they have no standalone RED receipt. GREEN and final gates remain pending.

## Known gaps (current cycle)

- [ ] GREEN, touched-module AFTER preservation, stock Retail lua-errors `[]`, warning-free build/check, formatting and persisted CVar invariance remain pending.

## Out of scope

Current Retail 12.0.x native grouping is UNKNOWN: laptop reported no current source available (2026-10-20). Historical 3.3.5a observations are not a Retail contract. INFERRED/held: header name/position, unused localized ordering, empty source categories, unused-header collapse/retained expansion and persistence. No grouping producer until a separate decision adopts a model. The staged `30-unused.patch` is a hypothesis, not an oracle; no native source read in this worktree.

Cached Retail `Blizzard_TokenUI.lua:505-542` consumes the two booleans, calls the setter, rescans by name and refreshes the provider. It does not establish native grouping. Duplicate names can select the first visible match; this work does not replace that consumer with an ID-based scan.

No vendor edits, shims, rilua edits, other-profile changes, new catalog, transfer/filter changes or event policy. Existing [displayed-index](currency-displayed-indices.md), [field-projection](currency-list-field-projection.md) and [watched](currency-watched-mutations.md) contracts otherwise remain unchanged.
