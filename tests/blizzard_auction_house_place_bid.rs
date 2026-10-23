//! Place-bid coverage for `Blizzard_AuctionHouseUI`.

use crate::common;

use common::auction_fixtures::{load_ui_for_model, with_model};
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};

const ROOT: &str = "Blizzard_AuctionHouseUI";
const PLACE_BID_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
    if not condition then
        table.insert(failures, message)
    end
end

local itemID = 210934
local itemLevel = 70
local auctionID = 42
local bidAmount = 1500
local buyoutAmount = 6000

A_Admin.ClearAuctionBrowseResults()
A_Admin.ClearAuctionBids()
A_Admin.SetMoney(1000000)
A_Admin.AddAuctionBrowseResult(itemID, itemLevel, buyoutAmount, 1, false, auctionID)

local beforeMoney = GetMoney()
local bidAddedCount = 0
local bidPayload = nil
local bidsUpdatedCount = 0

local listener = CreateFrame("Frame")
listener:RegisterEvent("BID_ADDED")
listener:RegisterEvent("BIDS_UPDATED")
listener:SetScript("OnEvent", function(_, eventName, payload)
    if eventName == "BID_ADDED" then
        bidAddedCount = bidAddedCount + 1
        bidPayload = payload
    elseif eventName == "BIDS_UPDATED" then
        bidsUpdatedCount = bidsUpdatedCount + 1
    end
end)

local ok = C_AuctionHouse.PlaceBid(auctionID, bidAmount)
local afterMoney = GetMoney()
local row = C_AuctionHouse.GetBidInfo(C_AuctionHouse.GetNumBids())

expect(ok == true, "PlaceBid return " .. tostring(ok))
expect(C_AuctionHouse.GetNumBids() == 1, "bid count " .. tostring(C_AuctionHouse.GetNumBids()))
expect(bidAddedCount == 1, "BID_ADDED count " .. tostring(bidAddedCount))
expect(bidPayload == auctionID, "BID_ADDED payload " .. tostring(bidPayload))
expect(bidsUpdatedCount == 1, "BIDS_UPDATED count " .. tostring(bidsUpdatedCount))
expect(afterMoney == beforeMoney - bidAmount,
       "money after bid " .. tostring(afterMoney) .. " bid " .. tostring(bidAmount))

expect(type(row) == "table", "bid row must be a table")
if type(row) == "table" then
    expect(row.auctionID == auctionID, "row auctionID")
    expect(row.bidAmount == bidAmount, "row bidAmount")
    expect(row.buyoutAmount == buyoutAmount, "row buyoutAmount")
end

return table.concat(failures, "\n")
"#;

#[test]
fn auction_house_place_bid_adds_bid_fires_events_and_deducts_money() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_model(|env| {
                clear_recorded_lua_errors(env);
                load_ui_for_model(env);

                let failures: String = env
                    .eval(PLACE_BID_PROBE_LUA)
                    .expect("AuctionHouse place-bid probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` place-bid failures:\n{failures}"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` place-bid probe emitted Lua errors:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}
