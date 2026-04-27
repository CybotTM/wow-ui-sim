//! Tests for `C_AuctionHouse` per-`itemID` commodity-search-results
//! state machine. Commodities are stack-only items so the bucket is
//! keyed by itemID alone (vs. items, which use the 4-tuple ItemKey).

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::{CommoditySearchResultInfo, CommoditySearchResults};

const AQIRITE_ITEM_ID: i32 = 210935;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

fn sample_entry(quantity: i32, unit_price: i64) -> CommoditySearchResultInfo {
    CommoditySearchResultInfo {
        item_id: AQIRITE_ITEM_ID,
        quantity,
        unit_price,
        auction_id: 1_111_111,
        owners: vec!["Tester-Realm".into()],
        time_left_seconds: 4 * 60 * 60,
        num_owner_items: 0,
        contains_owner_item: false,
        contains_account_item: false,
    }
}

fn seed_aqirite_bucket(env: &WowLuaEnv, entries: Vec<CommoditySearchResultInfo>, has_full: bool) {
    let mut state = env.state().borrow_mut();
    state.auction_commodity_searches.insert(
        AQIRITE_ITEM_ID,
        CommoditySearchResults {
            entries,
            has_full_results: has_full,
        },
    );
}

#[test]
fn get_num_commodity_search_results_returns_zero_for_unseeded_item() {
    let env = env();
    let count: i32 = env
        .eval("return C_AuctionHouse.GetNumCommoditySearchResults(210935)")
        .unwrap();
    assert_eq!(count, 0);
}

#[test]
fn get_num_commodity_search_results_counts_seeded_entries() {
    let env = env();
    seed_aqirite_bucket(
        &env,
        vec![
            sample_entry(20, 100),
            sample_entry(40, 110),
            sample_entry(7, 125),
        ],
        true,
    );
    let count: i32 = env
        .eval("return C_AuctionHouse.GetNumCommoditySearchResults(210935)")
        .unwrap();
    assert_eq!(count, 3);
}

#[test]
fn get_commodity_search_result_info_returns_seeded_row_fields() {
    let env = env();
    seed_aqirite_bucket(
        &env,
        vec![CommoditySearchResultInfo {
            item_id: AQIRITE_ITEM_ID,
            quantity: 25,
            unit_price: 9_999,
            auction_id: 7_654_321,
            owners: vec!["Alice-Realm".into(), "Bob-Realm".into()],
            time_left_seconds: 90 * 60,
            num_owner_items: 5,
            contains_owner_item: true,
            contains_account_item: false,
        }],
        true,
    );
    let (
        item_id,
        quantity,
        unit_price,
        auction_id,
        owner_count,
        first_owner,
        time_left_seconds,
        num_owner_items,
        contains_owner,
    ): (i32, i32, i64, i64, i32, String, i64, i32, bool) = env
        .eval(
            r#"
            local info = C_AuctionHouse.GetCommoditySearchResultInfo(210935, 1)
            return info.itemID, info.quantity, info.unitPrice, info.auctionID,
                   #info.owners, info.owners[1], info.timeLeftSeconds,
                   info.numOwnerItems, info.containsOwnerItem
            "#,
        )
        .unwrap();
    assert_eq!(item_id, AQIRITE_ITEM_ID);
    assert_eq!(quantity, 25);
    assert_eq!(unit_price, 9_999);
    assert_eq!(auction_id, 7_654_321);
    assert_eq!(owner_count, 2);
    assert_eq!(first_owner, "Alice-Realm");
    assert_eq!(time_left_seconds, 90 * 60);
    assert_eq!(num_owner_items, 5);
    assert!(contains_owner);
}

#[test]
fn get_commodity_search_result_info_returns_nothing_out_of_range() {
    let env = env();
    seed_aqirite_bucket(&env, vec![sample_entry(5, 100)], true);
    let (high, zero, negative): (i32, i32, i32) = env
        .eval(
            r#"
            return select('#', C_AuctionHouse.GetCommoditySearchResultInfo(210935, 99)),
                   select('#', C_AuctionHouse.GetCommoditySearchResultInfo(210935, 0)),
                   select('#', C_AuctionHouse.GetCommoditySearchResultInfo(210935, -1))
            "#,
        )
        .unwrap();
    assert_eq!(high, 0);
    assert_eq!(zero, 0);
    assert_eq!(negative, 0);
}

#[test]
fn get_commodity_search_result_info_returns_nothing_for_unseeded_item() {
    let env = env();
    let nothing: i32 = env
        .eval("return select('#', C_AuctionHouse.GetCommoditySearchResultInfo(99999, 1))")
        .unwrap();
    assert_eq!(nothing, 0);
}

#[test]
fn get_commodity_search_results_quantity_sums_entries() {
    let env = env();
    seed_aqirite_bucket(
        &env,
        vec![
            sample_entry(20, 100),
            sample_entry(40, 110),
            sample_entry(7, 125),
        ],
        true,
    );
    let total: i32 = env
        .eval("return C_AuctionHouse.GetCommoditySearchResultsQuantity(210935)")
        .unwrap();
    assert_eq!(total, 67);
}

#[test]
fn get_commodity_search_results_quantity_returns_zero_for_unseeded_item() {
    let env = env();
    let total: i32 = env
        .eval("return C_AuctionHouse.GetCommoditySearchResultsQuantity(99999)")
        .unwrap();
    assert_eq!(total, 0);
}

#[test]
fn has_full_commodity_search_results_defaults_true_for_unseeded_item() {
    let env = env();
    let full: bool = env
        .eval("return C_AuctionHouse.HasFullCommoditySearchResults(210935)")
        .unwrap();
    assert!(full);
}

#[test]
fn has_full_commodity_search_results_reflects_seeded_pagination_flag() {
    let env = env();
    seed_aqirite_bucket(&env, vec![sample_entry(1, 100)], false);
    let full: bool = env
        .eval("return C_AuctionHouse.HasFullCommoditySearchResults(210935)")
        .unwrap();
    assert!(!full);
}

#[test]
fn get_max_commodity_search_result_price_returns_max_over_entries() {
    let env = env();
    seed_aqirite_bucket(
        &env,
        vec![
            sample_entry(1, 100),
            sample_entry(1, 750),
            sample_entry(1, 250),
        ],
        true,
    );
    let max: i64 = env
        .eval("return C_AuctionHouse.GetMaxCommoditySearchResultPrice(210935)")
        .unwrap();
    assert_eq!(max, 750);
}

#[test]
fn get_max_commodity_search_result_price_is_zero_for_unseeded_item() {
    let env = env();
    let max: i64 = env
        .eval("return C_AuctionHouse.GetMaxCommoditySearchResultPrice(99999)")
        .unwrap();
    assert_eq!(max, 0);
}

#[test]
fn refresh_commodity_search_results_fires_event_with_item_id() {
    let env = env();
    seed_aqirite_bucket(&env, vec![sample_entry(1, 100)], true);
    let (fire_count, fired_item_id): (i32, i32) = env
        .eval(
            r#"
            local count, lastID = 0, 0
            local listener = CreateFrame("Frame")
            listener:RegisterEvent("COMMODITY_SEARCH_RESULTS_UPDATED")
            listener:SetScript("OnEvent", function(_, _, payload)
                count = count + 1
                if payload then lastID = payload end
            end)
            C_AuctionHouse.RefreshCommoditySearchResults(210935)
            return count, lastID
            "#,
        )
        .unwrap();
    assert_eq!(fire_count, 1);
    assert_eq!(fired_item_id, AQIRITE_ITEM_ID);
}

#[test]
fn request_more_commodity_search_results_returns_false_when_full() {
    let env = env();
    seed_aqirite_bucket(&env, vec![sample_entry(1, 100)], true);
    let result: bool = env
        .eval("return C_AuctionHouse.RequestMoreCommoditySearchResults(210935)")
        .unwrap();
    assert!(!result, "already-full bucket has no more pages to load");
}

#[test]
fn request_more_commodity_search_results_returns_true_when_not_full() {
    let env = env();
    seed_aqirite_bucket(&env, vec![sample_entry(1, 100)], false);
    let result: bool = env
        .eval("return C_AuctionHouse.RequestMoreCommoditySearchResults(210935)")
        .unwrap();
    assert!(result, "partial bucket signals more pages are available");
}

#[test]
fn request_more_commodity_search_results_returns_false_for_unseeded_item() {
    let env = env();
    let result: bool = env
        .eval("return C_AuctionHouse.RequestMoreCommoditySearchResults(99999)")
        .unwrap();
    assert!(
        !result,
        "unseeded item is treated as already-full so addons skip pointless paging"
    );
}

#[test]
fn commodity_lookups_isolate_buckets_by_item_id() {
    // Two different items in the bucket map must not bleed quantities
    // into each other.
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.auction_commodity_searches.insert(
            210935,
            CommoditySearchResults {
                entries: vec![sample_entry(10, 100)],
                has_full_results: true,
            },
        );
        state.auction_commodity_searches.insert(
            999_999,
            CommoditySearchResults {
                entries: vec![sample_entry(99, 9999)],
                has_full_results: true,
            },
        );
    }
    let (low_qty, high_qty): (i32, i32) = env
        .eval(
            r#"
            return C_AuctionHouse.GetCommoditySearchResultsQuantity(210935),
                   C_AuctionHouse.GetCommoditySearchResultsQuantity(999999)
            "#,
        )
        .unwrap();
    assert_eq!(low_qty, 10);
    assert_eq!(high_qty, 99);
}
