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

use super::{ensure_namespace, set_table_array};
use crate::items;
use crate::lua_api::globals::state_backed_queries::dispatch_event_now;
use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, create_string, create_table, create_table_with_fields,
    table_set,
};
use crate::lua_api::state::{
    AuctionBrowseResult, AuctionItemClassFilter, AuctionRowInfo, AuctionSellQuote,
    AuctionSellQuoteKind, AuctionSortSpec, BidAuction, BrowseQuery, CommodityPurchaseQuote,
    CommoditySearchResultInfo, CommoditySearchResults, ItemSearchKey, ItemSearchResultInfo,
    ItemSearchResults, OwnedAuction,
};
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::vm::{gc::arena::GcRef, table::Table};
use rilua::{LuaResult, Val};
use std::collections::HashSet;

type AuctionHouseMethod = fn(&mut LuaState) -> LuaResult<u32>;
const AUCTION_HOUSE_METHODS: &[(&'static str, AuctionHouseMethod)] = &[
    (
        "GetAuctionItemSubClasses",
        c_auction_house_get_auction_item_sub_classes,
    ),
    (
        "GetReplicateItemInfo",
        c_auction_house_get_replicate_item_info,
    ),
    ("GetBrowseResults", c_auction_house_get_browse_results),
    ("HasFavorites", c_auction_house_has_favorites),
    (
        "HasFullBrowseResults",
        c_auction_house_has_full_browse_results,
    ),
    (
        "RequestMoreBrowseResults",
        c_auction_house_request_more_browse_results,
    ),
    ("GetItemKeyInfo", c_auction_house_get_item_key_info),
    (
        "GetItemKeyRequiredLevel",
        c_auction_house_get_item_key_required_level,
    ),
    ("GetExtraBrowseInfo", c_auction_house_get_extra_browse_info),
    (
        "SupportsCopperValues",
        c_auction_house_supports_copper_values,
    ),
    (
        "FavoritesAreAvailable",
        c_auction_house_favorites_are_available,
    ),
    ("HasMaxFavorites", c_auction_house_has_max_favorites),
    ("IsFavoriteItem", c_auction_house_is_favorite_item),
    ("SetFavoriteItem", c_auction_house_set_favorite_item),
    ("GetFilterGroups", c_auction_house_get_filter_groups),
    ("CloseAuctionHouse", c_auction_house_close_auction_house),
    ("QueryBids", c_auction_house_query_bids),
    ("GetNumBids", c_auction_house_get_num_bids),
    ("GetBidInfo", c_auction_house_get_bid_info),
    ("HasFullBidResults", c_auction_house_has_full_bid_results),
    ("GetNumBidTypes", c_auction_house_get_num_bid_types),
    ("GetBidType", c_auction_house_get_bid_type),
    ("GetMaxBidItemBid", c_auction_house_get_max_bid_item_bid),
    (
        "GetMaxBidItemBuyout",
        c_auction_house_get_max_bid_item_buyout,
    ),
    ("QueryOwnedAuctions", c_auction_house_query_owned_auctions),
    (
        "GetNumOwnedAuctions",
        c_auction_house_get_num_owned_auctions,
    ),
    (
        "GetOwnedAuctionInfo",
        c_auction_house_get_owned_auction_info,
    ),
    (
        "HasFullOwnedAuctionResults",
        c_auction_house_has_full_owned_auction_results,
    ),
    (
        "GetNumOwnedAuctionTypes",
        c_auction_house_get_num_owned_auction_types,
    ),
    (
        "GetOwnedAuctionType",
        c_auction_house_get_owned_auction_type,
    ),
    (
        "GetMaxOwnedAuctionBid",
        c_auction_house_get_max_owned_auction_bid,
    ),
    (
        "GetMaxOwnedAuctionBuyout",
        c_auction_house_get_max_owned_auction_buyout,
    ),
    ("MakeItemKey", c_auction_house_make_item_key),
    ("GetItemKeyFromItem", c_auction_house_get_item_key_from_item),
    (
        "GetTimeLeftBandInfo",
        c_auction_house_get_time_left_band_info,
    ),
    (
        "IsThrottledMessageSystemReady",
        c_auction_house_is_throttled_message_system_ready,
    ),
    (
        "ShouldAutoPopulatePrice",
        c_auction_house_should_auto_populate_price,
    ),
    ("IsSellItemValid", c_auction_house_is_sell_item_valid),
    ("GetCancelCost", c_auction_house_get_cancel_cost),
    (
        "GetAvailablePostCount",
        c_auction_house_get_available_post_count,
    ),
    (
        "GetItemCommodityStatus",
        c_auction_house_get_item_commodity_status,
    ),
    (
        "GetQuoteDurationRemaining",
        c_auction_house_get_quote_duration_remaining,
    ),
    (
        "GetNumItemSearchResults",
        c_auction_house_get_num_item_search_results,
    ),
    (
        "GetItemSearchResultInfo",
        c_auction_house_get_item_search_result_info,
    ),
    (
        "GetItemSearchResultsQuantity",
        c_auction_house_get_item_search_results_quantity,
    ),
    (
        "HasFullItemSearchResults",
        c_auction_house_has_full_item_search_results,
    ),
    (
        "GetMaxItemSearchResultBid",
        c_auction_house_get_max_item_search_result_bid,
    ),
    (
        "GetMaxItemSearchResultBuyout",
        c_auction_house_get_max_item_search_result_buyout,
    ),
    (
        "RefreshItemSearchResults",
        c_auction_house_refresh_item_search_results,
    ),
    (
        "RequestMoreItemSearchResults",
        c_auction_house_request_more_item_search_results,
    ),
    ("HasSearchResults", c_auction_house_has_search_results),
    (
        "GetNumCommoditySearchResults",
        c_auction_house_get_num_commodity_search_results,
    ),
    (
        "GetCommoditySearchResultInfo",
        c_auction_house_get_commodity_search_result_info,
    ),
    (
        "GetCommoditySearchResultsQuantity",
        c_auction_house_get_commodity_search_results_quantity,
    ),
    (
        "HasFullCommoditySearchResults",
        c_auction_house_has_full_commodity_search_results,
    ),
    (
        "GetMaxCommoditySearchResultPrice",
        c_auction_house_get_max_commodity_search_result_price,
    ),
    (
        "RefreshCommoditySearchResults",
        c_auction_house_refresh_commodity_search_results,
    ),
    (
        "RequestMoreCommoditySearchResults",
        c_auction_house_request_more_commodity_search_results,
    ),
    ("SendBrowseQuery", c_auction_house_send_browse_query),
    ("SendSearchQuery", c_auction_house_send_search_query),
    (
        "SendSellSearchQuery",
        c_auction_house_send_sell_search_query,
    ),
    ("SearchForFavorites", c_auction_house_search_for_favorites),
    (
        "CalculateItemDeposit",
        c_auction_house_calculate_item_deposit,
    ),
    (
        "CalculateCommodityDeposit",
        c_auction_house_calculate_commodity_deposit,
    ),
    ("PostItem", c_auction_house_post_item),
    ("PostCommodity", c_auction_house_post_commodity),
    ("ConfirmPostItem", c_auction_house_confirm_post_item),
    (
        "ConfirmPostCommodity",
        c_auction_house_confirm_post_commodity,
    ),
    ("CancelSell", c_auction_house_cancel_sell),
    ("CancelAuction", c_auction_house_cancel_auction),
    ("PlaceBid", c_auction_house_place_bid),
    ("GetAuctionInfoByID", c_auction_house_get_auction_info_by_id),
    (
        "RequestOwnedAuctionBidderInfo",
        c_auction_house_request_owned_auction_bidder_info,
    ),
    (
        "StartCommoditiesPurchase",
        c_auction_house_start_commodities_purchase,
    ),
    (
        "ConfirmCommoditiesPurchase",
        c_auction_house_confirm_commodities_purchase,
    ),
    (
        "CancelCommoditiesPurchase",
        c_auction_house_cancel_commodities_purchase,
    ),
];

const ITEM_BONDING_BIND_ON_PICKUP: u8 = 1;
const ITEM_BONDING_QUEST: u8 = 4;

const SECONDS_30_MINUTES: i64 = 30 * 60;
const SECONDS_2_HOURS: i64 = 2 * 60 * 60;
const SECONDS_12_HOURS: i64 = 12 * 60 * 60;
const SECONDS_48_HOURS: i64 = 48 * 60 * 60;

const ITEM_COMMODITY_STATUS_UNKNOWN: i32 = 0;
const ITEM_COMMODITY_STATUS_ITEM: i32 = 1;
const ITEM_COMMODITY_STATUS_COMMODITY: i32 = 2;

pub(super) fn register_auction_house_surface(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_AuctionHouse")?;
    register_auction_house_methods(state, table_ref)
}

fn register_auction_house_methods(state: &mut LuaState, table_ref: GcRef<Table>) -> LuaResult<()> {
    for &(name, func) in AUCTION_HOUSE_METHODS {
        table_set_rust_fn_static(state, table_ref, name, func)?;
    }
    Ok(())
}

fn c_auction_house_get_auction_item_sub_classes(state: &mut LuaState) -> LuaResult<u32> {
    let class_id = i32::from_stack(state, 1)?;
    let subclass_count = standard_subclass_count(class_id);
    let array = create_table(state);
    for i in 0..subclass_count {
        set_table_array(state, array, i as i64 + 1, Val::Num(i as f64));
    }
    state.push(array);
    Ok(1)
}

/// Count of subclasses retail exposes per item class (from
/// `Enum.ItemClass`). Values are defensive — unknown class ids return
/// 0 and produce an empty array.
fn standard_subclass_count(class_id: i32) -> i32 {
    match class_id {
        0 => 12,  // Consumable
        1 => 8,   // Container
        2 => 21,  // Weapon
        3 => 11,  // Gem
        4 => 12,  // Armor
        5 => 5,   // Reagent
        6 => 6,   // Projectile
        7 => 21,  // Tradegoods
        9 => 11,  // Recipe
        12 => 1,  // Quest
        13 => 1,  // Key
        15 => 5,  // Miscellaneous
        16 => 10, // Glyph
        17 => 8,  // Battle Pet
        19 => 1,  // Wow Token
        _ => 0,
    }
}

fn c_auction_house_get_replicate_item_info(state: &mut LuaState) -> LuaResult<u32> {
    let index = i32::from_stack(state, 1)?;
    let entry = {
        let sim = borrow_state(state)?;
        // Retail uses 0-based indexing here (unlike most Blizzard APIs).
        usize::try_from(index)
            .ok()
            .and_then(|idx| sim.auction_replicate_items.get(idx).cloned())
    };
    let Some(item) = entry else {
        return Ok(0);
    };

    let name = create_string(state, &item.name);
    let level_type = create_string(state, &item.level_type);
    state.push(name);
    state.push(Val::Num(item.texture as f64));
    state.push(Val::Num(item.count as f64));
    state.push(Val::Num(item.quality_id as f64));
    state.push(Val::Bool(item.usable));
    state.push(Val::Num(item.level as f64));
    state.push(level_type);
    Ok(7)
}

fn c_auction_house_get_browse_results(state: &mut LuaState) -> LuaResult<u32> {
    let results = borrow_state(state)?.auction_browse_results.clone();
    let array = create_table(state);
    for (index, row) in results.into_iter().enumerate() {
        let entry = push_browse_result_table(state, &row);
        set_table_array(state, array, index as i64 + 1, entry);
    }
    state.push(array);
    Ok(1)
}

fn c_auction_house_has_favorites(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn c_auction_house_has_full_browse_results(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    Ok(1)
}

fn c_auction_house_request_more_browse_results(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    Ok(1)
}

fn c_auction_house_get_item_key_info(state: &mut LuaState) -> LuaResult<u32> {
    let item_key = Val::from_stack(state, 1)?;
    let Some(item) = item_from_item_key(state, item_key) else {
        state.push(Val::Nil);
        return Ok(1);
    };

    let info = push_item_key_info_table(state, item);
    state.push(info);
    Ok(1)
}

fn c_auction_house_get_item_key_required_level(state: &mut LuaState) -> LuaResult<u32> {
    let item_key = Val::from_stack(state, 1)?;
    let level = extract_item_key_id(state, item_key)
        .and_then(items::get_item)
        .map(|item| item.required_level as f64)
        .unwrap_or(0.0);
    state.push(Val::Num(level));
    Ok(1)
}

fn c_auction_house_get_extra_browse_info(state: &mut LuaState) -> LuaResult<u32> {
    let item_key = Val::from_stack(state, 1)?;
    let quality = extract_item_key_id(state, item_key)
        .and_then(items::get_item)
        .map(|item| item.quality as f64)
        .unwrap_or(0.0);
    state.push(Val::Nil);
    state.push(Val::Num(quality));
    Ok(2)
}

fn c_auction_house_supports_copper_values(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    Ok(1)
}

fn c_auction_house_favorites_are_available(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    Ok(1)
}

fn c_auction_house_has_max_favorites(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn c_auction_house_is_favorite_item(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn c_auction_house_set_favorite_item(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn c_auction_house_get_filter_groups(state: &mut LuaState) -> LuaResult<u32> {
    let groups = create_table(state);
    state.push(groups);
    Ok(1)
}

fn c_auction_house_close_auction_house(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn c_auction_house_query_bids(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn c_auction_house_get_num_bids(state: &mut LuaState) -> LuaResult<u32> {
    let count = borrow_state(state)?.auction_bids.len() as f64;
    state.push(Val::Num(count));
    Ok(1)
}

fn c_auction_house_get_bid_info(state: &mut LuaState) -> LuaResult<u32> {
    let index = i32::from_stack(state, 1)?;
    if index < 1 {
        return Ok(0);
    }
    let entry = borrow_state(state)?
        .auction_bids
        .get((index - 1) as usize)
        .cloned();
    let Some(entry) = entry else { return Ok(0) };
    let bid = push_bid_auction_table(state, &entry);
    state.push(bid);
    Ok(1)
}

fn c_auction_house_has_full_bid_results(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    Ok(1)
}

fn c_auction_house_get_num_bid_types(state: &mut LuaState) -> LuaResult<u32> {
    let count = distinct_bid_item_keys(state)?.len() as f64;
    state.push(Val::Num(count));
    Ok(1)
}

fn c_auction_house_get_bid_type(state: &mut LuaState) -> LuaResult<u32> {
    let index = i32::from_stack(state, 1)?;
    let item_keys = distinct_bid_item_keys(state)?;
    push_distinct_item_key_at_index(state, &item_keys, index)
}

fn c_auction_house_query_owned_auctions(_state: &mut LuaState) -> LuaResult<u32> {
    // Real WoW would fire OWNED_AUCTIONS_UPDATED; the sim has no
    // back-end query, so it's a no-op — `auction_owned` is whatever
    // `A_Admin.AddOwnedAuction` (or seeded defaults) put there.
    Ok(0)
}

fn c_auction_house_get_num_owned_auctions(state: &mut LuaState) -> LuaResult<u32> {
    let count = borrow_state(state)?.auction_owned.len() as f64;
    state.push(Val::Num(count));
    Ok(1)
}

fn c_auction_house_get_owned_auction_info(state: &mut LuaState) -> LuaResult<u32> {
    let index = i32::from_stack(state, 1)?;
    if index < 1 {
        return Ok(0);
    }
    let entry = borrow_state(state)?
        .auction_owned
        .get((index - 1) as usize)
        .cloned();
    let Some(entry) = entry else { return Ok(0) };
    let owned = push_owned_auction_table(state, &entry);
    state.push(owned);
    Ok(1)
}

fn c_auction_house_has_full_owned_auction_results(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    Ok(1)
}

fn c_auction_house_get_num_owned_auction_types(state: &mut LuaState) -> LuaResult<u32> {
    let count = distinct_owned_item_keys(state)?.len() as f64;
    state.push(Val::Num(count));
    Ok(1)
}

fn c_auction_house_get_owned_auction_type(state: &mut LuaState) -> LuaResult<u32> {
    let index = i32::from_stack(state, 1)?;
    let item_keys = distinct_owned_item_keys(state)?;
    push_distinct_item_key_at_index(state, &item_keys, index)
}

fn c_auction_house_get_max_bid_item_bid(state: &mut LuaState) -> LuaResult<u32> {
    push_max_price_for(
        state,
        |entry: &BidAuction| entry.bid_amount,
        |sim| &sim.auction_bids,
    )
}

fn c_auction_house_get_max_bid_item_buyout(state: &mut LuaState) -> LuaResult<u32> {
    push_max_price_for(
        state,
        |entry: &BidAuction| entry.buyout_amount,
        |sim| &sim.auction_bids,
    )
}

fn c_auction_house_get_max_owned_auction_bid(state: &mut LuaState) -> LuaResult<u32> {
    push_max_price_for(
        state,
        |entry: &OwnedAuction| entry.bid_amount,
        |sim| &sim.auction_owned,
    )
}

fn c_auction_house_get_max_owned_auction_buyout(state: &mut LuaState) -> LuaResult<u32> {
    push_max_price_for(
        state,
        |entry: &OwnedAuction| entry.buyout_amount,
        |sim| &sim.auction_owned,
    )
}

fn push_owned_auction_table(state: &mut LuaState, entry: &OwnedAuction) -> Val {
    let t = push_auction_row_table(state, entry);
    table_set(state, t, "status", Val::Num(entry.status as f64));
    t
}

fn push_bid_auction_table(state: &mut LuaState, entry: &BidAuction) -> Val {
    let t = push_auction_row_table(state, entry);
    match &entry.bidder {
        Some(bidder) => {
            let bidder_val = create_string(state, bidder);
            table_set(state, t, "bidder", bidder_val);
        }
        None => table_set(state, t, "bidder", Val::Nil),
    }
    t
}

fn push_item_key_table(state: &mut LuaState, item_id: i32, item_level: i32) -> Val {
    let item_key = create_table(state);
    table_set(state, item_key, "itemID", Val::Num(item_id as f64));
    table_set(state, item_key, "itemLevel", Val::Num(item_level as f64));
    table_set(state, item_key, "itemSuffix", Val::Num(0.0));
    table_set(state, item_key, "battlePetSpeciesID", Val::Num(0.0));
    item_key
}

fn push_item_key_info_table(state: &mut LuaState, item: &items::ItemInfo) -> Val {
    let info = create_table(state);
    let item_name = create_string(state, item.name);
    table_set(state, info, "itemName", item_name);
    table_set(state, info, "iconFileID", Val::Num(item_icon_file_id(item)));
    table_set(state, info, "quality", Val::Num(item.quality as f64));
    table_set(
        state,
        info,
        "isCommodity",
        Val::Bool(item_is_commodity(item)),
    );
    table_set(state, info, "battlePetLink", Val::Nil);
    table_set(state, info, "appearanceLink", Val::Nil);
    info
}

fn item_from_item_key(state: &mut LuaState, item_key: Val) -> Option<&'static items::ItemInfo> {
    extract_item_key_id(state, item_key).and_then(items::get_item)
}

fn item_icon_file_id(item: &items::ItemInfo) -> f64 {
    match item.icon_file_data_id {
        0 => 134400.0,
        icon_file_data_id => icon_file_data_id as f64,
    }
}

fn item_is_commodity(item: &items::ItemInfo) -> bool {
    item.stackable > 1 && item.inventory_type == 0
}

trait AuctionRow {
    fn item_id(&self) -> i32;
    fn item_level(&self) -> i32;
    fn auction_id(&self) -> i32;
    fn quantity(&self) -> i32;
    fn bid_amount(&self) -> i64;
    fn buyout_amount(&self) -> i64;
    fn time_left(&self) -> i32;
    fn time_left_seconds(&self) -> i64;
}

impl AuctionRow for OwnedAuction {
    fn item_id(&self) -> i32 {
        self.item_id
    }

    fn item_level(&self) -> i32 {
        self.item_level
    }

    fn auction_id(&self) -> i32 {
        self.auction_id
    }

    fn quantity(&self) -> i32 {
        self.quantity
    }

    fn bid_amount(&self) -> i64 {
        self.bid_amount
    }

    fn buyout_amount(&self) -> i64 {
        self.buyout_amount
    }

    fn time_left(&self) -> i32 {
        self.time_left
    }

    fn time_left_seconds(&self) -> i64 {
        self.time_left_seconds
    }
}

impl AuctionRow for BidAuction {
    fn item_id(&self) -> i32 {
        self.item_id
    }

    fn item_level(&self) -> i32 {
        self.item_level
    }

    fn auction_id(&self) -> i32 {
        self.auction_id
    }

    fn quantity(&self) -> i32 {
        self.quantity
    }

    fn bid_amount(&self) -> i64 {
        self.bid_amount
    }

    fn buyout_amount(&self) -> i64 {
        self.buyout_amount
    }

    fn time_left(&self) -> i32 {
        self.time_left
    }

    fn time_left_seconds(&self) -> i64 {
        self.time_left_seconds
    }
}

fn push_auction_row_table<T: AuctionRow>(state: &mut LuaState, entry: &T) -> Val {
    let t = create_table(state);
    let item_key = push_item_key_table(state, entry.item_id(), entry.item_level());
    set_common_auction_row_fields(
        state,
        t,
        item_key,
        entry.auction_id(),
        entry.quantity(),
        entry.bid_amount(),
        entry.buyout_amount(),
        entry.time_left(),
        entry.time_left_seconds(),
    );
    t
}

fn set_common_auction_row_fields(
    state: &mut LuaState,
    row_table: Val,
    item_key: Val,
    auction_id: i32,
    quantity: i32,
    bid_amount: i64,
    buyout_amount: i64,
    time_left: i32,
    time_left_seconds: i64,
) {
    table_set(state, row_table, "itemKey", item_key);
    table_set(state, row_table, "auctionID", Val::Num(auction_id as f64));
    table_set(state, row_table, "quantity", Val::Num(quantity as f64));
    table_set(state, row_table, "bidAmount", Val::Num(bid_amount as f64));
    table_set(
        state,
        row_table,
        "buyoutAmount",
        Val::Num(buyout_amount as f64),
    );
    table_set(state, row_table, "timeLeft", Val::Num(time_left as f64));
    table_set(
        state,
        row_table,
        "timeLeftSeconds",
        Val::Num(time_left_seconds as f64),
    );
}

fn push_max_money_value(state: &mut LuaState, amount: i64) {
    state.push(Val::Num(amount as f64));
}

fn push_distinct_item_key_at_index(
    state: &mut LuaState,
    item_keys: &[(i32, i32)],
    index: i32,
) -> LuaResult<u32> {
    if index < 1 {
        return Ok(0);
    }
    let Some((item_id, item_level)) = item_keys.get((index - 1) as usize).copied() else {
        return Ok(0);
    };
    let item_key = push_item_key_table(state, item_id, item_level);
    state.push(item_key);
    Ok(1)
}

fn push_max_price_for<T>(
    state: &mut LuaState,
    amount_for: impl Fn(&T) -> i64,
    rows_for: impl Fn(&crate::lua_api::state::SimState) -> &[T],
) -> LuaResult<u32> {
    let amount = {
        let sim = borrow_state(state)?;
        rows_for(&sim).iter().map(amount_for).max().unwrap_or(0)
    };
    push_max_money_value(state, amount);
    Ok(1)
}

fn distinct_owned_item_keys(state: &mut LuaState) -> LuaResult<Vec<(i32, i32)>> {
    let rows = borrow_state(state)?.auction_owned.clone();
    Ok(distinct_item_keys(
        rows.into_iter()
            .map(|entry| (entry.item_id, entry.item_level)),
    ))
}

fn distinct_bid_item_keys(state: &mut LuaState) -> LuaResult<Vec<(i32, i32)>> {
    let rows = borrow_state(state)?.auction_bids.clone();
    Ok(distinct_item_keys(
        rows.into_iter()
            .map(|entry| (entry.item_id, entry.item_level)),
    ))
}

fn distinct_item_keys(rows: impl IntoIterator<Item = (i32, i32)>) -> Vec<(i32, i32)> {
    let mut distinct = Vec::new();
    let mut seen = HashSet::new();
    for item_key in rows {
        if seen.insert(item_key) {
            distinct.push(item_key);
        }
    }
    distinct
}

fn extract_item_key_id(state: &mut LuaState, value: Val) -> Option<u32> {
    let table_ref = match value {
        Val::Table(table_ref) => table_ref,
        _ => return None,
    };
    let item_id_key = state.gc.intern_string_static(b"itemID");
    match state
        .gc
        .tables
        .get(table_ref)
        .map(|table| table.get_str(item_id_key, &state.gc.string_arena))
    {
        Some(Val::Num(item_id)) if item_id > 0.0 => Some(item_id as u32),
        _ => None,
    }
}

fn c_auction_house_make_item_key(state: &mut LuaState) -> LuaResult<u32> {
    let item_id = i32::from_stack(state, 1)?;
    let item_level = optional_int_arg(state, 2)?;
    let item_suffix = optional_int_arg(state, 3)?;
    let battle_pet_species_id = optional_int_arg(state, 4)?;
    let item_key = create_table(state);
    table_set(state, item_key, "itemID", Val::Num(item_id as f64));
    table_set(state, item_key, "itemLevel", Val::Num(item_level as f64));
    table_set(state, item_key, "itemSuffix", Val::Num(item_suffix as f64));
    table_set(
        state,
        item_key,
        "battlePetSpeciesID",
        Val::Num(battle_pet_species_id as f64),
    );
    state.push(item_key);
    Ok(1)
}

fn c_auction_house_get_item_key_from_item(state: &mut LuaState) -> LuaResult<u32> {
    let location = Val::from_stack(state, 1)?;
    let Some(item_id) = extract_item_id_from_location(state, location) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let item_level = items::get_item(item_id)
        .map(|item| item.item_level as i32)
        .unwrap_or(0);
    let item_key = push_item_key_table(state, item_id as i32, item_level);
    state.push(item_key);
    Ok(1)
}

fn c_auction_house_get_time_left_band_info(state: &mut LuaState) -> LuaResult<u32> {
    let band = i32::from_stack(state, 1)?;
    let Some((min_seconds, max_seconds)) = time_left_band_range(band) else {
        return Ok(0);
    };
    state.push(Val::Num(min_seconds as f64));
    state.push(Val::Num(max_seconds as f64));
    Ok(2)
}

fn c_auction_house_is_throttled_message_system_ready(state: &mut LuaState) -> LuaResult<u32> {
    let ready = borrow_state(state)?.auction_throttle_ready;
    state.push(Val::Bool(ready));
    Ok(1)
}

fn c_auction_house_should_auto_populate_price(state: &mut LuaState) -> LuaResult<u32> {
    let auto_populate = borrow_state(state)?.auction_should_auto_populate_price;
    state.push(Val::Bool(auto_populate));
    Ok(1)
}

fn c_auction_house_is_sell_item_valid(state: &mut LuaState) -> LuaResult<u32> {
    let location = Val::from_stack(state, 1)?;
    let valid = extract_item_id_from_location(state, location)
        .and_then(items::get_item)
        .is_some_and(item_is_sellable);
    state.push(Val::Bool(valid));
    Ok(1)
}

fn c_auction_house_get_cancel_cost(state: &mut LuaState) -> LuaResult<u32> {
    let auction_id = i32::from_stack(state, 1)?;
    let buyout = borrow_state(state)?
        .auction_owned
        .iter()
        .find(|row| row.auction_id == auction_id)
        .map(|row| row.buyout_amount)
        .unwrap_or(0);
    let cost = cancel_cost_for_buyout(buyout);
    state.push(Val::Num(cost as f64));
    Ok(1)
}

fn c_auction_house_get_available_post_count(state: &mut LuaState) -> LuaResult<u32> {
    let location = Val::from_stack(state, 1)?;
    let count = available_post_count_for_location(state, location)?;
    state.push(Val::Num(count as f64));
    Ok(1)
}

fn c_auction_house_get_item_commodity_status(state: &mut LuaState) -> LuaResult<u32> {
    let location = Val::from_stack(state, 1)?;
    let status = match extract_item_id_from_location(state, location).and_then(items::get_item) {
        Some(item) if item_is_commodity(item) => ITEM_COMMODITY_STATUS_COMMODITY,
        Some(_) => ITEM_COMMODITY_STATUS_ITEM,
        None => ITEM_COMMODITY_STATUS_UNKNOWN,
    };
    state.push(Val::Num(status as f64));
    Ok(1)
}

fn c_auction_house_get_quote_duration_remaining(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

/// Read an optional positive integer from the Lua stack, defaulting to
/// 0 when the slot is nil or non-numeric. Matches the retail
/// `MakeItemKey` contract where `itemLevel`/`itemSuffix` are optional.
fn optional_int_arg(state: &mut LuaState, slot: i32) -> LuaResult<i32> {
    match Val::from_stack(state, slot)? {
        Val::Num(n) => Ok(n as i32),
        _ => Ok(0),
    }
}

/// Resolve an `ItemLocation`-shaped Lua table to an `itemID`. Retail
/// sell flows pass `{ bagID, slotIndex }`; direct `{ itemID = X }`
/// remains as a test-friendly shortcut for API-level probes.
fn extract_item_id_from_location(state: &mut LuaState, value: Val) -> Option<u32> {
    if let Some(item_id) = extract_item_key_id(state, value) {
        return Some(item_id);
    }
    let (bag, slot) = extract_bag_slot_location(state, value)?;
    borrow_state(state)
        .ok()?
        .get_bag_item(bag, slot)
        .map(|item| item.0)
}

fn extract_bag_slot_location(state: &mut LuaState, value: Val) -> Option<(i32, i32)> {
    let table_ref = match value {
        Val::Table(table_ref) => table_ref,
        _ => return None,
    };
    let bag_key = state.gc.intern_string_static(b"bagID");
    let slot_key = state.gc.intern_string_static(b"slotIndex");
    let table = state.gc.tables.get(table_ref)?;
    let bag = read_optional_int_field(table, bag_key, &state.gc.string_arena)?;
    let slot = read_optional_int_field(table, slot_key, &state.gc.string_arena)?;
    Some((bag, slot))
}

/// `(min, max)` seconds for each `Enum.AuctionHouseTimeLeftBand`
/// member (0=Short..3=VeryLong). Mirrors the real client's bands per
/// `Blizzard_AuctionHouseUtil.lua` time-left labels: Short=under 30
/// min, Medium=30 min..2 h, Long=2..12 h, VeryLong=12..48 h.
fn time_left_band_range(band: i32) -> Option<(i64, i64)> {
    match band {
        0 => Some((0, SECONDS_30_MINUTES)),
        1 => Some((SECONDS_30_MINUTES, SECONDS_2_HOURS)),
        2 => Some((SECONDS_2_HOURS, SECONDS_12_HOURS)),
        3 => Some((SECONDS_12_HOURS, SECONDS_48_HOURS)),
        _ => None,
    }
}

/// `IsSellItemValid` blocks soulbound and quest items. The sim has no
/// "locked" cursor state yet, so the bonding flag is the only gate.
fn item_is_sellable(item: &items::ItemInfo) -> bool {
    item.bonding != ITEM_BONDING_BIND_ON_PICKUP && item.bonding != ITEM_BONDING_QUEST
}

/// Cancellation deposit forfeit: 5% of buyout, rounded down. Matches
/// the live client's `AUCTION_HOUSE_CANCEL_FEE_PERCENT` constant.
fn cancel_cost_for_buyout(buyout: i64) -> i64 {
    (buyout * 5) / 100
}

fn available_post_count_for_location(state: &mut LuaState, location: Val) -> LuaResult<i32> {
    let Some(item_id) = extract_item_id_from_location(state, location) else {
        return Ok(0);
    };
    let max_stack = items::get_item(item_id)
        .map(|item| item.stackable.max(1) as i32)
        .unwrap_or(1);
    let already_listed: i32 = borrow_state(state)?
        .auction_owned
        .iter()
        .filter(|row| row.item_id as u32 == item_id)
        .map(|row| row.quantity)
        .sum();
    Ok((max_stack - already_listed).max(0))
}

fn push_browse_result_table(state: &mut LuaState, row: &AuctionBrowseResult) -> Val {
    let t = create_table(state);
    let item_key = create_table(state);
    table_set(state, item_key, "itemID", Val::Num(row.item_id as f64));
    table_set(
        state,
        item_key,
        "itemLevel",
        Val::Num(row.item_level as f64),
    );
    table_set(state, item_key, "itemSuffix", Val::Num(0.0));
    table_set(state, item_key, "battlePetSpeciesID", Val::Num(0.0));

    table_set(state, t, "itemKey", item_key);
    table_set(state, t, "minPrice", Val::Num(row.min_price as f64));
    table_set(
        state,
        t,
        "totalQuantity",
        Val::Num(row.total_quantity as f64),
    );
    table_set(
        state,
        t,
        "containsOwnerItem",
        Val::Bool(row.contains_owner_item),
    );
    table_set(state, t, "appearanceLink", Val::Nil);
    t
}

const ITEM_SEARCH_RESULTS_UPDATED: &str = "ITEM_SEARCH_RESULTS_UPDATED";

fn c_auction_house_get_num_item_search_results(state: &mut LuaState) -> LuaResult<u32> {
    let arg = Val::from_stack(state, 1)?;
    let count = with_item_search_bucket(state, arg, |bucket| bucket.entries.len())?.unwrap_or(0);
    state.push(Val::Num(count as f64));
    Ok(1)
}

fn c_auction_house_get_item_search_result_info(state: &mut LuaState) -> LuaResult<u32> {
    let arg = Val::from_stack(state, 1)?;
    let index = i32::from_stack(state, 2)?;
    if index < 1 {
        return Ok(0);
    }
    let Some(key) = extract_item_search_key(state, arg) else {
        return Ok(0);
    };
    let entry = borrow_state(state)?
        .auction_item_searches
        .get(&key)
        .and_then(|bucket| bucket.entries.get((index - 1) as usize).cloned());
    let Some(entry) = entry else { return Ok(0) };
    let table = push_item_search_result_info_table(state, &entry);
    state.push(table);
    Ok(1)
}

fn c_auction_house_get_item_search_results_quantity(state: &mut LuaState) -> LuaResult<u32> {
    let arg = Val::from_stack(state, 1)?;
    let total = with_item_search_bucket(state, arg, sum_quantity)?.unwrap_or(0);
    state.push(Val::Num(total as f64));
    Ok(1)
}

fn c_auction_house_has_full_item_search_results(state: &mut LuaState) -> LuaResult<u32> {
    let arg = Val::from_stack(state, 1)?;
    // Default true when no bucket exists — addons treat "no results yet" as
    // already-full so they don't pointlessly call `RequestMoreItemSearchResults`.
    let full =
        with_item_search_bucket(state, arg, |bucket| bucket.has_full_results)?.unwrap_or(true);
    state.push(Val::Bool(full));
    Ok(1)
}

fn c_auction_house_get_max_item_search_result_bid(state: &mut LuaState) -> LuaResult<u32> {
    let arg = Val::from_stack(state, 1)?;
    let max = with_item_search_bucket(state, arg, max_bid_amount)?.unwrap_or(0);
    push_max_money_value(state, max);
    Ok(1)
}

fn c_auction_house_get_max_item_search_result_buyout(state: &mut LuaState) -> LuaResult<u32> {
    let arg = Val::from_stack(state, 1)?;
    let max = with_item_search_bucket(state, arg, max_buyout_amount)?.unwrap_or(0);
    push_max_money_value(state, max);
    Ok(1)
}

fn c_auction_house_refresh_item_search_results(state: &mut LuaState) -> LuaResult<u32> {
    let arg = Val::from_stack(state, 1)?;
    let Some(key) = extract_item_search_key(state, arg) else {
        return Ok(0);
    };
    let item_key = push_item_key_table_from_search_key(state, key);
    dispatch_event_now(state, ITEM_SEARCH_RESULTS_UPDATED, &[item_key])?;
    Ok(0)
}

fn c_auction_house_request_more_item_search_results(state: &mut LuaState) -> LuaResult<u32> {
    let arg = Val::from_stack(state, 1)?;
    let Some(key) = extract_item_search_key(state, arg) else {
        state.push(Val::Bool(false));
        return Ok(1);
    };
    let already_full = borrow_state(state)?
        .auction_item_searches
        .get(&key)
        .is_none_or(|bucket| bucket.has_full_results);
    let item_key = push_item_key_table_from_search_key(state, key);
    dispatch_event_now(state, ITEM_SEARCH_RESULTS_UPDATED, &[item_key])?;
    state.push(Val::Bool(!already_full));
    Ok(1)
}

fn c_auction_house_has_search_results(state: &mut LuaState) -> LuaResult<u32> {
    let arg = Val::from_stack(state, 1)?;
    let has =
        with_item_search_bucket(state, arg, |bucket| !bucket.entries.is_empty())?.unwrap_or(false);
    state.push(Val::Bool(has));
    Ok(1)
}

fn sum_quantity(bucket: &ItemSearchResults) -> i64 {
    bucket.entries.iter().map(|e| e.quantity as i64).sum()
}

fn max_bid_amount(bucket: &ItemSearchResults) -> i64 {
    bucket
        .entries
        .iter()
        .map(|e| e.bid_amount)
        .max()
        .unwrap_or(0)
}

fn max_buyout_amount(bucket: &ItemSearchResults) -> i64 {
    bucket
        .entries
        .iter()
        .map(|e| e.buyout_amount)
        .max()
        .unwrap_or(0)
}

/// Run `f` against the per-key search bucket, returning `None` when the
/// argument is not a valid item key or no bucket has been seeded.
fn with_item_search_bucket<R>(
    state: &mut LuaState,
    arg: Val,
    f: impl FnOnce(&ItemSearchResults) -> R,
) -> LuaResult<Option<R>> {
    let Some(key) = extract_item_search_key(state, arg) else {
        return Ok(None);
    };
    Ok(borrow_state(state)?.auction_item_searches.get(&key).map(f))
}

fn push_item_key_table_from_search_key(state: &mut LuaState, key: ItemSearchKey) -> Val {
    let (item_id, item_level, item_suffix, species) = key;
    create_table_with_fields(
        state,
        &[
            ("itemID", Val::Num(item_id as f64)),
            ("itemLevel", Val::Num(item_level as f64)),
            ("itemSuffix", Val::Num(item_suffix as f64)),
            ("battlePetSpeciesID", Val::Num(species as f64)),
        ],
    )
}

fn push_item_search_result_info_table(state: &mut LuaState, entry: &ItemSearchResultInfo) -> Val {
    let owners = build_owners_array(state, &entry.owners);
    let item_link = create_string(state, &entry.item_link);
    let bidder = match &entry.bidder {
        Some(name) => create_string(state, name),
        None => Val::Nil,
    };
    create_table_with_fields(
        state,
        &[
            ("owners", owners),
            ("timeLeft", Val::Num(entry.time_left as f64)),
            ("auctionID", Val::Num(entry.auction_id as f64)),
            ("quantity", Val::Num(entry.quantity as f64)),
            ("itemLink", item_link),
            ("containsOwnerItem", Val::Bool(entry.contains_owner_item)),
            (
                "containsAccountItem",
                Val::Bool(entry.contains_account_item),
            ),
            (
                "containsSocketedItem",
                Val::Bool(entry.contains_socketed_item),
            ),
            ("bidder", bidder),
            ("minBid", Val::Num(entry.min_bid as f64)),
            ("bidAmount", Val::Num(entry.bid_amount as f64)),
            ("buyoutAmount", Val::Num(entry.buyout_amount as f64)),
            ("timeLeftSeconds", Val::Num(entry.time_left_seconds as f64)),
        ],
    )
}

fn build_owners_array(state: &mut LuaState, owners: &[String]) -> Val {
    let array = create_table(state);
    for (index, owner) in owners.iter().enumerate() {
        let owner_val = create_string(state, owner);
        set_table_array(state, array, index as i64 + 1, owner_val);
    }
    array
}

/// Read the canonical 4-tuple `(itemID, itemLevel, itemSuffix,
/// battlePetSpeciesID)` from a Lua `ItemKey` table. Returns `None` when
/// the value is not a table or `itemID` is missing/zero.
fn extract_item_search_key(state: &mut LuaState, value: Val) -> Option<ItemSearchKey> {
    let Val::Table(table_ref) = value else {
        return None;
    };
    let item_id_key = state.gc.intern_string_static(b"itemID");
    let item_level_key = state.gc.intern_string_static(b"itemLevel");
    let item_suffix_key = state.gc.intern_string_static(b"itemSuffix");
    let species_key = state.gc.intern_string_static(b"battlePetSpeciesID");
    let table = state.gc.tables.get(table_ref)?;
    let arena = &state.gc.string_arena;
    let item_id = match table.get_str(item_id_key, arena) {
        Val::Num(n) if n > 0.0 => n as i32,
        _ => return None,
    };
    Some((
        item_id,
        read_int_field(table, item_level_key, arena),
        read_int_field(table, item_suffix_key, arena),
        read_int_field(table, species_key, arena),
    ))
}

fn read_int_field(
    table: &Table,
    key: GcRef<rilua::vm::string::LuaString>,
    arena: &rilua::vm::gc::arena::Arena<rilua::vm::string::LuaString>,
) -> i32 {
    match table.get_str(key, arena) {
        Val::Num(n) => n as i32,
        _ => 0,
    }
}

const COMMODITY_SEARCH_RESULTS_UPDATED: &str = "COMMODITY_SEARCH_RESULTS_UPDATED";

fn c_auction_house_get_num_commodity_search_results(state: &mut LuaState) -> LuaResult<u32> {
    let item_id = i32::from_stack(state, 1)?;
    let count =
        with_commodity_search_bucket(state, item_id, |bucket| bucket.entries.len()).unwrap_or(0);
    state.push(Val::Num(count as f64));
    Ok(1)
}

fn c_auction_house_get_commodity_search_result_info(state: &mut LuaState) -> LuaResult<u32> {
    let item_id = i32::from_stack(state, 1)?;
    let index = i32::from_stack(state, 2)?;
    if index < 1 {
        return Ok(0);
    }
    let entry = borrow_state(state)?
        .auction_commodity_searches
        .get(&item_id)
        .and_then(|bucket| bucket.entries.get((index - 1) as usize).cloned());
    let Some(entry) = entry else { return Ok(0) };
    let table = push_commodity_search_result_info_table(state, &entry);
    state.push(table);
    Ok(1)
}

fn c_auction_house_get_commodity_search_results_quantity(state: &mut LuaState) -> LuaResult<u32> {
    let item_id = i32::from_stack(state, 1)?;
    let total = with_commodity_search_bucket(state, item_id, sum_commodity_quantity).unwrap_or(0);
    state.push(Val::Num(total as f64));
    Ok(1)
}

fn c_auction_house_has_full_commodity_search_results(state: &mut LuaState) -> LuaResult<u32> {
    let item_id = i32::from_stack(state, 1)?;
    // Default true when no bucket exists — addons treat "no results yet" as
    // already-full so they don't pointlessly call `RequestMoreCommoditySearchResults`.
    let full = with_commodity_search_bucket(state, item_id, |bucket| bucket.has_full_results)
        .unwrap_or(true);
    state.push(Val::Bool(full));
    Ok(1)
}

fn c_auction_house_get_max_commodity_search_result_price(state: &mut LuaState) -> LuaResult<u32> {
    let item_id = i32::from_stack(state, 1)?;
    let max = with_commodity_search_bucket(state, item_id, max_unit_price).unwrap_or(0);
    push_max_money_value(state, max);
    Ok(1)
}

fn c_auction_house_refresh_commodity_search_results(state: &mut LuaState) -> LuaResult<u32> {
    let item_id = i32::from_stack(state, 1)?;
    dispatch_event_now(
        state,
        COMMODITY_SEARCH_RESULTS_UPDATED,
        &[Val::Num(item_id as f64)],
    )?;
    Ok(0)
}

fn c_auction_house_request_more_commodity_search_results(state: &mut LuaState) -> LuaResult<u32> {
    let item_id = i32::from_stack(state, 1)?;
    let already_full = borrow_state(state)?
        .auction_commodity_searches
        .get(&item_id)
        .is_none_or(|bucket| bucket.has_full_results);
    state.push(Val::Bool(!already_full));
    Ok(1)
}

fn sum_commodity_quantity(bucket: &CommoditySearchResults) -> i64 {
    bucket.entries.iter().map(|e| e.quantity as i64).sum()
}

fn max_unit_price(bucket: &CommoditySearchResults) -> i64 {
    bucket
        .entries
        .iter()
        .map(|e| e.unit_price)
        .max()
        .unwrap_or(0)
}

/// Run `f` against the per-itemID commodity bucket, returning `None`
/// when no bucket has been seeded.
fn with_commodity_search_bucket<R>(
    state: &mut LuaState,
    item_id: i32,
    f: impl FnOnce(&CommoditySearchResults) -> R,
) -> Option<R> {
    let sim = borrow_state(state).ok()?;
    sim.auction_commodity_searches.get(&item_id).map(f)
}

fn push_commodity_search_result_info_table(
    state: &mut LuaState,
    entry: &CommoditySearchResultInfo,
) -> Val {
    let owners = build_owners_array(state, &entry.owners);
    create_table_with_fields(
        state,
        &[
            ("itemID", Val::Num(entry.item_id as f64)),
            ("quantity", Val::Num(entry.quantity as f64)),
            ("unitPrice", Val::Num(entry.unit_price as f64)),
            ("auctionID", Val::Num(entry.auction_id as f64)),
            ("owners", owners),
            ("timeLeftSeconds", Val::Num(entry.time_left_seconds as f64)),
            ("numOwnerItems", Val::Num(entry.num_owner_items as f64)),
            ("containsOwnerItem", Val::Bool(entry.contains_owner_item)),
            (
                "containsAccountItem",
                Val::Bool(entry.contains_account_item),
            ),
        ],
    )
}

const AUCTION_HOUSE_BROWSE_RESULTS_UPDATED: &str = "AUCTION_HOUSE_BROWSE_RESULTS_UPDATED";
const AUCTION_HOUSE_BROWSE_FAILURE: &str = "AUCTION_HOUSE_BROWSE_FAILURE";
const AUCTION_HOUSE_THROTTLED_MESSAGE_QUEUED: &str = "AUCTION_HOUSE_THROTTLED_MESSAGE_QUEUED";

/// Distinguishes the buyer-tab cache (`auction_item_searches`) from the
/// seller-tab cache (`auction_sell_search_results`) when dispatching a
/// search-query event.
#[derive(Copy, Clone)]
enum SearchSide {
    Buyer,
    Seller,
}

fn c_auction_house_send_browse_query(state: &mut LuaState) -> LuaResult<u32> {
    let query_arg = Val::from_stack(state, 1)?;
    let query = parse_browse_query(state, query_arg);
    if dispatch_throttled_browse_query(state, query.clone())? {
        return Ok(0);
    }
    borrow_state_mut(state)?.auction_last_browse_query = Some(query);
    dispatch_event_now(state, AUCTION_HOUSE_BROWSE_RESULTS_UPDATED, &[])?;
    Ok(0)
}

fn c_auction_house_send_search_query(state: &mut LuaState) -> LuaResult<u32> {
    let item_key_arg = Val::from_stack(state, 1)?;
    dispatch_search_query_event(state, item_key_arg, SearchSide::Buyer)
}

fn c_auction_house_send_sell_search_query(state: &mut LuaState) -> LuaResult<u32> {
    let item_key_arg = Val::from_stack(state, 1)?;
    dispatch_search_query_event(state, item_key_arg, SearchSide::Seller)
}

fn c_auction_house_search_for_favorites(state: &mut LuaState) -> LuaResult<u32> {
    if dispatch_throttle_if_pending(state)? {
        return Ok(0);
    }
    dispatch_event_now(state, AUCTION_HOUSE_BROWSE_RESULTS_UPDATED, &[])?;
    Ok(0)
}

/// Common dispatch for `SendSearchQuery` / `SendSellSearchQuery`. Fires
/// `AUCTION_HOUSE_THROTTLED_MESSAGE_QUEUED` while the throttle gate is
/// closed, `COMMODITY_SEARCH_RESULTS_UPDATED` for seeded buyer-side
/// commodity buckets, `ITEM_SEARCH_RESULTS_UPDATED` when the
/// corresponding item bucket is already seeded, and
/// `AUCTION_HOUSE_BROWSE_FAILURE` otherwise.
fn dispatch_search_query_event(
    state: &mut LuaState,
    item_key_arg: Val,
    side: SearchSide,
) -> LuaResult<u32> {
    if dispatch_throttle_if_pending(state)? {
        return Ok(0);
    }
    let Some(key) = extract_item_search_key(state, item_key_arg) else {
        dispatch_event_now(state, AUCTION_HOUSE_BROWSE_FAILURE, &[])?;
        return Ok(0);
    };
    if has_buyer_commodity_bucket(state, side, key.0)? {
        dispatch_event_now(
            state,
            COMMODITY_SEARCH_RESULTS_UPDATED,
            &[Val::Num(key.0 as f64)],
        )?;
        return Ok(0);
    }
    let has_bucket = bucket_exists(state, side, &key)?;
    if has_bucket {
        let item_key = push_item_key_table_from_search_key(state, key);
        dispatch_event_now(state, ITEM_SEARCH_RESULTS_UPDATED, &[item_key])?;
    } else {
        dispatch_event_now(state, AUCTION_HOUSE_BROWSE_FAILURE, &[])?;
    }
    Ok(0)
}

fn has_buyer_commodity_bucket(
    state: &mut LuaState,
    side: SearchSide,
    item_id: i32,
) -> LuaResult<bool> {
    if !matches!(side, SearchSide::Buyer) {
        return Ok(false);
    }
    Ok(borrow_state(state)?
        .auction_commodity_searches
        .contains_key(&item_id))
}

fn bucket_exists(state: &mut LuaState, side: SearchSide, key: &ItemSearchKey) -> LuaResult<bool> {
    let sim = borrow_state(state)?;
    Ok(match side {
        SearchSide::Buyer => sim.auction_item_searches.contains_key(key),
        SearchSide::Seller => sim.auction_sell_search_results.contains_key(key),
    })
}

/// Returns true when the throttle gate is closed and the queued event
/// has been dispatched. Callers should bail out without firing the
/// success event in that case.
fn dispatch_throttle_if_pending(state: &mut LuaState) -> LuaResult<bool> {
    if borrow_state(state)?.auction_throttle_ready {
        return Ok(false);
    }
    dispatch_event_now(state, AUCTION_HOUSE_THROTTLED_MESSAGE_QUEUED, &[])?;
    Ok(true)
}

fn dispatch_throttled_browse_query(state: &mut LuaState, query: BrowseQuery) -> LuaResult<bool> {
    if borrow_state(state)?.auction_throttle_ready {
        return Ok(false);
    }
    borrow_state_mut(state)?.auction_queued_browse_query = Some(query);
    dispatch_event_now(state, AUCTION_HOUSE_THROTTLED_MESSAGE_QUEUED, &[])?;
    Ok(true)
}

/// Pre-interned `BrowseQuery` field keys passed through the parser so
/// helpers can read row tables without re-interning per-row.
struct BrowseQueryKeys {
    search_string: GcRef<rilua::vm::string::LuaString>,
    sorts: GcRef<rilua::vm::string::LuaString>,
    min_level: GcRef<rilua::vm::string::LuaString>,
    max_level: GcRef<rilua::vm::string::LuaString>,
    filters: GcRef<rilua::vm::string::LuaString>,
    item_class_filters: GcRef<rilua::vm::string::LuaString>,
    sort_order: GcRef<rilua::vm::string::LuaString>,
    reverse_sort: GcRef<rilua::vm::string::LuaString>,
    class_id: GcRef<rilua::vm::string::LuaString>,
    sub_class_id: GcRef<rilua::vm::string::LuaString>,
    inventory_type: GcRef<rilua::vm::string::LuaString>,
}

fn intern_browse_query_keys(state: &mut LuaState) -> BrowseQueryKeys {
    BrowseQueryKeys {
        search_string: state.gc.intern_string_static(b"searchString"),
        sorts: state.gc.intern_string_static(b"sorts"),
        min_level: state.gc.intern_string_static(b"minLevel"),
        max_level: state.gc.intern_string_static(b"maxLevel"),
        filters: state.gc.intern_string_static(b"filters"),
        item_class_filters: state.gc.intern_string_static(b"itemClassFilters"),
        sort_order: state.gc.intern_string_static(b"sortOrder"),
        reverse_sort: state.gc.intern_string_static(b"reverseSort"),
        class_id: state.gc.intern_string_static(b"classID"),
        sub_class_id: state.gc.intern_string_static(b"subClassID"),
        inventory_type: state.gc.intern_string_static(b"inventoryType"),
    }
}

fn parse_browse_query(state: &mut LuaState, arg: Val) -> BrowseQuery {
    let Val::Table(table_ref) = arg else {
        return BrowseQuery::default();
    };
    let keys = intern_browse_query_keys(state);
    let arena = &state.gc.string_arena;
    let Some(table) = state.gc.tables.get(table_ref) else {
        return BrowseQuery::default();
    };
    let search_string = read_string_field(table, keys.search_string, arena);
    let min_level = read_optional_int_field(table, keys.min_level, arena);
    let max_level = read_optional_int_field(table, keys.max_level, arena);
    let sorts_ref = nested_table_ref(table, keys.sorts, arena);
    let filters_ref = nested_table_ref(table, keys.filters, arena);
    let class_filters_ref = nested_table_ref(table, keys.item_class_filters, arena);
    BrowseQuery {
        search_string,
        sorts: sorts_ref
            .map(|r| read_sort_array(state, r, &keys))
            .unwrap_or_default(),
        min_level,
        max_level,
        filters: filters_ref
            .map(|r| read_int_array(state, r))
            .unwrap_or_default(),
        item_class_filters: class_filters_ref
            .map(|r| read_item_class_filter_array(state, r, &keys))
            .unwrap_or_default(),
    }
}

fn read_string_field(
    table: &Table,
    key: GcRef<rilua::vm::string::LuaString>,
    arena: &rilua::vm::gc::arena::Arena<rilua::vm::string::LuaString>,
) -> String {
    match table.get_str(key, arena) {
        Val::Str(s) => arena
            .get(s)
            .and_then(|ls| ls.as_str())
            .map(|s| s.to_string())
            .unwrap_or_default(),
        _ => String::new(),
    }
}

fn read_optional_int_field(
    table: &Table,
    key: GcRef<rilua::vm::string::LuaString>,
    arena: &rilua::vm::gc::arena::Arena<rilua::vm::string::LuaString>,
) -> Option<i32> {
    match table.get_str(key, arena) {
        Val::Num(n) => Some(n as i32),
        _ => None,
    }
}

fn collect_table_array_row_refs(table: &Table) -> Vec<GcRef<Table>> {
    table
        .array_slice()
        .iter()
        .filter_map(|val| match val {
            Val::Table(row_ref) => Some(*row_ref),
            _ => None,
        })
        .collect()
}

fn nested_table_ref(
    table: &Table,
    key: GcRef<rilua::vm::string::LuaString>,
    arena: &rilua::vm::gc::arena::Arena<rilua::vm::string::LuaString>,
) -> Option<GcRef<Table>> {
    match table.get_str(key, arena) {
        Val::Table(r) => Some(r),
        _ => None,
    }
}

fn read_sort_array(
    state: &LuaState,
    table_ref: GcRef<Table>,
    keys: &BrowseQueryKeys,
) -> Vec<AuctionSortSpec> {
    let arena = &state.gc.string_arena;
    let Some(table) = state.gc.tables.get(table_ref) else {
        return Vec::new();
    };
    let row_refs = collect_table_array_row_refs(table);
    row_refs
        .into_iter()
        .filter_map(|row_ref| state.gc.tables.get(row_ref))
        .map(|row| AuctionSortSpec {
            sort_order: read_int_field(row, keys.sort_order, arena),
            reverse_sort: matches!(row.get_str(keys.reverse_sort, arena), Val::Bool(true)),
        })
        .collect()
}

fn read_int_array(state: &LuaState, table_ref: GcRef<Table>) -> Vec<i32> {
    let Some(table) = state.gc.tables.get(table_ref) else {
        return Vec::new();
    };
    table
        .array_slice()
        .iter()
        .filter_map(|val| match val {
            Val::Num(n) => Some(*n as i32),
            _ => None,
        })
        .collect()
}

fn read_item_class_filter_array(
    state: &LuaState,
    table_ref: GcRef<Table>,
    keys: &BrowseQueryKeys,
) -> Vec<AuctionItemClassFilter> {
    let arena = &state.gc.string_arena;
    let Some(table) = state.gc.tables.get(table_ref) else {
        return Vec::new();
    };
    let row_refs = collect_table_array_row_refs(table);
    row_refs
        .into_iter()
        .filter_map(|row_ref| state.gc.tables.get(row_ref))
        .map(|row| AuctionItemClassFilter {
            class_id: read_int_field(row, keys.class_id, arena),
            sub_class_id: read_optional_int_field(row, keys.sub_class_id, arena),
            inventory_type: read_optional_int_field(row, keys.inventory_type, arena),
        })
        .collect()
}

const AUCTION_HOUSE_AUCTION_CREATED: &str = "AUCTION_HOUSE_AUCTION_CREATED";
const AUCTION_CANCELED: &str = "AUCTION_CANCELED";
const OWNED_AUCTIONS_UPDATED: &str = "OWNED_AUCTIONS_UPDATED";

/// Vendor-sell percentage paid as deposit. Multiplied by quantity and
/// duration band to mirror the live client's deposit formula
/// (`AuctionHouseUtil.lua` builds it the same way).
const DEPOSIT_PERCENT: i64 = 15;
const SECONDS_24_HOURS: i64 = 24 * 60 * 60;

/// `Enum.AuctionStatus` values used by `OwnedAuction.status`. The sim
/// flips a canceled row to `Sold` so the Auctions tab repaints with the
/// "no longer active" treatment.
const AUCTION_STATUS_ACTIVE: i32 = 0;
const AUCTION_STATUS_SOLD: i32 = 1;

/// `Enum.AuctionHouseTimeLeftBand.VeryLong`. Fresh posts always start
/// here — 12h/24h/48h durations all sit inside the 12..48h window the
/// VeryLong band spans.
const TIME_LEFT_BAND_VERY_LONG: i32 = 4;

/// 15% of vendor-sell × quantity × duration band, rounded down to copper.
/// Matches the live client's posted-deposit formula closely enough for
/// the sell-frame to display consistent numbers.
fn deposit_for(item_id: u32, duration: i32, quantity: i32) -> i64 {
    let Some(item) = items::get_item(item_id) else {
        return 0;
    };
    let sell_price = item.sell_price as i64;
    let qty = quantity.max(0) as i64;
    let dur = duration.clamp(1, 3) as i64;
    (sell_price * qty * DEPOSIT_PERCENT * dur) / 100
}

/// Maps `Enum.AuctionHouseDuration` (1..3) to seconds. Out-of-range
/// values fall back to the longest duration so a tester passing 0
/// still gets a usable row.
fn time_left_seconds_for_duration(duration: i32) -> i64 {
    match duration {
        1 => SECONDS_12_HOURS,
        2 => SECONDS_24_HOURS,
        _ => SECONDS_48_HOURS,
    }
}

fn next_auction_id(state: &mut LuaState) -> LuaResult<i32> {
    let next = borrow_state(state)?
        .auction_owned
        .iter()
        .map(|row| row.auction_id)
        .max()
        .unwrap_or(0)
        + 1;
    Ok(next)
}

fn deduct_player_money(state: &mut LuaState, amount: i64) -> LuaResult<()> {
    borrow_state_mut(state)?.player.money -= amount;
    Ok(())
}

fn refund_player_money(state: &mut LuaState, amount: i64) -> LuaResult<()> {
    borrow_state_mut(state)?.player.money += amount;
    Ok(())
}

fn dispatch_owned_auctions_updated(state: &mut LuaState) -> LuaResult<()> {
    dispatch_event_now(state, OWNED_AUCTIONS_UPDATED, &[])
}

fn dispatch_auction_created(state: &mut LuaState, auction_id: i32) -> LuaResult<()> {
    dispatch_event_now(
        state,
        AUCTION_HOUSE_AUCTION_CREATED,
        &[Val::Num(auction_id as f64)],
    )
}

fn dispatch_auction_canceled(state: &mut LuaState, auction_id: i32) -> LuaResult<()> {
    dispatch_event_now(state, AUCTION_CANCELED, &[Val::Num(auction_id as f64)])
}

/// Read a nilable copper amount. `bid?`/`buyout?` slots default to 0
/// when the addon omits them (matches the live client treating
/// `nil`/`0` as "no bid set").
fn read_optional_money_arg(state: &mut LuaState, slot: i32) -> LuaResult<i64> {
    Ok(match Val::from_stack(state, slot)? {
        Val::Num(n) => n as i64,
        _ => 0,
    })
}

/// Captures the per-listing inputs `PostItem`/`PostCommodity` collect
/// from the Lua stack so the shared finalize path stays under the
/// param-overload threshold.
struct PostListingContext {
    item_id: u32,
    duration: i32,
    quantity: i32,
    bid_amount: i64,
    buyout_amount: i64,
    deposit: i64,
}

/// Build + append a fresh `OwnedAuction`, deduct the deposit, fire the
/// pair of post-listing events, and clear the in-flight quote. Shared
/// between `PostItem`/`PostCommodity` and their `Confirm*` siblings.
fn finalize_owned_auction_post(state: &mut LuaState, ctx: &PostListingContext) -> LuaResult<i32> {
    let auction_id = next_auction_id(state)?;
    let row = build_owned_auction_row(auction_id, ctx);
    let owner_name = borrow_state(state)?.player.name.clone();
    {
        let mut sim = borrow_state_mut(state)?;
        sim.auction_owned.push(row);
        sim.auction_index.insert(
            auction_id as i64,
            AuctionRowInfo {
                owner: owner_name,
                bid_amount: ctx.bid_amount,
                buyout_amount: ctx.buyout_amount,
                deposit: ctx.deposit,
                consortium_cut: 0,
            },
        );
        sim.auction_sell_quote = None;
    }
    deduct_player_money(state, ctx.deposit)?;
    dispatch_auction_created(state, auction_id)?;
    dispatch_owned_auctions_updated(state)?;
    Ok(auction_id)
}

fn build_owned_auction_row(auction_id: i32, ctx: &PostListingContext) -> OwnedAuction {
    let item_level = items::get_item(ctx.item_id)
        .map(|item| item.item_level as i32)
        .unwrap_or(0);
    OwnedAuction {
        auction_id,
        item_id: ctx.item_id as i32,
        item_level,
        quantity: ctx.quantity,
        bid_amount: ctx.bid_amount,
        buyout_amount: ctx.buyout_amount,
        status: AUCTION_STATUS_ACTIVE,
        time_left: TIME_LEFT_BAND_VERY_LONG,
        time_left_seconds: time_left_seconds_for_duration(ctx.duration),
    }
}

fn capture_sell_quote(state: &mut LuaState, quote: AuctionSellQuote) -> LuaResult<()> {
    borrow_state_mut(state)?.auction_sell_quote = Some(quote);
    Ok(())
}

/// Buyout for the owned-auction row: items post a flat buyout, commodities
/// post `unit_price * quantity`. Mirrors retail's `PostCommodity` math.
fn buyout_from_quote(quote: &AuctionSellQuote) -> i64 {
    match quote.kind {
        AuctionSellQuoteKind::Item => quote.unit_price,
        AuctionSellQuoteKind::Commodity => quote.unit_price * quote.quantity as i64,
    }
}

/// Capture the in-flight quote, then finalize the listing using the same
/// quote fields. Shared between `PostItem` and `PostCommodity` so each
/// entry point only does Lua-stack parsing.
fn post_listing(state: &mut LuaState, quote: AuctionSellQuote, bid: i64) -> LuaResult<()> {
    let ctx = PostListingContext {
        item_id: quote.item_id as u32,
        duration: quote.duration,
        quantity: quote.quantity,
        bid_amount: bid,
        buyout_amount: buyout_from_quote(&quote),
        deposit: quote.deposit,
    };
    capture_sell_quote(state, quote)?;
    finalize_owned_auction_post(state, &ctx)?;
    Ok(())
}

/// Flip an `Active` row to `Sold` and return its `(bid, buyout)` so the
/// caller can compute the cancel refund. `None` when the auction id is
/// unknown or already inactive — matches the live client treating
/// `CancelAuction` on a stale id as a silent no-op.
fn mark_owned_auction_canceled(
    state: &mut LuaState,
    auction_id: i32,
) -> LuaResult<Option<(i64, i64)>> {
    let mut sim = borrow_state_mut(state)?;
    let Some(row) = sim
        .auction_owned
        .iter_mut()
        .find(|r| r.auction_id == auction_id)
    else {
        return Ok(None);
    };
    if row.status != AUCTION_STATUS_ACTIVE {
        return Ok(None);
    }
    row.status = AUCTION_STATUS_SOLD;
    Ok(Some((row.bid_amount, row.buyout_amount)))
}

fn c_auction_house_calculate_item_deposit(state: &mut LuaState) -> LuaResult<u32> {
    let location = Val::from_stack(state, 1)?;
    let duration = i32::from_stack(state, 2)?;
    let quantity = i32::from_stack(state, 3)?;
    let Some(item_id) = extract_item_id_from_location(state, location) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let deposit = deposit_for(item_id, duration, quantity);
    state.push(Val::Num(deposit as f64));
    Ok(1)
}

fn c_auction_house_calculate_commodity_deposit(state: &mut LuaState) -> LuaResult<u32> {
    let item_id = i32::from_stack(state, 1)?;
    let duration = i32::from_stack(state, 2)?;
    let quantity = i32::from_stack(state, 3)?;
    let deposit = deposit_for(item_id as u32, duration, quantity);
    state.push(Val::Num(deposit as f64));
    Ok(1)
}

fn c_auction_house_post_item(state: &mut LuaState) -> LuaResult<u32> {
    let location = Val::from_stack(state, 1)?;
    let duration = i32::from_stack(state, 2)?;
    let quantity = i32::from_stack(state, 3)?;
    let bid = read_optional_money_arg(state, 4)?;
    let buyout = read_optional_money_arg(state, 5)?;
    let Some(item_id) = extract_item_id_from_location(state, location) else {
        state.push(Val::Bool(false));
        return Ok(1);
    };
    let quote = AuctionSellQuote {
        kind: AuctionSellQuoteKind::Item,
        item_id: item_id as i32,
        duration,
        quantity,
        unit_price: buyout,
        deposit: deposit_for(item_id, duration, quantity),
    };
    post_listing(state, quote, bid)?;
    state.push(Val::Bool(true));
    Ok(1)
}

fn c_auction_house_post_commodity(state: &mut LuaState) -> LuaResult<u32> {
    let location = Val::from_stack(state, 1)?;
    let duration = i32::from_stack(state, 2)?;
    let quantity = i32::from_stack(state, 3)?;
    let unit_price = read_optional_money_arg(state, 4)?;
    let Some(item_id) = extract_item_id_from_location(state, location) else {
        state.push(Val::Bool(false));
        return Ok(1);
    };
    let quote = AuctionSellQuote {
        kind: AuctionSellQuoteKind::Commodity,
        item_id: item_id as i32,
        duration,
        quantity,
        unit_price,
        deposit: deposit_for(item_id, duration, quantity),
    };
    post_listing(state, quote, 0)?;
    state.push(Val::Bool(true));
    Ok(1)
}

fn c_auction_house_confirm_post_item(state: &mut LuaState) -> LuaResult<u32> {
    c_auction_house_post_item(state)
}

fn c_auction_house_confirm_post_commodity(state: &mut LuaState) -> LuaResult<u32> {
    c_auction_house_post_commodity(state)
}

fn c_auction_house_cancel_sell(state: &mut LuaState) -> LuaResult<u32> {
    borrow_state_mut(state)?.auction_sell_quote = None;
    Ok(0)
}

fn c_auction_house_cancel_auction(state: &mut LuaState) -> LuaResult<u32> {
    let auction_id = i32::from_stack(state, 1)?;
    let Some((bid_amount, buyout_amount)) = mark_owned_auction_canceled(state, auction_id)? else {
        return Ok(0);
    };
    let cancel_cost = cancel_cost_for_buyout(buyout_amount);
    let refund = (bid_amount - cancel_cost).max(0);
    refund_player_money(state, refund)?;
    dispatch_auction_canceled(state, auction_id)?;
    dispatch_owned_auctions_updated(state)?;
    Ok(0)
}

const BID_ADDED: &str = "BID_ADDED";
const BIDS_UPDATED: &str = "BIDS_UPDATED";
const OWNED_AUCTION_BIDDER_INFO_RECEIVED: &str = "OWNED_AUCTION_BIDDER_INFO_RECEIVED";
const COMMODITY_PRICE_UPDATED: &str = "COMMODITY_PRICE_UPDATED";
const COMMODITY_QUOTE_UPDATED: &str = "COMMODITY_QUOTE_UPDATED";
const COMMODITY_PURCHASE_SUCCEEDED: &str = "COMMODITY_PURCHASE_SUCCEEDED";
const COMMODITY_PURCHASED: &str = "COMMODITY_PURCHASED";

/// Fresh quote-window seeded by `StartCommoditiesPurchase`. The retail
/// server holds a 15s lock; the sim mirrors that even though tests
/// usually confirm immediately.
const COMMODITY_QUOTE_WINDOW_SECONDS: i32 = 15;

/// Bag slot used when adding a purchased commodity to the player's
/// inventory. The sim has no real free-slot picker; Auctions UI tests
/// only care that *some* slot got the item, so the deterministic
/// (4, 0) slot keeps assertions simple.
const PURCHASE_DEPOSIT_BAG: i32 = 4;
const PURCHASE_DEPOSIT_SLOT: i32 = 0;

fn dispatch_bid_added(state: &mut LuaState, auction_id: i64) -> LuaResult<()> {
    dispatch_event_now(state, BID_ADDED, &[Val::Num(auction_id as f64)])
}

fn dispatch_bids_updated(state: &mut LuaState) -> LuaResult<()> {
    dispatch_event_now(state, BIDS_UPDATED, &[])
}

fn dispatch_owned_auction_bidder_info_received(
    state: &mut LuaState,
    auction_id: i32,
) -> LuaResult<()> {
    dispatch_event_now(
        state,
        OWNED_AUCTION_BIDDER_INFO_RECEIVED,
        &[Val::Num(auction_id as f64)],
    )
}

fn dispatch_commodity_price_updated(state: &mut LuaState, item_id: i32) -> LuaResult<()> {
    dispatch_event_now(state, COMMODITY_PRICE_UPDATED, &[Val::Num(item_id as f64)])
}

fn dispatch_commodity_quote_updated(state: &mut LuaState, item_id: i32) -> LuaResult<()> {
    dispatch_event_now(state, COMMODITY_QUOTE_UPDATED, &[Val::Num(item_id as f64)])
}

fn dispatch_commodity_purchase_succeeded(state: &mut LuaState, item_id: i32) -> LuaResult<()> {
    dispatch_event_now(
        state,
        COMMODITY_PURCHASE_SUCCEEDED,
        &[Val::Num(item_id as f64)],
    )
}

fn dispatch_commodity_purchased(state: &mut LuaState, item_id: i32) -> LuaResult<()> {
    dispatch_event_now(state, COMMODITY_PURCHASED, &[Val::Num(item_id as f64)])
}

/// Cheapest-first walk of the listings stack: returns the per-row
/// `(unit_price, drained)` pairs the buyer pays for, and the leftover
/// requested quantity if the stack runs dry. Pure function — does not
/// mutate listings, so callers can use the same plan for both the
/// `Start` (quote) and `Confirm` (drain) paths.
fn plan_commodity_purchase(
    listings: &[CommoditySearchResultInfo],
    requested: i32,
) -> (Vec<(i64, i32)>, i32) {
    let mut plan: Vec<(i64, i32)> = Vec::new();
    let mut remaining = requested.max(0);
    let mut sorted_listings: Vec<&CommoditySearchResultInfo> = listings.iter().collect();
    sorted_listings.sort_by_key(|row| row.unit_price);
    for row in sorted_listings {
        if remaining == 0 {
            break;
        }
        let take = row.quantity.min(remaining);
        if take > 0 {
            plan.push((row.unit_price, take));
            remaining -= take;
        }
    }
    (plan, remaining)
}

fn total_price_for_plan(plan: &[(i64, i32)]) -> i64 {
    plan.iter().map(|(price, qty)| price * (*qty as i64)).sum()
}

/// Drain the planned quantity from the cheapest-first stack. Rows that
/// are fully consumed are removed; partially consumed rows shrink in
/// place. Mirrors the retail server's commodity book draining the
/// lowest-priced listing first.
fn drain_commodity_listings(listings: &mut Vec<CommoditySearchResultInfo>, requested: i32) {
    let mut remaining = requested.max(0);
    listings.sort_by_key(|row| row.unit_price);
    listings.retain_mut(|row| {
        if remaining == 0 {
            return true;
        }
        let take = row.quantity.min(remaining);
        row.quantity -= take;
        remaining -= take;
        row.quantity > 0
    });
}

fn deposit_purchased_into_bag(state: &mut LuaState, item_id: i32, quantity: i32) -> LuaResult<()> {
    let mut sim = borrow_state_mut(state)?;
    let slot = sim
        .bag_items
        .entry((PURCHASE_DEPOSIT_BAG, PURCHASE_DEPOSIT_SLOT))
        .or_insert_with(|| crate::lua_api::state::BagItem {
            item_id: item_id as u32,
            stack_count: 0,
            hyperlink: None,
        });
    if slot.item_id != item_id as u32 {
        slot.item_id = item_id as u32;
        slot.stack_count = 0;
    }
    slot.stack_count += quantity;
    Ok(())
}

fn read_bid_target_buyout(state: &mut LuaState, auction_id: i64) -> LuaResult<Option<i64>> {
    let sim = borrow_state(state)?;
    if let Some(info) = sim.auction_index.get(&auction_id) {
        return Ok(Some(info.buyout_amount));
    }
    let from_owned = sim
        .auction_owned
        .iter()
        .find(|row| row.auction_id as i64 == auction_id)
        .map(|row| row.buyout_amount);
    if let Some(buyout) = from_owned {
        return Ok(Some(buyout));
    }
    let from_searches = sim.auction_item_searches.values().find_map(|bucket| {
        bucket
            .entries
            .iter()
            .find(|entry| entry.auction_id == auction_id)
            .map(|entry| entry.buyout_amount)
    });
    Ok(from_searches)
}

fn append_bid_record(
    state: &mut LuaState,
    auction_id: i64,
    bid_amount: i64,
    buyout_amount: i64,
) -> LuaResult<()> {
    let mut sim = borrow_state_mut(state)?;
    let bidder_name = sim.player.name.clone();
    sim.auction_bids.push(BidAuction {
        auction_id: auction_id as i32,
        item_id: 0,
        item_level: 0,
        quantity: 1,
        bid_amount,
        buyout_amount,
        time_left: TIME_LEFT_BAND_VERY_LONG,
        time_left_seconds: SECONDS_48_HOURS,
        bidder: Some(bidder_name),
    });
    Ok(())
}

fn c_auction_house_place_bid(state: &mut LuaState) -> LuaResult<u32> {
    let auction_id = i64::from_stack(state, 1)?;
    let bid_amount = read_optional_money_arg(state, 2)?;
    let Some(buyout_amount) = read_bid_target_buyout(state, auction_id)? else {
        state.push(Val::Bool(false));
        return Ok(1);
    };
    append_bid_record(state, auction_id, bid_amount, buyout_amount)?;
    deduct_player_money(state, bid_amount)?;
    dispatch_bid_added(state, auction_id)?;
    dispatch_bids_updated(state)?;
    state.push(Val::Bool(true));
    Ok(1)
}

fn c_auction_house_get_auction_info_by_id(state: &mut LuaState) -> LuaResult<u32> {
    let auction_id = i64::from_stack(state, 1)?;
    let info = borrow_state(state)?.auction_index.get(&auction_id).cloned();
    let Some(info) = info else { return Ok(0) };
    push_auction_row_info(state, &info);
    Ok(5)
}

fn push_auction_row_info(state: &mut LuaState, info: &AuctionRowInfo) {
    let owner = create_string(state, &info.owner);
    state.push(owner);
    state.push(Val::Num(info.bid_amount as f64));
    state.push(Val::Num(info.buyout_amount as f64));
    state.push(Val::Num(info.deposit as f64));
    state.push(Val::Num(info.consortium_cut as f64));
}

fn c_auction_house_request_owned_auction_bidder_info(state: &mut LuaState) -> LuaResult<u32> {
    let auction_id = i32::from_stack(state, 1)?;
    dispatch_owned_auction_bidder_info_received(state, auction_id)?;
    state.push(Val::Bool(true));
    Ok(1)
}

fn c_auction_house_start_commodities_purchase(state: &mut LuaState) -> LuaResult<u32> {
    let item_id = i32::from_stack(state, 1)?;
    let quantity = i32::from_stack(state, 2)?;
    let listings = borrow_state(state)?
        .auction_commodity_searches
        .get(&item_id)
        .map(|bucket| bucket.entries.clone())
        .unwrap_or_default();
    let (plan, _) = plan_commodity_purchase(&listings, quantity);
    let total_price = total_price_for_plan(&plan);
    borrow_state_mut(state)?.commodity_purchase_quote = Some(CommodityPurchaseQuote {
        item_id,
        quantity,
        total_price,
        quote_duration_remaining: COMMODITY_QUOTE_WINDOW_SECONDS,
    });
    dispatch_commodity_price_updated(state, item_id)?;
    dispatch_commodity_quote_updated(state, item_id)?;
    Ok(0)
}

fn c_auction_house_confirm_commodities_purchase(state: &mut LuaState) -> LuaResult<u32> {
    let item_id = i32::from_stack(state, 1)?;
    let quantity = i32::from_stack(state, 2)?;
    let total_price = drain_and_total_commodity_purchase(state, item_id, quantity)?;
    deduct_player_money(state, total_price)?;
    deposit_purchased_into_bag(state, item_id, quantity)?;
    borrow_state_mut(state)?.commodity_purchase_quote = None;
    dispatch_commodity_purchase_succeeded(state, item_id)?;
    dispatch_commodity_purchased(state, item_id)?;
    Ok(0)
}

/// Build the per-row spend plan, drain the listings stack, and return
/// the total copper the buyer owes. Wraps the borrow scopes so the
/// caller's deduct/event sequence stays linear.
fn drain_and_total_commodity_purchase(
    state: &mut LuaState,
    item_id: i32,
    quantity: i32,
) -> LuaResult<i64> {
    let mut sim = borrow_state_mut(state)?;
    let bucket = sim.auction_commodity_searches.entry(item_id).or_default();
    let (plan, _) = plan_commodity_purchase(&bucket.entries, quantity);
    drain_commodity_listings(&mut bucket.entries, quantity);
    Ok(total_price_for_plan(&plan))
}

fn c_auction_house_cancel_commodities_purchase(state: &mut LuaState) -> LuaResult<u32> {
    borrow_state_mut(state)?.commodity_purchase_quote = None;
    Ok(0)
}
