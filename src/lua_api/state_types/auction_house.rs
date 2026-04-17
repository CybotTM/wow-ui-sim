//! Auction House sim-state types.

/// One row of the Auction House Browse results list. Drives
/// `C_AuctionHouse.GetBrowseResults`. Seeded with a couple of
/// representative entries.
#[derive(Debug, Clone)]
pub struct AuctionBrowseResult {
    pub item_id: i32,
    pub item_level: i32,
    /// Seller-posted minimum price in copper.
    pub min_price: i64,
    pub total_quantity: i32,
    pub contains_owner_item: bool,
}

/// One row of the Auction House replicate scan (commodities listing
/// snapshot). Drives `C_AuctionHouse.GetReplicateItemInfo`.
#[derive(Debug, Clone)]
pub struct AuctionReplicateItem {
    pub name: String,
    pub texture: u32,
    pub count: i32,
    pub quality_id: i32,
    pub usable: bool,
    pub level: i32,
    pub level_type: String,
}
