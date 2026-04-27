//! Tests for the `C_AuctionHouse` bid + commodity-purchase flow:
//! `PlaceBid`, `GetAuctionInfoByID`, `RequestOwnedAuctionBidderInfo`,
//! `StartCommoditiesPurchase`, `ConfirmCommoditiesPurchase`,
//! `CancelCommoditiesPurchase`. Covers `auction_index` side-population,
//! `commodity_purchase_quote` capture, listings drain math, money flow,
//! bag deposit, and the BID_ADDED / BIDS_UPDATED /
//! OWNED_AUCTION_BIDDER_INFO_RECEIVED / COMMODITY_PRICE_UPDATED /
//! COMMODITY_QUOTE_UPDATED / COMMODITY_PURCHASE_SUCCEEDED /
//! COMMODITY_PURCHASED event dispatch.

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::{
    AuctionRowInfo, CommoditySearchResultInfo, CommoditySearchResults, ItemSearchResultInfo,
    ItemSearchResults,
};

const AQIRITE_ID: i32 = 210934;
const STARTING_MONEY: i64 = 1_000_000;
const BAG_INDEX: i32 = 4;
const BAG_SLOT: i32 = 0;

fn env_with_money() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.state().borrow_mut().player.money = STARTING_MONEY;
    env
}

fn seed_searchable_auction(env: &WowLuaEnv, auction_id: i64, bid: i64, buyout: i64) {
    let key = (AQIRITE_ID, 0, 0, 0);
    let mut results = ItemSearchResults::default();
    results.entries.push(ItemSearchResultInfo {
        owners: vec!["Bob".to_string()],
        time_left: 3,
        auction_id,
        quantity: 1,
        item_link: format!("|cff9d9d9d|Hitem:{AQIRITE_ID}::::|h[item]|h|r"),
        contains_owner_item: false,
        contains_account_item: false,
        contains_socketed_item: false,
        bidder: None,
        min_bid: bid,
        bid_amount: bid,
        buyout_amount: buyout,
        time_left_seconds: 48 * 60 * 60,
    });
    results.has_full_results = true;
    env.state()
        .borrow_mut()
        .auction_item_searches
        .insert(key, results);
}

fn seed_commodity_listings(env: &WowLuaEnv, listings: &[(i32, i64)]) {
    let entries: Vec<CommoditySearchResultInfo> = listings
        .iter()
        .map(|(qty, price)| CommoditySearchResultInfo {
            item_id: AQIRITE_ID,
            quantity: *qty,
            unit_price: *price,
            auction_id: 0,
            owners: vec!["Carol".to_string()],
            time_left_seconds: 48 * 60 * 60,
            num_owner_items: 0,
            contains_owner_item: false,
            contains_account_item: false,
        })
        .collect();
    let mut results = CommoditySearchResults::default();
    results.entries = entries;
    results.has_full_results = true;
    env.state()
        .borrow_mut()
        .auction_commodity_searches
        .insert(AQIRITE_ID, results);
}

fn seed_auction_index(env: &WowLuaEnv, auction_id: i64, info: AuctionRowInfo) {
    env.state()
        .borrow_mut()
        .auction_index
        .insert(auction_id, info);
}

const BID_LISTENER: &str = r#"
    local bid_added_count, last_bid_id = 0, 0
    local bids_updated_count = 0
    local added_listener = CreateFrame("Frame")
    added_listener:RegisterEvent("BID_ADDED")
    added_listener:SetScript("OnEvent", function(_, _, id)
        bid_added_count = bid_added_count + 1
        last_bid_id = id or 0
    end)
    local updated_listener = CreateFrame("Frame")
    updated_listener:RegisterEvent("BIDS_UPDATED")
    updated_listener:SetScript("OnEvent", function() bids_updated_count = bids_updated_count + 1 end)
"#;

const PURCHASE_LISTENER: &str = r#"
    local price_count, quote_count = 0, 0
    local succeeded_count, purchased_count = 0, 0
    local price_listener = CreateFrame("Frame")
    price_listener:RegisterEvent("COMMODITY_PRICE_UPDATED")
    price_listener:SetScript("OnEvent", function() price_count = price_count + 1 end)
    local quote_listener = CreateFrame("Frame")
    quote_listener:RegisterEvent("COMMODITY_QUOTE_UPDATED")
    quote_listener:SetScript("OnEvent", function() quote_count = quote_count + 1 end)
    local succeeded_listener = CreateFrame("Frame")
    succeeded_listener:RegisterEvent("COMMODITY_PURCHASE_SUCCEEDED")
    succeeded_listener:SetScript("OnEvent", function() succeeded_count = succeeded_count + 1 end)
    local purchased_listener = CreateFrame("Frame")
    purchased_listener:RegisterEvent("COMMODITY_PURCHASED")
    purchased_listener:SetScript("OnEvent", function() purchased_count = purchased_count + 1 end)
"#;

#[test]
fn place_bid_appends_bid_record_deducts_money_and_fires_events() {
    let env = env_with_money();
    seed_searchable_auction(&env, 555, 100, 500);
    let script = format!(
        r#"
        {listener}
        local ok = C_AuctionHouse.PlaceBid(555, 200)
        return ok, bid_added_count, last_bid_id, bids_updated_count
        "#,
        listener = BID_LISTENER,
    );
    let (ok, added_count, last_id, updated_count): (bool, i32, i32, i32) =
        env.eval(&script).unwrap();
    assert!(ok);
    assert_eq!(added_count, 1);
    assert_eq!(last_id, 555);
    assert_eq!(updated_count, 1);

    let state = env.state().borrow();
    assert_eq!(state.auction_bids.len(), 1);
    let row = &state.auction_bids[0];
    assert_eq!(row.auction_id, 555);
    assert_eq!(row.bid_amount, 200);
    assert_eq!(row.buyout_amount, 500);
    assert_eq!(state.player.money, STARTING_MONEY - 200);
}

#[test]
fn place_bid_returns_false_for_unknown_auction_id() {
    let env = env_with_money();
    let ok: bool = env
        .eval("return C_AuctionHouse.PlaceBid(9999, 100)")
        .unwrap();
    assert!(!ok);
    assert!(env.state().borrow().auction_bids.is_empty());
    assert_eq!(env.state().borrow().player.money, STARTING_MONEY);
}

#[test]
fn place_bid_resolves_auction_via_owned_listing() {
    let env = env_with_money();
    use wow_ui_sim::lua_api::state::OwnedAuction;
    env.state().borrow_mut().auction_owned.push(OwnedAuction {
        auction_id: 77,
        item_id: AQIRITE_ID,
        item_level: 70,
        quantity: 1,
        bid_amount: 0,
        buyout_amount: 1234,
        status: 0,
        time_left: 4,
        time_left_seconds: 48 * 60 * 60,
    });
    let ok: bool = env.eval("return C_AuctionHouse.PlaceBid(77, 50)").unwrap();
    assert!(ok);
    let state = env.state().borrow();
    assert_eq!(state.auction_bids[0].buyout_amount, 1234);
}

#[test]
fn get_auction_info_by_id_returns_owner_bid_buyout_deposit_consortium_tuple() {
    let env = env_with_money();
    seed_auction_index(
        &env,
        42,
        AuctionRowInfo {
            owner: "Stormwind Vendor".to_string(),
            bid_amount: 100,
            buyout_amount: 500,
            deposit: 75,
            consortium_cut: 25,
        },
    );
    let (owner, bid, buyout, deposit, consortium): (String, i64, i64, i64, i64) = env
        .eval("return C_AuctionHouse.GetAuctionInfoByID(42)")
        .unwrap();
    assert_eq!(owner, "Stormwind Vendor");
    assert_eq!(bid, 100);
    assert_eq!(buyout, 500);
    assert_eq!(deposit, 75);
    assert_eq!(consortium, 25);
}

#[test]
fn get_auction_info_by_id_returns_nil_when_index_misses() {
    let env = env_with_money();
    let owner: Option<String> = env
        .eval("return C_AuctionHouse.GetAuctionInfoByID(404)")
        .unwrap();
    assert_eq!(owner, None);
}

#[test]
fn post_item_populates_auction_index_with_bid_buyout_deposit() {
    let env = env_with_money();
    let auction_id: i32 = env
        .eval(
            r#"
            C_AuctionHouse.PostItem({ itemID = 210934 }, 1, 1, 100, 500)
            return C_AuctionHouse.GetNumOwnedAuctions() > 0 and 1 or 0
            "#,
        )
        .unwrap();
    assert_eq!(auction_id, 1);
    let state = env.state().borrow();
    let posted_id = state.auction_owned[0].auction_id as i64;
    let info = state
        .auction_index
        .get(&posted_id)
        .expect("PostItem must populate auction_index");
    assert_eq!(info.bid_amount, 100);
    assert_eq!(info.buyout_amount, 500);
    let expected_deposit = (1500_i64 * 1 * 15 * 1) / 100;
    assert_eq!(info.deposit, expected_deposit);
}

#[test]
fn request_owned_auction_bidder_info_fires_event_and_returns_true() {
    let env = env_with_money();
    let script = r#"
        local count, last_id = 0, 0
        local listener = CreateFrame("Frame")
        listener:RegisterEvent("OWNED_AUCTION_BIDDER_INFO_RECEIVED")
        listener:SetScript("OnEvent", function(_, _, id)
            count = count + 1
            last_id = id or 0
        end)
        local ok = C_AuctionHouse.RequestOwnedAuctionBidderInfo(99)
        return ok, count, last_id
    "#;
    let (ok, count, last_id): (bool, i32, i32) = env.eval(script).unwrap();
    assert!(ok);
    assert_eq!(count, 1);
    assert_eq!(last_id, 99);
}

#[test]
fn start_commodities_purchase_writes_quote_and_fires_price_quote_events() {
    let env = env_with_money();
    seed_commodity_listings(&env, &[(5, 100), (10, 150)]);
    let script = format!(
        r#"
        {listener}
        C_AuctionHouse.StartCommoditiesPurchase(210934, 12)
        return price_count, quote_count
        "#,
        listener = PURCHASE_LISTENER,
    );
    let (price_count, quote_count): (i32, i32) = env.eval(&script).unwrap();
    assert_eq!(price_count, 1);
    assert_eq!(quote_count, 1);

    let state = env.state().borrow();
    let quote = state
        .commodity_purchase_quote
        .as_ref()
        .expect("Start writes the quote");
    assert_eq!(quote.item_id, AQIRITE_ID);
    assert_eq!(quote.quantity, 12);
    let expected_total = 5 * 100 + 7 * 150;
    assert_eq!(quote.total_price, expected_total);
}

#[test]
fn start_commodities_purchase_with_empty_listings_writes_zero_total() {
    let env = env_with_money();
    let script = format!(
        r#"
        {listener}
        C_AuctionHouse.StartCommoditiesPurchase(210934, 5)
        return price_count, quote_count
        "#,
        listener = PURCHASE_LISTENER,
    );
    let (price_count, quote_count): (i32, i32) = env.eval(&script).unwrap();
    assert_eq!(price_count, 1);
    assert_eq!(quote_count, 1);

    let state = env.state().borrow();
    let quote = state.commodity_purchase_quote.as_ref().unwrap();
    assert_eq!(quote.total_price, 0);
    assert_eq!(quote.quantity, 5);
}

#[test]
fn confirm_commodities_purchase_drains_listings_cheapest_first() {
    let env = env_with_money();
    seed_commodity_listings(&env, &[(5, 100), (10, 150), (20, 200)]);
    env.eval::<()>("C_AuctionHouse.StartCommoditiesPurchase(210934, 12)")
        .unwrap();
    env.eval::<()>("C_AuctionHouse.ConfirmCommoditiesPurchase(210934, 12)")
        .unwrap();

    let state = env.state().borrow();
    let bucket = state
        .auction_commodity_searches
        .get(&AQIRITE_ID)
        .expect("bucket survives drain");
    assert_eq!(
        bucket.entries.len(),
        2,
        "5-stack fully drained, 10-stack partial, 20-stack untouched"
    );
    assert_eq!(bucket.entries[0].quantity, 3);
    assert_eq!(bucket.entries[0].unit_price, 150);
    assert_eq!(bucket.entries[1].quantity, 20);
    assert_eq!(bucket.entries[1].unit_price, 200);
}

#[test]
fn confirm_commodities_purchase_deducts_money_and_fires_succeeded_purchased_events() {
    let env = env_with_money();
    seed_commodity_listings(&env, &[(5, 100), (10, 150)]);
    let script = format!(
        r#"
        {listener}
        C_AuctionHouse.StartCommoditiesPurchase(210934, 12)
        C_AuctionHouse.ConfirmCommoditiesPurchase(210934, 12)
        return succeeded_count, purchased_count
        "#,
        listener = PURCHASE_LISTENER,
    );
    let (succeeded_count, purchased_count): (i32, i32) = env.eval(&script).unwrap();
    assert_eq!(succeeded_count, 1);
    assert_eq!(purchased_count, 1);

    let state = env.state().borrow();
    let expected_total = 5 * 100 + 7 * 150;
    assert_eq!(state.player.money, STARTING_MONEY - expected_total);
    assert!(
        state.commodity_purchase_quote.is_none(),
        "Confirm clears the quote after committing"
    );
}

#[test]
fn confirm_commodities_purchase_deposits_into_player_bag() {
    let env = env_with_money();
    seed_commodity_listings(&env, &[(20, 50)]);
    env.eval::<()>("C_AuctionHouse.StartCommoditiesPurchase(210934, 7)")
        .unwrap();
    env.eval::<()>("C_AuctionHouse.ConfirmCommoditiesPurchase(210934, 7)")
        .unwrap();

    let state = env.state().borrow();
    let slot = state
        .bag_items
        .get(&(BAG_INDEX, BAG_SLOT))
        .expect("commodity lands in deterministic deposit slot");
    assert_eq!(slot.item_id, AQIRITE_ID as u32);
    assert_eq!(slot.stack_count, 7);
}

#[test]
fn cancel_commodities_purchase_clears_quote_without_firing_events() {
    let env = env_with_money();
    seed_commodity_listings(&env, &[(5, 100)]);
    let script = format!(
        r#"
        {listener}
        C_AuctionHouse.StartCommoditiesPurchase(210934, 5)
        local before_succeeded = succeeded_count
        local before_purchased = purchased_count
        C_AuctionHouse.CancelCommoditiesPurchase()
        return before_succeeded, before_purchased, succeeded_count, purchased_count
        "#,
        listener = PURCHASE_LISTENER,
    );
    let (before_s, before_p, after_s, after_p): (i32, i32, i32, i32) = env.eval(&script).unwrap();
    assert_eq!(before_s, 0);
    assert_eq!(before_p, 0);
    assert_eq!(after_s, before_s, "Cancel must not fire SUCCEEDED");
    assert_eq!(after_p, before_p, "Cancel must not fire PURCHASED");

    let state = env.state().borrow();
    assert!(state.commodity_purchase_quote.is_none());
    let bucket = state.auction_commodity_searches.get(&AQIRITE_ID).unwrap();
    assert_eq!(
        bucket.entries.len(),
        1,
        "Cancel must not drain listings — only Confirm does"
    );
    assert_eq!(state.player.money, STARTING_MONEY);
}
