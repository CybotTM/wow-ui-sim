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

/// One row of the player's *own* posted auctions (Auctions tab).
/// Drives `C_AuctionHouse.GetNumOwnedAuctions` /
/// `GetOwnedAuctionInfo`. Real retail returns a richer shape; the
/// fields here are the ones the Auctions panel templates actually
/// read.
#[derive(Debug, Clone)]
pub struct OwnedAuction {
    /// Unique auction id assigned by the server. Tests can pass any
    /// non-zero value; the panel uses it as the row key.
    pub auction_id: i32,
    pub item_id: i32,
    pub item_level: i32,
    pub quantity: i32,
    /// Highest bid in copper (0 when no bid has been placed).
    pub bid_amount: i64,
    /// Buyout price in copper (0 when none was set).
    pub buyout_amount: i64,
    /// `Enum.AuctionStatus` (0 = Active, 1 = Sold).
    pub status: i32,
    /// `Enum.AuctionHouseTimeLeftBand` (1 = Short … 4 = VeryLong).
    pub time_left: i32,
    /// Remaining auction duration in seconds.
    pub time_left_seconds: i64,
}

/// One row of the player's active bid list (Bids tab). Drives
/// `C_AuctionHouse.GetNumBids` / `GetBidInfo`. The bidder field uses
/// the same shape Blizzard expects from `GetBidStatus`: nil = no bid,
/// player GUID = player's current high bid, anything else = another
/// bidder currently leads.
#[derive(Debug, Clone)]
pub struct BidAuction {
    pub auction_id: i32,
    pub item_id: i32,
    pub item_level: i32,
    pub quantity: i32,
    pub bid_amount: i64,
    pub buyout_amount: i64,
    /// `Enum.AuctionHouseTimeLeftBand` (1 = Short … 4 = VeryLong).
    pub time_left: i32,
    /// Remaining auction duration in seconds.
    pub time_left_seconds: i64,
    pub bidder: Option<String>,
}
