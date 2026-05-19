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
    ("PlaceBid", purchases::c_auction_house_place_bid),
    (
        "GetAuctionInfoByID",
        purchases::c_auction_house_get_auction_info_by_id,
    ),
    (
        "RequestOwnedAuctionBidderInfo",
        purchases::c_auction_house_request_owned_auction_bidder_info,
    ),
    (
        "StartCommoditiesPurchase",
        purchases::c_auction_house_start_commodities_purchase,
    ),
    (
        "ConfirmCommoditiesPurchase",
        purchases::c_auction_house_confirm_commodities_purchase,
    ),
    (
        "CancelCommoditiesPurchase",
        purchases::c_auction_house_cancel_commodities_purchase,
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
    table_set_static(state, t, "status", Val::Num(entry.status as f64));
    t
}

fn push_bid_auction_table(state: &mut LuaState, entry: &BidAuction) -> Val {
    let t = push_auction_row_table(state, entry);
    match &entry.bidder {
        Some(bidder) => {
            let bidder_val = create_string(state, bidder);
            table_set_static(state, t, "bidder", bidder_val);
        }
        None => table_set_static(state, t, "bidder", Val::Nil),
    }
    t
}

fn push_item_key_table(state: &mut LuaState, item_id: i32, item_level: i32) -> Val {
    let item_key = create_table(state);
    table_set_static(state, item_key, "itemID", Val::Num(item_id as f64));
    table_set_static(state, item_key, "itemLevel", Val::Num(item_level as f64));
    table_set_static(state, item_key, "itemSuffix", Val::Num(0.0));
    table_set_static(state, item_key, "battlePetSpeciesID", Val::Num(0.0));
    item_key
}

fn push_item_key_info_table(state: &mut LuaState, item: &items::ItemInfo) -> Val {
    let info = create_table(state);
    let item_name = create_string(state, item.name);
    table_set_static(state, info, "itemName", item_name);
    table_set_static(state, info, "iconFileID", Val::Num(item_icon_file_id(item)));
    table_set_static(state, info, "quality", Val::Num(item.quality as f64));
    table_set_static(
        state,
        info,
        "isCommodity",
        Val::Bool(item_is_commodity(item)),
    );
    table_set_static(state, info, "battlePetLink", Val::Nil);
    table_set_static(state, info, "appearanceLink", Val::Nil);
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
    table_set_static(state, row_table, "itemKey", item_key);
    table_set_static(state, row_table, "auctionID", Val::Num(auction_id as f64));
    table_set_static(state, row_table, "quantity", Val::Num(quantity as f64));
    table_set_static(state, row_table, "bidAmount", Val::Num(bid_amount as f64));
    table_set_static(
        state,
        row_table,
        "buyoutAmount",
        Val::Num(buyout_amount as f64),
    );
    table_set_static(state, row_table, "timeLeft", Val::Num(time_left as f64));
    table_set_static(
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
