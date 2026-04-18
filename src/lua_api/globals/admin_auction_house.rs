//! Rilua A_Admin handlers — Auction House seeding.
//!
//! Focused admin-only mutators for the simulator-backed Auction House data
//! rows used by `C_AuctionHouse.GetBrowseResults` and
//! `C_AuctionHouse.GetReplicateItemInfo`.

use crate::lua_api::methods::borrow_state_mut;
use crate::lua_api::state::{AuctionBrowseResult, AuctionReplicateItem};
use crate::lua_bridge::FromStack;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn add_auction_browse_result(state: &mut LuaState) -> LuaResult<u32> {
    let item_id = i32::from_stack(state, 1)?;
    let item_level = i32::from_stack(state, 2)?;
    let min_price = i64::from_stack(state, 3)?;
    let total_quantity = i32::from_stack(state, 4)?;
    let contains_owner_item = bool::from_stack(state, 5)?;

    borrow_state_mut(state)?
        .auction_browse_results
        .push(AuctionBrowseResult {
            item_id,
            item_level,
            min_price,
            total_quantity,
            contains_owner_item,
        });
    Ok(0)
}

pub(super) fn clear_auction_browse_results(state: &mut LuaState) -> LuaResult<u32> {
    borrow_state_mut(state)?.auction_browse_results.clear();
    Ok(0)
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
