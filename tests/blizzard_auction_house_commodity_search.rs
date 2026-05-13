//! Commodity-search coverage for `Blizzard_AuctionHouseUI`.

use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};

const ROOT: &str = "Blizzard_AuctionHouseUI";
const COMMODITY_SEARCH_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
    if not condition then
        table.insert(failures, message)
    end
end

local itemID = 210935
local key = C_AuctionHouse.MakeItemKey(itemID)

A_Admin.ClearAuctionCommoditySearchResults()
A_Admin.AddAuctionCommoditySearchResult(itemID, 100, 2500, 201, "SellerOne")
A_Admin.AddAuctionCommoditySearchResult(itemID, 25, 4000, 202, "SellerTwo")
A_Admin.AddAuctionCommoditySearchResult(itemID, 80, 3100, 203, "SellerThree")

local eventCount = 0
local payloadItemID = nil
local listener = CreateFrame("Frame")
listener:RegisterEvent("COMMODITY_SEARCH_RESULTS_UPDATED")
listener:SetScript("OnEvent", function(_, _, payload)
    eventCount = eventCount + 1
    payloadItemID = payload
end)

C_AuctionHouse.SendSearchQuery(key, {}, false)

expect(eventCount == 1, "commodity event count " .. tostring(eventCount))
expect(payloadItemID == itemID, "payload itemID " .. tostring(payloadItemID))
expect(C_AuctionHouse.GetNumCommoditySearchResults(itemID) == 3,
       "commodity result count " .. tostring(C_AuctionHouse.GetNumCommoditySearchResults(itemID)))
expect(C_AuctionHouse.GetMaxCommoditySearchResultPrice(itemID) == 4000,
       "max commodity price " .. tostring(C_AuctionHouse.GetMaxCommoditySearchResultPrice(itemID)))

local first = C_AuctionHouse.GetCommoditySearchResultInfo(itemID, 1)
expect(type(first) == "table", "first commodity result must be a table")
if type(first) == "table" then
    expect(first.auctionID == 201, "first auctionID")
    expect(first.quantity == 100, "first quantity")
    expect(first.unitPrice == 2500, "first unitPrice")
end

return table.concat(failures, "\n")
"#;

#[test]
fn auction_house_commodity_search_query_returns_seeded_rows() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AuctionHouseUI")"#)
                    .expect("C_AddOns.LoadAddOn should return");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let failures: String = env
                    .eval(COMMODITY_SEARCH_PROBE_LUA)
                    .expect("AuctionHouse commodity-search probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` commodity-search failures:\n{failures}"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` commodity-search probe emitted Lua errors:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}
