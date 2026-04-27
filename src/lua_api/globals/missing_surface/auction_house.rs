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
    borrow_state, create_string, create_table, create_table_with_fields, table_set,
};
use crate::lua_api::state::{
    AuctionBrowseResult, BidAuction, ItemSearchKey, ItemSearchResultInfo, ItemSearchResults,
    OwnedAuction,
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
    let Some(item) = item_from_item_key(state) else {
        state.push(Val::Nil);
        return Ok(1);
    };

    let info = push_item_key_info_table(state, item);
    state.push(info);
    Ok(1)
}

fn c_auction_house_get_item_key_required_level(state: &mut LuaState) -> LuaResult<u32> {
    let item_key = state.stack_get(1);
    let level = extract_item_key_id(state, item_key)
        .and_then(items::get_item)
        .map(|item| item.required_level as f64)
        .unwrap_or(0.0);
    state.push(Val::Num(level));
    Ok(1)
}

fn c_auction_house_get_extra_browse_info(state: &mut LuaState) -> LuaResult<u32> {
    let item_key = state.stack_get(1);
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

fn item_from_item_key(state: &mut LuaState) -> Option<&'static items::ItemInfo> {
    extract_item_key_id(state, state.stack_get(1)).and_then(items::get_item)
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

/// Resolve an `ItemLocation`-shaped Lua table to an `itemID`. The sim
/// has no bag/equipment inventory yet, so tests pass `{ itemID = X }`
/// directly. When a `resolve_item_location` helper lands this can grow
/// a bag/slot fallback; today the shape matches an `ItemKey`, so
/// delegate to the shared field reader.
fn extract_item_id_from_location(state: &mut LuaState, value: Val) -> Option<u32> {
    extract_item_key_id(state, value)
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
            ("containsAccountItem", Val::Bool(entry.contains_account_item)),
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
