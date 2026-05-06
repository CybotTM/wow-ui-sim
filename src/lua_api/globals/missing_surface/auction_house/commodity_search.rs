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
