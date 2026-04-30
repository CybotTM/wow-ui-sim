//! Mixin-method surface pins for the `Blizzard_AccountStore` lane.
//!
//! Split into per-mixin submodules under `surface_mixins/`:
//!
//! - `account_store` — `AccountStoreMixin` (panel root)
//! - `base_card` — `AccountStoreBaseCardMixin` (card template parent of the
//!   four derived card mixins)
//! - `item_display` — `AccountStoreItemDisplayMixin` (StoreDisplay panel
//!   driving paging and per-store state)
//! - `item_rack` — `AccountStoreItemRackMixin` (per-category card pool laid
//!   out via grid)
//!
//! Each submodule pins both halves of its mixin contract: a positive test
//! that walks every method actually present on the source mixin (PLAN-named
//! plus PLAN-omitted-but-present), and a negative tripwire that asserts every
//! PLAN-named-but-absent method is reported `nil`. Spec/source mismatch
//! reasoning lives in the per-submodule docstring.

#[path = "surface_mixins/account_store.rs"]
mod account_store;

#[path = "surface_mixins/base_card.rs"]
mod base_card;

#[path = "surface_mixins/item_display.rs"]
mod item_display;

#[path = "surface_mixins/item_rack.rs"]
mod item_rack;
