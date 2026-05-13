//! Item-search coverage for `Blizzard_AuctionHouseUI`.

use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};

const ROOT: &str = "Blizzard_AuctionHouseUI";
const ITEM_SEARCH_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
    if not condition then
        table.insert(failures, message)
    end
end

local itemID = 210935
local itemLevel = 70
local key = C_AuctionHouse.MakeItemKey(itemID, itemLevel)

A_Admin.ClearAuctionItemSearchResults()
A_Admin.AddAuctionItemSearchResult(itemID, itemLevel, 101, 4, 1000, 1100, 2500, 1, 1800, "SellerOne")
A_Admin.AddAuctionItemSearchResult(itemID, itemLevel, 102, 8, 1200, 1300, 2700, 2, 3600, "SellerTwo")
A_Admin.AddAuctionItemSearchResult(itemID, itemLevel, 103, 2, 900, 950, 2000, 3, 7200, "SellerThree")

local eventCount = 0
local payloadItemID = nil
local payloadItemLevel = nil
local listener = CreateFrame("Frame")
listener:RegisterEvent("ITEM_SEARCH_RESULTS_UPDATED")
listener:SetScript("OnEvent", function(_, _, payload)
    eventCount = eventCount + 1
    if type(payload) == "table" then
        payloadItemID = payload.itemID
        payloadItemLevel = payload.itemLevel
    end
end)

C_AuctionHouse.SendSearchQuery(key, {}, false)

expect(eventCount == 1, "item-search event count " .. tostring(eventCount))
expect(payloadItemID == itemID, "payload itemID " .. tostring(payloadItemID))
expect(payloadItemLevel == itemLevel, "payload itemLevel " .. tostring(payloadItemLevel))
expect(C_AuctionHouse.GetNumItemSearchResults(key) == 3,
       "item-search result count " .. tostring(C_AuctionHouse.GetNumItemSearchResults(key)))

local first = C_AuctionHouse.GetItemSearchResultInfo(key, 1)
expect(type(first) == "table", "first result must be a table")
if type(first) == "table" then
    expect(first.auctionID == 101, "first auctionID")
    expect(first.quantity == 4, "first quantity")
    expect(first.owners and first.owners[1] == "SellerOne", "first owner")
end

return table.concat(failures, "\n")
"#;

#[test]
fn auction_house_item_search_query_returns_seeded_rows() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AuctionHouseUI")"#)
                    .expect("C_AddOns.LoadAddOn should return");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let failures: String = env
                    .eval(ITEM_SEARCH_PROBE_LUA)
                    .expect("AuctionHouse item-search probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` item-search failures:\n{failures}"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` item-search probe emitted Lua errors:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}
