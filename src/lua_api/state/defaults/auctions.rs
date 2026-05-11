use super::*;

pub(in crate::lua_api::state) fn default_auction_browse_results() -> Vec<AuctionBrowseResult> {
    vec![
        AuctionBrowseResult {
            auction_id: Some(1),
            item_id: 210935,
            item_level: 70,
            min_price: 25_000,
            total_quantity: 400,
            contains_owner_item: false,
        },
        AuctionBrowseResult {
            auction_id: Some(2),
            item_id: 122245,
            item_level: 50,
            min_price: 1_500_000,
            total_quantity: 1,
            contains_owner_item: true,
        },
    ]
}

/// Seed the `SimState.auction_replicate_items` list with two
/// commodity rows so `GetReplicateItemInfo(index)` returns data for
/// both index 1 and 2.
pub(in crate::lua_api::state) fn default_auction_replicate_items() -> Vec<AuctionReplicateItem> {
    vec![
        AuctionReplicateItem {
            name: "Aqirite".into(),
            texture: 0,
            count: 20,
            quality_id: 2,
            usable: true,
            level: 70,
            level_type: "Item Level".into(),
        },
        AuctionReplicateItem {
            name: "Burnished Helm of Might".into(),
            texture: 133071,
            count: 1,
            quality_id: 3,
            usable: true,
            level: 50,
            level_type: "Item Level".into(),
        },
    ]
}
