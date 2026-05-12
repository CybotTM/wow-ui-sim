//! Integration tests for `src/lua_api/globals/inventory_counts.rs`.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("WowLuaEnv init")
}

// ── GetContainerNumFreeSlots ──────────────────────────────────────────────────

#[test]
fn get_container_num_free_slots_backpack_reports_free_and_bag_type() {
    let env = env();
    // The seeded backpack has some items already — free = 16 - occupied.
    let (free, bag_type, occupied): (i32, i32, i32) = env
        .eval(
            r#"
            local free, bagType = GetContainerNumFreeSlots(0)
            local totalSlots = C_Container.GetContainerNumSlots(0)
            return free, bagType, totalSlots - free
            "#,
        )
        .unwrap();
    assert_eq!(bag_type, 0, "normal backpack reports bagType 0");
    assert!(free >= 0);
    assert!(
        occupied > 0,
        "seeded backpack should have at least one occupied slot"
    );
}

#[test]
fn get_container_num_free_slots_nonbackpack_bags_report_zero() {
    let env = env();
    let (free, bag_type): (i32, i32) = env.eval("return GetContainerNumFreeSlots(3)").unwrap();
    assert_eq!(free, 0, "non-backpack bags aren't modelled; no free slots");
    assert_eq!(bag_type, 0);
}

// ── GetNumLootItems ───────────────────────────────────────────────────────────

#[test]
fn get_num_loot_items_zero_when_no_loot_window() {
    let env = env();
    let n: i32 = env.eval("return GetNumLootItems()").unwrap();
    assert_eq!(n, 0);
}

#[test]
fn get_num_loot_items_counts_seeded_slots() {
    use wow_ui_sim::lua_api::state::BagItem;

    let env = env();
    env.state().borrow_mut().loot_slots = vec![
        BagItem {
            item_id: 6948,
            stack_count: 1,
            hyperlink: None,
        },
        BagItem {
            item_id: 19019,
            stack_count: 1,
            hyperlink: None,
        },
        BagItem {
            item_id: 36942,
            stack_count: 1,
            hyperlink: None,
        },
    ];
    let n: i32 = env.eval("return GetNumLootItems()").unwrap();
    assert_eq!(n, 3);
}

// ── GetMerchantNumItems ───────────────────────────────────────────────────────

#[test]
fn get_merchant_num_items_zero_when_merchant_closed() {
    let env = env();
    let n: i32 = env.eval("return GetMerchantNumItems()").unwrap();
    assert_eq!(n, 0);
}

#[test]
fn get_merchant_num_items_counts_seeded_items() {
    let env = env();
    env.state().borrow_mut().merchant_items = vec![159, 160, 4536, 4540];
    let n: i32 = env.eval("return GetMerchantNumItems()").unwrap();
    assert_eq!(n, 4);
}

// ── GetNumAuctionItems ────────────────────────────────────────────────────────

#[test]
fn get_num_auction_items_list_reports_browse_size() {
    let env = env();
    {
        let mut state = env.state().borrow_mut();
        state.auction_browse_results.clear();
        state.auction_browse_items = vec![6948; 12];
    }
    let (n, total): (i32, i32) = env.eval(r#"return GetNumAuctionItems("list")"#).unwrap();
    assert_eq!(n, 12);
    assert_eq!(total, 12);
}

#[test]
fn get_num_auction_items_owner_and_bidder_always_zero() {
    let env = env();
    env.state().borrow_mut().auction_browse_items = vec![6948; 5];
    let (owner_n, owner_total): (i32, i32) =
        env.eval(r#"return GetNumAuctionItems("owner")"#).unwrap();
    let (bidder_n, bidder_total): (i32, i32) =
        env.eval(r#"return GetNumAuctionItems("bidder")"#).unwrap();
    assert_eq!((owner_n, owner_total), (0, 0));
    assert_eq!((bidder_n, bidder_total), (0, 0));
}

#[test]
fn get_num_auction_items_unknown_list_type_reports_zero() {
    let env = env();
    env.state().borrow_mut().auction_browse_items = vec![6948; 3];
    let (n, total): (i32, i32) = env.eval("return GetNumAuctionItems()").unwrap();
    assert_eq!((n, total), (0, 0));
}
