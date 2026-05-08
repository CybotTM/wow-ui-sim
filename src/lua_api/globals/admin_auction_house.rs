//! Rilua A_Admin handlers — Auction House seeding.
//!
//! Focused admin-only mutators for the simulator-backed Auction House data
//! rows used by `C_AuctionHouse.GetBrowseResults`,
//! `C_AuctionHouse.GetReplicateItemInfo`, and the player-owned auctions
//! / bids lists.

use crate::lua_api::methods::{borrow_state_mut, val_to_string};
use crate::lua_api::state::{
    AuctionBrowseResult, AuctionReplicateItem, AuctionRowInfo, BidAuction,
    CommoditySearchResultInfo, CommoditySearchResults, ItemSearchResultInfo, ItemSearchResults,
    OwnedAuction,
};
use crate::lua_bridge::FromStack;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

use super::state_backed_queries::dispatch_event_now;

const AUCTION_HOUSE_BROWSE_RESULTS_UPDATED: &str = "AUCTION_HOUSE_BROWSE_RESULTS_UPDATED";
const AUCTION_HOUSE_THROTTLED_SYSTEM_READY: &str = "AUCTION_HOUSE_THROTTLED_SYSTEM_READY";

pub(super) fn add_auction_browse_result(state: &mut LuaState) -> LuaResult<u32> {
    let item_id = i32::from_stack(state, 1)?;
    let item_level = i32::from_stack(state, 2)?;
    let min_price = i64::from_stack(state, 3)?;
    let total_quantity = i32::from_stack(state, 4)?;
    let contains_owner_item = bool::from_stack(state, 5)?;
    let auction_id = optional_i64_arg(state, 6)?;

    let mut sim = borrow_state_mut(state)?;
    sim.auction_browse_results.push(AuctionBrowseResult {
        item_id,
        item_level,
        min_price,
        total_quantity,
        contains_owner_item,
    });
    if let Some(auction_id) = auction_id {
        sim.auction_index.insert(
            auction_id,
            AuctionRowInfo {
                owner: "Browse Seller".to_string(),
                bid_amount: 0,
                buyout_amount: min_price,
                deposit: 0,
                consortium_cut: 0,
            },
        );
    }
    Ok(0)
}

fn optional_i64_arg(state: &mut LuaState, slot: i32) -> LuaResult<Option<i64>> {
    Ok(match Val::from_stack(state, slot)? {
        Val::Num(value) => Some(value as i64),
        _ => None,
    })
}

pub(super) fn clear_auction_browse_results(state: &mut LuaState) -> LuaResult<u32> {
    borrow_state_mut(state)?.auction_browse_results.clear();
    Ok(0)
}

pub(super) fn add_auction_item_search_result(state: &mut LuaState) -> LuaResult<u32> {
    let result = item_search_result_from_args(state)?;
    let key = (result.item_id, result.item_level, 0, 0);

    borrow_state_mut(state)?
        .auction_item_searches
        .entry(key)
        .or_insert_with(empty_item_search_results)
        .entries
        .push(result.info);
    Ok(0)
}

struct AdminItemSearchResult {
    item_id: i32,
    item_level: i32,
    info: ItemSearchResultInfo,
}

fn item_search_result_from_args(state: &mut LuaState) -> LuaResult<AdminItemSearchResult> {
    let item_id = i32::from_stack(state, 1)?;
    let item_level = i32::from_stack(state, 2)?;
    let auction_id = i32::from_stack(state, 3)?;
    let quantity = i32::from_stack(state, 4)?;
    let min_bid = i64::from_stack(state, 5)?;
    let bid_amount = i64::from_stack(state, 6)?;
    let buyout_amount = i64::from_stack(state, 7)?;
    let time_left = i32::from_stack(state, 8)?;
    let time_left_seconds = i64::from_stack(state, 9)?;
    let owner = val_to_string(state, Val::from_stack(state, 10)?).unwrap_or_default();

    Ok(AdminItemSearchResult {
        item_id,
        item_level,
        info: ItemSearchResultInfo {
            owners: vec![owner],
            time_left,
            auction_id: i64::from(auction_id),
            quantity,
            item_link: format!("|cffffffff|Hitem:{item_id}|h[Item {item_id}]|h|r"),
            contains_owner_item: false,
            contains_account_item: false,
            contains_socketed_item: false,
            bidder: None,
            min_bid,
            bid_amount,
            buyout_amount,
            time_left_seconds,
        },
    })
}

fn empty_item_search_results() -> ItemSearchResults {
    ItemSearchResults {
        entries: Vec::new(),
        has_full_results: true,
    }
}

pub(super) fn clear_auction_item_search_results(state: &mut LuaState) -> LuaResult<u32> {
    borrow_state_mut(state)?.auction_item_searches.clear();
    Ok(0)
}

pub(super) fn add_auction_commodity_search_result(state: &mut LuaState) -> LuaResult<u32> {
    let item_id = i32::from_stack(state, 1)?;
    let quantity = i32::from_stack(state, 2)?;
    let unit_price = i64::from_stack(state, 3)?;
    let auction_id = i64::from_stack(state, 4)?;
    let owner = val_to_string(state, Val::from_stack(state, 5)?).unwrap_or_default();

    borrow_state_mut(state)?
        .auction_commodity_searches
        .entry(item_id)
        .or_insert_with(|| CommoditySearchResults {
            entries: Vec::new(),
            has_full_results: true,
        })
        .entries
        .push(CommoditySearchResultInfo {
            item_id,
            quantity,
            unit_price,
            auction_id,
            owners: vec![owner],
            time_left_seconds: 4 * 60 * 60,
            num_owner_items: 0,
            contains_owner_item: false,
            contains_account_item: false,
        });
    Ok(0)
}

pub(super) fn clear_auction_commodity_search_results(state: &mut LuaState) -> LuaResult<u32> {
    borrow_state_mut(state)?.auction_commodity_searches.clear();
    Ok(0)
}

pub(super) fn set_auction_throttle_ready(state: &mut LuaState) -> LuaResult<u32> {
    let ready = bool::from_stack(state, 1)?;
    let should_dispatch_ready = {
        let mut sim = borrow_state_mut(state)?;
        let was_ready = sim.auction_throttle_ready;
        sim.auction_throttle_ready = ready;
        ready && !was_ready
    };

    if should_dispatch_ready {
        dispatch_event_now(state, AUCTION_HOUSE_THROTTLED_SYSTEM_READY, &[])?;
        dispatch_queued_browse_results(state)?;
    }

    Ok(0)
}

fn dispatch_queued_browse_results(state: &mut LuaState) -> LuaResult<()> {
    let queued_query = {
        let mut sim = borrow_state_mut(state)?;
        sim.auction_queued_browse_query.take()
    };

    if let Some(query) = queued_query {
        borrow_state_mut(state)?.auction_last_browse_query = Some(query);
        dispatch_event_now(state, AUCTION_HOUSE_BROWSE_RESULTS_UPDATED, &[])?;
    }

    Ok(())
}

pub(super) fn add_auction_replicate_item(state: &mut LuaState) -> LuaResult<u32> {
    let name = String::from_stack(state, 1)?;
    let texture = match Val::from_stack(state, 2)? {
        Val::Num(value) if value >= 0.0 => value as u32,
        _ => 0,
    };
    let count = i32::from_stack(state, 3)?;
    let quality_id = i32::from_stack(state, 4)?;
    let usable = bool::from_stack(state, 5)?;
    let level = i32::from_stack(state, 6)?;
    let level_type = String::from_stack(state, 7)?;

    borrow_state_mut(state)?
        .auction_replicate_items
        .push(AuctionReplicateItem {
            name,
            texture,
            count,
            quality_id,
            usable,
            level,
            level_type,
        });
    Ok(0)
}

pub(super) fn clear_auction_replicate_items(state: &mut LuaState) -> LuaResult<u32> {
    borrow_state_mut(state)?.auction_replicate_items.clear();
    Ok(0)
}

/// Append one player-owned auction. Args, in order:
/// `auction_id`, `item_id`, `item_level`, `quantity`, `bid_amount`,
/// `buyout_amount`, `status`, `time_left`, `time_left_seconds`.
/// `status` is `Enum.AuctionStatus` (0 Active / 1 Sold); `time_left`
/// is `Enum.AuctionHouseTimeLeftBand` (1 Short … 4 VeryLong).
pub(super) fn add_owned_auction(state: &mut LuaState) -> LuaResult<u32> {
    let auction_id = i32::from_stack(state, 1)?;
    let item_id = i32::from_stack(state, 2)?;
    let item_level = i32::from_stack(state, 3)?;
    let quantity = i32::from_stack(state, 4)?;
    let bid_amount = i64::from_stack(state, 5)?;
    let buyout_amount = i64::from_stack(state, 6)?;
    let status = i32::from_stack(state, 7)?;
    let time_left = i32::from_stack(state, 8)?;
    let time_left_seconds = i64::from_stack(state, 9)?;

    borrow_state_mut(state)?.auction_owned.push(OwnedAuction {
        auction_id,
        item_id,
        item_level,
        quantity,
        bid_amount,
        buyout_amount,
        status,
        time_left,
        time_left_seconds,
    });
    Ok(0)
}

pub(super) fn clear_owned_auctions(state: &mut LuaState) -> LuaResult<u32> {
    borrow_state_mut(state)?.auction_owned.clear();
    Ok(0)
}

/// Append one active bid row. Args, in order:
/// `auction_id`, `item_id`, `item_level`, `quantity`, `bid_amount`,
/// `buyout_amount`, `time_left`, `time_left_seconds`, `bidder`.
/// Pass nil for `bidder` to model "no bid yet"; pass `UnitGUID("player")`
/// to model a player-leading bid.
pub(super) fn add_auction_bid(state: &mut LuaState) -> LuaResult<u32> {
    let auction_id = i32::from_stack(state, 1)?;
    let item_id = i32::from_stack(state, 2)?;
    let item_level = i32::from_stack(state, 3)?;
    let quantity = i32::from_stack(state, 4)?;
    let bid_amount = i64::from_stack(state, 5)?;
    let buyout_amount = i64::from_stack(state, 6)?;
    let time_left = i32::from_stack(state, 7)?;
    let time_left_seconds = i64::from_stack(state, 8)?;
    let bidder_value = Val::from_stack(state, 9)?;
    let bidder = match bidder_value {
        Val::Nil => None,
        value => val_to_string(state, value),
    };

    borrow_state_mut(state)?.auction_bids.push(BidAuction {
        auction_id,
        item_id,
        item_level,
        quantity,
        bid_amount,
        buyout_amount,
        time_left,
        time_left_seconds,
        bidder,
    });
    Ok(0)
}

pub(super) fn clear_auction_bids(state: &mut LuaState) -> LuaResult<u32> {
    borrow_state_mut(state)?.auction_bids.clear();
    Ok(0)
}
