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

/// Canonical `ItemKey` tuple: `(itemID, itemLevel, itemSuffix,
/// battlePetSpeciesID)`. Used as the lookup key for
/// `state.auction_item_searches` so the same query that returned a row
/// can later read or refresh it. Tests build it via the helper that
/// matches `MakeItemKey`'s shape.
pub type ItemSearchKey = (i32, i32, i32, i32);

/// Per-search-key result bucket. Drives the
/// `C_AuctionHouse.GetNumItemSearchResults` /
/// `GetItemSearchResultInfo` family. `has_full_results == false`
/// signals the addon's "load more" button (i.e. the next
/// `RequestMoreItemSearchResults` call should refill the bucket).
#[derive(Debug, Clone, Default)]
pub struct ItemSearchResults {
    pub entries: Vec<ItemSearchResultInfo>,
    pub has_full_results: bool,
}

/// One row of an item-search result list. Mirrors the canonical retail
/// shape consumed by `AuctionHouseItemBuyFrame.lua`. The 13 fields
/// match the public `ItemSearchResultInfo` struct in
/// `AuctionHouseDocumentation.lua`.
#[derive(Debug, Clone)]
pub struct ItemSearchResultInfo {
    pub owners: Vec<String>,
    /// `Enum.AuctionHouseTimeLeftBand` (0..3).
    pub time_left: i32,
    pub auction_id: i64,
    pub quantity: i32,
    pub item_link: String,
    pub contains_owner_item: bool,
    pub contains_account_item: bool,
    pub contains_socketed_item: bool,
    /// `nil` when no bid; player GUID when the player is the high
    /// bidder; any other string when another bidder leads.
    pub bidder: Option<String>,
    pub min_bid: i64,
    pub bid_amount: i64,
    pub buyout_amount: i64,
    pub time_left_seconds: i64,
}

/// Per-itemID commodity-search result bucket. Drives the
/// `C_AuctionHouse.GetNumCommoditySearchResults` /
/// `GetCommoditySearchResultInfo` family. Commodity searches do not use
/// the full `ItemKey` tuple — commodities are stack-only items so the
/// `itemID` alone uniquely identifies the listing pile.
/// `has_full_results == false` signals the addon's "load more" button.
#[derive(Debug, Clone, Default)]
pub struct CommoditySearchResults {
    pub entries: Vec<CommoditySearchResultInfo>,
    pub has_full_results: bool,
}

/// One row of a commodity search result list. Mirrors the canonical
/// retail shape consumed by `AuctionHouseCommoditiesBuyFrame.lua` to
/// render the per-unit pricing list.
#[derive(Debug, Clone)]
pub struct CommoditySearchResultInfo {
    pub item_id: i32,
    pub quantity: i32,
    /// Per-unit price in copper. Lowest unit price posts first.
    pub unit_price: i64,
    pub auction_id: i64,
    pub owners: Vec<String>,
    pub time_left_seconds: i64,
    /// How many of `quantity` belong to the player. Used by the buy UI
    /// to gray out rows that would buy back the player's own listing.
    pub num_owner_items: i32,
    pub contains_owner_item: bool,
    pub contains_account_item: bool,
}

/// One sort spec from the `sorts` table accepted by `SendBrowseQuery`,
/// `SendSearchQuery`, `SendSellSearchQuery`, and `SearchForFavorites`.
/// Matches retail's `AuctionHouseSortType` shape.
#[derive(Debug, Clone)]
pub struct AuctionSortSpec {
    /// Numeric `Enum.AuctionHouseSortOrder` (column index).
    pub sort_order: i32,
    pub reverse_sort: bool,
}

/// One row from `BrowseQuery.itemClassFilters`. Mirrors retail's
/// `AuctionHouseItemClassFilter` struct.
#[derive(Debug, Clone)]
pub struct AuctionItemClassFilter {
    pub class_id: i32,
    pub sub_class_id: Option<i32>,
    pub inventory_type: Option<i32>,
}

/// Captured `BrowseQuery` payload stored into
/// `state.auction_last_browse_query` when `SendBrowseQuery` runs. Mirrors
/// retail's `AuctionHouseBrowseQuery` struct so tests can introspect the
/// most-recent query exactly the way the addon submitted it.
#[derive(Debug, Clone, Default)]
pub struct BrowseQuery {
    pub search_string: String,
    pub sorts: Vec<AuctionSortSpec>,
    pub min_level: Option<i32>,
    pub max_level: Option<i32>,
    /// `Enum.AuctionHouseFilter` ids.
    pub filters: Vec<i32>,
    pub item_class_filters: Vec<AuctionItemClassFilter>,
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
