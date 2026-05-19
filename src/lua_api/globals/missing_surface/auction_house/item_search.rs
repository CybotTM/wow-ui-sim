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
    table_set_static(state, item_key, "itemID", Val::Num(item_id as f64));
    table_set_static(state, item_key, "itemLevel", Val::Num(item_level as f64));
    table_set_static(state, item_key, "itemSuffix", Val::Num(item_suffix as f64));
    table_set_static(
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
    table_set_static(state, item_key, "itemID", Val::Num(row.item_id as f64));
    table_set_static(
        state,
        item_key,
        "itemLevel",
        Val::Num(row.item_level as f64),
    );
    table_set_static(state, item_key, "itemSuffix", Val::Num(0.0));
    table_set_static(state, item_key, "battlePetSpeciesID", Val::Num(0.0));

    table_set_static(state, t, "itemKey", item_key);
    table_set_static(state, t, "minPrice", Val::Num(row.min_price as f64));
    table_set_static(
        state,
        t,
        "totalQuantity",
        Val::Num(row.total_quantity as f64),
    );
    table_set_static(
        state,
        t,
        "containsOwnerItem",
        Val::Bool(row.contains_owner_item),
    );
    table_set_static(state, t, "appearanceLink", Val::Nil);
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
    let table = create_table(state);
    set_item_search_result_identity_fields(state, table, entry);
    set_item_search_result_flag_fields(state, table, entry);
    set_item_search_result_price_fields(state, table, entry);
    table
}

fn set_item_search_result_identity_fields(
    state: &mut LuaState,
    table: Val,
    entry: &ItemSearchResultInfo,
) {
    let owners = build_owners_array(state, &entry.owners);
    let item_link = create_string(state, &entry.item_link);
    let bidder = match &entry.bidder {
        Some(name) => create_string(state, name),
        None => Val::Nil,
    };
    table_set_static(state, table, "owners", owners);
    table_set_static(state, table, "timeLeft", Val::Num(entry.time_left as f64));
    table_set_static(state, table, "auctionID", Val::Num(entry.auction_id as f64));
    table_set_static(state, table, "quantity", Val::Num(entry.quantity as f64));
    table_set_static(state, table, "itemLink", item_link);
    table_set_static(state, table, "bidder", bidder);
}

fn set_item_search_result_flag_fields(
    state: &mut LuaState,
    table: Val,
    entry: &ItemSearchResultInfo,
) {
    table_set_static(
        state,
        table,
        "containsOwnerItem",
        Val::Bool(entry.contains_owner_item),
    );
    table_set_static(
        state,
        table,
        "containsAccountItem",
        Val::Bool(entry.contains_account_item),
    );
    table_set_static(
        state,
        table,
        "containsSocketedItem",
        Val::Bool(entry.contains_socketed_item),
    );
}

fn set_item_search_result_price_fields(
    state: &mut LuaState,
    table: Val,
    entry: &ItemSearchResultInfo,
) {
    table_set_static(state, table, "minBid", Val::Num(entry.min_bid as f64));
    table_set_static(state, table, "bidAmount", Val::Num(entry.bid_amount as f64));
    table_set_static(
        state,
        table,
        "buyoutAmount",
        Val::Num(entry.buyout_amount as f64),
    );
    table_set_static(
        state,
        table,
        "timeLeftSeconds",
        Val::Num(entry.time_left_seconds as f64),
    );
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
