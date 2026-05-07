//! Mixin-method surface pins for the `Blizzard_AccountStore` lane.
//!
//! Split into per-mixin submodules:
//!
//! - `account_store` — `AccountStoreMixin` (panel root)
//! - `base_card` — `AccountStoreBaseCardMixin` (card template parent of the
//!   four derived card mixins)
//! - `category` — `AccountStoreCategoryMixin` (per-row category button)
//! - `category_list` — `AccountStoreCategoryListMixin` (left-side category
//!   column wrapping the ScrollBox)
//! - `fullscreen_container` — `FullscreenAccountStoreContainerMixin`
//!   (toplevel container hosting AccountStoreFrame in WoW Labs /
//!   Plunderstorm fullscreen mode)
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

mod account_store;
mod base_card;
mod category;
mod category_list;
mod fullscreen_container;
mod item_display;
mod item_rack;
