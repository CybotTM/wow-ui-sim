//! Tests for `C_AuctionHouse` per-`ItemKey` item-search-results state
//! machine. The simulator stores `state.auction_item_searches` as a
//! `HashMap<ItemSearchKey, ItemSearchResults>`, keyed by the canonical
//! `(itemID, itemLevel, itemSuffix, battlePetSpeciesID)` tuple. The 9
//! probes here read/refresh that bucket; mutators fire
//! `ITEM_SEARCH_RESULTS_UPDATED`.

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::{ItemSearchKey, ItemSearchResultInfo, ItemSearchResults};

const AQIRITE_KEY: ItemSearchKey = (210935, 70, 0, 0);

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

fn sample_entry(quantity: i32, bid: i64, buyout: i64) -> ItemSearchResultInfo {
    ItemSearchResultInfo {
        owners: vec!["Tester-Realm".into()],
        time_left: 2,
        auction_id: 1234567,
        quantity,
        item_link: "|cffffffff|Hitem:210935::::::::70:::::|h[Aqirite]|h|r".into(),
        contains_owner_item: false,
        contains_account_item: false,
        contains_socketed_item: false,
        bidder: None,
        min_bid: bid,
        bid_amount: bid,
        buyout_amount: buyout,
        time_left_seconds: 4 * 60 * 60,
    }
}

fn seed_aqirite_bucket(env: &WowLuaEnv, entries: Vec<ItemSearchResultInfo>, has_full: bool) {
    let mut state = env.state().borrow_mut();
    state.auction_item_searches.insert(
        AQIRITE_KEY,
        ItemSearchResults {
            entries,
            has_full_results: has_full,
        },
    );
}

#[test]
fn get_num_item_search_results_returns_zero_for_unseeded_key() {
    let env = env();
    let count: i32 = env
        .eval(
            r#"
            local key = C_AuctionHouse.MakeItemKey(210935, 70)
            return C_AuctionHouse.GetNumItemSearchResults(key)
            "#,
        )
        .unwrap();
    assert_eq!(count, 0);
}

#[test]
fn get_num_item_search_results_counts_seeded_entries() {
    let env = env();
    seed_aqirite_bucket(
        &env,
        vec![
            sample_entry(20, 1_000, 5_000),
            sample_entry(40, 1_500, 7_500),
        ],
        true,
    );
    let count: i32 = env
        .eval(
            r#"
            local key = C_AuctionHouse.MakeItemKey(210935, 70)
            return C_AuctionHouse.GetNumItemSearchResults(key)
            "#,
        )
        .unwrap();
    assert_eq!(count, 2);
}

#[test]
fn get_item_search_result_info_returns_seeded_row_fields() {
    let env = env();
    seed_aqirite_bucket(
        &env,
        vec![ItemSearchResultInfo {
            owners: vec!["Alice-Realm".into(), "Bob-Realm".into()],
            time_left: 1,
            auction_id: 9_876_543,
            quantity: 25,
            item_link: "|cffffffff|Hitem:210935|h[Aqirite]|h|r".into(),
            contains_owner_item: true,
            contains_account_item: false,
            contains_socketed_item: false,
            bidder: Some("HighBidder".into()),
            min_bid: 100,
            bid_amount: 500,
            buyout_amount: 9_999,
            time_left_seconds: 90 * 60,
        }],
        true,
    );
    let (
        owner_count,
        first_owner,
        time_left,
        auction_id,
        quantity,
        item_link,
        contains_owner,
        bidder,
        min_bid,
        bid_amount,
        buyout,
        time_left_seconds,
    ): (
        i32,
        String,
        i32,
        i64,
        i32,
        String,
        bool,
        String,
        i64,
        i64,
        i64,
        i64,
    ) = env
        .eval(
            r#"
            local key = C_AuctionHouse.MakeItemKey(210935, 70)
            local info = C_AuctionHouse.GetItemSearchResultInfo(key, 1)
            return #info.owners, info.owners[1], info.timeLeft, info.auctionID,
                   info.quantity, info.itemLink, info.containsOwnerItem,
                   info.bidder, info.minBid, info.bidAmount, info.buyoutAmount,
                   info.timeLeftSeconds
            "#,
        )
        .unwrap();
    assert_eq!(owner_count, 2);
    assert_eq!(first_owner, "Alice-Realm");
    assert_eq!(time_left, 1);
    assert_eq!(auction_id, 9_876_543);
    assert_eq!(quantity, 25);
    assert_eq!(item_link, "|cffffffff|Hitem:210935|h[Aqirite]|h|r");
    assert!(contains_owner);
    assert_eq!(bidder, "HighBidder");
    assert_eq!(min_bid, 100);
    assert_eq!(bid_amount, 500);
    assert_eq!(buyout, 9_999);
    assert_eq!(time_left_seconds, 90 * 60);
}

#[test]
fn get_item_search_result_info_returns_nothing_out_of_range() {
    let env = env();
    seed_aqirite_bucket(&env, vec![sample_entry(5, 100, 200)], true);
    let (high, zero, negative): (i32, i32, i32) = env
        .eval(
            r#"
            local key = C_AuctionHouse.MakeItemKey(210935, 70)
            return select('#', C_AuctionHouse.GetItemSearchResultInfo(key, 99)),
                   select('#', C_AuctionHouse.GetItemSearchResultInfo(key, 0)),
                   select('#', C_AuctionHouse.GetItemSearchResultInfo(key, -1))
            "#,
        )
        .unwrap();
    assert_eq!(high, 0);
    assert_eq!(zero, 0);
    assert_eq!(negative, 0);
}

#[test]
fn get_item_search_result_info_bidder_is_nil_when_no_bid() {
    let env = env();
    seed_aqirite_bucket(&env, vec![sample_entry(1, 0, 1_000)], true);
    let bidder_is_nil: bool = env
        .eval(
            r#"
            local key = C_AuctionHouse.MakeItemKey(210935, 70)
            return C_AuctionHouse.GetItemSearchResultInfo(key, 1).bidder == nil
            "#,
        )
        .unwrap();
    assert!(bidder_is_nil);
}

#[test]
fn get_item_search_results_quantity_sums_entries() {
    let env = env();
    seed_aqirite_bucket(
        &env,
        vec![
            sample_entry(20, 0, 0),
            sample_entry(40, 0, 0),
            sample_entry(7, 0, 0),
        ],
        true,
    );
    let total: i32 = env
        .eval(
            r#"
            local key = C_AuctionHouse.MakeItemKey(210935, 70)
            return C_AuctionHouse.GetItemSearchResultsQuantity(key)
            "#,
        )
        .unwrap();
    assert_eq!(total, 67);
}

#[test]
fn get_item_search_results_quantity_returns_zero_for_unseeded_key() {
    let env = env();
    let total: i32 = env
        .eval(
            r#"
            local key = C_AuctionHouse.MakeItemKey(99999, 70)
            return C_AuctionHouse.GetItemSearchResultsQuantity(key)
            "#,
        )
        .unwrap();
    assert_eq!(total, 0);
}

#[test]
fn has_full_item_search_results_defaults_true_for_unseeded_key() {
    let env = env();
    let full: bool = env
        .eval(
            r#"
            local key = C_AuctionHouse.MakeItemKey(210935, 70)
            return C_AuctionHouse.HasFullItemSearchResults(key)
            "#,
        )
        .unwrap();
    assert!(full);
}

#[test]
fn has_full_item_search_results_reflects_seeded_pagination_flag() {
    let env = env();
    seed_aqirite_bucket(&env, vec![sample_entry(1, 0, 0)], false);
    let full: bool = env
        .eval(
            r#"
            local key = C_AuctionHouse.MakeItemKey(210935, 70)
            return C_AuctionHouse.HasFullItemSearchResults(key)
            "#,
        )
        .unwrap();
    assert!(!full);
}

#[test]
fn get_max_item_search_result_bid_returns_max_over_entries() {
    let env = env();
    seed_aqirite_bucket(
        &env,
        vec![
            sample_entry(1, 100, 0),
            sample_entry(1, 750, 0),
            sample_entry(1, 250, 0),
        ],
        true,
    );
    let max: i64 = env
        .eval(
            r#"
            local key = C_AuctionHouse.MakeItemKey(210935, 70)
            return C_AuctionHouse.GetMaxItemSearchResultBid(key)
            "#,
        )
        .unwrap();
    assert_eq!(max, 750);
}

#[test]
fn get_max_item_search_result_buyout_returns_max_over_entries() {
    let env = env();
    seed_aqirite_bucket(
        &env,
        vec![
            sample_entry(1, 0, 5_000),
            sample_entry(1, 0, 9_999),
            sample_entry(1, 0, 1_000),
        ],
        true,
    );
    let max: i64 = env
        .eval(
            r#"
            local key = C_AuctionHouse.MakeItemKey(210935, 70)
            return C_AuctionHouse.GetMaxItemSearchResultBuyout(key)
            "#,
        )
        .unwrap();
    assert_eq!(max, 9_999);
}

#[test]
fn get_max_item_search_result_bid_is_zero_for_unseeded_key() {
    let env = env();
    let max: i64 = env
        .eval(
            r#"
            local key = C_AuctionHouse.MakeItemKey(99999, 70)
            return C_AuctionHouse.GetMaxItemSearchResultBid(key)
            "#,
        )
        .unwrap();
    assert_eq!(max, 0);
}

#[test]
fn refresh_item_search_results_fires_event_with_item_key() {
    let env = env();
    seed_aqirite_bucket(&env, vec![sample_entry(1, 0, 1_000)], true);
    let (fire_count, fired_item_id): (i32, i32) = env
        .eval(
            r#"
            local key = C_AuctionHouse.MakeItemKey(210935, 70)
            local count, lastID = 0, 0
            local listener = CreateFrame("Frame")
            listener:RegisterEvent("ITEM_SEARCH_RESULTS_UPDATED")
            listener:SetScript("OnEvent", function(_, _, payload)
                count = count + 1
                if payload then lastID = payload.itemID end
            end)
            C_AuctionHouse.RefreshItemSearchResults(key)
            return count, lastID
            "#,
        )
        .unwrap();
    assert_eq!(fire_count, 1);
    assert_eq!(fired_item_id, 210935);
}

#[test]
fn request_more_item_search_results_returns_false_when_full() {
    let env = env();
    seed_aqirite_bucket(&env, vec![sample_entry(1, 0, 1_000)], true);
    let result: bool = env
        .eval(
            r#"
            local key = C_AuctionHouse.MakeItemKey(210935, 70)
            return C_AuctionHouse.RequestMoreItemSearchResults(key)
            "#,
        )
        .unwrap();
    assert!(!result, "already-full bucket has no more pages to load");
}

#[test]
fn request_more_item_search_results_returns_true_when_not_full() {
    let env = env();
    seed_aqirite_bucket(&env, vec![sample_entry(1, 0, 1_000)], false);
    let result: bool = env
        .eval(
            r#"
            local key = C_AuctionHouse.MakeItemKey(210935, 70)
            return C_AuctionHouse.RequestMoreItemSearchResults(key)
            "#,
        )
        .unwrap();
    assert!(result, "partial bucket signals more pages are available");
}

#[test]
fn request_more_item_search_results_fires_event() {
    let env = env();
    seed_aqirite_bucket(&env, vec![sample_entry(1, 0, 1_000)], false);
    let fire_count: i32 = env
        .eval(
            r#"
            local key = C_AuctionHouse.MakeItemKey(210935, 70)
            local count = 0
            local listener = CreateFrame("Frame")
            listener:RegisterEvent("ITEM_SEARCH_RESULTS_UPDATED")
            listener:SetScript("OnEvent", function() count = count + 1 end)
            C_AuctionHouse.RequestMoreItemSearchResults(key)
            return count
            "#,
        )
        .unwrap();
    assert_eq!(fire_count, 1);
}

#[test]
fn has_search_results_returns_false_for_unseeded_key() {
    let env = env();
    let has: bool = env
        .eval(
            r#"
            local key = C_AuctionHouse.MakeItemKey(99999, 70)
            return C_AuctionHouse.HasSearchResults(key)
            "#,
        )
        .unwrap();
    assert!(!has);
}

#[test]
fn has_search_results_returns_false_for_empty_bucket() {
    let env = env();
    seed_aqirite_bucket(&env, vec![], true);
    let has: bool = env
        .eval(
            r#"
            local key = C_AuctionHouse.MakeItemKey(210935, 70)
            return C_AuctionHouse.HasSearchResults(key)
            "#,
        )
        .unwrap();
    assert!(!has);
}

#[test]
fn has_search_results_returns_true_when_entries_present() {
    let env = env();
    seed_aqirite_bucket(&env, vec![sample_entry(1, 0, 100)], true);
    let has: bool = env
        .eval(
            r#"
            local key = C_AuctionHouse.MakeItemKey(210935, 70)
            return C_AuctionHouse.HasSearchResults(key)
            "#,
        )
        .unwrap();
    assert!(has);
}

#[test]
fn item_search_lookups_use_full_4_tuple_key() {
    // Two entries sharing itemID but different itemLevel must not bleed
    // into each other's bucket — the key is the full 4-tuple.
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.auction_item_searches.insert(
            (210935, 70, 0, 0),
            ItemSearchResults {
                entries: vec![sample_entry(10, 0, 1_000)],
                has_full_results: true,
            },
        );
        state.auction_item_searches.insert(
            (210935, 100, 0, 0),
            ItemSearchResults {
                entries: vec![sample_entry(99, 0, 5_000)],
                has_full_results: true,
            },
        );
    }
    let (low_qty, high_qty): (i32, i32) = env
        .eval(
            r#"
            local lowKey  = C_AuctionHouse.MakeItemKey(210935, 70)
            local highKey = C_AuctionHouse.MakeItemKey(210935, 100)
            return C_AuctionHouse.GetItemSearchResultsQuantity(lowKey),
                   C_AuctionHouse.GetItemSearchResultsQuantity(highKey)
            "#,
        )
        .unwrap();
    assert_eq!(low_qty, 10);
    assert_eq!(high_qty, 99);
}
