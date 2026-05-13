//! Commodity-purchase coverage for `Blizzard_AuctionHouseUI`.

use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};

const ROOT: &str = "Blizzard_AuctionHouseUI";
const AQIRITE_ID: i32 = 210934;
const PURCHASE_BAG: i32 = 4;
const PURCHASE_SLOT: i32 = 0;
const SETUP_AND_START_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
    if not condition then
        table.insert(failures, message)
    end
end

local itemID = 210934

A_Admin.ClearAuctionCommoditySearchResults()
A_Admin.ClearBags()
A_Admin.SetMoney(1000000)
A_Admin.AddAuctionCommoditySearchResult(itemID, 2, 300, 301, "HighSeller")
A_Admin.AddAuctionCommoditySearchResult(itemID, 3, 100, 302, "LowSeller")
A_Admin.AddAuctionCommoditySearchResult(itemID, 5, 200, 303, "MidSeller")

local priceUpdatedCount = 0
local pricePayload = nil
local listener = CreateFrame("Frame")
listener:RegisterEvent("COMMODITY_PRICE_UPDATED")
listener:SetScript("OnEvent", function(_, _, payload)
    priceUpdatedCount = priceUpdatedCount + 1
    pricePayload = payload
end)

C_AuctionHouse.StartCommoditiesPurchase(itemID, 5)

expect(priceUpdatedCount == 1, "price updated count " .. tostring(priceUpdatedCount))
expect(pricePayload == itemID, "price payload " .. tostring(pricePayload))

return table.concat(failures, "\n")
"#;

const CONFIRM_PURCHASE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
    if not condition then
        table.insert(failures, message)
    end
end

local itemID = 210934
local succeededCount = 0
local succeededPayload = nil
local listener = CreateFrame("Frame")
listener:RegisterEvent("COMMODITY_PURCHASE_SUCCEEDED")
listener:SetScript("OnEvent", function(_, _, payload)
    succeededCount = succeededCount + 1
    succeededPayload = payload
end)

C_AuctionHouse.ConfirmCommoditiesPurchase(itemID, 5)

expect(succeededCount == 1, "purchase succeeded count " .. tostring(succeededCount))
expect(succeededPayload == itemID, "purchase succeeded payload " .. tostring(succeededPayload))

return table.concat(failures, "\n")
"#;

#[test]
fn auction_house_commodity_purchase_quotes_drains_and_deposits_cheapest_first() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AuctionHouseUI")"#)
                    .expect("C_AddOns.LoadAddOn should return");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let start_failures: String = env
                    .eval(SETUP_AND_START_LUA)
                    .expect("AuctionHouse commodity-purchase start probe should run");
                assert!(
                    start_failures.is_empty(),
                    "`{ROOT}` commodity-purchase start failures:\n{start_failures}"
                );

                assert_cheapest_first_quote(env);

                let confirm_failures: String = env
                    .eval(CONFIRM_PURCHASE_LUA)
                    .expect("AuctionHouse commodity-purchase confirm probe should run");
                assert!(
                    confirm_failures.is_empty(),
                    "`{ROOT}` commodity-purchase confirm failures:\n{confirm_failures}"
                );

                assert_post_purchase_state(env);

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` commodity-purchase probe emitted Lua errors:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}

fn assert_cheapest_first_quote(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let state = env.state().borrow();
    let quote = state
        .commodity_purchase_quote
        .as_ref()
        .expect("StartCommoditiesPurchase should write a quote");
    assert_eq!(quote.item_id, AQIRITE_ID);
    assert_eq!(quote.quantity, 5);
    assert_eq!(quote.total_price, 700, "3 at 100 plus 2 at 200");
}

fn assert_post_purchase_state(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let state = env.state().borrow();
    assert!(
        state.commodity_purchase_quote.is_none(),
        "ConfirmCommoditiesPurchase should clear the quote"
    );

    let bucket = state
        .auction_commodity_searches
        .get(&AQIRITE_ID)
        .expect("commodity bucket should remain after partial drain");
    assert_eq!(bucket.entries.len(), 2);
    assert_eq!(bucket.entries[0].unit_price, 200);
    assert_eq!(bucket.entries[0].quantity, 3);
    assert_eq!(bucket.entries[1].unit_price, 300);
    assert_eq!(bucket.entries[1].quantity, 2);

    let slot = state
        .bag_items
        .get(&(PURCHASE_BAG, PURCHASE_SLOT))
        .expect("purchase should land in deterministic bag slot");
    assert_eq!(slot.item_id, AQIRITE_ID as u32);
    assert_eq!(slot.stack_count, 5);
}
