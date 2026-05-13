//! Cancel-auction coverage for `Blizzard_AuctionHouseUI`.

use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};

const ROOT: &str = "Blizzard_AuctionHouseUI";
const CANCEL_AUCTION_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
    if not condition then
        table.insert(failures, message)
    end
end

local auctionID = 9876
local itemID = 210934
local itemLevel = 70
local quantity = 1
local bidAmount = 10000
local buyoutAmount = 50000
local status = 0
local timeLeft = 3
local timeLeftSeconds = 12 * 60 * 60

A_Admin.ClearOwnedAuctions()
A_Admin.AddOwnedAuction(
    auctionID,
    itemID,
    itemLevel,
    quantity,
    bidAmount,
    buyoutAmount,
    status,
    timeLeft,
    timeLeftSeconds
)
A_Admin.SetMoney(1000000)

local beforeMoney = GetMoney()
local cancelCost = C_AuctionHouse.GetCancelCost(auctionID)
local canceledCount = 0
local canceledPayload = nil
local ownedUpdatedCount = 0

local listener = CreateFrame("Frame")
listener:RegisterEvent("AUCTION_CANCELED")
listener:RegisterEvent("OWNED_AUCTIONS_UPDATED")
listener:SetScript("OnEvent", function(_, eventName, payload)
    if eventName == "AUCTION_CANCELED" then
        canceledCount = canceledCount + 1
        canceledPayload = payload
    elseif eventName == "OWNED_AUCTIONS_UPDATED" then
        ownedUpdatedCount = ownedUpdatedCount + 1
    end
end)

C_AuctionHouse.CancelAuction(auctionID)

local afterMoney = GetMoney()
local row = C_AuctionHouse.GetOwnedAuctionInfo(1)
local expectedRefund = bidAmount - cancelCost

expect(canceledCount == 1, "cancel event count " .. tostring(canceledCount))
expect(canceledPayload == auctionID, "cancel payload " .. tostring(canceledPayload))
expect(ownedUpdatedCount == 1, "owned updated count " .. tostring(ownedUpdatedCount))
expect(afterMoney == beforeMoney + expectedRefund,
       "money after cancel " .. tostring(afterMoney) .. " expected refund " .. tostring(expectedRefund))
expect(type(row) == "table", "owned auction row should remain as sold")
if type(row) == "table" then
    expect(row.auctionID == auctionID, "row auctionID")
    expect(row.status == 1, "row status " .. tostring(row.status))
end

return table.concat(failures, "\n")
"#;

#[test]
fn auction_house_cancel_auction_marks_sold_fires_events_and_refunds_bid_minus_cost() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AuctionHouseUI")"#)
                    .expect("C_AddOns.LoadAddOn should return");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let failures: String = env
                    .eval(CANCEL_AUCTION_PROBE_LUA)
                    .expect("AuctionHouse cancel-auction probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` cancel-auction failures:\n{failures}"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` cancel-auction probe emitted Lua errors:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}
