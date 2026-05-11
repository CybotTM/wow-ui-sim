//! Tests for `C_AuctionHouse` probes backed by
//! `SimState.auction_browse_results` + `auction_replicate_items`.

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::{AuctionBrowseResult, AuctionReplicateItem, OwnedAuction};

const AQIRITE_ITEM_ID: i32 = 210935;
const COMMODITY_STATUS_UNKNOWN: i32 = 0;
const COMMODITY_STATUS_ITEM: i32 = 1;
const COMMODITY_STATUS_COMMODITY: i32 = 2;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn get_auction_item_sub_classes_returns_standard_ranges() {
    let env = env();
    let (consumable_count, armor_count, tradegoods_count, unknown_count): (i32, i32, i32, i32) =
        env.eval(
            r#"
            return #C_AuctionHouse.GetAuctionItemSubClasses(0),
                   #C_AuctionHouse.GetAuctionItemSubClasses(4),
                   #C_AuctionHouse.GetAuctionItemSubClasses(7),
                   #C_AuctionHouse.GetAuctionItemSubClasses(999)
            "#,
        )
        .unwrap();
    assert_eq!(consumable_count, 12);
    assert_eq!(armor_count, 12);
    assert_eq!(tradegoods_count, 21);
    assert_eq!(unknown_count, 0, "unknown class ids yield empty arrays");
}

#[test]
fn get_auction_item_sub_classes_returns_zero_based_ids() {
    let env = env();
    let (first, last): (i32, i32) = env
        .eval(
            r#"
            local subs = C_AuctionHouse.GetAuctionItemSubClasses(4)
            return subs[1], subs[#subs]
            "#,
        )
        .unwrap();
    // The array is Lua-indexed (1..N) but carries the retail subclass
    // ids starting at 0.
    assert_eq!(first, 0);
    assert_eq!(last, 11);
}

#[test]
fn get_replicate_item_info_returns_first_seeded_row() {
    let env = env();
    let (name, texture, count, quality, usable, level, level_type): (
        String,
        i32,
        i32,
        i32,
        bool,
        i32,
        String,
    ) = env
        .eval("return C_AuctionHouse.GetReplicateItemInfo(0)")
        .unwrap();
    assert_eq!(name, "Aqirite");
    assert_eq!(texture, 0);
    assert_eq!(count, 20);
    assert_eq!(quality, 2);
    assert!(usable);
    assert_eq!(level, 70);
    assert_eq!(level_type, "Item Level");
}

#[test]
fn get_replicate_item_info_supports_multiple_rows() {
    let env = env();
    let second_name: String = env
        .eval("return (C_AuctionHouse.GetReplicateItemInfo(1))")
        .unwrap();
    assert_eq!(second_name, "Burnished Helm of Might");
}

#[test]
fn get_replicate_item_info_returns_nothing_out_of_range() {
    let env = env();
    let (high, negative): (i32, i32) = env
        .eval(
            r#"
            return select('#', C_AuctionHouse.GetReplicateItemInfo(99)),
                   select('#', C_AuctionHouse.GetReplicateItemInfo(-1))
            "#,
        )
        .unwrap();
    assert_eq!(high, 0);
    assert_eq!(negative, 0);
}

#[test]
fn get_browse_results_returns_seeded_rows_with_retail_fields() {
    let env = env();
    let (count, first_item_id, first_min_price, first_qty, second_owner): (
        i32,
        i32,
        i64,
        i32,
        bool,
    ) = env
        .eval(
            r#"
            local rows = C_AuctionHouse.GetBrowseResults()
            return #rows,
                   rows[1].itemKey.itemID,
                   rows[1].minPrice,
                   rows[1].totalQuantity,
                   rows[2].containsOwnerItem
            "#,
        )
        .unwrap();
    assert_eq!(count, 2);
    assert_eq!(first_item_id, 210935, "Aqirite");
    assert_eq!(first_min_price, 25_000);
    assert_eq!(first_qty, 400);
    assert!(second_owner, "Burnished Helm is the owner's own listing");
}

#[test]
fn get_browse_results_item_key_carries_zeroed_defaults() {
    let env = env();
    let (suffix, battle_pet): (i32, i32) = env
        .eval(
            r#"
            local rows = C_AuctionHouse.GetBrowseResults()
            return rows[1].itemKey.itemSuffix, rows[1].itemKey.battlePetSpeciesID
            "#,
        )
        .unwrap();
    assert_eq!(suffix, 0);
    assert_eq!(battle_pet, 0);
}

#[test]
fn get_browse_results_appearance_link_is_nil() {
    let env = env();
    let is_nil: bool = env
        .eval("return C_AuctionHouse.GetBrowseResults()[1].appearanceLink == nil")
        .unwrap();
    assert!(is_nil);
}

#[test]
fn browse_results_reflect_sim_state_mutation() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.auction_browse_results.clear();
        state.auction_browse_results.push(AuctionBrowseResult {
            auction_id: Some(99),
            item_id: 12345,
            item_level: 70,
            min_price: 99,
            total_quantity: 7,
            contains_owner_item: false,
        });
    }
    let (count, item_id, min_price): (i32, i32, i64) = env
        .eval(
            r#"
            local rows = C_AuctionHouse.GetBrowseResults()
            return #rows, rows[1].itemKey.itemID, rows[1].minPrice
            "#,
        )
        .unwrap();
    assert_eq!(count, 1);
    assert_eq!(item_id, 12345);
    assert_eq!(min_price, 99);
}

#[test]
fn replicate_items_reflect_sim_state_mutation() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.auction_replicate_items.clear();
        state.auction_replicate_items.push(AuctionReplicateItem {
            name: "Custom Stone".into(),
            texture: 999,
            count: 3,
            quality_id: 4,
            usable: false,
            level: 50,
            level_type: "Item Level".into(),
        });
    }
    let (name, texture, quality, usable): (String, i32, i32, bool) = env
        .eval(
            r#"
            local n, t, c, q, u, l, lt = C_AuctionHouse.GetReplicateItemInfo(0)
            return n, t, q, u
            "#,
        )
        .unwrap();
    assert_eq!(name, "Custom Stone");
    assert_eq!(texture, 999);
    assert_eq!(quality, 4);
    assert!(!usable);
}

#[test]
fn make_item_key_returns_canonical_4_field_table() {
    let env = env();
    let (item_id, item_level, suffix, species): (i32, i32, i32, i32) = env
        .eval(
            r#"
            local key = C_AuctionHouse.MakeItemKey(12345, 70, 99, 42)
            return key.itemID, key.itemLevel, key.itemSuffix, key.battlePetSpeciesID
            "#,
        )
        .unwrap();
    assert_eq!(item_id, 12345);
    assert_eq!(item_level, 70);
    assert_eq!(suffix, 99);
    assert_eq!(species, 42);
}

#[test]
fn make_item_key_zeroes_optional_args_when_omitted() {
    let env = env();
    let (item_id, item_level, suffix, species): (i32, i32, i32, i32) = env
        .eval(
            r#"
            local key = C_AuctionHouse.MakeItemKey(7777)
            return key.itemID, key.itemLevel, key.itemSuffix, key.battlePetSpeciesID
            "#,
        )
        .unwrap();
    assert_eq!(item_id, 7777);
    assert_eq!(item_level, 0);
    assert_eq!(suffix, 0);
    assert_eq!(species, 0);
}

#[test]
fn get_item_key_info_resolves_make_item_key_result() {
    let env = env();
    let (name, icon, quality): (String, i32, i32) = env
        .eval(
            r#"
            local key = C_AuctionHouse.MakeItemKey(210935, 70)
            local info = C_AuctionHouse.GetItemKeyInfo(key)
            return info.itemName, info.iconFileID, info.quality
            "#,
        )
        .unwrap();
    assert_eq!(name, "Aqirite");
    assert!(icon > 0, "iconFileID should be populated");
    assert!(quality >= 0, "quality should be populated");
}

#[test]
fn get_item_key_from_item_resolves_item_id_to_full_key() {
    let env = env();
    let (item_id, item_level): (i32, i32) = env
        .eval(
            r#"
            local key = C_AuctionHouse.GetItemKeyFromItem({ itemID = 210935 })
            return key.itemID, key.itemLevel
            "#,
        )
        .unwrap();
    assert_eq!(item_id, AQIRITE_ITEM_ID);
    assert_eq!(item_level, 70, "itemLevel filled in from items DB");
}

#[test]
fn get_item_key_from_item_returns_nil_when_location_missing_id() {
    let env = env();
    let is_nil: bool = env
        .eval("return C_AuctionHouse.GetItemKeyFromItem({}) == nil")
        .unwrap();
    assert!(is_nil);
}

#[test]
fn get_time_left_band_info_returns_min_max_seconds_per_band() {
    let env = env();
    let (short_min, short_max, medium_min, medium_max, long_max, very_long_max): (
        i64,
        i64,
        i64,
        i64,
        i64,
        i64,
    ) = env
        .eval(
            r#"
            local sMin, sMax = C_AuctionHouse.GetTimeLeftBandInfo(0)
            local mMin, mMax = C_AuctionHouse.GetTimeLeftBandInfo(1)
            local _, lMax = C_AuctionHouse.GetTimeLeftBandInfo(2)
            local _, vMax = C_AuctionHouse.GetTimeLeftBandInfo(3)
            return sMin, sMax, mMin, mMax, lMax, vMax
            "#,
        )
        .unwrap();
    assert_eq!(short_min, 0);
    assert_eq!(short_max, 30 * 60);
    assert_eq!(medium_min, 30 * 60);
    assert_eq!(medium_max, 2 * 60 * 60);
    assert_eq!(long_max, 12 * 60 * 60);
    assert_eq!(very_long_max, 48 * 60 * 60);
}

#[test]
fn get_time_left_band_info_returns_nothing_for_unknown_band() {
    let env = env();
    let count: i32 = env
        .eval("return select('#', C_AuctionHouse.GetTimeLeftBandInfo(99))")
        .unwrap();
    assert_eq!(count, 0);
}

#[test]
fn is_throttled_message_system_ready_reflects_state_flag() {
    let env = env();
    let default_ready: bool = env
        .eval("return C_AuctionHouse.IsThrottledMessageSystemReady()")
        .unwrap();
    assert!(default_ready, "default sim state is throttle-ready");

    {
        let mut state = env.state().borrow_mut();
        state.auction_throttle_ready = false;
    }
    let throttled: bool = env
        .eval("return C_AuctionHouse.IsThrottledMessageSystemReady()")
        .unwrap();
    assert!(!throttled, "flips to false after state mutation");
}

#[test]
fn should_auto_populate_price_reflects_state_flag() {
    let env = env();
    let default_auto: bool = env
        .eval("return C_AuctionHouse.ShouldAutoPopulatePrice()")
        .unwrap();
    assert!(default_auto);

    {
        let mut state = env.state().borrow_mut();
        state.auction_should_auto_populate_price = false;
    }
    let disabled: bool = env
        .eval("return C_AuctionHouse.ShouldAutoPopulatePrice()")
        .unwrap();
    assert!(!disabled);
}

#[test]
fn is_sell_item_valid_blocks_soulbound_items() {
    let env = env();
    // Hearthstone (6948): bonding=1 (BoP) → not sellable.
    let bop_valid: bool = env
        .eval("return C_AuctionHouse.IsSellItemValid({ itemID = 6948 })")
        .unwrap();
    assert!(!bop_valid);

    // Aqirite: bonding=0 → sellable.
    let commodity_valid: bool = env
        .eval("return C_AuctionHouse.IsSellItemValid({ itemID = 210935 })")
        .unwrap();
    assert!(commodity_valid);
}

#[test]
fn is_sell_item_valid_returns_false_for_missing_location() {
    let env = env();
    let valid: bool = env
        .eval("return C_AuctionHouse.IsSellItemValid({})")
        .unwrap();
    assert!(!valid);
}

#[test]
fn get_cancel_cost_returns_5_percent_of_buyout() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.auction_owned.push(OwnedAuction {
            auction_id: 4242,
            item_id: AQIRITE_ITEM_ID,
            item_level: 70,
            quantity: 5,
            bid_amount: 1_000,
            buyout_amount: 100_000,
            status: 0,
            time_left: 3,
            time_left_seconds: 12 * 3600,
        });
    }
    let cost: i64 = env
        .eval("return C_AuctionHouse.GetCancelCost(4242)")
        .unwrap();
    assert_eq!(cost, 5_000, "5% of 100_000 buyout");
}

#[test]
fn get_cancel_cost_returns_zero_for_unknown_auction() {
    let env = env();
    let cost: i64 = env
        .eval("return C_AuctionHouse.GetCancelCost(999999)")
        .unwrap();
    assert_eq!(cost, 0);
}

#[test]
fn get_available_post_count_returns_max_stack_minus_listed_quantity() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        // Aqirite max stack = 1000. List 250, expect 750 available.
        state.auction_owned.push(OwnedAuction {
            auction_id: 1,
            item_id: AQIRITE_ITEM_ID,
            item_level: 70,
            quantity: 250,
            bid_amount: 0,
            buyout_amount: 0,
            status: 0,
            time_left: 3,
            time_left_seconds: 12 * 3600,
        });
    }
    let available: i32 = env
        .eval("return C_AuctionHouse.GetAvailablePostCount({ itemID = 210935 })")
        .unwrap();
    assert_eq!(available, 750);
}

#[test]
fn get_available_post_count_returns_zero_for_unknown_item() {
    let env = env();
    let available: i32 = env
        .eval("return C_AuctionHouse.GetAvailablePostCount({})")
        .unwrap();
    assert_eq!(available, 0);
}

#[test]
fn get_item_commodity_status_classifies_item_vs_commodity() {
    let env = env();
    let (commodity, item, unknown): (i32, i32, i32) = env
        .eval(
            r#"
            return C_AuctionHouse.GetItemCommodityStatus({ itemID = 210935 }),
                   C_AuctionHouse.GetItemCommodityStatus({ itemID = 211988 }),
                   C_AuctionHouse.GetItemCommodityStatus({})
            "#,
        )
        .unwrap();
    assert_eq!(
        commodity, COMMODITY_STATUS_COMMODITY,
        "Aqirite stacks to 1000"
    );
    assert_eq!(
        item, COMMODITY_STATUS_ITEM,
        "Greatcloak is gear, not a commodity"
    );
    assert_eq!(unknown, COMMODITY_STATUS_UNKNOWN);
}

#[test]
fn get_quote_duration_remaining_defaults_to_zero() {
    let env = env();
    let remaining: i64 = env
        .eval("return C_AuctionHouse.GetQuoteDurationRemaining()")
        .unwrap();
    assert_eq!(remaining, 0);
}
