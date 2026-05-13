//! Post-item coverage for `Blizzard_AuctionHouseUI`.

use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};

const ROOT: &str = "Blizzard_AuctionHouseUI";
const POST_ITEM_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
    if not condition then
        table.insert(failures, message)
    end
end

local itemID = 210934
local itemLocation = ItemLocation:CreateFromBagAndSlot(0, 1)
local duration = 2
local quantity = 1
local bid = 1000
local buyout = 2000

A_Admin.ClearOwnedAuctions()
A_Admin.ClearBags()
A_Admin.AddBagItem(0, 1, itemID, 1)
A_Admin.SetMoney(1000000)

local beforeOwned = C_AuctionHouse.GetNumOwnedAuctions()
local beforeMoney = GetMoney()
local deposit = C_AuctionHouse.CalculateItemDeposit(itemLocation, duration, quantity)

local createdCount = 0
local createdAuctionID = nil
local ownedUpdatedCount = 0

local listener = CreateFrame("Frame")
listener:RegisterEvent("AUCTION_HOUSE_AUCTION_CREATED")
listener:RegisterEvent("OWNED_AUCTIONS_UPDATED")
listener:SetScript("OnEvent", function(_, eventName, auctionID)
    if eventName == "AUCTION_HOUSE_AUCTION_CREATED" then
        createdCount = createdCount + 1
        createdAuctionID = auctionID
    elseif eventName == "OWNED_AUCTIONS_UPDATED" then
        ownedUpdatedCount = ownedUpdatedCount + 1
    end
end)

local ok = C_AuctionHouse.PostItem(itemLocation, duration, quantity, bid, buyout)
local afterOwned = C_AuctionHouse.GetNumOwnedAuctions()
local afterMoney = GetMoney()
local row = C_AuctionHouse.GetOwnedAuctionInfo(afterOwned)

expect(ok == true, "PostItem return " .. tostring(ok))
expect(afterOwned == beforeOwned + 1, "owned count " .. tostring(afterOwned))
expect(createdCount == 1, "created event count " .. tostring(createdCount))
expect(ownedUpdatedCount == 1, "owned updated event count " .. tostring(ownedUpdatedCount))
expect(afterMoney == beforeMoney - deposit,
       "money after post " .. tostring(afterMoney) .. " deposit " .. tostring(deposit))

expect(type(row) == "table", "owned row must be a table")
if type(row) == "table" then
    expect(row.auctionID == createdAuctionID, "row auctionID")
    expect(row.itemKey.itemID == itemID, "row itemID")
    expect(row.quantity == quantity, "row quantity")
    expect(row.bidAmount == bid, "row bid")
    expect(row.buyoutAmount == buyout, "row buyout")
end

return table.concat(failures, "\n")
"#;

#[test]
fn auction_house_post_item_adds_owned_row_fires_events_and_deducts_deposit() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AuctionHouseUI")"#)
                    .expect("C_AddOns.LoadAddOn should return");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let failures: String = env
                    .eval(POST_ITEM_PROBE_LUA)
                    .expect("AuctionHouse post-item probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` post-item failures:\n{failures}"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` post-item probe emitted Lua errors:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}
