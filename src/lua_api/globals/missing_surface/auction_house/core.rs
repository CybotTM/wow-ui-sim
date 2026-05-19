//! `C_AuctionHouse` probe surface backed by
//! `SimState.auction_browse_results` + `auction_replicate_items`.
//!
//! Migrates 3 entries off the namespace stub tables:
//!
//! - `C_AuctionHouse.GetAuctionItemSubClasses(classID)` — returns the
//!   subclass id array for an item class. Hard-coded to the standard
//!   retail ranges (Consumable=0..9, Weapon=0..20, Armor=0..11,
//!   etc.); unknown class ids return an empty array.
//! - `C_AuctionHouse.GetReplicateItemInfo(index)` — returns the
//!   7-tuple row from `auction_replicate_items` for a 0-based index
//!   (retail uses 0-based indexing here), or nothing out of range.
//! - `C_AuctionHouse.GetBrowseResults()` — returns an array of
//!   `BrowseResultInfo` tables (itemKey, minPrice, totalQuantity,
//!   containsOwnerItem, appearanceLink nilable).

use super::super::{ensure_namespace, set_table_array};
use crate::items;
use crate::lua_api::globals::state_backed_queries::dispatch_event_now;
use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, create_string, create_table, create_table_with_fields,
    table_set_static,
};
use crate::lua_api::state::{
    AuctionBrowseResult, AuctionItemClassFilter, AuctionRowInfo, AuctionSellQuote,
    AuctionSellQuoteKind, AuctionSortSpec, BidAuction, BrowseQuery, CommoditySearchResultInfo,
    CommoditySearchResults, ItemSearchKey, ItemSearchResultInfo, ItemSearchResults, OwnedAuction,
};
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::vm::{gc::arena::GcRef, table::Table};
use rilua::{LuaResult, Val};
use std::collections::HashSet;

#[path = "purchases.rs"]
mod purchases;

include!("registry_and_rows.rs");
include!("item_search.rs");
include!("commodity_search.rs");
include!("posting.rs");
