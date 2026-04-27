//! Tests for `C_AuctionHouse` browse + search query messages. Covers
//! the four entry points: `SendBrowseQuery`, `SendSearchQuery`,
//! `SendSellSearchQuery`, `SearchForFavorites`. Each verifies the
//! correct event dispatch (`AUCTION_HOUSE_BROWSE_RESULTS_UPDATED`,
//! `ITEM_SEARCH_RESULTS_UPDATED`, `AUCTION_HOUSE_BROWSE_FAILURE`, or
//! `AUCTION_HOUSE_THROTTLED_MESSAGE_QUEUED`) and exercises the
//! throttle gate via `state.auction_throttle_ready = false`.

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::{ItemSearchKey, ItemSearchResultInfo, ItemSearchResults};

const AQIRITE_KEY: ItemSearchKey = (210935, 70, 0, 0);

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

fn sample_item_search_entry() -> ItemSearchResultInfo {
    ItemSearchResultInfo {
        owners: vec!["Tester-Realm".into()],
        time_left: 2,
        auction_id: 7_654_321,
        quantity: 1,
        item_link: "|cffffffff|Hitem:210935::::::::70:::::|h[Aqirite]|h|r".into(),
        contains_owner_item: false,
        contains_account_item: false,
        contains_socketed_item: false,
        bidder: None,
        min_bid: 100,
        bid_amount: 100,
        buyout_amount: 200,
        time_left_seconds: 4 * 60 * 60,
    }
}

fn seed_buyer_bucket(env: &WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    state.auction_item_searches.insert(
        AQIRITE_KEY,
        ItemSearchResults {
            entries: vec![sample_item_search_entry()],
            has_full_results: true,
        },
    );
}

fn seed_seller_bucket(env: &WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    state.auction_sell_search_results.insert(
        AQIRITE_KEY,
        ItemSearchResults {
            entries: vec![sample_item_search_entry()],
            has_full_results: true,
        },
    );
}

fn close_throttle_gate(env: &WowLuaEnv) {
    env.state().borrow_mut().auction_throttle_ready = false;
}

const COUNT_EVENT_LISTENER: &str = r#"
    local count = 0
    local listener = CreateFrame("Frame")
    listener:RegisterEvent(EVENT_NAME)
    listener:SetScript("OnEvent", function() count = count + 1 end)
"#;

fn count_listener_for(event_name: &str) -> String {
    COUNT_EVENT_LISTENER.replace("EVENT_NAME", &format!("\"{}\"", event_name))
}

#[test]
fn send_browse_query_stores_query_and_fires_browse_results_updated() {
    let env = env();
    let script = format!(
        r#"
        {listener}
        C_AuctionHouse.SendBrowseQuery({{
            searchString = "aqirite",
            sorts = {{
                {{ sortOrder = 4, reverseSort = false }},
                {{ sortOrder = 1, reverseSort = true }},
            }},
            minLevel = 60,
            maxLevel = 80,
            filters = {{ 1, 7 }},
            itemClassFilters = {{
                {{ classID = 7, subClassID = 12 }},
            }},
        }})
        return count
        "#,
        listener = count_listener_for("AUCTION_HOUSE_BROWSE_RESULTS_UPDATED"),
    );
    let fire_count: i32 = env.eval(&script).unwrap();
    assert_eq!(fire_count, 1);

    let state = env.state().borrow();
    let query = state
        .auction_last_browse_query
        .as_ref()
        .expect("query should be stored");
    assert_eq!(query.search_string, "aqirite");
    assert_eq!(query.min_level, Some(60));
    assert_eq!(query.max_level, Some(80));
    assert_eq!(query.sorts.len(), 2);
    assert_eq!(query.sorts[0].sort_order, 4);
    assert!(!query.sorts[0].reverse_sort);
    assert_eq!(query.sorts[1].sort_order, 1);
    assert!(query.sorts[1].reverse_sort);
    assert_eq!(query.filters, vec![1, 7]);
    assert_eq!(query.item_class_filters.len(), 1);
    assert_eq!(query.item_class_filters[0].class_id, 7);
    assert_eq!(query.item_class_filters[0].sub_class_id, Some(12));
    assert_eq!(query.item_class_filters[0].inventory_type, None);
}

#[test]
fn send_browse_query_omits_optional_fields_as_none() {
    let env = env();
    env.eval::<()>(
        r#"
        C_AuctionHouse.SendBrowseQuery({
            searchString = "",
            sorts = {},
        })
        "#,
    )
    .unwrap();
    let state = env.state().borrow();
    let query = state.auction_last_browse_query.as_ref().unwrap();
    assert_eq!(query.search_string, "");
    assert!(query.sorts.is_empty());
    assert_eq!(query.min_level, None);
    assert_eq!(query.max_level, None);
    assert!(query.filters.is_empty());
    assert!(query.item_class_filters.is_empty());
}

#[test]
fn send_browse_query_dispatches_throttle_event_when_gate_closed() {
    let env = env();
    close_throttle_gate(&env);
    let script = format!(
        r#"
        {browse_listener}
        local throttle_count = 0
        local throttle_listener = CreateFrame("Frame")
        throttle_listener:RegisterEvent("AUCTION_HOUSE_THROTTLED_MESSAGE_QUEUED")
        throttle_listener:SetScript("OnEvent", function() throttle_count = throttle_count + 1 end)
        C_AuctionHouse.SendBrowseQuery({{ searchString = "x", sorts = {{}} }})
        return count, throttle_count
        "#,
        browse_listener = count_listener_for("AUCTION_HOUSE_BROWSE_RESULTS_UPDATED"),
    );
    let (browse_count, throttle_count): (i32, i32) = env.eval(&script).unwrap();
    assert_eq!(browse_count, 0, "throttled query should not fire results");
    assert_eq!(throttle_count, 1);
    assert!(
        env.state().borrow().auction_last_browse_query.is_none(),
        "throttled query must not be stored"
    );
}

#[test]
fn send_search_query_fires_item_search_results_when_bucket_seeded() {
    let env = env();
    seed_buyer_bucket(&env);
    let script = format!(
        r#"
        local count, last_id = 0, 0
        local listener = CreateFrame("Frame")
        listener:RegisterEvent("ITEM_SEARCH_RESULTS_UPDATED")
        listener:SetScript("OnEvent", function(_, _, payload)
            count = count + 1
            if type(payload) == "table" then last_id = payload.itemID end
        end)
        local key = C_AuctionHouse.MakeItemKey(210935, 70)
        C_AuctionHouse.SendSearchQuery(key, {{}}, false)
        return count, last_id
        "#,
    );
    let (fire_count, item_id): (i32, i32) = env.eval(&script).unwrap();
    assert_eq!(fire_count, 1);
    assert_eq!(item_id, 210935);
}

#[test]
fn send_search_query_fires_browse_failure_when_bucket_missing() {
    let env = env();
    let script = format!(
        r#"
        {failure_listener}
        local key = C_AuctionHouse.MakeItemKey(99999, 70)
        C_AuctionHouse.SendSearchQuery(key, {{}}, false)
        return count
        "#,
        failure_listener = count_listener_for("AUCTION_HOUSE_BROWSE_FAILURE"),
    );
    let fire_count: i32 = env.eval(&script).unwrap();
    assert_eq!(fire_count, 1);
}

#[test]
fn send_search_query_dispatches_throttle_event_when_gate_closed() {
    let env = env();
    seed_buyer_bucket(&env);
    close_throttle_gate(&env);
    let script = format!(
        r#"
        local item_count, throttle_count = 0, 0
        local item_listener = CreateFrame("Frame")
        item_listener:RegisterEvent("ITEM_SEARCH_RESULTS_UPDATED")
        item_listener:SetScript("OnEvent", function() item_count = item_count + 1 end)
        local throttle_listener = CreateFrame("Frame")
        throttle_listener:RegisterEvent("AUCTION_HOUSE_THROTTLED_MESSAGE_QUEUED")
        throttle_listener:SetScript("OnEvent", function() throttle_count = throttle_count + 1 end)
        local key = C_AuctionHouse.MakeItemKey(210935, 70)
        C_AuctionHouse.SendSearchQuery(key, {{}}, false)
        return item_count, throttle_count
        "#,
    );
    let (item_count, throttle_count): (i32, i32) = env.eval(&script).unwrap();
    assert_eq!(item_count, 0);
    assert_eq!(throttle_count, 1);
}

#[test]
fn send_sell_search_query_reads_separate_seller_bucket() {
    // Buyer cache MUST NOT satisfy a sell-side query — the seller cache
    // is intentionally separate so the sell tab's "what is currently
    // listed" view stays decoupled from the buyer browse cache.
    let env = env();
    seed_buyer_bucket(&env);
    let script_seller_only_in_buyer_cache = r#"
        local item_count, failure_count = 0, 0
        local item_listener = CreateFrame("Frame")
        item_listener:RegisterEvent("ITEM_SEARCH_RESULTS_UPDATED")
        item_listener:SetScript("OnEvent", function() item_count = item_count + 1 end)
        local failure_listener = CreateFrame("Frame")
        failure_listener:RegisterEvent("AUCTION_HOUSE_BROWSE_FAILURE")
        failure_listener:SetScript("OnEvent", function() failure_count = failure_count + 1 end)
        local key = C_AuctionHouse.MakeItemKey(210935, 70)
        C_AuctionHouse.SendSellSearchQuery(key, {}, false)
        return item_count, failure_count
    "#;
    let (item_count, failure_count): (i32, i32) =
        env.eval(script_seller_only_in_buyer_cache).unwrap();
    assert_eq!(item_count, 0);
    assert_eq!(failure_count, 1);
}

#[test]
fn send_sell_search_query_succeeds_when_seller_bucket_seeded() {
    let env = env();
    seed_seller_bucket(&env);
    let script = r#"
        local count, last_id = 0, 0
        local listener = CreateFrame("Frame")
        listener:RegisterEvent("ITEM_SEARCH_RESULTS_UPDATED")
        listener:SetScript("OnEvent", function(_, _, payload)
            count = count + 1
            if type(payload) == "table" then last_id = payload.itemID end
        end)
        local key = C_AuctionHouse.MakeItemKey(210935, 70)
        C_AuctionHouse.SendSellSearchQuery(key, {}, false)
        return count, last_id
    "#;
    let (fire_count, item_id): (i32, i32) = env.eval(script).unwrap();
    assert_eq!(fire_count, 1);
    assert_eq!(item_id, 210935);
}

#[test]
fn search_for_favorites_fires_browse_results_updated() {
    let env = env();
    let script = format!(
        r#"
        {listener}
        C_AuctionHouse.SearchForFavorites({{}})
        return count
        "#,
        listener = count_listener_for("AUCTION_HOUSE_BROWSE_RESULTS_UPDATED"),
    );
    let fire_count: i32 = env.eval(&script).unwrap();
    assert_eq!(fire_count, 1);
}

#[test]
fn search_for_favorites_dispatches_throttle_event_when_gate_closed() {
    let env = env();
    close_throttle_gate(&env);
    let script = format!(
        r#"
        {browse_listener}
        local throttle_count = 0
        local throttle_listener = CreateFrame("Frame")
        throttle_listener:RegisterEvent("AUCTION_HOUSE_THROTTLED_MESSAGE_QUEUED")
        throttle_listener:SetScript("OnEvent", function() throttle_count = throttle_count + 1 end)
        C_AuctionHouse.SearchForFavorites({{}})
        return count, throttle_count
        "#,
        browse_listener = count_listener_for("AUCTION_HOUSE_BROWSE_RESULTS_UPDATED"),
    );
    let (browse_count, throttle_count): (i32, i32) = env.eval(&script).unwrap();
    assert_eq!(browse_count, 0);
    assert_eq!(throttle_count, 1);
}
