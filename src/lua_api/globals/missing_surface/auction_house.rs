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
use crate::lua_api::methods::{borrow_state, create_string, create_table, table_set};
use crate::lua_api::state::{AuctionBrowseResult, OwnedAuction};
use crate::lua_bridge::{FromStack, table_set_rust_fn};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn register_auction_house_surface(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_AuctionHouse")?;
    let methods: &[(&str, fn(&mut LuaState) -> LuaResult<u32>)] = &[
        (
            "GetAuctionItemSubClasses",
            c_auction_house_get_auction_item_sub_classes,
        ),
        ("GetReplicateItemInfo", c_auction_house_get_replicate_item_info),
        ("GetBrowseResults", c_auction_house_get_browse_results),
        ("HasFavorites", c_auction_house_has_favorites),
        ("HasFullBrowseResults", c_auction_house_has_full_browse_results),
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
        ("SupportsCopperValues", c_auction_house_supports_copper_values),
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
        ("QueryOwnedAuctions", c_auction_house_query_owned_auctions),
        ("GetNumOwnedAuctions", c_auction_house_get_num_owned_auctions),
        ("GetOwnedAuctionInfo", c_auction_house_get_owned_auction_info),
        (
            "HasFullOwnedAuctionResults",
            c_auction_house_has_full_owned_auction_results,
        ),
        (
            "GetNumOwnedAuctionTypes",
            c_auction_house_get_num_owned_auction_types,
        ),
        ("GetOwnedAuctionType", c_auction_house_get_owned_auction_type),
    ];
    for &(name, func) in methods {
        table_set_rust_fn(state, table_ref, name, func)?;
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
    let item_key = state.stack_get(1);
    let item_id = match extract_item_key_id(state, item_key) {
        Some(item_id) => item_id,
        None => {
            state.push(Val::Nil);
            return Ok(1);
        }
    };
    let Some(item) = items::get_item(item_id) else {
        state.push(Val::Nil);
        return Ok(1);
    };

    let info = create_table(state);
    let item_name = create_string(state, item.name);
    table_set(state, info, "itemName", item_name);
    table_set(
        state,
        info,
        "iconFileID",
        Val::Num(if item.icon_file_data_id == 0 {
            134400.0
        } else {
            item.icon_file_data_id as f64
        }),
    );
    table_set(state, info, "quality", Val::Num(item.quality as f64));
    table_set(
        state,
        info,
        "isCommodity",
        Val::Bool(item.stackable > 1 && item.inventory_type == 0),
    );
    table_set(state, info, "battlePetLink", Val::Nil);
    table_set(state, info, "appearanceLink", Val::Nil);
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
    state.push(Val::Num(0.0));
    Ok(1)
}

fn c_auction_house_get_bid_info(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn c_auction_house_has_full_bid_results(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    Ok(1)
}

fn c_auction_house_get_num_bid_types(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn c_auction_house_get_bid_type(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
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
    let count = borrow_state(state)?.auction_owned.len() as f64;
    state.push(Val::Num(count));
    Ok(1)
}

fn c_auction_house_get_owned_auction_type(state: &mut LuaState) -> LuaResult<u32> {
    let index = i32::from_stack(state, 1)?;
    if index < 1 {
        return Ok(0);
    }
    let entry = borrow_state(state)?
        .auction_owned
        .get((index - 1) as usize)
        .cloned();
    let Some(entry) = entry else { return Ok(0) };
    let t = create_table(state);
    let item_key = create_table(state);
    table_set(state, item_key, "itemID", Val::Num(entry.item_id as f64));
    table_set(
        state,
        item_key,
        "itemLevel",
        Val::Num(entry.item_level as f64),
    );
    table_set(state, t, "itemKey", item_key);
    state.push(t);
    Ok(1)
}

fn push_owned_auction_table(state: &mut LuaState, entry: &OwnedAuction) -> Val {
    let t = create_table(state);
    let item_key = create_table(state);
    table_set(state, item_key, "itemID", Val::Num(entry.item_id as f64));
    table_set(
        state,
        item_key,
        "itemLevel",
        Val::Num(entry.item_level as f64),
    );
    table_set(state, item_key, "itemSuffix", Val::Num(0.0));
    table_set(state, item_key, "battlePetSpeciesID", Val::Num(0.0));
    table_set(state, t, "itemKey", item_key);
    table_set(
        state,
        t,
        "auctionID",
        Val::Num(entry.auction_id as f64),
    );
    table_set(state, t, "quantity", Val::Num(entry.quantity as f64));
    table_set(state, t, "bidAmount", Val::Num(entry.bid_amount as f64));
    table_set(
        state,
        t,
        "buyoutAmount",
        Val::Num(entry.buyout_amount as f64),
    );
    table_set(state, t, "status", Val::Num(entry.status as f64));
    table_set(state, t, "timeLeft", Val::Num(entry.time_left as f64));
    table_set(
        state,
        t,
        "timeLeftSeconds",
        Val::Num(entry.time_left_seconds as f64),
    );
    t
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
