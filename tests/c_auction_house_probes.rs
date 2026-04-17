//! Tests for `C_AuctionHouse` probes backed by
//! `SimState.auction_browse_results` + `auction_replicate_items`.

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::{AuctionBrowseResult, AuctionReplicateItem};

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
